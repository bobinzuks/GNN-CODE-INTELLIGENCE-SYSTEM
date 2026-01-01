//! LLM Bridge - Integration Layer for GNN Embeddings and LLM APIs
//!
//! This crate provides a bridge between GNN embeddings and Large Language Models,
//! enabling context-enhanced code generation, analysis, and validation.
//!
//! # Architecture
//!
//! ```text
//! GNN Embeddings (512-dim) → Projection Layer → LLM Token Space
//!                                  ↓
//!                          Token Injection
//!                                  ↓
//!                            LLM API Call
//!                                  ↓
//!                          Post-Processing
//!                                  ↓
//!                        GNN Validation & Fixing
//! ```
//!
//! # Features
//!
//! - **Projection Layer**: Maps GNN embeddings to LLM token space with learned weights
//! - **Token Injection**: Multiple strategies (prepend, append, interleave)
//! - **Multi-Provider Support**: Ollama, OpenAI, and compatible APIs
//! - **Streaming**: Async streaming responses for real-time generation
//! - **Post-Processing**: Code fixing, validation, and diff generation
//! - **Error Recovery**: Robust error handling across API boundaries
//!
//! # Example Usage
//!
//! ```rust,ignore
//! use llm_bridge::*;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), LLMBridgeError> {
//!     // Create bridge with Ollama backend
//!     let ollama = OllamaClient::new("http://localhost:11434");
//!     let mut bridge = LLMBridge::new(
//!         Box::new(ollama),
//!         ProjectionConfig::default(),
//!     );
//!
//!     // GNN embedding from code analysis
//!     let embedding = vec![0.1; 512];
//!
//!     // Generate code with GNN context
//!     let response = bridge.generate(
//!         "Implement a binary search function",
//!         &embedding,
//!         InjectionStrategy::Prepend,
//!     ).await?;
//!
//!     println!("Generated code: {}", response.text);
//!     Ok(())
//! }
//! ```

#![warn(missing_docs)]

pub mod projection;
pub mod injection;
pub mod postprocess;
pub mod ollama;
pub mod openai;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub use projection::{ProjectionLayer, ProjectionConfig};
pub use injection::{InjectionStrategy, TokenInjector};
pub use postprocess::{PostProcessor, CodeFix, ValidationResult};
pub use ollama::OllamaClient;
pub use openai::OpenAIClient;

/// Errors that can occur in the LLM Bridge
#[derive(Error, Debug)]
pub enum LLMBridgeError {
    /// HTTP request error
    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),

    /// JSON serialization/deserialization error
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    /// Projection error
    #[error("Projection error: {0}")]
    ProjectionError(String),

    /// Injection error
    #[error("Injection error: {0}")]
    InjectionError(String),

    /// LLM API error
    #[error("LLM API error: {0}")]
    APIError(String),

    /// Post-processing error
    #[error("Post-processing error: {0}")]
    PostProcessError(String),

    /// Invalid configuration
    #[error("Invalid configuration: {0}")]
    ConfigError(String),

    /// Stream error
    #[error("Stream error: {0}")]
    StreamError(String),
}

/// Result type for LLM Bridge operations
pub type Result<T> = std::result::Result<T, LLMBridgeError>;

/// LLM provider trait for abstracting different API backends
#[async_trait]
pub trait LLMProvider: Send + Sync {
    /// Generate a completion for the given prompt
    async fn generate(&self, request: &GenerationRequest) -> Result<GenerationResponse>;

    /// Stream a completion for the given prompt
    async fn generate_stream(
        &self,
        request: &GenerationRequest,
    ) -> Result<Box<dyn futures::Stream<Item = Result<StreamChunk>> + Unpin + Send>>;

    /// Get the provider name
    fn name(&self) -> &str;

    /// Get model information
    async fn get_model_info(&self, model: &str) -> Result<ModelInfo>;
}

/// Request for text generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationRequest {
    /// Model to use for generation
    pub model: String,

    /// Input prompt
    pub prompt: String,

    /// Optional system message
    pub system: Option<String>,

    /// Generation parameters
    pub parameters: GenerationParameters,

    /// Whether to stream the response
    pub stream: bool,
}

/// Parameters controlling generation behavior
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationParameters {
    /// Temperature for sampling (0.0 = deterministic, 1.0 = creative)
    pub temperature: f32,

    /// Maximum tokens to generate
    pub max_tokens: Option<usize>,

    /// Top-p nucleus sampling
    pub top_p: Option<f32>,

    /// Top-k sampling
    pub top_k: Option<usize>,

    /// Repetition penalty
    pub repeat_penalty: Option<f32>,

    /// Stop sequences
    pub stop: Vec<String>,
}

impl Default for GenerationParameters {
    fn default() -> Self {
        Self {
            temperature: 0.7,
            max_tokens: Some(2048),
            top_p: Some(0.9),
            top_k: Some(40),
            repeat_penalty: Some(1.1),
            stop: vec![],
        }
    }
}

/// Response from text generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationResponse {
    /// Generated text
    pub text: String,

    /// Model used
    pub model: String,

    /// Generation statistics
    pub stats: Option<GenerationStats>,
}

/// Statistics about the generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationStats {
    /// Tokens in prompt
    pub prompt_tokens: usize,

    /// Tokens generated
    pub completion_tokens: usize,

    /// Total tokens
    pub total_tokens: usize,

    /// Generation time in milliseconds
    pub generation_time_ms: u64,
}

/// Streaming chunk from generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamChunk {
    /// Chunk of generated text
    pub text: String,

    /// Whether this is the final chunk
    pub done: bool,

    /// Optional stats (only in final chunk)
    pub stats: Option<GenerationStats>,
}

