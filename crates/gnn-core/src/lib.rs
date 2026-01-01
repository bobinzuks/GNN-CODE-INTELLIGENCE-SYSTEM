//! GNN Core - Pure Rust Graph Neural Network Implementation
//!
//! A WASM-compatible GNN library featuring:
//! - GraphSAGE and GAT layers for efficient large graph processing
//! - Contrastive learning for unsupervised training
//! - Semantic compression: codebase → 512-dim embedding
//! - Runtime inference engine with caching
//!
//! # Architecture
//!
//! ```text
//! Code Graph → Feature Extraction → GNN Layers → Pooling → Embedding (512-dim)
//!     ↓              ↓                  ↓            ↓          ↓
//!   AST          Structural         GraphSAGE    Attention   L2 Norm
//!  Nodes          Features            GAT        Pooling
//! ```
//!
//! # Example Usage
//!
//! ```rust,ignore
//! use gnn_core::*;
//! use rand::SeedableRng;
//! use rand::rngs::SmallRng;
//!
//! // Create a GNN model
//! let mut rng = SmallRng::seed_from_u64(42);
//! let config = model::GNNConfig {
//!     input_dim: 128,
//!     hidden_dims: vec![256, 256],
//!     output_dim: 512,
//!     num_heads: 4,
//!     dropout: 0.1,
//!     use_attention_pooling: true,
//!     aggregation: layers::AggregationType::Mean,
//! };
//!
//! let model = model::GNNModel::new_sage(config, &mut rng);
//!
//! // Create a semantic compressor
//! let compressor = compression::SemanticCompressor::new(
//!     model,
//!     compression::FeatureConfig::default(),
//!     512,
//! );
//!
//! // Compress a code graph
//! let graph = compression::examples::create_example_graph();
//! let embedding = compressor.compress(&graph).unwrap();
//! ```
//!
//! # WASM Support
//!
//! This crate is designed to work in both native and WASM environments.
//! All operations use pure Rust without platform-specific dependencies.

#![cfg_attr(not(test), warn(missing_docs))]

pub mod tensor;
pub mod layers;
pub mod model;
pub mod training;
pub mod compression;
pub mod inference;

// Re-export commonly used types
pub use tensor::Tensor;
pub use layers::{SAGELayer, GATLayer, AggregationType, AttentionPooling};
pub use model::{GNNModel, GNNConfig, PoolingLayer, PoolingType, GraphBatch};
pub use training::{Trainer, TrainingConfig, LossType, Optimizer, LRScheduler};
pub use compression::{
    SemanticCompressor, FeatureExtractor, FeatureConfig,
    CodeGraph, CodeNode, CodeElementType, CodebaseIndex,
};
pub use inference::{InferenceEngine, InferenceConfig, InferenceResult, SimilaritySearch};

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Get library information
pub fn info() -> LibraryInfo {
    LibraryInfo {
        name: "gnn-core".to_string(),
        version: VERSION.to_string(),
        description: "Pure Rust GNN implementation with GraphSAGE, training, and semantic compression".to_string(),
        features: vec![
            "WASM-compatible".to_string(),
            "GraphSAGE layers".to_string(),
            "GAT layers".to_string(),
            "Contrastive learning".to_string(),
            "Semantic compression".to_string(),
            "Inference engine".to_string(),
        ],
    }
}

/// Library information
#[derive(Debug, Clone)]
pub struct LibraryInfo {
    /// Library name
    pub name: String,
    /// Version string
    pub version: String,
    /// Description
    pub description: String,
    /// Feature list
    pub features: Vec<String>,
}

/// Prelude module for convenient imports
pub mod prelude {
    pub use crate::tensor::Tensor;
    pub use crate::layers::{SAGELayer, GATLayer, AggregationType};
    pub use crate::model::{GNNModel, GNNConfig};
    pub use crate::training::{Trainer, TrainingConfig, LossType};
    pub use crate::compression::{SemanticCompressor, CodeGraph, CodeElementType};
    pub use crate::inference::{InferenceEngine, InferenceConfig};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_library_info() {
        let info = info();
        assert_eq!(info.name, "gnn-core");
        assert!(!info.version.is_empty());
        assert!(!info.features.is_empty());
    }

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }
}
