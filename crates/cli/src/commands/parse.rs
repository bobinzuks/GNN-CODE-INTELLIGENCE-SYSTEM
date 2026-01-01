//! Parse command - Convert code to graph representation

use anyhow::{Context, Result};
use colored::Colorize;
use console::style;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::path::PathBuf;
use std::sync::Arc;
use tracing::{info, warn};

use gnn_parser::{ExportFormat, ParserConfig, ProjectParser};

/// Run the parse command
pub fn run(
    input: PathBuf,
    output: PathBuf,
    parallel: usize,
    export_format: String,
    max_depth: Option<usize>,
    ignore_hidden: bool,
) -> Result<()> {
    println!("{}", style("Parsing code to graph representation...").bold().cyan());
    println!();

    // Validate input
    if !input.exists() {
        anyhow::bail!("Input path does not exist: {}", input.display());
    }

    // Create output directory
    std::fs::create_dir_all(&output)
        .with_context(|| format!("Failed to create output directory: {}", output.display()))?;

    // Parse export format
    let format = match export_format.to_lowercase().as_str() {
        "bincode" => ExportFormat::Bincode,
        "json" => ExportFormat::Json,
        "dot" => ExportFormat::Dot,
        _ => anyhow::bail!("Invalid export format: {}", export_format),
    };

    // Display configuration
    println!("Configuration:");
    println!("  Input: {}", style(input.display()).green());
    println!("  Output: {}", style(output.display()).green());
    println!("  Parallel workers: {}", style(parallel).green());
    println!("  Export format: {}", style(&export_format).green());
    if let Some(depth) = max_depth {
        println!("  Max depth: {}", style(depth).green());
    }
    println!("  Ignore hidden: {}", style(ignore_hidden).green());
    println!();

    // Configure parser
    let mut config = ParserConfig::default();
    config.parallel = parallel > 1;
    config.max_depth = max_depth;
    config.ignore_hidden = ignore_hidden;

    let parser = ProjectParser::with_config(config);

    // Check if input is a directory or file
    if input.is_dir() {
        parse_directory(&parser, &input, &output, format)?;
    } else if input.extension().and_then(|s| s.to_str()) == Some("map") {
        parse_map_file(&parser, &input, &output, format)?;
    } else {
        parse_single_file(&parser, &input, &output, format)?;
    }

    Ok(())
}

/// Parse a single directory
fn parse_directory(
    parser: &ProjectParser,
    input: &PathBuf,
    output: &PathBuf,
    format: ExportFormat,
) -> Result<()> {
    println!("{}", style("Parsing project directory...").bold());

    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .expect("Invalid spinner template"),
    );
    spinner.set_message("Scanning files...");

    // Parse the project
    let (graph, stats) = parser
        .parse_project(input)
        .context("Failed to parse project")?;

    spinner.finish_and_clear();

    // Display statistics
    display_parse_stats(&stats);

    // Export graph
    let output_file = output.join("graph").with_extension(match format {
        ExportFormat::Bincode => "bin",
        ExportFormat::Json => "json",
        ExportFormat::Dot => "dot",
    });

    println!();
    println!("{}", style("Exporting graph...").bold());

    parser.export_graph(&graph, &output_file, format)
        .context("Failed to export graph")?;

    println!("{} Graph exported to: {}", "✓".green(), style(output_file.display()).cyan());

    Ok(())
}

/// Parse repositories from a .map file
fn parse_map_file(
    parser: &ProjectParser,
    input: &PathBuf,
    output: &PathBuf,
    format: ExportFormat,
) -> Result<()> {
    println!("{}", style("Parsing repositories from map file...").bold());
    println!();

    // Read map file
    let map_content = std::fs::read_to_string(input)
        .with_context(|| format!("Failed to read map file: {}", input.display()))?;

    let repos: Vec<String> = map_content
        .lines()
        .skip(1) // Skip header
        .filter(|line| !line.trim().is_empty())
        .filter_map(|line| {
            // Parse CSV line: tier,url,language,stars,commits,pushed_at,score
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() >= 2 {
                Some(parts[1].to_string())
            } else {
                None
            }
        })
        .collect();

    println!("Found {} repositories in map file", style(repos.len()).green());

    if repos.is_empty() {
        warn!("No repositories found in map file");
        return Ok(());
    }

    // Setup progress tracking
    let multi_progress = MultiProgress::new();
    let overall_progress = multi_progress.add(ProgressBar::new(repos.len() as u64));
    overall_progress.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}")
            .expect("Invalid progress bar template")
            .progress_chars("#>-"),
    );

    // Parse each repository
    for (idx, repo_url) in repos.iter().enumerate() {
        overall_progress.set_position(idx as u64);
        overall_progress.set_message(format!("Parsing {}", repo_url));

        // TODO: Clone and parse repository
        // For now, just skip
        warn!("Skipping repository parsing (not implemented): {}", repo_url);
    }

    overall_progress.finish_with_message("Parsing complete!");

    Ok(())
}

/// Parse a single file
fn parse_single_file(
    parser: &ProjectParser,
    input: &PathBuf,
    output: &PathBuf,
    format: ExportFormat,
) -> Result<()> {
    println!("{}", style("Parsing single file...").bold());

    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .expect("Invalid spinner template"),
    );
    spinner.set_message(format!("Parsing {}", input.display()));

    // Parse the file
    let graph = parser
        .parse_file(input)
        .context("Failed to parse file")?;

    spinner.finish_and_clear();

    // Display statistics
    println!();
    println!("Parse complete:");
    println!("  Nodes: {}", style(graph.node_count()).green());
    println!("  Edges: {}", style(graph.edge_count()).green());

    // Export graph
    let output_file = output.join(
        input
            .file_stem()
            .unwrap_or_else(|| std::ffi::OsStr::new("graph"))
    ).with_extension(match format {
        ExportFormat::Bincode => "bin",
        ExportFormat::Json => "json",
        ExportFormat::Dot => "dot",
    });

    println!();
    println!("{}", style("Exporting graph...").bold());

    parser.export_graph(&graph, &output_file, format)
        .context("Failed to export graph")?;

    println!("{} Graph exported to: {}", "✓".green(), style(output_file.display()).cyan());

    Ok(())
}

/// Display parsing statistics
fn display_parse_stats(stats: &gnn_parser::ParseStats) {
    println!();
    println!("{}", style("Parse Statistics:").bold().green());
    println!("{}", "=".repeat(50));
    println!("Files found:   {}", style(stats.files_found).green());
    println!("Files parsed:  {}", style(stats.files_parsed).green());
    println!("Files failed:  {}", if stats.files_failed > 0 {
        style(stats.files_failed).red()
    } else {
        style(stats.files_failed).green()
    });
    println!("Files skipped: {}", style(stats.files_skipped).yellow());
    println!();
    println!("Total nodes:   {}", style(stats.total_nodes).cyan());
    println!("Total edges:   {}", style(stats.total_edges).cyan());

    if !stats.nodes_per_language.is_empty() {
        println!();
        println!("Nodes per language:");
        for entry in stats.nodes_per_language.iter() {
            println!("  {}: {}", entry.key(), style(entry.value()).cyan());
        }
    }

    if stats.files_failed > 0 && !stats.errors.is_empty() {
        println!();
        println!("{}", style("Errors:").yellow());
        for (i, error) in stats.errors.iter().take(5).enumerate() {
            println!("  {}. {}", i + 1, error);
        }
        if stats.errors.len() > 5 {
            println!("  ... and {} more", stats.errors.len() - 5);
        }
    }
}
