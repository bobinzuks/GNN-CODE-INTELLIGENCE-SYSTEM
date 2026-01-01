//! Training infrastructure for GNN models
//! Includes contrastive learning and optimization

use crate::model::{GNNModel, GraphBatch};
use crate::tensor::Tensor;
use serde::{Deserialize, Serialize};
use rand::Rng;

/// Training configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingConfig {
    pub learning_rate: f32,
    pub num_epochs: usize,
    pub batch_size: usize,
    pub temperature: f32, // For contrastive loss
    pub margin: f32,      // For triplet loss
    pub weight_decay: f32,
}

impl Default for TrainingConfig {
    fn default() -> Self {
        Self {
            learning_rate: 0.001,
            num_epochs: 100,
            batch_size: 32,
            temperature: 0.07,
            margin: 1.0,
            weight_decay: 0.0001,
        }
    }
}

/// Loss function type
#[derive(Debug, Clone, Copy)]
pub enum LossType {
    Contrastive,
    Triplet,
    InfoNCE,
}

/// Gradient information for a single parameter
#[derive(Debug, Clone)]
pub struct Gradient {
    pub param_path: String,
    pub gradient: Tensor,
}

/// Training statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingStats {
    pub epoch: usize,
    pub loss: f32,
    pub learning_rate: f32,
}

/// GNN Trainer
pub struct Trainer {
    pub config: TrainingConfig,
    pub loss_type: LossType,
    pub stats_history: Vec<TrainingStats>,
}

impl Trainer {
    /// Create a new trainer
    pub fn new(config: TrainingConfig, loss_type: LossType) -> Self {
        Self {
            config,
            loss_type,
            stats_history: Vec::new(),
        }
    }

    /// Compute contrastive loss (InfoNCE)
    /// Pulls together similar embeddings, pushes apart dissimilar ones
    pub fn contrastive_loss(
        &self,
        anchor_embeddings: &[Tensor],
        positive_embeddings: &[Tensor],
        negative_embeddings: &[Tensor],
    ) -> Result<f32, String> {
        if anchor_embeddings.len() != positive_embeddings.len() {
            return Err("Anchor and positive embeddings must have same length".to_string());
        }

        let batch_size = anchor_embeddings.len();
        let mut total_loss = 0.0;

        for i in 0..batch_size {
            let anchor = &anchor_embeddings[i];
            let positive = &positive_embeddings[i];

            // Positive similarity
            let pos_sim = self.cosine_similarity(anchor, positive)? / self.config.temperature;

            // Negative similarities
            let mut neg_sims = Vec::new();
            for negative in negative_embeddings {
                let neg_sim = self.cosine_similarity(anchor, negative)? / self.config.temperature;
                neg_sims.push(neg_sim);
            }

            // InfoNCE loss: -log(exp(pos_sim) / (exp(pos_sim) + sum(exp(neg_sims))))
            let pos_exp = pos_sim.exp();
            let neg_exp_sum: f32 = neg_sims.iter().map(|s| s.exp()).sum();
            let denominator = pos_exp + neg_exp_sum;

            let loss = -(pos_exp / denominator).ln();
            total_loss += loss;
        }

        Ok(total_loss / batch_size as f32)
    }

    /// Compute triplet loss
    pub fn triplet_loss(
        &self,
        anchor_embeddings: &[Tensor],
        positive_embeddings: &[Tensor],
        negative_embeddings: &[Tensor],
    ) -> Result<f32, String> {
        if anchor_embeddings.len() != positive_embeddings.len()
            || anchor_embeddings.len() != negative_embeddings.len()
        {
            return Err("All embedding lists must have same length".to_string());
        }

        let batch_size = anchor_embeddings.len();
        let mut total_loss = 0.0;

        for i in 0..batch_size {
            let anchor = &anchor_embeddings[i];
            let positive = &positive_embeddings[i];
            let negative = &negative_embeddings[i];

            // Compute distances
            let pos_dist = self.euclidean_distance(anchor, positive)?;
            let neg_dist = self.euclidean_distance(anchor, negative)?;

            // Triplet loss: max(0, pos_dist - neg_dist + margin)
            let loss = (pos_dist - neg_dist + self.config.margin).max(0.0);
            total_loss += loss;
        }

        Ok(total_loss / batch_size as f32)
    }

    /// Compute cosine similarity between two embeddings
    pub fn cosine_similarity(&self, a: &Tensor, b: &Tensor) -> Result<f32, String> {
        if a.shape != b.shape {
            return Err(format!(
                "Embeddings must have same shape: {:?} vs {:?}",
                a.shape, b.shape
            ));
        }

        // Dot product
        let dot_product: f32 = a.data.iter().zip(b.data.iter()).map(|(x, y)| x * y).sum();

        // Norms
        let norm_a: f32 = a.data.iter().map(|x| x * x).sum::<f32>().sqrt();
        let norm_b: f32 = b.data.iter().map(|x| x * x).sum::<f32>().sqrt();

        if norm_a < 1e-8 || norm_b < 1e-8 {
            return Ok(0.0);
        }

        Ok(dot_product / (norm_a * norm_b))
    }

