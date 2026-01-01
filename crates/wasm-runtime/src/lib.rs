//! GNN WASM Runtime Module
//!
//! Provides browser/edge deployment capabilities for the GNN Code Intelligence System.
//! Exposes a clean JavaScript API for code analysis, compression, and quality checking.

use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod bindings;
pub mod bridge;
pub mod memory;

use bindings::*;
use bridge::*;
use memory::*;

/// Initialize WASM module with panic hooks for better error messages
#[wasm_bindgen(start)]
pub fn init() {
    console_error_panic_hook::set_once();
    log("GNN WASM Runtime initialized");
}

/// Log to browser console
#[wasm_bindgen]
pub fn log(s: &str) {
    web_sys::console::log_1(&JsValue::from_str(s));
}

/// Main GNN Runtime class exposed to JavaScript
#[wasm_bindgen]
pub struct GNNRuntime {
    /// Pre-trained model weights
    models: ModelStore,
    /// Memory pool for efficient allocation
    memory_pool: WasmMemoryPool,
    /// Feature extractor for code nodes
    feature_extractor: FeatureExtractor,
    /// Configuration
    config: RuntimeConfig,
}

#[wasm_bindgen]
impl GNNRuntime {
    /// Create a new runtime instance
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<GNNRuntime, JsValue> {
        log("Creating new GNN Runtime");

        let config = RuntimeConfig::default();
        let memory_pool = WasmMemoryPool::new(config.max_memory_mb);

        Ok(GNNRuntime {
            models: ModelStore::new(),
            memory_pool,
            feature_extractor: FeatureExtractor::new(128),
            config,
        })
    }

    /// Load pre-trained models from bytes
    #[wasm_bindgen(js_name = loadModel)]
    pub fn load_model(&mut self, model_bytes: &[u8], language: &str) -> Result<(), JsValue> {
        log(&format!("Loading model for language: {}", language));

        // Deserialize model weights
        let model = self.models.load_from_bytes(model_bytes, language)
            .map_err(|e| JsValue::from_str(&format!("Failed to load model: {}", e)))?;

        log(&format!("Model loaded successfully: {} KB", model_bytes.len() / 1024));
        Ok(())
    }

    /// Compress code to fixed-size embedding
    #[wasm_bindgen]
    pub fn compress(&self, code: &str, language: &str) -> Result<Vec<f32>, JsValue> {
        self.memory_pool.check_allocation(code.len())
            .map_err(|e| JsValue::from_str(&e))?;

        // Parse code to graph
        let graph = parse_code_to_graph(code, language)
            .map_err(|e| JsValue::from_str(&format!("Parse error: {}", e)))?;

        // Extract features
        let features = self.feature_extractor.extract(&graph);

        // Get model for language
        let model = self.models.get(language)
            .ok_or_else(|| JsValue::from_str(&format!("No model loaded for {}", language)))?;

        // Compress to embedding
        let embedding = model.forward_graph(&graph, &features);

        Ok(embedding.data)
    }

    /// Check code for issues and patterns
    #[wasm_bindgen]
    pub fn check(&self, code: &str, language: &str) -> Result<JsValue, JsValue> {
        self.memory_pool.check_allocation(code.len())
            .map_err(|e| JsValue::from_str(&e))?;

        // Parse code to graph
        let graph = parse_code_to_graph(code, language)
            .map_err(|e| JsValue::from_str(&format!("Parse error: {}", e)))?;

        // Get model for language
        let model = self.models.get(language)
            .ok_or_else(|| JsValue::from_str(&format!("No model loaded for {}", language)))?;

        // Extract features and get embedding
        let features = self.feature_extractor.extract(&graph);
        let embedding = model.forward_graph(&graph, &features);

        // Detect issues
        let issues = detect_issues(&graph, &embedding, language);

        // Convert to JS-compatible format
        let result = CheckResult {
            issues,
            embedding: embedding.data,
            node_count: graph.node_count() as u32,
            edge_count: graph.edge_count() as u32,
        };

        serde_wasm_bindgen::to_value(&result)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
    }

