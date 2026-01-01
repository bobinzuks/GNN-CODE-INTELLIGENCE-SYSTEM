//! Graph neural network layers
//! Implements GraphSAGE and GAT layers for efficient graph processing

use crate::tensor::Tensor;
use serde::{Deserialize, Serialize};
use rand::Rng;

/// GraphSAGE aggregation types
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum AggregationType {
    Mean,
    Pool,
    LSTM,
}

/// GraphSAGE Layer for efficient large graph processing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SAGELayer {
    pub in_features: usize,
    pub out_features: usize,
    pub aggregation: AggregationType,

    // Weights for self and neighbor transformations
    pub weight_self: Tensor,
    pub weight_neigh: Tensor,
    pub bias: Option<Tensor>,

    // For pooling aggregation
    pub pool_weight: Option<Tensor>,
}

impl SAGELayer {
    /// Create a new GraphSAGE layer
    pub fn new(
        in_features: usize,
        out_features: usize,
        aggregation: AggregationType,
        use_bias: bool,
        rng: &mut impl Rng,
    ) -> Self {
        let weight_self = Tensor::xavier_uniform(vec![in_features, out_features], rng);
        let weight_neigh = Tensor::xavier_uniform(vec![in_features, out_features], rng);

        let bias = if use_bias {
            Some(Tensor::zeros(vec![out_features]))
        } else {
            None
        };

        let pool_weight = match aggregation {
            AggregationType::Pool => {
                Some(Tensor::xavier_uniform(vec![in_features, in_features], rng))
            }
            _ => None,
        };

        Self {
            in_features,
            out_features,
            aggregation,
            weight_self,
            weight_neigh,
            bias,
            pool_weight,
        }
    }

    /// Forward pass through the layer
    /// features: [num_nodes, in_features]
    /// adjacency: list of (node_id, neighbor_ids)
    pub fn forward(
        &self,
        features: &Tensor,
        adjacency: &[(usize, Vec<usize>)],
    ) -> Result<Tensor, String> {
        if features.shape.len() != 2 || features.shape[1] != self.in_features {
            return Err(format!(
                "Expected features shape [num_nodes, {}], got {:?}",
                self.in_features, features.shape
            ));
        }

        let num_nodes = features.shape[0];

        // Aggregate neighbor features
        let aggregated = self.aggregate_neighbors(features, adjacency)?;

        // Transform self and neighbor features
        let self_transformed = features.matmul(&self.weight_self)?;
        let neigh_transformed = aggregated.matmul(&self.weight_neigh)?;

        // Combine
        let mut output = self_transformed.add(&neigh_transformed)?;

        // Add bias if present
        if let Some(bias) = &self.bias {
            // Broadcast bias to all nodes
            for i in 0..num_nodes {
                for j in 0..self.out_features {
                    let idx = i * self.out_features + j;
                    output.data[idx] += bias.data[j];
                }
            }
        }

        // Apply activation (ReLU)
        Ok(output.relu())
    }

    /// Aggregate neighbor features based on aggregation type
    fn aggregate_neighbors(
        &self,
        features: &Tensor,
        adjacency: &[(usize, Vec<usize>)],
    ) -> Result<Tensor, String> {
        let num_nodes = features.shape[0];
        let mut aggregated = Tensor::zeros(vec![num_nodes, self.in_features]);

        match self.aggregation {
            AggregationType::Mean => {
                // Mean aggregation
                for (node_id, neighbors) in adjacency {
                    if neighbors.is_empty() {
                        // If no neighbors, use self features
                        let self_features = features.get_row(*node_id)?;
                        for (j, val) in self_features.data.iter().enumerate() {
                            aggregated.data[node_id * self.in_features + j] = *val;
                        }
                    } else {
                        // Average neighbor features
                        for &neighbor in neighbors {
                            let neigh_features = features.get_row(neighbor)?;
                            for (j, val) in neigh_features.data.iter().enumerate() {
                                aggregated.data[node_id * self.in_features + j] += val;
                            }
                        }
                        let num_neighbors = neighbors.len() as f32;
                        for j in 0..self.in_features {
                            aggregated.data[node_id * self.in_features + j] /= num_neighbors;
                        }
                    }
                }
            }
            AggregationType::Pool => {
                // Max pooling aggregation with transformation
                let pool_weight = self.pool_weight.as_ref()
                    .ok_or("Pool weight not initialized")?;

                for (node_id, neighbors) in adjacency {
                    if neighbors.is_empty() {
                        let self_features = features.get_row(*node_id)?;
                        for (j, val) in self_features.data.iter().enumerate() {
                            aggregated.data[node_id * self.in_features + j] = *val;
                        }
                    } else {
                        // Transform and max pool
                        let mut max_features = vec![f32::NEG_INFINITY; self.in_features];
                        for &neighbor in neighbors {
                            let neigh_features = features.get_row(neighbor)?;
                            let transformed = neigh_features.matmul(pool_weight)?;
                            let activated = transformed.relu();
                            for (j, val) in activated.data.iter().enumerate() {
                                max_features[j] = max_features[j].max(*val);
                            }
                        }
                        for (j, val) in max_features.iter().enumerate() {
                            aggregated.data[node_id * self.in_features + j] = *val;
                        }
                    }
                }
            }
            AggregationType::LSTM => {
                // Simplified LSTM aggregation (using mean for now)
                // Full LSTM would require sequential processing
                for (node_id, neighbors) in adjacency {
                    if neighbors.is_empty() {
                        let self_features = features.get_row(*node_id)?;
                        for (j, val) in self_features.data.iter().enumerate() {
                            aggregated.data[node_id * self.in_features + j] = *val;
                        }
                    } else {
                        for &neighbor in neighbors {
                            let neigh_features = features.get_row(neighbor)?;
                            for (j, val) in neigh_features.data.iter().enumerate() {
                                aggregated.data[node_id * self.in_features + j] += val;
                            }
                        }
                        let num_neighbors = neighbors.len() as f32;
                        for j in 0..self.in_features {
                            aggregated.data[node_id * self.in_features + j] /= num_neighbors;
                        }
                    }
                }
            }
        }

        Ok(aggregated)
    }
}

