//! Python language parser implementation
//!
//! This module provides a comprehensive parser for Python source code,
//! extracting classes, functions, decorators, imports, comprehensions, and their relationships.

use crate::graph::{CodeEdge, CodeGraph, CodeGraphBuilder, CodeNode, EdgeKind, NodeKind};
use crate::languages::LanguageParser;
use anyhow::{Context, Result};
use petgraph::graph::NodeIndex;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use tree_sitter::{Node, Parser, Tree};

/// Python language parser
pub struct PythonParser {
    parser: Parser,
}

impl PythonParser {
    /// Create a new Python parser
    pub fn new() -> Self {
        let mut parser = Parser::new();
        parser
            .set_language(tree_sitter_python::language())
            .expect("Error loading Python grammar");

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

    /// Extract documentation from docstring
    fn extract_documentation(&self, node: &Node, source: &str) -> Option<String> {
        // Look for first child expression_statement with string
        for child in node.children(&mut node.walk()) {
            if child.kind() == "expression_statement" {
                for expr_child in child.children(&mut child.walk()) {
                    if expr_child.kind() == "string" {
                        let text = self.node_text(&expr_child, source);
                        // Remove quotes and clean up
                        let cleaned = text
                            .trim_start_matches("\"\"\"")
                            .trim_start_matches("'''")
                            .trim_end_matches("\"\"\"")
                            .trim_end_matches("'''")
                            .trim();
                        if !cleaned.is_empty() {
                            return Some(cleaned.to_string());
                        }
                    }
                }
            }
        }
        None
    }

    /// Extract decorators from a node
    fn extract_decorators(&self, node: &Node, source: &str) -> Vec<String> {
        let mut decorators = Vec::new();

        // Look for decorator siblings before this node
        let mut current = node.prev_sibling();
        while let Some(sibling) = current {
            if sibling.kind() == "decorator" {
                let dec_text = self.node_text(&sibling, source).to_string();
                decorators.push(dec_text);
                current = sibling.prev_sibling();
            } else {
                break;
            }
        }

        decorators.reverse();
        decorators
    }

    /// Parse function definitions
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

        // Get function name
        if let Some(name_node) = node.child_by_field_name("name") {
            name = self.node_text(&name_node, source).to_string();
        }

        // Get parameters
        if let Some(params_node) = node.child_by_field_name("parameters") {
            for param in params_node.children(&mut params_node.walk()) {
                if param.kind() == "identifier" || param.kind() == "typed_parameter" || param.kind() == "default_parameter" {
                    parameters.push(self.node_text(&param, source).to_string());
                }
            }
        }

        // Get return type if present
        if let Some(return_node) = node.child_by_field_name("return_type") {
            return_type = Some(self.node_text(&return_node, source).to_string());
        }

        let qualified_name = if parent_scope.is_empty() {
            name.clone()
        } else {
            format!("{}.{}", parent_scope, name)
        };

        let (start, end) = self.get_position(node);
        let decorators = self.extract_decorators(node, source);

        // Get documentation from function body
        let documentation = if let Some(body) = node.child_by_field_name("body") {
            self.extract_documentation(&body, source)
        } else {
            None
        };

        let type_signature = if !parameters.is_empty() || return_type.is_some() {
            let params_str = parameters.join(", ");
            let ret_str = return_type.unwrap_or_else(|| "None".to_string());
            Some(format!("def ({}) -> {}", params_str, ret_str))
        } else {
            None
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
            "python".to_string(),
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

        // Add decorators as metadata
        if !decorators.is_empty() {
            code_node = code_node.add_metadata("decorators".to_string(), decorators.join(","));
        }

        let func_idx = builder.add_node(code_node);

        // Parse function body for calls and control flow
        if let Some(body) = node.child_by_field_name("body") {
            self.parse_function_body(&body, source, file_path, builder, func_idx, &qualified_name)?;
        }

        Ok(func_idx)
    }

    /// Parse function body to extract calls, control flow, and data flow
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
                "call" => {
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
                "assignment" => {
                    self.parse_assignment(&child, source, file_path, builder, parent_idx)?;
                }
                "list_comprehension" | "dictionary_comprehension" | "set_comprehension" => {
                    self.parse_comprehension(&child, source, file_path, builder, parent_idx)?;
                }
                _ => {
                    // Recursively search
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
                "python".to_string(),
            )
            .with_position(start, end);

            let call_idx = builder.add_node(call_node);
            builder.add_edge(caller_idx, call_idx, CodeEdge::new(EdgeKind::Calls));

            // Try to resolve the call to a definition
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
            "python".to_string(),
        )
        .with_position(start, end);

        let if_idx = builder.add_node(if_node);
        builder.add_edge(parent_idx, if_idx, CodeEdge::new(EdgeKind::Contains));

        // Parse consequence and alternative
        if let Some(consequence) = node.child_by_field_name("consequence") {
            self.parse_function_body(&consequence, source, file_path, builder, if_idx, "")?;
        }
        if let Some(alternative) = node.child_by_field_name("alternative") {
            self.parse_function_body(&alternative, source, file_path, builder, if_idx, "")?;
        }

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
            "python".to_string(),
        )
        .with_position(start, end);

