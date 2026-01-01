//! C language parser implementation
//!
//! This module provides a comprehensive parser for C source code,
//! extracting functions, structs, pointers, memory operations, and their relationships.

use crate::graph::{CodeEdge, CodeGraph, CodeGraphBuilder, CodeNode, EdgeKind, NodeKind};
use crate::languages::LanguageParser;
use anyhow::{Context, Result};
use petgraph::graph::NodeIndex;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use tree_sitter::{Node, Parser, Tree};

/// C language parser
pub struct CParser {
    parser: Parser,
}

impl CParser {
    /// Create a new C parser
    pub fn new() -> Self {
        let mut parser = Parser::new();
        parser
            .set_language(tree_sitter_c::language())
            .expect("Error loading C grammar");

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

    /// Extract documentation comments
    fn extract_documentation(&self, node: &Node, source: &str) -> Option<String> {
        let mut docs = Vec::new();

        if let Some(prev_sibling) = node.prev_sibling() {
            if prev_sibling.kind() == "comment" {
                let text = self.node_text(&prev_sibling, source);
                if text.starts_with("/**") || text.starts_with("//") {
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

    /// Parse function definition
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

        // Get function declarator
        if let Some(declarator) = node.child_by_field_name("declarator") {
            // Extract name from nested declarator
            if let Some(name_node) = self.find_function_name(&declarator) {
                name = self.node_text(&name_node, source).to_string();
            }

            // Get parameters
            if let Some(params_node) = declarator.child_by_field_name("parameters") {
                for param in params_node.children(&mut params_node.walk()) {
                    if param.kind() == "parameter_declaration" {
                        parameters.push(self.node_text(&param, source).to_string());
                    }
                }
            }
        }

        // Get return type
        if let Some(type_node) = node.child_by_field_name("type") {
            return_type = Some(self.node_text(&type_node, source).to_string());
        }

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
            Some(format!("{} {}({})", ret_str, name, params_str))
        };

        let mut code_node = CodeNode::new(
            NodeKind::Function,
            name.clone(),
            file_path.to_path_buf(),
            "c".to_string(),
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

        let func_idx = builder.add_node(code_node);

        // Parse function body
        if let Some(body) = node.child_by_field_name("body") {
            self.parse_function_body(&body, source, file_path, builder, func_idx, &qualified_name)?;
        }

        Ok(func_idx)
    }

    /// Helper to find function name in nested declarator
    fn find_function_name<'a>(&self, node: &'a Node) -> Option<Node<'a>> {
        if node.kind() == "identifier" {
            return Some(*node);
        }

        for child in node.children(&mut node.walk()) {
            if child.kind() == "identifier" {
                return Some(child);
            } else if let Some(found) = self.find_function_name(&child) {
                return Some(found);
            }
        }

        None
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
                "for_statement" => {
                    self.parse_for_loop(&child, source, file_path, builder, parent_idx)?;
                }
                "while_statement" => {
                    self.parse_while_loop(&child, source, file_path, builder, parent_idx)?;
                }
                "declaration" => {
                    self.parse_variable_declaration(&child, source, file_path, builder, parent_idx)?;
                }
                "pointer_expression" => {
                    self.parse_pointer_operation(&child, source, file_path, builder, parent_idx)?;
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

            // Track memory allocation functions
            let is_memory_fn = matches!(call_name.as_str(), "malloc" | "calloc" | "realloc" | "free");

            let mut call_node = CodeNode::new(
                NodeKind::FunctionCall,
                call_name.clone(),
                file_path.to_path_buf(),
                "c".to_string(),
            )
            .with_position(start, end);

            if is_memory_fn {
                call_node = call_node.add_metadata("memory_function".to_string(), "true".to_string());
            }

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
            "c".to_string(),
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
            "c".to_string(),
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
            "c".to_string(),
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
            if child.kind() == "init_declarator" || child.kind() == "pointer_declarator" {
                if let Some(declarator) = child.child_by_field_name("declarator") {
                    if let Some(name_node) = self.find_function_name(&declarator) {
                        let var_name = self.node_text(&name_node, source).to_string();
                        let (start, end) = self.get_position(&child);

                        // Check if it's a pointer
                        let is_pointer = child.kind() == "pointer_declarator" ||
                                        self.node_text(&child, source).contains("*");

                        let mut var_node = CodeNode::new(
                            NodeKind::Variable,
                            var_name.clone(),
                            file_path.to_path_buf(),
                            "c".to_string(),
                        )
                        .with_position(start, end);

                        if is_pointer {
                            var_node = var_node.add_metadata("pointer".to_string(), "true".to_string());
                        }

                        let var_idx = builder.add_node(var_node);
                        builder.add_edge(parent_idx, var_idx, CodeEdge::new(EdgeKind::Writes));
                    }
                }
            }
        }

        Ok(())
    }

    /// Parse pointer operations (dereference, address-of)
    fn parse_pointer_operation(
        &self,
        node: &Node,
        source: &str,
        file_path: &Path,
        builder: &mut CodeGraphBuilder,
        parent_idx: NodeIndex,
    ) -> Result<()> {
        let (start, end) = self.get_position(node);
        let op_text = self.node_text(node, source).to_string();

        let ptr_node = CodeNode::new(
            NodeKind::Variable,
            op_text.clone(),
            file_path.to_path_buf(),
            "c".to_string(),
        )
        .with_position(start, end)
        .add_metadata("pointer_op".to_string(), "true".to_string());

        let ptr_idx = builder.add_node(ptr_node);
        builder.add_edge(parent_idx, ptr_idx, CodeEdge::new(EdgeKind::Reads));

        Ok(())
    }

    /// Parse struct definition
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

        // Get struct name
        if let Some(name_node) = node.child_by_field_name("name") {
            name = self.node_text(&name_node, source).to_string();
        }

        // Get fields
        if let Some(body) = node.child_by_field_name("body") {
            for field in body.children(&mut body.walk()) {
                if field.kind() == "field_declaration" {
                    for declarator in field.children(&mut field.walk()) {
                        if declarator.kind() == "field_declarator" {
                            if let Some(name_node) = self.find_function_name(&declarator) {
                                fields.push(self.node_text(&name_node, source).to_string());
                            }
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
            NodeKind::Struct,
            name.clone(),
            file_path.to_path_buf(),
            "c".to_string(),
        )
        .with_position(start, end)
        .with_qualified_name(qualified_name.clone())
        .with_source(self.node_text(node, source).to_string());

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
                "c".to_string(),
            )
            .with_qualified_name(format!("{}.{}", qualified_name, field_name));

            let field_idx = builder.add_node(field_node);
            builder.add_edge(struct_idx, field_idx, CodeEdge::new(EdgeKind::Contains));
        }

        Ok(struct_idx)
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
                "function_definition" => {
                    self.parse_function(&child, source, file_path, builder, parent_scope)?;
                }
                "struct_specifier" => {
                    self.parse_struct(&child, source, file_path, builder, parent_scope)?;
                }
                "translation_unit" => {
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

impl Default for CParser {
    fn default() -> Self {
        Self::new()
    }
}

impl LanguageParser for CParser {
    fn language_name(&self) -> &str {
        "c"
    }

    fn file_extensions(&self) -> &[&str] {
        &["c", "h"]
    }

    fn parse_file(&self, file_path: &Path, builder: &mut CodeGraphBuilder) -> Result<()> {
        let source = fs::read_to_string(file_path)
            .with_context(|| format!("Failed to read file: {}", file_path.display()))?;

        let mut parser = Parser::new();
        parser
            .set_language(tree_sitter_c::language())
            .expect("Error loading C grammar");

        let tree = parser
            .parse(&source, None)
            .context("Failed to parse C source")?;

        let root_node = tree.root_node();

        let file_node = CodeNode::new(
            NodeKind::File,
            file_path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string(),
            file_path.to_path_buf(),
            "c".to_string(),
        )
        .with_qualified_name(file_path.to_string_lossy().to_string());

        let _file_idx = builder.add_node(file_node);

        self.parse_items(&root_node, &source, file_path, builder, "")?;

        Ok(())
    }
}
