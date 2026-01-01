//! OpenAI Client - Integration with OpenAI-compatible APIs
//!
//! Supports OpenAI API and compatible services (e.g., Together AI, Anyscale, etc.)
//! Provides both standard and streaming generation.

use crate::{
    GenerationRequest, GenerationResponse, GenerationStats, LLMBridgeError, LLMProvider,
    ModelInfo, Result, StreamChunk,
};
use async_trait::async_trait;
use futures::Stream;
use serde::{Deserialize, Serialize};
use std::time::Instant;

/// OpenAI API client
pub struct OpenAIClient {
    /// Base URL for API
    base_url: String,

    /// API key
    api_key: String,

    /// HTTP client
    client: reqwest::Client,

    /// Default model
    default_model: String,

    /// Organization ID (optional)
    organization: Option<String>,
}

impl OpenAIClient {
    /// Create a new OpenAI client
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            base_url: "https://api.openai.com/v1".to_string(),
            api_key: api_key.into(),
            client: reqwest::Client::new(),
            default_model: "gpt-4".to_string(),
            organization: None,
        }
    }

    /// Create a client for an OpenAI-compatible API
    pub fn with_base_url(base_url: impl Into<String>, api_key: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            api_key: api_key.into(),
            client: reqwest::Client::new(),
            default_model: "gpt-4".to_string(),
            organization: None,
        }
    }

    /// Set the organization ID
    pub fn set_organization(&mut self, org: impl Into<String>) {
        self.organization = Some(org.into());
    }

    /// Set the default model
    pub fn set_default_model(&mut self, model: impl Into<String>) {
        self.default_model = model.into();
    }

    /// List available models
    pub async fn list_models(&self) -> Result<Vec<OpenAIModel>> {
        let url = format!("{}/models", self.base_url);
        let mut request = self.client.get(&url).bearer_auth(&self.api_key);

        if let Some(org) = &self.organization {
            request = request.header("OpenAI-Organization", org);
        }

        let response = request.send().await?;

        if !response.status().is_success() {
            return Err(LLMBridgeError::APIError(format!(
                "Failed to list models: {}",
                response.status()
            )));
        }

        let models_response: OpenAIModelsResponse = response.json().await?;
        Ok(models_response.data)
    }

    /// Check if API is accessible
    pub async fn health_check(&self) -> Result<bool> {
        let url = format!("{}/models", self.base_url);
        let request = self.client.get(&url).bearer_auth(&self.api_key);

        match request.send().await {
            Ok(response) => Ok(response.status().is_success()),
            Err(_) => Ok(false),
        }
    }

    /// Build authorization headers
    fn build_headers(&self) -> reqwest::header::HeaderMap {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {}", self.api_key).parse().unwrap(),
        );
        if let Some(org) = &self.organization {
            headers.insert(
                reqwest::header::HeaderName::from_static("openai-organization"),
                org.parse().unwrap(),
            );
        }
        headers
    }
}

#[async_trait]
impl LLMProvider for OpenAIClient {
    async fn generate(&self, request: &GenerationRequest) -> Result<GenerationResponse> {
        let url = format!("{}/chat/completions", self.base_url);

        let mut messages = Vec::new();

        // Add system message if provided
        if let Some(system) = &request.system {
            messages.push(OpenAIMessage {
                role: "system".to_string(),
                content: system.clone(),
            });
        }

        // Add user message
        messages.push(OpenAIMessage {
            role: "user".to_string(),
            content: request.prompt.clone(),
        });

        let openai_request = OpenAIChatRequest {
            model: if request.model.is_empty() {
                self.default_model.clone()
            } else {
                request.model.clone()
            },
            messages,
            temperature: Some(request.parameters.temperature),
            max_tokens: request.parameters.max_tokens,
            top_p: request.parameters.top_p,
            stop: if request.parameters.stop.is_empty() {
                None
            } else {
                Some(request.parameters.stop.clone())
            },
            stream: Some(false),
        };

        let start = Instant::now();
        let response = self
            .client
            .post(&url)
            .headers(self.build_headers())
            .json(&openai_request)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(LLMBridgeError::APIError(format!(
                "OpenAI API error: {}",
                error_text
            )));
        }

        let openai_response: OpenAIChatResponse = response.json().await?;
        let generation_time = start.elapsed().as_millis() as u64;

        let text = openai_response
            .choices
            .first()
            .map(|c| c.message.content.clone())
            .unwrap_or_default();

