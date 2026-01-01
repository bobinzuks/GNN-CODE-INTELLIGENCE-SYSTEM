//! Merger module for combining outputs from multiple language experts

use crate::{ExpertOutput, Issue, Suggestion, Severity};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// MergerModel combines outputs from multiple experts
#[derive(Debug)]
pub struct MergerModel {
    pub deduplicate: bool,
    pub severity_threshold: Severity,
}

impl MergerModel {
    /// Create a new merger with default settings
    pub fn new() -> Self {
        Self {
            deduplicate: true,
            severity_threshold: Severity::Info,
        }
    }

    /// Merge outputs from multiple experts
    pub fn merge(&self, outputs: Vec<ExpertOutput>) -> MergedOutput {
        if outputs.is_empty() {
            return MergedOutput::empty();
        }

        let mut all_issues = Vec::new();
        let mut all_suggestions = Vec::new();
        let mut language_confidences = HashMap::new();
        let mut metadata = HashMap::new();

        // Collect all outputs
        for output in outputs {
            // Store language-specific confidence
            language_confidences.insert(output.language.clone(), output.confidence);

            // Collect issues
            for issue in output.issues {
                if issue.severity >= self.severity_threshold {
                    all_issues.push(issue);
                }
            }

            // Collect suggestions
            all_suggestions.extend(output.suggestions);

            // Merge metadata
            for (key, value) in output.metadata {
                metadata.insert(format!("{}.{}", output.language, key), value);
            }
        }

        // Deduplicate if enabled
        if self.deduplicate {
            all_issues = self.deduplicate_issues(all_issues);
            all_suggestions = self.deduplicate_suggestions(all_suggestions);
        }

        // Sort by severity (highest first) and confidence
        all_issues.sort_by(|a, b| {
            b.severity.cmp(&a.severity)
                .then_with(|| {
                    // Extract confidence from issue if available
                    let a_conf = 1.0; // Default confidence
                    let b_conf = 1.0;
                    b_conf.partial_cmp(&a_conf).unwrap()
                })
        });

        all_suggestions.sort_by(|a, b| {
            b.confidence.partial_cmp(&a.confidence).unwrap()
        });

        // Calculate overall confidence
        let overall_confidence = if language_confidences.is_empty() {
            0.0
        } else {
            language_confidences.values().sum::<f32>() / language_confidences.len() as f32
        };

        MergedOutput {
            issues: all_issues,
            suggestions: all_suggestions,
            language_confidences,
            overall_confidence,
            metadata,
        }
    }

    /// Remove duplicate issues
    fn deduplicate_issues(&self, issues: Vec<Issue>) -> Vec<Issue> {
        let mut seen = HashSet::new();
        let mut unique_issues = Vec::new();

        for issue in issues {
            let key = self.issue_key(&issue);
            if seen.insert(key) {
                unique_issues.push(issue);
            }
        }

        unique_issues
    }

    /// Remove duplicate suggestions
    fn deduplicate_suggestions(&self, suggestions: Vec<Suggestion>) -> Vec<Suggestion> {
        let mut seen = HashSet::new();
        let mut unique_suggestions = Vec::new();

        for suggestion in suggestions {
            let key = self.suggestion_key(&suggestion);
            if seen.insert(key) {
                unique_suggestions.push(suggestion);
            }
        }

        unique_suggestions
    }

    /// Generate a unique key for an issue
    fn issue_key(&self, issue: &Issue) -> String {
        format!(
            "{}:{}:{}:{}",
            issue.pattern,
            issue.location.file_path,
            issue.location.start_line,
            issue.severity as u8
        )
    }

    /// Generate a unique key for a suggestion
    fn suggestion_key(&self, suggestion: &Suggestion) -> String {
        if let Some(ref loc) = suggestion.location {
            format!(
                "{}:{}:{}",
                suggestion.pattern,
                loc.file_path,
                loc.start_line
            )
        } else {
            format!("{}:{}", suggestion.pattern, suggestion.description)
        }
    }