/// Graph Attention Layer (GAT)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GATLayer {
    pub in_features: usize,
    pub out_features: usize,
    pub num_heads: usize,

    // Weight matrix for feature transformation
    pub weight: Tensor,

    // Attention mechanism parameters
    pub attention_weight: Tensor,
    pub bias: Option<Tensor>,

    pub dropout: f32,
    pub alpha: f32, // LeakyReLU negative slope
}

impl GATLayer {
    /// Create a new GAT layer
    pub fn new(
        in_features: usize,
        out_features: usize,
        num_heads: usize,
        dropout: f32,
        alpha: f32,
        use_bias: bool,
        rng: &mut impl Rng,
    ) -> Self {
        let weight = Tensor::xavier_uniform(
            vec![in_features, out_features * num_heads],
            rng,
        );

        // Attention weights: 2 * out_features per head
        let attention_weight = Tensor::xavier_uniform(
            vec![num_heads, 2 * out_features],
            rng,
        );

        let bias = if use_bias {
            Some(Tensor::zeros(vec![out_features * num_heads]))
        } else {
            None
        };

        Self {
            in_features,
            out_features,
            num_heads,
            weight,
            attention_weight,
            bias,
            dropout,
            alpha,
        }
    }

    /// Forward pass through GAT layer
    pub fn forward(
        &self,
        features: &Tensor,
        adjacency: &[(usize, Vec<usize>)],
    ) -> Result<Tensor, String> {
        if features.shape.len() != 2 || features.shape[1] != self.in_features {
            return Err(format!(
                "Expected features shape [num_nodes, {}], got {:?}",
                self.in_features, features.shape
            ));
        }

        let num_nodes = features.shape[0];

        // Linear transformation
        let transformed = features.matmul(&self.weight)?;

        // Apply attention mechanism
        let attended = self.apply_attention(&transformed, adjacency)?;

        // Add bias if present
        let mut output = attended;
        if let Some(bias) = &self.bias {
            for i in 0..num_nodes {
                for j in 0..(self.out_features * self.num_heads) {
                    let idx = i * (self.out_features * self.num_heads) + j;
                    output.data[idx] += bias.data[j];
                }
            }
        }

        // Apply ELU activation
        Ok(self.elu(&output, 1.0))
    }

    /// Apply attention mechanism
    fn apply_attention(
        &self,
        features: &Tensor,
        adjacency: &[(usize, Vec<usize>)],
    ) -> Result<Tensor, String> {
        let num_nodes = features.shape[0];
        let head_dim = self.out_features;
        let mut output = Tensor::zeros(vec![num_nodes, self.num_heads * head_dim]);

        for head in 0..self.num_heads {
            let offset = head * head_dim;

            for (node_id, neighbors) in adjacency {
                if neighbors.is_empty() {
                    // Use self features if no neighbors
                    for j in 0..head_dim {
                        let src_idx = node_id * (self.num_heads * head_dim) + offset + j;
                        let dst_idx = node_id * (self.num_heads * head_dim) + offset + j;
                        output.data[dst_idx] = features.data[src_idx];
                    }
                    continue;
                }

                // Compute attention scores
                let mut attention_scores = Vec::new();
                let self_features = self.get_head_features(features, *node_id, head, head_dim);

                for &neighbor in neighbors {
                    let neigh_features = self.get_head_features(features, neighbor, head, head_dim);

                    // Compute attention coefficient
                    let score = self.attention_score(&self_features, &neigh_features, head)?;
                    attention_scores.push(score);
                }

                // Apply softmax to attention scores
                let max_score = attention_scores.iter().cloned()
                    .fold(f32::NEG_INFINITY, f32::max);
                let exp_scores: Vec<f32> = attention_scores.iter()
                    .map(|s| (s - max_score).exp())
                    .collect();
                let sum_exp: f32 = exp_scores.iter().sum();
                let normalized_scores: Vec<f32> = exp_scores.iter()
                    .map(|s| s / sum_exp)
                    .collect();

                // Aggregate neighbor features with attention weights
                for j in 0..head_dim {
                    let mut aggregated = 0.0;
                    for (i, &neighbor) in neighbors.iter().enumerate() {
                        let feat_idx = neighbor * (self.num_heads * head_dim) + offset + j;
                        aggregated += normalized_scores[i] * features.data[feat_idx];
                    }
                    let out_idx = node_id * (self.num_heads * head_dim) + offset + j;
                    output.data[out_idx] = aggregated;
                }
            }
        }

        Ok(output)
    }

