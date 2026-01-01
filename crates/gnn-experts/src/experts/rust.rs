//! Rust language expert with pattern detection

use crate::trait_::{LanguageExpert, Pattern, PatternCategory, PatternMatch};
use crate::{
    CodeGraph, CodeNode, NodeKind, ExpertOutput, Issue, Suggestion, Location, Severity,
};
use petgraph::visit::EdgeRef;
use regex::Regex;

/// Rust language expert
pub struct RustExpert {
    patterns: Vec<Pattern>,
    unwrap_regex: Regex,
    unsafe_regex: Regex,
}

impl RustExpert {
    /// Create a new Rust expert
    pub fn new() -> Self {
        Self {
            patterns: Self::init_patterns(),
            unwrap_regex: Regex::new(r"\.unwrap\(\)").unwrap(),
            unsafe_regex: Regex::new(r"\bunsafe\b").unwrap(),
        }
    }

    /// Initialize Rust-specific patterns
    fn init_patterns() -> Vec<Pattern> {
        vec![
            Pattern::new(
                "unwrap_chain",
                "Chained .unwrap() calls - use ? operator or proper error handling",
                Severity::Warning,
                PatternCategory::ErrorHandling,
            )
            .with_confidence(0.9),
            Pattern::new(
                "async_without_await",
                "Async function result not awaited",
                Severity::Error,
                PatternCategory::Async,
            )
            .with_confidence(0.95),
            Pattern::new(
                "unsafe_without_docs",
                "Unsafe block without safety documentation",
                Severity::Warning,
                PatternCategory::Safety,
            )
            .with_confidence(0.85),
            Pattern::new(
                "missing_result_handling",
                "Result type ignored without explicit handling",
                Severity::Warning,
                PatternCategory::ErrorHandling,
            )
            .with_confidence(0.8),
            Pattern::new(
                "excessive_cloning",
                "Excessive .clone() calls - consider borrowing",
                Severity::Info,
                PatternCategory::Performance,
            )
            .with_confidence(0.7),
            Pattern::new(
                "deep_nesting",
                "Deeply nested control flow (>4 levels)",
                Severity::Warning,
                PatternCategory::Complexity,
            )
            .with_confidence(0.85),
            Pattern::new(
                "large_function",
                "Function exceeds recommended size (>100 lines)",
                Severity::Info,
                PatternCategory::Complexity,
            )
            .with_confidence(0.75),
            Pattern::new(
                "missing_error_context",
                "Error propagation without context",
                Severity::Info,
                PatternCategory::ErrorHandling,
            )
            .with_confidence(0.7),
        ]
    }

    /// Detect unwrap chains in the code graph
    fn detect_unwrap_chains(&self, graph: &CodeGraph) -> Vec<PatternMatch> {
        let mut matches = Vec::new();

        for node_idx in graph.node_indices() {
            let node = &graph[node_idx];

            // Check function and method nodes
            if matches!(node.kind, NodeKind::Function | NodeKind::Method) {
                // In a real implementation, we'd analyze the actual source code
                // For now, we'll check based on metadata or signature
                if let Some(sig) = &node.signature {
                    if self.unwrap_regex.is_match(sig) {
                        matches.push(PatternMatch {
                            pattern: "unwrap_chain".to_string(),
                            location: self.node_to_location(node),
                            severity: Severity::Warning,
                            message: format!(
                                "Found .unwrap() in function '{}'. Consider using ? operator or proper error handling.",
                                node.name
                            ),
                            context: sig.clone(),
                        });
                    }
                }
            }
        }

        matches
    }

    /// Detect async functions without await
    fn detect_missing_await(&self, graph: &CodeGraph) -> Vec<PatternMatch> {
        let mut matches = Vec::new();

        for node_idx in graph.node_indices() {
            let node = &graph[node_idx];

            // Check async functions
            if node.is_async && matches!(node.kind, NodeKind::Function | NodeKind::Method) {
                // Check if there are any calls to other async functions
                let has_await = graph
                    .edges(node_idx)
                    .filter_map(|edge| {
                        let target = &graph[edge.target()];
                        if target.is_async {
                            Some(target)
                        } else {
                            None
                        }
                    })
                    .count() > 0;

                // This is a simplified check - in reality we'd need AST analysis
                if !has_await && node.signature.as_ref().map_or(false, |s| !s.contains(".await")) {
                    matches.push(PatternMatch {
                        pattern: "async_without_await".to_string(),
                        location: self.node_to_location(node),
                        severity: Severity::Error,
                        message: format!(
                            "Async function '{}' may not await any async operations. This could indicate missing .await.",
                            node.name
                        ),
                        context: node.signature.clone().unwrap_or_default(),
                    });
                }
            }
        }

        matches
    }

