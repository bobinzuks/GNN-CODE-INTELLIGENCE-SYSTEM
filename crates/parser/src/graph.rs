//! Universal graph schema for code representation
//!
//! This module defines the core data structures for representing code as a graph,
//! supporting multiple programming languages with a unified schema.

use petgraph::graph::{DiGraph, NodeIndex};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Universal node types that span across programming languages
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NodeKind {
    // Top-level entities
    Function,
    Method,
    Struct,
    Class,
    Enum,
    Trait,
    Interface,
    Module,
    Namespace,

    // Variables and constants
    Variable,
    Parameter,
    Field,
    Constant,

    // Imports and dependencies
    Import,
    Export,

    // Type-related
    TypeAlias,
    GenericParameter,

    // Control flow
    IfStatement,
    ForLoop,
    WhileLoop,
    MatchExpression,

    // Expressions
    FunctionCall,
    MethodCall,
    BinaryExpression,
    UnaryExpression,

    // Literals
    StringLiteral,
    NumberLiteral,
    BooleanLiteral,

    // Other
    Comment,
    Attribute,
    Annotation,
    File,
}

impl NodeKind {
    /// Check if this node kind represents a definition
    pub fn is_definition(&self) -> bool {
        matches!(
            self,
            NodeKind::Function
                | NodeKind::Method
                | NodeKind::Struct
                | NodeKind::Class
                | NodeKind::Enum
                | NodeKind::Trait
                | NodeKind::Interface
                | NodeKind::Variable
                | NodeKind::Constant
                | NodeKind::TypeAlias
        )
    }

    /// Check if this node kind represents a callable
    pub fn is_callable(&self) -> bool {
        matches!(self, NodeKind::Function | NodeKind::Method)
    }
}

/// Universal edge types representing relationships between code entities
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EdgeKind {
    // Structural relationships
    Contains,       // Parent-child relationship (e.g., function contains statements)
    Defines,        // Definition relationship

    // Call relationships
    Calls,          // Function/method call
    CallsStatic,    // Static method call

    // Type relationships
    HasType,        // Variable/parameter has a type
    Implements,     // Class implements interface/trait
    Extends,        // Class extends another class

    // Import relationships
    Imports,        // Import relationship
    Exports,        // Export relationship

    // Data flow
    Reads,          // Reads a variable
    Writes,         // Writes to a variable

    // Control flow
    ControlFlow,    // Sequential control flow

    // Documentation
    Documents,      // Comment/doc string relationship

    // Attributes
    Annotates,      // Annotation/attribute relationship
}

/// Universal code node with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeNode {
    /// Type of the node
    pub kind: NodeKind,

    /// Name or identifier of the node
    pub name: String,

    /// Fully qualified name (e.g., "module::struct::method")
    pub qualified_name: Option<String>,

    /// Source file path
    pub file_path: PathBuf,

    /// Start position in source (line, column)
    pub start_position: (usize, usize),

    /// End position in source (line, column)
    pub end_position: (usize, usize),

    /// Source code snippet
    pub source_text: Option<String>,

    /// Programming language
    pub language: String,

    /// Additional metadata
    pub metadata: HashMap<String, String>,

    /// Visibility (public, private, etc.)
    pub visibility: Option<String>,

    /// Type signature (for functions, variables, etc.)
    pub type_signature: Option<String>,

    /// Documentation/comments
    pub documentation: Option<String>,
}

impl CodeNode {
    /// Create a new code node with minimal information
    pub fn new(kind: NodeKind, name: String, file_path: PathBuf, language: String) -> Self {
        Self {
            kind,
            name,
            qualified_name: None,
            file_path,
            start_position: (0, 0),
            end_position: (0, 0),
            source_text: None,
            language,
            metadata: HashMap::new(),
            visibility: None,
            type_signature: None,
            documentation: None,
        }
    }

    /// Set position information
    pub fn with_position(mut self, start: (usize, usize), end: (usize, usize)) -> Self {
        self.start_position = start;
        self.end_position = end;
        self
    }

    /// Set source text
    pub fn with_source(mut self, source: String) -> Self {
        self.source_text = Some(source);
        self
    }

    /// Set qualified name
    pub fn with_qualified_name(mut self, qname: String) -> Self {
        self.qualified_name = Some(qname);
        self
    }

