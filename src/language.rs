//! Interacting with the language identification endpoint

use crate::{client::WitClient, errors::Error};
use reqwest::Method;
use serde::Deserialize;
use serde_json::Value;

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
    ///
    /// Example:
    /// ```rust,no_run
    /// # tokio_test::block_on(async {
    /// # use wit_ai_rs::client::WitClient;
    /// # use wit_ai_rs::language::LanguageResponse;
    /// # let wit_client = WitClient::new(String::new(), String::new());
    /// let response: LanguageResponse = wit_client.language("some query sentence".to_string(), 1)
    ///     .await
    ///     .unwrap();
    /// # })
    /// ```
    pub async fn language(&self, query: String, limit: u16) -> Result<LanguageResponse, Error> {
        if !(1..=8).contains(&limit) {
            return Err(Error::InvalidArgument(format!(
                "limit must be between 1 and 8 inclusive, got {limit}",
            )));
        }

        let mut url_params = Vec::new();

        url_params.push((String::from("q"), query));

        url_params.push((String::from("n"), limit.to_string()));

        self.make_request(Method::GET, "/language", url_params, Option::<Value>::None)
            .await
    }
}