    /// Get features for a specific head
    fn get_head_features(&self, features: &Tensor, node_id: usize, head: usize, head_dim: usize) -> Vec<f32> {
        let offset = head * head_dim;
        let start = node_id * (self.num_heads * head_dim) + offset;
        features.data[start..start + head_dim].to_vec()
    }

    /// Compute attention score between two feature vectors
    fn attention_score(&self, feat_i: &[f32], feat_j: &[f32], head: usize) -> Result<f32, String> {
        // Concatenate features
        let concat: Vec<f32> = feat_i.iter().chain(feat_j.iter()).cloned().collect();

        // Compute attention: a^T [Wh_i || Wh_j]
        let mut score = 0.0;
        let att_offset = head * (2 * self.out_features);
        for (i, &val) in concat.iter().enumerate() {
            score += self.attention_weight.data[att_offset + i] * val;
        }

        // Apply LeakyReLU
        if score > 0.0 {
            Ok(score)
        } else {
            Ok(self.alpha * score)
        }
    }

    /// ELU activation
    fn elu(&self, tensor: &Tensor, alpha: f32) -> Tensor {
        let data: Vec<f32> = tensor.data.iter()
            .map(|x| if *x > 0.0 { *x } else { alpha * (x.exp() - 1.0) })
            .collect();
        Tensor {
            data,
            shape: tensor.shape.clone(),
        }
    }
}

/// Multi-head attention pooling for graph-level representations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttentionPooling {
    pub in_features: usize,
    pub attention_dim: usize,

    // Attention weights
    pub weight_query: Tensor,
    pub weight_key: Tensor,
    pub weight_value: Tensor,
}

impl AttentionPooling {
    /// Create a new attention pooling layer
    pub fn new(in_features: usize, attention_dim: usize, rng: &mut impl Rng) -> Self {
        let weight_query = Tensor::xavier_uniform(vec![in_features, attention_dim], rng);
        let weight_key = Tensor::xavier_uniform(vec![in_features, attention_dim], rng);
        let weight_value = Tensor::xavier_uniform(vec![in_features, in_features], rng);

        Self {
            in_features,
            attention_dim,
            weight_query,
            weight_key,
            weight_value,
        }
    }

    /// Pool node features to graph-level representation
    pub fn forward(&self, node_features: &Tensor) -> Result<Tensor, String> {
        if node_features.shape.len() != 2 || node_features.shape[1] != self.in_features {
            return Err(format!(
                "Expected node features shape [num_nodes, {}], got {:?}",
                self.in_features, node_features.shape
            ));
        }

        let num_nodes = node_features.shape[0];

        // Compute queries, keys, and values
        let queries = node_features.matmul(&self.weight_query)?;
        let keys = node_features.matmul(&self.weight_key)?;
        let values = node_features.matmul(&self.weight_value)?;

        // Compute attention scores: Q @ K^T / sqrt(d)
        let keys_t = keys.transpose()?;
        let scores = queries.matmul(&keys_t)?;
        let scale = (self.attention_dim as f32).sqrt();
        let scaled_scores = scores.scale(1.0 / scale);

        // Apply softmax
        let attention_weights = scaled_scores.softmax()?;

        // Apply attention to values
        let attended = attention_weights.matmul(&values)?;

        // Global mean pooling over attended features
        attended.mean(0, false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand::rngs::SmallRng;

    #[test]
    fn test_sage_layer() {
        let mut rng = SmallRng::seed_from_u64(42);
        let layer = SAGELayer::new(4, 8, AggregationType::Mean, true, &mut rng);

        let features = Tensor::ones(vec![3, 4]);
        let adjacency = vec![
            (0, vec![1, 2]),
            (1, vec![0, 2]),
            (2, vec![0, 1]),
        ];

        let output = layer.forward(&features, &adjacency).unwrap();
        assert_eq!(output.shape, vec![3, 8]);
    }

    #[test]
    fn test_gat_layer() {
        let mut rng = SmallRng::seed_from_u64(42);
        let layer = GATLayer::new(4, 8, 2, 0.6, 0.2, true, &mut rng);

        let features = Tensor::ones(vec![3, 4]);
        let adjacency = vec![
            (0, vec![1, 2]),
            (1, vec![0, 2]),
            (2, vec![0, 1]),
        ];

        let output = layer.forward(&features, &adjacency).unwrap();
        assert_eq!(output.shape, vec![3, 16]); // 2 heads * 8 features
    }
}
