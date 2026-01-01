//! Language Expert trait definition

use crate::{CodeGraph, ExpertOutput, Issue, Suggestion, Location};
use serde::{Deserialize, Serialize};

/// Trait for language-specific experts
pub trait LanguageExpert: Send + Sync {
    /// Get the language this expert handles
    fn language(&self) -> &str;

    /// Check code for language-specific issues
    fn check(&self, graph: &CodeGraph) -> ExpertOutput;

    /// Get all patterns this expert can detect
    fn patterns(&self) -> &[Pattern];

    /// Suggest fixes for detected issues
    fn suggest_fixes(&self, issues: &[Issue]) -> Vec<Suggestion>;

    /// Get expert version
    fn version(&self) -> &str {
        "0.1.0"
    }

    /// Check if expert can handle this language variant
    fn can_handle(&self, language: &str) -> bool {
        language.eq_ignore_ascii_case(self.language())
    }
}

/// Pattern definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pattern {
    pub name: String,
    pub description: String,
    pub severity: crate::Severity,
    pub category: PatternCategory,
    pub confidence: f32,
}

impl Pattern {
    /// Create a new pattern
    pub fn new(
        name: impl Into<String>,
        description: impl Into<String>,
        severity: crate::Severity,
        category: PatternCategory,
    ) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            severity,
            category,
            confidence: 0.8,
        }
    }

    /// Set confidence level
    pub fn with_confidence(mut self, confidence: f32) -> Self {
        self.confidence = confidence.clamp(0.0, 1.0);
        self
    }
}

/// Pattern category
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PatternCategory {
    ErrorHandling,
    Async,
    Performance,
    Safety,
    Memory,
    Concurrency,
    CodeStyle,
    Complexity,
    Documentation,
    Security,
    Testing,
    Lifetimes,
    TypeSystem,
}

/// Pattern match result
#[derive(Debug, Clone)]
pub struct PatternMatch {
    pub pattern: String,
    pub location: Location,
    pub severity: crate::Severity,
    pub message: String,
    pub context: String,
}

impl PatternMatch {
    /// Convert to an Issue
    pub fn to_issue(&self) -> Issue {
        Issue {
            pattern: self.pattern.clone(),
            severity: self.severity,
            location: self.location.clone(),
            message: self.message.clone(),
            suggested_fix: None,
        }
    }

    /// Convert to an Issue with a suggested fix
    pub fn to_issue_with_fix(&self, fix: impl Into<String>) -> Issue {
        Issue {
            pattern: self.pattern.clone(),
            severity: self.severity,
            location: self.location.clone(),
            message: self.message.clone(),
            suggested_fix: Some(fix.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Severity;

    #[test]
    fn test_pattern_creation() {
        let pattern = Pattern::new(
            "test_pattern",
            "A test pattern",
            Severity::Warning,
            PatternCategory::ErrorHandling,
        );

        assert_eq!(pattern.name, "test_pattern");
        assert_eq!(pattern.description, "A test pattern");
        assert_eq!(pattern.severity, Severity::Warning);
        assert_eq!(pattern.category, PatternCategory::ErrorHandling);
        assert_eq!(pattern.confidence, 0.8);
    }

    #[test]
    fn test_pattern_with_confidence() {
        let pattern = Pattern::new(
            "test",
            "test",
            Severity::Info,
            PatternCategory::CodeStyle,
        )
        .with_confidence(0.95);

        assert_eq!(pattern.confidence, 0.95);
    }

    #[test]
    fn test_pattern_confidence_clamping() {
        let pattern = Pattern::new(
            "test",
            "test",
            Severity::Info,
            PatternCategory::CodeStyle,
        )
        .with_confidence(1.5);

        assert_eq!(pattern.confidence, 1.0);

        let pattern2 = Pattern::new(
            "test",
            "test",
            Severity::Info,
            PatternCategory::CodeStyle,
        )
        .with_confidence(-0.5);

        assert_eq!(pattern2.confidence, 0.0);
    }
}
