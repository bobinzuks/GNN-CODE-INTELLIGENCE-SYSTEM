//! Init command - Initialize configuration file

use anyhow::{Context, Result};
use colored::Colorize;
use console::style;
use std::path::PathBuf;

use crate::config::Config;

/// Run the init command
pub fn run(output: PathBuf, force: bool) -> Result<()> {
    println!("{}", style("Initializing configuration...").bold().cyan());
    println!();

    // Check if file already exists
    if output.exists() && !force {
        anyhow::bail!(
            "Configuration file already exists: {}\nUse --force to overwrite",
            output.display()
        );
    }

    // Create default configuration
    let config = Config::default();

    // Save to file
    config.to_file(&output)
        .with_context(|| format!("Failed to save configuration: {}", output.display()))?;

    println!("{} Configuration file created: {}", "✓".green(), style(output.display()).cyan());
    println!();

    // Display sample configuration
    println!("{}", style("Sample Configuration:").bold().green());
    println!("{}", "=".repeat(80));
    println!();

    print_sample_config();

    println!();
    println!("{}", "=".repeat(80));
    println!();

    println!("Edit {} to customize your settings", style(output.display()).cyan());
    println!();

    // Show quick start guide
    println!("{}", style("Quick Start:").bold().green());
    println!();
    println!("  1. Sweep GitHub repositories:");
    println!("     {}", style("gnn-intel sweep --language rust --output repos.map").cyan());
    println!();
    println!("  2. Parse repositories to graphs:");
    println!("     {}", style("gnn-intel parse --input ./my-repo --output graphs/").cyan());
    println!();
    println!("  3. Train GNN models:");
    println!("     {}", style("gnn-intel train --graphs graphs/ --output models/").cyan());
    println!();
    println!("  4. Check code quality:");
    println!("     {}", style("gnn-intel check --path ./my-code").cyan());
    println!();
    println!("  5. Compress codebase:");
    println!("     {}", style("gnn-intel compress --path ./my-repo --output embedding.bin").cyan());
    println!();
    println!("  6. Generate code:");
    println!("     {}", style("gnn-intel generate --prompt \"Create a REST API\" --context embedding.bin").cyan());
    println!();

    Ok(())
}

/// Print sample configuration
fn print_sample_config() {
    let sample = r#"# GNN Code Intelligence Configuration

[sweep]
default_language = "Rust"
min_stars = 1
min_commits = 50
max_results = 1000
rate_limit = 10
use_cache = true
cache_dir = ".cache"

[parser]
parallel_workers = 8
ignore_hidden = true
ignore_patterns = ["target", "node_modules", ".git", "dist", "build"]
export_format = "bincode"
follow_links = false

[training]
epochs = 100
batch_size = 32
learning_rate = 0.001
architecture = "sage"
hidden_dims = [256, 256]
output_dim = 512
checkpoint_frequency = 10
early_stopping_patience = 20

[models]
models_dir = "models"
default_experts = ["rust", "python", "go", "typescript"]

[llm]
default_model = "codellama"
endpoint = "http://localhost:11434"
temperature = 0.7
max_tokens = 2048
gnn_fix = true

[output]
output_dir = "output"
colored = true
show_progress = true
verbosity = 0
"#;

    println!("{}", sample);
}
