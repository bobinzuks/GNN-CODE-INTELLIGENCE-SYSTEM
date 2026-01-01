pub mod github;
pub mod filter;
pub mod cache;
pub mod output;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Repository metadata from GitHub API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepoMeta {
    pub full_name: String,
    pub html_url: String,
    pub description: Option<String>,
    pub stargazers_count: u32,
    pub forks_count: u32,
    pub open_issues_count: u32,
    pub size: u32,
    pub language: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub pushed_at: Option<DateTime<Utc>>,
    pub default_branch: String,
    pub topics: Vec<String>,
    pub license: Option<License>,
    pub has_issues: bool,
    pub archived: bool,
    pub disabled: bool,
    pub fork: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct License {
    pub key: String,
    pub name: String,
    pub spdx_id: Option<String>,
}

/// Repository with computed score
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoredRepo {
    pub repo: RepoMeta,
    pub score: f64,
    pub tier: Tier,
}

impl ScoredRepo {
    pub fn new(repo: RepoMeta, score: f64) -> Self {
        let tier = Tier::from_score(score);
        Self { repo, score, tier }
    }
}

/// Quality tier classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Tier {
    S,  // 90-100: Elite repos
    A,  // 75-89: High quality
    B,  // 50-74: Good quality
    C,  // 25-49: Average
    D,  // 0-24: Low quality
}

impl Tier {
    pub fn from_score(score: f64) -> Self {
        match score {
            s if s >= 90.0 => Tier::S,
            s if s >= 75.0 => Tier::A,
            s if s >= 50.0 => Tier::B,
            s if s >= 25.0 => Tier::C,
            _ => Tier::D,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Tier::S => "S",
            Tier::A => "A",
            Tier::B => "B",
            Tier::C => "C",
            Tier::D => "D",
        }
    }
}

impl fmt::Display for Tier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Configuration for sweep operations
#[derive(Debug, Clone)]
pub struct SweepConfig {
    pub language: String,
    pub min_stars: u32,
    pub max_results: usize,
    pub output_dir: String,
    pub rate_limit: usize,
    pub use_cache: bool,
}

impl Default for SweepConfig {
    fn default() -> Self {
        Self {
            language: "Rust".to_string(),
            min_stars: 100,
            max_results: 1000,
            output_dir: "./output".to_string(),
            rate_limit: 10,
            use_cache: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tier_classification() {
        assert_eq!(Tier::from_score(95.0), Tier::S);
        assert_eq!(Tier::from_score(80.0), Tier::A);
        assert_eq!(Tier::from_score(60.0), Tier::B);
        assert_eq!(Tier::from_score(30.0), Tier::C);
        assert_eq!(Tier::from_score(10.0), Tier::D);
    }

    #[test]
    fn test_tier_display() {
        assert_eq!(Tier::S.to_string(), "S");
        assert_eq!(Tier::A.to_string(), "A");
        assert_eq!(Tier::B.to_string(), "B");
        assert_eq!(Tier::C.to_string(), "C");
        assert_eq!(Tier::D.to_string(), "D");
    }
}
