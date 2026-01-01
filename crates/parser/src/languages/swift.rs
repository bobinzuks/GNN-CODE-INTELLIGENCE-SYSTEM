//! Swift language parser implementation
//!
//! This module provides a comprehensive parser for Swift source code,
//! extracting classes, protocols, closures, optionals, and their relationships.

use crate::graph::{CodeEdge, CodeGraph, CodeGraphBuilder, CodeNode, EdgeKind, NodeKind};
use crate::languages::LanguageParser;
use anyhow::{Context, Result};
use petgraph::graph::NodeIndex;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use tree_sitter::{Node, Parser, Tree};

/// Swift language parser
pub struct SwiftParser {
    parser: Parser,
}

impl SwiftParser {
    /// Create a new Swift parser
    pub fn new() -> Self {
        let mut parser = Parser::new();
        parser
            .set_language(tree_sitter_swift::language())
            .expect("Error loading Swift grammar");

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
            if prev_sibling.kind() == "comment" || prev_sibling.kind() == "multiline_comment" {
                let text = self.node_text(&prev_sibling, source);
                if text.starts_with("///") || text.starts_with("/**") {
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

    /// Extract attributes (like @objc, @available, etc.)
    fn extract_attributes(&self, node: &Node, source: &str) -> Vec<String> {
        let mut attributes = Vec::new();

        for child in node.children(&mut node.walk()) {
            if child.kind() == "attribute" || child.kind() == "modifiers" {
                attributes.push(self.node_text(&child, source).to_string());
            }
        }

        attributes
    }

    /// Parse function declaration
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
        let mut is_async = false;
        let mut is_throws = false;

        // Get function name
        if let Some(name_node) = node.child_by_field_name("name") {
            name = self.node_text(&name_node, source).to_string();
        }

        // Check for async/throws
        for child in node.children(&mut node.walk()) {
            if child.kind() == "async" {
                is_async = true;
            } else if child.kind() == "throws" {
                is_throws = true;
            }
        }

        // Get parameters
        if let Some(params_node) = node.child_by_field_name("parameters") {
            for param in params_node.children(&mut params_node.walk()) {
                if param.kind() == "parameter" {
                    parameters.push(self.node_text(&param, source).to_string());
                }
            }
        }

        // Get return type
        if let Some(result) = node.child_by_field_name("result") {
            return_type = Some(self.node_text(&result, source).to_string());
        }

        let qualified_name = if parent_scope.is_empty() {
            name.clone()
        } else {
            format!("{}.{}", parent_scope, name)
        };

        let (start, end) = self.get_position(node);
        let documentation = self.extract_documentation(node, source);
        let attributes = self.extract_attributes(node, source);

        let type_signature = {
            let params_str = parameters.join(", ");
            let ret_str = return_type.unwrap_or_else(|| "Void".to_string());
            let async_str = if is_async { " async" } else { "" };
            let throws_str = if is_throws { " throws" } else { "" };
            Some(format!("func {}({}){}{} -> {}", name, params_str, async_str, throws_str, ret_str))
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
            "swift".to_string(),
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

        if !attributes.is_empty() {
            code_node = code_node.add_metadata("attributes".to_string(), attributes.join(","));
        }

        if is_async {
            code_node = code_node.add_metadata("async".to_string(), "true".to_string());
        }

        if is_throws {
            code_node = code_node.add_metadata("throws".to_string(), "true".to_string());
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
                "for_statement" => {
                    self.parse_for_loop(&child, source, file_path, builder, parent_idx)?;
                }
                "while_statement" => {
                    self.parse_while_loop(&child, source, file_path, builder, parent_idx)?;
                }
                "guard_statement" => {
                    self.parse_guard_statement(&child, source, file_path, builder, parent_idx)?;
                }
                "property_declaration" | "variable_declaration" => {
                    self.parse_variable_declaration(&child, source, file_path, builder, parent_idx)?;
                }
                "closure_expression" => {
                    self.parse_closure(&child, source, file_path, builder, parent_idx)?;
                }
                "optional_binding_condition" => {
                    self.parse_optional_binding(&child, source, file_path, builder, parent_idx)?;
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
                "swift".to_string(),
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
            "swift".to_string(),
        )
        .with_position(start, end);

        let if_idx = builder.add_node(if_node);
        builder.add_edge(parent_idx, if_idx, CodeEdge::new(EdgeKind::Contains));

        Ok(())
    }

    /// Parse guard statement (Swift-specific)
    fn parse_guard_statement(
        &self,
        node: &Node,
        source: &str,
        file_path: &Path,
        builder: &mut CodeGraphBuilder,
        parent_idx: NodeIndex,
    ) -> Result<()> {
        let (start, end) = self.get_position(node);
        let guard_node = CodeNode::new(
            NodeKind::IfStatement,
            "guard".to_string(),
            file_path.to_path_buf(),
            "swift".to_string(),
        )
        .with_position(start, end)
        .add_metadata("guard".to_string(), "true".to_string());

        let guard_idx = builder.add_node(guard_node);
        builder.add_edge(parent_idx, guard_idx, CodeEdge::new(EdgeKind::Contains));

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
            "swift".to_string(),
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
            "swift".to_string(),
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
            if child.kind() == "pattern_binding" {
                if let Some(pattern) = child.child_by_field_name("pattern") {
                    let var_name = self.node_text(&pattern, source).to_string();
                    let (start, end) = self.get_position(&child);

                    // Check if it's an optional type
                    let is_optional = self.node_text(&child, source).contains("?");

                    let var_type = child.child_by_field_name("type")
                        .map(|t| self.node_text(&t, source).to_string());

                    let mut var_node = CodeNode::new(
                        NodeKind::Variable,
                        var_name.clone(),
                        file_path.to_path_buf(),
                        "swift".to_string(),
                    )
                    .with_position(start, end);

                    if let Some(type_str) = var_type {
                        var_node = var_node.with_type(type_str);
                    }

                    if is_optional {
                        var_node = var_node.add_metadata("optional".to_string(), "true".to_string());
                    }

                    let var_idx = builder.add_node(var_node);
                    builder.add_edge(parent_idx, var_idx, CodeEdge::new(EdgeKind::Writes));
                }
            }
        }

        Ok(())
    }

    /// Parse closure expression
    fn parse_closure(
        &self,
        node: &Node,
        source: &str,
        file_path: &Path,
        builder: &mut CodeGraphBuilder,
        parent_idx: NodeIndex,
    ) -> Result<()> {
        let (start, end) = self.get_position(node);

        let closure_node = CodeNode::new(
            NodeKind::Function,
            "<closure>".to_string(),
            file_path.to_path_buf(),
            "swift".to_string(),
        )
        .with_position(start, end)
        .add_metadata("closure".to_string(), "true".to_string());

        let closure_idx = builder.add_node(closure_node);
        builder.add_edge(parent_idx, closure_idx, CodeEdge::new(EdgeKind::Contains));

        // Parse closure body
        self.parse_function_body(node, source, file_path, builder, closure_idx, "")?;

        Ok(())
    }

    /// Parse optional binding (if let, guard let)
    fn parse_optional_binding(
        &self,
        node: &Node,
        source: &str,
        file_path: &Path,
        builder: &mut CodeGraphBuilder,
        parent_idx: NodeIndex,
    ) -> Result<()> {
        let (start, end) = self.get_position(node);
        let binding_text = self.node_text(node, source).to_string();

        let binding_node = CodeNode::new(
            NodeKind::Variable,
            binding_text.clone(),
            file_path.to_path_buf(),
            "swift".to_string(),
        )
        .with_position(start, end)
        .add_metadata("optional_binding".to_string(), "true".to_string());

        let binding_idx = builder.add_node(binding_node);
        builder.add_edge(parent_idx, binding_idx, CodeEdge::new(EdgeKind::Writes));

        Ok(())
    }

    /// Parse class declaration
    fn parse_class(
        &self,
        node: &Node,
        source: &str,
        file_path: &Path,
        builder: &mut CodeGraphBuilder,
        parent_scope: &str,
    ) -> Result<NodeIndex> {
        let mut name = String::new();
        let mut superclass = None;
        let mut protocols = Vec::new();

        // Get class name
        if let Some(name_node) = node.child_by_field_name("name") {
            name = self.node_text(&name_node, source).to_string();
        }

        // Get inheritance
        if let Some(type_inheritance) = node.child_by_field_name("type_inheritance_clause") {
            let mut is_first = true;
            for type_child in type_inheritance.children(&mut type_inheritance.walk()) {
                if type_child.kind() == "type_identifier" {
                    let type_name = self.node_text(&type_child, source).to_string();
                    if is_first {
                        // First could be superclass or protocol
                        superclass = Some(type_name.clone());
                        is_first = false;
                    } else {
                        protocols.push(type_name);
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
        let attributes = self.extract_attributes(node, source);

        let mut code_node = CodeNode::new(
            NodeKind::Class,
            name.clone(),
            file_path.to_path_buf(),
            "swift".to_string(),
        )
        .with_position(start, end)
        .with_qualified_name(qualified_name.clone())
        .with_source(self.node_text(node, source).to_string());

        if let Some(doc) = documentation {
            code_node = code_node.with_doc(doc);
        }

        if !attributes.is_empty() {
            code_node = code_node.add_metadata("attributes".to_string(), attributes.join(","));
        }

        if let Some(ref superclass_str) = superclass {
            code_node = code_node.add_metadata("superclass".to_string(), superclass_str.clone());
        }

        if !protocols.is_empty() {
            code_node = code_node.add_metadata("protocols".to_string(), protocols.join(","));
        }

        let class_idx = builder.add_node(code_node);

        // Add inheritance edges
        if let Some(superclass_str) = superclass {
            if let Some(super_idx) = builder.find_node(&superclass_str) {
                builder.add_edge(class_idx, super_idx, CodeEdge::new(EdgeKind::Extends));
            }
        }

        // Add protocol conformance edges
        for protocol_name in &protocols {
            if let Some(protocol_idx) = builder.find_node(protocol_name) {
                builder.add_edge(class_idx, protocol_idx, CodeEdge::new(EdgeKind::Implements));
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
                "function_declaration" | "init_declaration" => {
                    let method_idx = self.parse_function(&child, source, file_path, builder, class_scope, true)?;
                    builder.add_edge(class_idx, method_idx, CodeEdge::new(EdgeKind::Contains));
                }
                "property_declaration" => {
                    for prop_child in child.children(&mut child.walk()) {
                        if prop_child.kind() == "pattern_binding" {
                            if let Some(pattern) = prop_child.child_by_field_name("pattern") {
                                let field_name = self.node_text(&pattern, source).to_string();
                                let (start, end) = self.get_position(&prop_child);

                                let field_node = CodeNode::new(
                                    NodeKind::Field,
                                    field_name.clone(),
                                    file_path.to_path_buf(),
                                    "swift".to_string(),
                                )
                                .with_position(start, end)
                                .with_qualified_name(format!("{}.{}", class_scope, field_name));

                                let field_idx = builder.add_node(field_node);
                                builder.add_edge(class_idx, field_idx, CodeEdge::new(EdgeKind::Contains));
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }

    /// Parse protocol declaration
    fn parse_protocol(
        &self,
        node: &Node,
        source: &str,
        file_path: &Path,
        builder: &mut CodeGraphBuilder,
        parent_scope: &str,
    ) -> Result<NodeIndex> {
        let mut name = String::new();

        if let Some(name_node) = node.child_by_field_name("name") {
            name = self.node_text(&name_node, source).to_string();
        }

        let qualified_name = if parent_scope.is_empty() {
            name.clone()
        } else {
            format!("{}.{}", parent_scope, name)
        };

        let (start, end) = self.get_position(node);
        let documentation = self.extract_documentation(node, source);

        let mut code_node = CodeNode::new(
            NodeKind::Trait,
            name.clone(),
            file_path.to_path_buf(),
            "swift".to_string(),
        )
        .with_position(start, end)
        .with_qualified_name(qualified_name.clone())
        .with_source(self.node_text(node, source).to_string());

        if let Some(doc) = documentation {
            code_node = code_node.with_doc(doc);
        }

        let protocol_idx = builder.add_node(code_node);

        Ok(protocol_idx)
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
            "swift".to_string(),
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
                "protocol_declaration" => {
                    self.parse_protocol(&child, source, file_path, builder, parent_scope)?;
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

impl Default for SwiftParser {
    fn default() -> Self {
        Self::new()
    }
}

impl LanguageParser for SwiftParser {
    fn language_name(&self) -> &str {
        "swift"
    }

    fn file_extensions(&self) -> &[&str] {
        &["swift"]
    }

    fn parse_file(&self, file_path: &Path, builder: &mut CodeGraphBuilder) -> Result<()> {
        let source = fs::read_to_string(file_path)
            .with_context(|| format!("Failed to read file: {}", file_path.display()))?;

        let mut parser = Parser::new();
        parser
            .set_language(tree_sitter_swift::language())
            .expect("Error loading Swift grammar");

        let tree = parser
            .parse(&source, None)
            .context("Failed to parse Swift source")?;

        let root_node = tree.root_node();

        let file_node = CodeNode::new(
            NodeKind::File,
            file_path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string(),
            file_path.to_path_buf(),
            "swift".to_string(),
        )
        .with_qualified_name(file_path.to_string_lossy().to_string());

        let _file_idx = builder.add_node(file_node);

        self.parse_items(&root_node, &source, file_path, builder, "")?;

        Ok(())
    }
}
