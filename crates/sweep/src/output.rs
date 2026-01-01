use crate::ScoredRepo;
use anyhow::{Context, Result};
use csv::Writer;
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use tracing::info;

/// CSV output writer for .map files
pub struct MapFile {
    writer: Writer<File>,
    path: PathBuf,
    count: usize,
}

impl MapFile {
    /// Create a new map file
    pub fn create<P: AsRef<Path>>(output_dir: P, language: &str) -> Result<Self> {
        let output_dir = output_dir.as_ref();
        fs::create_dir_all(output_dir)
            .context("Failed to create output directory")?;

        let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
        let filename = format!("{}_{}.map", language.to_lowercase(), timestamp);
        let path = output_dir.join(&filename);

        let file = File::create(&path)
            .context("Failed to create map file")?;

        let mut writer = Writer::from_writer(file);

        // Write CSV header
        writer.write_record(&[
            "full_name",
            "url",
            "stars",
            "forks",
            "language",
            "score",
            "tier",
            "description",
            "topics",
            "license",
            "created_at",
            "updated_at",
            "pushed_at",
        ]).context("Failed to write CSV header")?;

        info!("Created map file: {}", path.display());

        Ok(Self {
            writer,
            path,
            count: 0,
        })
    }

    /// Write a scored repository to the map file
    pub fn write(&mut self, scored: &ScoredRepo) -> Result<()> {
        let repo = &scored.repo;

        let topics = repo.topics.join(", ");
        let license = repo.license.as_ref()
            .map(|l| l.name.clone())
            .unwrap_or_else(|| "None".to_string());

        let description = repo.description.as_ref()
            .map(|d| d.as_str())
            .unwrap_or("");

        let pushed_at = repo.pushed_at
            .map(|dt| dt.to_rfc3339())
            .unwrap_or_else(|| "N/A".to_string());

        self.writer.write_record(&[
            &repo.full_name,
            &repo.html_url,
            &repo.stargazers_count.to_string(),
            &repo.forks_count.to_string(),
            repo.language.as_ref().unwrap_or(&"Unknown".to_string()),
            &format!("{:.2}", scored.score),
            scored.tier.as_str(),
            description,
            &topics,
            &license,
            &repo.created_at.to_rfc3339(),
            &repo.updated_at.to_rfc3339(),
            &pushed_at,
        ]).context("Failed to write repository record")?;

        self.count += 1;

        // Flush periodically
        if self.count % 100 == 0 {
            self.writer.flush()
                .context("Failed to flush CSV writer")?;
        }

        Ok(())
    }

    /// Flush and finalize the file
    pub fn finalize(mut self) -> Result<PathBuf> {
        self.writer.flush()
            .context("Failed to flush CSV writer")?;

        info!("Finalized map file with {} repositories: {}", self.count, self.path.display());
        Ok(self.path)
    }

    /// Get the current path
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Get the number of repositories written
    pub fn count(&self) -> usize {
        self.count
    }
}

/// JSON output writer for detailed data
pub struct JsonOutput {
    repos: Vec<ScoredRepo>,
    path: PathBuf,
}

impl JsonOutput {
    pub fn new<P: AsRef<Path>>(output_dir: P, language: &str) -> Result<Self> {
        let output_dir = output_dir.as_ref();
        fs::create_dir_all(output_dir)
            .context("Failed to create output directory")?;

        let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
        let filename = format!("{}_{}.json", language.to_lowercase(), timestamp);
        let path = output_dir.join(&filename);

        Ok(Self {
            repos: Vec::new(),
            path,
        })
    }

    pub fn add(&mut self, scored: ScoredRepo) {
        self.repos.push(scored);
    }

    pub fn finalize(self) -> Result<PathBuf> {
        let file = File::create(&self.path)
            .context("Failed to create JSON output file")?;

        serde_json::to_writer_pretty(file, &self.repos)
            .context("Failed to write JSON output")?;

        info!("Finalized JSON file with {} repositories: {}", self.repos.len(), self.path.display());
        Ok(self.path)
    }
}

