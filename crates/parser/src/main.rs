//! GNN Parser CLI
//!
//! Command-line interface for parsing code projects and generating code graphs.

use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};
use gnn_parser::{ExportFormat, ParserConfig, ProjectParser};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "gnn-parser")]
#[command(about = "Multi-language code parser for GNN Code Intelligence System", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Enable verbose logging
    #[arg(short, long, global = true)]
    verbose: bool,

    /// Disable parallel processing
    #[arg(long, global = true)]
    no_parallel: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Parse a single file
    File {
        /// Path to the file to parse
        #[arg(value_name = "FILE")]
        path: PathBuf,

        /// Output file for the graph
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Output format
        #[arg(short, long, value_enum, default_value = "json")]
        format: Format,
    },

    /// Parse an entire project directory
    Project {
        /// Path to the project directory
        #[arg(value_name = "DIR")]
        path: PathBuf,

        /// Output file for the graph
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Output format
        #[arg(short, long, value_enum, default_value = "bincode")]
        format: Format,

        /// Maximum depth for directory traversal
        #[arg(long)]
        max_depth: Option<usize>,

        /// Don't ignore hidden files
        #[arg(long)]
        include_hidden: bool,

        /// Additional patterns to ignore
        #[arg(long, value_name = "PATTERN")]
        ignore: Vec<String>,

        /// Follow symbolic links
        #[arg(long)]
        follow_links: bool,
    },

    /// List supported languages and file extensions
    Languages,

    /// Display statistics about a parsed graph
    Stats {
        /// Path to the graph file
        #[arg(value_name = "FILE")]
        path: PathBuf,
    },
}

#[derive(Clone, Copy, ValueEnum)]
enum Format {
    /// JSON format
    Json,
    /// Binary format (bincode)
    Bincode,
    /// GraphViz DOT format
    Dot,
}

impl From<Format> for ExportFormat {
    fn from(format: Format) -> Self {
        match format {
            Format::Json => ExportFormat::Json,
            Format::Bincode => ExportFormat::Bincode,
            Format::Dot => ExportFormat::Dot,
        }
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize logger
    let log_level = if cli.verbose {
        "debug"
    } else {
        "info"
    };
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(log_level))
        .init();

    match cli.command {
        Commands::File { path, output, format } => {
            handle_file_command(path, output, format, cli.no_parallel)?;
        }
        Commands::Project {
            path,
            output,
            format,
            max_depth,
            include_hidden,
            ignore,
            follow_links,
        } => {
            handle_project_command(
                path,
                output,
                format,
                max_depth,
                include_hidden,
                ignore,
                follow_links,
                cli.no_parallel,
            )?;
        }
        Commands::Languages => {
            handle_languages_command();
        }
        Commands::Stats { path } => {
            handle_stats_command(path)?;
        }
    }

    Ok(())
}

fn handle_file_command(
    path: PathBuf,
    output: Option<PathBuf>,
    format: Format,
    no_parallel: bool,
) -> Result<()> {
    println!("Parsing file: {}", path.display());

    let config = ParserConfig {
        parallel: !no_parallel,
        ..Default::default()
    };

    let parser = ProjectParser::with_config(config);
    let graph = parser.parse_file(&path)?;

    println!("\nParse Results:");
    println!("  Nodes: {}", graph.node_count());
    println!("  Edges: {}", graph.edge_count());

    // Print some sample nodes
    println!("\nSample nodes:");
    for (i, node) in graph.node_weights().take(10).enumerate() {
        println!("  {}. {:?} '{}'", i + 1, node.kind, node.name);
    }

    if graph.node_count() > 10 {
        println!("  ... and {} more nodes", graph.node_count() - 10);
    }

    // Export if output is specified
    if let Some(output_path) = output {
        parser.export_graph(&graph, &output_path, format.into())?;
        println!("\nGraph exported to: {}", output_path.display());
    }

    Ok(())
}