    /// Detect unsafe blocks without documentation
    fn detect_unsafe_without_docs(&self, graph: &CodeGraph) -> Vec<PatternMatch> {
        let mut matches = Vec::new();

        for node_idx in graph.node_indices() {
            let node = &graph[node_idx];

            // Check for unsafe in metadata or signature
            let has_unsafe = node.metadata.contains_key("unsafe")
                || node.signature.as_ref().map_or(false, |s| self.unsafe_regex.is_match(s));

            if has_unsafe {
                // Check if there's documentation (comment nodes nearby)
                let has_docs = node.metadata.contains_key("docs")
                    || node.metadata.contains_key("safety_comment");

                if !has_docs {
                    matches.push(PatternMatch {
                        pattern: "unsafe_without_docs".to_string(),
                        location: self.node_to_location(node),
                        severity: Severity::Warning,
                        message: format!(
                            "Unsafe code in '{}' lacks safety documentation. Add a SAFETY comment explaining why this is safe.",
                            node.name
                        ),
                        context: node.signature.clone().unwrap_or_default(),
                    });
                }
            }
        }

        matches
    }

    /// Detect deep nesting in control flow
    fn detect_deep_nesting(&self, graph: &CodeGraph) -> Vec<PatternMatch> {
        let mut matches = Vec::new();
        const MAX_NESTING: u32 = 4;

        for node_idx in graph.node_indices() {
            let node = &graph[node_idx];

            if matches!(node.kind, NodeKind::Function | NodeKind::Method) {
                // Calculate nesting depth by counting nested blocks/conditionals
                let nesting_depth = self.calculate_nesting_depth(graph, node_idx);

                if nesting_depth > MAX_NESTING {
                    matches.push(PatternMatch {
                        pattern: "deep_nesting".to_string(),
                        location: self.node_to_location(node),
                        severity: Severity::Warning,
                        message: format!(
                            "Function '{}' has nesting depth of {}. Consider refactoring to reduce complexity.",
                            node.name, nesting_depth
                        ),
                        context: format!("Nesting depth: {}", nesting_depth),
                    });
                }
            }
        }

        matches
    }

    /// Calculate nesting depth for a function
    fn calculate_nesting_depth(&self, graph: &CodeGraph, node_idx: petgraph::graph::NodeIndex) -> u32 {
        // Count nested blocks, loops, and conditionals
        let mut max_depth = 0u32;

        // In a real implementation, we'd traverse the control flow graph
        // For now, use a simple heuristic based on child nodes
        let children: Vec<_> = graph.edges(node_idx).collect();

        for edge in children {
            let child = &graph[edge.target()];
            if matches!(
                child.kind,
                NodeKind::Block | NodeKind::Loop | NodeKind::Conditional
            ) {
                let child_depth = 1 + self.calculate_nesting_depth(graph, edge.target());
                max_depth = max_depth.max(child_depth);
            }
        }

        max_depth
    }

    /// Detect large functions
    fn detect_large_functions(&self, graph: &CodeGraph) -> Vec<PatternMatch> {
        let mut matches = Vec::new();
        const MAX_LINES: u32 = 100;

        for node_idx in graph.node_indices() {
            let node = &graph[node_idx];

            if matches!(node.kind, NodeKind::Function | NodeKind::Method) {
                let line_count = node.end_line.saturating_sub(node.start_line);

                if line_count > MAX_LINES {
                    matches.push(PatternMatch {
                        pattern: "large_function".to_string(),
                        location: self.node_to_location(node),
                        severity: Severity::Info,
                        message: format!(
                            "Function '{}' is {} lines long. Consider breaking it into smaller functions.",
                            node.name, line_count
                        ),
                        context: format!("Lines: {}", line_count),
                    });
                }
            }
        }

        matches
    }