    /// Compute Euclidean distance between two embeddings
    pub fn euclidean_distance(&self, a: &Tensor, b: &Tensor) -> Result<f32, String> {
        if a.shape != b.shape {
            return Err(format!(
                "Embeddings must have same shape: {:?} vs {:?}",
                a.shape, b.shape
            ));
        }

        let squared_diff: f32 = a.data.iter()
            .zip(b.data.iter())
            .map(|(x, y)| (x - y).powi(2))
            .sum();

        Ok(squared_diff.sqrt())
    }

    /// Train step (simplified - actual training would require automatic differentiation)
    /// This is a placeholder showing the training loop structure
    pub fn train_step(
        &mut self,
        model: &mut GNNModel,
        batch: &GraphBatch,
        positive_batch: &GraphBatch,
        negative_batch: &GraphBatch,
    ) -> Result<f32, String> {
        // Forward pass
        let anchor_embeddings = vec![model.forward(
            &batch.node_features,
            &batch.adjacency_lists,
        )?];

        let positive_embeddings = vec![model.forward(
            &positive_batch.node_features,
            &positive_batch.adjacency_lists,
        )?];

        let negative_embeddings = vec![model.forward(
            &negative_batch.node_features,
            &negative_batch.adjacency_lists,
        )?];

        // Compute loss
        let loss = match self.loss_type {
            LossType::Contrastive | LossType::InfoNCE => {
                self.contrastive_loss(
                    &anchor_embeddings,
                    &positive_embeddings,
                    &negative_embeddings,
                )?
            }
            LossType::Triplet => {
                self.triplet_loss(
                    &anchor_embeddings,
                    &positive_embeddings,
                    &negative_embeddings,
                )?
            }
        };

        // Note: In a real implementation, we would compute gradients here
        // and update model parameters. This requires automatic differentiation
        // which is beyond the scope of this pure Rust implementation.
        // For WASM compatibility, gradient computation would need to be
        // implemented manually or using a compatible autodiff library.

        Ok(loss)
    }

    /// Record training statistics
    pub fn record_stats(&mut self, epoch: usize, loss: f32) {
        let stats = TrainingStats {
            epoch,
            loss,
            learning_rate: self.config.learning_rate,
        };
        self.stats_history.push(stats);
    }
}

/// Simple optimizer (SGD with momentum)
#[derive(Debug, Clone)]
pub struct Optimizer {
    pub learning_rate: f32,
    pub momentum: f32,
    pub weight_decay: f32,
    pub velocity: Vec<(String, Tensor)>,
}

impl Optimizer {
    /// Create a new optimizer
    pub fn new(learning_rate: f32, momentum: f32, weight_decay: f32) -> Self {
        Self {
            learning_rate,
            momentum,
            weight_decay,
            velocity: Vec::new(),
        }
    }

    /// Update parameters (placeholder - requires gradient computation)
    pub fn step(&mut self, _gradients: &[Gradient]) -> Result<(), String> {
        // In a real implementation, this would:
        // 1. Apply weight decay
        // 2. Update velocity with momentum
        // 3. Update parameters using velocity
        // For now, this is a placeholder
        Ok(())
    }

    /// Zero gradients
    pub fn zero_grad(&mut self) {
        // Clear accumulated gradients
        self.velocity.clear();
    }
}

/// Learning rate scheduler
#[derive(Debug, Clone)]
pub struct LRScheduler {
    pub initial_lr: f32,
    pub schedule_type: ScheduleType,
    pub current_epoch: usize,
}

#[derive(Debug, Clone, Copy)]
pub enum ScheduleType {
    Constant,
    StepDecay { step_size: usize, gamma: f32 },
    ExponentialDecay { gamma: f32 },
    CosineAnnealing { t_max: usize, eta_min: f32 },
}

impl LRScheduler {
    /// Create a new learning rate scheduler
    pub fn new(initial_lr: f32, schedule_type: ScheduleType) -> Self {
        Self {
            initial_lr,
            schedule_type,
            current_epoch: 0,
        }
    }

    /// Get current learning rate
    pub fn get_lr(&self) -> f32 {
        match self.schedule_type {
            ScheduleType::Constant => self.initial_lr,
            ScheduleType::StepDecay { step_size, gamma } => {
                let num_decays = self.current_epoch / step_size;
                self.initial_lr * gamma.powi(num_decays as i32)
            }
            ScheduleType::ExponentialDecay { gamma } => {
                self.initial_lr * gamma.powi(self.current_epoch as i32)
            }
            ScheduleType::CosineAnnealing { t_max, eta_min } => {
                eta_min + (self.initial_lr - eta_min)
                    * (1.0 + ((self.current_epoch as f32 * std::f32::consts::PI) / t_max as f32).cos())
                    / 2.0
            }
        }
    }

    /// Step the scheduler
    pub fn step(&mut self) {
        self.current_epoch += 1;
    }
}

