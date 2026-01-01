//! Token Injection - Strategies for enhancing prompts with GNN embeddings
//!
//! This module provides various strategies for injecting GNN-derived context
//! into LLM prompts to enhance code understanding and generation.

use crate::{LLMBridgeError, Result};
use serde::{Deserialize, Serialize};

/// Strategy for injecting GNN context into prompts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InjectionStrategy {
    /// Prepend context before the main prompt
    Prepend,

    /// Append context after the main prompt
    Append,

    /// Interleave context throughout the prompt
    Interleave { chunks: usize },

    /// Embed context as structured data
    Structured { format: StructuredFormat },

    /// Inject as special tokens (for models that support it)
    SpecialTokens { token_prefix: String },

    /// Use context to modify the system message
    SystemMessage,

    /// No injection (pass-through)
    None,
}

/// Format for structured injection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StructuredFormat {
    /// JSON format
    JSON,
    /// XML format
    XML,
    /// Markdown format
    Markdown,
    /// Custom format with template
    Custom { template: String },
}

/// Token injector that enhances prompts with GNN context
pub struct TokenInjector {
    /// Configuration for encoding
    config: InjectorConfig,
}

/// Configuration for the injector
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InjectorConfig {
    /// Maximum length of injected context
    pub max_context_length: usize,

    /// Whether to encode embeddings as readable text
    pub encode_as_text: bool,

    /// Precision for embedding values
    pub precision: usize,

    /// Whether to include metadata
    pub include_metadata: bool,
}

impl Default for InjectorConfig {
    fn default() -> Self {
        Self {
            max_context_length: 500,
            encode_as_text: true,
            precision: 3,
            include_metadata: true,
        }
    }
}

impl TokenInjector {
    /// Create a new token injector with default configuration
    pub fn new() -> Self {
        Self {
            config: InjectorConfig::default(),
        }
    }

    /// Create a token injector with custom configuration
    pub fn with_config(config: InjectorConfig) -> Self {
        Self { config }
    }

    /// Inject GNN context into a prompt using the specified strategy
    pub fn inject(
        &self,
        prompt: &str,
        projected_embedding: &[f32],
        strategy: InjectionStrategy,
    ) -> Result<String> {
        match strategy {
            InjectionStrategy::Prepend => self.inject_prepend(prompt, projected_embedding),
            InjectionStrategy::Append => self.inject_append(prompt, projected_embedding),
            InjectionStrategy::Interleave { chunks } => {
                self.inject_interleave(prompt, projected_embedding, chunks)
            }
            InjectionStrategy::Structured { format } => {
                self.inject_structured(prompt, projected_embedding, format)
            }
            InjectionStrategy::SpecialTokens { token_prefix } => {
                self.inject_special_tokens(prompt, projected_embedding, &token_prefix)
            }
            InjectionStrategy::SystemMessage => {
                // System message injection is handled separately
                Ok(prompt.to_string())
            }
            InjectionStrategy::None => Ok(prompt.to_string()),
        }
    }

    /// Prepend context before the prompt
    fn inject_prepend(&self, prompt: &str, embedding: &[f32]) -> Result<String> {
        let context = self.encode_embedding(embedding)?;
        Ok(format!(
            "[Code Context]\n{}\n\n[Task]\n{}",
            context, prompt
        ))
    }

    /// Append context after the prompt
    fn inject_append(&self, prompt: &str, embedding: &[f32]) -> Result<String> {
        let context = self.encode_embedding(embedding)?;
        Ok(format!(
            "[Task]\n{}\n\n[Code Context]\n{}",
            prompt, context
        ))
    }

    /// Interleave context throughout the prompt
    fn inject_interleave(&self, prompt: &str, embedding: &[f32], chunks: usize) -> Result<String> {
        if chunks == 0 {
            return Err(LLMBridgeError::InjectionError(
                "Chunks must be greater than 0".to_string(),
            ));
        }

        let context = self.encode_embedding(embedding)?;
        let lines: Vec<&str> = prompt.lines().collect();
        let chunk_size = (lines.len() + chunks - 1) / chunks; // Ceiling division

        let mut result = String::new();
        for (i, chunk) in lines.chunks(chunk_size).enumerate() {
            if i > 0 {
                result.push_str("\n[Context Reminder]\n");
                result.push_str(&self.create_context_summary(embedding, i, chunks)?);
                result.push_str("\n\n");
            }
            result.push_str(&chunk.join("\n"));
            result.push('\n');
        }

        result.push_str("\n[Full Context]\n");
        result.push_str(&context);

        Ok(result)
    }

    /// Inject as structured data
    fn inject_structured(
        &self,
        prompt: &str,
        embedding: &[f32],
        format: StructuredFormat,
    ) -> Result<String> {
        let structured_context = match format {
            StructuredFormat::JSON => self.encode_as_json(embedding)?,
            StructuredFormat::XML => self.encode_as_xml(embedding)?,
            StructuredFormat::Markdown => self.encode_as_markdown(embedding)?,
            StructuredFormat::Custom { template } => {
                self.encode_with_template(embedding, &template)?
            }
        };

        Ok(format!("{}\n\n{}", structured_context, prompt))
    }

