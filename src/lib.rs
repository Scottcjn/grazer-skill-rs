// SPDX-License-Identifier: MIT
//! # Grazer Skill
//!
//! Rust client for multi-platform AI agent content discovery across
//! BoTTube, Moltbook, 4claw, The Colony, MoltX, MoltExchange, and more.
//!
//! ## Quick Start
//!
//! ```no_run
//! use grazer_skill::GrazerClient;
//!
//! let client = GrazerClient::new();
//!
//! // Discover trending videos
//! let videos = client.discover_bottube(None, None, Some(5)).unwrap();
//! for v in &videos {
//!     println!("{} by {} ({} views)", v.title, v.agent, v.views);
//! }
//!
//! // Browse 4claw boards
//! let boards = client.fourclaw_boards().unwrap();
//! for b in &boards {
//!     println!("/{} — {} ({} threads)", b.slug, b.name, b.thread_count);
//! }
//! ```
//!
//! ## Supported Platforms
//!
//! | Platform | Type | Methods |
//! |----------|------|---------|
//! | BoTTube | Video | `discover_bottube`, `search_bottube`, `bottube_stats` |
//! | Moltbook | Social | `discover_moltbook` |
//! | 4claw | Imageboard | `fourclaw_boards`, `discover_fourclaw`, `fourclaw_thread` |
//! | The Colony | Social | `discover_colony` |
//! | MoltX | Microblog | `discover_moltx`, `discover_moltx_trending` |
//! | MoltExchange | Q&A | `discover_moltexchange` |
//! | ClawCities | Homepages | `discover_clawcities` |
//! | Clawsta | Visual | `discover_clawsta` |

use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

/// Errors returned by the Grazer client.
#[derive(Debug, thiserror::Error)]
pub enum GrazerError {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),
    #[error("API error: {0}")]
    Api(String),
    #[error("JSON parse error: {0}")]
    Json(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, GrazerError>;

// ── Data types ──────────────────────────────────────────────────

/// A video from BoTTube.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BottubeVideo {
    pub id: String,
    pub title: String,
    #[serde(default)]
    pub agent: String,
    #[serde(default)]
    pub category: String,
    #[serde(default)]
    pub views: u64,
    #[serde(default)]
    pub duration: f64,
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub stream_url: String,
}

/// A post from Moltbook.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoltbookPost {
    pub id: u64,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub content: String,
    #[serde(default)]
    pub submolt: String,
    #[serde(default)]
    pub author: String,
    #[serde(default)]
    pub upvotes: i64,
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub url: String,
}

/// A thread from 4claw.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FourclawThread {
    pub id: String,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub content: String,
    #[serde(default, rename = "agentName")]
    pub agent_name: String,
    #[serde(default)]
    pub board: String,
    #[serde(default, rename = "replyCount")]
    pub reply_count: u64,
    #[serde(default)]
    pub created_at: String,
}

/// A 4claw board.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FourclawBoard {
    pub slug: String,
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default, rename = "threadCount")]
    pub thread_count: u64,
}

/// A post from The Colony.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColonyPost {
    pub id: String,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub body: String,
    #[serde(default)]
    pub post_type: String,
    #[serde(default)]
    pub comment_count: u64,
    #[serde(default)]
    pub created_at: String,
}

/// A post from MoltX.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoltXPost {
    pub id: String,
    #[serde(default)]
    pub content: String,
    #[serde(default)]
    pub author_display_name: String,
    #[serde(default)]
    pub like_count: u64,
    #[serde(default)]
    pub reply_count: u64,
    #[serde(default)]
    pub created_at: String,
}

/// A question from MoltExchange.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoltExchangeQuestion {
    pub id: String,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub body: String,
    #[serde(default)]
    pub author: String,
    #[serde(default)]
    pub answer_count: u64,
    #[serde(default)]
    pub created_at: String,
}

/// A site from ClawCities.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClawCitiesSite {
    pub name: String,
    #[serde(default)]
    pub display_name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub guestbook_count: u64,
}

/// A post from Clawsta.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClawstaPost {
    pub id: String,
    #[serde(default)]
    pub content: String,
    #[serde(default)]
    pub author: String,
    #[serde(default)]
    pub created_at: String,
}

/// BoTTube platform statistics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BottubeStats {
    #[serde(default)]
    pub total_videos: u64,
    #[serde(default)]
    pub total_agents: u64,
    #[serde(default)]
    pub total_views: u64,
}

// ── Client ──────────────────────────────────────────────────────

/// Multi-platform content discovery client.
///
/// Discovers content across BoTTube, Moltbook, 4claw, The Colony,
/// MoltX, MoltExchange, ClawCities, and Clawsta.
pub struct GrazerClient {
    http: Client,
}

impl GrazerClient {
    /// Create a new Grazer client with default settings.
    pub fn new() -> Self {
        Self {
            http: Client::builder()
                .user_agent("Grazer/1.9.0 (Rust; Elyan Labs)")
                .timeout(std::time::Duration::from_secs(15))
                .build()
                .unwrap_or_default(),
        }
    }

    // ── BoTTube ─────────────────────────────────────────────────

    /// Discover videos on BoTTube.
    pub fn discover_bottube(
        &self,
        category: Option<&str>,
        agent: Option<&str>,
        limit: Option<u32>,
    ) -> Result<Vec<BottubeVideo>> {
        let mut url = "https://bottube.ai/api/videos?".to_string();
        if let Some(cat) = category {
            url.push_str(&format!("category={cat}&"));
        }
        if let Some(ag) = agent {
            url.push_str(&format!("agent={ag}&"));
        }
        url.push_str(&format!("limit={}", limit.unwrap_or(20)));

        let resp: Vec<BottubeVideo> = self.http.get(&url).send()?.json()?;
        Ok(resp)
    }

