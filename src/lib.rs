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
//! ```rust,no_run
//! # tokio_test::block_on(async {
//! # use wit_ai_rs::client::WitClient;
//! # use wit_ai_rs::message::{MessageResponse, MessageOptions};
//! # let wit_client = WitClient::new(String::new(), String::new());
//! let response: MessageResponse = wit_client
//!     .message("Some query sentence".to_string(), MessageOptions::default())
//!     .await
//!     .unwrap();
//! # })
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
pub mod speech;
pub mod traits;
pub mod utterances;

pub use common_types::*;
