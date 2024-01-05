//! Types used throughout the crate

use serde::Deserialize;

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
