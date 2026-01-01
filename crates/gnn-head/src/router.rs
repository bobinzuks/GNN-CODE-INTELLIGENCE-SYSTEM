//! Router module for language detection and expert weighting

use crate::{CodeGraph, ParsedProject};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// RouterModel determines which experts to activate and their weights
#[derive(Debug)]
pub struct RouterModel {
    pub min_weight: f32,
}

impl RouterModel {
    /// Create a new router with default settings
    pub fn new() -> Self {
        Self {
            min_weight: 0.05, // Minimum weight for an expert to be activated
        }
    }

    /// Route a project to appropriate language experts
    pub fn route(&self, project: &ParsedProject) -> Vec<LanguageRoute> {
        let mut routes = Vec::new();

        // Analyze language distribution in the project
        let lang_stats = self.analyze_language_distribution(&project.graph);

        // Calculate weights for each language
        for (language, stats) in lang_stats {
            let weight = self.calculate_weight(&stats, project);

            if weight >= self.min_weight {
                routes.push(LanguageRoute {
                    language: language.clone(),
                    weight,
                });
            }
        }

        // Normalize weights to sum to 1.0
        self.normalize_weights(&mut routes);

        // Sort by weight descending
        routes.sort_by(|a, b| b.weight.partial_cmp(&a.weight).unwrap());

        routes
    }

    /// Analyze language distribution in the code graph
    fn analyze_language_distribution(&self, graph: &CodeGraph) -> HashMap<String, LanguageStats> {
        let mut stats: HashMap<String, LanguageStats> = HashMap::new();

        for node_idx in graph.node_indices() {
            let node = &graph[node_idx];
            let lang_stats = stats.entry(node.language.clone()).or_insert_with(|| LanguageStats {
                language: node.language.clone(),
                node_count: 0,
                function_count: 0,
                class_count: 0,
                line_count: 0,
                complexity: 0.0,
            });

            lang_stats.node_count += 1;

            match node.kind {
                crate::NodeKind::Function | crate::NodeKind::Method => {
                    lang_stats.function_count += 1;
                }
                crate::NodeKind::Class | crate::NodeKind::Struct => {
                    lang_stats.class_count += 1;
                }
                _ => {}
            }

            lang_stats.line_count += node.end_line.saturating_sub(node.start_line);

            // Estimate complexity based on node properties
            let complexity_factor = if node.is_async { 1.2 } else { 1.0 }
                * if node.is_generic { 1.3 } else { 1.0 };
            lang_stats.complexity += complexity_factor;
        }

        stats
    }

    /// Calculate weight for a language based on its statistics
    fn calculate_weight(&self, stats: &LanguageStats, project: &ParsedProject) -> f32 {
        // Node count ratio
        let node_ratio = stats.node_count as f32 / project.node_count.max(1) as f32;

        // Line count contribution (weighted less than node count)
        let total_lines: u32 = stats.line_count;
        let line_factor = (total_lines as f32).ln().max(1.0) / 10.0;

        // Complexity contribution
        let complexity_factor = (stats.complexity / stats.node_count.max(1) as f32) / 5.0;

        // Structural importance (functions and classes)
        let structural_factor = (stats.function_count + stats.class_count * 2) as f32 /
                                project.node_count.max(1) as f32;

        // Combine factors
        let weight = node_ratio * 0.5 +
                     line_factor * 0.1 +
                     complexity_factor.min(0.2) * 0.2 +
                     structural_factor.min(0.3) * 0.2;

        weight.clamp(0.0, 1.0)
    }

    /// Normalize weights so they sum to 1.0
    fn normalize_weights(&self, routes: &mut [LanguageRoute]) {
        if routes.is_empty() {
            return;
        }

        let total: f32 = routes.iter().map(|r| r.weight).sum();
        if total > 0.0 {
            for route in routes.iter_mut() {
                route.weight /= total;
            }
        }
    }

    /// Predict language for a code snippet (simple heuristic-based)
    pub fn predict_language(&self, code: &str, file_extension: Option<&str>) -> Option<String> {
        // First try by file extension
        if let Some(ext) = file_extension {
            if let Some(lang) = self.extension_to_language(ext) {
                return Some(lang.to_string());
            }
        }

        // Fall back to content-based detection
        self.detect_by_content(code)
    }

