use serde::{Deserialize, Serialize};

type AnilistID = u32;
type MalID = u32;

/// The body of a search request.
#[derive(Serialize)]
pub struct SearchRequest {
    /// The Base64-encoded image.
    pub image: String,
    /// An optional AniList ID to filter on.
    pub filter: Option<AnilistID>,
}

impl SearchRequest {
    /// Creates a [`SearchRequest`] from raw image bytes.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use moe::types::SearchRequest;
    /// let image = std::fs::read("image.jpg").unwrap();
    /// let _request = SearchRequest::new(image);
    /// ```
    pub fn new(image: Vec<u8>) -> Self {
        Self {
            image: base64::encode(&image),
            filter: None,
        }
    }
}

/// A response from [`Client::search`].
#[derive(Debug, Deserialize)]
pub struct SearchResponse {
    /// Total number of frames searched.
    #[serde(rename = "RawDocsCount")]
    pub raw_docs_count: u32,
    /// Time taken to retrieve the frames from database (sum of all cores).
    #[serde(rename = "RawDocsSearchTime")]
    pub raw_docs_search_time: u64,
    /// Time taken to compare the frames (sum of all cores).
    #[serde(rename = "ReRankSearchTime")]
    pub re_rank_search_time: u64,
    /// Whether the search result is cached.
    ///
    /// Results are cached by extracted image feature.
    #[serde(rename = "CacheHit")]
    pub cache_hit: bool,
    /// Number of times searched.
    pub trial: u32,
    #[serde(flatten)]
    pub limit: Limit,
    #[serde(flatten)]
    pub quota: Quota,
    pub docs: Vec<Doc>,
}

#[derive(Debug, Deserialize)]
pub struct Doc {
    pub from: f64,
    pub to: f64,
    pub at: f64,
    // pub episode: Episode,
    pub similarity: f64,
    pub anilist_id: AnilistID,
    pub mal_id: Option<MalID>,
    pub is_adult: bool,
    pub title_native: Option<String>,
    pub title_chinese: Option<String>,
    pub title_english: Option<String>,
    pub title_romaji: String,
    pub synonyms: Vec<String>,
    pub synonyms_chinese: Vec<String>,
    pub filename: String,
    pub tokenthumb: String,
}

#[derive(Debug, Deserialize)]
pub enum Episode {
    Number(u32),
    OVA,
    Special,
    Other,
}

#[derive(Debug, Deserialize)]
pub struct Me {
    pub user_id: Option<u32>,
    pub email: String,
    #[serde(flatten)]
    pub limit: Limit,
    #[serde(flatten)]
    pub quota: Quota,
    #[serde(flatten)]
    pub user_limit: UserLimit,
    #[serde(flatten)]
    pub user_quota: UserQuota,
}

/// A rate limit. Usually resets every minute.
#[derive(Debug, Deserialize)]
pub struct Limit {
    /// Number of requests remaining in limit period.
    pub limit: u32,
    /// Time until limit resets (in seconds).
    pub limit_ttl: u32,
}

/// Quota on requests. Usually resets every day.
#[derive(Debug, Deserialize)]
pub struct Quota {
    /// Number of requests remaining in quota.
    pub quota: u32,
    /// Time until quota resets (in seconds).
    pub quota_ttl: u32,
}

#[derive(Debug, Deserialize)]
pub struct UserLimit {
    /// Maximum number of requests per limit period.
    pub user_limit: u32,
    /// Time between limit resets (in seconds).
    pub user_limit_ttl: u32,
}

#[derive(Debug, Deserialize)]
pub struct UserQuota {
    /// Maximum number of requests per quota.
    pub user_quota: u32,
    /// Time between quota resets (in seconds).
    pub user_quota_ttl: u32,
}
