//! Semantic compression for codebase representation
//! Transforms entire codebases into compact 512-dimensional embeddings

use crate::model::GNNModel;
use crate::tensor::Tensor;
use serde::{Deserialize, Serialize};

/// Feature extraction configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureConfig {
    pub node_feature_dim: usize,
    pub max_nodes: usize,
    pub include_structure: bool,
    pub include_types: bool,
    pub include_names: bool,
}

impl Default for FeatureConfig {
    fn default() -> Self {
        Self {
            node_feature_dim: 128,
            max_nodes: 10000,
            include_structure: true,
            include_types: true,
            include_names: true,
        }
    }
}

/// Code element types for feature extraction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CodeElementType {
    Function,
    Class,
    Method,
    Variable,
    Parameter,
    Import,
    Module,
    Statement,
    Expression,
    Comment,
    Unknown,
}

impl CodeElementType {
    /// Convert to one-hot encoding index
    pub fn to_index(&self) -> usize {
        match self {
            CodeElementType::Function => 0,
            CodeElementType::Class => 1,
            CodeElementType::Method => 2,
            CodeElementType::Variable => 3,
            CodeElementType::Parameter => 4,
            CodeElementType::Import => 5,
            CodeElementType::Module => 6,
            CodeElementType::Statement => 7,
            CodeElementType::Expression => 8,
            CodeElementType::Comment => 9,
            CodeElementType::Unknown => 10,
        }
    }

    /// Total number of element types
    pub fn num_types() -> usize {
        11
    }
}

/// Represents a node in the code graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeNode {
    pub id: usize,
    pub element_type: CodeElementType,
    pub name: String,
    pub type_annotation: Option<String>,
    pub depth: usize, // Nesting depth in AST
    pub line_number: usize,
}

/// Code graph representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeGraph {
    pub nodes: Vec<CodeNode>,
    pub edges: Vec<(usize, usize)>, // (source, target) pairs
    pub file_path: String,
}

impl CodeGraph {
    /// Create a new code graph
    pub fn new(file_path: String) -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
            file_path,
        }
    }

    /// Add a node to the graph
    pub fn add_node(&mut self, node: CodeNode) -> usize {
        let id = self.nodes.len();
        self.nodes.push(node);
        id
    }

    /// Add an edge between nodes
    pub fn add_edge(&mut self, source: usize, target: usize) {
        self.edges.push((source, target));
    }

    /// Get adjacency list representation
    pub fn get_adjacency_list(&self) -> Vec<(usize, Vec<usize>)> {
        let mut adjacency: Vec<Vec<usize>> = vec![Vec::new(); self.nodes.len()];

        for &(src, tgt) in &self.edges {
            if src < self.nodes.len() && tgt < self.nodes.len() {
                adjacency[src].push(tgt);
            }
        }

        adjacency
            .into_iter()
            .enumerate()
            .map(|(i, neighbors)| (i, neighbors))
            .collect()
    }
}

/// Feature extractor for code graphs
pub struct FeatureExtractor {
    pub config: FeatureConfig,
}

impl FeatureExtractor {
    /// Create a new feature extractor
    pub fn new(config: FeatureConfig) -> Self {
        Self { config }
    }

    /// Extract features from a code graph
    pub fn extract_features(&self, graph: &CodeGraph) -> Result<Tensor, String> {
        let num_nodes = graph.nodes.len().min(self.config.max_nodes);
        let feature_dim = self.config.node_feature_dim;

        let mut features = vec![0.0; num_nodes * feature_dim];

        for (i, node) in graph.nodes.iter().take(num_nodes).enumerate() {
            let node_features = self.extract_node_features(node)?;

            // Copy features into the main feature tensor
            let offset = i * feature_dim;
            let copy_len = node_features.len().min(feature_dim);
            features[offset..offset + copy_len]
                .copy_from_slice(&node_features[..copy_len]);
        }

        Tensor::from_vec(features, vec![num_nodes, feature_dim])
    }

    /// Extract features for a single node
    fn extract_node_features(&self, node: &CodeNode) -> Result<Vec<f32>, String> {
        let mut features = Vec::new();

        // One-hot encode element type
        if self.config.include_types {
            let num_types = CodeElementType::num_types();
            let mut type_encoding = vec![0.0; num_types];
            type_encoding[node.element_type.to_index()] = 1.0;
            features.extend(type_encoding);
        }

        // Encode structural features
        if self.config.include_structure {
            // Normalized depth (assume max depth of 20)
            features.push((node.depth as f32).min(20.0) / 20.0);

            // Normalized line number (assume max 1000 lines)
            features.push((node.line_number as f32).min(1000.0) / 1000.0);
        }

        // Name-based features (simplified - just length)
        if self.config.include_names {
            // Name length (normalized)
            let name_len = (node.name.len() as f32).min(50.0) / 50.0;
            features.push(name_len);

            // Has type annotation
            features.push(if node.type_annotation.is_some() { 1.0 } else { 0.0 });
        }

        // Pad or truncate to match feature_dim
        while features.len() < self.config.node_feature_dim {
            features.push(0.0);
        }
        features.truncate(self.config.node_feature_dim);

        Ok(features)
    }
}

