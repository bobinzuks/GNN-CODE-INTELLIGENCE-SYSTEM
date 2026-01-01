//! Projection Layer - Maps GNN embeddings to LLM token space
//!
//! This module implements a learned projection layer that transforms
//! GNN embeddings (512-dimensional) into a format suitable for injection
//! into LLM prompts. The projection can be linear or use learned weights.

use crate::{LLMBridgeError, Result};
use serde::{Deserialize, Serialize};

/// Configuration for the projection layer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectionConfig {
    /// Input dimension (GNN embedding size)
    pub input_dim: usize,

    /// Output dimension (target LLM space)
    pub output_dim: usize,

    /// Type of projection to use
    pub projection_type: ProjectionType,

    /// Whether to normalize the output
    pub normalize: bool,

    /// Activation function to apply
    pub activation: ActivationType,
}

impl Default for ProjectionConfig {
    fn default() -> Self {
        Self {
            input_dim: 512,
            output_dim: 768, // Common transformer dimension
            projection_type: ProjectionType::Linear,
            normalize: true,
            activation: ActivationType::ReLU,
        }
    }
}

/// Type of projection transformation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProjectionType {
    /// Linear projection with learned weights
    Linear,

    /// Multi-layer perceptron projection
    MLP { hidden_dims: Vec<usize> },

    /// Identity projection (no transformation)
    Identity,

    /// Pooling-based projection (average, max)
    Pooling { pool_type: PoolingType },
}

/// Pooling type for projection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PoolingType {
    /// Average pooling
    Average,
    /// Max pooling
    Max,
    /// Attention-based pooling
    Attention,
}

/// Activation function types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActivationType {
    /// Rectified Linear Unit
    ReLU,
    /// Gaussian Error Linear Unit
    GELU,
    /// Hyperbolic tangent
    Tanh,
    /// Sigmoid
    Sigmoid,
    /// No activation
    None,
}

/// Projection layer that transforms embeddings
pub struct ProjectionLayer {
    /// Configuration
    config: ProjectionConfig,

    /// Learned weights (for linear projection)
    weights: Option<Vec<Vec<f32>>>,

    /// Bias terms
    bias: Option<Vec<f32>>,
}

impl ProjectionLayer {
    /// Create a new projection layer with the given configuration
    pub fn new(config: ProjectionConfig) -> Self {
        let (weights, bias) = match &config.projection_type {
            ProjectionType::Linear => {
                // Initialize weights with Xavier/Glorot initialization
                let scale = (2.0 / (config.input_dim + config.output_dim) as f32).sqrt();
                let weights = (0..config.output_dim)
                    .map(|_| {
                        (0..config.input_dim)
                            .map(|_| (rand::random::<f32>() - 0.5) * 2.0 * scale)
                            .collect()
                    })
                    .collect();
                let bias = vec![0.0; config.output_dim];
                (Some(weights), Some(bias))
            }
            ProjectionType::MLP { hidden_dims } => {
                // For MLP, we'll initialize the first layer weights
                let first_dim = hidden_dims.first().unwrap_or(&config.output_dim);
                let scale = (2.0 / (config.input_dim + first_dim) as f32).sqrt();
                let weights = (0..*first_dim)
                    .map(|_| {
                        (0..config.input_dim)
                            .map(|_| (rand::random::<f32>() - 0.5) * 2.0 * scale)
                            .collect()
                    })
                    .collect();
                let bias = vec![0.0; *first_dim];
                (Some(weights), Some(bias))
            }
            _ => (None, None),
        };

        Self {
            config,
            weights,
            bias,
        }
    }

    /// Create a projection layer from pre-trained weights
    pub fn from_weights(
        config: ProjectionConfig,
        weights: Vec<Vec<f32>>,
        bias: Vec<f32>,
    ) -> Result<Self> {
        // Validate dimensions
        if weights.len() != config.output_dim {
            return Err(LLMBridgeError::ProjectionError(format!(
                "Weight dimension mismatch: expected {}, got {}",
                config.output_dim,
                weights.len()
            )));
        }

        for (i, row) in weights.iter().enumerate() {
            if row.len() != config.input_dim {
                return Err(LLMBridgeError::ProjectionError(format!(
                    "Weight row {} dimension mismatch: expected {}, got {}",
                    i,
                    config.input_dim,
                    row.len()
                )));
            }
        }

        if bias.len() != config.output_dim {
            return Err(LLMBridgeError::ProjectionError(format!(
                "Bias dimension mismatch: expected {}, got {}",
                config.output_dim,
                bias.len()
            )));
        }

        Ok(Self {
            config,
            weights: Some(weights),
            bias: Some(bias),
        })
    }

