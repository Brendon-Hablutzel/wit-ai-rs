//! Interacting with wit utterances

use crate::{client::WitClient, errors::Error, IntentBasic};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

/// A request for getting information about all utterances
#[derive(Debug)]
pub struct GetUtterancesRequest {
    url_params: Vec<(String, String)>,
}

/// Builder for `GetUtterancesRequest`
#[derive(Debug)]
pub struct GetUtterancesRequestBuilder {
    limit: u32,
    offset: Option<u32>,
    intents: Option<Vec<String>>,
}

impl GetUtterancesRequestBuilder {
    /// Creates a new builder for `GetUtterancesRequest`, with the given limit value, which is the
    /// maximum number of utterances to return, between 1 and 10000 inclusive
    pub fn new(limit: u32) -> Result<Self, Error> {
        if limit < 1 || limit > 10000 {
            return Err(Error::InvalidArgument(format!(
                "limit for getting utterances must be between 1 and 10000 inclusive, got {}",
                limit
            )));
        }

        Ok(Self {
            limit,
            offset: None,
            intents: None,
        })
    }

    /// Number of utterances to skip (default is 0)
    pub fn offset(mut self, offset: u32) -> Self {
        self.offset = Some(offset);
        self
    }

    /// A list of intents to filter the utterances
    pub fn intents(mut self, intents: Vec<String>) -> Self {
        self.intents = Some(intents);
        self
    }

    /// Transform the `GetUtterancesBuilder` into a `GetUtterancesRequest`
    pub fn build(self) -> GetUtterancesRequest {
        let mut url_params = Vec::new();

        url_params.push((String::from("limit"), self.limit.to_string()));

        if let Some(offset) = self.offset {
            url_params.push((String::from("offset"), offset.to_string()));
        }

        if let Some(intents) = self.intents {
            url_params.push((String::from("intents"), intents.join(",")))
        }

        GetUtterancesRequest { url_params }
    }
}

/// Struct for associating an entity with a new utterace
#[derive(Debug, Serialize)]
pub struct NewUtteranceEntity {
    entity: String,
    start: u32,
    end: u32,
    body: String,
    entities: Vec<NewUtteranceEntity>,
}

impl NewUtteranceEntity {
    /// Create a `NewUtteranceEntity`, which provides data about an entity associated with an utterance
    /// * `entity` - the name and role of the entity (ex. `entity:role`, or `wit$number:number` for built-in entities)
    /// * `start` - the start index of the entity value in the text of the utterance (inclusive)
    /// * `end` - the end index of the entity value in the text of the utterance (exclusive)
    /// * `body` - the value of the entity as it appears in the text
    /// * `entity` - other entities within this entity
    pub fn new(
        entity: String,
        start: u32,
        end: u32,
        body: String,
        entities: Vec<NewUtteranceEntity>,
    ) -> Self {
        Self {
            entity,
            start,
            end,
            body,
            entities,
        }
    }
}

/// Struct for associating a trait with a new utternace
#[derive(Debug, Serialize)]
pub struct NewUtteranceTrait {
    #[serde(rename = "trait")]
    trait_: String,
    value: String,
}

impl NewUtteranceTrait {
    /// Constructor for `NewUtteranceTrait`
    pub fn new(trait_name: String, value: String) -> Self {
        Self {
            trait_: trait_name,
            value,
        }
    }
}

/// Struct for creating a new utterance
#[derive(Debug, Serialize)]
pub struct NewUtterance {
    text: String,
    entities: Vec<NewUtteranceEntity>,
    traits: Vec<NewUtteranceTrait>,
    intent: Option<String>,
}

impl NewUtterance {
    /// Create a new utterance
    /// * `text` - the text of the utterance
    /// * `entities` - vector of entities associated with the utterance--must be preexisting entities (empty if no entities)
    /// * `traits` - vector of traits associated with the utterance--must be preexisting traits (empty if no traits)
    /// * `intent` - the intent associated with the utterance--None if the intent is out of scope
    pub fn new(
        text: String,
        entities: Vec<NewUtteranceEntity>,
        traits: Vec<NewUtteranceTrait>,
        intent: Option<String>,
    ) -> Self {
        Self {
            text,
            entities,
            traits,
            intent,
        }
    }
}

/// Response to a request to create an utterance
#[derive(Debug, Deserialize, PartialEq)]
pub struct CreateUtteranceResponse {
    /// Whether the request was sent successfully
    pub sent: bool,
    /// The number of utterances created
    pub n: u32,
}

