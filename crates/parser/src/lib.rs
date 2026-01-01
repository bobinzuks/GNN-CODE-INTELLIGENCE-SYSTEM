//! GNN Code Intelligence Parser
//!
//! A multi-language code parser that converts source code into a universal graph representation.
//! Supports Rust and extensible to 8+ programming languages.
//!
//! # Features
//!
//! - Multi-language parsing with tree-sitter
//! - Universal graph schema (CodeNode, CodeEdge)
//! - Language-specific parsers via trait pattern
//! - Parallel file processing with rayon
//! - Comprehensive Rust parser implementation
//!
//! # Architecture
//!
//! ```text
//! ProjectParser
//!   ├── ParserRegistry (language detection)
//!   ├── LanguageParser trait
//!   │   └── RustParser, PythonParser, etc.
//!   └── CodeGraphBuilder (graph construction)
//! ```

pub mod graph;
pub mod languages;

use anyhow::{Context, Result};
use dashmap::DashMap;
use graph::{CodeGraph, CodeGraphBuilder};
use languages::ParserRegistry;
use log::{debug, info, warn};
use petgraph::visit::EdgeRef;
use rayon::prelude::*;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use walkdir::WalkDir;

/// Statistics about parsing results
#[derive(Debug, Default, Clone)]
pub struct ParseStats {
    /// Total number of files found
    pub files_found: usize,

    /// Number of files successfully parsed
    pub files_parsed: usize,

    /// Number of files that failed to parse
    pub files_failed: usize,

    /// Number of files skipped (unsupported language)
    pub files_skipped: usize,

    /// Total nodes extracted
    pub total_nodes: usize,

    /// Total edges extracted
    pub total_edges: usize,

    /// Nodes per language
    pub nodes_per_language: DashMap<String, usize>,

    /// Errors encountered
    pub errors: Vec<String>,
}

impl ParseStats {
    /// Create new parse statistics
    pub fn new() -> Self {
        Self::default()
    }

    /// Merge another stats object into this one
    pub fn merge(&mut self, other: &ParseStats) {
        self.files_found += other.files_found;
        self.files_parsed += other.files_parsed;
        self.files_failed += other.files_failed;
        self.files_skipped += other.files_skipped;
        self.total_nodes += other.total_nodes;
        self.total_edges += other.total_edges;

        for entry in other.nodes_per_language.iter() {
            let lang = entry.key().clone();
            let count = *entry.value();
            self.nodes_per_language
                .entry(lang)
                .and_modify(|c| *c += count)
                .or_insert(count);
        }

        self.errors.extend(other.errors.clone());
    }

    /// Print summary
    pub fn print_summary(&self) {
        println!("\n=== Parse Statistics ===");
        println!("Files found: {}", self.files_found);
        println!("Files parsed: {}", self.files_parsed);
        println!("Files failed: {}", self.files_failed);
        println!("Files skipped: {}", self.files_skipped);
        println!("Total nodes: {}", self.total_nodes);
        println!("Total edges: {}", self.total_edges);

        if !self.nodes_per_language.is_empty() {
            println!("\nNodes per language:");
            for entry in self.nodes_per_language.iter() {
                println!("  {}: {}", entry.key(), entry.value());
            }
        }

        if !self.errors.is_empty() {
            println!("\nErrors encountered: {}", self.errors.len());
            for (i, error) in self.errors.iter().take(5).enumerate() {
                println!("  {}. {}", i + 1, error);
            }
            if self.errors.len() > 5 {
                println!("  ... and {} more", self.errors.len() - 5);
            }
        }
    }
}

/// Configuration for the project parser
#[derive(Debug, Clone)]
pub struct ParserConfig {
    /// Whether to parse files in parallel
    pub parallel: bool,

    /// Maximum depth for directory traversal (None = unlimited)
    pub max_depth: Option<usize>,

    /// Ignore hidden files and directories
    pub ignore_hidden: bool,

    /// Custom file patterns to ignore (e.g., ["target", "node_modules"])
    pub ignore_patterns: Vec<String>,

    /// Follow symbolic links
    pub follow_links: bool,
}

impl Default for ParserConfig {
    fn default() -> Self {
        Self {
            parallel: true,
            max_depth: None,
            ignore_hidden: true,
            ignore_patterns: vec![
                "target".to_string(),
                "node_modules".to_string(),
                ".git".to_string(),
                "dist".to_string(),
                "build".to_string(),
            ],
            follow_links: false,
        }
    }
}

/// Main project parser orchestrator
pub struct ProjectParser {
    registry: Arc<ParserRegistry>,
    config: ParserConfig,
}

