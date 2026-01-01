//! TypeScript language parser implementation
//!
//! This module provides a comprehensive parser for TypeScript source code,
//! extracting all JavaScript features plus types, interfaces, generics, and type annotations.

use crate::graph::{CodeEdge, CodeGraph, CodeGraphBuilder, CodeNode, EdgeKind, NodeKind};
use crate::languages::LanguageParser;
use anyhow::{Context, Result};
use petgraph::graph::NodeIndex;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use tree_sitter::{Node, Parser, Tree};

/// TypeScript language parser
pub struct TypeScriptParser {
    parser: Parser,
}

impl TypeScriptParser {
    /// Create a new TypeScript parser
    pub fn new() -> Self {
        let mut parser = Parser::new();
        parser
            .set_language(tree_sitter_typescript::language_typescript())
            .expect("Error loading TypeScript grammar");

        Self { parser }
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

    /// Extract TSDoc/JSDoc documentation
    fn extract_documentation(&self, node: &Node, source: &str) -> Option<String> {
        let mut docs = Vec::new();

        if let Some(prev_sibling) = node.prev_sibling() {
            if prev_sibling.kind() == "comment" {
                let text = self.node_text(&prev_sibling, source);
                if text.starts_with("/**") {
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

    /// Extract type annotation
    fn extract_type_annotation(&self, node: &Node, source: &str) -> Option<String> {
        if let Some(type_node) = node.child_by_field_name("type") {
            return Some(self.node_text(&type_node, source).to_string());
        }

        // Also check for type_annotation child
        for child in node.children(&mut node.walk()) {
            if child.kind() == "type_annotation" {
                if let Some(type_node) = child.named_child(0) {
                    return Some(self.node_text(&type_node, source).to_string());
                }
            }
        }

        None
    }

    /// Extract generic parameters
    fn extract_generic_parameters(&self, node: &Node, source: &str) -> Option<String> {
        for child in node.children(&mut node.walk()) {
            if child.kind() == "type_parameters" {
                return Some(self.node_text(&child, source).to_string());
            }
        }
        None
    }

    /// Parse function declarations and expressions
    fn parse_function(
        &self,
        node: &Node,
        source: &str,
        file_path: &Path,
        builder: &mut CodeGraphBuilder,
        parent_scope: &str,
        is_method: bool,
    ) -> Result<NodeIndex> {
        let mut name = String::new();
        let mut parameters = Vec::new();
        let mut is_async = false;
        let mut is_generator = false;
        let mut return_type = None;
        let mut generic_params = None;

        // Check for modifiers
        for child in node.children(&mut node.walk()) {
            if child.kind() == "async" {
                is_async = true;
            } else if child.kind() == "*" {
                is_generator = true;
            }
        }

        // Get function name
        if let Some(name_node) = node.child_by_field_name("name") {
            name = self.node_text(&name_node, source).to_string();
        } else {
            name = "<anonymous>".to_string();
        }

        // Get generic parameters
        generic_params = self.extract_generic_parameters(node, source);

        // Get parameters with types
        if let Some(params_node) = node.child_by_field_name("parameters") {
            for param in params_node.children(&mut params_node.walk()) {
                if param.kind() == "required_parameter" || param.kind() == "optional_parameter" {
                    let param_text = self.node_text(&param, source).to_string();
                    parameters.push(param_text);
                }
            }
        }

        // Get return type
        return_type = self.extract_type_annotation(node, source);

        let qualified_name = if parent_scope.is_empty() {
            name.clone()
        } else {
            format!("{}.{}", parent_scope, name)
        };

        let (start, end) = self.get_position(node);
        let documentation = self.extract_documentation(node, source);

        let type_signature = {
            let params_str = parameters.join(", ");
            let ret_str = return_type.unwrap_or_else(|| "void".to_string());
            let generic_str = generic_params.unwrap_or_default();
            let prefix = if is_async {
                "async "
            } else if is_generator {
                "function* "
            } else {
                "function "
            };
            Some(format!("{}{}({}) -> {}", prefix, generic_str, params_str, ret_str))
        };

        let node_kind = if is_method {
            NodeKind::Method
        } else {
            NodeKind::Function
        };

        let mut code_node = CodeNode::new(
            node_kind,
            name.clone(),
            file_path.to_path_buf(),
            "typescript".to_string(),
        )
        .with_position(start, end)
        .with_qualified_name(qualified_name.clone())
        .with_source(self.node_text(node, source).to_string());

        if let Some(type_sig) = type_signature {
            code_node = code_node.with_type(type_sig);
        }

        if let Some(doc) = documentation {
            code_node = code_node.with_doc(doc);
        }

        if is_async {
            code_node = code_node.add_metadata("async".to_string(), "true".to_string());
        }

        let func_idx = builder.add_node(code_node);

        // Parse function body
        if let Some(body) = node.child_by_field_name("body") {
            self.parse_function_body(&body, source, file_path, builder, func_idx, &qualified_name)?;
        }

        Ok(func_idx)
    }

    /// Parse function body
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
                "if_statement" => {
                    self.parse_if_statement(&child, source, file_path, builder, parent_idx)?;
                }
                "for_statement" | "for_in_statement" | "for_of_statement" => {
                    self.parse_for_loop(&child, source, file_path, builder, parent_idx)?;
                }
                "while_statement" => {
                    self.parse_while_loop(&child, source, file_path, builder, parent_idx)?;
                }
                "variable_declaration" | "lexical_declaration" => {
                    self.parse_variable_declaration(&child, source, file_path, builder, parent_idx)?;
                }
                _ => {
                    self.parse_function_body(&child, source, file_path, builder, parent_idx, parent_scope)?;
                }
            }
        }

        Ok(())
    }

    /// Parse call expression
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
                "typescript".to_string(),
            )
            .with_position(start, end);

            let call_idx = builder.add_node(call_node);
            builder.add_edge(caller_idx, call_idx, CodeEdge::new(EdgeKind::Calls));

            if let Some(target_idx) = builder.find_node(&call_name) {
                builder.add_edge(call_idx, target_idx, CodeEdge::new(EdgeKind::Calls));
            }
        }

