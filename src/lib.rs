//! # wit_ai_rs
//!
//! A crate for interacting with the wit.ai API
//!
//! The following code shows an example how to get started with this crate
//! by instantiating a WitClient:
//! ```rust
//! # use wit_ai_rs::client::WitClient;
//! let wit_client = WitClient::new("TOKEN".to_string(), "20240215".to_string());
//! ```
//!
//! Specific endpoints can be called using various methods of the WitClient struct, for
//! example the message endpoint can be called as follows:
//! ```rust,ignore
//! let response = wit_client.message_simple("Some query sentence".to_string()).await.unwrap();
//! ```
//! Examples for most methods can be found in their respective modules. For each of these examples,
//! assume that `wit_client` is a valid WitClient.

#![warn(missing_docs)]

pub mod client;
pub mod common_types;
pub mod dictation;
pub mod entities;
pub mod errors;
pub mod intents;
pub mod language;
pub mod message;
pub mod traits;
pub mod utterances;

pub use common_types::*;
