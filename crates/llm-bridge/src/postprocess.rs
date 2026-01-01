//! Post-Processing - Code validation and fixing with GNN feedback
//!
//! This module validates LLM-generated code against GNN embeddings
//! and provides automatic fixes when validation fails.

use crate::{LLMBridgeError, Result};
use serde::{Deserialize, Serialize};

/// Post-processor for validation and fixing
pub struct PostProcessor {
    config: PostProcessConfig,
}

/// Configuration for post-processing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostProcessConfig {
    /// Enable syntax validation
    pub validate_syntax: bool,

    /// Enable semantic validation against GNN
    pub validate_semantics: bool,

    /// Similarity threshold for semantic validation
    pub semantic_threshold: f32,

    /// Enable automatic fixing
    pub auto_fix: bool,

    /// Maximum fix iterations
    pub max_fix_iterations: usize,
}

impl Default for PostProcessConfig {
    fn default() -> Self {
        Self {
            validate_syntax: true,
            validate_semantics: true,
            semantic_threshold: 0.7,
            auto_fix: true,
            max_fix_iterations: 3,
        }
    }
}

/// Result of validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    /// Whether the code is valid
    pub is_valid: bool,

    /// List of validation issues
    pub issues: Vec<String>,

    /// Semantic similarity score (if available)
    pub semantic_similarity: Option<f32>,

    /// Syntax errors (if any)
    pub syntax_errors: Vec<SyntaxError>,

    /// Suggestions for improvement
    pub suggestions: Vec<String>,
}

/// Syntax error information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyntaxError {
    /// Line number
    pub line: usize,

    /// Column number
    pub column: usize,

    /// Error message
    pub message: String,

    /// Error severity
    pub severity: ErrorSeverity,
}

/// Error severity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ErrorSeverity {
    /// Critical error that prevents execution
    Error,
    /// Warning that should be addressed
    Warning,
    /// Informational note
    Info,
}

/// Code fix with diff
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeFix {
    /// Original code
    pub original: String,

    /// Fixed code
    pub fixed_code: String,

    /// Diff in unified format
    pub diff: String,

    /// Description of changes
    pub changes: Vec<Change>,

    /// Fix confidence score
    pub confidence: f32,
}

/// A single change in the fix
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Change {
    /// Type of change
    pub change_type: ChangeType,

    /// Line number affected
    pub line: usize,

    /// Description of the change
    pub description: String,
}

/// Type of change
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChangeType {
    /// Added a line
    Addition,
    /// Removed a line
    Deletion,
    /// Modified a line
    Modification,
    /// Moved a line
    Move,
}

impl PostProcessor {
    /// Create a new post-processor with default configuration
    pub fn new() -> Self {
        Self {
            config: PostProcessConfig::default(),
        }
    }

    /// Create a post-processor with custom configuration
    pub fn with_config(config: PostProcessConfig) -> Self {
        Self { config }
    }

    /// Validate generated code against GNN embedding
    pub fn validate(&self, code: &str, gnn_embedding: &[f32]) -> Result<ValidationResult> {
        let mut issues = Vec::new();
        let mut syntax_errors = Vec::new();
        let mut suggestions = Vec::new();

        // Syntax validation
        if self.config.validate_syntax {
            let syntax_result = self.validate_syntax(code)?;
            syntax_errors.extend(syntax_result.errors);
            if !syntax_result.is_valid {
                issues.push("Code contains syntax errors".to_string());
            }
        }

        // Semantic validation
        let semantic_similarity = if self.config.validate_semantics {
            let similarity = self.compute_semantic_similarity(code, gnn_embedding)?;
            if similarity < self.config.semantic_threshold {
                issues.push(format!(
                    "Semantic similarity too low: {:.3} < {:.3}",
                    similarity, self.config.semantic_threshold
                ));
            }
            Some(similarity)
        } else {
            None
        };

        // Additional heuristic checks
        self.check_code_quality(code, &mut issues, &mut suggestions)?;

        let is_valid = issues.is_empty() && syntax_errors.is_empty();

        Ok(ValidationResult {
            is_valid,
            issues,
            semantic_similarity,
            syntax_errors,
            suggestions,
        })
    }

