//! Runtime inference engine for GNN models
//! Optimized for both native and WASM environments

use crate::compression::{CodeGraph, SemanticCompressor, FeatureExtractor};
use crate::model::GNNModel;
use crate::tensor::Tensor;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Inference configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InferenceConfig {
    pub batch_size: usize,
    pub max_nodes_per_graph: usize,
    pub use_caching: bool,
    pub normalize_embeddings: bool,
}

impl Default for InferenceConfig {
    fn default() -> Self {
        Self {
            batch_size: 16,
            max_nodes_per_graph: 10000,
            use_caching: true,
            normalize_embeddings: true,
        }
    }
}

/// Inference result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InferenceResult {
    pub embedding: Tensor,
    pub node_embeddings: Option<Tensor>,
    pub inference_time_ms: f32,
}

/// Runtime inference engine
pub struct InferenceEngine {
    pub compressor: SemanticCompressor,
    pub config: InferenceConfig,
    pub cache: Option<HashMap<String, Tensor>>,
}

impl InferenceEngine {
    /// Create a new inference engine
    pub fn new(compressor: SemanticCompressor, config: InferenceConfig) -> Self {
        let cache = if config.use_caching {
            Some(HashMap::new())
        } else {
            None
        };

        Self {
            compressor,
            config,
            cache,
        }
    }

    /// Run inference on a single code graph
    pub fn infer(&mut self, graph: &CodeGraph) -> Result<InferenceResult, String> {
        let start_time = Self::get_time_ms();

        // Check cache
        if let Some(cache) = &self.cache {
            if let Some(cached_embedding) = cache.get(&graph.file_path) {
                let inference_time_ms = Self::get_time_ms() - start_time;
                return Ok(InferenceResult {
                    embedding: cached_embedding.clone(),
                    node_embeddings: None,
                    inference_time_ms,
                });
            }
        }

        // Run compression
        let embedding = self.compressor.compress(graph)?;

        // Cache the result
        if let Some(cache) = &mut self.cache {
            cache.insert(graph.file_path.clone(), embedding.clone());
        }

        let inference_time_ms = Self::get_time_ms() - start_time;

        Ok(InferenceResult {
            embedding,
            node_embeddings: None,
            inference_time_ms,
        })
    }

    /// Run inference on multiple code graphs
    pub fn infer_batch(&mut self, graphs: &[CodeGraph]) -> Result<Vec<InferenceResult>, String> {
        let mut results = Vec::new();

        for graph in graphs {
            let result = self.infer(graph)?;
            results.push(result);
        }

        Ok(results)
    }

    /// Get node-level embeddings (before pooling)
    pub fn get_node_embeddings(&self, graph: &CodeGraph) -> Result<Tensor, String> {
        let extractor = FeatureExtractor::new(self.compressor.feature_config.clone());
        let node_features = extractor.extract_features(graph)?;
        let adjacency = graph.get_adjacency_list();

        self.compressor.model.get_node_embeddings(&node_features, &adjacency)
    }

    /// Clear the embedding cache
    pub fn clear_cache(&mut self) {
        if let Some(cache) = &mut self.cache {
            cache.clear();
        }
    }

    /// Get cache statistics
    pub fn cache_stats(&self) -> (usize, usize) {
        if let Some(cache) = &self.cache {
            let num_entries = cache.len();
            let total_size: usize = cache.values().map(|t| t.data.len()).sum();
            (num_entries, total_size)
        } else {
            (0, 0)
        }
    }

    /// Get current time in milliseconds (WASM-compatible)
    #[cfg(not(target_arch = "wasm32"))]
    fn get_time_ms() -> f32 {
        use std::time::SystemTime;
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis() as f32
    }

    #[cfg(target_arch = "wasm32")]
    fn get_time_ms() -> f32 {
        // In WASM, use performance.now() via js_sys if available
        // For simplicity, return 0.0 here
        0.0
    }
}

/// Batch processor for efficient multi-graph inference
pub struct BatchProcessor {
    pub engine: InferenceEngine,
    pub batch_size: usize,
}