        let for_idx = builder.add_node(for_node);
        builder.add_edge(parent_idx, for_idx, CodeEdge::new(EdgeKind::Contains));

        // Parse body
        if let Some(body) = node.child_by_field_name("body") {
            self.parse_function_body(&body, source, file_path, builder, for_idx, "")?;
        }

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
            "python".to_string(),
        )
        .with_position(start, end);

        let while_idx = builder.add_node(while_node);
        builder.add_edge(parent_idx, while_idx, CodeEdge::new(EdgeKind::Contains));

        // Parse body
        if let Some(body) = node.child_by_field_name("body") {
            self.parse_function_body(&body, source, file_path, builder, while_idx, "")?;
        }

        Ok(())
    }

    /// Parse assignment for data flow tracking
    fn parse_assignment(
        &self,
        node: &Node,
        source: &str,
        file_path: &Path,
        builder: &mut CodeGraphBuilder,
        parent_idx: NodeIndex,
    ) -> Result<()> {
        if let Some(left) = node.child_by_field_name("left") {
            let var_name = self.node_text(&left, source).to_string();
            let (start, end) = self.get_position(node);

            let var_node = CodeNode::new(
                NodeKind::Variable,
                var_name.clone(),
                file_path.to_path_buf(),
                "python".to_string(),
            )
            .with_position(start, end);

            let var_idx = builder.add_node(var_node);
            builder.add_edge(parent_idx, var_idx, CodeEdge::new(EdgeKind::Writes));
        }

        Ok(())
    }

    /// Parse comprehension expressions
    fn parse_comprehension(
        &self,
        node: &Node,
        source: &str,
        file_path: &Path,
        builder: &mut CodeGraphBuilder,
        parent_idx: NodeIndex,
    ) -> Result<()> {
        // Recursively parse comprehension body
        self.parse_function_body(node, source, file_path, builder, parent_idx, "")?;
        Ok(())
    }

    /// Parse class definitions
    fn parse_class(
        &self,
        node: &Node,
        source: &str,
        file_path: &Path,
        builder: &mut CodeGraphBuilder,
        parent_scope: &str,
    ) -> Result<NodeIndex> {
        let mut name = String::new();
        let mut base_classes = Vec::new();

        // Get class name
        if let Some(name_node) = node.child_by_field_name("name") {
            name = self.node_text(&name_node, source).to_string();
        }

        // Get base classes
        if let Some(superclasses) = node.child_by_field_name("superclasses") {
            for base in superclasses.children(&mut superclasses.walk()) {
                if base.kind() == "identifier" || base.kind() == "attribute" {
                    base_classes.push(self.node_text(&base, source).to_string());
                }
            }
        }

        let qualified_name = if parent_scope.is_empty() {
            name.clone()
        } else {
            format!("{}.{}", parent_scope, name)
        };

        let (start, end) = self.get_position(node);
        let decorators = self.extract_decorators(node, source);

        // Get documentation from class body
        let documentation = if let Some(body) = node.child_by_field_name("body") {
            self.extract_documentation(&body, source)
        } else {
            None
        };

        let mut code_node = CodeNode::new(
            NodeKind::Class,
            name.clone(),
            file_path.to_path_buf(),
            "python".to_string(),
        )
        .with_position(start, end)
        .with_qualified_name(qualified_name.clone())
        .with_source(self.node_text(node, source).to_string());

        if let Some(doc) = documentation {
            code_node = code_node.with_doc(doc);
        }

        if !decorators.is_empty() {
            code_node = code_node.add_metadata("decorators".to_string(), decorators.join(","));
        }

        if !base_classes.is_empty() {
            code_node = code_node.add_metadata("base_classes".to_string(), base_classes.join(","));
        }

        let class_idx = builder.add_node(code_node);

        // Add inheritance edges
        for base in &base_classes {
            if let Some(base_idx) = builder.find_node(base) {
                builder.add_edge(class_idx, base_idx, CodeEdge::new(EdgeKind::Extends));
            }
        }

        // Parse class body for methods and fields
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
                "function_definition" => {
                    let method_idx = self.parse_function(&child, source, file_path, builder, class_scope, true)?;
                    builder.add_edge(class_idx, method_idx, CodeEdge::new(EdgeKind::Contains));
                }
                "assignment" => {
                    // Class field
                    if let Some(left) = child.child_by_field_name("left") {
                        let field_name = self.node_text(&left, source).to_string();
                        let (start, end) = self.get_position(&child);

                        let field_node = CodeNode::new(
                            NodeKind::Field,
                            field_name.clone(),
                            file_path.to_path_buf(),
                            "python".to_string(),
                        )
                        .with_position(start, end)
                        .with_qualified_name(format!("{}.{}", class_scope, field_name));

                        let field_idx = builder.add_node(field_node);
                        builder.add_edge(class_idx, field_idx, CodeEdge::new(EdgeKind::Contains));
                    }
                }
                _ => {}
            }
        }

        Ok(())
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
            "python".to_string(),
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
                "function_definition" => {
                    self.parse_function(&child, source, file_path, builder, parent_scope, false)?;
                }
                "class_definition" => {
                    self.parse_class(&child, source, file_path, builder, parent_scope)?;
                }
                "import_statement" | "import_from_statement" => {
                    self.parse_import(&child, source, file_path, builder)?;
                }
                "module" | "block" => {
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

impl Default for PythonParser {
    fn default() -> Self {
        Self::new()
    }
}

impl LanguageParser for PythonParser {
    fn language_name(&self) -> &str {
        "python"
    }

    fn file_extensions(&self) -> &[&str] {
        &["py", "pyi", "pyw"]
    }

    fn parse_file(&self, file_path: &Path, builder: &mut CodeGraphBuilder) -> Result<()> {
        let source = fs::read_to_string(file_path)
            .with_context(|| format!("Failed to read file: {}", file_path.display()))?;

        let mut parser = Parser::new();
        parser
            .set_language(tree_sitter_python::language())
            .expect("Error loading Python grammar");

        let tree = parser
            .parse(&source, None)
            .context("Failed to parse Python source")?;

        let root_node = tree.root_node();

        let file_node = CodeNode::new(
            NodeKind::File,
            file_path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string(),
            file_path.to_path_buf(),
            "python".to_string(),
        )
        .with_qualified_name(file_path.to_string_lossy().to_string());

        let _file_idx = builder.add_node(file_node);

        self.parse_items(&root_node, &source, file_path, builder, "")?;

        Ok(())
    }
}
