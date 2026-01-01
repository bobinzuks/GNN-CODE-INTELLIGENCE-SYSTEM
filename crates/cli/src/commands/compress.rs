//! Compress command - Compress codebase to semantic embedding

use anyhow::{Context, Result};
use colored::Colorize;
use console::style;
use std::path::PathBuf;
use tracing::info;

use gnn_core::{
    compression::{SemanticCompressor, FeatureConfig},
    model::{GNNConfig, GNNModel},
};
use gnn_parser::ProjectParser;
use rand::SeedableRng;
use rand::rngs::StdRng;

/// Run the compress command
pub fn run(
    path: PathBuf,
    output: PathBuf,
    model: Option<PathBuf>,
    metadata: bool,
    format: String,
) -> Result<()> {
    println!("{}", style("Compressing codebase to semantic embedding...").bold().cyan());
    println!();

    // Validate input
    if !path.exists() {
        anyhow::bail!("Path does not exist: {}", path.display());
    }

    // Create output directory
    if let Some(parent) = output.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create output directory: {}", parent.display()))?;
    }

    // Display configuration
    println!("Configuration:");
    println!("  Path: {}", style(path.display()).green());
    println!("  Output: {}", style(output.display()).green());
    if let Some(ref model_path) = model {
        println!("  Model: {}", style(model_path.display()).green());
    } else {
        println!("  Model: {}", style("default").yellow());
    }
    println!("  Metadata: {}", style(metadata).green());
    println!("  Format: {}", style(&format).green());
    println!();

    // Parse the codebase
    println!("{}", style("Parsing codebase...").bold());

    let parser = ProjectParser::new();
    let (graph, stats) = if path.is_dir() {
        parser.parse_project(&path)?
    } else {
        let graph = parser.parse_file(&path)?;
        let mut stats = gnn_parser::ParseStats::new();
        stats.files_parsed = 1;
        stats.total_nodes = graph.node_count();
        stats.total_edges = graph.edge_count();
        (graph, stats)
    };

    println!("  Files: {}", style(stats.files_parsed).green());
    println!("  Nodes: {}", style(stats.total_nodes).green());
    println!("  Edges: {}", style(stats.total_edges).green());
    println!();

    // Load or create model
    println!("{}", style("Loading model...").bold());

    let gnn_model = if let Some(model_path) = model {
        load_model(&model_path)?
    } else {
        // Create default model
        let mut rng = StdRng::seed_from_u64(42);
        let config = GNNConfig {
            input_dim: 128,
            hidden_dims: vec![256, 256],
            output_dim: 512,
            num_heads: 4,
            dropout: 0.1,
            use_attention_pooling: true,
            aggregation: gnn_core::layers::AggregationType::Mean,
        };
        GNNModel::new_sage(config, &mut rng)
    };

    println!("  Output dimension: {}", style(gnn_model.output_dim()).green());
    println!();

    // Create compressor
    let feature_config = FeatureConfig::default();
    let compressor = SemanticCompressor::new(gnn_model, feature_config, 512);

    // Convert graph to compression format
    println!("{}", style("Compressing...").bold());
    let compression_graph = convert_to_compression_graph(&graph);

    // Compress
    let embedding = compressor.compress(&compression_graph)?;

    println!("  Embedding size: {} dimensions", style(embedding.vector.len()).green());
    println!("  Memory size: {} bytes", style(embedding.vector.len() * 4).green());
    println!();

    // Save embedding
    println!("{}", style("Saving embedding...").bold());

    match format.to_lowercase().as_str() {
        "bincode" => save_bincode(&embedding, &output, metadata)?,
        "json" => save_json(&embedding, &output, metadata)?,
        _ => anyhow::bail!("Invalid format: {}", format),
    }

    println!("{} Embedding saved to: {}", "✓".green(), style(output.display()).cyan());
    println!();

    // Display summary
    println!("{}", style("Compression Summary:").bold().green());
    println!("{}", "=".repeat(50));
    println!("Original:");
    println!("  Files: {}", style(stats.files_parsed).green());
    println!("  Nodes: {}", style(stats.total_nodes).green());
    println!("  Edges: {}", style(stats.total_edges).green());
    println!();
    println!("Compressed:");
    println!("  Dimensions: {}", style(embedding.vector.len()).green());
    println!("  Size: {} KB", style((embedding.vector.len() * 4) / 1024).green());
    println!();

    // Calculate compression ratio
    let original_size_estimate = stats.total_nodes * 100; // Rough estimate
    let compressed_size = embedding.vector.len() * 4;
    let ratio = original_size_estimate as f64 / compressed_size as f64;
    println!("Compression ratio: {}x", style(format!("{:.1}", ratio)).green().bold());

    Ok(())
}

