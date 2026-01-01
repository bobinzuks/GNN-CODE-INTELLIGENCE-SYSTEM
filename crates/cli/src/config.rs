//! Configuration file handling for GNN CLI
//!
//! Provides TOML-based configuration with sensible defaults.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Main configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Sweep configuration
    #[serde(default)]
    pub sweep: SweepConfig,

    /// Parser configuration
    #[serde(default)]
    pub parser: ParserConfig,

    /// Training configuration
    #[serde(default)]
    pub training: TrainingConfig,

    /// Model paths
    #[serde(default)]
    pub models: ModelConfig,

    /// LLM configuration
    #[serde(default)]
    pub llm: LLMConfig,

    /// Output configuration
    #[serde(default)]
    pub output: OutputConfig,
}

/// Sweep-specific configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SweepConfig {
    /// Default language to sweep
    pub default_language: Option<String>,

    /// Default minimum stars
    #[serde(default = "default_min_stars")]
    pub min_stars: u32,

    /// Default minimum commits
    #[serde(default = "default_min_commits")]
    pub min_commits: u32,

    /// Default maximum results
    #[serde(default = "default_max_results")]
    pub max_results: usize,

    /// Default rate limit
    #[serde(default = "default_rate_limit")]
    pub rate_limit: usize,

    /// Use cache by default
    #[serde(default = "default_use_cache")]
    pub use_cache: bool,

    /// Cache directory
    #[serde(default = "default_cache_dir")]
    pub cache_dir: PathBuf,
}

/// Parser-specific configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParserConfig {
    /// Default number of parallel workers
    #[serde(default = "default_parallel_workers")]
    pub parallel_workers: usize,

    /// Ignore hidden files by default
    #[serde(default = "default_ignore_hidden")]
    pub ignore_hidden: bool,

    /// Ignore patterns
    #[serde(default = "default_ignore_patterns")]
    pub ignore_patterns: Vec<String>,

    /// Default export format
    #[serde(default = "default_export_format")]
    pub export_format: String,

    /// Follow symbolic links
    #[serde(default)]
    pub follow_links: bool,
}

/// Training-specific configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingConfig {
    /// Default number of epochs
    #[serde(default = "default_epochs")]
    pub epochs: u32,

    /// Default batch size
    #[serde(default = "default_batch_size")]
    pub batch_size: usize,

    /// Default learning rate
    #[serde(default = "default_learning_rate")]
    pub learning_rate: f32,

    /// Default architecture
    #[serde(default = "default_architecture")]
    pub architecture: String,

    /// Default hidden dimensions
    #[serde(default = "default_hidden_dims")]
    pub hidden_dims: Vec<usize>,

    /// Default output dimension
    #[serde(default = "default_output_dim")]
    pub output_dim: usize,

    /// Save checkpoint frequency (epochs)
    #[serde(default = "default_checkpoint_frequency")]
    pub checkpoint_frequency: u32,

    /// Early stopping patience
    #[serde(default = "default_early_stopping_patience")]
    pub early_stopping_patience: Option<u32>,
}

/// Model paths configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    /// Directory containing models
    #[serde(default = "default_models_dir")]
    pub models_dir: PathBuf,

    /// Head model path
    pub head_model: Option<PathBuf>,

    /// Expert models directory
    pub experts_dir: Option<PathBuf>,

    /// Default expert models
    #[serde(default)]
    pub default_experts: Vec<String>,
}

/// LLM configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMConfig {
    /// Default LLM model
    #[serde(default = "default_llm_model")]
    pub default_model: String,

    /// Default endpoint
    #[serde(default = "default_llm_endpoint")]
    pub endpoint: String,

    /// Default temperature
    #[serde(default = "default_temperature")]
    pub temperature: f32,

    /// Default max tokens
    #[serde(default = "default_max_tokens")]
    pub max_tokens: usize,

    /// Apply GNN post-processing by default
    #[serde(default = "default_gnn_fix")]
    pub gnn_fix: bool,

    /// API key (for OpenAI-compatible APIs)
    pub api_key: Option<String>,
}

/// Output configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputConfig {
    /// Default output directory
    #[serde(default = "default_output_dir")]
    pub output_dir: PathBuf,

    /// Enable colored output
    #[serde(default = "default_colored")]
    pub colored: bool,

    /// Show progress bars
    #[serde(default = "default_show_progress")]
    pub show_progress: bool,

    /// Verbosity level
    #[serde(default)]
    pub verbosity: u8,
}

// Default value functions
fn default_min_stars() -> u32 {
    1
}

fn default_min_commits() -> u32 {
    50
}

fn default_max_results() -> usize {
    1000
}

fn default_rate_limit() -> usize {
    10
}

fn default_use_cache() -> bool {
    true
}

fn default_cache_dir() -> PathBuf {
    PathBuf::from(".cache")
}

fn default_parallel_workers() -> usize {
    num_cpus::get().max(1)
}

fn default_ignore_hidden() -> bool {
    true
}

fn default_ignore_patterns() -> Vec<String> {
    vec![
        "target".to_string(),
        "node_modules".to_string(),
        ".git".to_string(),
        "dist".to_string(),
        "build".to_string(),
        "__pycache__".to_string(),
        ".venv".to_string(),
    ]
}