/// Semantic compressor that transforms codebases into embeddings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticCompressor {
    pub model: GNNModel,
    pub feature_config: FeatureConfig,
    pub target_dim: usize,
}

impl SemanticCompressor {
    /// Create a new semantic compressor
    pub fn new(model: GNNModel, feature_config: FeatureConfig, target_dim: usize) -> Self {
        Self {
            model,
            feature_config,
            target_dim,
        }
    }

    /// Compress a code graph into a fixed-size embedding
    pub fn compress(&self, graph: &CodeGraph) -> Result<Tensor, String> {
        // Extract features from the graph
        let extractor = FeatureExtractor::new(self.feature_config.clone());
        let node_features = extractor.extract_features(graph)?;

        // Get adjacency list
        let adjacency = graph.get_adjacency_list();

        // Run through GNN model
        let embedding = self.model.forward(&node_features, &adjacency)?;

        // Ensure embedding is the target dimension
        if embedding.data.len() != self.target_dim {
            return Err(format!(
                "Model output dimension {} does not match target dimension {}",
                embedding.data.len(),
                self.target_dim
            ));
        }

        // L2 normalize the embedding
        embedding.l2_normalize(0, 1e-8)
    }

    /// Compress multiple code graphs (e.g., for an entire codebase)
    pub fn compress_codebase(&self, graphs: &[CodeGraph]) -> Result<Tensor, String> {
        if graphs.is_empty() {
            return Err("Cannot compress empty codebase".to_string());
        }

        // Compress each graph
        let mut embeddings = Vec::new();
        for graph in graphs {
            let embedding = self.compress(graph)?;
            embeddings.push(embedding);
        }

        // Aggregate embeddings (mean pooling)
        let mut aggregated = Tensor::zeros(vec![self.target_dim]);

        for embedding in &embeddings {
            for i in 0..self.target_dim {
                aggregated.data[i] += embedding.data[i];
            }
        }

        let num_graphs = embeddings.len() as f32;
        for i in 0..self.target_dim {
            aggregated.data[i] /= num_graphs;
        }

        // L2 normalize
        aggregated.l2_normalize(0, 1e-8)
    }

    /// Compute similarity between two code graphs
    pub fn similarity(&self, graph1: &CodeGraph, graph2: &CodeGraph) -> Result<f32, String> {
        let embedding1 = self.compress(graph1)?;
        let embedding2 = self.compress(graph2)?;

        // Cosine similarity
        let dot_product: f32 = embedding1
            .data
            .iter()
            .zip(embedding2.data.iter())
            .map(|(a, b)| a * b)
            .sum();

        let norm1: f32 = embedding1.data.iter().map(|x| x * x).sum::<f32>().sqrt();
        let norm2: f32 = embedding2.data.iter().map(|x| x * x).sum::<f32>().sqrt();

        if norm1 < 1e-8 || norm2 < 1e-8 {
            return Ok(0.0);
        }

        Ok(dot_product / (norm1 * norm2))
    }
}

/// Codebase index for efficient similarity search
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodebaseIndex {
    pub embeddings: Vec<Tensor>,
    pub graph_paths: Vec<String>,
    pub dimension: usize,
}

impl CodebaseIndex {
    /// Create a new codebase index
    pub fn new(dimension: usize) -> Self {
        Self {
            embeddings: Vec::new(),
            graph_paths: Vec::new(),
            dimension,
        }
    }

    /// Add a graph embedding to the index
    pub fn add(&mut self, embedding: Tensor, path: String) -> Result<(), String> {
        if embedding.data.len() != self.dimension {
            return Err(format!(
                "Embedding dimension {} does not match index dimension {}",
                embedding.data.len(),
                self.dimension
            ));
        }

        self.embeddings.push(embedding);
        self.graph_paths.push(path);
        Ok(())
    }

