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
use serde::{de::DeserializeOwned, Deserialize, Serialize};

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

    fn get_json<T: DeserializeOwned>(&self, url: &str) -> Result<T> {
        Ok(self.http.get(url).send()?.error_for_status()?.json()?)
    }

    /// Same contract as `get_json`, but lets reqwest build the query string
    /// so every value gets percent-encoded. Hand-built `format!("...{v}...")`
    /// query strings break on any value containing a reserved character
    /// (`&`, `#`, `+`, `=`, space): the embedded delimiter gets read as a
    /// parameter separator by the server instead of as data.
    fn get_json_query<T: DeserializeOwned>(&self, url: &str, params: &[(&str, String)]) -> Result<T> {
        Ok(self
            .http
            .get(url)
            .query(params)
            .send()?
            .error_for_status()?
            .json()?)
    }

    // ── BoTTube ─────────────────────────────────────────────────

    /// Discover videos on BoTTube.
    pub fn discover_bottube(
        &self,
        category: Option<&str>,
        agent: Option<&str>,
        limit: Option<u32>,
    ) -> Result<Vec<BottubeVideo>> {
        let mut params: Vec<(&str, String)> = Vec::new();
        if let Some(cat) = category {
            params.push(("category", cat.to_string()));
        }
        if let Some(ag) = agent {
            params.push(("agent", ag.to_string()));
        }
        params.push(("limit", limit.unwrap_or(20).to_string()));

        let resp: Vec<BottubeVideo> =
            self.get_json_query("https://bottube.ai/api/videos", &params)?;
        Ok(resp)
    }

    /// Search BoTTube videos by query.
    pub fn search_bottube(&self, query: &str, limit: Option<u32>) -> Result<Vec<BottubeVideo>> {
        let params = [
            ("q", query.to_string()),
            ("limit", limit.unwrap_or(20).to_string()),
        ];
        let resp: Vec<BottubeVideo> =
            self.get_json_query("https://bottube.ai/api/videos/search", &params)?;
        Ok(resp)
    }

    /// Get BoTTube platform statistics.
    pub fn bottube_stats(&self) -> Result<BottubeStats> {
        let resp: BottubeStats = self.get_json("https://bottube.ai/api/stats")?;
        Ok(resp)
    }

    // ── Moltbook ────────────────────────────────────────────────

    /// Discover posts on Moltbook.
    pub fn discover_moltbook(
        &self,
        submolt: Option<&str>,
        limit: Option<u32>,
    ) -> Result<Vec<MoltbookPost>> {
        let mut params: Vec<(&str, String)> = Vec::new();
        if let Some(s) = submolt {
            params.push(("submolt", s.to_string()));
        }
        params.push(("limit", limit.unwrap_or(20).to_string()));

        let resp: Vec<MoltbookPost> =
            self.get_json_query("https://www.moltbook.com/api/v1/posts", &params)?;
        Ok(resp)
    }

    // ── 4claw ───────────────────────────────────────────────────

    /// List all 4claw boards.
    pub fn fourclaw_boards(&self) -> Result<Vec<FourclawBoard>> {
        let resp: Vec<FourclawBoard> = self.get_json("https://www.4claw.org/api/v1/boards")?;
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
        let resp: Vec<FourclawThread> = self.get_json(&url)?;
        Ok(resp)
    }

    /// Get a specific 4claw thread with replies.
    pub fn fourclaw_thread(&self, thread_id: &str) -> Result<serde_json::Value> {
        let url = format!("https://www.4claw.org/api/v1/threads/{thread_id}");
        let resp: serde_json::Value = self.get_json(&url)?;
        Ok(resp)
    }

    // ── The Colony ──────────────────────────────────────────────

    /// Discover posts on The Colony.
    pub fn discover_colony(
        &self,
        colony: Option<&str>,
        limit: Option<u32>,
    ) -> Result<Vec<ColonyPost>> {
        let mut params: Vec<(&str, String)> = Vec::new();
        if let Some(c) = colony {
            params.push(("colony", c.to_string()));
        }
        params.push(("limit", limit.unwrap_or(20).to_string()));

        let resp: Vec<ColonyPost> =
            self.get_json_query("https://thecolony.cc/api/v1/posts", &params)?;
        Ok(resp)
    }

    // ── MoltX ───────────────────────────────────────────────────

    /// Discover posts on MoltX.
    pub fn discover_moltx(&self, limit: Option<u32>) -> Result<Vec<MoltXPost>> {
        let url = format!(
            "https://moltx.io/v1/posts?limit={}",
            limit.unwrap_or(20)
        );
        let resp: Vec<MoltXPost> = self.get_json(&url)?;
        Ok(resp)
    }

    /// Discover trending posts on MoltX.
    pub fn discover_moltx_trending(&self, limit: Option<u32>) -> Result<Vec<MoltXPost>> {
        let url = format!(
            "https://moltx.io/v1/posts/trending?limit={}",
            limit.unwrap_or(20)
        );
        let resp: Vec<MoltXPost> = self.get_json(&url)?;
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
        let resp: Vec<MoltExchangeQuestion> = self.get_json(&url)?;
        Ok(resp)
    }

    // ── ClawCities ──────────────────────────────────────────────

    /// Discover sites on ClawCities.
    pub fn discover_clawcities(&self, limit: Option<u32>) -> Result<Vec<ClawCitiesSite>> {
        let url = format!(
            "https://clawcities.com/api/v1/sites?limit={}",
            limit.unwrap_or(20)
        );
        let resp: Vec<ClawCitiesSite> = self.get_json(&url)?;
        Ok(resp)
    }

    // ── Clawsta ─────────────────────────────────────────────────

    /// Discover posts on Clawsta.
    pub fn discover_clawsta(&self, limit: Option<u32>) -> Result<Vec<ClawstaPost>> {
        let url = format!(
            "https://clawsta.io/v1/posts?limit={}",
            limit.unwrap_or(20)
        );
        let resp: Vec<ClawstaPost> = self.get_json(&url)?;
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

    fn assert_http_status<T: std::fmt::Debug>(result: Result<T>, expected: reqwest::StatusCode) {
        match result {
            Err(GrazerError::Http(error)) => {
                assert_eq!(
                    error.status(),
                    Some(expected),
                    "unexpected error: {error:?}"
                );
            }
            other => panic!("expected HTTP {expected} error, got: {other:?}"),
        }
    }

    #[test]
    fn get_json_preserves_404_status_before_decoding() {
        let mut server = mockito::Server::new();
        let request = server
            .mock("GET", "/missing")
            .with_status(404)
            .with_header("content-type", "application/json")
            .with_body(r#"{"error":"not found"}"#)
            .create();
        let client = GrazerClient::new();

        let result: Result<serde_json::Value> =
            client.get_json(&format!("{}/missing", server.url()));

        request.assert();
        assert_http_status(result, reqwest::StatusCode::NOT_FOUND);
    }

    #[test]
    fn get_json_preserves_500_status_before_malformed_json() {
        let mut server = mockito::Server::new();
        let request = server
            .mock("GET", "/broken")
            .with_status(500)
            .with_body("not json")
            .create();
        let client = GrazerClient::new();

        let result: Result<serde_json::Value> =
            client.get_json(&format!("{}/broken", server.url()));

        request.assert();
        assert_http_status(result, reqwest::StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[test]
    fn get_json_decodes_successful_json() {
        let mut server = mockito::Server::new();
        let request = server
            .mock("GET", "/ok")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"ok":true}"#)
            .create();
        let client = GrazerClient::new();

        let value: serde_json::Value = client
            .get_json(&format!("{}/ok", server.url()))
            .expect("successful JSON response should decode");

        request.assert();
        assert_eq!(value, serde_json::json!({"ok": true}));
    }

    #[test]
    fn get_json_query_percent_encodes_reserved_characters() {
        // A value containing '&' built into a hand-written format!() query
        // string ("q={query}&limit=20") would be read by the server as a
        // second parameter, truncating the real query. reqwest's .query()
        // must percent-encode it so the server sees the literal value.
        let mut server = mockito::Server::new();
        let request = server
            .mock("GET", "/search")
            .match_query(mockito::Matcher::UrlEncoded(
                "q".into(),
                "rust & go".into(),
            ))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"ok":true}"#)
            .create();
        let client = GrazerClient::new();

        let params = [("q", "rust & go".to_string())];
        let value: serde_json::Value = client
            .get_json_query(&format!("{}/search", server.url()), &params)
            .expect("percent-encoded query should reach the mock");

        request.assert();
        assert_eq!(value, serde_json::json!({"ok": true}));
    }

    #[test]
    fn get_json_query_preserves_error_status_before_decoding() {
        // Same error_for_status()-before-json() contract as get_json above,
        // just through the query-building path -- the percent-encoding fix
        // must not reintroduce the "decode the error body as the success
        // type" bug that get_json (#17) already closed.
        let mut server = mockito::Server::new();
        let request = server
            .mock("GET", "/search")
            .match_query(mockito::Matcher::Any)
            .with_status(404)
            .with_header("content-type", "application/json")
            .with_body(r#"{"error":"not found"}"#)
            .create();
        let client = GrazerClient::new();

        let params = [("q", "anything".to_string())];
        let result: Result<serde_json::Value> =
            client.get_json_query(&format!("{}/search", server.url()), &params);

        request.assert();
        assert_http_status(result, reqwest::StatusCode::NOT_FOUND);
    }
}