    /// Validate syntax
    fn validate_syntax(&self, code: &str) -> Result<SyntaxValidationResult> {
        let mut errors = Vec::new();
        let mut is_valid = true;

        // Basic syntax checks (simplified - in production would use tree-sitter or similar)

        // Check for balanced braces
        let mut brace_stack = Vec::new();
        for (line_idx, line) in code.lines().enumerate() {
            for (col_idx, ch) in line.chars().enumerate() {
                match ch {
                    '{' | '[' | '(' => brace_stack.push((ch, line_idx, col_idx)),
                    '}' | ']' | ')' => {
                        if let Some((open, _, _)) = brace_stack.pop() {
                            let matches = match (open, ch) {
                                ('{', '}') | ('[', ']') | ('(', ')') => true,
                                _ => false,
                            };
                            if !matches {
                                errors.push(SyntaxError {
                                    line: line_idx + 1,
                                    column: col_idx + 1,
                                    message: format!(
                                        "Mismatched brackets: expected closing for '{}', found '{}'",
                                        open, ch
                                    ),
                                    severity: ErrorSeverity::Error,
                                });
                                is_valid = false;
                            }
                        } else {
                            errors.push(SyntaxError {
                                line: line_idx + 1,
                                column: col_idx + 1,
                                message: format!("Unmatched closing bracket '{}'", ch),
                                severity: ErrorSeverity::Error,
                            });
                            is_valid = false;
                        }
                    }
                    _ => {}
                }
            }
        }

        // Check for unclosed braces
        if !brace_stack.is_empty() {
            for (open, line, col) in brace_stack {
                errors.push(SyntaxError {
                    line: line + 1,
                    column: col + 1,
                    message: format!("Unclosed bracket '{}'", open),
                    severity: ErrorSeverity::Error,
                });
            }
            is_valid = false;
        }

        // Check for incomplete strings
        for (line_idx, line) in code.lines().enumerate() {
            let mut in_string = false;
            let mut escape_next = false;
            let mut string_char = ' ';

            for ch in line.chars() {
                if escape_next {
                    escape_next = false;
                    continue;
                }

                match ch {
                    '\\' if in_string => escape_next = true,
                    '"' | '\'' => {
                        if in_string && ch == string_char {
                            in_string = false;
                        } else if !in_string {
                            in_string = true;
                            string_char = ch;
                        }
                    }
                    _ => {}
                }
            }

            if in_string && !line.ends_with('\\') {
                errors.push(SyntaxError {
                    line: line_idx + 1,
                    column: line.len(),
                    message: "Unterminated string literal".to_string(),
                    severity: ErrorSeverity::Warning,
                });
            }
        }

        Ok(SyntaxValidationResult { is_valid, errors })
    }

    /// Compute semantic similarity between generated code and expected embedding
    fn compute_semantic_similarity(&self, code: &str, gnn_embedding: &[f32]) -> Result<f32> {
        // In production, this would:
        // 1. Parse the generated code
        // 2. Extract features
        // 3. Compute embedding
        // 4. Calculate cosine similarity

        // Simplified heuristic based on code characteristics
        let code_features = self.extract_simple_features(code);

        // Compute similarity based on feature match
        let similarity = self.compute_feature_similarity(&code_features, gnn_embedding);

        Ok(similarity)
    }