/// Data augmentation for graphs
pub struct GraphAugmentor {
    pub drop_edge_prob: f32,
    pub feature_noise_std: f32,
}

impl GraphAugmentor {
    /// Create a new graph augmentor
    pub fn new(drop_edge_prob: f32, feature_noise_std: f32) -> Self {
        Self {
            drop_edge_prob,
            feature_noise_std,
        }
    }

    /// Augment graph by dropping edges
    pub fn drop_edges(
        &self,
        adjacency: &[(usize, Vec<usize>)],
        rng: &mut impl Rng,
    ) -> Vec<(usize, Vec<usize>)> {
        adjacency
            .iter()
            .map(|(node, neighbors)| {
                let kept_neighbors: Vec<usize> = neighbors
                    .iter()
                    .filter(|_| rng.gen::<f32>() > self.drop_edge_prob)
                    .copied()
                    .collect();
                (*node, kept_neighbors)
            })
            .collect()
    }

    /// Augment features with noise
    pub fn add_feature_noise(&self, features: &Tensor, rng: &mut impl Rng) -> Tensor {
        let mut noisy_features = features.clone();
        for val in &mut noisy_features.data {
            let noise = rng.gen::<f32>() * self.feature_noise_std;
            *val += noise;
        }
        noisy_features
    }

    /// Create augmented view of graph
    pub fn augment(
        &self,
        features: &Tensor,
        adjacency: &[(usize, Vec<usize>)],
        rng: &mut impl Rng,
    ) -> (Tensor, Vec<(usize, Vec<usize>)>) {
        let noisy_features = self.add_feature_noise(features, rng);
        let dropped_adjacency = self.drop_edges(adjacency, rng);
        (noisy_features, dropped_adjacency)
    }
}

/// Contrastive learning dataset
pub struct ContrastiveDataset {
    pub graphs: Vec<(Tensor, Vec<(usize, Vec<usize>)>)>,
    pub augmentor: GraphAugmentor,
}

impl ContrastiveDataset {
    /// Create a new contrastive dataset
    pub fn new(
        graphs: Vec<(Tensor, Vec<(usize, Vec<usize>)>)>,
        augmentor: GraphAugmentor,
    ) -> Self {
        Self { graphs, augmentor }
    }

    /// Get a batch of contrastive pairs
    pub fn get_batch(
        &self,
        batch_size: usize,
        rng: &mut impl Rng,
    ) -> Result<(Vec<GraphBatch>, Vec<GraphBatch>, Vec<GraphBatch>), String> {
        if self.graphs.is_empty() {
            return Err("Dataset is empty".to_string());
        }

        let mut anchors = Vec::new();
        let mut positives = Vec::new();
        let mut negatives = Vec::new();

        for _ in 0..batch_size {
            // Sample a random graph
            let idx = rng.gen_range(0..self.graphs.len());
            let (features, adjacency) = &self.graphs[idx];

            // Create two augmented views (anchor and positive)
            let (anchor_features, anchor_adj) = self.augmentor.augment(features, adjacency, rng);
            let (positive_features, positive_adj) = self.augmentor.augment(features, adjacency, rng);

            // Sample a different graph as negative
            let neg_idx = loop {
                let candidate = rng.gen_range(0..self.graphs.len());
                if candidate != idx {
                    break candidate;
                }
            };
            let (neg_features, neg_adjacency) = self.graphs[neg_idx].clone();

            anchors.push((anchor_features, anchor_adj));
            positives.push((positive_features, positive_adj));
            negatives.push((neg_features, neg_adjacency));
        }

        // Create batches
        let anchor_batch = GraphBatch::from_graphs(anchors)?;
        let positive_batch = GraphBatch::from_graphs(positives)?;
        let negative_batch = GraphBatch::from_graphs(negatives)?;

        Ok((vec![anchor_batch], vec![positive_batch], vec![negative_batch]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cosine_similarity() {
        let config = TrainingConfig::default();
        let trainer = Trainer::new(config, LossType::Contrastive);

        let a = Tensor::from_vec(vec![1.0, 0.0, 0.0], vec![3]).unwrap();
        let b = Tensor::from_vec(vec![1.0, 0.0, 0.0], vec![3]).unwrap();

        let sim = trainer.cosine_similarity(&a, &b).unwrap();
        assert!((sim - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_euclidean_distance() {
        let config = TrainingConfig::default();
        let trainer = Trainer::new(config, LossType::Triplet);

        let a = Tensor::from_vec(vec![0.0, 0.0, 0.0], vec![3]).unwrap();
        let b = Tensor::from_vec(vec![3.0, 4.0, 0.0], vec![3]).unwrap();

        let dist = trainer.euclidean_distance(&a, &b).unwrap();
        assert!((dist - 5.0).abs() < 1e-6);
    }

    #[test]
    fn test_lr_scheduler() {
        let scheduler = LRScheduler::new(
            0.1,
            ScheduleType::StepDecay {
                step_size: 10,
                gamma: 0.5,
            },
        );

        assert_eq!(scheduler.get_lr(), 0.1);
    }
}
