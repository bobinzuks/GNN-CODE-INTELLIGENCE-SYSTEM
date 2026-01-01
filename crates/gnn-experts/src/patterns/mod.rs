//! Pattern detection modules for all languages
//!
//! This module contains comprehensive pattern detectors for detecting
//! language-specific anti-patterns, code smells, and best practices.

pub mod rust_patterns;
pub mod python_patterns;
pub mod javascript_patterns;
pub mod typescript_patterns;
pub mod go_patterns;
pub mod java_patterns;
pub mod cpp_patterns;
pub mod c_patterns;

pub use rust_patterns::RustPatterns;
pub use python_patterns::PythonPatterns;
pub use javascript_patterns::JavaScriptPatterns;
pub use typescript_patterns::TypeScriptPatterns;
pub use go_patterns::GoPatterns;
pub use java_patterns::JavaPatterns;
pub use cpp_patterns::CppPatterns;
pub use c_patterns::CPatterns;

use crate::{CodeGraph, Location, Severity};
use serde::{Deserialize, Serialize};

/// Pattern detector trait
pub trait PatternDetector: Send + Sync {
    /// Get the pattern name
    fn name(&self) -> &str;

    /// Get the pattern description
    fn description(&self) -> &str;

    /// Get the pattern severity
    fn severity(&self) -> Severity;

    /// Detect the pattern in the graph
    fn detect(&self, graph: &CodeGraph) -> Vec<PatternInstance>;

    /// Get confidence score for this detector
    fn confidence(&self) -> f32 {
        0.8
    }

    /// Generate fix suggestion
    fn suggest_fix(&self, instance: &PatternInstance) -> Option<FixSuggestion>;
}

/// Instance of a detected pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternInstance {
    pub pattern_name: String,
    pub location: Location,
    pub severity: Severity,
    pub message: String,
    pub context: String,
    pub confidence: f32,
    pub metadata: std::collections::HashMap<String, String>,
}

/// Fix suggestion for a pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FixSuggestion {
    pub pattern_name: String,
    pub description: String,
    pub before_code: String,
    pub after_code: String,
    pub confidence: f32,
    pub automated: bool,
}

impl PatternInstance {
    /// Create a new pattern instance
    pub fn new(
        pattern_name: impl Into<String>,
        location: Location,
        severity: Severity,
        message: impl Into<String>,
    ) -> Self {
        Self {
            pattern_name: pattern_name.into(),
            location,
            severity,
            message: message.into(),
            context: String::new(),
            confidence: 0.8,
            metadata: std::collections::HashMap::new(),
        }
    }

    /// Set context
    pub fn with_context(mut self, context: impl Into<String>) -> Self {
        self.context = context.into();
        self
    }

    /// Set confidence
    pub fn with_confidence(mut self, confidence: f32) -> Self {
        self.confidence = confidence.clamp(0.0, 1.0);
        self
    }

    /// Add metadata
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }
}

impl FixSuggestion {
    /// Create a new fix suggestion
    pub fn new(
        pattern_name: impl Into<String>,
        description: impl Into<String>,
        before_code: impl Into<String>,
        after_code: impl Into<String>,
    ) -> Self {
        Self {
            pattern_name: pattern_name.into(),
            description: description.into(),
            before_code: before_code.into(),
            after_code: after_code.into(),
            confidence: 0.8,
            automated: false,
        }
    }

    /// Set confidence
    pub fn with_confidence(mut self, confidence: f32) -> Self {
        self.confidence = confidence.clamp(0.0, 1.0);
        self
    }

    /// Mark as automated fix
    pub fn automated(mut self) -> Self {
        self.automated = true;
        self
    }
}
