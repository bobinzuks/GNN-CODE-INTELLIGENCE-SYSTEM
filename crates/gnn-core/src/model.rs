//! GNN model architecture
//! Combines layers into a complete graph neural network

use crate::layers::{SAGELayer, GATLayer, AttentionPooling, AggregationType};
use crate::tensor::Tensor;
use serde::{Deserialize, Serialize};
use rand::Rng;

/// Layer type enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LayerType {
    SAGE(SAGELayer),
    GAT(GATLayer),
}

impl LayerType {
    pub fn forward(
        &self,
        features: &Tensor,
        adjacency: &[(usize, Vec<usize>)],
    ) -> Result<Tensor, String> {
        match self {
            LayerType::SAGE(layer) => layer.forward(features, adjacency),
            LayerType::GAT(layer) => layer.forward(features, adjacency),
        }
    }

    pub fn out_features(&self) -> usize {
        match self {
            LayerType::SAGE(layer) => layer.out_features,
            LayerType::GAT(layer) => layer.out_features * layer.num_heads,
        }
    }
}

/// GNN Model configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GNNConfig {
    pub input_dim: usize,
    pub hidden_dims: Vec<usize>,
    pub output_dim: usize,
    pub num_heads: usize, // For GAT layers
    pub dropout: f32,
    pub use_attention_pooling: bool,
    pub aggregation: AggregationType,
}

impl Default for GNNConfig {
    fn default() -> Self {
        Self {
            input_dim: 128,
            hidden_dims: vec![256, 256],
            output_dim: 512,
            num_heads: 4,
            dropout: 0.1,
            use_attention_pooling: true,
            aggregation: AggregationType::Mean,
        }
    }
}

/// Complete GNN Model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GNNModel {
    pub config: GNNConfig,
    pub layers: Vec<LayerType>,
    pub pooling: Option<AttentionPooling>,
    pub output_projection: Option<Tensor>,
}

impl GNNModel {
    /// Create a new GNN model with GraphSAGE layers
    pub fn new_sage(config: GNNConfig, rng: &mut impl Rng) -> Self {
        let mut layers = Vec::new();
        let mut in_dim = config.input_dim;

        // Hidden layers
        for &hidden_dim in &config.hidden_dims {
            let layer = SAGELayer::new(
                in_dim,
                hidden_dim,
                config.aggregation,
                true,
                rng,
            );
            layers.push(LayerType::SAGE(layer));
            in_dim = hidden_dim;
        }

        // Pooling layer
        let pooling = if config.use_attention_pooling {
            Some(AttentionPooling::new(in_dim, in_dim / 2, rng))
        } else {
            None
        };

        // Output projection
        let output_projection = if in_dim != config.output_dim {
            Some(Tensor::xavier_uniform(vec![in_dim, config.output_dim], rng))
        } else {
            None
        };

        Self {
            config,
            layers,
            pooling,
            output_projection,
        }
    }

    /// Create a new GNN model with GAT layers
    pub fn new_gat(config: GNNConfig, rng: &mut impl Rng) -> Self {
        let mut layers = Vec::new();
        let mut in_dim = config.input_dim;

        // Hidden layers
        for &hidden_dim in &config.hidden_dims {
            let layer = GATLayer::new(
                in_dim,
                hidden_dim,
                config.num_heads,
                config.dropout,
                0.2, // LeakyReLU alpha
                true,
                rng,
            );
            layers.push(LayerType::GAT(layer));
            in_dim = hidden_dim * config.num_heads;
        }

        // Pooling layer
        let pooling = if config.use_attention_pooling {
            Some(AttentionPooling::new(in_dim, in_dim / 2, rng))
        } else {
            None
        };

        // Output projection
        let output_projection = if in_dim != config.output_dim {
            Some(Tensor::xavier_uniform(vec![in_dim, config.output_dim], rng))
        } else {
            None
        };

        Self {
            config,
            layers,
            pooling,
            output_projection,
        }
    }

