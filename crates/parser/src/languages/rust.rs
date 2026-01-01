//! Rust language parser implementation
//!
//! This module provides a comprehensive parser for Rust source code,
//! extracting functions, structs, enums, traits, imports, and their relationships.

use crate::graph::{CodeEdge, CodeGraph, CodeGraphBuilder, CodeNode, EdgeKind, NodeKind};
use crate::languages::LanguageParser;
use anyhow::{Context, Result};
use petgraph::graph::NodeIndex;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use tree_sitter::{Node, Parser, Query, QueryCursor, Tree};

/// Rust language parser
pub struct RustParser {
    parser: Parser,
}

impl RustParser {
    /// Create a new Rust parser
    pub fn new() -> Self {
        let mut parser = Parser::new();
        parser
            .set_language(tree_sitter_rust::language())
            .expect("Error loading Rust grammar");

        Self { parser }
    }

    /// Parse a Rust file and build the AST
    fn parse_source(&mut self, source: &str) -> Result<Tree> {
        self.parser
            .parse(source, None)
            .context("Failed to parse Rust source")
    }

    /// Extract the text of a node from the source
    fn node_text<'a>(&self, node: &Node, source: &'a str) -> &'a str {
        node.utf8_text(source.as_bytes()).unwrap_or("")
    }

    /// Get position as (line, column)
    fn get_position(&self, node: &Node) -> ((usize, usize), (usize, usize)) {
        let start = node.start_position();
        let end = node.end_position();
        ((start.row, start.column), (end.row, end.column))
    }

    /// Extract visibility from a node
    fn extract_visibility(&self, node: &Node, source: &str) -> Option<String> {
        // Look for visibility_modifier child
        for child in node.children(&mut node.walk()) {
            if child.kind() == "visibility_modifier" {
                return Some(self.node_text(&child, source).to_string());
            }
        }
        None
    }

    /// Extract documentation comments
    fn extract_documentation(&self, node: &Node, source: &str) -> Option<String> {
        let mut docs = Vec::new();

        // Look for preceding siblings that are line_comment or block_comment
        if let Some(prev_sibling) = node.prev_sibling() {
            if prev_sibling.kind() == "line_comment" || prev_sibling.kind() == "block_comment" {
                let text = self.node_text(&prev_sibling, source);
                if text.starts_with("///") || text.starts_with("//!") || text.starts_with("/**") {
                    docs.push(text.to_string());
                }
            }
        }

        if docs.is_empty() {
            None
        } else {
            Some(docs.join("\n"))
        }
    }

    /// Parse function definitions
    fn parse_function(
        &self,
        node: &Node,
        source: &str,
        file_path: &Path,
        builder: &mut CodeGraphBuilder,
        parent_scope: &str,
    ) -> Result<NodeIndex> {
        let mut name = String::new();
        let mut parameters = Vec::new();
        let mut return_type = None;

        for child in node.children(&mut node.walk()) {
            match child.kind() {
                "identifier" => {
                    if name.is_empty() {
                        name = self.node_text(&child, source).to_string();
                    }
                }
                "parameters" => {
                    // Extract parameter names and types
                    for param in child.children(&mut child.walk()) {
                        if param.kind() == "parameter" {
                            let param_text = self.node_text(&param, source);
                            parameters.push(param_text.to_string());
                        }
                    }
                }
                "type_identifier" | "primitive_type" | "generic_type" => {
                    if return_type.is_none() {
                        return_type = Some(self.node_text(&child, source).to_string());
                    }
                }
                _ => {}
            }
        }

        let qualified_name = if parent_scope.is_empty() {
            name.clone()
        } else {
            format!("{}::{}", parent_scope, name)
        };

        let (start, end) = self.get_position(node);
        let visibility = self.extract_visibility(node, source);
        let documentation = self.extract_documentation(node, source);

        let type_signature = if parameters.is_empty() && return_type.is_none() {
            None
        } else {
            let params_str = parameters.join(", ");
            let ret_str = return_type.unwrap_or_else(|| "()".to_string());
            Some(format!("fn({}) -> {}", params_str, ret_str))
        };

        let code_node = CodeNode::new(
            NodeKind::Function,
            name.clone(),
            file_path.to_path_buf(),
            "rust".to_string(),
        )
        .with_position(start, end)
        .with_qualified_name(qualified_name.clone())
        .with_source(self.node_text(node, source).to_string());

        let code_node = if let Some(vis) = visibility {
            code_node.with_visibility(vis)
        } else {
            code_node
        };

        let code_node = if let Some(type_sig) = type_signature {
            code_node.with_type(type_sig)
        } else {
            code_node
        };

        let code_node = if let Some(doc) = documentation {
            code_node.with_doc(doc)
        } else {
            code_node
        };

        let func_idx = builder.add_node(code_node);

        // Parse function body for calls
        if let Some(body) = node.child_by_field_name("body") {
            self.parse_function_body(&body, source, file_path, builder, func_idx, &qualified_name)?;
        }

        Ok(func_idx)
    }

    /// Parse function body to extract function calls
    fn parse_function_body(
        &self,
        node: &Node,
        source: &str,
        file_path: &Path,
        builder: &mut CodeGraphBuilder,
        parent_idx: NodeIndex,
        parent_scope: &str,
    ) -> Result<()> {
        let mut cursor = node.walk();

        for child in node.children(&mut cursor) {
            match child.kind() {
                "call_expression" => {
                    self.parse_call_expression(&child, source, file_path, builder, parent_idx)?;
                }
                _ => {
                    // Recursively search for call expressions
                    self.parse_function_body(&child, source, file_path, builder, parent_idx, parent_scope)?;
                }
            }
        }

        Ok(())
    }

    /// Parse a call expression
    fn parse_call_expression(
        &self,
        node: &Node,
        source: &str,
        file_path: &Path,
        builder: &mut CodeGraphBuilder,
        caller_idx: NodeIndex,
    ) -> Result<()> {
        if let Some(function_node) = node.child_by_field_name("function") {
            let call_name = self.node_text(&function_node, source).to_string();
            let (start, end) = self.get_position(node);

            let call_node = CodeNode::new(
                NodeKind::FunctionCall,
                call_name.clone(),
                file_path.to_path_buf(),
                "rust".to_string(),
            )
            .with_position(start, end);

            let call_idx = builder.add_node(call_node);

            // Add call edge from caller to callee
            builder.add_edge(caller_idx, call_idx, CodeEdge::new(EdgeKind::Calls));

            // Try to resolve the call to a definition
            if let Some(target_idx) = builder.find_node(&call_name) {
                builder.add_edge(call_idx, target_idx, CodeEdge::new(EdgeKind::Calls));
            }
        }

        Ok(())
    }

    /// Parse struct definitions
    fn parse_struct(
        &self,
        node: &Node,
        source: &str,
        file_path: &Path,
        builder: &mut CodeGraphBuilder,
        parent_scope: &str,
    ) -> Result<NodeIndex> {
        let mut name = String::new();
        let mut fields = Vec::new();

        for child in node.children(&mut node.walk()) {
            match child.kind() {
                "type_identifier" => {
                    if name.is_empty() {
                        name = self.node_text(&child, source).to_string();
                    }
                }
                "field_declaration_list" => {
                    for field in child.children(&mut child.walk()) {
                        if field.kind() == "field_declaration" {
                            if let Some(field_name) = field.child_by_field_name("name") {
                                fields.push(self.node_text(&field_name, source).to_string());
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        let qualified_name = if parent_scope.is_empty() {
            name.clone()
        } else {
            format!("{}::{}", parent_scope, name)
        };

        let (start, end) = self.get_position(node);
        let visibility = self.extract_visibility(node, source);
        let documentation = self.extract_documentation(node, source);

        let mut code_node = CodeNode::new(
            NodeKind::Struct,
            name.clone(),
            file_path.to_path_buf(),
            "rust".to_string(),
        )
        .with_position(start, end)
        .with_qualified_name(qualified_name.clone())
        .with_source(self.node_text(node, source).to_string());

        if let Some(vis) = visibility {
            code_node = code_node.with_visibility(vis);
        }

        if let Some(doc) = documentation {
            code_node = code_node.with_doc(doc);
        }

        let struct_idx = builder.add_node(code_node);

        // Add field nodes
        for field_name in fields {
            let field_node = CodeNode::new(
                NodeKind::Field,
                field_name.clone(),
                file_path.to_path_buf(),
                "rust".to_string(),
            )
            .with_qualified_name(format!("{}::{}", qualified_name, field_name));

            let field_idx = builder.add_node(field_node);
            builder.add_edge(struct_idx, field_idx, CodeEdge::new(EdgeKind::Contains));
        }

        Ok(struct_idx)
    }

    /// Parse enum definitions
    fn parse_enum(
        &self,
        node: &Node,
        source: &str,
        file_path: &Path,
        builder: &mut CodeGraphBuilder,
        parent_scope: &str,
    ) -> Result<NodeIndex> {
        let mut name = String::new();
        let mut variants = Vec::new();

        for child in node.children(&mut node.walk()) {
            match child.kind() {
                "type_identifier" => {
                    if name.is_empty() {
                        name = self.node_text(&child, source).to_string();
                    }
                }
                "enum_variant_list" => {
                    for variant in child.children(&mut child.walk()) {
                        if variant.kind() == "enum_variant" {
                            if let Some(variant_name) = variant.child_by_field_name("name") {
                                variants.push(self.node_text(&variant_name, source).to_string());
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        let qualified_name = if parent_scope.is_empty() {
            name.clone()
        } else {
            format!("{}::{}", parent_scope, name)
        };

        let (start, end) = self.get_position(node);
        let visibility = self.extract_visibility(node, source);
        let documentation = self.extract_documentation(node, source);

        let mut code_node = CodeNode::new(
            NodeKind::Enum,
            name.clone(),
            file_path.to_path_buf(),
            "rust".to_string(),
        )
        .with_position(start, end)
        .with_qualified_name(qualified_name.clone())
        .with_source(self.node_text(node, source).to_string());

        if let Some(vis) = visibility {
            code_node = code_node.with_visibility(vis);
        }

        if let Some(doc) = documentation {
            code_node = code_node.with_doc(doc);
        }

        let enum_idx = builder.add_node(code_node);

        Ok(enum_idx)
    }

    /// Parse trait definitions
    fn parse_trait(
        &self,
        node: &Node,
        source: &str,
        file_path: &Path,
        builder: &mut CodeGraphBuilder,
        parent_scope: &str,
    ) -> Result<NodeIndex> {
        let mut name = String::new();

        for child in node.children(&mut node.walk()) {
            if child.kind() == "type_identifier" {
                if name.is_empty() {
                    name = self.node_text(&child, source).to_string();
                }
            }
        }

        let qualified_name = if parent_scope.is_empty() {
            name.clone()
        } else {
            format!("{}::{}", parent_scope, name)
        };

        let (start, end) = self.get_position(node);
        let visibility = self.extract_visibility(node, source);
        let documentation = self.extract_documentation(node, source);

        let mut code_node = CodeNode::new(
            NodeKind::Trait,
            name.clone(),
            file_path.to_path_buf(),
            "rust".to_string(),
        )
        .with_position(start, end)
        .with_qualified_name(qualified_name.clone())
        .with_source(self.node_text(node, source).to_string());

        if let Some(vis) = visibility {
            code_node = code_node.with_visibility(vis);
        }

        if let Some(doc) = documentation {
            code_node = code_node.with_doc(doc);
        }

        let trait_idx = builder.add_node(code_node);

        Ok(trait_idx)
    }

    /// Parse impl blocks
    fn parse_impl(
        &self,
        node: &Node,
        source: &str,
        file_path: &Path,
        builder: &mut CodeGraphBuilder,
        parent_scope: &str,
    ) -> Result<()> {
        let mut type_name = String::new();
        let mut trait_name = None;

        // Extract the type being implemented
        if let Some(type_node) = node.child_by_field_name("type") {
            type_name = self.node_text(&type_node, source).to_string();
        }

        // Check if this is a trait implementation
        if let Some(trait_node) = node.child_by_field_name("trait") {
            trait_name = Some(self.node_text(&trait_node, source).to_string());
        }

        let impl_scope = if parent_scope.is_empty() {
            type_name.clone()
        } else {
            format!("{}::{}", parent_scope, type_name)
        };

        // Parse methods in the impl block
        if let Some(body) = node.child_by_field_name("body") {
            for child in body.children(&mut body.walk()) {
                if child.kind() == "function_item" {
                    let method_idx = self.parse_function(&child, source, file_path, builder, &impl_scope)?;

                    // If we have the type, add a contains edge
                    if let Some(type_idx) = builder.find_node(&type_name) {
                        builder.add_edge(type_idx, method_idx, CodeEdge::new(EdgeKind::Contains));
                    }

                    // If this is a trait impl, add implements edge
                    if let Some(ref trait_name_str) = trait_name {
                        if let Some(trait_idx) = builder.find_node(trait_name_str) {
                            if let Some(type_idx) = builder.find_node(&type_name) {
                                builder.add_edge(type_idx, trait_idx, CodeEdge::new(EdgeKind::Implements));
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Parse use declarations (imports)
    fn parse_use_declaration(
        &self,
        node: &Node,
        source: &str,
        file_path: &Path,
        builder: &mut CodeGraphBuilder,
    ) -> Result<()> {
        let import_text = self.node_text(node, source).to_string();
        let (start, end) = self.get_position(node);

        // Extract the actual path being imported
        let import_path = import_text
            .strip_prefix("use ")
            .unwrap_or(&import_text)
            .trim_end_matches(';')
            .trim()
            .to_string();

        let import_node = CodeNode::new(
            NodeKind::Import,
            import_path.clone(),
            file_path.to_path_buf(),
            "rust".to_string(),
        )
        .with_position(start, end)
        .with_source(import_text);

        builder.add_node(import_node);

        Ok(())
    }

    /// Parse module declarations
    fn parse_mod_declaration(
        &self,
        node: &Node,
        source: &str,
        file_path: &Path,
        builder: &mut CodeGraphBuilder,
        parent_scope: &str,
    ) -> Result<NodeIndex> {
        let mut name = String::new();

        for child in node.children(&mut node.walk()) {
            if child.kind() == "identifier" {
                name = self.node_text(&child, source).to_string();
                break;
            }
        }

        let qualified_name = if parent_scope.is_empty() {
            name.clone()
        } else {
            format!("{}::{}", parent_scope, name)
        };

        let (start, end) = self.get_position(node);
        let visibility = self.extract_visibility(node, source);

        let mut code_node = CodeNode::new(
            NodeKind::Module,
            name.clone(),
            file_path.to_path_buf(),
            "rust".to_string(),
        )
        .with_position(start, end)
        .with_qualified_name(qualified_name.clone());

        if let Some(vis) = visibility {
            code_node = code_node.with_visibility(vis);
        }

        let mod_idx = builder.add_node(code_node);

        // If the module has a body, parse it
        if let Some(body) = node.child_by_field_name("body") {
            self.parse_items(&body, source, file_path, builder, &qualified_name)?;
        }

        Ok(mod_idx)
    }

    /// Parse top-level items recursively
    fn parse_items(
        &self,
        node: &Node,
        source: &str,
        file_path: &Path,
        builder: &mut CodeGraphBuilder,
        parent_scope: &str,
    ) -> Result<()> {
        let mut cursor = node.walk();

        for child in node.children(&mut cursor) {
            match child.kind() {
                "function_item" => {
                    self.parse_function(&child, source, file_path, builder, parent_scope)?;
                }
                "struct_item" => {
                    self.parse_struct(&child, source, file_path, builder, parent_scope)?;
                }
                "enum_item" => {
                    self.parse_enum(&child, source, file_path, builder, parent_scope)?;
                }
                "trait_item" => {
                    self.parse_trait(&child, source, file_path, builder, parent_scope)?;
                }
                "impl_item" => {
                    self.parse_impl(&child, source, file_path, builder, parent_scope)?;
                }
                "use_declaration" => {
                    self.parse_use_declaration(&child, source, file_path, builder)?;
                }
                "mod_item" => {
                    let _mod_idx = self.parse_mod_declaration(&child, source, file_path, builder, parent_scope)?;
                }
                "declaration_list" | "source_file" => {
                    // Recursively parse nested items
                    self.parse_items(&child, source, file_path, builder, parent_scope)?;
                }
                _ => {
                    // Check if this node has children that might contain items
                    if child.child_count() > 0 {
                        self.parse_items(&child, source, file_path, builder, parent_scope)?;
                    }
                }
            }
        }

        Ok(())
    }
}

impl Default for RustParser {
    fn default() -> Self {
        Self::new()
    }
}

impl LanguageParser for RustParser {
    fn language_name(&self) -> &str {
        "rust"
    }

    fn file_extensions(&self) -> &[&str] {
        &["rs"]
    }

    fn parse_file(&self, file_path: &Path, builder: &mut CodeGraphBuilder) -> Result<()> {
        // Read the file
        let source = fs::read_to_string(file_path)
            .with_context(|| format!("Failed to read file: {}", file_path.display()))?;

        // Parse the source code
        let mut parser = Parser::new();
        parser
            .set_language(tree_sitter_rust::language())
            .expect("Error loading Rust grammar");

        let tree = parser
            .parse(&source, None)
            .context("Failed to parse Rust source")?;

        let root_node = tree.root_node();

        // Create a file node
        let file_node = CodeNode::new(
            NodeKind::File,
            file_path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string(),
            file_path.to_path_buf(),
            "rust".to_string(),
        )
        .with_qualified_name(file_path.to_string_lossy().to_string());

        let _file_idx = builder.add_node(file_node);

        // Parse all top-level items
        self.parse_items(&root_node, &source, file_path, builder, "")?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use std::io::Write;

    #[test]
    fn test_parse_function() {
        let source = r#"
            pub fn add(a: i32, b: i32) -> i32 {
                a + b
            }
        "#;

        let mut file = NamedTempFile::new().unwrap();
        file.write_all(source.as_bytes()).unwrap();

        let parser = RustParser::new();
        let mut builder = CodeGraphBuilder::new();
        parser.parse_file(file.path(), &mut builder).unwrap();

        let graph = builder.build();
        assert!(graph.node_count() > 0);

        // Find the function node
        let func_node = graph
            .node_weights()
            .find(|n| n.kind == NodeKind::Function && n.name == "add");
        assert!(func_node.is_some());
    }

    #[test]
    fn test_parse_struct() {
        let source = r#"
            pub struct Point {
                x: f64,
                y: f64,
            }
        "#;

        let mut file = NamedTempFile::new().unwrap();
        file.write_all(source.as_bytes()).unwrap();

        let parser = RustParser::new();
        let mut builder = CodeGraphBuilder::new();
        parser.parse_file(file.path(), &mut builder).unwrap();

        let graph = builder.build();
        let struct_node = graph
            .node_weights()
            .find(|n| n.kind == NodeKind::Struct && n.name == "Point");
        assert!(struct_node.is_some());
    }

    #[test]
    fn test_parse_enum() {
        let source = r#"
            enum Color {
                Red,
                Green,
                Blue,
            }
        "#;

        let mut file = NamedTempFile::new().unwrap();
        file.write_all(source.as_bytes()).unwrap();

        let parser = RustParser::new();
        let mut builder = CodeGraphBuilder::new();
        parser.parse_file(file.path(), &mut builder).unwrap();

        let graph = builder.build();
        let enum_node = graph
            .node_weights()
            .find(|n| n.kind == NodeKind::Enum && n.name == "Color");
        assert!(enum_node.is_some());
    }

    #[test]
    fn test_parse_trait() {
        let source = r#"
            pub trait Drawable {
                fn draw(&self);
            }
        "#;

        let mut file = NamedTempFile::new().unwrap();
        file.write_all(source.as_bytes()).unwrap();

        let parser = RustParser::new();
        let mut builder = CodeGraphBuilder::new();
        parser.parse_file(file.path(), &mut builder).unwrap();

        let graph = builder.build();
        let trait_node = graph
            .node_weights()
            .find(|n| n.kind == NodeKind::Trait && n.name == "Drawable");
        assert!(trait_node.is_some());
    }
}