/// Response to a request to delete an utterance
#[derive(Debug, Deserialize, PartialEq)]
pub struct DeleteUtteranceResponse {
    /// Whether the request was sent successfully
    pub sent: bool,
    /// The number of utterances deleted
    pub n: u32,
}

/// Represents data about an utterance returned from the Wit API
#[derive(Debug, Deserialize, PartialEq)]
pub struct UtteranceResponse {
    /// The text of the utterance
    pub text: String,
    /// An intent associated with the utterance
    pub intent: IntentBasic,
    /// Entities associated with the utterance
    pub entities: Vec<UtteranceResponseEntity>,
    /// Traits associated with the utterance
    pub traits: Vec<UtteranceResponseTrait>,
}

/// An entity associated with a returned utterance
#[derive(Debug, Deserialize, PartialEq)]
pub struct UtteranceResponseEntity {
    /// The id of the entity
    pub id: String,
    /// The name of the entity
    pub name: String,
    /// The entity role
    pub role: String,
    /// The start index of the entity in the utterance text
    pub start: u32,
    /// The end index of the entity in the utterance text
    pub end: u32,
    /// The entity as it appears in the utterance
    pub body: String,
    /// Sub-entities associated with the entity
    pub entities: Vec<UtteranceResponseEntity>,
}

/// A trait associated with a returned utterance
#[derive(Debug, Deserialize, PartialEq)]
pub struct UtteranceResponseTrait {
    /// The id of the trait
    pub id: String,
    /// The name of the trait
    pub name: String,
    /// The value of the trait in the utterance
    pub value: String,
}

impl WitClient {
    /// Return information about all utterances associated with the given app
    ///
    /// Example:
    /// ```rust
    /// let request = GetUtterancesRequestBuilder::new(5)
    ///     .unwrap()
    ///     .offset(10)
    ///     .intents(vec!["intent_name".to_string()])
    ///     .build();
    ///
    /// let response = wit_client.get_utterances(request).await.unwrap();
    /// ```
    pub async fn get_utterances(
        &self,
        utterances_request: GetUtterancesRequest,
    ) -> Result<Vec<UtteranceResponse>, Error> {
        let data = self
            .make_request(
                Method::GET,
                "/utterances",
                utterances_request.url_params,
                Option::<Value>::None,
            )
            .await?;

        Ok(data)
    }

    /// Create new utterances for the given app
    ///
    /// Example:
    /// ```rust
    /// let utterance_entity = NewUtteranceEntity::new(
    ///     "entity:entity".to_string(),
    ///     3,
    ///     12,
    ///     "utterance".to_string(),
    ///     vec![],
    /// );
    ///
    /// let utterance_trait = NewUtteranceTrait::new("trait_name".to_string(), "value1".to_string());
    ///
    /// let new_utterance = NewUtterance::new(
    ///     "an utterance".to_string(),
    ///     vec![utterance_entity],
    ///     vec![utterance_trait],
    ///     Some("intent_name".to_string()),
    /// );
    ///
    /// let response: CreateUtteranceResponse = wit_client
    ///     .create_utterances(vec![new_utterance])
    ///     .await
    ///     .unwrap();
    /// ```
    pub async fn create_utterances(
        &self,
        utterances: Vec<NewUtterance>,
    ) -> Result<CreateUtteranceResponse, Error> {
        let data = self
            .make_request(Method::POST, "/utterances", vec![], Some(utterances))
            .await?;

        Ok(data)
    }

    /// Delete utterances
    /// * `utterance_texts` - a vector of strings, where each string is the text of an utterance to delete
    ///
    /// Example:
    /// ```rust
    /// let utterances = vec!["an utterance".to_string()];
    ///
    /// let response = wit_client.delete_utterances(utterances).await.unwrap();
    /// ```
    pub async fn delete_utterances(
        &self,
        utterance_texts: Vec<String>,
    ) -> Result<DeleteUtteranceResponse, Error> {
        // this might be inefficient--we are converting vec to iter to vec
        let utterances: Vec<Value> = utterance_texts
            .into_iter()
            .map(|text| json!({"text": text}))
            .collect();

        let data = self
            .make_request(Method::DELETE, "/utterances", vec![], Some(utterances))
            .await?;

        Ok(data)
    }
}