    /// Map file extension to language
    fn extension_to_language(&self, ext: &str) -> Option<&'static str> {
        match ext {
            "rs" => Some("rust"),
            "cpp" | "cc" | "cxx" | "hpp" | "h" => Some("cpp"),
            "go" => Some("go"),
            "ts" | "tsx" => Some("typescript"),
            "js" | "jsx" => Some("javascript"),
            "py" => Some("python"),
            "java" => Some("java"),
            "swift" => Some("swift"),
            "cs" => Some("csharp"),
            "zig" => Some("zig"),
            _ => None,
        }
    }

    /// Detect language by analyzing code content
    fn detect_by_content(&self, code: &str) -> Option<String> {
        // Simple heuristic-based detection
        if code.contains("fn ") && code.contains("let ") {
            Some("rust".to_string())
        } else if code.contains("func ") && code.contains("package ") {
            Some("go".to_string())
        } else if code.contains("class ") && code.contains("public static void main") {
            Some("java".to_string())
        } else if code.contains("function ") || code.contains("const ") {
            Some("javascript".to_string())
        } else {
            None
        }
    }
}

impl Default for RouterModel {
    fn default() -> Self {
        Self::new()
    }
}

/// Language routing information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageRoute {
    pub language: String,
    pub weight: f32,
}

/// Statistics for a language in the project
#[derive(Debug, Clone)]
struct LanguageStats {
    language: String,
    node_count: u32,
    function_count: u32,
    class_count: u32,
    line_count: u32,
    complexity: f32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{CodeNode, NodeKind, Visibility};
    use std::collections::{HashMap, HashSet};

    #[test]
    fn test_router_creation() {
        let router = RouterModel::new();
        assert_eq!(router.min_weight, 0.05);
    }

    #[test]
    fn test_extension_to_language() {
        let router = RouterModel::new();
        assert_eq!(router.extension_to_language("rs"), Some("rust"));
        assert_eq!(router.extension_to_language("go"), Some("go"));
        assert_eq!(router.extension_to_language("unknown"), None);
    }

    #[test]
    fn test_detect_by_content() {
        let router = RouterModel::new();

        let rust_code = "fn main() { let x = 5; }";
        assert_eq!(router.detect_by_content(rust_code), Some("rust".to_string()));

        let go_code = "package main\nfunc main() {}";
        assert_eq!(router.detect_by_content(go_code), Some("go".to_string()));
    }

    #[test]
    fn test_route_single_language() {
        let router = RouterModel::new();
        let mut graph = CodeGraph::new();

        // Add some Rust nodes
        for i in 0..10 {
            graph.add_node(CodeNode {
                kind: NodeKind::Function,
                name: format!("fn_{}", i),
                language: "rust".to_string(),
                file_path: Some("test.rs".to_string()),
                start_line: i * 10,
                end_line: i * 10 + 10,
                start_col: 0,
                end_col: 0,
                signature: None,
                visibility: Visibility::Public,
                is_async: false,
                is_static: false,
                is_generic: false,
                metadata: HashMap::new(),
            });
        }

        let project = ParsedProject {
            graph,
            languages: vec!["rust".to_string()].into_iter().collect::<HashSet<_>>(),
            file_count: 1,
            node_count: 10,
            edge_count: 0,
        };

        let routes = router.route(&project);
        assert_eq!(routes.len(), 1);
        assert_eq!(routes[0].language, "rust");
        assert!((routes[0].weight - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_route_multi_language() {
        let router = RouterModel::new();
        let mut graph = CodeGraph::new();

        // Add 7 Rust nodes
        for i in 0..7 {
            graph.add_node(CodeNode {
                kind: NodeKind::Function,
                name: format!("fn_{}", i),
                language: "rust".to_string(),
                file_path: Some("test.rs".to_string()),
                start_line: i * 10,
                end_line: i * 10 + 10,
                start_col: 0,
                end_col: 0,
                signature: None,
                visibility: Visibility::Public,
                is_async: false,
                is_static: false,
                is_generic: false,
                metadata: HashMap::new(),
            });
        }

        // Add 3 Go nodes
        for i in 0..3 {
            graph.add_node(CodeNode {
                kind: NodeKind::Function,
                name: format!("func_{}", i),
                language: "go".to_string(),
                file_path: Some("test.go".to_string()),
                start_line: i * 10,
                end_line: i * 10 + 10,
                start_col: 0,
                end_col: 0,
                signature: None,
                visibility: Visibility::Public,
                is_async: false,
                is_static: false,
                is_generic: false,
                metadata: HashMap::new(),
            });
        }

        let mut languages = HashSet::new();
        languages.insert("rust".to_string());
        languages.insert("go".to_string());

        let project = ParsedProject {
            graph,
            languages,
            file_count: 2,
            node_count: 10,
            edge_count: 0,
        };

        let routes = router.route(&project);
        assert_eq!(routes.len(), 2);

        // Rust should have higher weight
        assert_eq!(routes[0].language, "rust");
        assert!(routes[0].weight > routes[1].weight);

        // Weights should sum to 1.0
        let total: f32 = routes.iter().map(|r| r.weight).sum();
        assert!((total - 1.0).abs() < 0.001);
    }
}