    /// Resolve conflicts between expert outputs
    pub fn resolve_conflicts(&self, outputs: &[ExpertOutput]) -> Vec<ConflictResolution> {
        let mut resolutions = Vec::new();

        // Group issues by location
        let mut location_map: HashMap<String, Vec<(String, &Issue)>> = HashMap::new();

        for output in outputs {
            for issue in &output.issues {
                let key = format!("{}:{}", issue.location.file_path, issue.location.start_line);
                location_map.entry(key).or_default().push((output.language.clone(), issue));
            }
        }

        // Find conflicts (multiple issues at same location)
        for (location_key, issues) in location_map {
            if issues.len() > 1 {
                // Select highest severity or highest confidence expert
                let resolved = self.select_best_issue(&issues);
                resolutions.push(ConflictResolution {
                    location: location_key,
                    conflicting_languages: issues.iter().map(|(lang, _)| lang.clone()).collect(),
                    resolved_language: resolved.0.clone(),
                    resolution_reason: "highest_severity".to_string(),
                });
            }
        }

        resolutions
    }

    /// Select the best issue from conflicting ones
    fn select_best_issue<'a>(&self, issues: &[( String, &'a Issue)]) -> (String, &'a Issue) {
        issues.iter()
            .max_by(|(_, a), (_, b)| a.severity.cmp(&b.severity))
            .map(|(lang, issue)| (lang.clone(), *issue))
            .unwrap()
    }
}

impl Default for MergerModel {
    fn default() -> Self {
        Self::new()
    }
}

/// Merged output from multiple experts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergedOutput {
    pub issues: Vec<Issue>,
    pub suggestions: Vec<Suggestion>,
    pub language_confidences: HashMap<String, f32>,
    pub overall_confidence: f32,
    pub metadata: HashMap<String, String>,
}

impl MergedOutput {
    /// Create an empty merged output
    pub fn empty() -> Self {
        Self {
            issues: Vec::new(),
            suggestions: Vec::new(),
            language_confidences: HashMap::new(),
            overall_confidence: 0.0,
            metadata: HashMap::new(),
        }
    }

    /// Count issues by severity
    pub fn issue_counts(&self) -> HashMap<Severity, usize> {
        let mut counts = HashMap::new();
        for issue in &self.issues {
            *counts.entry(issue.severity).or_insert(0) += 1;
        }
        counts
    }

    /// Get issues of a specific severity
    pub fn issues_by_severity(&self, severity: Severity) -> Vec<&Issue> {
        self.issues.iter().filter(|i| i.severity == severity).collect()
    }

    /// Get high-confidence suggestions
    pub fn high_confidence_suggestions(&self, threshold: f32) -> Vec<&Suggestion> {
        self.suggestions.iter().filter(|s| s.confidence >= threshold).collect()
    }

    /// Get summary statistics
    pub fn summary(&self) -> MergedOutputSummary {
        let counts = self.issue_counts();
        MergedOutputSummary {
            total_issues: self.issues.len(),
            critical_issues: *counts.get(&Severity::Critical).unwrap_or(&0),
            error_issues: *counts.get(&Severity::Error).unwrap_or(&0),
            warning_issues: *counts.get(&Severity::Warning).unwrap_or(&0),
            info_issues: *counts.get(&Severity::Info).unwrap_or(&0),
            total_suggestions: self.suggestions.len(),
            languages_analyzed: self.language_confidences.len(),
            overall_confidence: self.overall_confidence,
        }
    }
}

/// Summary statistics for merged output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergedOutputSummary {
    pub total_issues: usize,
    pub critical_issues: usize,
    pub error_issues: usize,
    pub warning_issues: usize,
    pub info_issues: usize,
    pub total_suggestions: usize,
    pub languages_analyzed: usize,
    pub overall_confidence: f32,
}

