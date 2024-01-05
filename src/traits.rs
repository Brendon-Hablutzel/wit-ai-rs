//! Interacting with Wit traits

use crate::{
    client::WitClient,
    common_types::{DeleteResponse, TraitBasic},
    errors::Error,
};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Struct to use for creating a new trait
#[derive(Debug, Serialize)]
pub struct NewTrait {
    name: String,
    values: Vec<String>,
}

impl NewTrait {
    /// Constructor for `NewTrait`
    pub fn new(name: String, values: Vec<String>) -> Self {
        Self { name, values }
    }
}

/// A trait object returned from the Wit API
#[derive(Debug, Deserialize, PartialEq)]
pub struct TraitResponse {
    /// The id of the trait
    pub id: String,
    /// The name of the trait
    pub name: String,
    /// Values that the trait may take on
    pub values: Vec<TraitValue>,
}

/// A trait value
#[derive(Debug, Deserialize, PartialEq)]
pub struct TraitValue {
    /// The id of the value
    pub id: String,
    /// The value itself
    pub value: String,
}

impl WitClient {
    /// Get all the traits associated with an app
    pub async fn get_traits(&self) -> Result<Vec<TraitBasic>, Error> {
        let data = self
            .make_request(Method::GET, "/traits", vec![], Option::<Value>::None)
            .await?;

        Ok(data)
    }

    /// Create a new trait
    pub async fn create_trait(&self, new_trait: NewTrait) -> Result<TraitResponse, Error> {
        let data = self
            .make_request(Method::POST, "/traits", vec![], Some(new_trait))
            .await?;

        Ok(data)
    }

    /// Get information about a given trait
    pub async fn get_trait(&self, trait_name: &str) -> Result<TraitResponse, Error> {
        let endpoint = format!("/traits/{trait_name}");

        let data = self
            .make_request(Method::GET, &endpoint, vec![], Option::<Value>::None)
            .await?;

        Ok(data)
    }

    /// Delete a given trait
    pub async fn delete_trait(&self, trait_name: &str) -> Result<DeleteResponse, Error> {
        let endpoint = format!("/traits/{trait_name}");

        let data = self
            .make_request(Method::DELETE, &endpoint, vec![], Option::<Value>::None)
            .await?;

        Ok(data)
    }
}