    /// Extract simple features from code
    fn extract_simple_features(&self, code: &str) -> Vec<f32> {
        let mut features = vec![0.0; 512];

        // Feature 0: Code length (normalized)
        features[0] = (code.len() as f32 / 1000.0).min(1.0);

        // Feature 1: Line count (normalized)
        features[1] = (code.lines().count() as f32 / 100.0).min(1.0);

        // Feature 2: Function count
        features[2] = (code.matches("fn ").count() as f32 / 10.0).min(1.0);

        // Feature 3: Brace density
        let brace_count = code.chars().filter(|&c| c == '{' || c == '}').count();
        features[3] = (brace_count as f32 / code.len() as f32).min(1.0);

        // Feature 4: Comment density
        let comment_lines = code.lines().filter(|l| l.trim().starts_with("//")).count();
        features[4] = (comment_lines as f32 / code.lines().count() as f32).min(1.0);

        // Fill remaining features with derived values
        for i in 5..512 {
            features[i] = ((i as f32).sin() * features[i % 5]).abs();
        }

        features
    }

    /// Compute feature similarity (cosine similarity)
    fn compute_feature_similarity(&self, features: &[f32], gnn_embedding: &[f32]) -> f32 {
        if features.len() != gnn_embedding.len() {
            return 0.0;
        }

        let dot_product: f32 = features
            .iter()
            .zip(gnn_embedding.iter())
            .map(|(a, b)| a * b)
            .sum();

        let norm_a: f32 = features.iter().map(|x| x * x).sum::<f32>().sqrt();
        let norm_b: f32 = gnn_embedding.iter().map(|x| x * x).sum::<f32>().sqrt();

        if norm_a < 1e-8 || norm_b < 1e-8 {
            return 0.0;
        }

        (dot_product / (norm_a * norm_b)).max(0.0).min(1.0)
    }

    /// Check code quality with heuristics
    fn check_code_quality(
        &self,
        code: &str,
        issues: &mut Vec<String>,
        suggestions: &mut Vec<String>,
    ) -> Result<()> {
        // Check for empty code
        if code.trim().is_empty() {
            issues.push("Generated code is empty".to_string());
            return Ok(());
        }

        // Check for TODO/FIXME comments
        if code.contains("TODO") || code.contains("FIXME") {
            suggestions.push("Code contains TODO/FIXME comments that should be addressed".to_string());
        }

        // Check for very long lines
        for (idx, line) in code.lines().enumerate() {
            if line.len() > 120 {
                suggestions.push(format!(
                    "Line {} is very long ({} characters), consider breaking it up",
                    idx + 1,
                    line.len()
                ));
            }
        }

        // Check for lack of comments
        let total_lines = code.lines().count();
        let comment_lines = code.lines().filter(|l| l.trim().starts_with("//")).count();
        if total_lines > 20 && comment_lines == 0 {
            suggestions.push("Consider adding comments to explain the code".to_string());
        }

        // Check for proper function documentation
        for (idx, line) in code.lines().enumerate() {
            if line.trim().starts_with("fn ") || line.trim().starts_with("pub fn ") {
                // Check if previous line is a doc comment
                if idx == 0 || !code.lines().nth(idx - 1).unwrap_or("").trim().starts_with("///") {
                    suggestions.push(format!(
                        "Function at line {} could benefit from documentation",
                        idx + 1
                    ));
                }
            }
        }

        Ok(())
    }

    /// Generate a fix for validation issues
    pub fn generate_fix(&self, code: &str, validation: &ValidationResult) -> Result<Option<CodeFix>> {
        if !self.config.auto_fix || validation.is_valid {
            return Ok(None);
        }

        let mut fixed_code = code.to_string();
        let mut changes = Vec::new();

        // Fix syntax errors
        for error in &validation.syntax_errors {
            if let Some((new_code, change)) = self.fix_syntax_error(&fixed_code, error)? {
                fixed_code = new_code;
                changes.push(change);
            }
        }

        // Generate diff
        let diff = self.generate_diff(code, &fixed_code);

        // Calculate confidence based on number of fixes
        let confidence = if changes.is_empty() {
            0.0
        } else {
            (1.0 / (changes.len() as f32 + 1.0)).max(0.3)
        };

        Ok(Some(CodeFix {
            original: code.to_string(),
            fixed_code,
            diff,
            changes,
            confidence,
        }))
    }

