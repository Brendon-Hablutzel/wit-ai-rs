//! Interacting with the message endpoint

use crate::{client::WitClient, errors::Error};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// Context that may be sent with a message
#[derive(Debug)]
pub struct Context {
    // serialized version of ContextBuilder, since Context will be passed as a serialized string in the url params
    serialized: String,
}

/// Builder for Context
#[derive(Debug, Serialize)]
pub struct ContextBuilder {
    reference_time: Option<String>,
    timezone: Option<String>,
    locale: Option<String>,
    coords: Option<Coordinates>,
}

impl ContextBuilder {
    /// Initialize an empty `ContextBuilder`
    pub fn new() -> Self {
        Self {
            reference_time: None,
            timezone: None,
            locale: None,
            coords: None,
        }
    }

    /// Set the reference time local date and time of the user, in ISO8601 format (more specifically, RFC3339).
    /// Do not use UTC time, which would defeat the purpose of this field.
    /// Example: "2014-10-30T12:18:45-07:00"
    pub fn reference_time(mut self, reference_time: String) -> Self {
        self.reference_time = Some(reference_time);
        self
    }

    /// Set the local timezone of the user, which must be a valid IANA timezone.
    /// Used only if no reference_time is provided--we will compute reference_time from timezone and the UTC time of the API server.
    /// If neither reference_time nor timezone are provided, we will use the default timezone of your app, which you can set in 'Settings' in the web console.
    /// Example: "America/Los_Angeles"
    pub fn timezone(mut self, timezone: String) -> Self {
        self.timezone = Some(timezone);
        self
    }

    /// Set the locale of the user: the first 2 letters must be a valid ISO639-1 language, followed by an underscore, followed by a valid ISO3166 alpha2 country code.
    /// Example: "en_GB".
    pub fn locale(mut self, value: String) -> Self {
        self.locale = Some(value);
        self
    }

    /// Set the coordinates of the user: coords is used to improve ranking for wit/location's resolved values.
    /// Example: {"lat": 37.47104, "long": -122.14703}
    pub fn coords(mut self, coords: Coordinates) -> Self {
        self.coords = Some(coords);
        self
    }

    /// Serialize the `ContextBuilder`, turning it into a `Context`
    pub fn build(self) -> Context {
        let serialized =
            serde_json::to_string(&self).expect("should be able to serialize `Context` struct");

        Context { serialized }
    }
}

impl Default for ContextBuilder {
    /// Default constructor for ContextBuilder that initializes all fields to None
    fn default() -> Self {
        Self::new()
    }
}

/// Coordinates for `Context`
#[derive(Debug, Serialize)]
pub struct Coordinates {
    lat: f64,
    long: f64,
}

impl Coordinates {
    /// Create a new Coordinates struct
    pub fn new(latitude: f64, longitude: f64) -> Self {
        Self {
            lat: latitude,
            long: longitude,
        }
    }
}

/// A request to send to the message endpoint
#[derive(Debug)]
pub struct MessageRequest {
    url_params: Vec<(String, String)>,
}

/// Builder for `MessageRequest`
#[derive(Debug)]
pub struct MessageRequestBuilder {
    query: String,
    tag: Option<String>,
    n: Option<u16>,
    context: Option<Context>,
}

impl MessageRequestBuilder {
    /// Initialize a new `MessageRequestBuilder`, with the given query and all other fields empty.
    /// Note: query must be no more than 280 characters.
    pub fn new(query: String) -> Self {
        MessageRequestBuilder {
            query,
            tag: None,
            n: None,
            context: None,
        }
    }

    /// Set the tag for the message request (tag indicates version).
    pub fn tag(mut self, tag: String) -> Self {
        self.tag = Some(tag);
        self
    }

    /// Set the maximum number of n-best intents and traits you want to get back.
    /// The default is 1, and the maximum is 8.
    pub fn limit(mut self, limit: u16) -> Result<Self, Error> {
        if !(1..=8).contains(&limit) {
            return Err(Error::InvalidArgument(format!(
                "limit should be between 1 and 8 inclusive, got {limit}"
            )));
        }

        self.n = Some(limit);
        Ok(self)
    }

    /// Set the context for the message
    pub fn context(mut self, context: Context) -> Self {
        self.context = Some(context);
        self
    }

