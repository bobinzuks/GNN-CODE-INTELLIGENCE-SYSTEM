//! GNN Experts module - Pluggable language-specific expert system
//!
//! This module provides a trait-based architecture for language experts that can:
//! - Detect language-specific patterns and anti-patterns
//! - Provide code quality analysis
//! - Suggest improvements with confidence scores
//! - Be dynamically loaded and registered

pub mod trait_;
pub mod registry;
pub mod experts;
pub mod patterns;

pub use trait_::{LanguageExpert, Pattern, PatternCategory, PatternMatch};
pub use registry::ExpertRegistry;
pub use experts::rust::RustExpert;
pub use patterns::{
    PatternDetector, PatternInstance, FixSuggestion,
    RustPatterns, PythonPatterns, JavaScriptPatterns, TypeScriptPatterns,
    GoPatterns, JavaPatterns, CppPatterns, CPatterns,
};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use petgraph::graph::{DiGraph, NodeIndex};

/// Re-export common types
pub type CodeGraph = DiGraph<CodeNode, CodeEdge>;
pub type NodeId = NodeIndex;

/// Code node in the graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeNode {
    pub kind: NodeKind,
    pub name: String,
    pub language: String,
    pub file_path: Option<String>,
    pub start_line: u32,
    pub end_line: u32,
    pub start_col: u32,
    pub end_col: u32,
    pub signature: Option<String>,
    pub visibility: Visibility,
    pub is_async: bool,
    pub is_static: bool,
    pub is_generic: bool,
    pub metadata: HashMap<String, String>,
}

/// Code edge in the graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeEdge {
    pub kind: EdgeKind,
    pub weight: f32,
    pub metadata: HashMap<String, String>,
}

/// Node kind enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NodeKind {
    File,
    Module,
    Function,
    Method,
    Class,
    Struct,
    Enum,
    Interface,
    Trait,
    Variable,
    Constant,
    Parameter,
    Field,
    Import,
    Export,
    Block,
    Loop,
    Conditional,
    Unknown,
}

/// Edge kind enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EdgeKind {
    Contains,
    Calls,
    HasType,
    Returns,
    Imports,
    DependsOn,
    Reads,
    Writes,
    ControlFlow,
}

/// Visibility level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Visibility {
    Private,
    Protected,
    Public,
    Internal,
}

/// Issue detected by an expert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Issue {
    pub pattern: String,
    pub severity: Severity,
    pub location: Location,
    pub message: String,
    pub suggested_fix: Option<String>,
}

/// Suggestion from an expert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Suggestion {
    pub pattern: String,
    pub description: String,
    pub confidence: f32,
    pub location: Option<Location>,
}

/// Severity level
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Severity {
    Info,
    Warning,
    Error,
    Critical,
}

/// Location in source code
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub file_path: String,
    pub start_line: u32,
    pub end_line: u32,
    pub start_col: u32,
    pub end_col: u32,
}

/// Expert output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpertOutput {
    pub language: String,
    pub issues: Vec<Issue>,
    pub suggestions: Vec<Suggestion>,
    pub confidence: f32,
    pub metadata: HashMap<String, String>,
}

impl ExpertOutput {
    /// Create a new expert output
    pub fn new(language: impl Into<String>) -> Self {
        Self {
            language: language.into(),
            issues: Vec::new(),
            suggestions: Vec::new(),
            confidence: 1.0,
            metadata: HashMap::new(),
        }
    }

    /// Add an issue
    pub fn add_issue(&mut self, issue: Issue) {
        self.issues.push(issue);
    }

    /// Add a suggestion
    pub fn add_suggestion(&mut self, suggestion: Suggestion) {
        self.suggestions.push(suggestion);
    }

    /// Set metadata
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expert_output_creation() {
        let output = ExpertOutput::new("rust");
        assert_eq!(output.language, "rust");
        assert_eq!(output.issues.len(), 0);
        assert_eq!(output.suggestions.len(), 0);
        assert_eq!(output.confidence, 1.0);
    }

    #[test]
    fn test_expert_output_with_metadata() {
        let output = ExpertOutput::new("rust")
            .with_metadata("version", "1.70");
        assert_eq!(output.metadata.get("version"), Some(&"1.70".to_string()));
    }
}