/// Model information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    /// Model name
    pub name: String,

    /// Model size in parameters
    pub parameters: Option<String>,

    /// Context window size
    pub context_length: Option<usize>,

    /// Model family
    pub family: Option<String>,
}

/// Main LLM Bridge struct
pub struct LLMBridge {
    /// LLM provider backend
    provider: Box<dyn LLMProvider>,

    /// Projection layer for embedding transformation
    projection: ProjectionLayer,

    /// Token injector for context enhancement
    injector: TokenInjector,

    /// Post-processor for validation and fixing
    postprocessor: PostProcessor,
}

impl LLMBridge {
    /// Create a new LLM Bridge with the given provider and configuration
    pub fn new(provider: Box<dyn LLMProvider>, projection_config: ProjectionConfig) -> Self {
        Self {
            provider,
            projection: ProjectionLayer::new(projection_config),
            injector: TokenInjector::new(),
            postprocessor: PostProcessor::new(),
        }
    }

    /// Generate text with GNN embedding context
    pub async fn generate(
        &mut self,
        prompt: &str,
        gnn_embedding: &[f32],
        injection_strategy: InjectionStrategy,
    ) -> Result<GenerationResponse> {
        self.generate_with_params(
            prompt,
            gnn_embedding,
            injection_strategy,
            GenerationParameters::default(),
            "llama3.2",
        )
        .await
    }

    /// Generate text with custom parameters
    pub async fn generate_with_params(
        &mut self,
        prompt: &str,
        gnn_embedding: &[f32],
        injection_strategy: InjectionStrategy,
        parameters: GenerationParameters,
        model: &str,
    ) -> Result<GenerationResponse> {
        // Project GNN embedding to LLM token space
        let projected = self.projection.project(gnn_embedding)?;

        // Inject context into prompt
        let enhanced_prompt = self.injector.inject(prompt, &projected, injection_strategy)?;

        // Create request
        let request = GenerationRequest {
            model: model.to_string(),
            prompt: enhanced_prompt,
            system: Some(
                "You are an AI assistant with deep understanding of code semantics and structure."
                    .to_string(),
            ),
            parameters,
            stream: false,
        };

        // Call LLM API
        let response = self.provider.generate(&request).await?;

        Ok(response)
    }

    /// Generate and validate code with GNN context
    pub async fn generate_and_validate(
        &mut self,
        prompt: &str,
        gnn_embedding: &[f32],
        injection_strategy: InjectionStrategy,
    ) -> Result<(GenerationResponse, ValidationResult)> {
        let response = self.generate(prompt, gnn_embedding, injection_strategy).await?;

        // Validate the generated code
        let validation = self.postprocessor.validate(&response.text, gnn_embedding)?;

        Ok((response, validation))
    }

    /// Generate code and apply fixes if validation fails
    pub async fn generate_with_fixing(
        &mut self,
        prompt: &str,
        gnn_embedding: &[f32],
        injection_strategy: InjectionStrategy,
        max_retries: usize,
    ) -> Result<GenerationResponse> {
        let mut response = self.generate(prompt, gnn_embedding, injection_strategy.clone()).await?;

        for retry in 0..max_retries {
            let validation = self.postprocessor.validate(&response.text, gnn_embedding)?;

            if validation.is_valid {
                return Ok(response);
            }

            // Apply fixes
            if let Some(fix) = self.postprocessor.generate_fix(&response.text, &validation)? {
                response.text = fix.fixed_code;

                // Re-validate
                let new_validation = self.postprocessor.validate(&response.text, gnn_embedding)?;
                if new_validation.is_valid {
                    return Ok(response);
                }
            }

            // If fixes didn't work, regenerate with error context
            if retry < max_retries - 1 {
                let error_context = format!(
                    "Previous attempt had issues: {}. Please fix these problems.",
                    validation.issues.join(", ")
                );
                let new_prompt = format!("{}\n\n{}", prompt, error_context);
                response = self.generate(&new_prompt, gnn_embedding, injection_strategy.clone()).await?;
            }
        }

        Ok(response)
    }

    /// Stream generation with GNN context
    pub async fn generate_stream(
        &mut self,
        prompt: &str,
        gnn_embedding: &[f32],
        injection_strategy: InjectionStrategy,
    ) -> Result<Box<dyn futures::Stream<Item = Result<StreamChunk>> + Unpin + Send>> {
        // Project and inject
        let projected = self.projection.project(gnn_embedding)?;
        let enhanced_prompt = self.injector.inject(prompt, &projected, injection_strategy)?;

        let request = GenerationRequest {
            model: "llama3.2".to_string(),
            prompt: enhanced_prompt,
            system: Some(
                "You are an AI assistant with deep understanding of code semantics and structure."
                    .to_string(),
            ),
            parameters: GenerationParameters::default(),
            stream: true,
        };

        self.provider.generate_stream(&request).await
    }

    /// Get the underlying provider
    pub fn provider(&self) -> &dyn LLMProvider {
        self.provider.as_ref()
    }

    /// Get model information
    pub async fn get_model_info(&self, model: &str) -> Result<ModelInfo> {
        self.provider.get_model_info(model).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generation_parameters_default() {
        let params = GenerationParameters::default();
        assert_eq!(params.temperature, 0.7);
        assert_eq!(params.max_tokens, Some(2048));
        assert!(params.top_p.is_some());
    }

    #[test]
    fn test_error_types() {
        let err = LLMBridgeError::ProjectionError("test".to_string());
        assert!(err.to_string().contains("Projection error"));

        let err = LLMBridgeError::APIError("test".to_string());
        assert!(err.to_string().contains("LLM API error"));
    }
}
