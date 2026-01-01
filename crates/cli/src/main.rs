//! GNN Code Intelligence CLI
//!
//! A comprehensive command-line interface for the GNN Code Intelligence System.
//! Orchestrates the entire pipeline: sweep, parse, train, check, compress, and generate.

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use tracing::{info, Level};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

mod commands;
mod config;

use commands::*;
use config::Config;

/// GNN Code Intelligence - Make any LLM output flawless code at scale
#[derive(Parser)]
#[command(name = "gnn-intel")]
#[command(author = "GNN Code Intelligence Team")]
#[command(version)]
#[command(about = "GNN-powered code intelligence system", long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    /// Enable verbose logging
    #[arg(short, long, global = true)]
    verbose: bool,

    /// Enable debug logging
    #[arg(short, long, global = true)]
    debug: bool,

    /// Output format (text, json)
    #[arg(long, global = true, default_value = "text")]
    format: OutputFormat,

    /// Configuration file path
    #[arg(short, long, global = true)]
    config: Option<PathBuf>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Sweep GitHub for quality repositories
    Sweep {
        /// Programming language to sweep (e.g., rust, python, go)
        #[arg(short, long)]
        language: Option<String>,

        /// Minimum number of stars
        #[arg(long, default_value = "1")]
        min_stars: u32,

        /// Minimum number of commits
        #[arg(long, default_value = "50")]
        min_commits: u32,

        /// Maximum number of results
        #[arg(long, default_value = "1000")]
        max_results: usize,

        /// Output file path
        #[arg(short, long, default_value = "output/repos.map")]
        output: PathBuf,

        /// GitHub API token (or use GITHUB_TOKEN env var)
        #[arg(long, env = "GITHUB_TOKEN")]
        token: Option<String>,

        /// Use cached results
        #[arg(long)]
        use_cache: bool,

        /// Rate limit (requests per second)
        #[arg(long, default_value = "10")]
        rate_limit: usize,
    },

    /// Parse repositories into graph format
    Parse {
        /// Input path (directory or .map file)
        #[arg(short, long)]
        input: PathBuf,

        /// Output directory for parsed graphs
        #[arg(short, long, default_value = "output/graphs")]
        output: PathBuf,

        /// Number of parallel workers
        #[arg(long, default_value = "8")]
        parallel: usize,

        /// Export format (bincode, json, dot)
        #[arg(long, default_value = "bincode")]
        export_format: String,

        /// Maximum depth for directory traversal
        #[arg(long)]
        max_depth: Option<usize>,

        /// Skip hidden files
        #[arg(long, default_value = "true")]
        ignore_hidden: bool,
    },

    /// Train GNN models on parsed graphs
    Train {
        /// Directory containing parsed graphs
        #[arg(short, long)]
        graphs: PathBuf,

        /// Output directory for trained models
        #[arg(short, long, default_value = "models")]
        output: PathBuf,

        /// Number of training epochs
        #[arg(long, default_value = "100")]
        epochs: u32,

        /// Batch size
        #[arg(long, default_value = "32")]
        batch_size: usize,

        /// Learning rate
        #[arg(long, default_value = "0.001")]
        learning_rate: f32,

        /// Model architecture (sage, gat)
        #[arg(long, default_value = "sage")]
        architecture: String,

        /// Hidden dimensions (comma-separated)
        #[arg(long, default_value = "256,256")]
        hidden_dims: String,

        /// Output embedding dimension
        #[arg(long, default_value = "512")]
        output_dim: usize,

        /// Train specific expert (rust, python, go, etc.)
        #[arg(long)]
        expert: Option<String>,

        /// Resume from checkpoint
        #[arg(long)]
        resume: Option<PathBuf>,
    },

    /// Check code for issues and quality
    Check {
        /// Path to code directory or file
        #[arg(short, long)]
        path: PathBuf,

        /// Programming language (auto-detected if not specified)
        #[arg(long)]
        language: Option<String>,

        /// Minimum severity level (info, warning, error, critical)
        #[arg(long, default_value = "info")]
        severity: String,

        /// Output as JSON
        #[arg(long)]
        json: bool,

        /// Model path (defaults to built-in models)
        #[arg(long)]
        model: Option<PathBuf>,

        /// Show suggestions
        #[arg(long)]
        suggestions: bool,

        /// Auto-fix issues where possible
        #[arg(long)]
        fix: bool,
    },

    /// Compress codebase to semantic embedding
    Compress {
        /// Path to codebase
        #[arg(short, long)]
        path: PathBuf,

        /// Output file for embedding
        #[arg(short, long)]
        output: PathBuf,

        /// Model path (defaults to built-in models)
        #[arg(long)]
        model: Option<PathBuf>,

        /// Include metadata
        #[arg(long)]
        metadata: bool,

        /// Compression format (bincode, json)
        #[arg(long, default_value = "bincode")]
        format: String,
    },

    /// Generate code with LLM + GNN guidance
    Generate {
        /// Code generation prompt
        #[arg(short, long)]
        prompt: String,

        /// Codebase to use as context
        #[arg(long)]
        context: Option<PathBuf>,

        /// LLM model to use
        #[arg(long, default_value = "codellama")]
        model: String,

        /// LLM endpoint (defaults to Ollama)
        #[arg(long, default_value = "http://localhost:11434")]
        endpoint: String,

        /// Output file (prints to stdout if not specified)
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Programming language
        #[arg(long)]
        language: Option<String>,

        /// Temperature for generation
        #[arg(long, default_value = "0.7")]
        temperature: f32,

        /// Apply GNN post-processing
        #[arg(long, default_value = "true")]
        gnn_fix: bool,
    },

    /// Show configuration and system info
    Info {
        /// Show detailed information
        #[arg(long)]
        detailed: bool,

        /// Show available models
        #[arg(long)]
        models: bool,

        /// Show available experts
        #[arg(long)]
        experts: bool,
    },

    /// Initialize configuration file
    Init {
        /// Output path for config file
        #[arg(short, long, default_value = "gnn-intel.toml")]
        output: PathBuf,

        /// Overwrite existing config
        #[arg(long)]
        force: bool,
    },
}

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
enum OutputFormat {
    Text,
    Json,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize logging
    setup_logging(cli.verbose, cli.debug)?;

