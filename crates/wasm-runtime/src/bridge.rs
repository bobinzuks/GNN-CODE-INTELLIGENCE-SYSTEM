//! Type conversions between Rust and JavaScript
//!
//! This module handles the conversion of complex Rust types to JSON-serializable
//! formats that can cross the WASM boundary cleanly.

use crate::bindings::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Parse code string to graph structure
pub fn parse_code_to_graph(code: &str, language: &str) -> Result<CodeGraph, String> {
    // Simple AST parsing - in production this would use tree-sitter
    let mut graph = CodeGraph::new();
    graph.languages.push(language.to_string());

    // Simple heuristic parser for demo purposes
    let lines: Vec<&str> = code.lines().collect();

    for (line_num, line) in lines.iter().enumerate() {
        let trimmed = line.trim();

        // Detect functions
        if trimmed.contains("fn ") || trimmed.contains("function ") || trimmed.contains("def ") {
            let name = extract_function_name(trimmed).unwrap_or("anonymous".to_string());
            let node = CodeNode {
                id: graph.nodes.len(),
                kind: NodeKind::Function,
                name,
                language: language.to_string(),
                file_path: None,
                start_line: line_num as u32,
                end_line: line_num as u32,
                start_col: 0,
                end_col: line.len() as u32,
                signature: Some(trimmed.to_string()),
                visibility: detect_visibility(trimmed),
                is_async: trimmed.contains("async"),
                is_static: trimmed.contains("static"),
                is_generic: trimmed.contains('<') && trimmed.contains('>'),
                metadata: HashMap::new(),
            };
            graph.add_node(node);
        }

        // Detect classes/structs
        if trimmed.contains("class ") || trimmed.contains("struct ") {
            let name = extract_type_name(trimmed).unwrap_or("Anonymous".to_string());
            let kind = if trimmed.contains("struct ") {
                NodeKind::Struct
            } else {
                NodeKind::Class
            };

            let node = CodeNode {
                id: graph.nodes.len(),
                kind,
                name,
                language: language.to_string(),
                file_path: None,
                start_line: line_num as u32,
                end_line: line_num as u32,
                start_col: 0,
                end_col: line.len() as u32,
                signature: Some(trimmed.to_string()),
                visibility: detect_visibility(trimmed),
                is_async: false,
                is_static: false,
                is_generic: trimmed.contains('<') && trimmed.contains('>'),
                metadata: HashMap::new(),
            };
            graph.add_node(node);
        }

        // Detect imports
        if trimmed.contains("import ") || trimmed.contains("use ") || trimmed.contains("require(") {
            let node = CodeNode {
                id: graph.nodes.len(),
                kind: NodeKind::Import,
                name: trimmed.to_string(),
                language: language.to_string(),
                file_path: None,
                start_line: line_num as u32,
                end_line: line_num as u32,
                start_col: 0,
                end_col: line.len() as u32,
                signature: None,
                visibility: Visibility::Private,
                is_async: false,
                is_static: false,
                is_generic: false,
                metadata: HashMap::new(),
            };
            graph.add_node(node);
        }
    }

    // Create some simple edges based on containment
    for i in 0..graph.nodes.len() {
        for j in (i + 1)..graph.nodes.len() {
            let node_i = &graph.nodes[i];
            let node_j = &graph.nodes[j];

            // Functions/methods contain other nodes
            if matches!(node_i.kind, NodeKind::Function | NodeKind::Method) &&
               node_j.start_line > node_i.start_line {
                let edge = CodeEdge {
                    from: i,
                    to: j,
                    kind: EdgeKind::Contains,
                    weight: 1.0,
                    metadata: HashMap::new(),
                };
                graph.add_edge(edge);
            }
        }
    }

    Ok(graph)
}

/// Extract function name from line
fn extract_function_name(line: &str) -> Option<String> {
    // Handle various function declaration patterns
    if let Some(pos) = line.find("fn ") {
        let after_fn = &line[pos + 3..];
        if let Some(paren) = after_fn.find('(') {
            return Some(after_fn[..paren].trim().to_string());
        }
    }

    if let Some(pos) = line.find("function ") {
        let after_fn = &line[pos + 9..];
        if let Some(paren) = after_fn.find('(') {
            return Some(after_fn[..paren].trim().to_string());
        }
    }

    if let Some(pos) = line.find("def ") {
        let after_def = &line[pos + 4..];
        if let Some(paren) = after_def.find('(') {
            return Some(after_def[..paren].trim().to_string());
        }
    }

    None
}

/// Extract type/class/struct name
fn extract_type_name(line: &str) -> Option<String> {
    for keyword in &["class ", "struct ", "interface ", "enum "] {
        if let Some(pos) = line.find(keyword) {
            let after_kw = &line[pos + keyword.len()..];
            let name = after_kw
                .split(|c: char| c.is_whitespace() || c == '{' || c == '<' || c == '(')
                .next()?
                .trim();
            if !name.is_empty() {
                return Some(name.to_string());
            }
        }
    }
    None
}

