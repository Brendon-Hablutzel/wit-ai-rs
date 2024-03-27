# wit_ai_rs
## An unofficial Rust crate for interacting with the wit.ai API
[![crates.io](https://img.shields.io/crates/v/wit_ai_rs.svg)](https://crates.io/crates/wit_ai_rs)
[![Documentation](https://docs.rs/wit_ai_rs/badge.svg)](https://docs.rs/wit_ai_rs)
[![MIT licensed](https://img.shields.io/crates/l/wit_ai_rs.svg)](./LICENSE)

This crate acts as a wrapper for the [wit.ai](https://wit.ai/) HTTP API, containing functions and types for interacting with the API programmatically using Rust. The core type is the `WitClient` struct, which must be initialized before any endpoint can be called.

## Usage

To get started, instantiate a `WitClient`--this has all the methods required for sending requests to the wit API:
```rust
let wit_client = WitClient::new("TOKEN".to_string(), "20240215".to_string());
```

Note that a token is required to interact with the wit API. This can be found under the `Settings` page in the dashboard for your app on the wit site. A token is associated with one app, and the app the token belongs to will be the app that the client acts upon.

## Functionality

This crate currently supports the following endpoints:

### Audio
- `POST /dictation` - takes an audio stream of speech and returns a transcription with text
- `POST /speech` - takes an audio stream of speech and returns transcription as well as extracted meaning

### Entities
- `GET /entities` - fetches all entities associated with the current app
- `POST /entities` - creates a new entity with the given name and roles
- `GET /entities/:entity` - fetches the entity with the given name
- `PUT /entities/:entity` - updates an entity with the given name
- `DELETE /entities/:entity` - deletes the entity with the given name

Wit has built in entities, which are listed [here](https://wit.ai/docs/built-in-entities/)

### Intents
- `GET /intents` - fetches all intents associated with the current app
- `POST /intents` - creates a new intent with the given name
- `GET /intents/:intent` - fetches the intent with the given name
- `DELETE /intents/:intent` - deletes the intent with the given name

Wit has built in intents, which are listed [here](https://wit.ai/docs/built-in-intents/)

### Language Detection
- `GET /language` - attempts to detect the language in a given piece of text

Supported languages are listed [here](https://wit.ai/faq)

### Message
- `GET /message` - analyzes a given piece of text for intent, entities, and traits

### Traits
- `GET /traits` - fetches all traits associated with the current app
- `POST /traits` - creates a new trait with the given name and values
- `GET /traits/:trait` - fetches all information about the trait with the given name
- `DELETE /traits/:trait` - deletes the trait with the given name

Wit has built in intents, which are listed [here](https://wit.ai/docs/built-in-traits/)

### Utterances
- `GET /utterances` - fetches all the utterances associated with the current app
- `POST /utterances` - creates a new utterance with the given text, intent, entities, and traits
- `DELETE /utterances` - deletes one or more utterances, given their text values

## Tests

Some tests use [mockito](https://crates.io/crates/mockito), while others interact with the actual wit.ai API. The tests that interact with the wit API are ignored by default--to run them, you must set the `WIT_TOKEN` environment variable to a token that has read and write access.

## Notes

The latest version of the wit.ai HTTP API docs can be found [here](https://wit.ai/docs/http/)