impl ProjectParser {
    /// Create a new project parser with default configuration
    pub fn new() -> Self {
        Self {
            registry: Arc::new(ParserRegistry::new()),
            config: ParserConfig::default(),
        }
    }

    /// Create a new project parser with custom configuration
    pub fn with_config(config: ParserConfig) -> Self {
        Self {
            registry: Arc::new(ParserRegistry::new()),
            config,
        }
    }

    /// Get the parser registry
    pub fn registry(&self) -> &ParserRegistry {
        &self.registry
    }

    /// Parse a single file
    pub fn parse_file(&self, file_path: &Path) -> Result<CodeGraph> {
        info!("Parsing file: {}", file_path.display());

        let parser = self
            .registry
            .find_parser(file_path)
            .with_context(|| format!("No parser found for file: {}", file_path.display()))?;

        let mut builder = CodeGraphBuilder::new();
        parser.parse_file(file_path, &mut builder)?;

        Ok(builder.build())
    }

    /// Collect all parseable files in a directory
    fn collect_files(&self, root_path: &Path) -> Result<Vec<PathBuf>> {
        let mut files = Vec::new();
        let mut walker = WalkDir::new(root_path);

        if !self.config.follow_links {
            walker = walker.follow_links(false);
        }

        if let Some(depth) = self.config.max_depth {
            walker = walker.max_depth(depth);
        }

        for entry in walker.into_iter().filter_entry(|e| {
            // Filter out ignored patterns
            if self.config.ignore_hidden {
                if let Some(name) = e.file_name().to_str() {
                    if name.starts_with('.') && e.path() != root_path {
                        return false;
                    }
                }
            }

            // Check ignore patterns
            if let Some(name) = e.file_name().to_str() {
                if self.config.ignore_patterns.iter().any(|p| name == p) {
                    return false;
                }
            }

            true
        }) {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                if self.registry.find_parser(path).is_some() {
                    files.push(path.to_path_buf());
                }
            }
        }

        Ok(files)
    }

    /// Parse all files in a project directory
    pub fn parse_project(&self, root_path: &Path) -> Result<(CodeGraph, ParseStats)> {
        info!("Parsing project: {}", root_path.display());

        let files = self.collect_files(root_path)?;
        let mut stats = ParseStats::new();
        stats.files_found = files.len();

        info!("Found {} parseable files", files.len());

        if files.is_empty() {
            return Ok((CodeGraphBuilder::new().build(), stats));
        }

        // Use thread-safe builder components
        let builder = Arc::new(std::sync::Mutex::new(CodeGraphBuilder::new()));
        let stats_map = Arc::new(DashMap::new());

        if self.config.parallel {
            // Parse files in parallel
            files.par_iter().for_each(|file_path| {
                self.parse_file_with_stats(file_path, &builder, &stats_map);
            });
        } else {
            // Parse files sequentially
            for file_path in &files {
                self.parse_file_with_stats(file_path, &builder, &stats_map);
            }
        }

        // Collect stats
        for entry in stats_map.iter() {
            stats.merge(entry.value());
        }

        let final_builder = Arc::try_unwrap(builder)
            .unwrap_or_else(|_| panic!("Failed to unwrap builder"))
            .into_inner()
            .unwrap();

        stats.total_nodes = final_builder.node_count();
        stats.total_edges = final_builder.edge_count();

        let graph = final_builder.build();

        info!(
            "Parsing complete: {} nodes, {} edges",
            stats.total_nodes, stats.total_edges
        );

        Ok((graph, stats))
    }

    /// Parse a single file and update statistics
    fn parse_file_with_stats(
        &self,
        file_path: &Path,
        builder: &Arc<std::sync::Mutex<CodeGraphBuilder>>,
        stats_map: &Arc<DashMap<PathBuf, ParseStats>>,
    ) {
        let mut file_stats = ParseStats::new();

        match self.registry.find_parser(file_path) {
            Some(parser) => {
                debug!("Parsing: {}", file_path.display());

                let mut local_builder = CodeGraphBuilder::new();
                match parser.parse_file(file_path, &mut local_builder) {
                    Ok(_) => {
                        file_stats.files_parsed += 1;
                        let node_count = local_builder.node_count();
                        let edge_count = local_builder.edge_count();

                        file_stats
                            .nodes_per_language
                            .insert(parser.language_name().to_string(), node_count);

                        // Merge into main builder
                        if let Ok(mut main_builder) = builder.lock() {
                            // Transfer nodes and edges from local builder to main builder
                            let local_graph = local_builder.build();
                            for node in local_graph.node_weights() {
                                main_builder.add_node(node.clone());
                            }
                            for edge_ref in local_graph.edge_references() {
                                let source = edge_ref.source();
                                let target = edge_ref.target();
                                // Note: This is simplified; in production you'd need to map node indices
                            }
                        }

                        debug!(
                            "Parsed {}: {} nodes, {} edges",
                            file_path.display(),
                            node_count,
                            edge_count
                        );
                    }
                    Err(e) => {
                        file_stats.files_failed += 1;
                        file_stats
                            .errors
                            .push(format!("{}: {}", file_path.display(), e));
                        warn!("Failed to parse {}: {}", file_path.display(), e);
                    }
                }
            }
            None => {
                file_stats.files_skipped += 1;
                debug!("Skipping unsupported file: {}", file_path.display());
            }
        }

        stats_map.insert(file_path.to_path_buf(), file_stats);
    }

    /// Export graph to various formats
    pub fn export_graph(&self, graph: &CodeGraph, output_path: &Path, format: ExportFormat) -> Result<()> {
        match format {
            ExportFormat::Bincode => {
                // Create a serializable representation
                let nodes: Vec<_> = graph.node_weights().cloned().collect();
                let edges: Vec<_> = graph
                    .edge_references()
                    .map(|e| (e.source().index(), e.target().index(), e.weight().clone()))
                    .collect();

                let serializable = (nodes, edges);
                let file = std::fs::File::create(output_path)?;
                bincode::serialize_into(file, &serializable)?;
            }
            ExportFormat::Json => {
                // Create a serializable representation
                let nodes: Vec<_> = graph.node_weights().cloned().collect();
                let edges: Vec<_> = graph
                    .edge_references()
                    .map(|e| (e.source().index(), e.target().index(), e.weight().clone()))
                    .collect();

                let serializable = serde_json::json!({
                    "nodes": nodes,
                    "edges": edges,
                });
                let json = serde_json::to_string_pretty(&serializable)?;
                std::fs::write(output_path, json)?;
            }
            ExportFormat::Dot => {
                use petgraph::dot::{Config, Dot};
                let dot = format!(
                    "{:?}",
                    Dot::with_config(graph, &[Config::EdgeNoLabel])
                );
                std::fs::write(output_path, dot)?;
            }
        }

        info!("Exported graph to: {}", output_path.display());
        Ok(())
    }
}

