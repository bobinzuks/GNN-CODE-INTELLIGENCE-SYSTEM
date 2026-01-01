//! Go language parser implementation
//!
//! This module provides a comprehensive parser for Go source code,
//! extracting functions, structs, interfaces, goroutines, channels, and their relationships.

use crate::graph::{CodeEdge, CodeGraph, CodeGraphBuilder, CodeNode, EdgeKind, NodeKind};
use crate::languages::LanguageParser;
use anyhow::{Context, Result};
use petgraph::graph::NodeIndex;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use tree_sitter::{Node, Parser, Tree};

/// Go language parser
pub struct GoParser {
    parser: Parser,
}

impl GoParser {
    /// Create a new Go parser
    pub fn new() -> Self {
        let mut parser = Parser::new();
        parser
            .set_language(tree_sitter_go::language())
            .expect("Error loading Go grammar");

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

        // Look for preceding comment siblings
        if let Some(prev_sibling) = node.prev_sibling() {
            if prev_sibling.kind() == "comment" {
                let text = self.node_text(&prev_sibling, source);
                if text.starts_with("//") || text.starts_with("/*") {
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

    /// Parse function declarations
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
        let mut return_type = None;
        let mut receiver = None;

        // Get function name
        if let Some(name_node) = node.child_by_field_name("name") {
            name = self.node_text(&name_node, source).to_string();
        }

        // Get receiver (for methods)
        if let Some(receiver_node) = node.child_by_field_name("receiver") {
            receiver = Some(self.node_text(&receiver_node, source).to_string());
        }

        // Get parameters
        if let Some(params_node) = node.child_by_field_name("parameters") {
            for param in params_node.children(&mut params_node.walk()) {
                if param.kind() == "parameter_declaration" {
                    parameters.push(self.node_text(&param, source).to_string());
                }
            }
        }

        // Get return type
        if let Some(result_node) = node.child_by_field_name("result") {
            return_type = Some(self.node_text(&result_node, source).to_string());
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
            let ret_str = return_type.unwrap_or_default();
            let receiver_str = receiver.as_ref().map(|r| format!("{} ", r)).unwrap_or_default();
            Some(format!("func {}{}({}) {}", receiver_str, name, params_str, ret_str))
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
            "go".to_string(),
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

        if let Some(receiver_str) = receiver {
            code_node = code_node.add_metadata("receiver".to_string(), receiver_str);
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
                "go_statement" => {
                    self.parse_go_statement(&child, source, file_path, builder, parent_idx)?;
                }
                "if_statement" => {
                    self.parse_if_statement(&child, source, file_path, builder, parent_idx)?;
                }
                "for_statement" => {
                    self.parse_for_loop(&child, source, file_path, builder, parent_idx)?;
                }
                "short_var_declaration" | "var_declaration" => {
                    self.parse_variable_declaration(&child, source, file_path, builder, parent_idx)?;
                }
                "send_statement" | "receive_statement" => {
                    self.parse_channel_operation(&child, source, file_path, builder, parent_idx)?;
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
                "go".to_string(),
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

    /// Parse go statement (goroutine launch)
    fn parse_go_statement(
        &self,
        node: &Node,
        source: &str,
        file_path: &Path,
        builder: &mut CodeGraphBuilder,
        parent_idx: NodeIndex,
    ) -> Result<()> {
        // Extract the function being launched as a goroutine
        for child in node.children(&mut node.walk()) {
            if child.kind() == "call_expression" {
                self.parse_call_expression(&child, source, file_path, builder, parent_idx)?;

                // Mark this call as a goroutine
                if let Some(function_node) = child.child_by_field_name("function") {
                    let call_name = self.node_text(&function_node, source).to_string();
                    let (start, end) = self.get_position(&child);

                    let goroutine_node = CodeNode::new(
                        NodeKind::FunctionCall,
                        format!("go {}", call_name),
                        file_path.to_path_buf(),
                        "go".to_string(),
                    )
                    .with_position(start, end)
                    .add_metadata("goroutine".to_string(), "true".to_string());

                    let goroutine_idx = builder.add_node(goroutine_node);
                    builder.add_edge(parent_idx, goroutine_idx, CodeEdge::new(EdgeKind::Calls));
                }
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
            "go".to_string(),
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
            "go".to_string(),
        )
        .with_position(start, end);

        let for_idx = builder.add_node(for_node);
        builder.add_edge(parent_idx, for_idx, CodeEdge::new(EdgeKind::Contains));

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
            if child.kind() == "var_spec" {
                if let Some(name_node) = child.child_by_field_name("name") {
                    let var_name = self.node_text(&name_node, source).to_string();
                    let (start, end) = self.get_position(&child);
                    let var_type = child.child_by_field_name("type")
                        .map(|t| self.node_text(&t, source).to_string());

                    let mut var_node = CodeNode::new(
                        NodeKind::Variable,
                        var_name.clone(),
                        file_path.to_path_buf(),
                        "go".to_string(),
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

    /// Parse channel operations (send/receive)
    fn parse_channel_operation(
        &self,
        node: &Node,
        source: &str,
        file_path: &Path,
        builder: &mut CodeGraphBuilder,
        parent_idx: NodeIndex,
    ) -> Result<()> {
        let (start, end) = self.get_position(node);
        let op_text = self.node_text(node, source).to_string();

        let channel_node = CodeNode::new(
            NodeKind::Variable,
            op_text.clone(),
            file_path.to_path_buf(),
            "go".to_string(),
        )
        .with_position(start, end)
        .add_metadata("channel_op".to_string(), "true".to_string());

        let channel_idx = builder.add_node(channel_node);
        builder.add_edge(parent_idx, channel_idx, CodeEdge::new(EdgeKind::Writes));

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

        // Get struct name (from parent type_spec)
        if let Some(parent) = node.parent() {
            if parent.kind() == "type_spec" {
                if let Some(name_node) = parent.child_by_field_name("name") {
                    name = self.node_text(&name_node, source).to_string();
                }
            }
        }

        // Get fields
        if let Some(field_list) = node.child_by_field_name("fields") {
            for field in field_list.children(&mut field_list.walk()) {
                if field.kind() == "field_declaration" {
                    if let Some(name_node) = field.child_by_field_name("name") {
                        fields.push(self.node_text(&name_node, source).to_string());
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
            "go".to_string(),
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
                "go".to_string(),
            )
            .with_qualified_name(format!("{}.{}", qualified_name, field_name));

            let field_idx = builder.add_node(field_node);
            builder.add_edge(struct_idx, field_idx, CodeEdge::new(EdgeKind::Contains));
        }

        Ok(struct_idx)
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

        // Get interface name (from parent type_spec)
        if let Some(parent) = node.parent() {
            if parent.kind() == "type_spec" {
                if let Some(name_node) = parent.child_by_field_name("name") {
                    name = self.node_text(&name_node, source).to_string();
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
            "go".to_string(),
        )
        .with_position(start, end)
        .with_qualified_name(qualified_name.clone())
        .with_source(self.node_text(node, source).to_string());

        if let Some(doc) = documentation {
            code_node = code_node.with_doc(doc);
        }

        let interface_idx = builder.add_node(code_node);

        Ok(interface_idx)
    }

    /// Parse import statements
    fn parse_import(
        &self,
        node: &Node,
        source: &str,
        file_path: &Path,
        builder: &mut CodeGraphBuilder,
    ) -> Result<()> {
        let import_text = self.node_text(node, source).to_string();
        let (start, end) = self.get_position(node);

        let import_node = CodeNode::new(
            NodeKind::Import,
            import_text.clone(),
            file_path.to_path_buf(),
            "go".to_string(),
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
                "function_declaration" | "method_declaration" => {
                    let is_method = child.kind() == "method_declaration";
                    self.parse_function(&child, source, file_path, builder, parent_scope, is_method)?;
                }
                "type_declaration" => {
                    // Parse type specs within
                    for type_child in child.children(&mut child.walk()) {
                        if type_child.kind() == "type_spec" {
                            if let Some(type_node) = type_child.child_by_field_name("type") {
                                match type_node.kind() {
                                    "struct_type" => {
                                        self.parse_struct(&type_node, source, file_path, builder, parent_scope)?;
                                    }
                                    "interface_type" => {
                                        self.parse_interface(&type_node, source, file_path, builder, parent_scope)?;
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                }
                "import_declaration" => {
                    self.parse_import(&child, source, file_path, builder)?;
                }
                "source_file" => {
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

impl Default for GoParser {
    fn default() -> Self {
        Self::new()
    }
}

impl LanguageParser for GoParser {
    fn language_name(&self) -> &str {
        "go"
    }

    fn file_extensions(&self) -> &[&str] {
        &["go"]
    }

    fn parse_file(&self, file_path: &Path, builder: &mut CodeGraphBuilder) -> Result<()> {
        let source = fs::read_to_string(file_path)
            .with_context(|| format!("Failed to read file: {}", file_path.display()))?;

        let mut parser = Parser::new();
        parser
            .set_language(tree_sitter_go::language())
            .expect("Error loading Go grammar");

        let tree = parser
            .parse(&source, None)
            .context("Failed to parse Go source")?;

        let root_node = tree.root_node();

        let file_node = CodeNode::new(
            NodeKind::File,
            file_path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string(),
            file_path.to_path_buf(),
            "go".to_string(),
        )
        .with_qualified_name(file_path.to_string_lossy().to_string());

        let _file_idx = builder.add_node(file_node);

        self.parse_items(&root_node, &source, file_path, builder, "")?;

        Ok(())
    }
}
