//! Includes a method and types related to sending dictation requests to the wit api

use crate::AudioType;
use crate::{client::WitClient, errors::Error};
use futures::{Stream, StreamExt};
use reqwest::header::{CONTENT_TYPE, TRANSFER_ENCODING};
use reqwest::Body;
use serde::Deserialize;
use serde_json;

/// A token (typically a word) returned from the wit api
#[derive(Debug, Deserialize)]
pub struct Token {
    /// Wit's confidence that the token was correctly identified
    pub confidence: f64,
    /// The start of the token in the audio, in milliseconds
    pub start: u64,
    /// The end of the token in the audio, in milliseconds
    pub end: u64,
    /// The token itself, as a string
    pub token: String,
}

/// An object containing details about all the tokens in the speech
#[derive(Debug, Deserialize)]
pub struct Speech {
    /// Wit's confidence in its dictation of the speech
    pub confidence: f64,
    /// The tokens in the dictation
    pub tokens: Vec<Token>,
}

/// The response returned by the dictation endpoint
#[derive(Debug, Deserialize)]
pub struct DictationResponse {
    /// A speech object containing information about tokens
    pub speech: Speech,
    /// The text that wit dictated
    pub text: String,
    /// Whether this chunk is the final chunk (final meaning something like a
    /// complete sentence; wit may contine sending additional chunks)
    pub is_final: Option<bool>,
}

impl WitClient {
    /// Sends a request to the dictation endpoint of wit, which takes in audio and returns
    /// a stream of partial transcriptions. Here, audio data is the audio data source
    /// (for example, a `tokio::fs::File``), and audio type is the type of audio (ex. mp3 or wav).
    ///
    /// Returns a result of a stream, and each item of this stream is a result where the Ok
    /// variant is a single object, a DictationResponse, representing a partial transcription
    ///
    /// Example:
    /// ```rust,no_run
    /// # tokio_test::block_on(async {
    /// # use wit_ai_rs::client::WitClient;
    /// # use wit_ai_rs::errors::Error;
    /// # use wit_ai_rs::common_types::AudioType;
    /// # use wit_ai_rs::dictation::DictationResponse;
    /// # use futures::StreamExt;
    /// # let wit_client = WitClient::new(String::new(), String::new());
    /// async fn process(res: Result<DictationResponse, Error>) {
    ///     println!("{:?}", res);
    /// }
    ///
    /// // Load an audio file
    /// let file = tokio::fs::File::open("test.mp3").await.unwrap();
    ///
    /// // Send the file
    /// let result = wit_client
    ///     .dictation(file, AudioType::MP3)
    ///     .await // for sending the file
    ///     .unwrap();
    ///
    /// // process the results, where `process` is some
    /// // async function that handles a `Result<DictationResponse, Error>
    /// result.for_each(process).await;
    /// # })
    /// ```
    pub async fn dictation(
        &self,
        audio_data: impl Into<Body>,
        audio_type: AudioType,
    ) -> Result<impl Stream<Item = Result<DictationResponse, Error>>, Error> {
        let url = "https://api.wit.ai/dictation?v=20230215";

        // internally, when a tokio::fs::File is passed to .body(), it is streamed with ReaderStream
        // and wrap_stream()

        let stream = self
            .reqwest_client
            .post(url)
            .bearer_auth(&self.auth_token)
            .header(CONTENT_TYPE, audio_type.to_string())
            .header(TRANSFER_ENCODING, "chunked") // DO I NEED THIS HEADER?
            .body(audio_data)
            .send()
            .await?
            .bytes_stream();

        let mut buffer: Vec<u8> = Vec::new();

        let stream_of_streams = stream.map(move |chunk_bytes| {
            if let Err(err) = chunk_bytes {
                return futures::stream::iter(vec![Err(Error::ResponseParseError(err))])
                    .right_stream();
            }

            let chunk_data =
                chunk_bytes.expect("chunk_bytes should cause an early return if it is an error");

            buffer.extend_from_slice(&chunk_data);

            let mut dictations = Vec::new();
            let mut start = 0;

            // every JSON object ends with a carriage return,
            // except for the last one
            let json_obj_separator = b"\r\n";
            let separator_length = json_obj_separator.len();

            while let Some(end) = buffer[start..]
                .windows(separator_length)
                .position(|w| w == json_obj_separator)
            {
                let json_chunk = &buffer[start..start + end + separator_length];
                start += end + separator_length;

                if let Ok(json_object) = serde_json::from_slice::<DictationResponse>(json_chunk) {
                    dictations.push(Ok(json_object));
                }
            }

            buffer.drain(..start);

            // the very last JSON object does not end with a carriage return
            if buffer.ends_with(b"\n}") {
                if let Ok(json_object) = serde_json::from_slice::<DictationResponse>(&buffer) {
                    dictations.push(Ok(json_object));
                }
            }

            // return the successfully deserialized JSON objects
            futures::stream::iter(dictations).left_stream()
        });

        let dictations = stream_of_streams.flatten();

        Ok(dictations)
    }
}