impl BatchProcessor {
    /// Create a new batch processor
    pub fn new(engine: InferenceEngine, batch_size: usize) -> Self {
        Self { engine, batch_size }
    }

    /// Process graphs in batches
    pub fn process(&mut self, graphs: Vec<CodeGraph>) -> Result<Vec<InferenceResult>, String> {
        let mut all_results = Vec::new();

        for chunk in graphs.chunks(self.batch_size) {
            let batch_results = self.engine.infer_batch(chunk)?;
            all_results.extend(batch_results);
        }

        Ok(all_results)
    }
}

/// Streaming inference for large codebases
pub struct StreamingInference {
    pub engine: InferenceEngine,
    pub window_size: usize,
    pub results: Vec<InferenceResult>,
}

impl StreamingInference {
    /// Create a new streaming inference processor
    pub fn new(engine: InferenceEngine, window_size: usize) -> Self {
        Self {
            engine,
            window_size,
            results: Vec::new(),
        }
    }

    /// Process a single graph in the stream
    pub fn process_one(&mut self, graph: &CodeGraph) -> Result<InferenceResult, String> {
        let result = self.engine.infer(graph)?;
        self.results.push(result.clone());

        // Keep only the last window_size results
        if self.results.len() > self.window_size {
            self.results.remove(0);
        }

        Ok(result)
    }

    /// Get aggregated embedding from the current window
    pub fn get_window_embedding(&self) -> Result<Tensor, String> {
        if self.results.is_empty() {
            return Err("No results in window".to_string());
        }

        let dim = self.results[0].embedding.data.len();
        let mut aggregated = Tensor::zeros(vec![dim]);

        for result in &self.results {
            for i in 0..dim {
                aggregated.data[i] += result.embedding.data[i];
            }
        }

        let num_results = self.results.len() as f32;
        for i in 0..dim {
            aggregated.data[i] /= num_results;
        }

        Ok(aggregated)
    }

    /// Clear the results window
    pub fn clear(&mut self) {
        self.results.clear();
    }
}

/// Similarity search engine
pub struct SimilaritySearch {
    pub engine: InferenceEngine,
    pub index: Vec<(String, Tensor)>, // (path, embedding) pairs
}

impl SimilaritySearch {
    /// Create a new similarity search engine
    pub fn new(engine: InferenceEngine) -> Self {
        Self {
            engine,
            index: Vec::new(),
        }
    }

    /// Index a code graph
    pub fn index_graph(&mut self, graph: &CodeGraph) -> Result<(), String> {
        let result = self.engine.infer(graph)?;
        self.index.push((graph.file_path.clone(), result.embedding));
        Ok(())
    }

    /// Index multiple graphs
    pub fn index_graphs(&mut self, graphs: &[CodeGraph]) -> Result<(), String> {
        for graph in graphs {
            self.index_graph(graph)?;
        }
        Ok(())
    }

    /// Find k most similar graphs to a query
    pub fn search(&mut self, query: &CodeGraph, k: usize) -> Result<Vec<(String, f32)>, String> {
        let query_result = self.engine.infer(query)?;
        let query_embedding = &query_result.embedding;

        let mut similarities: Vec<(String, f32)> = self
            .index
            .iter()
            .map(|(path, embedding)| {
                let similarity = self.cosine_similarity(query_embedding, embedding);
                (path.clone(), similarity)
            })
            .collect();

        // Sort by similarity (descending)
        similarities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        Ok(similarities.into_iter().take(k).collect())
    }

    /// Compute cosine similarity between two embeddings
    fn cosine_similarity(&self, a: &Tensor, b: &Tensor) -> f32 {
        let dot_product: f32 = a.data.iter().zip(b.data.iter()).map(|(x, y)| x * y).sum();
        let norm_a: f32 = a.data.iter().map(|x| x * x).sum::<f32>().sqrt();
        let norm_b: f32 = b.data.iter().map(|x| x * x).sum::<f32>().sqrt();

        if norm_a < 1e-8 || norm_b < 1e-8 {
            return 0.0;
        }

        dot_product / (norm_a * norm_b)
    }

    /// Get index size
    pub fn index_size(&self) -> usize {
        self.index.len()
    }