/// Conflict resolution information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictResolution {
    pub location: String,
    pub conflicting_languages: Vec<String>,
    pub resolved_language: String,
    pub resolution_reason: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Location;

    fn create_test_issue(pattern: &str, severity: Severity, file: &str, line: u32) -> Issue {
        Issue {
            pattern: pattern.to_string(),
            severity,
            location: Location {
                file_path: file.to_string(),
                start_line: line,
                end_line: line,
                start_col: 0,
                end_col: 0,
            },
            message: format!("Test issue: {}", pattern),
            suggested_fix: None,
        }
    }

    fn create_test_suggestion(pattern: &str, confidence: f32) -> Suggestion {
        Suggestion {
            pattern: pattern.to_string(),
            description: format!("Test suggestion: {}", pattern),
            confidence,
            location: None,
        }
    }

    #[test]
    fn test_merger_creation() {
        let merger = MergerModel::new();
        assert!(merger.deduplicate);
        assert_eq!(merger.severity_threshold, Severity::Info);
    }

    #[test]
    fn test_merge_empty_outputs() {
        let merger = MergerModel::new();
        let merged = merger.merge(vec![]);
        assert_eq!(merged.issues.len(), 0);
        assert_eq!(merged.suggestions.len(), 0);
        assert_eq!(merged.overall_confidence, 0.0);
    }

    #[test]
    fn test_merge_single_output() {
        let merger = MergerModel::new();
        let output = ExpertOutput {
            language: "rust".to_string(),
            issues: vec![
                create_test_issue("unwrap_chain", Severity::Warning, "test.rs", 10),
            ],
            suggestions: vec![
                create_test_suggestion("use_question_mark", 0.9),
            ],
            confidence: 0.85,
            metadata: HashMap::new(),
        };

        let merged = merger.merge(vec![output]);
        assert_eq!(merged.issues.len(), 1);
        assert_eq!(merged.suggestions.len(), 1);
        assert!((merged.overall_confidence - 0.85).abs() < 0.001);
    }

    #[test]
    fn test_merge_multiple_outputs() {
        let merger = MergerModel::new();

        let rust_output = ExpertOutput {
            language: "rust".to_string(),
            issues: vec![
                create_test_issue("unwrap_chain", Severity::Warning, "test.rs", 10),
            ],
            suggestions: vec![],
            confidence: 0.9,
            metadata: HashMap::new(),
        };

        let cpp_output = ExpertOutput {
            language: "cpp".to_string(),
            issues: vec![
                create_test_issue("memory_leak", Severity::Error, "test.cpp", 20),
            ],
            suggestions: vec![],
            confidence: 0.8,
            metadata: HashMap::new(),
        };

        let merged = merger.merge(vec![rust_output, cpp_output]);
        assert_eq!(merged.issues.len(), 2);
        assert_eq!(merged.language_confidences.len(), 2);

        // Error should come before Warning
        assert_eq!(merged.issues[0].severity, Severity::Error);
        assert_eq!(merged.issues[1].severity, Severity::Warning);
    }

    #[test]
    fn test_deduplication() {
        let merger = MergerModel::new();

        let output1 = ExpertOutput {
            language: "rust".to_string(),
            issues: vec![
                create_test_issue("unwrap_chain", Severity::Warning, "test.rs", 10),
                create_test_issue("unwrap_chain", Severity::Warning, "test.rs", 10), // Duplicate
            ],
            suggestions: vec![],
            confidence: 0.9,
            metadata: HashMap::new(),
        };

        let merged = merger.merge(vec![output1]);
        assert_eq!(merged.issues.len(), 1); // Duplicate removed
    }

    #[test]
    fn test_severity_filtering() {
        let mut merger = MergerModel::new();
        merger.severity_threshold = Severity::Warning;

        let output = ExpertOutput {
            language: "rust".to_string(),
            issues: vec![
                create_test_issue("info_issue", Severity::Info, "test.rs", 10),
                create_test_issue("warning_issue", Severity::Warning, "test.rs", 20),
                create_test_issue("error_issue", Severity::Error, "test.rs", 30),
            ],
            suggestions: vec![],
            confidence: 0.9,
            metadata: HashMap::new(),
        };

        let merged = merger.merge(vec![output]);
        assert_eq!(merged.issues.len(), 2); // Only Warning and Error
        assert!(merged.issues.iter().all(|i| i.severity >= Severity::Warning));
    }

    #[test]
    fn test_summary() {
        let merger = MergerModel::new();
        let output = ExpertOutput {
            language: "rust".to_string(),
            issues: vec![
                create_test_issue("critical", Severity::Critical, "test.rs", 10),
                create_test_issue("error", Severity::Error, "test.rs", 20),
                create_test_issue("warning", Severity::Warning, "test.rs", 30),
                create_test_issue("info", Severity::Info, "test.rs", 40),
            ],
            suggestions: vec![
                create_test_suggestion("suggestion1", 0.9),
                create_test_suggestion("suggestion2", 0.8),
            ],
            confidence: 0.85,
            metadata: HashMap::new(),
        };

        let merged = merger.merge(vec![output]);
        let summary = merged.summary();

        assert_eq!(summary.total_issues, 4);
        assert_eq!(summary.critical_issues, 1);
        assert_eq!(summary.error_issues, 1);
        assert_eq!(summary.warning_issues, 1);
        assert_eq!(summary.info_issues, 1);
        assert_eq!(summary.total_suggestions, 2);
        assert_eq!(summary.languages_analyzed, 1);
    }
}
