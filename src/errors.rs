//! wit_ai_rs crate-related errors

use serde::Deserialize;

/// Errors that may occur while using the wit_ai_rs crate
#[derive(Debug)]
pub enum Error {
    /// An error while sending the HTTP request to Wit
    RequestError(reqwest::Error),
    /// An error parsing the HTTP request body
    ResponseParseError(reqwest::Error),
    /// An invalid argument was passed to a function
    InvalidArgument(String),
    /// The request was sent and the response parsed successfully, but Wit returned an error
    WitError(ErrorResponse),
    /// An error parsing the url (base string + headers)
    URLParseError(url::ParseError),
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        if error.is_body() || error.is_decode() {
            Self::ResponseParseError(error)
        } else {
            Self::RequestError(error)
        }
    }
}

impl From<ErrorResponse> for Error {
    fn from(error_json: ErrorResponse) -> Self {
        Self::WitError(error_json)
    }
}

impl From<url::ParseError> for Error {
    fn from(error: url::ParseError) -> Self {
        Self::URLParseError(error)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RequestError(source) => write!(f, "request error: {}", source),
            Self::ResponseParseError(source) => write!(f, "response parse error: {}", source),
            Self::InvalidArgument(details) => write!(f, "invalid argument: {}", details),
            Self::WitError(source) => write!(f, "error from wit.ai: {}", source),
            Self::URLParseError(source) => write!(f, "URL parse error: {}", source),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::RequestError(source) => Some(source),
            Self::ResponseParseError(source) => Some(source),
            Self::InvalidArgument(_) => None,
            Self::WitError(source) => Some(source),
            Self::URLParseError(source) => Some(source),
        }
    }
}

/// An error returned by the Wit API
#[derive(Debug, Deserialize)]
pub struct ErrorResponse {
    /// Information about the error
    pub error: String,
    /// The error type (not a numeric value)
    pub code: String,
}

impl std::fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.code, self.error)
    }
}

impl std::error::Error for ErrorResponse {}