    /// Create a hybrid model with both SAGE and GAT layers
    pub fn new_hybrid(config: GNNConfig, rng: &mut impl Rng) -> Self {
        let mut layers = Vec::new();
        let mut in_dim = config.input_dim;

        // Alternate between SAGE and GAT layers
        for (i, &hidden_dim) in config.hidden_dims.iter().enumerate() {
            if i % 2 == 0 {
                // SAGE layer
                let layer = SAGELayer::new(
                    in_dim,
                    hidden_dim,
                    config.aggregation,
                    true,
                    rng,
                );
                layers.push(LayerType::SAGE(layer));
                in_dim = hidden_dim;
            } else {
                // GAT layer
                let layer = GATLayer::new(
                    in_dim,
                    hidden_dim,
                    config.num_heads,
                    config.dropout,
                    0.2,
                    true,
                    rng,
                );
                layers.push(LayerType::GAT(layer));
                in_dim = hidden_dim * config.num_heads;
            }
        }

        // Pooling layer
        let pooling = if config.use_attention_pooling {
            Some(AttentionPooling::new(in_dim, in_dim / 2, rng))
        } else {
            None
        };

        // Output projection
        let output_projection = if in_dim != config.output_dim {
            Some(Tensor::xavier_uniform(vec![in_dim, config.output_dim], rng))
        } else {
            None
        };

        Self {
            config,
            layers,
            pooling,
            output_projection,
        }
    }

    /// Forward pass: node features → graph embedding
    pub fn forward(
        &self,
        node_features: &Tensor,
        adjacency: &[(usize, Vec<usize>)],
    ) -> Result<Tensor, String> {
        // Pass through GNN layers
        let mut features = node_features.clone();
        for layer in &self.layers {
            features = layer.forward(&features, adjacency)?;
        }

        // Apply pooling to get graph-level representation
        let graph_embedding = if let Some(pooling) = &self.pooling {
            pooling.forward(&features)?
        } else {
            // Simple mean pooling
            features.mean(0, false)?
        };

        // Apply output projection if present
        if let Some(projection) = &self.output_projection {
            let reshaped = graph_embedding.reshape(vec![1, graph_embedding.data.len()])?;
            let projected = reshaped.matmul(projection)?;
            projected.reshape(vec![self.config.output_dim])
        } else {
            Ok(graph_embedding)
        }
    }

    /// Get node embeddings (before pooling)
    pub fn get_node_embeddings(
        &self,
        node_features: &Tensor,
        adjacency: &[(usize, Vec<usize>)],
    ) -> Result<Tensor, String> {
        let mut features = node_features.clone();
        for layer in &self.layers {
            features = layer.forward(&features, adjacency)?;
        }
        Ok(features)
    }

    /// Get graph embedding (after pooling)
    pub fn get_graph_embedding(
        &self,
        node_features: &Tensor,
        adjacency: &[(usize, Vec<usize>)],
    ) -> Result<Tensor, String> {
        self.forward(node_features, adjacency)
    }
}