    /// Set visibility
    pub fn with_visibility(mut self, vis: String) -> Self {
        self.visibility = Some(vis);
        self
    }

    /// Set type signature
    pub fn with_type(mut self, type_sig: String) -> Self {
        self.type_signature = Some(type_sig);
        self
    }

    /// Set documentation
    pub fn with_doc(mut self, doc: String) -> Self {
        self.documentation = Some(doc);
        self
    }

    /// Add metadata
    pub fn add_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }
}

/// Universal code edge
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeEdge {
    /// Type of the edge
    pub kind: EdgeKind,

    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

impl CodeEdge {
    /// Create a new edge
    pub fn new(kind: EdgeKind) -> Self {
        Self {
            kind,
            metadata: HashMap::new(),
        }
    }

    /// Add metadata to edge
    pub fn add_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }
}

/// Code graph representation using petgraph
pub type CodeGraph = DiGraph<CodeNode, CodeEdge>;

/// Builder for constructing code graphs
#[derive(Debug)]
pub struct CodeGraphBuilder {
    graph: CodeGraph,
    /// Map from qualified name to node index for quick lookups
    node_map: HashMap<String, NodeIndex>,
}

impl CodeGraphBuilder {
    /// Create a new graph builder
    pub fn new() -> Self {
        Self {
            graph: DiGraph::new(),
            node_map: HashMap::new(),
        }
    }

    /// Add a node to the graph
    pub fn add_node(&mut self, node: CodeNode) -> NodeIndex {
        let index = self.graph.add_node(node.clone());

        // Store in map if we have a qualified name
        if let Some(ref qname) = node.qualified_name {
            self.node_map.insert(qname.clone(), index);
        }

        index
    }

    /// Add an edge between two nodes
    pub fn add_edge(&mut self, from: NodeIndex, to: NodeIndex, edge: CodeEdge) {
        self.graph.add_edge(from, to, edge);
    }

    /// Find a node by qualified name
    pub fn find_node(&self, qualified_name: &str) -> Option<NodeIndex> {
        self.node_map.get(qualified_name).copied()
    }

    /// Get a reference to a node
    pub fn get_node(&self, index: NodeIndex) -> Option<&CodeNode> {
        self.graph.node_weight(index)
    }

    /// Get a mutable reference to a node
    pub fn get_node_mut(&mut self, index: NodeIndex) -> Option<&mut CodeNode> {
        self.graph.node_weight_mut(index)
    }

    /// Build and return the final graph
    pub fn build(self) -> CodeGraph {
        self.graph
    }

    /// Get reference to the graph
    pub fn graph(&self) -> &CodeGraph {
        &self.graph
    }

    /// Get the number of nodes
    pub fn node_count(&self) -> usize {
        self.graph.node_count()
    }

    /// Get the number of edges
    pub fn edge_count(&self) -> usize {
        self.graph.edge_count()
    }
}

impl Default for CodeGraphBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_creation() {
        let node = CodeNode::new(
            NodeKind::Function,
            "test_func".to_string(),
            PathBuf::from("test.rs"),
            "rust".to_string(),
        )
        .with_position((10, 0), (20, 1))
        .with_visibility("pub".to_string());

        assert_eq!(node.kind, NodeKind::Function);
        assert_eq!(node.name, "test_func");
        assert_eq!(node.start_position, (10, 0));
        assert_eq!(node.visibility, Some("pub".to_string()));
    }

    #[test]
    fn test_graph_builder() {
        let mut builder = CodeGraphBuilder::new();

        let node1 = CodeNode::new(
            NodeKind::Function,
            "main".to_string(),
            PathBuf::from("main.rs"),
            "rust".to_string(),
        )
        .with_qualified_name("main".to_string());

        let node2 = CodeNode::new(
            NodeKind::FunctionCall,
            "println".to_string(),
            PathBuf::from("main.rs"),
            "rust".to_string(),
        );

        let idx1 = builder.add_node(node1);
        let idx2 = builder.add_node(node2);
        builder.add_edge(idx1, idx2, CodeEdge::new(EdgeKind::Calls));

        let graph = builder.build();
        assert_eq!(graph.node_count(), 2);
        assert_eq!(graph.edge_count(), 1);
    }
}
