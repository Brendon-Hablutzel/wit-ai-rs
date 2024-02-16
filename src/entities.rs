//! Methods and types for interacting with wit entities
//!
//! Includes methods for CRUD operations so that entities can be
//! managed programmatically

use crate::{client::WitClient, errors::Error, DeleteResponse, EntityBasic};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Keywords associated with entities that may be extracted from text
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Keyword {
    /// Canonical value of the entity.
    pub keyword: String,
    /// Ways of expressing, or aliases for this canonical value.
    pub synonyms: Vec<String>,
}

impl Keyword {
    /// Create a new Keyword struct
    pub fn new(keyword: String, synonyms: Vec<String>) -> Self {
        Self { keyword, synonyms }
    }
}

/// A struct to use for creating a new entity
#[derive(Debug, Serialize)]
pub struct NewEntity {
    name: String,
    roles: Vec<String>,
    lookups: Option<Vec<String>>,
    keywords: Option<Vec<Keyword>>,
}

/// Builder for `NewEntity`--use for creating entities
#[derive(Debug)]
pub struct NewEntityBuilder {
    new_entity: NewEntity,
}

impl NewEntityBuilder {
    /// Create a `NewEntityBuilder` with the given name, empty lookups and keywords, and the default role
    /// * `name` - Name for the entity. For built-in entities, use the wit$ prefix.
    pub fn new(name: String) -> Self {
        Self {
            new_entity: NewEntity {
                name: name.clone(),
                roles: vec![name],
                lookups: None,
                keywords: None,
            },
        }
    }

    /// A list of roles to create for the entity
    pub fn roles(mut self, roles: Vec<String>) -> Self {
        self.new_entity.roles = roles;
        self
    }

    /// Set the lookup strategies for a custom entity (free-text, keywords).
    /// Both lookup strategies will be created if this is left empty.
    pub fn lookups(mut self, lookups: Vec<String>) -> Self {
        self.new_entity.lookups = Some(lookups);
        self
    }

    /// Set the keywords associated with this entity
    pub fn keywords(mut self, keywords: Vec<Keyword>) -> Self {
        self.new_entity.keywords = Some(keywords);
        self
    }

    /// Create a `NewEntity` from this `NewEntityBuilder`
    pub fn build(self) -> NewEntity {
        self.new_entity
    }
}

/// A response from creating, updating, or getting an entity
#[derive(Debug, Deserialize, PartialEq)]
pub struct EntityResponse {
    /// The id of the entity
    pub id: String,
    /// The name of the entity
    pub name: String,
    /// Roles of the entity
    pub roles: Vec<EntityRole>,
    /// Lookup strategies for the entity. Does not exist when the entity is built into Wit
    pub lookups: Option<Vec<String>>,
    /// Keywords associated with the entity. Does not exist when the entity is built into Wit
    pub keywords: Option<Vec<Keyword>>,
}

/// A role for an entity
#[derive(Debug, Deserialize, PartialEq)]
pub struct EntityRole {
    /// The id of the role
    pub id: String,
    /// The name of the role
    pub name: String,
}

impl WitClient {
    /// Returns basic information about all entities
    ///
    /// Example:
    /// ```rust,ignore
    /// let response: Vec<EntityBasic> = wit_client.get_entities().await.unwrap();
    /// ```
    pub async fn get_entities(&self) -> Result<Vec<EntityBasic>, Error> {
        self.make_request(Method::GET, "/entities", vec![], Option::<Value>::None)
            .await
    }

    /// Creates a new entity
    ///
    /// Example:
    /// ```rust,ignore
    /// let new_entity = NewEntityBuilder::new("entity_name".to_string())
    ///     .roles(vec!["role".to_string()])
    ///     .build();
    ///
    /// let response: EntityResponse = wit_client.create_entity(new_entity).await.unwrap();
    /// ```
    pub async fn create_entity(&self, new_entity: NewEntity) -> Result<EntityResponse, Error> {
        self.make_request(Method::POST, "/entities", vec![], Some(new_entity))
            .await
    }

    /// Returns information about the entity with the given name
    ///
    /// Example:
    /// ```rust,ignore
    /// let response: EntityResponse = wit_client.get_entity("entity".to_string()).await.unwrap();
    /// ```
    pub async fn get_entity(&self, entity_name: String) -> Result<EntityResponse, Error> {
        let endpoint = format!("/entities/{}", entity_name);

        self.make_request(Method::GET, &endpoint, vec![], Option::<Value>::None)
            .await
    }

    /// Update information about an entity with the current name `old_name`, overwriting its
    /// data with `updated_entity`
    ///
    /// Example:
    /// ```rust,ignore
    /// let updated_entity = NewEntityBuilder::new("updated_name".to_string())
    ///     .roles(vec!["updated_role".to_string()])
    ///     .build();
    ///
    /// let response: EntityResponse = wit_client.update_entity(new_entity).await.unwrap();
    /// ```
    pub async fn update_entity(
        &self,
        old_name: &str,
        updated_entity: NewEntity,
    ) -> Result<EntityResponse, Error> {
        let endpoint = format!("/entities/{}", old_name);

        self.make_request(Method::PUT, &endpoint, vec![], Some(updated_entity))
            .await
    }

    /// Deletes the entity with the given name
    ///
    /// Example:
    /// ```rust,ignore
    /// let response: DeleteResponse = wit_client.delete_entity("entity_name").await.unwrap();
    /// ```
    pub async fn delete_entity(&self, entity_name: &str) -> Result<DeleteResponse, Error> {
        let endpoint = format!("/entities/{}", entity_name);

        self.make_request(Method::DELETE, &endpoint, vec![], Option::<Value>::None)
            .await
    }
}
