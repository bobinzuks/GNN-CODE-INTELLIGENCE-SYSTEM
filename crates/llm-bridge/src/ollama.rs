//! Ollama Client - Integration with Ollama LLM API
//!
//! Provides async client for Ollama's local LLM serving platform.
//! Supports both standard and streaming generation.

use crate::{
    GenerationRequest, GenerationResponse, GenerationStats, LLMBridgeError, LLMProvider,
    ModelInfo, Result, StreamChunk,
};
use async_trait::async_trait;
use futures::Stream;
use serde::{Deserialize, Serialize};
use std::time::Instant;

/// Ollama API client
pub struct OllamaClient {
    /// Base URL for Ollama API
    base_url: String,

    /// HTTP client
    client: reqwest::Client,

    /// Default model to use
    default_model: String,
}

impl OllamaClient {
    /// Create a new Ollama client
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            client: reqwest::Client::new(),
            default_model: "llama3.2".to_string(),
        }
    }

    /// Create a new Ollama client with custom model
    pub fn with_model(base_url: impl Into<String>, default_model: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            client: reqwest::Client::new(),
            default_model: default_model.into(),
        }
    }

    /// Set the default model
    pub fn set_default_model(&mut self, model: impl Into<String>) {
        self.default_model = model.into();
    }

    /// List available models
    pub async fn list_models(&self) -> Result<Vec<OllamaModel>> {
        let url = format!("{}/api/tags", self.base_url);
        let response = self.client.get(&url).send().await?;

        if !response.status().is_success() {
            return Err(LLMBridgeError::APIError(format!(
                "Failed to list models: {}",
                response.status()
            )));
        }

        let tags: OllamaTagsResponse = response.json().await?;
        Ok(tags.models)
    }

    /// Pull a model from Ollama registry
    pub async fn pull_model(&self, model: &str) -> Result<()> {
        let url = format!("{}/api/pull", self.base_url);
        let request = OllamaPullRequest {
            name: model.to_string(),
        };

        let response = self.client.post(&url).json(&request).send().await?;

        if !response.status().is_success() {
            return Err(LLMBridgeError::APIError(format!(
                "Failed to pull model: {}",
                response.status()
            )));
        }

        Ok(())
    }

    /// Delete a model
    pub async fn delete_model(&self, model: &str) -> Result<()> {
        let url = format!("{}/api/delete", self.base_url);
        let request = OllamaDeleteRequest {
            name: model.to_string(),
        };

        let response = self.client.delete(&url).json(&request).send().await?;

        if !response.status().is_success() {
            return Err(LLMBridgeError::APIError(format!(
                "Failed to delete model: {}",
                response.status()
            )));
        }

        Ok(())
    }

    /// Check if Ollama is running and accessible
    pub async fn health_check(&self) -> Result<bool> {
        let url = format!("{}/api/tags", self.base_url);
        match self.client.get(&url).send().await {
            Ok(response) => Ok(response.status().is_success()),
            Err(_) => Ok(false),
        }
    }
}

#[async_trait]
impl LLMProvider for OllamaClient {
    async fn generate(&self, request: &GenerationRequest) -> Result<GenerationResponse> {
        let url = format!("{}/api/generate", self.base_url);

        let ollama_request = OllamaGenerateRequest {
            model: if request.model.is_empty() {
                self.default_model.clone()
            } else {
                request.model.clone()
            },
            prompt: request.prompt.clone(),
            system: request.system.clone(),
            stream: false,
            options: Some(OllamaOptions {
                temperature: Some(request.parameters.temperature),
                num_predict: request.parameters.max_tokens.map(|x| x as i32),
                top_p: request.parameters.top_p,
                top_k: request.parameters.top_k.map(|x| x as i32),
                repeat_penalty: request.parameters.repeat_penalty,
                stop: if request.parameters.stop.is_empty() {
                    None
                } else {
                    Some(request.parameters.stop.clone())
                },
            }),
        };

        let start = Instant::now();
        let response = self.client.post(&url).json(&ollama_request).send().await?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(LLMBridgeError::APIError(format!(
                "Ollama API error: {}",
                error_text
            )));
        }

        let ollama_response: OllamaGenerateResponse = response.json().await?;
        let generation_time = start.elapsed().as_millis() as u64;