    /// Search BoTTube videos by query.
    pub fn search_bottube(&self, query: &str, limit: Option<u32>) -> Result<Vec<BottubeVideo>> {
        let url = format!(
            "https://bottube.ai/api/videos/search?q={}&limit={}",
            query,
            limit.unwrap_or(20)
        );
        let resp: Vec<BottubeVideo> = self.http.get(&url).send()?.json()?;
        Ok(resp)
    }

    /// Get BoTTube platform statistics.
    pub fn bottube_stats(&self) -> Result<BottubeStats> {
        let resp: BottubeStats = self.http.get("https://bottube.ai/api/stats").send()?.json()?;
        Ok(resp)
    }

    // ── Moltbook ────────────────────────────────────────────────

    /// Discover posts on Moltbook.
    pub fn discover_moltbook(
        &self,
        submolt: Option<&str>,
        limit: Option<u32>,
    ) -> Result<Vec<MoltbookPost>> {
        let mut url = "https://www.moltbook.com/api/v1/posts?".to_string();
        if let Some(s) = submolt {
            url.push_str(&format!("submolt={s}&"));
        }
        url.push_str(&format!("limit={}", limit.unwrap_or(20)));

        let resp: Vec<MoltbookPost> = self.http.get(&url).send()?.json()?;
        Ok(resp)
    }

    // ── 4claw ───────────────────────────────────────────────────

    /// List all 4claw boards.
    pub fn fourclaw_boards(&self) -> Result<Vec<FourclawBoard>> {
        let resp: Vec<FourclawBoard> = self
            .http
            .get("https://www.4claw.org/api/v1/boards")
            .send()?
            .json()?;
        Ok(resp)
    }

    /// Discover threads on a 4claw board.
    pub fn discover_fourclaw(
        &self,
        board: Option<&str>,
        limit: Option<u32>,
    ) -> Result<Vec<FourclawThread>> {
        let board = board.unwrap_or("b");
        let url = format!(
            "https://www.4claw.org/api/v1/boards/{board}/threads?limit={}",
            limit.unwrap_or(20).min(20)
        );
        let resp: Vec<FourclawThread> = self.http.get(&url).send()?.json()?;
        Ok(resp)
    }

    /// Get a specific 4claw thread with replies.
    pub fn fourclaw_thread(&self, thread_id: &str) -> Result<serde_json::Value> {
        let url = format!("https://www.4claw.org/api/v1/threads/{thread_id}");
        let resp: serde_json::Value = self.http.get(&url).send()?.json()?;
        Ok(resp)
    }

    // ── The Colony ──────────────────────────────────────────────

    /// Discover posts on The Colony.
    pub fn discover_colony(
        &self,
        colony: Option<&str>,
        limit: Option<u32>,
    ) -> Result<Vec<ColonyPost>> {
        let mut url = "https://thecolony.cc/api/v1/posts?".to_string();
        if let Some(c) = colony {
            url.push_str(&format!("colony={c}&"));
        }
        url.push_str(&format!("limit={}", limit.unwrap_or(20)));

        let resp: Vec<ColonyPost> = self.http.get(&url).send()?.json()?;
        Ok(resp)
    }

    // ── MoltX ───────────────────────────────────────────────────

    /// Discover posts on MoltX.
    pub fn discover_moltx(&self, limit: Option<u32>) -> Result<Vec<MoltXPost>> {
        let url = format!(
            "https://moltx.io/v1/posts?limit={}",
            limit.unwrap_or(20)
        );
        let resp: Vec<MoltXPost> = self.http.get(&url).send()?.json()?;
        Ok(resp)
    }

    /// Discover trending posts on MoltX.
    pub fn discover_moltx_trending(&self, limit: Option<u32>) -> Result<Vec<MoltXPost>> {
        let url = format!(
            "https://moltx.io/v1/posts/trending?limit={}",
            limit.unwrap_or(20)
        );
        let resp: Vec<MoltXPost> = self.http.get(&url).send()?.json()?;
        Ok(resp)
    }

    // ── MoltExchange ────────────────────────────────────────────

    /// Discover questions on MoltExchange.
    pub fn discover_moltexchange(
        &self,
        limit: Option<u32>,
    ) -> Result<Vec<MoltExchangeQuestion>> {
        let url = format!(
            "https://moltexchange.ai/v1/questions?limit={}",
            limit.unwrap_or(20)
        );
        let resp: Vec<MoltExchangeQuestion> = self.http.get(&url).send()?.json()?;
        Ok(resp)
    }

    // ── ClawCities ──────────────────────────────────────────────

    /// Discover sites on ClawCities.
    pub fn discover_clawcities(&self, limit: Option<u32>) -> Result<Vec<ClawCitiesSite>> {
        let url = format!(
            "https://clawcities.com/api/v1/sites?limit={}",
            limit.unwrap_or(20)
        );
        let resp: Vec<ClawCitiesSite> = self.http.get(&url).send()?.json()?;
        Ok(resp)
    }

    // ── Clawsta ─────────────────────────────────────────────────

    /// Discover posts on Clawsta.
    pub fn discover_clawsta(&self, limit: Option<u32>) -> Result<Vec<ClawstaPost>> {
        let url = format!(
            "https://clawsta.io/v1/posts?limit={}",
            limit.unwrap_or(20)
        );
        let resp: Vec<ClawstaPost> = self.http.get(&url).send()?.json()?;
        Ok(resp)
    }
}

impl Default for GrazerClient {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let _client = GrazerClient::new();
    }

    #[test]
    fn test_default_impl() {
        let _client = GrazerClient::default();
    }
}
