//! Contains a client struct for interacting with the wit.ai API

use crate::errors::{Error, ErrorResponse};
use reqwest::{header::ACCEPT, Method, StatusCode};
use serde::{de::DeserializeOwned, Serialize};

const DEFAULT_API_HOST: &str = "https://api.wit.ai";

/// The main struct for interacting with the Wit API
#[derive(Debug, Clone)]
pub struct WitClient {
    pub(crate) api_host: String,
    version: String,
    pub(crate) auth_token: String,
    // reqwest stores the client in an `Arc` internally, so it can be safely cloned
    pub(crate) reqwest_client: reqwest::Client,
}

impl WitClient {
    /// Create a new WitClient with the given `auth_token` and `version` and the default
    /// API host. `version` is a date string of the form yyyymmdd (ex. 20231231)
    pub fn new(auth_token: String, version: String) -> Self {
        let api_host = String::from(DEFAULT_API_HOST);

        let reqwest_client = reqwest::Client::new();

        Self {
            api_host,
            version,
            auth_token,
            reqwest_client,
        }
    }

    /// Changes the API host--only recommended for use while testing
    pub fn set_api_host(self, api_host: String) -> Self {
        Self {
            api_host,
            auth_token: self.auth_token,
            version: self.version,
            reqwest_client: self.reqwest_client.clone(),
        }
    }

    pub(crate) async fn make_request<T: DeserializeOwned>(
        &self,
        method: Method,
        endpoint: &str,
        url_params: Vec<(String, String)>,
        body: Option<impl Serialize>,
    ) -> Result<T, Error> {
        let url = format!("{}{endpoint}?v={}", self.api_host, self.version);

        let mut request = match method {
            Method::GET => self.reqwest_client.get(url),
            Method::POST => self.reqwest_client.post(url),
            Method::DELETE => self.reqwest_client.delete(url),
            Method::PUT => self.reqwest_client.put(url),
            _ => panic!("invalid method passed to internal `make_request` method"),
        };

        request = request.query(&url_params);

        request = match body {
            // .json() internally sets the content type header to application/json
            Some(body) => request.json(&body),
            None => request,
        };

        let response = request
            .bearer_auth(&self.auth_token)
            .header(ACCEPT, format!("application/vnd.wit.{}+json", self.version))
            .send()
            .await?;

        let data = match response.status() {
            StatusCode::OK => Ok(response.json::<T>().await?),
            _ => Err(response.json::<ErrorResponse>().await?),
        }?;

        Ok(data)
    }

    /// Getter for `WitClient` version
    pub fn get_version(&self) -> &str {
        &self.version
    }
}