/// Summary statistics for a sweep operation
#[derive(Debug, Clone, serde::Serialize)]
pub struct SweepSummary {
    pub language: String,
    pub total_repos: usize,
    pub tier_counts: TierCounts,
    pub avg_score: f64,
    pub top_repos: Vec<String>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct TierCounts {
    pub s: usize,
    pub a: usize,
    pub b: usize,
    pub c: usize,
    pub d: usize,
}

impl SweepSummary {
    pub fn from_repos(language: String, repos: &[ScoredRepo]) -> Self {
        let total_repos = repos.len();

        let mut tier_counts = TierCounts {
            s: 0,
            a: 0,
            b: 0,
            c: 0,
            d: 0,
        };

        let mut total_score = 0.0;

        for repo in repos {
            total_score += repo.score;
            match repo.tier {
                crate::Tier::S => tier_counts.s += 1,
                crate::Tier::A => tier_counts.a += 1,
                crate::Tier::B => tier_counts.b += 1,
                crate::Tier::C => tier_counts.c += 1,
                crate::Tier::D => tier_counts.d += 1,
            }
        }

        let avg_score = if total_repos > 0 {
            total_score / total_repos as f64
        } else {
            0.0
        };

        // Get top 10 repos by score
        let mut sorted = repos.to_vec();
        sorted.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        let top_repos = sorted.iter()
            .take(10)
            .map(|r| format!("{} ({})", r.repo.full_name, r.score))
            .collect();

        Self {
            language,
            total_repos,
            tier_counts,
            avg_score,
            top_repos,
        }
    }

    pub fn print(&self) {
        println!("\n=== Sweep Summary for {} ===", self.language);
        println!("Total repositories: {}", self.total_repos);
        println!("Average score: {:.2}", self.avg_score);
        println!("\nTier distribution:");
        println!("  S: {} ({:.1}%)", self.tier_counts.s,
                 self.tier_counts.s as f64 / self.total_repos as f64 * 100.0);
        println!("  A: {} ({:.1}%)", self.tier_counts.a,
                 self.tier_counts.a as f64 / self.total_repos as f64 * 100.0);
        println!("  B: {} ({:.1}%)", self.tier_counts.b,
                 self.tier_counts.b as f64 / self.total_repos as f64 * 100.0);
        println!("  C: {} ({:.1}%)", self.tier_counts.c,
                 self.tier_counts.c as f64 / self.total_repos as f64 * 100.0);
        println!("  D: {} ({:.1}%)", self.tier_counts.d,
                 self.tier_counts.d as f64 / self.total_repos as f64 * 100.0);

        if !self.top_repos.is_empty() {
            println!("\nTop repositories:");
            for (i, repo) in self.top_repos.iter().enumerate() {
                println!("  {}. {}", i + 1, repo);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{RepoMeta, Tier};
    use chrono::Utc;
    use tempfile::TempDir;

    fn create_test_scored_repo(name: &str, score: f64) -> ScoredRepo {
        let repo = RepoMeta {
            full_name: name.to_string(),
            html_url: format!("https://github.com/{}", name),
            description: Some("Test repo".to_string()),
            stargazers_count: 1000,
            forks_count: 100,
            open_issues_count: 10,
            size: 5000,
            language: Some("Rust".to_string()),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            pushed_at: Some(Utc::now()),
            default_branch: "main".to_string(),
            topics: vec!["rust".to_string()],
            license: Some(crate::License {
                key: "mit".to_string(),
                name: "MIT License".to_string(),
                spdx_id: Some("MIT".to_string()),
            }),
            has_issues: true,
            archived: false,
            disabled: false,
            fork: false,
        };

        ScoredRepo::new(repo, score)
    }

    #[test]
    fn test_map_file_creation() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let map_file = MapFile::create(temp_dir.path(), "rust")?;
        assert!(map_file.path().exists());
        Ok(())
    }

    #[test]
    fn test_map_file_write() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let mut map_file = MapFile::create(temp_dir.path(), "rust")?;

        let scored = create_test_scored_repo("test/repo", 85.0);
        map_file.write(&scored)?;

        assert_eq!(map_file.count(), 1);
        Ok(())
    }

    #[test]
    fn test_sweep_summary() {
        let repos = vec![
            create_test_scored_repo("repo1", 95.0),
            create_test_scored_repo("repo2", 80.0),
            create_test_scored_repo("repo3", 60.0),
            create_test_scored_repo("repo4", 40.0),
            create_test_scored_repo("repo5", 20.0),
        ];

        let summary = SweepSummary::from_repos("Rust".to_string(), &repos);
        assert_eq!(summary.total_repos, 5);
        assert_eq!(summary.tier_counts.s, 1);
        assert_eq!(summary.tier_counts.a, 1);
        assert_eq!(summary.tier_counts.b, 1);
        assert_eq!(summary.tier_counts.c, 1);
        assert_eq!(summary.tier_counts.d, 1);
    }
}