        Ok(GenerationResponse {
            text,
            model: openai_response.model,
            stats: Some(GenerationStats {
                prompt_tokens: openai_response.usage.prompt_tokens,
                completion_tokens: openai_response.usage.completion_tokens,
                total_tokens: openai_response.usage.total_tokens,
                generation_time_ms: generation_time,
            }),
        })
    }

    async fn generate_stream(
        &self,
        request: &GenerationRequest,
    ) -> Result<Box<dyn Stream<Item = Result<StreamChunk>> + Unpin + Send>> {
        use futures::StreamExt;

        let url = format!("{}/chat/completions", self.base_url);

        let mut messages = Vec::new();

        if let Some(system) = &request.system {
            messages.push(OpenAIMessage {
                role: "system".to_string(),
                content: system.clone(),
            });
        }

        messages.push(OpenAIMessage {
            role: "user".to_string(),
            content: request.prompt.clone(),
        });

        let openai_request = OpenAIChatRequest {
            model: if request.model.is_empty() {
                self.default_model.clone()
            } else {
                request.model.clone()
            },
            messages,
            temperature: Some(request.parameters.temperature),
            max_tokens: request.parameters.max_tokens,
            top_p: request.parameters.top_p,
            stop: if request.parameters.stop.is_empty() {
                None
            } else {
                Some(request.parameters.stop.clone())
            },
            stream: Some(true),
        };

        let response = self
            .client
            .post(&url)
            .headers(self.build_headers())
            .json(&openai_request)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(LLMBridgeError::APIError(format!(
                "OpenAI API error: {}",
                error_text
            )));
        }

        let stream = response.bytes_stream().map(|result| match result {
            Ok(bytes) => {
                let text = String::from_utf8_lossy(&bytes);
                // Parse SSE format: "data: {json}\n\n"
                for line in text.lines() {
                    if line.starts_with("data: ") {
                        let json_str = &line[6..];
                        if json_str == "[DONE]" {
                            return Ok(StreamChunk {
                                text: String::new(),
                                done: true,
                                stats: None,
                            });
                        }
                        match serde_json::from_str::<OpenAIStreamResponse>(json_str) {
                            Ok(chunk) => {
                                let delta = chunk
                                    .choices
                                    .first()
                                    .and_then(|c| c.delta.content.clone())
                                    .unwrap_or_default();
                                return Ok(StreamChunk {
                                    text: delta,
                                    done: false,
                                    stats: None,
                                });
                            }
                            Err(_) => continue,
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
        "OpenAI"
    }

    async fn get_model_info(&self, model: &str) -> Result<ModelInfo> {
        let url = format!("{}/models/{}", self.base_url, model);
        let mut request = self.client.get(&url).bearer_auth(&self.api_key);

        if let Some(org) = &self.organization {
            request = request.header("OpenAI-Organization", org);
        }

        let response = request.send().await?;

        if !response.status().is_success() {
            return Err(LLMBridgeError::APIError(format!(
                "Failed to get model info: {}",
                response.status()
            )));
        }

        let model_data: OpenAIModel = response.json().await?;

        Ok(ModelInfo {
            name: model_data.id,
            parameters: None,
            context_length: None,
            family: Some(model_data.owned_by),
        })
    }
}

// OpenAI API types

#[derive(Debug, Serialize)]
struct OpenAIChatRequest {
    model: String,
    messages: Vec<OpenAIMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stop: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stream: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
struct OpenAIMessage {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct OpenAIChatResponse {
    id: String,
    model: String,
    choices: Vec<OpenAIChoice>,
    usage: OpenAIUsage,
}

#[derive(Debug, Deserialize)]
struct OpenAIChoice {
    message: OpenAIMessage,
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct OpenAIUsage {
    prompt_tokens: usize,
    completion_tokens: usize,
    total_tokens: usize,
}

#[derive(Debug, Deserialize)]
struct OpenAIStreamResponse {
    choices: Vec<OpenAIStreamChoice>,
}

#[derive(Debug, Deserialize)]
struct OpenAIStreamChoice {
    delta: OpenAIStreamDelta,
}

#[derive(Debug, Deserialize)]
struct OpenAIStreamDelta {
    #[serde(default)]
    content: Option<String>,
}

#[derive(Debug, Deserialize)]
struct OpenAIModelsResponse {
    data: Vec<OpenAIModel>,
}

/// OpenAI model information
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OpenAIModel {
    /// Model ID
    pub id: String,
    /// Owner
    pub owned_by: String,
    /// Creation timestamp
    pub created: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_openai_client_creation() {
        let client = OpenAIClient::new("test-key");
        assert_eq!(client.name(), "OpenAI");
        assert_eq!(client.default_model, "gpt-4");
    }

    #[test]
    fn test_openai_client_with_base_url() {
        let client = OpenAIClient::with_base_url("https://api.together.xyz/v1", "test-key");
        assert_eq!(client.base_url, "https://api.together.xyz/v1");
    }

    #[test]
    fn test_set_organization() {
        let mut client = OpenAIClient::new("test-key");
        client.set_organization("org-123");
        assert_eq!(client.organization, Some("org-123".to_string()));
    }

    #[test]
    fn test_set_default_model() {
        let mut client = OpenAIClient::new("test-key");
        client.set_default_model("gpt-3.5-turbo");
        assert_eq!(client.default_model, "gpt-3.5-turbo");
    }

    #[test]
    fn test_chat_request_serialization() {
        let request = OpenAIChatRequest {
            model: "gpt-4".to_string(),
            messages: vec![OpenAIMessage {
                role: "user".to_string(),
                content: "Hello".to_string(),
            }],
            temperature: Some(0.7),
            max_tokens: Some(100),
            top_p: Some(0.9),
            stop: None,
            stream: Some(false),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("gpt-4"));
        assert!(json.contains("Hello"));
    }
}
