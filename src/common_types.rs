//! Types used throughout the crate
//!
//! Types specific to each endpoint are stored in the module relating to that endpoint, but
//! here are types that are used in or returned from multiple endpoints.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// The response returned when deleting an object
#[derive(Debug, Deserialize, PartialEq)]
pub struct DeleteResponse {
    /// A string giving details about what was deleted
    pub deleted: String,
}

/// Basic information about a trait
#[derive(Debug, Deserialize, PartialEq)]
pub struct TraitBasic {
    /// The trait id
    pub id: String,
    /// The trait name
    pub name: String,
}

/// Basic information about an intent
#[derive(Debug, Deserialize, PartialEq)]
pub struct IntentBasic {
    /// The intent id
    pub id: String,
    /// The intent name
    pub name: String,
}

/// Basic information about an entity
#[derive(Debug, Deserialize, PartialEq)]
pub struct EntityBasic {
    /// The entity id
    pub id: String,
    /// The entity name
    pub name: String,
}

/// Keywords associated with entities that may be extracted from text
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct EntityKeyword {
    /// Canonical value of the entity.
    pub keyword: String,
    /// Ways of expressing, or aliases for this canonical value.
    pub synonyms: Vec<String>,
}

impl EntityKeyword {
    /// Create a new Keyword struct
    pub fn new(keyword: String, synonyms: Vec<String>) -> Self {
        Self { keyword, synonyms }
    }
}

/// A dynamic entity object
#[derive(Debug)]
pub struct DynamicEntity {
    name: String,
    keywords: Vec<EntityKeyword>,
}

impl DynamicEntity {
    /// Creates a new dynamic entity with the given name and keywords. Note that
    /// dynamic entities can only be used to extend existing keyword entities.
    pub fn new(name: String, keywords: Vec<EntityKeyword>) -> Self {
        Self { name, keywords }
    }
}

/// One or many dynamic entities to be passed with a request
#[derive(Debug, Serialize)]
pub struct DynamicEntities {
    entities: HashMap<String, Vec<EntityKeyword>>,
}

impl DynamicEntities {
    /// Creates a new DynamicEntities object to be included in a request, given
    /// some dynamic entities
    pub fn new(entities: Vec<DynamicEntity>) -> Self {
        let mut entities_map: HashMap<String, Vec<EntityKeyword>> = HashMap::new();

        for entity in entities {
            entities_map.insert(entity.name, entity.keywords);
        }

        Self {
            entities: entities_map,
        }
    }

    pub(crate) fn get_serialized(&self) -> String {
        serde_json::to_string(&self).expect("should be able to serialize DynamicEntities")
    }
}

/// The audio type
pub enum AudioType {
    /// MP3 (files ending in .mp3, for example)
    MP3,
    /// WAV (files ending in .wav, for example)
    /// NOTE: this format is not streamable, which will slow down
    /// dictation speed
    WAV,
}

impl ToString for AudioType {
    fn to_string(&self) -> String {
        String::from(match self {
            Self::MP3 => "audio/mpeg",
            Self::WAV => "audio/wav",
        })
    }
}