fn default_export_format() -> String {
    "bincode".to_string()
}

fn default_epochs() -> u32 {
    100
}

fn default_batch_size() -> usize {
    32
}

fn default_learning_rate() -> f32 {
    0.001
}

fn default_architecture() -> String {
    "sage".to_string()
}

fn default_hidden_dims() -> Vec<usize> {
    vec![256, 256]
}

fn default_output_dim() -> usize {
    512
}

fn default_checkpoint_frequency() -> u32 {
    10
}

fn default_early_stopping_patience() -> Option<u32> {
    Some(20)
}

fn default_models_dir() -> PathBuf {
    PathBuf::from("models")
}

fn default_llm_model() -> String {
    "codellama".to_string()
}

fn default_llm_endpoint() -> String {
    "http://localhost:11434".to_string()
}

fn default_temperature() -> f32 {
    0.7
}

fn default_max_tokens() -> usize {
    2048
}

fn default_gnn_fix() -> bool {
    true
}

fn default_output_dir() -> PathBuf {
    PathBuf::from("output")
}

fn default_colored() -> bool {
    true
}

fn default_show_progress() -> bool {
    true
}

impl Default for Config {
    fn default() -> Self {
        Self {
            sweep: SweepConfig::default(),
            parser: ParserConfig::default(),
            training: TrainingConfig::default(),
            models: ModelConfig::default(),
            llm: LLMConfig::default(),
            output: OutputConfig::default(),
        }
    }
}

impl Default for SweepConfig {
    fn default() -> Self {
        Self {
            default_language: None,
            min_stars: default_min_stars(),
            min_commits: default_min_commits(),
            max_results: default_max_results(),
            rate_limit: default_rate_limit(),
            use_cache: default_use_cache(),
            cache_dir: default_cache_dir(),
        }
    }
}

impl Default for ParserConfig {
    fn default() -> Self {
        Self {
            parallel_workers: default_parallel_workers(),
            ignore_hidden: default_ignore_hidden(),
            ignore_patterns: default_ignore_patterns(),
            export_format: default_export_format(),
            follow_links: false,
        }
    }
}

impl Default for TrainingConfig {
    fn default() -> Self {
        Self {
            epochs: default_epochs(),
            batch_size: default_batch_size(),
            learning_rate: default_learning_rate(),
            architecture: default_architecture(),
            hidden_dims: default_hidden_dims(),
            output_dim: default_output_dim(),
            checkpoint_frequency: default_checkpoint_frequency(),
            early_stopping_patience: default_early_stopping_patience(),
        }
    }
}

impl Default for ModelConfig {
    fn default() -> Self {
        Self {
            models_dir: default_models_dir(),
            head_model: None,
            experts_dir: None,
            default_experts: vec![
                "rust".to_string(),
                "python".to_string(),
                "go".to_string(),
                "typescript".to_string(),
            ],
        }
    }
}

impl Default for LLMConfig {
    fn default() -> Self {
        Self {
            default_model: default_llm_model(),
            endpoint: default_llm_endpoint(),
            temperature: default_temperature(),
            max_tokens: default_max_tokens(),
            gnn_fix: default_gnn_fix(),
            api_key: None,
        }
    }
}

impl Default for OutputConfig {
    fn default() -> Self {
        Self {
            output_dir: default_output_dir(),
            colored: default_colored(),
            show_progress: default_show_progress(),
            verbosity: 0,
        }
    }
}

impl Config {
    /// Load configuration from file
    pub fn from_file(path: &Path) -> Result<Self> {
        let contents = std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read config file: {}", path.display()))?;

        let config: Config = toml::from_str(&contents)
            .with_context(|| format!("Failed to parse config file: {}", path.display()))?;

        Ok(config)
    }

    /// Save configuration to file
    pub fn to_file(&self, path: &Path) -> Result<()> {
        let contents = toml::to_string_pretty(self)
            .context("Failed to serialize config")?;

        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create directory: {}", parent.display()))?;
        }

        std::fs::write(path, contents)
            .with_context(|| format!("Failed to write config file: {}", path.display()))?;

        Ok(())
    }

    /// Get the example configuration as a string
    pub fn example_toml() -> &'static str {
        include_str!("../examples/gnn-intel.toml.example")
    }
}

// Add num_cpus as a dependency helper
mod num_cpus {
    pub fn get() -> usize {
        std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(4)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.sweep.min_stars, 1);
        assert_eq!(config.training.epochs, 100);
        assert_eq!(config.llm.default_model, "codellama");
    }

    #[test]
    fn test_save_and_load_config() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("test_config.toml");

        let config = Config::default();
        config.to_file(&config_path).unwrap();

        let loaded = Config::from_file(&config_path).unwrap();
        assert_eq!(loaded.sweep.min_stars, config.sweep.min_stars);
    }

    #[test]
    fn test_config_serialization() {
        let config = Config::default();
        let toml_str = toml::to_string(&config).unwrap();
        assert!(toml_str.contains("[sweep]"));
        assert!(toml_str.contains("[parser]"));
        assert!(toml_str.contains("[training]"));
    }
}
