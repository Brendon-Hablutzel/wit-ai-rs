//! Interacting with the language endpoint

use crate::{client::WitClient, errors::Error};
use reqwest::Method;
use serde::Deserialize;
use serde_json::Value;

/// A request to send to the language endpoint
#[derive(Debug)]
pub struct LanguageRequest {
    url_params: Vec<(String, String)>,
}

impl LanguageRequest {
    /// Create a language request
    pub fn new(query: String, limit: u16) -> Result<Self, Error> {
        if limit > 8 || limit < 1 {
            return Err(Error::InvalidArgument(format!(
                "limit must be between 1 and 8 inclusive, got {limit}",
            )));
        }

        let mut url_params = Vec::new();

        url_params.push((String::from("q"), query));

        url_params.push((String::from("n"), limit.to_string()));

        Ok(Self { url_params })
    }
}

/// A response from the language endpoint
#[derive(Debug, Deserialize, PartialEq)]
pub struct LanguageResponse {
    /// The locales predicted from the query
    pub detected_locales: Vec<Locale>,
}

/// A locale predicted from the query
#[derive(Debug, Deserialize, PartialEq)]
pub struct Locale {
    /// The locale string
    pub locale: String,
    /// Wit's confidence in the locale
    pub confidence: f64,
}

impl WitClient {
    /// Make a request to the language endpoint
    pub async fn language(&self, request: LanguageRequest) -> Result<LanguageResponse, Error> {
        self.make_request(
            Method::GET,
            "/language",
            request.url_params,
            Option::<Value>::None,
        )
        .await
    }
}