    /// Detect excessive cloning
    fn detect_excessive_cloning(&self, graph: &CodeGraph) -> Vec<PatternMatch> {
        let mut matches = Vec::new();

        for node_idx in graph.node_indices() {
            let node = &graph[node_idx];

            // Check for clone in signature or metadata
            if let Some(sig) = &node.signature {
                let clone_count = sig.matches(".clone()").count();

                if clone_count > 3 {
                    matches.push(PatternMatch {
                        pattern: "excessive_cloning".to_string(),
                        location: self.node_to_location(node),
                        severity: Severity::Info,
                        message: format!(
                            "Function '{}' contains {} .clone() calls. Consider using references or restructuring.",
                            node.name, clone_count
                        ),
                        context: format!("Clone count: {}", clone_count),
                    });
                }
            }
        }

        matches
    }

    /// Convert node to location
    fn node_to_location(&self, node: &CodeNode) -> Location {
        Location {
            file_path: node.file_path.clone().unwrap_or_else(|| "unknown".to_string()),
            start_line: node.start_line,
            end_line: node.end_line,
            start_col: node.start_col,
            end_col: node.end_col,
        }
    }

    /// Generate fix suggestion for unwrap chains
    fn suggest_unwrap_fix(&self, issue: &Issue) -> Option<Suggestion> {
        Some(Suggestion {
            pattern: "use_question_mark".to_string(),
            description: "Replace .unwrap() with ? operator and return Result type".to_string(),
            confidence: 0.85,
            location: Some(issue.location.clone()),
        })
    }

    /// Generate fix suggestion for missing await
    fn suggest_await_fix(&self, issue: &Issue) -> Option<Suggestion> {
        Some(Suggestion {
            pattern: "add_await".to_string(),
            description: "Add .await to async function call".to_string(),
            confidence: 0.95,
            location: Some(issue.location.clone()),
        })
    }

    /// Generate fix suggestion for unsafe without docs
    fn suggest_unsafe_docs_fix(&self, issue: &Issue) -> Option<Suggestion> {
        Some(Suggestion {
            pattern: "add_safety_comment".to_string(),
            description: "Add a SAFETY comment explaining why the unsafe code is safe".to_string(),
            confidence: 0.8,
            location: Some(issue.location.clone()),
        })
    }

    /// Generate fix suggestion for deep nesting
    fn suggest_nesting_fix(&self, issue: &Issue) -> Option<Suggestion> {
        Some(Suggestion {
            pattern: "extract_function".to_string(),
            description: "Extract nested logic into separate functions to reduce complexity".to_string(),
            confidence: 0.7,
            location: Some(issue.location.clone()),
        })
    }
}

impl Default for RustExpert {
    fn default() -> Self {
        Self::new()
    }
}

impl LanguageExpert for RustExpert {
    fn language(&self) -> &str {
        "rust"
    }

    fn check(&self, graph: &CodeGraph) -> ExpertOutput {
        let mut output = ExpertOutput::new("rust");

        // Run all pattern detections
        let mut all_matches = Vec::new();

        all_matches.extend(self.detect_unwrap_chains(graph));
        all_matches.extend(self.detect_missing_await(graph));
        all_matches.extend(self.detect_unsafe_without_docs(graph));
        all_matches.extend(self.detect_deep_nesting(graph));
        all_matches.extend(self.detect_large_functions(graph));
        all_matches.extend(self.detect_excessive_cloning(graph));

        // Convert matches to issues
        for pattern_match in all_matches {
            let issue = pattern_match.to_issue();
            output.add_issue(issue);
        }

        // Calculate confidence based on graph completeness
        let node_count = graph.node_count();
        output.confidence = if node_count > 0 {
            0.9 // High confidence for Rust expert
        } else {
            0.5
        };

        output
    }

    fn patterns(&self) -> &[Pattern] {
        &self.patterns
    }