/// Detect visibility from line
fn detect_visibility(line: &str) -> Visibility {
    if line.contains("pub ") || line.contains("public ") {
        Visibility::Public
    } else if line.contains("protected ") {
        Visibility::Protected
    } else if line.contains("internal ") {
        Visibility::Internal
    } else {
        Visibility::Private
    }
}

/// Detect issues in the code graph
pub fn detect_issues(graph: &CodeGraph, _embedding: &Tensor, language: &str) -> Vec<Issue> {
    let mut issues = Vec::new();

    // Language-specific pattern detection
    match language {
        "rust" => issues.extend(detect_rust_issues(graph)),
        "javascript" | "typescript" => issues.extend(detect_js_issues(graph)),
        "python" => issues.extend(detect_python_issues(graph)),
        _ => {}
    }

    // Generic issues
    issues.extend(detect_generic_issues(graph));

    issues
}

/// Detect Rust-specific issues
fn detect_rust_issues(graph: &CodeGraph) -> Vec<Issue> {
    let mut issues = Vec::new();

    for node in &graph.nodes {
        // Check for unwrap usage in function signatures
        if let Some(sig) = &node.signature {
            if sig.contains(".unwrap()") {
                issues.push(Issue {
                    pattern: "unwrap_usage".to_string(),
                    severity: Severity::Warning,
                    location: Location {
                        file_path: node.file_path.clone(),
                        start_line: node.start_line,
                        end_line: node.end_line,
                        start_col: node.start_col,
                        end_col: node.end_col,
                    },
                    message: "Direct .unwrap() call found - consider using ? operator or proper error handling".to_string(),
                    suggested_fix: Some("Use the ? operator or match expression instead".to_string()),
                    confidence: 0.9,
                });
            }

            // Check for missing async/await
            if node.is_async && !sig.contains(".await") && sig.contains("async ") {
                issues.push(Issue {
                    pattern: "async_without_await".to_string(),
                    severity: Severity::Warning,
                    location: Location {
                        file_path: node.file_path.clone(),
                        start_line: node.start_line,
                        end_line: node.end_line,
                        start_col: node.start_col,
                        end_col: node.end_col,
                    },
                    message: "Async function may be missing .await calls".to_string(),
                    suggested_fix: Some("Ensure async results are awaited".to_string()),
                    confidence: 0.7,
                });
            }
        }
    }

    issues
}

/// Detect JavaScript/TypeScript issues
fn detect_js_issues(graph: &CodeGraph) -> Vec<Issue> {
    let mut issues = Vec::new();

    for node in &graph.nodes {
        if let Some(sig) = &node.signature {
            // Check for var usage
            if sig.contains("var ") {
                issues.push(Issue {
                    pattern: "var_usage".to_string(),
                    severity: Severity::Warning,
                    location: Location {
                        file_path: node.file_path.clone(),
                        start_line: node.start_line,
                        end_line: node.end_line,
                        start_col: node.start_col,
                        end_col: node.end_col,
                    },
                    message: "Use of 'var' - consider using 'let' or 'const'".to_string(),
                    suggested_fix: Some("Replace 'var' with 'let' or 'const'".to_string()),
                    confidence: 0.95,
                });
            }

            // Check for == usage
            if sig.contains("==") && !sig.contains("===") {
                issues.push(Issue {
                    pattern: "loose_equality".to_string(),
                    severity: Severity::Info,
                    location: Location {
                        file_path: node.file_path.clone(),
                        start_line: node.start_line,
                        end_line: node.end_line,
                        start_col: node.start_col,
                        end_col: node.end_col,
                    },
                    message: "Use of loose equality (==) - consider strict equality (===)".to_string(),
                    suggested_fix: Some("Replace '==' with '==='".to_string()),
                    confidence: 0.85,
                });
            }
        }
    }

    issues
}

/// Detect Python-specific issues
fn detect_python_issues(graph: &CodeGraph) -> Vec<Issue> {
    let mut issues = Vec::new();

    for node in &graph.nodes {
        if let Some(sig) = &node.signature {
            // Check for bare except
            if sig.contains("except:") && !sig.contains("except ") {
                issues.push(Issue {
                    pattern: "bare_except".to_string(),
                    severity: Severity::Warning,
                    location: Location {
                        file_path: node.file_path.clone(),
                        start_line: node.start_line,
                        end_line: node.end_line,
                        start_col: node.start_col,
                        end_col: node.end_col,
                    },
                    message: "Bare except clause - specify exception type".to_string(),
                    suggested_fix: Some("Use 'except Exception:' or specific exception type".to_string()),
                    confidence: 0.9,
                });
            }
        }
    }

    issues
}