    /// Inject using special tokens
    fn inject_special_tokens(
        &self,
        prompt: &str,
        embedding: &[f32],
        token_prefix: &str,
    ) -> Result<String> {
        // Create special tokens from embedding statistics
        let stats = self.compute_embedding_stats(embedding);

        let special_context = format!(
            "<{}_SEMANTIC_MEAN_{:.3}> <{}_SEMANTIC_MAX_{:.3}> <{}_SEMANTIC_DIM_{}>",
            token_prefix,
            stats.mean,
            token_prefix,
            stats.max,
            token_prefix,
            embedding.len()
        );

        Ok(format!("{} {}", special_context, prompt))
    }

    /// Encode embedding as human-readable text
    fn encode_embedding(&self, embedding: &[f32]) -> Result<String> {
        if !self.config.encode_as_text {
            return Ok(format!("Embedding vector of dimension {}", embedding.len()));
        }

        let stats = self.compute_embedding_stats(embedding);
        let mut context = String::new();

        if self.config.include_metadata {
            context.push_str(&format!(
                "Semantic embedding analysis:\n\
                 - Dimension: {}\n\
                 - Mean activation: {:.prec$}\n\
                 - Max activation: {:.prec$}\n\
                 - Min activation: {:.prec$}\n\
                 - Std deviation: {:.prec$}\n\n",
                embedding.len(),
                stats.mean,
                stats.max,
                stats.min,
                stats.std,
                prec = self.config.precision
            ));
        }

        // Encode top activations
        let mut indexed: Vec<(usize, f32)> = embedding.iter().copied().enumerate().collect();
        indexed.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        context.push_str("Top semantic features:\n");
        for (idx, val) in indexed.iter().take(10) {
            context.push_str(&format!(
                "  - Feature {}: {:.prec$}\n",
                idx,
                val,
                prec = self.config.precision
            ));
        }

        // Truncate if too long
        if context.len() > self.config.max_context_length {
            context.truncate(self.config.max_context_length);
            context.push_str("...\n[Context truncated]");
        }

        Ok(context)
    }

    /// Encode as JSON
    fn encode_as_json(&self, embedding: &[f32]) -> Result<String> {
        let stats = self.compute_embedding_stats(embedding);

        let json = serde_json::json!({
            "semantic_context": {
                "dimension": embedding.len(),
                "statistics": {
                    "mean": format!("{:.prec$}", stats.mean, prec = self.config.precision),
                    "max": format!("{:.prec$}", stats.max, prec = self.config.precision),
                    "min": format!("{:.prec$}", stats.min, prec = self.config.precision),
                    "std": format!("{:.prec$}", stats.std, prec = self.config.precision),
                },
                "top_features": self.get_top_features(embedding, 10),
            }
        });

        Ok(format!("```json\n{}\n```", serde_json::to_string_pretty(&json)?))
    }

    /// Encode as XML
    fn encode_as_xml(&self, embedding: &[f32]) -> Result<String> {
        let stats = self.compute_embedding_stats(embedding);
        let top_features = self.get_top_features(embedding, 10);

        let mut xml = String::from("<semantic_context>\n");
        xml.push_str(&format!("  <dimension>{}</dimension>\n", embedding.len()));
        xml.push_str("  <statistics>\n");
        xml.push_str(&format!(
            "    <mean>{:.prec$}</mean>\n",
            stats.mean,
            prec = self.config.precision
        ));
        xml.push_str(&format!(
            "    <max>{:.prec$}</max>\n",
            stats.max,
            prec = self.config.precision
        ));
        xml.push_str(&format!(
            "    <min>{:.prec$}</min>\n",
            stats.min,
            prec = self.config.precision
        ));
        xml.push_str(&format!(
            "    <std>{:.prec$}</std>\n",
            stats.std,
            prec = self.config.precision
        ));
        xml.push_str("  </statistics>\n");
        xml.push_str("  <top_features>\n");
        for (idx, val) in top_features {
            xml.push_str(&format!(
                "    <feature index=\"{}\">{:.prec$}</feature>\n",
                idx,
                val,
                prec = self.config.precision
            ));
        }
        xml.push_str("  </top_features>\n");
        xml.push_str("</semantic_context>");

        Ok(xml)
    }