    /// Turn the `MessageRequestBuilder` into a `MessageRequest`
    pub fn build(self) -> MessageRequest {
        let mut url_params = Vec::new();

        url_params.push((String::from("q"), self.query));

        if let Some(tag) = self.tag {
            url_params.push((String::from("tag"), tag));
        }

        if let Some(n) = self.n {
            url_params.push((String::from("n"), n.to_string()));
        }

        if let Some(context) = self.context {
            url_params.push((String::from("context"), context.serialized));
        }

        MessageRequest { url_params }
    }
}

/// A response from the essage endpoint
#[derive(Debug, Deserialize, PartialEq)]
pub struct MessageResponse {
    /// Either the text sent in the q argument or the transcript of the speech input.
    /// This value should be used only for debug as Wit.ai focuses on entities.
    pub text: String,
    /// Vector of intents sorted by decreasing order of confidence.
    pub intents: Vec<MessageIntent>,
    /// HashMap of entities.
    /// Each entity will contain a vector of values even if there is only one value.
    pub entities: HashMap<String, Vec<MessageEntity>>,
    /// HashMap of traits.
    /// Each trait will contain a vector of values even if there is only one value returned.
    pub traits: HashMap<String, Vec<MessageTrait>>,
}

/// Intents extracted from the message request
#[derive(Debug, Deserialize, PartialEq)]
pub struct MessageIntent {
    /// The id of the intent
    pub id: String,
    /// The name of the intent
    pub name: String,
    /// Wit's confidence in the intent
    pub confidence: f64,
}

/// Entities associated with the message request
#[derive(Debug, Deserialize, PartialEq)]
pub struct MessageEntity {
    /// The entity id
    pub id: String,
    /// The entity name
    pub name: String,
    /// The entity role
    pub role: String,
    /// The start index of the entity in the query text
    pub start: u32,
    /// The end index of the entity in the query text
    pub end: u32,
    /// The entity as it appears in the query
    pub body: String,
    /// Wit's confidence in the entity
    pub confidence: f64,
    /// A HashMap of sub-entities
    pub entities: HashMap<String, MessageEntity>,
    /// The value of the entity (this does not exist when the entity's value is a range)
    pub value: Option<Value>,
    /// The lower end of the range for interval-type values.
    /// This does not exist when the value type is not interval, or when the interval only has an upper bound
    pub from: Option<IntervalEndpoint>,
    /// The upper end of the range for interval-type values.
    /// This does not exist when the value type is not interval, or when the interval only has a lower bound
    pub to: Option<IntervalEndpoint>,
    // a little complicated to implement in tests
    // pub values: Option<Vec<Value>>,
}

/// The data associated with an interval endpoint
#[derive(Debug, Deserialize, PartialEq)]
pub struct IntervalEndpoint {
    /// The value of the unit given
    pub unit: Option<String>,
    /// The level of precision/specificity of the value. Ex. "day"
    pub grain: Option<String>,
    /// The value of the interval endpoint
    pub value: Value,
}

/// A trait determined from the message request
#[derive(Debug, Deserialize, PartialEq)]
pub struct MessageTrait {
    /// The id of the trait
    pub id: String,
    /// The value of the trait
    pub value: Value,
    /// Wit's confidence in the trait
    pub confidence: f64,
}

impl WitClient {
    /// Send a request to wit's /message endpoint, using a request builder `MessageRequestBuilder`.
    /// Information regarding each argument that can be used in `MessageRequestBuilder` can be found
    /// in the documentation for that struct.
    ///
    /// Example:
    /// ```rust,ignore
    /// let request = MessageRequestBuilder::new("Some query sentence".to_string())
    ///     .limit(2)
    ///     .unwrap()
    ///     .build();
    ///
    /// let response: MessageResponse = wit_client.message(request).await.unwrap();
    /// ```
    pub async fn message(&self, request: MessageRequest) -> Result<MessageResponse, Error> {
        self.make_request(
            Method::GET,
            "/message",
            request.url_params,
            Option::<Value>::None,
        )
        .await
    }

    /// Send a request to wit's /message endpoint, using the given query string and
    /// defaults for the other arguments.
    ///
    /// Example:
    /// ```rust,ignore
    /// let response: MessageResponse = wit_client.message_simple("Some query sentence".to_string())
    ///     .await
    ///     .unwrap();
    /// ```
    pub async fn message_simple(&self, query: String) -> Result<MessageResponse, Error> {
        let request = MessageRequestBuilder::new(query).build();
        self.message(request).await
    }
}
