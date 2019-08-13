#[deny(missing_docs)]
extern crate base64;
extern crate reqwest;
extern crate serde;
extern crate snafu;

use serde::Serialize;
use snafu::{ResultExt, Snafu};

/// Various types used by trace.moe API.
pub mod types;

pub use types::Doc;

use types::{Me, SearchRequest, SearchResponse};

/// A wrapper around [`reqwest::Client`] to talk to trace.moe API.
pub struct Client {
    /// Defaults to `"https://trace.moe/api"`. Primarily useful for testing.
    pub base_uri: String,
    /// Access token that registered developers have.
    pub token: Option<String>,
    client: reqwest::Client,
}

impl Client {
    /// Create a [`Client`] with default settings.
    pub fn new() -> Self {
        Self {
            base_uri: "https://trace.moe/api".into(),
            client: reqwest::Client::new(),
            token: None,
        }
    }

    /// Create a [`Client`] with an API token.
    pub fn with_token(token: String) -> Self {
        Self {
            token: Some(token),
            ..Self::new()
        }
    }

    /// Search for image.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use moe::Client;
    /// # let client = Client::new();
    /// let image = std::fs::read("image.jpg").unwrap();
    /// let response = client.search(image).unwrap();
    /// assert_eq!("Hataraku Saibou", response.docs[0].title_romaji);
    /// ```
    ///
    /// # Errors
    ///
    /// - HTTP 400 if your search image is empty
    /// - HTTP 403 if are using an invalid token
    /// - HTTP 413 if the image is >1MB
    /// - HTTP 429 if are using requesting too fast
    /// - HTTP 500 or HTTP 503 if something went wrong in backend
    pub fn search(&self, image: Vec<u8>) -> Result<SearchResponse> {
        let body = SearchRequest::new(image);
        let mut response = self.request(reqwest::Method::POST, "search", &body)?;
        match response.status().as_u16() {
            400 => Err(Error::ImageEmpty),
            403 => Err(Error::InvalidToken),
            413 => Err(Error::ImageTooLarge),
            429 => Err(Error::RateLimit {
                message: response.text().context(ResponseEmpty)?,
            }),
            500 | 503 => Err(Error::InternalServerError),
            _ => response.json().context(JsonFailed),
        }
    }

    /// Check the search quota and limit for your account (or IP address).
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use moe::Client;
    /// # let client = Client::new();
    /// let me = client.me().unwrap();
    /// assert_eq!("176.38.191.44", me.email);
    /// ```
    pub fn me(&self) -> Result<Me> {
        self.request(reqwest::Method::GET, "me", "")?
            .json()
            .context(JsonFailed)
    }

    fn request<T>(&self, method: reqwest::Method, path: &str, body: &T) -> Result<reqwest::Response>
    where
        T: Serialize + ?Sized,
    {
        let url = format!("{}/{}", self.base_uri, path);
        let mut request = self.client.request(method, &url);
        if let Some(token) = &self.token {
            request = request.query(&[("token", token)])
        }

        request.json(body).send().context(RequestFailed)
    }
}

/// An error that can happen while processing a request.
#[derive(Debug, Snafu)]
pub enum Error {
    /// The request failed to complete.
    RequestFailed { source: reqwest::Error },
    /// Parsing response failed (e.g. some fields are missing).
    JsonFailed { source: reqwest::Error },
    /// Response is empty when it's not expected.
    ResponseEmpty { source: reqwest::Error },

    /// An empty image was provided.
    ImageEmpty,
    /// Provided token is invalid, expired, etc.
    InvalidToken,
    /// Provided image is >1MB.
    ImageTooLarge,
    /// Limit or quota was reached.
    RateLimit { message: String },
    /// Something went wrong on the trace.moe side of things.
    InternalServerError,
}

/// A convenience type to wrap [`Error`].
pub type Result<T, E = Error> = std::result::Result<T, E>;