    /// Find k most similar graphs to a query embedding
    pub fn search(&self, query: &Tensor, k: usize) -> Result<Vec<(usize, f32, String)>, String> {
        if query.data.len() != self.dimension {
            return Err(format!(
                "Query dimension {} does not match index dimension {}",
                query.data.len(),
                self.dimension
            ));
        }

        // Compute similarities
        let mut similarities: Vec<(usize, f32)> = self
            .embeddings
            .iter()
            .enumerate()
            .map(|(i, emb)| {
                let dot_product: f32 = query
                    .data
                    .iter()
                    .zip(emb.data.iter())
                    .map(|(a, b)| a * b)
                    .sum();

                let norm_query: f32 = query.data.iter().map(|x| x * x).sum::<f32>().sqrt();
                let norm_emb: f32 = emb.data.iter().map(|x| x * x).sum::<f32>().sqrt();

                let similarity = if norm_query < 1e-8 || norm_emb < 1e-8 {
                    0.0
                } else {
                    dot_product / (norm_query * norm_emb)
                };

                (i, similarity)
            })
            .collect();

        // Sort by similarity (descending)
        similarities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        // Take top k
        let results: Vec<(usize, f32, String)> = similarities
            .iter()
            .take(k.min(self.embeddings.len()))
            .map(|(idx, sim)| (*idx, *sim, self.graph_paths[*idx].clone()))
            .collect();

        Ok(results)
    }

    /// Get total number of indexed graphs
    pub fn size(&self) -> usize {
        self.embeddings.len()
    }
}

/// Utility for creating example code graphs (for testing)
pub mod examples {
    use super::*;

    /// Create a simple example code graph
    pub fn create_example_graph() -> CodeGraph {
        let mut graph = CodeGraph::new("example.rs".to_string());

        // Add nodes
        let fn_node = CodeNode {
            id: 0,
            element_type: CodeElementType::Function,
            name: "main".to_string(),
            type_annotation: Some("fn() -> ()".to_string()),
            depth: 0,
            line_number: 1,
        };
        graph.add_node(fn_node);

        let var_node = CodeNode {
            id: 1,
            element_type: CodeElementType::Variable,
            name: "x".to_string(),
            type_annotation: Some("i32".to_string()),
            depth: 1,
            line_number: 2,
        };
        graph.add_node(var_node);

        let stmt_node = CodeNode {
            id: 2,
            element_type: CodeElementType::Statement,
            name: "println".to_string(),
            type_annotation: None,
            depth: 1,
            line_number: 3,
        };
        graph.add_node(stmt_node);

        // Add edges (control flow / data dependencies)
        graph.add_edge(0, 1); // main -> x
        graph.add_edge(0, 2); // main -> println
        graph.add_edge(1, 2); // x -> println (data dependency)

        graph
    }

    /// Create a class-based example graph
    pub fn create_class_graph() -> CodeGraph {
        let mut graph = CodeGraph::new("example.py".to_string());

        let class_node = CodeNode {
            id: 0,
            element_type: CodeElementType::Class,
            name: "MyClass".to_string(),
            type_annotation: None,
            depth: 0,
            line_number: 1,
        };
        graph.add_node(class_node);

        let method_node = CodeNode {
            id: 1,
            element_type: CodeElementType::Method,
            name: "process".to_string(),
            type_annotation: Some("def(self, data: str) -> str".to_string()),
            depth: 1,
            line_number: 3,
        };
        graph.add_node(method_node);

        let param_node = CodeNode {
            id: 2,
            element_type: CodeElementType::Parameter,
            name: "data".to_string(),
            type_annotation: Some("str".to_string()),
            depth: 2,
            line_number: 3,
        };
        graph.add_node(param_node);

        graph.add_edge(0, 1); // class -> method
        graph.add_edge(1, 2); // method -> parameter

        graph
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::GNNConfig;
    use rand::SeedableRng;
    use rand::rngs::SmallRng;

    #[test]
    fn test_code_graph_creation() {
        let graph = examples::create_example_graph();
        assert_eq!(graph.nodes.len(), 3);
        assert_eq!(graph.edges.len(), 3);
    }

    #[test]
    fn test_feature_extraction() {
        let config = FeatureConfig::default();
        let extractor = FeatureExtractor::new(config);
        let graph = examples::create_example_graph();

        let features = extractor.extract_features(&graph).unwrap();
        assert_eq!(features.shape, vec![3, 128]);
    }

    #[test]
    fn test_codebase_index() {
        let mut index = CodebaseIndex::new(512);

        let embedding1 = Tensor::ones(vec![512]);
        let embedding2 = Tensor::zeros(vec![512]);

        index.add(embedding1.clone(), "file1.rs".to_string()).unwrap();
        index.add(embedding2, "file2.rs".to_string()).unwrap();

        assert_eq!(index.size(), 2);

        let results = index.search(&embedding1, 1).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].2, "file1.rs");
    }
}
