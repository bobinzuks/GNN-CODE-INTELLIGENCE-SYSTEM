//! Info command - Show system information and configuration

use colored::Colorize;
use comfy_table::{presets::UTF8_FULL, ContentArrangement, Table};
use console::style;

/// Run the info command
pub fn run(detailed: bool, models: bool, experts: bool) {
    println!();
    println!("{}", style("GNN Code Intelligence System").bold().cyan());
    println!("{}", "=".repeat(80));
    println!();

    // Basic info
    show_basic_info();

    if detailed {
        println!();
        show_detailed_info();
    }

    if models {
        println!();
        show_models_info();
    }

    if experts {
        println!();
        show_experts_info();
    }

    if !detailed && !models && !experts {
        println!();
        println!("{}", style("Use --detailed, --models, or --experts for more information").dim());
    }

    println!();
}

/// Show basic system information
fn show_basic_info() {
    println!("{}", style("System Information:").bold().green());
    println!();
    println!("  Version: {}", style(env!("CARGO_PKG_VERSION")).green());
    println!("  Build: {}", style("release").green());
    println!("  Authors: {}", style(env!("CARGO_PKG_AUTHORS")).green());
    println!();

    println!("{}", style("Components:").bold().green());
    println!("  CLI: {}", style("✓").green());
    println!("  Sweep: {}", style("✓").green());
    println!("  Parser: {}", style("✓").green());
    println!("  GNN Core: {}", style("✓").green());
    println!("  GNN Head: {}", style("✓").green());
    println!("  GNN Experts: {}", style("✓").green());
    println!("  WASM Runtime: {}", style("✓").green());
}

/// Show detailed system information
fn show_detailed_info() {
    println!("{}", style("Detailed Configuration:").bold().green());
    println!();

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic);

    table.set_header(vec!["Component", "Version", "Status", "Description"]);

    table.add_row(vec![
        "gnn-sweep",
        env!("CARGO_PKG_VERSION"),
        "Active",
        "GitHub repository discovery",
    ]);

    table.add_row(vec![
        "gnn-parser",
        env!("CARGO_PKG_VERSION"),
        "Active",
        "Multi-language code parser",
    ]);

    table.add_row(vec![
        "gnn-core",
        env!("CARGO_PKG_VERSION"),
        "Active",
        "GNN training and inference",
    ]);

    table.add_row(vec![
        "gnn-head",
        env!("CARGO_PKG_VERSION"),
        "Active",
        "Expert orchestrator",
    ]);

    table.add_row(vec![
        "gnn-experts",
        env!("CARGO_PKG_VERSION"),
        "Active",
        "Language-specific experts",
    ]);

    table.add_row(vec![
        "wasm-runtime",
        env!("CARGO_PKG_VERSION"),
        "Active",
        "WASM compilation target",
    ]);

    println!("{}", table);
    println!();

    println!("{}", style("Capabilities:").bold().green());
    println!("  Supported languages: Rust, Python, Go, TypeScript, Java, Swift, C#, Zig");
    println!("  GNN architectures: GraphSAGE, GAT");
    println!("  Training modes: Contrastive, Supervised");
    println!("  Output formats: Bincode, JSON, DOT");
    println!("  LLM integration: Ollama, OpenAI-compatible APIs");
}

/// Show available models
fn show_models_info() {
    println!("{}", style("Available Models:").bold().green());
    println!();

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic);

    table.set_header(vec!["Model", "Type", "Input Dim", "Output Dim", "Status"]);

    // Check for models
    let models_dir = std::path::Path::new("models");

    if models_dir.exists() {
        // List actual models
        if let Ok(entries) = std::fs::read_dir(models_dir) {
            let mut has_models = false;

            for entry in entries.flatten() {
                if entry.path().extension().and_then(|s| s.to_str()) == Some("bin") {
                    has_models = true;
                    let name = entry.file_name().to_string_lossy().to_string();
                    table.add_row(vec![
                        name,
                        "GraphSAGE".to_string(),
                        "128".to_string(),
                        "512".to_string(),
                        "Available".to_string(),
                    ]);
                }
            }

            if !has_models {
                println!("  {}", style("No trained models found").yellow());
                println!("  Run 'gnn-intel train' to train models");
            } else {
                println!("{}", table);
            }
        }
    } else {
        println!("  {}", style("Models directory not found").yellow());
        println!("  Run 'gnn-intel train' to train models");
    }

    println!();
    println!("{}", style("Default Model Configuration:").bold());
    println!("  Architecture: GraphSAGE");
    println!("  Input dimension: 128");
    println!("  Hidden dimensions: [256, 256]");
    println!("  Output dimension: 512");
    println!("  Aggregation: Mean");
    println!("  Attention heads: 4");
}

/// Show available experts
fn show_experts_info() {
    println!("{}", style("Available Language Experts:").bold().green());
    println!();

    let registry = gnn_experts::registry::ExpertRegistry::new();
    let languages = registry.languages();

    if languages.is_empty() {
        println!("  {}", style("No experts loaded").yellow());
        return;
    }

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic);

    table.set_header(vec!["Language", "Patterns", "Status"]);

    for lang in languages {
        if let Some(expert) = registry.get(lang) {
            let pattern_count = expert.patterns().len();
            table.add_row(vec![
                lang.to_string(),
                pattern_count.to_string(),
                "Active".to_string(),
            ]);
        }
    }

    println!("{}", table);
    println!();

    // Show example patterns for Rust
    if let Some(rust_expert) = registry.get("rust") {
        println!("{}", style("Example Patterns (Rust Expert):").bold());
        for (i, pattern) in rust_expert.patterns().iter().take(5).enumerate() {
            println!("  {}. {} - {}", i + 1, pattern.name, pattern.description);
        }
        if rust_expert.patterns().len() > 5 {
            println!("  ... and {} more", rust_expert.patterns().len() - 5);
        }
    }
}
