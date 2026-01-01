//! JavaScript-compatible API wrappers
//!
//! This module provides clean type conversions and wrappers for exposing
//! Rust functionality to JavaScript in a WASM context.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

/// Severity levels for detected issues
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[wasm_bindgen]
pub enum Severity {
    Info = 0,
    Warning = 1,
    Error = 2,
    Critical = 3,
}

/// Code issue detected by the GNN
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Issue {
    pub pattern: String,
    pub severity: Severity,
    pub location: Location,
    pub message: String,
    pub suggested_fix: Option<String>,
    pub confidence: f32,
}

/// Location in source code
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub file_path: Option<String>,
    pub start_line: u32,
    pub end_line: u32,
    pub start_col: u32,
    pub end_col: u32,
}

/// Suggested fix for an issue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fix {
    pub issue_pattern: String,
    pub location: Location,
    pub original_code: String,
    pub fixed_code: String,
    pub description: String,
    pub confidence: f32,
}

/// Node in the code graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeNode {
    pub id: usize,
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

/// Types of code nodes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NodeKind {
    // Structural
    File,
    Module,
    Namespace,
    Package,

    // Definitions
    Function,
    Method,
    Constructor,
    Destructor,

    // Types
    Class,
    Struct,
    Enum,
    Interface,
    Trait,
    TypeAlias,
    Generic,

    // Data
    Variable,
    Constant,
    Parameter,
    Field,
    Property,

    // Dependencies
    Import,
    Export,

    // Control
    Block,
    Loop,
    Conditional,
    Match,
    TryCatch,

    // Other
    Comment,
    Annotation,
    Macro,
    Unknown,
}

/// Edge types in the code graph
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EdgeKind {
    // Structural
    Contains,
    BelongsTo,

    // Calls
    Calls,
    Constructs,

    // Types
    HasType,
    Returns,
    Inherits,
    Implements,

    // Dependencies
    Imports,
    Exports,
    DependsOn,

    // Data flow
    Reads,
    Writes,
    DataFlow,

    // Control flow
    ControlFlow,
    Branches,
    Loops,
    Throws,
    Catches,
}

/// Visibility modifiers
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub enum Visibility {
    #[default]
    Private,
    Protected,
    Public,
    Internal,
}

/// Edge in the code graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeEdge {
    pub from: usize,
    pub to: usize,
    pub kind: EdgeKind,
    pub weight: f32,
    pub metadata: HashMap<String, String>,
}

/// Simplified graph structure for WASM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeGraph {
    pub nodes: Vec<CodeNode>,
    pub edges: Vec<CodeEdge>,
    pub root_path: Option<String>,
    pub languages: Vec<String>,
}

impl CodeGraph {
    pub fn new() -> Self {
        CodeGraph {
            nodes: Vec::new(),
            edges: Vec::new(),
            root_path: None,
            languages: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node: CodeNode) -> usize {
        let id = self.nodes.len();
        self.nodes.push(node);
        id
    }

    pub fn add_edge(&mut self, edge: CodeEdge) {
        self.edges.push(edge);
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    pub fn get_node(&self, id: usize) -> Option<&CodeNode> {
        self.nodes.get(id)
    }

    pub fn neighbors(&self, node_id: usize) -> Vec<usize> {
        self.edges
            .iter()
            .filter(|e| e.from == node_id)
            .map(|e| e.to)
            .collect()
    }
}

/// Tensor for neural network operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tensor {
    pub data: Vec<f32>,
    pub shape: Vec<usize>,
}

impl Tensor {
    pub fn zeros(shape: &[usize]) -> Self {
        let size: usize = shape.iter().product();
        Self {
            data: vec![0.0; size],
            shape: shape.to_vec(),
        }
    }

    pub fn from_vec(data: Vec<f32>, shape: Vec<usize>) -> Self {
        assert_eq!(data.len(), shape.iter().product());
        Self { data, shape }
    }

    pub fn random(shape: &[usize], scale: f32) -> Self {
        let size: usize = shape.iter().product();
        let data: Vec<f32> = (0..size)
            .map(|_| (js_sys::Math::random() as f32 - 0.5) * scale * 2.0)
            .collect();
        Self {
            data,
            shape: shape.to_vec(),
        }
    }

    pub fn matmul(&self, other: &Tensor) -> Tensor {
        // Simple matrix multiplication for 2D tensors
        assert_eq!(self.shape.len(), 2);
        assert_eq!(other.shape.len(), 2);
        assert_eq!(self.shape[1], other.shape[0]);

        let m = self.shape[0];
        let n = other.shape[1];
        let k = self.shape[1];

        let mut result = vec![0.0; m * n];

        for i in 0..m {
            for j in 0..n {
                let mut sum = 0.0;
                for p in 0..k {
                    sum += self.data[i * k + p] * other.data[p * n + j];
                }
                result[i * n + j] = sum;
            }
        }

        Tensor {
            data: result,
            shape: vec![m, n],
        }
    }