fn handle_project_command(
    path: PathBuf,
    output: Option<PathBuf>,
    format: Format,
    max_depth: Option<usize>,
    include_hidden: bool,
    ignore: Vec<String>,
    follow_links: bool,
    no_parallel: bool,
) -> Result<()> {
    println!("Parsing project: {}", path.display());

    let mut config = ParserConfig {
        parallel: !no_parallel,
        max_depth,
        ignore_hidden: !include_hidden,
        follow_links,
        ..Default::default()
    };

    // Add custom ignore patterns
    config.ignore_patterns.extend(ignore);

    let parser = ProjectParser::with_config(config);
    let (graph, stats) = parser.parse_project(&path)?;

    stats.print_summary();

    // Print sample nodes grouped by kind
    println!("\n=== Sample Nodes by Type ===");
    let mut nodes_by_kind: std::collections::HashMap<_, Vec<_>> = std::collections::HashMap::new();
    for node in graph.node_weights() {
        nodes_by_kind
            .entry(node.kind)
            .or_default()
            .push(node);
    }

    for (kind, nodes) in nodes_by_kind.iter().take(5) {
        println!("\n{:?} ({} total):", kind, nodes.len());
        for node in nodes.iter().take(3) {
            println!("  - {} ({}:{})", node.name, node.file_path.display(), node.start_position.0);
        }
        if nodes.len() > 3 {
            println!("  ... and {} more", nodes.len() - 3);
        }
    }

    // Export if output is specified
    if let Some(output_path) = output {
        parser.export_graph(&graph, &output_path, format.into())?;
        println!("\nGraph exported to: {}", output_path.display());
    }

    Ok(())
}

fn handle_languages_command() {
    let parser = ProjectParser::new();
    let registry = parser.registry();

    println!("Supported Languages:");
    println!("===================\n");

    for language_parser in registry.parsers() {
        println!("Language: {}", language_parser.language_name());
        println!("Extensions: {}", language_parser.file_extensions().join(", "));
        println!();
    }

    println!("Total languages supported: {}", registry.parsers().len());
}

fn handle_stats_command(path: PathBuf) -> Result<()> {
    use gnn_parser::graph::{CodeNode, CodeEdge};
    use petgraph::graph::DiGraph;

    println!("Loading graph from: {}", path.display());

    // Determine format from extension
    let extension = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("");

    let (nodes, edges): (Vec<CodeNode>, Vec<(usize, usize, CodeEdge)>) = match extension {
        "json" => {
            let json_str = std::fs::read_to_string(&path)?;
            let json: serde_json::Value = serde_json::from_str(&json_str)?;
            let nodes: Vec<CodeNode> = serde_json::from_value(json["nodes"].clone())?;
            let edges: Vec<(usize, usize, CodeEdge)> = serde_json::from_value(json["edges"].clone())?;
            (nodes, edges)
        }
        "bin" | "bincode" => {
            let file = std::fs::File::open(&path)?;
            bincode::deserialize_from(file)?
        }
        _ => {
            anyhow::bail!("Unsupported file format: {}", extension);
        }
    };

    // Reconstruct the graph
    let mut graph = DiGraph::new();
    let node_indices: Vec<_> = nodes.iter().map(|n| graph.add_node(n.clone())).collect();
    for (from, to, edge) in edges {
        if from < node_indices.len() && to < node_indices.len() {
            graph.add_edge(node_indices[from], node_indices[to], edge);
        }
    }

    println!("\n=== Graph Statistics ===");
    println!("Total nodes: {}", graph.node_count());
    println!("Total edges: {}", graph.edge_count());

    // Count nodes by kind
    let mut nodes_by_kind: std::collections::HashMap<_, usize> = std::collections::HashMap::new();
    for node in graph.node_weights() {
        *nodes_by_kind.entry(node.kind).or_default() += 1;
    }

    println!("\nNodes by kind:");
    let mut sorted_kinds: Vec<_> = nodes_by_kind.iter().collect();
    sorted_kinds.sort_by(|a, b| b.1.cmp(a.1));
    for (kind, count) in sorted_kinds {
        println!("  {:?}: {}", kind, count);
    }

    // Count edges by kind
    let mut edges_by_kind: std::collections::HashMap<_, usize> = std::collections::HashMap::new();
    for edge in graph.edge_weights() {
        *edges_by_kind.entry(edge.kind).or_default() += 1;
    }

    println!("\nEdges by kind:");
    let mut sorted_edge_kinds: Vec<_> = edges_by_kind.iter().collect();
    sorted_edge_kinds.sort_by(|a, b| b.1.cmp(a.1));
    for (kind, count) in sorted_edge_kinds {
        println!("  {:?}: {}", kind, count);
    }

    // Count nodes by language
    let mut nodes_by_language: std::collections::HashMap<_, usize> = std::collections::HashMap::new();
    for node in graph.node_weights() {
        *nodes_by_language.entry(&node.language).or_default() += 1;
    }

    println!("\nNodes by language:");
    for (lang, count) in nodes_by_language {
        println!("  {}: {}", lang, count);
    }

    // Count files
    let mut files: std::collections::HashSet<_> = std::collections::HashSet::new();
    for node in graph.node_weights() {
        files.insert(&node.file_path);
    }

    println!("\nFiles processed: {}", files.len());

    Ok(())
}