    /// Suggest fixes for detected issues
    #[wasm_bindgen(js_name = suggestFixes)]
    pub fn suggest_fixes(&self, code: &str, language: &str) -> Result<JsValue, JsValue> {
        self.memory_pool.check_allocation(code.len())
            .map_err(|e| JsValue::from_str(&e))?;

        // Parse and detect issues
        let graph = parse_code_to_graph(code, language)
            .map_err(|e| JsValue::from_str(&format!("Parse error: {}", e)))?;

        let model = self.models.get(language)
            .ok_or_else(|| JsValue::from_str(&format!("No model loaded for {}", language)))?;

        let features = self.feature_extractor.extract(&graph);
        let embedding = model.forward_graph(&graph, &features);
        let issues = detect_issues(&graph, &embedding, language);

        // Generate fixes for each issue
        let fixes: Vec<Fix> = issues.iter()
            .filter_map(|issue| generate_fix(issue, code, &graph))
            .collect();

        let result = FixResult {
            original_code: code.to_string(),
            fixes,
            total_issues: issues.len() as u32,
        };

        serde_wasm_bindgen::to_value(&result)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
    }

    /// Apply fixes to code
    #[wasm_bindgen(js_name = applyFixes)]
    pub fn apply_fixes(&self, code: &str, fixes_json: &str) -> Result<String, JsValue> {
        let fixes: Vec<Fix> = serde_json::from_str(fixes_json)
            .map_err(|e| JsValue::from_str(&format!("Invalid fixes JSON: {}", e)))?;

        let mut result = code.to_string();

        // Apply fixes in reverse order to maintain positions
        let mut sorted_fixes = fixes;
        sorted_fixes.sort_by(|a, b| b.location.start_line.cmp(&a.location.start_line));

        for fix in sorted_fixes {
            result = apply_single_fix(&result, &fix)
                .map_err(|e| JsValue::from_str(&e))?;
        }

        Ok(result)
    }

    /// Get memory usage statistics
    #[wasm_bindgen(js_name = getMemoryStats)]
    pub fn get_memory_stats(&self) -> JsValue {
        let stats = self.memory_pool.stats();
        serde_wasm_bindgen::to_value(&stats).unwrap()
    }

    /// Clear memory pool cache
    #[wasm_bindgen(js_name = clearCache)]
    pub fn clear_cache(&mut self) {
        self.memory_pool.clear();
        log("Memory pool cache cleared");
    }
}

/// Standalone function to load model from Uint8Array
#[wasm_bindgen(js_name = loadModelFromBytes)]
pub fn load_model_from_bytes(model_bytes: &[u8], language: &str) -> Result<JsValue, JsValue> {
    log(&format!("Loading standalone model for {}", language));

    let mut runtime = GNNRuntime::new()?;
    runtime.load_model(model_bytes, language)?;

    Ok(JsValue::from_str("Model loaded successfully"))
}

/// Quick check without full runtime initialization
#[wasm_bindgen(js_name = quickCheck)]
pub fn quick_check(code: &str, language: &str) -> Result<JsValue, JsValue> {
    // Lightweight check without models - uses pattern matching only
    let graph = parse_code_to_graph(code, language)
        .map_err(|e| JsValue::from_str(&format!("Parse error: {}", e)))?;

    let issues = quick_pattern_check(&graph, language);

    let result = CheckResult {
        issues,
        embedding: vec![],
        node_count: graph.node_count() as u32,
        edge_count: graph.edge_count() as u32,
    };

    serde_wasm_bindgen::to_value(&result)
        .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
}

/// Get supported languages
#[wasm_bindgen(js_name = getSupportedLanguages)]
pub fn get_supported_languages() -> Vec<String> {
    vec![
        "rust".to_string(),
        "javascript".to_string(),
        "typescript".to_string(),
        "python".to_string(),
        "go".to_string(),
        "java".to_string(),
        "cpp".to_string(),
        "csharp".to_string(),
    ]
}

/// Get runtime version
#[wasm_bindgen(js_name = getVersion)]
pub fn get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

// Internal runtime configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
struct RuntimeConfig {
    max_memory_mb: usize,
    feature_dim: usize,
    embedding_dim: usize,
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        RuntimeConfig {
            max_memory_mb: 128,
            feature_dim: 128,
            embedding_dim: 512,
        }
    }
}

// Result structures for JS interop
#[derive(Debug, Clone, Serialize, Deserialize)]
struct CheckResult {
    issues: Vec<Issue>,
    embedding: Vec<f32>,
    node_count: u32,
    edge_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct FixResult {
    original_code: String,
    fixes: Vec<Fix>,
    total_issues: u32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn test_runtime_creation() {
        let runtime = GNNRuntime::new();
        assert!(runtime.is_ok());
    }

    #[wasm_bindgen_test]
    fn test_supported_languages() {
        let langs = get_supported_languages();
        assert!(langs.contains(&"rust".to_string()));
    }
}
