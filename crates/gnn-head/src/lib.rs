//! GNN HEAD module - Orchestrator for routing to language experts
//!
//! This module provides the HeadGNN which acts as an orchestrator that:
//! - Routes code to appropriate language experts
//! - Weights multi-language projects
//! - Merges outputs from multiple experts

pub mod router;
pub mod merger;

use std::collections::{HashMap, HashSet};
use petgraph::graph::{DiGraph, NodeIndex};
use serde::{Deserialize, Serialize};

pub use router::{RouterModel, LanguageRoute};
pub use merger::{MergerModel, MergedOutput};

/// Type alias for code graph
pub type CodeGraph = DiGraph<CodeNode, CodeEdge>;
pub type NodeId = NodeIndex;

/// HeadGNN orchestrates the expert system
#[derive(Debug)]
pub struct HeadGNN {
    pub router: RouterModel,
    pub weighter: WeighterModel,
    pub merger: MergerModel,
}

impl HeadGNN {
    /// Create a new HeadGNN with default models
    pub fn new() -> Self {
        Self {
            router: RouterModel::new(),
            weighter: WeighterModel::new(),
            merger: MergerModel::new(),
        }
    }

    /// Route a project to appropriate language experts with weights
    pub fn route(&self, project: &ParsedProject) -> Vec<LanguageRoute> {
        self.router.route(project)
    }

    /// Merge outputs from multiple experts
    pub fn merge(&self, expert_outputs: Vec<ExpertOutput>) -> MergedOutput {
        self.merger.merge(expert_outputs)
    }

    /// Full pipeline: route -> weight -> merge
    pub fn process(&self, project: &ParsedProject, expert_outputs: Vec<ExpertOutput>) -> MergedOutput {
        let routes = self.route(project);
        let weighted_outputs = self.weighter.apply_weights(expert_outputs, &routes);
        self.merger.merge(weighted_outputs)
    }
}

impl Default for HeadGNN {
    fn default() -> Self {
        Self::new()
    }
}

/// WeighterModel applies routing weights to expert outputs
#[derive(Debug)]
pub struct WeighterModel {
    // Model parameters for weighting
}

impl WeighterModel {
    pub fn new() -> Self {
        Self {}
    }

    /// Apply routing weights to expert outputs
    pub fn apply_weights(&self, outputs: Vec<ExpertOutput>, routes: &[LanguageRoute]) -> Vec<ExpertOutput> {
        let route_map: HashMap<String, f32> = routes
            .iter()
            .map(|r| (r.language.clone(), r.weight))
            .collect();

        outputs
            .into_iter()
            .map(|mut output| {
                if let Some(&weight) = route_map.get(&output.language) {
                    output.confidence *= weight;
                }
                output
            })
            .collect()
    }
}

impl Default for WeighterModel {
    fn default() -> Self {
        Self::new()
    }
}

/// Parsed project structure
#[derive(Debug, Clone)]
pub struct ParsedProject {
    pub graph: CodeGraph,
    pub languages: HashSet<String>,
    pub file_count: u32,
    pub node_count: u32,
    pub edge_count: u32,
}

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

/// Output from a language expert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpertOutput {
    pub language: String,
    pub issues: Vec<Issue>,
    pub suggestions: Vec<Suggestion>,
    pub confidence: f32,
    pub metadata: HashMap<String, String>,
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_head_gnn_creation() {
        let head = HeadGNN::new();
        assert!(head.router.min_weight > 0.0);
    }

    #[test]
    fn test_weighter_applies_weights() {
        let weighter = WeighterModel::new();
        let routes = vec![
            LanguageRoute {
                language: "rust".to_string(),
                weight: 0.8,
            },
        ];

        let outputs = vec![ExpertOutput {
            language: "rust".to_string(),
            issues: vec![],
            suggestions: vec![],
            confidence: 1.0,
            metadata: HashMap::new(),
        }];

        let weighted = weighter.apply_weights(outputs, &routes);
        assert!((weighted[0].confidence - 0.8).abs() < 0.001);
    }
}