        Ok(GenerationResponse {
            text: ollama_response.response,
            model: ollama_response.model,
            stats: Some(GenerationStats {
                prompt_tokens: ollama_response.prompt_eval_count.unwrap_or(0),
                completion_tokens: ollama_response.eval_count.unwrap_or(0),
                total_tokens: ollama_response.prompt_eval_count.unwrap_or(0)
                    + ollama_response.eval_count.unwrap_or(0),
                generation_time_ms: generation_time,
            }),
        })
    }

    async fn generate_stream(
        &self,
        request: &GenerationRequest,
    ) -> Result<Box<dyn Stream<Item = Result<StreamChunk>> + Unpin + Send>> {
        use futures::StreamExt;

        let url = format!("{}/api/generate", self.base_url);

        let ollama_request = OllamaGenerateRequest {
            model: if request.model.is_empty() {
                self.default_model.clone()
            } else {
                request.model.clone()
            },
            prompt: request.prompt.clone(),
            system: request.system.clone(),
            stream: true,
            options: Some(OllamaOptions {
                temperature: Some(request.parameters.temperature),
                num_predict: request.parameters.max_tokens.map(|x| x as i32),
                top_p: request.parameters.top_p,
                top_k: request.parameters.top_k.map(|x| x as i32),
                repeat_penalty: request.parameters.repeat_penalty,
                stop: if request.parameters.stop.is_empty() {
                    None
                } else {
                    Some(request.parameters.stop.clone())
                },
            }),
        };

        let response = self.client.post(&url).json(&ollama_request).send().await?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(LLMBridgeError::APIError(format!(
                "Ollama API error: {}",
                error_text
            )));
        }

        let stream = response.bytes_stream().map(|result| match result {
            Ok(bytes) => {
                let text = String::from_utf8_lossy(&bytes);
                // Parse NDJSON
                for line in text.lines() {
                    if line.is_empty() {
                        continue;
                    }
                    match serde_json::from_str::<OllamaStreamResponse>(line) {
                        Ok(chunk) => {
                            return Ok(StreamChunk {
                                text: chunk.response,
                                done: chunk.done,
                                stats: if chunk.done {
                                    Some(GenerationStats {
                                        prompt_tokens: chunk.prompt_eval_count.unwrap_or(0),
                                        completion_tokens: chunk.eval_count.unwrap_or(0),
                                        total_tokens: chunk.prompt_eval_count.unwrap_or(0)
                                            + chunk.eval_count.unwrap_or(0),
                                        generation_time_ms: 0,
                                    })
                                } else {
                                    None
                                },
                            });
                        }
                        Err(e) => {
                            return Err(LLMBridgeError::StreamError(format!(
                                "Failed to parse stream chunk: {}",
                                e
                            )));
                        }
                    }
                }
                Ok(StreamChunk {
                    text: String::new(),
                    done: false,
                    stats: None,
                })
            }
            Err(e) => Err(LLMBridgeError::HttpError(e)),
        });

        Ok(Box::new(Box::pin(stream)))
    }

    fn name(&self) -> &str {
        "Ollama"
    }

    async fn get_model_info(&self, model: &str) -> Result<ModelInfo> {
        let url = format!("{}/api/show", self.base_url);
        let request = OllamaShowRequest {
            name: model.to_string(),
        };

        let response = self.client.post(&url).json(&request).send().await?;

        if !response.status().is_success() {
            return Err(LLMBridgeError::APIError(format!(
                "Failed to get model info: {}",
                response.status()
            )));
        }

        let show_response: OllamaShowResponse = response.json().await?;

        let (parameters, context_length, family) = if let Some(details) = show_response.details {
            (details.parameter_size, details.context_length, details.family)
        } else {
            (None, None, None)
        };

        Ok(ModelInfo {
            name: model.to_string(),
            parameters,
            context_length,
            family,
        })
    }
}

// Ollama API types

#[derive(Debug, Serialize)]
struct OllamaGenerateRequest {
    model: String,
    prompt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    system: Option<String>,
    stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<OllamaOptions>,
}

#[derive(Debug, Serialize)]
struct OllamaOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    num_predict: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_k: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repeat_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stop: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
struct OllamaGenerateResponse {
    model: String,
    response: String,
    done: bool,
    #[serde(default)]
    prompt_eval_count: Option<usize>,
    #[serde(default)]
    eval_count: Option<usize>,
}

#[derive(Debug, Deserialize)]
struct OllamaStreamResponse {
    response: String,
    done: bool,
    #[serde(default)]
    prompt_eval_count: Option<usize>,
    #[serde(default)]
    eval_count: Option<usize>,
}

#[derive(Debug, Serialize)]
struct OllamaPullRequest {
    name: String,
}

#[derive(Debug, Serialize)]
struct OllamaDeleteRequest {
    name: String,
}

#[derive(Debug, Serialize)]
struct OllamaShowRequest {
    name: String,
}

#[derive(Debug, Deserialize)]
struct OllamaShowResponse {
    #[serde(default)]
    details: Option<OllamaModelDetails>,
}

#[derive(Debug, Clone, Deserialize)]
struct OllamaModelDetails {
    #[serde(default)]
    parameter_size: Option<String>,
    #[serde(default)]
    context_length: Option<usize>,
    #[serde(default)]
    family: Option<String>,
}

#[derive(Debug, Deserialize)]
struct OllamaTagsResponse {
    models: Vec<OllamaModel>,
}

/// Ollama model information
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OllamaModel {
    /// Model name
    pub name: String,
    /// Model size
    pub size: i64,
    /// Modification time
    pub modified_at: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ollama_client_creation() {
        let client = OllamaClient::new("http://localhost:11434");
        assert_eq!(client.name(), "Ollama");
        assert_eq!(client.default_model, "llama3.2");
    }

    #[test]
    fn test_ollama_client_with_model() {
        let client = OllamaClient::with_model("http://localhost:11434", "codellama");
        assert_eq!(client.default_model, "codellama");
    }

    #[test]
    fn test_set_default_model() {
        let mut client = OllamaClient::new("http://localhost:11434");
        client.set_default_model("mistral");
        assert_eq!(client.default_model, "mistral");
    }

    #[tokio::test]
    async fn test_generate_request_serialization() {
        let request = OllamaGenerateRequest {
            model: "llama3.2".to_string(),
            prompt: "Hello".to_string(),
            system: Some("You are helpful".to_string()),
            stream: false,
            options: Some(OllamaOptions {
                temperature: Some(0.7),
                num_predict: Some(100),
                top_p: Some(0.9),
                top_k: Some(40),
                repeat_penalty: Some(1.1),
                stop: None,
            }),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("llama3.2"));
        assert!(json.contains("Hello"));
    }
}