/// Detect generic code quality issues
fn detect_generic_issues(graph: &CodeGraph) -> Vec<Issue> {
    let mut issues = Vec::new();

    // Check for very long functions
    for node in &graph.nodes {
        if matches!(node.kind, NodeKind::Function | NodeKind::Method) {
            let length = node.end_line - node.start_line;
            if length > 100 {
                issues.push(Issue {
                    pattern: "long_function".to_string(),
                    severity: Severity::Info,
                    location: Location {
                        file_path: node.file_path.clone(),
                        start_line: node.start_line,
                        end_line: node.end_line,
                        start_col: node.start_col,
                        end_col: node.end_col,
                    },
                    message: format!("Function '{}' is {} lines long - consider refactoring", node.name, length),
                    suggested_fix: Some("Break function into smaller, focused functions".to_string()),
                    confidence: 0.6,
                });
            }
        }
    }

    issues
}

/// Quick pattern-based check without models
pub fn quick_pattern_check(graph: &CodeGraph, language: &str) -> Vec<Issue> {
    detect_issues(graph, &Tensor::zeros(&[1]), language)
}

/// Generate a fix for an issue
pub fn generate_fix(issue: &Issue, _code: &str, _graph: &CodeGraph) -> Option<Fix> {
    // Generate fixes based on pattern
    match issue.pattern.as_str() {
        "unwrap_usage" => Some(Fix {
            issue_pattern: issue.pattern.clone(),
            location: issue.location.clone(),
            original_code: "".to_string(),
            fixed_code: "".to_string(),
            description: "Replace .unwrap() with ? operator".to_string(),
            confidence: issue.confidence,
        }),
        "var_usage" => Some(Fix {
            issue_pattern: issue.pattern.clone(),
            location: issue.location.clone(),
            original_code: "".to_string(),
            fixed_code: "".to_string(),
            description: "Replace var with const or let".to_string(),
            confidence: issue.confidence,
        }),
        _ => None,
    }
}

/// Apply a single fix to code
pub fn apply_single_fix(code: &str, fix: &Fix) -> Result<String, String> {
    // Simple line-based replacement
    let lines: Vec<&str> = code.lines().collect();
    let mut result = Vec::new();

    for (i, line) in lines.iter().enumerate() {
        let line_num = i as u32;

        if line_num >= fix.location.start_line && line_num <= fix.location.end_line {
            // Apply fix to this line
            match fix.issue_pattern.as_str() {
                "var_usage" => {
                    result.push(line.replace("var ", "const "));
                }
                "loose_equality" => {
                    result.push(line.replace("==", "==="));
                }
                _ => {
                    result.push(line.to_string());
                }
            }
        } else {
            result.push(line.to_string());
        }
    }

    Ok(result.join("\n"))
}

/// Feature extractor for code nodes
pub struct FeatureExtractor {
    feature_dim: usize,
}

impl FeatureExtractor {
    pub fn new(feature_dim: usize) -> Self {
        FeatureExtractor { feature_dim }
    }

    pub fn extract(&self, graph: &CodeGraph) -> HashMap<usize, Tensor> {
        graph.nodes.iter().enumerate()
            .map(|(id, node)| (id, self.node_features(node)))
            .collect()
    }

    fn node_features(&self, node: &CodeNode) -> Tensor {
        let mut features = Vec::with_capacity(self.feature_dim);

        // Node type one-hot encoding (first 32 dims)
        let node_type_id = node.kind as usize;
        for i in 0..32 {
            features.push(if i == node_type_id { 1.0 } else { 0.0 });
        }

        // Structural features
        features.push((node.end_line - node.start_line) as f32 / 100.0); // Normalized size
        features.push(if node.is_async { 1.0 } else { 0.0 });
        features.push(if node.is_static { 1.0 } else { 0.0 });
        features.push(if node.is_generic { 1.0 } else { 0.0 });

        // Visibility encoding
        features.push(match node.visibility {
            Visibility::Public => 1.0,
            Visibility::Protected => 0.66,
            Visibility::Internal => 0.33,
            Visibility::Private => 0.0,
        });

        // Pad to feature_dim
        while features.len() < self.feature_dim {
            features.push(0.0);
        }

        features.truncate(self.feature_dim);

        Tensor::from_vec(features, vec![self.feature_dim])
    }
}

/// Model store for managing loaded models
pub struct ModelStore {
    models: HashMap<String, GNNModel>,
}

impl ModelStore {
    pub fn new() -> Self {
        ModelStore {
            models: HashMap::new(),
        }
    }

    pub fn load_from_bytes(&mut self, bytes: &[u8], language: &str) -> Result<&GNNModel, String> {
        // Deserialize model from bytes
        let model: GNNModel = bincode::deserialize(bytes)
            .or_else(|_| {
                // If deserialize fails, create a default model
                Ok(GNNModel::new(128, vec![256, 256], 512))
            })
            .map_err(|e: bincode::Error| format!("Deserialization error: {}", e))?;

        self.models.insert(language.to_string(), model);
        Ok(self.models.get(language).unwrap())
    }

    pub fn get(&self, language: &str) -> Option<&GNNModel> {
        self.models.get(language)
    }

    pub fn has(&self, language: &str) -> bool {
        self.models.contains_key(language)
    }
}
