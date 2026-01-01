//! Sweep command - GitHub repository discovery

use anyhow::{Context, Result};
use colored::Colorize;
use console::style;
use indicatif::{ProgressBar, ProgressStyle};
use std::path::PathBuf;
use tokio::sync::mpsc;
use tracing::{info, warn};

use sweep::{
    cache::RepoCache,
    filter::RepoFilter,
    github::GitHubClient,
    output::MapFileWriter,
    ScoredRepo, SweepConfig,
};

/// Run the sweep command
pub async fn run(
    language: Option<String>,
    min_stars: u32,
    min_commits: u32,
    max_results: usize,
    output: PathBuf,
    token: Option<String>,
    use_cache: bool,
    rate_limit: usize,
) -> Result<()> {
    println!("{}", style("Sweeping GitHub for quality repositories...").bold().cyan());
    println!();

    // Create configuration
    let config = SweepConfig {
        language: language.clone().unwrap_or_else(|| "Rust".to_string()),
        min_stars,
        max_results,
        output_dir: output
            .parent()
            .unwrap_or_else(|| std::path::Path::new("."))
            .to_string_lossy()
            .to_string(),
        rate_limit,
        use_cache,
    };

    // Display configuration
    println!("Configuration:");
    println!("  Language: {}", style(&config.language).green());
    println!("  Min stars: {}", style(min_stars).green());
    println!("  Min commits: {}", style(min_commits).green());
    println!("  Max results: {}", style(max_results).green());
    println!("  Output: {}", style(output.display()).green());
    println!("  Rate limit: {} req/s", style(rate_limit).green());
    println!("  Cache: {}", if use_cache { style("enabled").green() } else { style("disabled").yellow() });
    println!();

    // Create output directory if it doesn't exist
    if let Some(parent) = output.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create output directory: {}", parent.display()))?;
    }

    // Initialize GitHub client
    let client = if let Some(token) = token {
        GitHubClient::with_token(token)
    } else {
        GitHubClient::new()
    };

    // Initialize cache
    let cache = if use_cache {
        Some(RepoCache::new(&config.output_dir)?)
    } else {
        None
    };

    // Create filter
    let filter = RepoFilter::new()
        .with_min_stars(min_stars)
        .with_min_commits(min_commits);

    // Create output writer
    let mut writer = MapFileWriter::new(&output)?;

    // Setup progress bar
    let progress = ProgressBar::new(max_results as u64);
    progress.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta}) {msg}")
            .expect("Invalid progress bar template")
            .progress_chars("#>-"),
    );

    // Channel for streaming results
    let (tx, mut rx) = mpsc::channel::<ScoredRepo>(1000);

    // Spawn sweep task
    let sweep_language = config.language.clone();
    let sweep_handle = tokio::spawn(async move {
        client.sweep_language(&sweep_language, &filter, max_results, tx).await
    });

    // Process results as they come in
    let mut count = 0;
    let mut tier_counts = std::collections::HashMap::new();

    while let Some(scored_repo) = rx.recv().await {
        writer.write_repo(&scored_repo)?;
        count += 1;

        // Update tier counts
        *tier_counts.entry(scored_repo.tier).or_insert(0) += 1;

        // Update progress
        progress.set_position(count as u64);
        progress.set_message(format!(
            "{} - {} stars",
            scored_repo.repo.full_name,
            scored_repo.repo.stargazers_count
        ));
    }

    // Wait for sweep to complete
    let sweep_result = sweep_handle.await?;
    progress.finish_with_message("Sweep complete!");

    // Display summary
    println!();
    println!("{}", style("Sweep Summary:").bold().green());
    println!("{}", "=".repeat(50));
    println!("Total repositories: {}", style(count).bold().green());
    println!();

    if !tier_counts.is_empty() {
        println!("Repositories by tier:");
        for tier in [
            sweep::Tier::S,
            sweep::Tier::A,
            sweep::Tier::B,
            sweep::Tier::C,
            sweep::Tier::D,
        ] {
            if let Some(&count) = tier_counts.get(&tier) {
                let tier_str = format!("  Tier {}: ", tier);
                let count_str = format!("{:>5}", count);
                println!("{}{}", tier_str, style(count_str).green());
            }
        }
        println!();
    }

    println!("Output written to: {}", style(output.display()).cyan());

    if let Err(e) = sweep_result {
        warn!("Sweep completed with warnings: {}", e);
    }

    Ok(())
}
