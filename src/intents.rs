//! Interacting with Wit intents

use crate::{
    client::WitClient,
    common_types::{DeleteResponse, EntityBasic, IntentBasic},
    errors::Error,
};
use reqwest::Method;
use serde::Deserialize;
use serde_json::{json, Value};

/// The response received when fetching an intent
#[derive(Debug, Deserialize, PartialEq)]
pub struct IntentResponse {
    /// The id of the intent
    pub id: String,
    /// The name of the intent
    pub name: String,
    /// Entities associated with the intent
    pub entities: Vec<EntityBasic>,
}

impl WitClient {
    /// Get basic information about all intents associated with an app
    pub async fn get_intents(&self) -> Result<Vec<IntentBasic>, Error> {
        self.make_request(Method::GET, "/intents", vec![], Option::<Value>::None)
            .await
    }

    /// Create a new intent
    pub async fn create_intent(&self, intent_name: &str) -> Result<IntentBasic, Error> {
        let new_intent = json!({"name": intent_name});

        self.make_request(Method::POST, "/intents", vec![], Some(new_intent))
            .await
    }

    /// Get more detailed information about a specific intent
    pub async fn get_intent(&self, intent_name: &str) -> Result<IntentResponse, Error> {
        let endpoint = format!("/intents/{}", intent_name);

        self.make_request(Method::GET, &endpoint, vec![], Option::<Value>::None)
            .await
    }

    /// Delete an intent
    pub async fn delete_intent(&self, intent_name: &str) -> Result<DeleteResponse, Error> {
        let endpoint = format!("/intents/{}", intent_name);

        self.make_request(Method::DELETE, &endpoint, vec![], Option::<Value>::None)
            .await
    }
}