        Ok(())
    }

    /// Parse if statement
    fn parse_if_statement(
        &self,
        node: &Node,
        source: &str,
        file_path: &Path,
        builder: &mut CodeGraphBuilder,
        parent_idx: NodeIndex,
    ) -> Result<()> {
        let (start, end) = self.get_position(node);
        let if_node = CodeNode::new(
            NodeKind::IfStatement,
            "if".to_string(),
            file_path.to_path_buf(),
            "typescript".to_string(),
        )
        .with_position(start, end);

        let if_idx = builder.add_node(if_node);
        builder.add_edge(parent_idx, if_idx, CodeEdge::new(EdgeKind::Contains));

        Ok(())
    }

    /// Parse for loop
    fn parse_for_loop(
        &self,
        node: &Node,
        source: &str,
        file_path: &Path,
        builder: &mut CodeGraphBuilder,
        parent_idx: NodeIndex,
    ) -> Result<()> {
        let (start, end) = self.get_position(node);
        let for_node = CodeNode::new(
            NodeKind::ForLoop,
            "for".to_string(),
            file_path.to_path_buf(),
            "typescript".to_string(),
        )
        .with_position(start, end);

        let for_idx = builder.add_node(for_node);
        builder.add_edge(parent_idx, for_idx, CodeEdge::new(EdgeKind::Contains));

        Ok(())
    }

    /// Parse while loop
    fn parse_while_loop(
        &self,
        node: &Node,
        source: &str,
        file_path: &Path,
        builder: &mut CodeGraphBuilder,
        parent_idx: NodeIndex,
    ) -> Result<()> {
        let (start, end) = self.get_position(node);
        let while_node = CodeNode::new(
            NodeKind::WhileLoop,
            "while".to_string(),
            file_path.to_path_buf(),
            "typescript".to_string(),
        )
        .with_position(start, end);

        let while_idx = builder.add_node(while_node);
        builder.add_edge(parent_idx, while_idx, CodeEdge::new(EdgeKind::Contains));

        Ok(())
    }

    /// Parse variable declaration
    fn parse_variable_declaration(
        &self,
        node: &Node,
        source: &str,
        file_path: &Path,
        builder: &mut CodeGraphBuilder,
        parent_idx: NodeIndex,
    ) -> Result<()> {
        for child in node.children(&mut node.walk()) {
            if child.kind() == "variable_declarator" {
                if let Some(name_node) = child.child_by_field_name("name") {
                    let var_name = self.node_text(&name_node, source).to_string();
                    let (start, end) = self.get_position(&child);
                    let var_type = self.extract_type_annotation(&child, source);

                    let mut var_node = CodeNode::new(
                        NodeKind::Variable,
                        var_name.clone(),
                        file_path.to_path_buf(),
                        "typescript".to_string(),
                    )
                    .with_position(start, end);

                    if let Some(type_str) = var_type {
                        var_node = var_node.with_type(type_str);
                    }

                    let var_idx = builder.add_node(var_node);
                    builder.add_edge(parent_idx, var_idx, CodeEdge::new(EdgeKind::Writes));
                }
            }
        }

        Ok(())
    }

    /// Parse class definition
    fn parse_class(
        &self,
        node: &Node,
        source: &str,
        file_path: &Path,
        builder: &mut CodeGraphBuilder,
        parent_scope: &str,
    ) -> Result<NodeIndex> {
        let mut name = String::new();
        let mut heritage = None;
        let mut implements = Vec::new();
        let mut generic_params = None;

        // Get class name
        if let Some(name_node) = node.child_by_field_name("name") {
            name = self.node_text(&name_node, source).to_string();
        }

        // Get generic parameters
        generic_params = self.extract_generic_parameters(node, source);

        // Get heritage (extends)
        if let Some(heritage_node) = node.child_by_field_name("heritage") {
            for child in heritage_node.children(&mut heritage_node.walk()) {
                if child.kind() == "extends_clause" {
                    if let Some(type_node) = child.named_child(0) {
                        heritage = Some(self.node_text(&type_node, source).to_string());
                    }
                } else if child.kind() == "implements_clause" {
                    for impl_child in child.children(&mut child.walk()) {
                        if impl_child.kind() == "type_identifier" {
                            implements.push(self.node_text(&impl_child, source).to_string());
                        }
                    }
                }
            }
        }

        let qualified_name = if parent_scope.is_empty() {
            name.clone()
        } else {
            format!("{}.{}", parent_scope, name)
        };

        let (start, end) = self.get_position(node);
        let documentation = self.extract_documentation(node, source);

        let mut code_node = CodeNode::new(
            NodeKind::Class,
            name.clone(),
            file_path.to_path_buf(),
            "typescript".to_string(),
        )
        .with_position(start, end)
        .with_qualified_name(qualified_name.clone())
        .with_source(self.node_text(node, source).to_string());

        if let Some(doc) = documentation {
            code_node = code_node.with_doc(doc);
        }

        if let Some(generic_str) = generic_params {
            code_node = code_node.add_metadata("generics".to_string(), generic_str);
        }

        if let Some(ref heritage_str) = heritage {
            code_node = code_node.add_metadata("extends".to_string(), heritage_str.clone());
        }

        if !implements.is_empty() {
            code_node = code_node.add_metadata("implements".to_string(), implements.join(","));
        }

        let class_idx = builder.add_node(code_node);

        // Add extends edge
        if let Some(heritage_str) = heritage {
            if let Some(base_idx) = builder.find_node(&heritage_str) {
                builder.add_edge(class_idx, base_idx, CodeEdge::new(EdgeKind::Extends));
            }
        }

        // Add implements edges
        for impl_name in &implements {
            if let Some(impl_idx) = builder.find_node(impl_name) {
                builder.add_edge(class_idx, impl_idx, CodeEdge::new(EdgeKind::Implements));
            }
        }

        // Parse class body
        if let Some(body) = node.child_by_field_name("body") {
            self.parse_class_body(&body, source, file_path, builder, class_idx, &qualified_name)?;
        }

        Ok(class_idx)
    }

    /// Parse class body
    fn parse_class_body(
        &self,
        node: &Node,
        source: &str,
        file_path: &Path,
        builder: &mut CodeGraphBuilder,
        class_idx: NodeIndex,
        class_scope: &str,
    ) -> Result<()> {
        for child in node.children(&mut node.walk()) {
            match child.kind() {
                "method_definition" => {
                    let method_idx = self.parse_function(&child, source, file_path, builder, class_scope, true)?;
                    builder.add_edge(class_idx, method_idx, CodeEdge::new(EdgeKind::Contains));
                }
                "public_field_definition" | "field_definition" => {
                    if let Some(property) = child.child_by_field_name("property") {
                        let field_name = self.node_text(&property, source).to_string();
                        let (start, end) = self.get_position(&child);
                        let field_type = self.extract_type_annotation(&child, source);

                        let mut field_node = CodeNode::new(
                            NodeKind::Field,
                            field_name.clone(),
                            file_path.to_path_buf(),
                            "typescript".to_string(),
                        )
                        .with_position(start, end)
                        .with_qualified_name(format!("{}.{}", class_scope, field_name));

                        if let Some(type_str) = field_type {
                            field_node = field_node.with_type(type_str);
                        }

                        let field_idx = builder.add_node(field_node);
                        builder.add_edge(class_idx, field_idx, CodeEdge::new(EdgeKind::Contains));
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }

    /// Parse interface definition
    fn parse_interface(
        &self,
        node: &Node,
        source: &str,
        file_path: &Path,
        builder: &mut CodeGraphBuilder,
        parent_scope: &str,
    ) -> Result<NodeIndex> {
        let mut name = String::new();
        let mut extends = Vec::new();
        let mut generic_params = None;

        // Get interface name
        if let Some(name_node) = node.child_by_field_name("name") {
            name = self.node_text(&name_node, source).to_string();
        }

        // Get generic parameters
        generic_params = self.extract_generic_parameters(node, source);

        // Get extends
        for child in node.children(&mut node.walk()) {
            if child.kind() == "extends_clause" {
                for type_child in child.children(&mut child.walk()) {
                    if type_child.kind() == "type_identifier" {
                        extends.push(self.node_text(&type_child, source).to_string());
                    }
                }
            }
        }

        let qualified_name = if parent_scope.is_empty() {
            name.clone()
        } else {
            format!("{}.{}", parent_scope, name)
        };

        let (start, end) = self.get_position(node);
        let documentation = self.extract_documentation(node, source);

        let mut code_node = CodeNode::new(
            NodeKind::Interface,
            name.clone(),
            file_path.to_path_buf(),
            "typescript".to_string(),
        )
        .with_position(start, end)
        .with_qualified_name(qualified_name.clone())
        .with_source(self.node_text(node, source).to_string());

        if let Some(doc) = documentation {
            code_node = code_node.with_doc(doc);
        }

        if let Some(generic_str) = generic_params {
            code_node = code_node.add_metadata("generics".to_string(), generic_str);
        }

        if !extends.is_empty() {
            code_node = code_node.add_metadata("extends".to_string(), extends.join(","));
        }

        let interface_idx = builder.add_node(code_node);

        // Add extends edges
        for ext_name in &extends {
            if let Some(ext_idx) = builder.find_node(ext_name) {
                builder.add_edge(interface_idx, ext_idx, CodeEdge::new(EdgeKind::Extends));
            }
        }

        Ok(interface_idx)
    }

    /// Parse type alias
    fn parse_type_alias(
        &self,
        node: &Node,
        source: &str,
        file_path: &Path,
        builder: &mut CodeGraphBuilder,
        parent_scope: &str,
    ) -> Result<NodeIndex> {
        let mut name = String::new();
        let mut type_value = None;

        if let Some(name_node) = node.child_by_field_name("name") {
            name = self.node_text(&name_node, source).to_string();
        }

        if let Some(value_node) = node.child_by_field_name("value") {
            type_value = Some(self.node_text(&value_node, source).to_string());
        }

        let qualified_name = if parent_scope.is_empty() {
            name.clone()
        } else {
            format!("{}.{}", parent_scope, name)
        };

        let (start, end) = self.get_position(node);

        let mut code_node = CodeNode::new(
            NodeKind::TypeAlias,
            name.clone(),
            file_path.to_path_buf(),
            "typescript".to_string(),
        )
        .with_position(start, end)
        .with_qualified_name(qualified_name.clone());

        if let Some(type_str) = type_value {
            code_node = code_node.with_type(type_str);
        }

        let type_idx = builder.add_node(code_node);

        Ok(type_idx)
    }

    /// Parse import/export statements
    fn parse_import_export(
        &self,
        node: &Node,
        source: &str,
        file_path: &Path,
        builder: &mut CodeGraphBuilder,
    ) -> Result<()> {
        let import_text = self.node_text(node, source).to_string();
        let (start, end) = self.get_position(node);

        let kind = if node.kind().starts_with("import") {
            NodeKind::Import
        } else {
            NodeKind::Export
        };

        let import_node = CodeNode::new(
            kind,
            import_text.clone(),
            file_path.to_path_buf(),
            "typescript".to_string(),
        )
        .with_position(start, end)
        .with_source(import_text);

        builder.add_node(import_node);

        Ok(())
    }

    /// Parse top-level items
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
                "function_declaration" => {
                    self.parse_function(&child, source, file_path, builder, parent_scope, false)?;
                }
                "class_declaration" => {
                    self.parse_class(&child, source, file_path, builder, parent_scope)?;
                }
                "interface_declaration" => {
                    self.parse_interface(&child, source, file_path, builder, parent_scope)?;
                }
                "type_alias_declaration" => {
                    self.parse_type_alias(&child, source, file_path, builder, parent_scope)?;
                }
                "import_statement" | "export_statement" => {
                    self.parse_import_export(&child, source, file_path, builder)?;
                }
                "program" => {
                    self.parse_items(&child, source, file_path, builder, parent_scope)?;
                }
                _ => {
                    if child.child_count() > 0 {
                        self.parse_items(&child, source, file_path, builder, parent_scope)?;
                    }
                }
            }
        }

        Ok(())
    }
}

impl Default for TypeScriptParser {
    fn default() -> Self {
        Self::new()
    }
}

impl LanguageParser for TypeScriptParser {
    fn language_name(&self) -> &str {
        "typescript"
    }

    fn file_extensions(&self) -> &[&str] {
        &["ts", "tsx"]
    }

    fn parse_file(&self, file_path: &Path, builder: &mut CodeGraphBuilder) -> Result<()> {
        let source = fs::read_to_string(file_path)
            .with_context(|| format!("Failed to read file: {}", file_path.display()))?;

        let mut parser = Parser::new();
        parser
            .set_language(tree_sitter_typescript::language_typescript())
            .expect("Error loading TypeScript grammar");

        let tree = parser
            .parse(&source, None)
            .context("Failed to parse TypeScript source")?;

        let root_node = tree.root_node();

        let file_node = CodeNode::new(
            NodeKind::File,
            file_path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string(),
            file_path.to_path_buf(),
            "typescript".to_string(),
        )
        .with_qualified_name(file_path.to_string_lossy().to_string());

        let _file_idx = builder.add_node(file_node);

        self.parse_items(&root_node, &source, file_path, builder, "")?;

        Ok(())
    }
}