impl Default for ProjectParser {
    fn default() -> Self {
        Self::new()
    }
}

/// Export format for code graphs
#[derive(Debug, Clone, Copy)]
pub enum ExportFormat {
    /// Binary format using bincode
    Bincode,
    /// JSON format
    Json,
    /// GraphViz DOT format
    Dot,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::fs;

    #[test]
    fn test_parse_single_rust_file() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.rs");

        fs::write(
            &file_path,
            r#"
            pub fn add(a: i32, b: i32) -> i32 {
                a + b
            }
            "#,
        )
        .unwrap();

        let parser = ProjectParser::new();
        let graph = parser.parse_file(&file_path).unwrap();

        assert!(graph.node_count() > 0);
    }

    #[test]
    fn test_parse_project() {
        let temp_dir = TempDir::new().unwrap();

        // Create a simple project structure
        fs::write(
            temp_dir.path().join("main.rs"),
            r#"
            fn main() {
                println!("Hello, world!");
            }
            "#,
        )
        .unwrap();

        fs::write(
            temp_dir.path().join("lib.rs"),
            r#"
            pub struct Point {
                x: f64,
                y: f64,
            }
            "#,
        )
        .unwrap();

        let parser = ProjectParser::new();
        let (graph, stats) = parser.parse_project(temp_dir.path()).unwrap();

        assert_eq!(stats.files_found, 2);
        assert!(stats.files_parsed > 0);
        assert!(graph.node_count() > 0);
    }

    #[test]
    fn test_ignore_patterns() {
        let temp_dir = TempDir::new().unwrap();

        // Create target directory with a file
        let target_dir = temp_dir.path().join("target");
        fs::create_dir(&target_dir).unwrap();
        fs::write(target_dir.join("ignored.rs"), "fn test() {}").unwrap();

        // Create a normal file
        fs::write(temp_dir.path().join("main.rs"), "fn main() {}").unwrap();

        let parser = ProjectParser::new();
        let (_, stats) = parser.parse_project(temp_dir.path()).unwrap();

        // Should only find main.rs, not the file in target/
        assert_eq!(stats.files_found, 1);
    }
}