    /// Encode as Markdown
    fn encode_as_markdown(&self, embedding: &[f32]) -> Result<String> {
        let stats = self.compute_embedding_stats(embedding);
        let top_features = self.get_top_features(embedding, 10);

        let mut md = String::from("## Semantic Context\n\n");
        md.push_str("### Statistics\n\n");
        md.push_str(&format!("- **Dimension**: {}\n", embedding.len()));
        md.push_str(&format!(
            "- **Mean**: {:.prec$}\n",
            stats.mean,
            prec = self.config.precision
        ));
        md.push_str(&format!(
            "- **Max**: {:.prec$}\n",
            stats.max,
            prec = self.config.precision
        ));
        md.push_str(&format!(
            "- **Min**: {:.prec$}\n",
            stats.min,
            prec = self.config.precision
        ));
        md.push_str(&format!(
            "- **Std Dev**: {:.prec$}\n",
            stats.std,
            prec = self.config.precision
        ));
        md.push_str("\n### Top Features\n\n");
        md.push_str("| Index | Activation |\n");
        md.push_str("|-------|------------|\n");
        for (idx, val) in top_features {
            md.push_str(&format!(
                "| {} | {:.prec$} |\n",
                idx,
                val,
                prec = self.config.precision
            ));
        }

        Ok(md)
    }

    /// Encode with custom template
    fn encode_with_template(&self, embedding: &[f32], template: &str) -> Result<String> {
        let stats = self.compute_embedding_stats(embedding);

        let result = template
            .replace("{dim}", &embedding.len().to_string())
            .replace(
                "{mean}",
                &format!("{:.prec$}", stats.mean, prec = self.config.precision),
            )
            .replace(
                "{max}",
                &format!("{:.prec$}", stats.max, prec = self.config.precision),
            )
            .replace(
                "{min}",
                &format!("{:.prec$}", stats.min, prec = self.config.precision),
            )
            .replace(
                "{std}",
                &format!("{:.prec$}", stats.std, prec = self.config.precision),
            );

        Ok(result)
    }

    /// Create a context summary for interleaving
    fn create_context_summary(&self, embedding: &[f32], chunk_idx: usize, total_chunks: usize) -> Result<String> {
        let stats = self.compute_embedding_stats(embedding);
        Ok(format!(
            "Chunk {}/{}: Semantic mean {:.3}",
            chunk_idx + 1,
            total_chunks,
            stats.mean
        ))
    }

    /// Compute embedding statistics
    fn compute_embedding_stats(&self, embedding: &[f32]) -> EmbeddingStats {
        let n = embedding.len() as f32;
        let mean = embedding.iter().sum::<f32>() / n;
        let max = embedding.iter().copied().fold(f32::NEG_INFINITY, f32::max);
        let min = embedding.iter().copied().fold(f32::INFINITY, f32::min);
        let variance = embedding.iter().map(|x| (x - mean).powi(2)).sum::<f32>() / n;
        let std = variance.sqrt();

        EmbeddingStats { mean, max, min, std }
    }

    /// Get top features
    fn get_top_features(&self, embedding: &[f32], k: usize) -> Vec<(usize, f32)> {
        let mut indexed: Vec<(usize, f32)> = embedding.iter().copied().enumerate().collect();
        indexed.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        indexed.into_iter().take(k).collect()
    }

    /// Get configuration
    pub fn config(&self) -> &InjectorConfig {
        &self.config
    }
}

impl Default for TokenInjector {
    fn default() -> Self {
        Self::new()
    }
}

/// Statistics about an embedding
#[derive(Debug, Clone)]
struct EmbeddingStats {
    mean: f32,
    max: f32,
    min: f32,
    std: f32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prepend_injection() {
        let injector = TokenInjector::new();
        let embedding = vec![0.5; 10];
        let result = injector
            .inject("Write a function", &embedding, InjectionStrategy::Prepend)
            .unwrap();

        assert!(result.contains("[Code Context]"));
        assert!(result.contains("[Task]"));
        assert!(result.contains("Write a function"));
    }

    #[test]
    fn test_append_injection() {
        let injector = TokenInjector::new();
        let embedding = vec![0.5; 10];
        let result = injector
            .inject("Write a function", &embedding, InjectionStrategy::Append)
            .unwrap();

        assert!(result.contains("[Code Context]"));
        assert!(result.contains("[Task]"));
    }

    #[test]
    fn test_json_encoding() {
        let injector = TokenInjector::new();
        let embedding = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = injector.encode_as_json(&embedding).unwrap();

        assert!(result.contains("semantic_context"));
        assert!(result.contains("dimension"));
        assert!(result.contains("statistics"));
    }

    #[test]
    fn test_markdown_encoding() {
        let injector = TokenInjector::new();
        let embedding = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = injector.encode_as_markdown(&embedding).unwrap();

        assert!(result.contains("## Semantic Context"));
        assert!(result.contains("### Statistics"));
        assert!(result.contains("### Top Features"));
    }

    #[test]
    fn test_embedding_stats() {
        let injector = TokenInjector::new();
        let embedding = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let stats = injector.compute_embedding_stats(&embedding);

        assert_eq!(stats.mean, 3.0);
        assert_eq!(stats.max, 5.0);
        assert_eq!(stats.min, 1.0);
    }

    #[test]
    fn test_none_strategy() {
        let injector = TokenInjector::new();
        let embedding = vec![0.5; 10];
        let prompt = "Write a function";
        let result = injector
            .inject(prompt, &embedding, InjectionStrategy::None)
            .unwrap();

        assert_eq!(result, prompt);
    }
}