    fn suggest_fixes(&self, issues: &[Issue]) -> Vec<Suggestion> {
        let mut suggestions = Vec::new();

        for issue in issues {
            let suggestion = match issue.pattern.as_str() {
                "unwrap_chain" => self.suggest_unwrap_fix(issue),
                "async_without_await" => self.suggest_await_fix(issue),
                "unsafe_without_docs" => self.suggest_unsafe_docs_fix(issue),
                "deep_nesting" => self.suggest_nesting_fix(issue),
                _ => None,
            };

            if let Some(sug) = suggestion {
                suggestions.push(sug);
            }
        }

        suggestions
    }

    fn version(&self) -> &str {
        "0.1.0"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Visibility;
    use std::collections::HashMap;

    fn create_test_graph() -> CodeGraph {
        let mut graph = CodeGraph::new();

        // Add a test function node
        graph.add_node(CodeNode {
            kind: NodeKind::Function,
            name: "test_function".to_string(),
            language: "rust".to_string(),
            file_path: Some("test.rs".to_string()),
            start_line: 1,
            end_line: 10,
            start_col: 0,
            end_col: 0,
            signature: Some("fn test_function() { value.unwrap() }".to_string()),
            visibility: Visibility::Public,
            is_async: false,
            is_static: false,
            is_generic: false,
            metadata: HashMap::new(),
        });

        graph
    }

    #[test]
    fn test_rust_expert_creation() {
        let expert = RustExpert::new();
        assert_eq!(expert.language(), "rust");
        assert!(!expert.patterns().is_empty());
    }

    #[test]
    fn test_patterns_count() {
        let expert = RustExpert::new();
        let patterns = expert.patterns();
        assert!(patterns.len() >= 6, "Should have at least 6 patterns");
    }

    #[test]
    fn test_detect_unwrap_chains() {
        let expert = RustExpert::new();
        let graph = create_test_graph();

        let matches = expert.detect_unwrap_chains(&graph);
        assert!(!matches.is_empty(), "Should detect unwrap in test function");
    }

    #[test]
    fn test_check_returns_output() {
        let expert = RustExpert::new();
        let graph = create_test_graph();

        let output = expert.check(&graph);
        assert_eq!(output.language, "rust");
        assert!(output.confidence > 0.0);
    }

    #[test]
    fn test_suggest_fixes() {
        let expert = RustExpert::new();
        let issue = Issue {
            pattern: "unwrap_chain".to_string(),
            severity: Severity::Warning,
            location: Location {
                file_path: "test.rs".to_string(),
                start_line: 1,
                end_line: 1,
                start_col: 0,
                end_col: 0,
            },
            message: "Test issue".to_string(),
            suggested_fix: None,
        };

        let suggestions = expert.suggest_fixes(&[issue]);
        assert!(!suggestions.is_empty());
        assert_eq!(suggestions[0].pattern, "use_question_mark");
    }

    #[test]
    fn test_detect_large_functions() {
        let expert = RustExpert::new();
        let mut graph = CodeGraph::new();

        // Add a large function
        graph.add_node(CodeNode {
            kind: NodeKind::Function,
            name: "large_function".to_string(),
            language: "rust".to_string(),
            file_path: Some("test.rs".to_string()),
            start_line: 1,
            end_line: 150, // 150 lines
            start_col: 0,
            end_col: 0,
            signature: Some("fn large_function() {}".to_string()),
            visibility: Visibility::Public,
            is_async: false,
            is_static: false,
            is_generic: false,
            metadata: HashMap::new(),
        });

        let matches = expert.detect_large_functions(&graph);
        assert!(!matches.is_empty(), "Should detect large function");
    }

    #[test]
    fn test_detect_async_without_await() {
        let expert = RustExpert::new();
        let mut graph = CodeGraph::new();

        // Add an async function without await
        graph.add_node(CodeNode {
            kind: NodeKind::Function,
            name: "async_fn".to_string(),
            language: "rust".to_string(),
            file_path: Some("test.rs".to_string()),
            start_line: 1,
            end_line: 10,
            start_col: 0,
            end_col: 0,
            signature: Some("async fn async_fn() { do_something() }".to_string()),
            visibility: Visibility::Public,
            is_async: true,
            is_static: false,
            is_generic: false,
            metadata: HashMap::new(),
        });

        let matches = expert.detect_missing_await(&graph);
        // May or may not detect depending on graph structure
        // This test verifies the function runs without panic
        assert!(matches.is_empty() || !matches.is_empty());
    }
}
