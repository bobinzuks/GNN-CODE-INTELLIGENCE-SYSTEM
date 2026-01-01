use crate::RepoMeta;
use chrono::Utc;

/// Repository filtering and scoring logic
#[derive(Debug, Clone)]
pub struct RepoFilter {
    pub min_stars: u32,
    pub min_forks: u32,
    pub max_age_days: Option<i64>,
    pub exclude_forks: bool,
    pub exclude_archived: bool,
    pub required_topics: Vec<String>,
}

impl Default for RepoFilter {
    fn default() -> Self {
        Self {
            min_stars: 100,
            min_forks: 10,
            max_age_days: None,
            exclude_forks: true,
            exclude_archived: true,
            required_topics: Vec::new(),
        }
    }
}

impl RepoFilter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn min_stars(mut self, stars: u32) -> Self {
        self.min_stars = stars;
        self
    }

    pub fn min_forks(mut self, forks: u32) -> Self {
        self.min_forks = forks;
        self
    }

    pub fn max_age_days(mut self, days: i64) -> Self {
        self.max_age_days = Some(days);
        self
    }

    pub fn exclude_forks(mut self, exclude: bool) -> Self {
        self.exclude_forks = exclude;
        self
    }

    pub fn exclude_archived(mut self, exclude: bool) -> Self {
        self.exclude_archived = exclude;
        self
    }

    pub fn required_topics(mut self, topics: Vec<String>) -> Self {
        self.required_topics = topics;
        self
    }

    /// Check if a repository should be included based on filters
    pub fn should_include(&self, repo: &RepoMeta) -> bool {
        // Check archived status
        if self.exclude_archived && repo.archived {
            return false;
        }

        // Check fork status
        if self.exclude_forks && repo.fork {
            return false;
        }

        // Check disabled status
        if repo.disabled {
            return false;
        }

        // Check minimum stars
        if repo.stargazers_count < self.min_stars {
            return false;
        }

        // Check minimum forks
        if repo.forks_count < self.min_forks {
            return false;
        }

        // Check age
        if let Some(max_age) = self.max_age_days {
            let age = Utc::now().signed_duration_since(repo.created_at);
            if age.num_days() > max_age {
                return false;
            }
        }

        // Check required topics
        if !self.required_topics.is_empty() {
            let has_all_topics = self
                .required_topics
                .iter()
                .all(|topic| repo.topics.contains(topic));
            if !has_all_topics {
                return false;
            }
        }

        true
    }

    /// Calculate quality score for a repository (0-100)
    pub fn calculate_score(&self, repo: &RepoMeta) -> f64 {
        let mut score = 0.0;

        // Stars contribution (40 points max)
        // Logarithmic scale: 1k stars = 20pts, 10k = 30pts, 100k = 40pts
        let star_score = if repo.stargazers_count > 0 {
            (repo.stargazers_count as f64).log10() * 10.0
        } else {
            0.0
        };
        score += star_score.min(40.0);

        // Forks contribution (20 points max)
        // More forks indicate community engagement
        let fork_ratio = repo.forks_count as f64 / (repo.stargazers_count.max(1) as f64);
        let fork_score = (fork_ratio * 100.0).min(20.0);
        score += fork_score;

        // Activity contribution (20 points max)
        // Recent activity is valuable
        if let Some(pushed_at) = repo.pushed_at {
            let days_since_push = Utc::now()
                .signed_duration_since(pushed_at)
                .num_days();

            let activity_score = match days_since_push {
                0..=30 => 20.0,      // Very active
                31..=90 => 15.0,     // Active
                91..=180 => 10.0,    // Moderately active
                181..=365 => 5.0,    // Somewhat active
                _ => 0.0,            // Inactive
            };
            score += activity_score;
        }

        // Documentation/Quality indicators (10 points max)
        let mut quality_score = 0.0;

        // Has description
        if repo.description.is_some() && !repo.description.as_ref().unwrap().is_empty() {
            quality_score += 2.0;
        }

        // Has license
        if repo.license.is_some() {
            quality_score += 3.0;
        }

        // Has topics
        if !repo.topics.is_empty() {
            quality_score += 2.0;
        }

        // Has issues enabled
        if repo.has_issues {
            quality_score += 1.0;
        }

        // Not too many open issues (indicates maintenance)
        let issue_ratio = repo.open_issues_count as f64 / (repo.stargazers_count.max(1) as f64);
        if issue_ratio < 0.1 {
            quality_score += 2.0;
        }

        score += quality_score;

        // Size contribution (10 points max)
        // Moderate size is good (not too small, not too large)
        let size_score = match repo.size {
            0..=100 => 2.0,           // Very small
            101..=1000 => 6.0,        // Small
            1001..=10000 => 10.0,     // Medium (ideal)
            10001..=100000 => 8.0,    // Large
            _ => 4.0,                 // Very large
        };
        score += size_score;

        // Normalize to 0-100
        score.min(100.0).max(0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, Utc};

    fn create_test_repo() -> RepoMeta {
        RepoMeta {
            full_name: "test/repo".to_string(),
            html_url: "https://github.com/test/repo".to_string(),
            description: Some("Test repo".to_string()),
            stargazers_count: 1000,
            forks_count: 100,
            open_issues_count: 10,
            size: 5000,
            language: Some("Rust".to_string()),
            created_at: Utc::now() - Duration::days(365),
            updated_at: Utc::now(),
            pushed_at: Some(Utc::now() - Duration::days(7)),
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
        }
    }

    #[test]
    fn test_should_include_valid_repo() {
        let filter = RepoFilter::default();
        let repo = create_test_repo();
        assert!(filter.should_include(&repo));
    }

    #[test]
    fn test_should_exclude_archived() {
        let filter = RepoFilter::default();
        let mut repo = create_test_repo();
        repo.archived = true;
        assert!(!filter.should_include(&repo));
    }

    #[test]
    fn test_should_exclude_fork() {
        let filter = RepoFilter::default();
        let mut repo = create_test_repo();
        repo.fork = true;
        assert!(!filter.should_include(&repo));
    }

    #[test]
    fn test_should_exclude_low_stars() {
        let filter = RepoFilter::default().min_stars(2000);
        let repo = create_test_repo();
        assert!(!filter.should_include(&repo));
    }

    #[test]
    fn test_calculate_score() {
        let filter = RepoFilter::default();
        let repo = create_test_repo();
        let score = filter.calculate_score(&repo);

        assert!(score > 0.0);
        assert!(score <= 100.0);
        // A decent repo should score reasonably well
        assert!(score > 40.0);
    }

    #[test]
    fn test_score_ranges() {
        let filter = RepoFilter::default();

        // High quality repo
        let mut repo = create_test_repo();
        repo.stargazers_count = 50000;
        repo.forks_count = 5000;
        let high_score = filter.calculate_score(&repo);
        assert!(high_score > 70.0);

        // Low quality repo
        repo.stargazers_count = 10;
        repo.forks_count = 1;
        repo.description = None;
        repo.license = None;
        repo.topics = vec![];
        let low_score = filter.calculate_score(&repo);
        assert!(low_score < 40.0);
    }
}
