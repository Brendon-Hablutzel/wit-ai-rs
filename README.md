# An unofficial Rust crate for interacting with the wit.ai API

It acts as a wrapper for the [wit.ai](https://wit.ai/) HTTP api, containing functions and types for interacting with the API programmatically. The core type is the `WitClient` struct, which must be initialized before any endpoint can be called.

## Usage

To get started, instantiate a `WitClient`--this has all the methods required for sending requests to the wit API:
```rust
let wit_client = WitClient::new("TOKEN".to_string(), "20240215".to_string());
```

## Functionality

This crate currently supports the following endpoints:

### Audio
- `POST /dictation` - takes an audio stream of speech and returns a transcription with text

### Entities
- `GET /entities`
- `POST /entities`
- `GET /entities/:entity`
- `PUT /entities/:entity`
- `DELETE /entities/:entity`

### Intents
- `GET /intents`
- `POST /intents`
- `GET /intents/:intent`
- `DELETE /intents/:intent`

### Language Detection
- `GET /language`

### Message
- `GET /message`

### Traits
- `GET /traits`
- `POST /traits`
- `GET /traits/:trait`
- `DELETE /traits/:trait`

### Utterances
- `GET /utterances`
- `POST /utterances`
- `DELETE /utterances`

## Auth

Note that a token is required to interact with the wit API. This can be found under the `Settings` page in the dashboard for your app on the wit site.

## Tests

Some tests use [mockito](https://crates.io/crates/mockito), while others interact with the actual wit.ai API. The tests that interact with the wit API are ignored by default--to run them, you must set the `WIT_TOKEN` environment variable to a token that has read and write access.