    /// Fix a specific syntax error
    fn fix_syntax_error(&self, code: &str, error: &SyntaxError) -> Result<Option<(String, Change)>> {
        // Simplified fixing logic
        if error.message.contains("Unclosed bracket") {
            let missing_bracket = if error.message.contains("'{'") {
                '}'
            } else if error.message.contains("'['") {
                ']'
            } else if error.message.contains("'('") {
                ')'
            } else {
                return Ok(None);
            };

            let mut fixed = code.to_string();
            fixed.push('\n');
            fixed.push(missing_bracket);

            let change = Change {
                change_type: ChangeType::Addition,
                line: code.lines().count() + 1,
                description: format!("Added missing closing bracket '{}'", missing_bracket),
            };

            return Ok(Some((fixed, change)));
        }

        Ok(None)
    }

    /// Generate unified diff
    fn generate_diff(&self, original: &str, fixed: &str) -> String {
        let mut diff = String::from("--- original\n+++ fixed\n");

        let original_lines: Vec<&str> = original.lines().collect();
        let fixed_lines: Vec<&str> = fixed.lines().collect();

        // Simple line-by-line diff
        let max_lines = original_lines.len().max(fixed_lines.len());

        for i in 0..max_lines {
            let orig_line = original_lines.get(i);
            let fixed_line = fixed_lines.get(i);

            match (orig_line, fixed_line) {
                (Some(o), Some(f)) if o != f => {
                    diff.push_str(&format!("-{}\n", o));
                    diff.push_str(&format!("+{}\n", f));
                }
                (Some(o), None) => {
                    diff.push_str(&format!("-{}\n", o));
                }
                (None, Some(f)) => {
                    diff.push_str(&format!("+{}\n", f));
                }
                _ => {
                    if let Some(line) = orig_line {
                        diff.push_str(&format!(" {}\n", line));
                    }
                }
            }
        }

        diff
    }

    /// Get configuration
    pub fn config(&self) -> &PostProcessConfig {
        &self.config
    }
}

impl Default for PostProcessor {
    fn default() -> Self {
        Self::new()
    }
}

/// Internal result for syntax validation
struct SyntaxValidationResult {
    is_valid: bool,
    errors: Vec<SyntaxError>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_balanced_braces() {
        let processor = PostProcessor::new();
        let code = "fn main() { println!(\"Hello\"); }";
        let result = processor.validate_syntax(code).unwrap();
        assert!(result.is_valid);
        assert!(result.errors.is_empty());
    }

    #[test]
    fn test_unbalanced_braces() {
        let processor = PostProcessor::new();
        let code = "fn main() { println!(\"Hello\");";
        let result = processor.validate_syntax(code).unwrap();
        assert!(!result.is_valid);
        assert!(!result.errors.is_empty());
    }

    #[test]
    fn test_semantic_similarity() {
        let processor = PostProcessor::new();
        let code = "fn test() {}";
        let embedding = vec![0.5; 512];
        let similarity = processor.compute_semantic_similarity(code, &embedding).unwrap();
        assert!(similarity >= 0.0 && similarity <= 1.0);
    }

    #[test]
    fn test_validation() {
        let processor = PostProcessor::new();
        let code = "fn main() { println!(\"Hello, world!\"); }";
        let embedding = vec![0.5; 512];
        let result = processor.validate(code, &embedding).unwrap();
        assert!(result.is_valid || !result.issues.is_empty());
    }

    #[test]
    fn test_empty_code() {
        let processor = PostProcessor::new();
        let code = "";
        let embedding = vec![0.5; 512];
        let result = processor.validate(code, &embedding).unwrap();
        assert!(!result.is_valid);
    }

    #[test]
    fn test_diff_generation() {
        let processor = PostProcessor::new();
        let original = "line1\nline2\nline3";
        let fixed = "line1\nmodified\nline3";
        let diff = processor.generate_diff(original, fixed);
        assert!(diff.contains("-line2"));
        assert!(diff.contains("+modified"));
    }
}
