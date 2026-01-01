use crate::RepoMeta;
use anyhow::{Context, Result};
use rusqlite::{params, Connection};
use std::path::Path;
use tracing::info;

/// SQLite cache for repository metadata
pub struct RepoCache {
    conn: Connection,
}

impl RepoCache {
    /// Create or open a cache database
    pub fn new<P: AsRef<Path>>(db_path: P) -> Result<Self> {
        let conn = Connection::open(db_path)
            .context("Failed to open cache database")?;

        let cache = Self { conn };
        cache.init_schema()?;

        Ok(cache)
    }

    /// Create in-memory cache (for testing)
    pub fn in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()
            .context("Failed to create in-memory database")?;

        let cache = Self { conn };
        cache.init_schema()?;

        Ok(cache)
    }

    /// Initialize database schema
    fn init_schema(&self) -> Result<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS repos (
                full_name TEXT PRIMARY KEY,
                html_url TEXT NOT NULL,
                description TEXT,
                stars INTEGER NOT NULL,
                forks INTEGER NOT NULL,
                open_issues INTEGER NOT NULL,
                size INTEGER NOT NULL,
                language TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                pushed_at TEXT,
                default_branch TEXT NOT NULL,
                topics TEXT,
                license_key TEXT,
                license_name TEXT,
                has_issues BOOLEAN NOT NULL,
                archived BOOLEAN NOT NULL,
                disabled BOOLEAN NOT NULL,
                fork BOOLEAN NOT NULL,
                cached_at TEXT NOT NULL
            )",
            [],
        ).context("Failed to create repos table")?;

        // Index for common queries
        self.conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_repos_language_stars
             ON repos(language, stars DESC)",
            [],
        ).context("Failed to create index")?;

        self.conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_repos_cached_at
             ON repos(cached_at)",
            [],
        ).context("Failed to create cached_at index")?;

        Ok(())
    }

    /// Store a repository in cache
    pub fn store(&self, repo: &RepoMeta) -> Result<()> {
        let topics = serde_json::to_string(&repo.topics)?;
        let cached_at = chrono::Utc::now().to_rfc3339();

        let (license_key, license_name) = match &repo.license {
            Some(license) => (Some(license.key.clone()), Some(license.name.clone())),
            None => (None, None),
        };

        self.conn.execute(
            "INSERT OR REPLACE INTO repos (
                full_name, html_url, description, stars, forks, open_issues, size,
                language, created_at, updated_at, pushed_at, default_branch,
                topics, license_key, license_name, has_issues, archived, disabled, fork,
                cached_at
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20)",
            params![
                repo.full_name,
                repo.html_url,
                repo.description,
                repo.stargazers_count,
                repo.forks_count,
                repo.open_issues_count,
                repo.size,
                repo.language,
                repo.created_at.to_rfc3339(),
                repo.updated_at.to_rfc3339(),
                repo.pushed_at.map(|dt| dt.to_rfc3339()),
                repo.default_branch,
                topics,
                license_key,
                license_name,
                repo.has_issues,
                repo.archived,
                repo.disabled,
                repo.fork,
                cached_at,
            ],
        ).context("Failed to insert repository into cache")?;

        Ok(())
    }

    /// Retrieve a repository from cache
    pub fn get(&self, full_name: &str) -> Result<Option<RepoMeta>> {
        let mut stmt = self.conn.prepare(
            "SELECT full_name, html_url, description, stars, forks, open_issues, size,
                    language, created_at, updated_at, pushed_at, default_branch,
                    topics, license_key, license_name, has_issues, archived, disabled, fork
             FROM repos WHERE full_name = ?1"
        )?;

        let result = stmt.query_row(params![full_name], |row| {
            let topics_json: String = row.get(12)?;
            let topics: Vec<String> = serde_json::from_str(&topics_json)
                .unwrap_or_default();

            let license_key: Option<String> = row.get(13)?;
            let license_name: Option<String> = row.get(14)?;
            let license = match (license_key, license_name) {
                (Some(key), Some(name)) => Some(crate::License {
                    key,
                    name,
                    spdx_id: None,
                }),
                _ => None,
            };

            let created_at: String = row.get(8)?;
            let updated_at: String = row.get(9)?;
            let pushed_at: Option<String> = row.get(10)?;

            Ok(RepoMeta {
                full_name: row.get(0)?,
                html_url: row.get(1)?,
                description: row.get(2)?,
                stargazers_count: row.get(3)?,
                forks_count: row.get(4)?,
                open_issues_count: row.get(5)?,
                size: row.get(6)?,
                language: row.get(7)?,
                created_at: chrono::DateTime::parse_from_rfc3339(&created_at)
                    .unwrap()
                    .with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(&updated_at)
                    .unwrap()
                    .with_timezone(&chrono::Utc),
                pushed_at: pushed_at.and_then(|s| {
                    chrono::DateTime::parse_from_rfc3339(&s)
                        .ok()
                        .map(|dt| dt.with_timezone(&chrono::Utc))
                }),
                default_branch: row.get(11)?,
                topics,
                license,
                has_issues: row.get(15)?,
                archived: row.get(16)?,
                disabled: row.get(17)?,
                fork: row.get(18)?,
            })
        });

        match result {
            Ok(repo) => Ok(Some(repo)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    /// Get cached repositories by language
    pub fn get_by_language(&self, language: &str, min_stars: u32, limit: usize) -> Result<Vec<RepoMeta>> {
        let mut stmt = self.conn.prepare(
            "SELECT full_name, html_url, description, stars, forks, open_issues, size,
                    language, created_at, updated_at, pushed_at, default_branch,
                    topics, license_key, license_name, has_issues, archived, disabled, fork
             FROM repos
             WHERE language = ?1 AND stars >= ?2
             ORDER BY stars DESC
             LIMIT ?3"
        )?;

        let repos = stmt.query_map(params![language, min_stars, limit as i64], |row| {
            let topics_json: String = row.get(12)?;
            let topics: Vec<String> = serde_json::from_str(&topics_json)
                .unwrap_or_default();

            let license_key: Option<String> = row.get(13)?;
            let license_name: Option<String> = row.get(14)?;
            let license = match (license_key, license_name) {
                (Some(key), Some(name)) => Some(crate::License {
                    key,
                    name,
                    spdx_id: None,
                }),
                _ => None,
            };

            let created_at: String = row.get(8)?;
            let updated_at: String = row.get(9)?;
            let pushed_at: Option<String> = row.get(10)?;

            Ok(RepoMeta {
                full_name: row.get(0)?,
                html_url: row.get(1)?,
                description: row.get(2)?,
                stargazers_count: row.get(3)?,
                forks_count: row.get(4)?,
                open_issues_count: row.get(5)?,
                size: row.get(6)?,
                language: row.get(7)?,
                created_at: chrono::DateTime::parse_from_rfc3339(&created_at)
                    .unwrap()
                    .with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(&updated_at)
                    .unwrap()
                    .with_timezone(&chrono::Utc),
                pushed_at: pushed_at.and_then(|s| {
                    chrono::DateTime::parse_from_rfc3339(&s)
                        .ok()
                        .map(|dt| dt.with_timezone(&chrono::Utc))
                }),
                default_branch: row.get(11)?,
                topics,
                license,
                has_issues: row.get(15)?,
                archived: row.get(16)?,
                disabled: row.get(17)?,
                fork: row.get(18)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

        Ok(repos)
    }

    /// Clear old cache entries
    pub fn clear_old_entries(&self, days: i64) -> Result<usize> {
        let cutoff = chrono::Utc::now() - chrono::Duration::days(days);
        let cutoff_str = cutoff.to_rfc3339();

        let deleted = self.conn.execute(
            "DELETE FROM repos WHERE cached_at < ?1",
            params![cutoff_str],
        )?;

        info!("Cleared {} old cache entries", deleted);
        Ok(deleted)
    }

    /// Get cache statistics
    pub fn stats(&self) -> Result<CacheStats> {
        let total: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM repos",
            [],
            |row| row.get(0),
        )?;

        let mut stmt = self.conn.prepare(
            "SELECT language, COUNT(*) as count
             FROM repos
             WHERE language IS NOT NULL
             GROUP BY language
             ORDER BY count DESC
             LIMIT 10"
        )?;

        let by_language: Vec<(String, i64)> = stmt.query_map([], |row| {
            Ok((row.get(0)?, row.get(1)?))
        })?
        .collect::<Result<Vec<_>, _>>()?;

        Ok(CacheStats {
            total_repos: total as usize,
            languages: by_language.into_iter()
                .map(|(lang, count)| (lang, count as usize))
                .collect(),
        })
    }
}

#[derive(Debug)]
pub struct CacheStats {
    pub total_repos: usize,
    pub languages: Vec<(String, usize)>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

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
        }
    }

    #[test]
    fn test_cache_store_and_retrieve() -> Result<()> {
        let cache = RepoCache::in_memory()?;
        let repo = create_test_repo();

        cache.store(&repo)?;
        let retrieved = cache.get(&repo.full_name)?;

        assert!(retrieved.is_some());
        let retrieved = retrieved.unwrap();
        assert_eq!(retrieved.full_name, repo.full_name);
        assert_eq!(retrieved.stargazers_count, repo.stargazers_count);

        Ok(())
    }

    #[test]
    fn test_cache_get_nonexistent() -> Result<()> {
        let cache = RepoCache::in_memory()?;
        let result = cache.get("nonexistent/repo")?;
        assert!(result.is_none());
        Ok(())
    }

    #[test]
    fn test_cache_stats() -> Result<()> {
        let cache = RepoCache::in_memory()?;
        let repo = create_test_repo();
        cache.store(&repo)?;

        let stats = cache.stats()?;
        assert_eq!(stats.total_repos, 1);
        assert!(!stats.languages.is_empty());

        Ok(())
    }
}