    /// Clear the index
    pub fn clear_index(&mut self) {
        self.index.clear();
    }
}

/// Model serving interface for production deployments
pub struct ModelServer {
    pub engine: InferenceEngine,
    pub model_version: String,
    pub request_count: usize,
}

impl ModelServer {
    /// Create a new model server
    pub fn new(engine: InferenceEngine, model_version: String) -> Self {
        Self {
            engine,
            model_version,
            request_count: 0,
        }
    }

    /// Handle inference request
    pub fn handle_request(&mut self, graph: &CodeGraph) -> Result<InferenceResult, String> {
        self.request_count += 1;
        self.engine.infer(graph)
    }

    /// Get server statistics
    pub fn get_stats(&self) -> ServerStats {
        let (cache_entries, cache_size) = self.engine.cache_stats();
        ServerStats {
            model_version: self.model_version.clone(),
            request_count: self.request_count,
            cache_entries,
            cache_size,
        }
    }

    /// Reset server statistics
    pub fn reset_stats(&mut self) {
        self.request_count = 0;
        self.engine.clear_cache();
    }
}

/// Server statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerStats {
    pub model_version: String,
    pub request_count: usize,
    pub cache_entries: usize,
    pub cache_size: usize,
}

/// WASM-specific inference utilities
#[cfg(target_arch = "wasm32")]
pub mod wasm {
    use super::*;
    use wasm_bindgen::prelude::*;

    /// WASM-friendly inference wrapper
    #[wasm_bindgen]
    pub struct WasmInference {
        engine: InferenceEngine,
    }

    #[wasm_bindgen]
    impl WasmInference {
        /// Create a new WASM inference instance
        #[wasm_bindgen(constructor)]
        pub fn new() -> Result<WasmInference, JsValue> {
            // This would need to be initialized with a proper model
            // For now, this is a placeholder
            Err(JsValue::from_str("Not implemented"))
        }

        /// Run inference on a serialized code graph
        pub fn infer(&mut self, _graph_json: &str) -> Result<String, JsValue> {
            // Deserialize graph, run inference, serialize result
            Err(JsValue::from_str("Not implemented"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compression::{examples, FeatureConfig};
    use crate::model::GNNConfig;
    use rand::SeedableRng;
    use rand::rngs::SmallRng;

    #[test]
    fn test_inference_engine() {
        let mut rng = SmallRng::seed_from_u64(42);
        let config = GNNConfig {
            input_dim: 128,
            hidden_dims: vec![256],
            output_dim: 512,
            num_heads: 4,
            dropout: 0.1,
            use_attention_pooling: true,
            aggregation: crate::layers::AggregationType::Mean,
        };

        let model = GNNModel::new_sage(config, &mut rng);
        let compressor = SemanticCompressor::new(
            model,
            FeatureConfig::default(),
            512,
        );

        let mut engine = InferenceEngine::new(compressor, InferenceConfig::default());

        let graph = examples::create_example_graph();
        let result = engine.infer(&graph).unwrap();

        assert_eq!(result.embedding.data.len(), 512);
    }

    #[test]
    fn test_similarity_search() {
        let mut rng = SmallRng::seed_from_u64(42);
        let config = GNNConfig {
            input_dim: 128,
            hidden_dims: vec![256],
            output_dim: 512,
            num_heads: 4,
            dropout: 0.1,
            use_attention_pooling: true,
            aggregation: crate::layers::AggregationType::Mean,
        };

        let model = GNNModel::new_sage(config, &mut rng);
        let compressor = SemanticCompressor::new(
            model,
            FeatureConfig::default(),
            512,
        );

        let engine = InferenceEngine::new(compressor, InferenceConfig::default());
        let mut search = SimilaritySearch::new(engine);

        let graph1 = examples::create_example_graph();
        let graph2 = examples::create_class_graph();

        search.index_graph(&graph1).unwrap();
        search.index_graph(&graph2).unwrap();

        assert_eq!(search.index_size(), 2);

        let results = search.search(&graph1, 1).unwrap();
        assert_eq!(results.len(), 1);
    }
}
