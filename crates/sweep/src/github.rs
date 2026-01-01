use crate::{RepoMeta, ScoredRepo};
use crate::filter::RepoFilter;
use anyhow::{Context, Result};
use futures::stream::{self, StreamExt};
use reqwest::{Client, StatusCode};
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::{mpsc, Semaphore};
use tracing::{debug, info, warn};

/// GitHub API client with rate limiting
pub struct GitHubClient {
    client: Client,
    token: Option<String>,
    rate_limiter: Arc<Semaphore>,
}

impl GitHubClient {
    /// Create a new GitHub client
    pub fn new(token: Option<String>, rate_limit: usize) -> Self {
        let client = Client::builder()
            .user_agent("GNN-Code-Intelligence-Sweep/0.1.0")
            .build()
            .expect("Failed to build HTTP client");

        Self {
            client,
            token,
            rate_limiter: Arc::new(Semaphore::new(rate_limit)),
        }
    }

    /// Search repositories by language
    pub async fn search_repos(
        &self,
        language: &str,
        min_stars: u32,
        max_results: usize,
    ) -> Result<Vec<RepoMeta>> {
        let mut repos = Vec::new();
        let per_page = 100; // GitHub max
        let max_pages = (max_results + per_page - 1) / per_page;

        for page in 1..=max_pages {
            let query = format!("language:{} stars:>={}", language, min_stars);
            let url = format!(
                "https://api.github.com/search/repositories?q={}&sort=stars&order=desc&per_page={}&page={}",
                urlencoding::encode(&query),
                per_page,
                page
            );

            debug!("Fetching page {} of repositories", page);

            let _permit = self.rate_limiter.acquire().await.unwrap();

            let mut request = self.client.get(&url);
            if let Some(token) = &self.token {
                request = request.header("Authorization", format!("Bearer {}", token));
            }

            let response = request
                .send()
                .await
                .context("Failed to send request to GitHub API")?;

            let status = response.status();

            if status == StatusCode::FORBIDDEN || status == StatusCode::TOO_MANY_REQUESTS {
                warn!("Rate limit hit, consider using a GitHub token");
                break;
            }

            if !status.is_success() {
                let error_text = response.text().await?;
                warn!("GitHub API error (status {}): {}", status, error_text);
                break;
            }

            let search_result: SearchResponse = response
                .json()
                .await
                .context("Failed to parse GitHub API response")?;

            if search_result.items.is_empty() {
                debug!("No more repositories found");
                break;
            }

            repos.extend(search_result.items);

            if repos.len() >= max_results {
                repos.truncate(max_results);
                break;
            }

            // Respect GitHub's rate limits
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }

        info!("Fetched {} repositories", repos.len());
        Ok(repos)
    }

    /// Fetch detailed repository information
    pub async fn get_repo_details(&self, owner: &str, repo: &str) -> Result<RepoMeta> {
        let url = format!("https://api.github.com/repos/{}/{}", owner, repo);

        let _permit = self.rate_limiter.acquire().await.unwrap();

        let mut request = self.client.get(&url);
        if let Some(token) = &self.token {
            request = request.header("Authorization", format!("Bearer {}", token));
        }

        let response = request
            .send()
            .await
            .context("Failed to fetch repository details")?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to fetch repo details: {}", response.status());
        }

        let repo_meta: RepoMeta = response
            .json()
            .await
            .context("Failed to parse repository details")?;

        Ok(repo_meta)
    }
}

#[derive(Debug, Deserialize)]
struct SearchResponse {
    items: Vec<RepoMeta>,
    total_count: u32,
}

/// Sweep repositories for a given language with filtering and scoring
pub async fn sweep_language(
    client: &GitHubClient,
    language: &str,
    min_stars: u32,
    max_results: usize,
    filter: RepoFilter,
    tx: mpsc::Sender<ScoredRepo>,
) -> Result<()> {
    info!("Starting sweep for language: {}", language);

    let repos = client
        .search_repos(language, min_stars, max_results)
        .await?;

    info!("Processing {} repositories", repos.len());

    // Process repos in parallel with rate limiting
    let semaphore = Arc::new(Semaphore::new(10));

    stream::iter(repos)
        .for_each_concurrent(10, |repo| {
            let filter = filter.clone();
            let tx = tx.clone();
            let sem = semaphore.clone();

            async move {
                let _permit = sem.acquire().await.unwrap();

                if filter.should_include(&repo) {
                    let score = filter.calculate_score(&repo);
                    let scored = ScoredRepo::new(repo, score);

                    if let Err(e) = tx.send(scored).await {
                        warn!("Failed to send scored repo: {}", e);
                    }
                }
            }
        })
        .await;

    info!("Sweep completed");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = GitHubClient::new(None, 10);
        assert!(client.token.is_none());
    }

    #[test]
    fn test_client_with_token() {
        let client = GitHubClient::new(Some("test_token".to_string()), 10);
        assert!(client.token.is_some());
    }
}