    info!("GNN Code Intelligence CLI v{}", env!("CARGO_PKG_VERSION"));

    // Load configuration
    let config = if let Some(config_path) = &cli.config {
        Config::from_file(config_path)?
    } else {
        Config::default()
    };

    // Execute command
    let result = match cli.command {
        Commands::Sweep {
            language,
            min_stars,
            min_commits,
            max_results,
            output,
            token,
            use_cache,
            rate_limit,
        } => {
            sweep::run(
                language,
                min_stars,
                min_commits,
                max_results,
                output,
                token,
                use_cache,
                rate_limit,
            )
            .await
        }

        Commands::Parse {
            input,
            output,
            parallel,
            export_format,
            max_depth,
            ignore_hidden,
        } => {
            parse::run(
                input,
                output,
                parallel,
                export_format,
                max_depth,
                ignore_hidden,
            )
        }

        Commands::Train {
            graphs,
            output,
            epochs,
            batch_size,
            learning_rate,
            architecture,
            hidden_dims,
            output_dim,
            expert,
            resume,
        } => {
            train::run(
                graphs,
                output,
                epochs,
                batch_size,
                learning_rate,
                architecture,
                hidden_dims,
                output_dim,
                expert,
                resume,
            )
        }

        Commands::Check {
            path,
            language,
            severity,
            json,
            model,
            suggestions,
            fix,
        } => check::run(path, language, severity, json, model, suggestions, fix),

        Commands::Compress {
            path,
            output,
            model,
            metadata,
            format,
        } => compress::run(path, output, model, metadata, format),

        Commands::Generate {
            prompt,
            context,
            model,
            endpoint,
            output,
            language,
            temperature,
            gnn_fix,
        } => {
            generate::run(
                prompt, context, model, endpoint, output, language, temperature, gnn_fix,
            )
            .await
        }

        Commands::Info {
            detailed,
            models,
            experts,
        } => {
            info::run(detailed, models, experts);
            Ok(())
        }

        Commands::Init { output, force } => {
            init::run(output, force)?;
            Ok(())
        }
    };

    // Handle result
    if let Err(e) = result {
        eprintln!("\n{} {}", console::style("Error:").red().bold(), e);
        std::process::exit(1);
    }

    Ok(())
}

/// Setup logging based on verbosity flags
fn setup_logging(verbose: bool, debug: bool) -> Result<()> {
    let level = if debug {
        Level::DEBUG
    } else if verbose {
        Level::INFO
    } else {
        Level::WARN
    };

    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        EnvFilter::new(format!(
            "gnn_cli={},gnn_sweep={},gnn_parser={},gnn_core={},gnn_head={},gnn_experts={}",
            level, level, level, level, level, level
        ))
    });

    tracing_subscriber::registry()
        .with(filter)
        .with(fmt::layer().with_target(false).with_thread_ids(false))
        .init();

    Ok(())
}
