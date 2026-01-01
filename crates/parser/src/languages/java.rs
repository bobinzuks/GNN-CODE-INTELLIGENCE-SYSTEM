//! Java language parser implementation
//!
//! This module provides a comprehensive parser for Java source code,
//! extracting classes, methods, annotations, generics, streams, and their relationships.

use crate::graph::{CodeEdge, CodeGraph, CodeGraphBuilder, CodeNode, EdgeKind, NodeKind};
use crate::languages::LanguageParser;
use anyhow::{Context, Result};
use petgraph::graph::NodeIndex;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use tree_sitter::{Node, Parser, Tree};

/// Java language parser
pub struct JavaParser {
    parser: Parser,
}

impl JavaParser {
    /// Create a new Java parser
    pub fn new() -> Self {
        let mut parser = Parser::new();
        parser
            .set_language(tree_sitter_java::language())
            .expect("Error loading Java grammar");

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

    /// Extract Javadoc documentation
    fn extract_documentation(&self, node: &Node, source: &str) -> Option<String> {
        let mut docs = Vec::new();

        if let Some(prev_sibling) = node.prev_sibling() {
            if prev_sibling.kind() == "block_comment" || prev_sibling.kind() == "line_comment" {
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

    /// Extract annotations
    fn extract_annotations(&self, node: &Node, source: &str) -> Vec<String> {
        let mut annotations = Vec::new();

        let mut current = node.prev_sibling();
        while let Some(sibling) = current {
            if sibling.kind() == "annotation" || sibling.kind() == "marker_annotation" {
                annotations.push(self.node_text(&sibling, source).to_string());
                current = sibling.prev_sibling();
            } else {
                break;
            }
        }

        annotations.reverse();
        annotations
    }

    /// Extract modifiers
    fn extract_modifiers(&self, node: &Node, source: &str) -> Vec<String> {
        let mut modifiers = Vec::new();

        for child in node.children(&mut node.walk()) {
            if child.kind() == "modifiers" {
                for modifier in child.children(&mut child.walk()) {
                    modifiers.push(self.node_text(&modifier, source).to_string());
                }
            }
        }

        modifiers
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

    /// Parse method declaration
    fn parse_method(
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
        let mut generic_params = None;

        // Get method name
        if let Some(name_node) = node.child_by_field_name("name") {
            name = self.node_text(&name_node, source).to_string();
        }

        // Get generic parameters
        generic_params = self.extract_generic_parameters(node, source);

        // Get parameters
        if let Some(params_node) = node.child_by_field_name("parameters") {
            for param in params_node.children(&mut params_node.walk()) {
                if param.kind() == "formal_parameter" {
                    parameters.push(self.node_text(&param, source).to_string());
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
        let annotations = self.extract_annotations(node, source);
        let modifiers = self.extract_modifiers(node, source);

        let type_signature = {
            let params_str = parameters.join(", ");
            let ret_str = return_type.unwrap_or_else(|| "void".to_string());
            let generic_str = generic_params.unwrap_or_default();
            let modifiers_str = if !modifiers.is_empty() {
                format!("{} ", modifiers.join(" "))
            } else {
                String::new()
            };
            Some(format!("{}{}{} {}({})", modifiers_str, generic_str, ret_str, name, params_str))
        };

        let mut code_node = CodeNode::new(
            NodeKind::Method,
            name.clone(),
            file_path.to_path_buf(),
            "java".to_string(),
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

        if !annotations.is_empty() {
            code_node = code_node.add_metadata("annotations".to_string(), annotations.join(","));
        }

        if !modifiers.is_empty() {
            code_node = code_node.add_metadata("modifiers".to_string(), modifiers.join(","));
        }

        let method_idx = builder.add_node(code_node);

        // Parse method body
        if let Some(body) = node.child_by_field_name("body") {
            self.parse_method_body(&body, source, file_path, builder, method_idx, &qualified_name)?;
        }

        Ok(method_idx)
    }

    /// Parse method body
    fn parse_method_body(
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
                "method_invocation" => {
                    self.parse_method_invocation(&child, source, file_path, builder, parent_idx)?;
                }
                "if_statement" => {
                    self.parse_if_statement(&child, source, file_path, builder, parent_idx)?;
                }
                "for_statement" | "enhanced_for_statement" => {
                    self.parse_for_loop(&child, source, file_path, builder, parent_idx)?;
                }
                "while_statement" => {
                    self.parse_while_loop(&child, source, file_path, builder, parent_idx)?;
                }
                "local_variable_declaration" => {
                    self.parse_variable_declaration(&child, source, file_path, builder, parent_idx)?;
                }
                "lambda_expression" => {
                    self.parse_lambda(&child, source, file_path, builder, parent_idx)?;
                }
                _ => {
                    self.parse_method_body(&child, source, file_path, builder, parent_idx, parent_scope)?;
                }
            }
        }

        Ok(())
    }

    /// Parse method invocation
    fn parse_method_invocation(
        &self,
        node: &Node,
        source: &str,
        file_path: &Path,
        builder: &mut CodeGraphBuilder,
        caller_idx: NodeIndex,
    ) -> Result<()> {
        if let Some(name_node) = node.child_by_field_name("name") {
            let call_name = self.node_text(&name_node, source).to_string();
            let (start, end) = self.get_position(node);

            let call_node = CodeNode::new(
                NodeKind::MethodCall,
                call_name.clone(),
                file_path.to_path_buf(),
                "java".to_string(),
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
            "java".to_string(),
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
            "java".to_string(),
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
            "java".to_string(),
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

                    // Get type from parent
                    let var_type = if let Some(type_node) = node.child_by_field_name("type") {
                        Some(self.node_text(&type_node, source).to_string())
                    } else {
                        None
                    };

                    let mut var_node = CodeNode::new(
                        NodeKind::Variable,
                        var_name.clone(),
                        file_path.to_path_buf(),
                        "java".to_string(),
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

    /// Parse lambda expression
    fn parse_lambda(
        &self,
        node: &Node,
        source: &str,
        file_path: &Path,
        builder: &mut CodeGraphBuilder,
        parent_idx: NodeIndex,
    ) -> Result<()> {
        let (start, end) = self.get_position(node);

        let lambda_node = CodeNode::new(
            NodeKind::Function,
            "<lambda>".to_string(),
            file_path.to_path_buf(),
            "java".to_string(),
        )
        .with_position(start, end)
        .add_metadata("lambda".to_string(), "true".to_string());

        let lambda_idx = builder.add_node(lambda_node);
        builder.add_edge(parent_idx, lambda_idx, CodeEdge::new(EdgeKind::Contains));

        // Parse lambda body
        if let Some(body) = node.child_by_field_name("body") {
            self.parse_method_body(&body, source, file_path, builder, lambda_idx, "")?;
        }

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
        let mut interfaces = Vec::new();
        let mut generic_params = None;

        // Get class name
        if let Some(name_node) = node.child_by_field_name("name") {
            name = self.node_text(&name_node, source).to_string();
        }

        // Get generic parameters
        generic_params = self.extract_generic_parameters(node, source);

        // Get superclass
        if let Some(superclass_node) = node.child_by_field_name("superclass") {
            superclass = Some(self.node_text(&superclass_node, source).to_string());
        }

        // Get interfaces
        if let Some(interfaces_node) = node.child_by_field_name("interfaces") {
            for interface in interfaces_node.children(&mut interfaces_node.walk()) {
                if interface.kind() == "type_identifier" {
                    interfaces.push(self.node_text(&interface, source).to_string());
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
        let annotations = self.extract_annotations(node, source);
        let modifiers = self.extract_modifiers(node, source);

        let mut code_node = CodeNode::new(
            NodeKind::Class,
            name.clone(),
            file_path.to_path_buf(),
            "java".to_string(),
        )
        .with_position(start, end)
        .with_qualified_name(qualified_name.clone())
        .with_source(self.node_text(node, source).to_string());

        if let Some(doc) = documentation {
            code_node = code_node.with_doc(doc);
        }

        if !annotations.is_empty() {
            code_node = code_node.add_metadata("annotations".to_string(), annotations.join(","));
        }

        if !modifiers.is_empty() {
            code_node = code_node.add_metadata("modifiers".to_string(), modifiers.join(","));
        }

        if let Some(generic_str) = generic_params {
            code_node = code_node.add_metadata("generics".to_string(), generic_str);
        }

        if let Some(ref superclass_str) = superclass {
            code_node = code_node.add_metadata("extends".to_string(), superclass_str.clone());
        }

        if !interfaces.is_empty() {
            code_node = code_node.add_metadata("implements".to_string(), interfaces.join(","));
        }

        let class_idx = builder.add_node(code_node);

        // Add extends edge
        if let Some(superclass_str) = superclass {
            if let Some(super_idx) = builder.find_node(&superclass_str) {
                builder.add_edge(class_idx, super_idx, CodeEdge::new(EdgeKind::Extends));
            }
        }

        // Add implements edges
        for interface_name in &interfaces {
            if let Some(interface_idx) = builder.find_node(interface_name) {
                builder.add_edge(class_idx, interface_idx, CodeEdge::new(EdgeKind::Implements));
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
                "method_declaration" | "constructor_declaration" => {
                    let method_idx = self.parse_method(&child, source, file_path, builder, class_scope)?;
                    builder.add_edge(class_idx, method_idx, CodeEdge::new(EdgeKind::Contains));
                }
                "field_declaration" => {
                    for field_child in child.children(&mut child.walk()) {
                        if field_child.kind() == "variable_declarator" {
                            if let Some(name_node) = field_child.child_by_field_name("name") {
                                let field_name = self.node_text(&name_node, source).to_string();
                                let (start, end) = self.get_position(&field_child);

                                let field_node = CodeNode::new(
                                    NodeKind::Field,
                                    field_name.clone(),
                                    file_path.to_path_buf(),
                                    "java".to_string(),
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

    /// Parse interface declaration
    fn parse_interface(
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
            NodeKind::Interface,
            name.clone(),
            file_path.to_path_buf(),
            "java".to_string(),
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
            "java".to_string(),
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
                "class_declaration" => {
                    self.parse_class(&child, source, file_path, builder, parent_scope)?;
                }
                "interface_declaration" => {
                    self.parse_interface(&child, source, file_path, builder, parent_scope)?;
                }
                "import_declaration" => {
                    self.parse_import(&child, source, file_path, builder)?;
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

impl Default for JavaParser {
    fn default() -> Self {
        Self::new()
    }
}

impl LanguageParser for JavaParser {
    fn language_name(&self) -> &str {
        "java"
    }

    fn file_extensions(&self) -> &[&str] {
        &["java"]
    }

    fn parse_file(&self, file_path: &Path, builder: &mut CodeGraphBuilder) -> Result<()> {
        let source = fs::read_to_string(file_path)
            .with_context(|| format!("Failed to read file: {}", file_path.display()))?;

        let mut parser = Parser::new();
        parser
            .set_language(tree_sitter_java::language())
            .expect("Error loading Java grammar");

        let tree = parser
            .parse(&source, None)
            .context("Failed to parse Java source")?;

        let root_node = tree.root_node();

        let file_node = CodeNode::new(
            NodeKind::File,
            file_path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string(),
            file_path.to_path_buf(),
            "java".to_string(),
        )
        .with_qualified_name(file_path.to_string_lossy().to_string());

        let _file_idx = builder.add_node(file_node);

        self.parse_items(&root_node, &source, file_path, builder, "")?;

        Ok(())
    }
}