/// Load model from file
fn load_model(path: &PathBuf) -> Result<GNNModel> {
    // TODO: Implement actual model loading
    // For now, return a default model
    let mut rng = StdRng::seed_from_u64(42);
    let config = GNNConfig {
        input_dim: 128,
        hidden_dims: vec![256, 256],
        output_dim: 512,
        num_heads: 4,
        dropout: 0.1,
        use_attention_pooling: true,
        aggregation: gnn_core::layers::AggregationType::Mean,
    };
    Ok(GNNModel::new_sage(config, &mut rng))
}

/// Convert parser graph to compression graph
fn convert_to_compression_graph(
    graph: &gnn_parser::graph::CodeGraph,
) -> gnn_core::compression::CodeGraph {
    use petgraph::graph::DiGraph;

    let mut compression_graph = DiGraph::new();

    // Map node indices
    let mut node_map = std::collections::HashMap::new();

    // Add nodes
    for (idx, node) in graph.node_references() {
        let compression_node = gnn_core::compression::CodeNode {
            element_type: convert_to_element_type(&node.kind),
            name: node.name.clone(),
            file_path: node.file_path.clone().unwrap_or_default(),
            start_line: node.start_line as usize,
            end_line: node.end_line as usize,
            complexity: 1,
            dependencies: 0,
        };

        let new_idx = compression_graph.add_node(compression_node);
        node_map.insert(idx, new_idx);
    }

    // Add edges
    for edge in graph.edge_references() {
        if let (Some(&from), Some(&to)) = (node_map.get(&edge.source()), node_map.get(&edge.target())) {
            let compression_edge = gnn_core::compression::CodeEdge {
                weight: edge.weight().weight,
            };
            compression_graph.add_edge(from, to, compression_edge);
        }
    }

    compression_graph
}

/// Convert node kind to element type
fn convert_to_element_type(kind: &gnn_parser::graph::NodeKind) -> gnn_core::compression::CodeElementType {
    use gnn_parser::graph::NodeKind as NK;
    use gnn_core::compression::CodeElementType as ET;

    match kind {
        NK::File => ET::File,
        NK::Module => ET::Module,
        NK::Function | NK::Method => ET::Function,
        NK::Class | NK::Struct => ET::Struct,
        NK::Enum => ET::Enum,
        NK::Interface | NK::Trait => ET::Trait,
        NK::Variable | NK::Constant | NK::Parameter | NK::Field => ET::Variable,
        _ => ET::Other,
    }
}

/// Save embedding in bincode format
fn save_bincode(
    embedding: &gnn_core::compression::ProjectEmbedding,
    path: &PathBuf,
    include_metadata: bool,
) -> Result<()> {
    let data = if include_metadata {
        bincode::serialize(embedding)?
    } else {
        bincode::serialize(&embedding.vector)?
    };

    std::fs::write(path, data)
        .with_context(|| format!("Failed to write embedding: {}", path.display()))?;

    Ok(())
}

/// Save embedding in JSON format
fn save_json(
    embedding: &gnn_core::compression::ProjectEmbedding,
    path: &PathBuf,
    include_metadata: bool,
) -> Result<()> {
    let json = if include_metadata {
        serde_json::to_string_pretty(embedding)?
    } else {
        serde_json::to_string_pretty(&embedding.vector)?
    };

    std::fs::write(path, json)
        .with_context(|| format!("Failed to write embedding: {}", path.display()))?;

    Ok(())
}
