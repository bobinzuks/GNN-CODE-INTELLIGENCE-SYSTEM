use anyhow::Result;
use clap::{Parser, Subcommand};
use sweep::{
    cache::RepoCache,
    filter::RepoFilter,
    github::{sweep_language, GitHubClient},
    output::{MapFile, SweepSummary},
    ScoredRepo,
};
use tokio::sync::mpsc;
use tracing::{info, warn};
use tracing_subscriber::EnvFilter;

#[derive(Parser)]
#[command(name = "sweep")]
#[command(about = "Fast GitHub repository discovery and scoring system", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Sweep repositories for a given language
    Sweep {
        /// Programming language to search for
        #[arg(short, long)]
        language: String,

        /// Minimum number of stars
        #[arg(short = 's', long, default_value = "100")]
        min_stars: u32,

        /// Minimum number of forks
        #[arg(short = 'f', long, default_value = "10")]
        min_forks: u32,

        /// Maximum number of results
        #[arg(short = 'n', long, default_value = "1000")]
        max_results: usize,

        /// Output directory for .map files
        #[arg(short, long, default_value = "./output")]
        output_dir: String,

        /// GitHub API token (optional, but recommended)
        /// Can also be set via GITHUB_TOKEN environment variable
        #[arg(short, long)]
        token: Option<String>,

        /// Rate limit (concurrent requests)
        #[arg(short, long, default_value = "10")]
        rate_limit: usize,

        /// Exclude forks
        #[arg(long, default_value = "true")]
        exclude_forks: bool,

        /// Exclude archived repos
        #[arg(long, default_value = "true")]
        exclude_archived: bool,

        /// Use cache
        #[arg(long, default_value = "true")]
        use_cache: bool,
    },

    /// View cache statistics
    CacheStats {
        /// Path to cache database
        #[arg(short, long, default_value = "./sweep_cache.db")]
        db_path: String,
    },

    /// Clear old cache entries
    CacheClear {
        /// Path to cache database
        #[arg(short, long, default_value = "./sweep_cache.db")]
        db_path: String,

        /// Number of days to keep
        #[arg(short, long, default_value = "7")]
        days: i64,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("info"))
        )
        .init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Sweep {
            language,
            min_stars,
            min_forks,
            max_results,
            output_dir,
            token,
            rate_limit,
            exclude_forks,
            exclude_archived,
            use_cache,
        } => {
            // Try to get token from environment if not provided
            let token = token.or_else(|| std::env::var("GITHUB_TOKEN").ok());

            run_sweep(
                language,
                min_stars,
                min_forks,
                max_results,
                output_dir,
                token,
                rate_limit,
                exclude_forks,
                exclude_archived,
                use_cache,
            )
            .await?;
        }
        Commands::CacheStats { db_path } => {
            show_cache_stats(&db_path)?;
        }
        Commands::CacheClear { db_path, days } => {
            clear_cache(&db_path, days)?;
        }
    }

    Ok(())
}

async fn run_sweep(
    language: String,
    min_stars: u32,
    min_forks: u32,
    max_results: usize,
    output_dir: String,
    token: Option<String>,
    rate_limit: usize,
    exclude_forks: bool,
    exclude_archived: bool,
    use_cache: bool,
) -> Result<()> {
    info!("Starting sweep for language: {}", language);
    info!("Minimum stars: {}, forks: {}", min_stars, min_forks);
    info!("Maximum results: {}", max_results);

    // Create GitHub client
    let client = GitHubClient::new(token.clone(), rate_limit);

    // Create filter
    let filter = RepoFilter::new()
        .min_stars(min_stars)
        .min_forks(min_forks)
        .exclude_forks(exclude_forks)
        .exclude_archived(exclude_archived);

    // Create output file
    let mut map_file = MapFile::create(&output_dir, &language)?;

    // Create channel for streaming results
    let (tx, mut rx) = mpsc::channel::<ScoredRepo>(1000);

    // Start sweep in background
    let sweep_handle = tokio::spawn({
        let language = language.clone();
        async move {
            sweep_language(&client, &language, min_stars, max_results, filter, tx).await
        }
    });

    // Collect and write results
    let mut all_repos = Vec::new();
    let mut count = 0;

    while let Some(scored) = rx.recv().await {
        map_file.write(&scored)?;
        all_repos.push(scored.clone());
        count += 1;

        if count % 50 == 0 {
            info!("Processed {} repositories", count);
        }
    }

    // Wait for sweep to complete
    sweep_handle.await??;

    // Finalize map file
    let map_path = map_file.finalize()?;

    // Store in cache if enabled
    if use_cache {
        let cache = RepoCache::new("./sweep_cache.db")?;
        for scored in &all_repos {
            if let Err(e) = cache.store(&scored.repo) {
                warn!("Failed to cache repository {}: {}", scored.repo.full_name, e);
            }
        }
        info!("Cached {} repositories", all_repos.len());
    }

    // Print summary
    let summary = SweepSummary::from_repos(language, &all_repos);
    summary.print();

    println!("\nOutput file: {}", map_path.display());
    println!("Total repositories written: {}", count);

    Ok(())
}

fn show_cache_stats(db_path: &str) -> Result<()> {
    let cache = RepoCache::new(db_path)?;
    let stats = cache.stats()?;

    println!("\n=== Cache Statistics ===");
    println!("Total repositories: {}", stats.total_repos);
    println!("\nTop languages:");
    for (i, (lang, count)) in stats.languages.iter().enumerate() {
        println!("  {}. {}: {}", i + 1, lang, count);
    }

    Ok(())
}

fn clear_cache(db_path: &str, days: i64) -> Result<()> {
    let cache = RepoCache::new(db_path)?;
    let deleted = cache.clear_old_entries(days)?;

    println!("Cleared {} cache entries older than {} days", deleted, days);

    Ok(())
}