    /// Project a GNN embedding to LLM token space
    pub fn project(&self, embedding: &[f32]) -> Result<Vec<f32>> {
        if embedding.len() != self.config.input_dim {
            return Err(LLMBridgeError::ProjectionError(format!(
                "Input dimension mismatch: expected {}, got {}",
                self.config.input_dim,
                embedding.len()
            )));
        }

        let mut output = match &self.config.projection_type {
            ProjectionType::Linear => self.project_linear(embedding)?,
            ProjectionType::MLP { hidden_dims } => self.project_mlp(embedding, hidden_dims)?,
            ProjectionType::Identity => {
                if self.config.input_dim == self.config.output_dim {
                    embedding.to_vec()
                } else {
                    // Pad or truncate
                    let mut result = vec![0.0; self.config.output_dim];
                    let copy_len = self.config.input_dim.min(self.config.output_dim);
                    result[..copy_len].copy_from_slice(&embedding[..copy_len]);
                    result
                }
            }
            ProjectionType::Pooling { pool_type } => {
                self.project_pooling(embedding, pool_type)?
            }
        };

        // Apply activation
        self.apply_activation(&mut output);

        // Normalize if configured
        if self.config.normalize {
            self.normalize(&mut output);
        }

        Ok(output)
    }

    /// Linear projection
    fn project_linear(&self, embedding: &[f32]) -> Result<Vec<f32>> {
        let weights = self.weights.as_ref().ok_or_else(|| {
            LLMBridgeError::ProjectionError("Weights not initialized".to_string())
        })?;
        let bias = self.bias.as_ref().ok_or_else(|| {
            LLMBridgeError::ProjectionError("Bias not initialized".to_string())
        })?;

        let mut output = vec![0.0; self.config.output_dim];

        for (i, (weight_row, bias_val)) in weights.iter().zip(bias.iter()).enumerate() {
            let mut sum = 0.0;
            for (w, &x) in weight_row.iter().zip(embedding.iter()) {
                sum += w * x;
            }
            output[i] = sum + bias_val;
        }

        Ok(output)
    }

    /// MLP projection
    fn project_mlp(&self, embedding: &[f32], hidden_dims: &[usize]) -> Result<Vec<f32>> {
        // First layer
        let mut current = self.project_linear(embedding)?;
        self.apply_activation(&mut current);

        // Additional hidden layers (simplified - in production would need more weights)
        for &dim in hidden_dims.iter().skip(1) {
            current = self.simple_linear(&current, dim)?;
            self.apply_activation(&mut current);
        }

        // Final projection to output_dim if needed
        if current.len() != self.config.output_dim {
            current = self.simple_linear(&current, self.config.output_dim)?;
        }

        Ok(current)
    }

    /// Simple linear layer (helper for MLP)
    fn simple_linear(&self, input: &[f32], output_dim: usize) -> Result<Vec<f32>> {
        let input_dim = input.len();
        let scale = (2.0 / (input_dim + output_dim) as f32).sqrt();

        let mut output = vec![0.0; output_dim];
        for i in 0..output_dim {
            let mut sum = 0.0;
            for (j, &x) in input.iter().enumerate() {
                // Simple random initialization (in production would be learned)
                let w = (((i * input_dim + j) as f32).sin() * 10000.0).fract() * scale;
                sum += w * x;
            }
            output[i] = sum;
        }

        Ok(output)
    }

    /// Pooling-based projection
    fn project_pooling(&self, embedding: &[f32], pool_type: &PoolingType) -> Result<Vec<f32>> {
        match pool_type {
            PoolingType::Average => {
                let pool_size = self.config.input_dim / self.config.output_dim;
                let mut output = vec![0.0; self.config.output_dim];

                for (i, chunk) in embedding.chunks(pool_size).enumerate() {
                    if i < output.len() {
                        output[i] = chunk.iter().sum::<f32>() / chunk.len() as f32;
                    }
                }

                Ok(output)
            }
            PoolingType::Max => {
                let pool_size = self.config.input_dim / self.config.output_dim;
                let mut output = vec![f32::MIN; self.config.output_dim];

                for (i, chunk) in embedding.chunks(pool_size).enumerate() {
                    if i < output.len() {
                        output[i] = chunk.iter().copied().fold(f32::MIN, f32::max);
                    }
                }

                Ok(output)
            }
            PoolingType::Attention => {
                // Simplified attention pooling
                let pool_size = self.config.input_dim / self.config.output_dim;
                let mut output = vec![0.0; self.config.output_dim];

                for (i, chunk) in embedding.chunks(pool_size).enumerate() {
                    if i < output.len() {
                        // Compute attention weights (simplified)
                        let weights: Vec<f32> = chunk
                            .iter()
                            .map(|&x| (x * x).exp())
                            .collect();
                        let sum_weights: f32 = weights.iter().sum();

                        // Weighted sum
                        output[i] = chunk
                            .iter()
                            .zip(weights.iter())
                            .map(|(&x, &w)| x * w / sum_weights)
                            .sum();
                    }
                }

                Ok(output)
            }
        }
    }