    pub fn add(&self, other: &Tensor) -> Tensor {
        assert_eq!(self.shape, other.shape);
        Tensor {
            data: self.data.iter()
                .zip(other.data.iter())
                .map(|(a, b)| a + b)
                .collect(),
            shape: self.shape.clone(),
        }
    }

    pub fn relu(&self) -> Tensor {
        Tensor {
            data: self.data.iter().map(|&x| x.max(0.0)).collect(),
            shape: self.shape.clone(),
        }
    }

    pub fn sigmoid(&self) -> Tensor {
        Tensor {
            data: self.data.iter().map(|&x| 1.0 / (1.0 + (-x).exp())).collect(),
            shape: self.shape.clone(),
        }
    }

    pub fn mean(&self) -> f32 {
        self.data.iter().sum::<f32>() / self.data.len() as f32
    }

    pub fn l2_norm(&self) -> f32 {
        self.data.iter().map(|x| x * x).sum::<f32>().sqrt()
    }
}

/// Simple GNN model for WASM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GNNModel {
    pub layers: Vec<SAGELayer>,
    pub output_dim: usize,
}

impl GNNModel {
    pub fn new(input_dim: usize, hidden_dims: Vec<usize>, output_dim: usize) -> Self {
        let mut layers = Vec::new();
        let mut in_dim = input_dim;

        for &out_dim in &hidden_dims {
            layers.push(SAGELayer::new(in_dim, out_dim));
            in_dim = out_dim;
        }

        Self { layers, output_dim }
    }

    pub fn forward_graph(&self, graph: &CodeGraph, features: &HashMap<usize, Tensor>) -> Tensor {
        // Compute node embeddings
        let mut node_embeddings = Vec::new();

        for node_id in 0..graph.node_count() {
            if let Some(h) = self.forward_node(graph, node_id, features) {
                node_embeddings.push(h);
            }
        }

        // Pool to single graph embedding using mean pooling
        if node_embeddings.is_empty() {
            return Tensor::zeros(&[self.output_dim]);
        }

        let mut pooled = Tensor::zeros(&[node_embeddings[0].data.len()]);
        for embedding in &node_embeddings {
            pooled = pooled.add(embedding);
        }

        // Average
        Tensor {
            data: pooled.data.iter().map(|x| x / node_embeddings.len() as f32).collect(),
            shape: pooled.shape,
        }
    }

    fn forward_node(&self, graph: &CodeGraph, node_id: usize, features: &HashMap<usize, Tensor>) -> Option<Tensor> {
        let mut h = features.get(&node_id)?.clone();

        for layer in &self.layers {
            let neighbors: Vec<Tensor> = graph.neighbors(node_id)
                .iter()
                .take(10) // Sample max 10 neighbors
                .filter_map(|&n| features.get(&n).cloned())
                .collect();

            h = layer.forward(&h, &neighbors);
        }

        Some(h)
    }
}

/// GraphSAGE layer for WASM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SAGELayer {
    pub weight_self: Tensor,
    pub weight_neigh: Tensor,
    pub bias: Tensor,
    pub input_dim: usize,
    pub output_dim: usize,
}

impl SAGELayer {
    pub fn new(input_dim: usize, output_dim: usize) -> Self {
        Self {
            weight_self: Tensor::random(&[input_dim, output_dim], 0.1),
            weight_neigh: Tensor::random(&[input_dim, output_dim], 0.1),
            bias: Tensor::zeros(&[output_dim]),
            input_dim,
            output_dim,
        }
    }

    pub fn forward(&self, node_features: &Tensor, neighbor_features: &[Tensor]) -> Tensor {
        // Aggregate neighbors (mean)
        let agg_neighbors = if neighbor_features.is_empty() {
            Tensor::zeros(&[self.input_dim])
        } else {
            let mut sum = Tensor::zeros(&[self.input_dim]);
            for n in neighbor_features {
                sum = sum.add(n);
            }
            Tensor {
                data: sum.data.iter().map(|x| x / neighbor_features.len() as f32).collect(),
                shape: sum.shape,
            }
        };

        // Transform self and neighbor features
        let h_self = node_features.matmul(&self.weight_self);
        let h_neigh = agg_neighbors.matmul(&self.weight_neigh);

        // Combine and activate
        h_self.add(&h_neigh).add(&self.bias).relu()
    }
}