/// Pooling layer for aggregating node features to graph-level
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolingLayer {
    pub pooling_type: PoolingType,
    pub attention_pooling: Option<AttentionPooling>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum PoolingType {
    Mean,
    Max,
    Sum,
    Attention,
}

impl PoolingLayer {
    /// Create a new pooling layer
    pub fn new(
        pooling_type: PoolingType,
        in_features: usize,
        rng: &mut impl Rng,
    ) -> Self {
        let attention_pooling = match pooling_type {
            PoolingType::Attention => {
                Some(AttentionPooling::new(in_features, in_features / 2, rng))
            }
            _ => None,
        };

        Self {
            pooling_type,
            attention_pooling,
        }
    }

    /// Pool node features to graph-level representation
    pub fn forward(&self, node_features: &Tensor) -> Result<Tensor, String> {
        match self.pooling_type {
            PoolingType::Mean => node_features.mean(0, false),
            PoolingType::Max => self.max_pooling(node_features),
            PoolingType::Sum => node_features.sum(0, false),
            PoolingType::Attention => {
                if let Some(pooling) = &self.attention_pooling {
                    pooling.forward(node_features)
                } else {
                    Err("Attention pooling not initialized".to_string())
                }
            }
        }
    }

    /// Max pooling over nodes
    fn max_pooling(&self, node_features: &Tensor) -> Result<Tensor, String> {
        if node_features.shape.len() != 2 {
            return Err("Max pooling requires 2D tensor".to_string());
        }

        let num_features = node_features.shape[1];
        let mut result = vec![f32::NEG_INFINITY; num_features];

        for i in 0..node_features.shape[0] {
            for j in 0..num_features {
                let idx = i * num_features + j;
                result[j] = result[j].max(node_features.data[idx]);
            }
        }

        Tensor::from_vec(result, vec![num_features])
    }
}

/// Graph batch for efficient training
#[derive(Debug, Clone)]
pub struct GraphBatch {
    pub node_features: Tensor,
    pub adjacency_lists: Vec<(usize, Vec<usize>)>,
    pub graph_indices: Vec<usize>, // Which graph each node belongs to
    pub num_graphs: usize,
}

impl GraphBatch {
    /// Create a batch from multiple graphs
    pub fn from_graphs(
        graphs: Vec<(Tensor, Vec<(usize, Vec<usize>)>)>,
    ) -> Result<Self, String> {
        if graphs.is_empty() {
            return Err("Cannot create batch from empty graph list".to_string());
        }

        let num_graphs = graphs.len();
        let mut all_features = Vec::new();
        let mut all_adjacency = Vec::new();
        let mut graph_indices = Vec::new();
        let mut node_offset = 0;

        for (graph_idx, (features, adjacency)) in graphs.into_iter().enumerate() {
            // Add features
            all_features.push(features.clone());

            // Add graph indices
            for _ in 0..features.shape[0] {
                graph_indices.push(graph_idx);
            }

            // Adjust adjacency indices
            for (node, neighbors) in adjacency {
                let adjusted_node = node + node_offset;
                let adjusted_neighbors: Vec<usize> = neighbors.iter()
                    .map(|n| n + node_offset)
                    .collect();
                all_adjacency.push((adjusted_node, adjusted_neighbors));
            }

            node_offset += features.shape[0];
        }

        // Concatenate all features
        let node_features = Tensor::concat(&all_features, 0)?;

        Ok(Self {
            node_features,
            adjacency_lists: all_adjacency,
            graph_indices,
            num_graphs,
        })
    }

    /// Get graph-level embeddings for each graph in the batch
    pub fn pool_by_graph(&self, node_embeddings: &Tensor) -> Result<Vec<Tensor>, String> {
        let mut graph_embeddings = Vec::new();

        for graph_idx in 0..self.num_graphs {
            // Find nodes belonging to this graph
            let node_indices: Vec<usize> = self.graph_indices.iter()
                .enumerate()
                .filter(|(_, &g)| g == graph_idx)
                .map(|(i, _)| i)
                .collect();

            if node_indices.is_empty() {
                return Err(format!("No nodes found for graph {}", graph_idx));
            }

            // Extract features for this graph's nodes
            let num_features = node_embeddings.shape[1];
            let mut graph_features = Vec::new();

            for &node_idx in &node_indices {
                for j in 0..num_features {
                    let idx = node_idx * num_features + j;
                    graph_features.push(node_embeddings.data[idx]);
                }
            }

            let graph_tensor = Tensor::from_vec(
                graph_features,
                vec![node_indices.len(), num_features],
            )?;

            // Mean pooling for this graph
            let pooled = graph_tensor.mean(0, false)?;
            graph_embeddings.push(pooled);
        }

        Ok(graph_embeddings)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand::rngs::SmallRng;

    #[test]
    fn test_gnn_model_sage() {
        let mut rng = SmallRng::seed_from_u64(42);
        let config = GNNConfig {
            input_dim: 4,
            hidden_dims: vec![8, 8],
            output_dim: 16,
            num_heads: 2,
            dropout: 0.1,
            use_attention_pooling: false,
            aggregation: AggregationType::Mean,
        };

        let model = GNNModel::new_sage(config, &mut rng);

        let features = Tensor::ones(vec![3, 4]);
        let adjacency = vec![
            (0, vec![1, 2]),
            (1, vec![0, 2]),
            (2, vec![0, 1]),
        ];

        let output = model.forward(&features, &adjacency).unwrap();
        assert_eq!(output.shape, vec![16]);
    }

    #[test]
    fn test_graph_batch() {
        let graph1 = (
            Tensor::ones(vec![2, 4]),
            vec![(0, vec![1]), (1, vec![0])],
        );
        let graph2 = (
            Tensor::ones(vec![3, 4]),
            vec![(0, vec![1, 2]), (1, vec![0]), (2, vec![0])],
        );

        let batch = GraphBatch::from_graphs(vec![graph1, graph2]).unwrap();
        assert_eq!(batch.num_graphs, 2);
        assert_eq!(batch.node_features.shape, vec![5, 4]);
        assert_eq!(batch.graph_indices, vec![0, 0, 1, 1, 1]);
    }
}