    /// Apply activation function
    fn apply_activation(&self, values: &mut [f32]) {
        match self.config.activation {
            ActivationType::ReLU => {
                for v in values.iter_mut() {
                    *v = v.max(0.0);
                }
            }
            ActivationType::GELU => {
                for v in values.iter_mut() {
                    // Approximation of GELU
                    *v = 0.5 * *v * (1.0 + ((2.0 / std::f32::consts::PI).sqrt() * (*v + 0.044715 * v.powi(3))).tanh());
                }
            }
            ActivationType::Tanh => {
                for v in values.iter_mut() {
                    *v = v.tanh();
                }
            }
            ActivationType::Sigmoid => {
                for v in values.iter_mut() {
                    *v = 1.0 / (1.0 + (-*v).exp());
                }
            }
            ActivationType::None => {}
        }
    }

    /// Normalize output vector (L2 normalization)
    fn normalize(&self, values: &mut [f32]) {
        let norm: f32 = values.iter().map(|x| x * x).sum::<f32>().sqrt();
        if norm > 1e-8 {
            for v in values.iter_mut() {
                *v /= norm;
            }
        }
    }

    /// Get the configuration
    pub fn config(&self) -> &ProjectionConfig {
        &self.config
    }

    /// Export weights for saving
    pub fn export_weights(&self) -> Option<(Vec<Vec<f32>>, Vec<f32>)> {
        match (&self.weights, &self.bias) {
            (Some(w), Some(b)) => Some((w.clone(), b.clone())),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_projection() {
        let config = ProjectionConfig {
            input_dim: 10,
            output_dim: 5,
            projection_type: ProjectionType::Linear,
            normalize: false,
            activation: ActivationType::None,
        };

        let layer = ProjectionLayer::new(config);
        let input = vec![1.0; 10];
        let output = layer.project(&input).unwrap();

        assert_eq!(output.len(), 5);
    }

    #[test]
    fn test_identity_projection() {
        let config = ProjectionConfig {
            input_dim: 10,
            output_dim: 10,
            projection_type: ProjectionType::Identity,
            normalize: false,
            activation: ActivationType::None,
        };

        let layer = ProjectionLayer::new(config);
        let input = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        let output = layer.project(&input).unwrap();

        assert_eq!(output, input);
    }

    #[test]
    fn test_pooling_projection() {
        let config = ProjectionConfig {
            input_dim: 10,
            output_dim: 5,
            projection_type: ProjectionType::Pooling {
                pool_type: PoolingType::Average,
            },
            normalize: false,
            activation: ActivationType::None,
        };

        let layer = ProjectionLayer::new(config);
        let input = vec![1.0; 10];
        let output = layer.project(&input).unwrap();

        assert_eq!(output.len(), 5);
        assert!(output.iter().all(|&x| (x - 1.0).abs() < 1e-5));
    }

    #[test]
    fn test_normalization() {
        let config = ProjectionConfig {
            input_dim: 10,
            output_dim: 10,
            projection_type: ProjectionType::Identity,
            normalize: true,
            activation: ActivationType::None,
        };

        let layer = ProjectionLayer::new(config);
        let input = vec![1.0; 10];
        let output = layer.project(&input).unwrap();

        let norm: f32 = output.iter().map(|x| x * x).sum::<f32>().sqrt();
        assert!((norm - 1.0).abs() < 1e-5);
    }

    #[test]
    fn test_relu_activation() {
        let config = ProjectionConfig {
            input_dim: 5,
            output_dim: 5,
            projection_type: ProjectionType::Identity,
            normalize: false,
            activation: ActivationType::ReLU,
        };

        let layer = ProjectionLayer::new(config);
        let input = vec![-1.0, 2.0, -3.0, 4.0, -5.0];
        let output = layer.project(&input).unwrap();

        assert_eq!(output, vec![0.0, 2.0, 0.0, 4.0, 0.0]);
    }

    #[test]
    fn test_dimension_mismatch() {
        let config = ProjectionConfig::default();
        let layer = ProjectionLayer::new(config);
        let input = vec![1.0; 100]; // Wrong size

        assert!(layer.project(&input).is_err());
    }
}
