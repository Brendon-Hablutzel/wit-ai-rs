//! Includes functionality related to sending speech requests to the wit api

use crate::{client::WitClient, errors::Error, AudioType};
use futures::{Stream, StreamExt};
use reqwest::{
    header::{CONTENT_TYPE, TRANSFER_ENCODING},
    Body,
};
use serde::Deserialize;
use serde_json::Value;
use std::{collections::HashMap, str::from_utf8};

/// A response chunk returned from the speech endpoint
#[derive(Debug)]
pub enum SpeechResponse {
    /// A transcription response, containing just transcribed text
    Transcription(TranscriptionResponse),
    /// A more detailed response about understanding, containing entities, text, and intents
    Understanding(UnderstandingResponse),
}

/// A simple partial transcription response
#[derive(Debug, Deserialize)]
pub struct TranscriptionResponse {
    /// The text detected in the audio
    pub text: String,
}

/// A response containing meaning extracted from some text
#[derive(Debug, Deserialize)]
pub struct UnderstandingResponse {
    /// The text detected in the audio
    pub text: String,
    /// Intents associated with the given text
    pub intents: Vec<UnderstandingIntent>,
    /// Entities found in the given text
    pub entities: HashMap<String, Vec<UnderstandingEntity>>,
    /// Traits associated with the given text
    pub traits: HashMap<String, Vec<UnderstandingTrait>>,
}

/// Information about an intent
#[derive(Debug, Deserialize)]
pub struct UnderstandingIntent {
    /// The intent's id
    pub id: String,
    /// The intent's name
    pub name: String,
    /// The model's confidence in its detection of the intent
    pub confidence: f64,
}

/// Information about an entity
#[derive(Debug, Deserialize)]
pub struct UnderstandingEntity {
    /// The entity's id
    pub id: String,
    /// The entity's name
    pub name: String,
    /// The entity's role
    pub role: String,
    /// The index of the first character of the entity in the ranscribed text
    pub start: u32,
    /// The index of the last character of the entity in the transcribed text
    pub end: u32,
    /// The body of the entity; what was found in the text
    pub body: String,
    /// The model's confidence in its detection of the entity
    pub confidence: f64,
    /// The parsed value of the entity
    pub value: Value, // this might not exist???
    /// Further entities associated with this entity
    pub entities: HashMap<String, Vec<UnderstandingEntity>>,
}

/// Information about a trait
#[derive(Debug, Deserialize)]
pub struct UnderstandingTrait {
    /// The trait's id
    pub id: String,
    /// The value of the trait
    pub value: Value,
    /// The model's confidence in its detection of the trait
    pub confidence: f64,
}

impl WitClient {
    /// Send a request to the speech endpoint, which takes in audio and returns both partial
    /// transcription and meaning extracted from the audio. Here, audio data is the audio data source
    /// (for example, a `tokio::fs::File``), and audio type is the type of audio (ex. mp3 or wav).
    ///
    /// Returns a result of a stream, and each item of this stream is a result where the Ok
    /// variant is an enum SpeechResponse, representing either a partial transcription or
    /// a more detailed understanding response
    ///
    /// Example:
    /// ```rust,no_run
    /// # tokio_test::block_on(async {
    /// # use wit_ai_rs::client::WitClient;
    /// # use wit_ai_rs::errors::Error;
    /// # use wit_ai_rs::common_types::AudioType;
    /// # use wit_ai_rs::speech::SpeechResponse;
    /// # use futures::StreamExt;
    /// # let wit_client = WitClient::new(String::new(), String::new());
    /// async fn process(res: Result<SpeechResponse, Error>) {
    ///     match res.unwrap() {
    ///         SpeechResponse::Transcription(transcription) => println!("transcription: {:?}", transcription),
    ///         SpeechResponse::Understanding(understanding) => println!("understanding: {:?}", understanding)
    ///     }
    /// }
    ///
    /// // Load an audio file
    /// let file = tokio::fs::File::open("test.mp3").await.unwrap();
    ///
    /// // Send the file
    /// let result = wit_client
    ///     .speech(file, AudioType::MP3)
    ///     .await // for sending the file
    ///     .unwrap();
    ///
    /// // process the results, where `process` is some
    /// // async function that handles a `Result<SpeechResponse, Error>
    /// result.for_each(process).await;
    /// # })
    /// ```
    pub async fn speech(
        &self,
        audio_data: impl Into<Body>,
        audio_type: AudioType,
    ) -> Result<impl Stream<Item = Result<SpeechResponse, Error>>, Error> {
        let url = "https://api.wit.ai/speech?v=20230215";

        // internally, when a tokio::fs::File is passed to .body(), it is streamed with ReaderStream
        // and wrap_stream()

        let response = self
            .reqwest_client
            .post(url)
            .bearer_auth(&self.auth_token)
            .header(CONTENT_TYPE, audio_type.to_string())
            .header(TRANSFER_ENCODING, "chunked") // DO I NEED THIS HEADER?
            .body(audio_data)
            .send()
            .await?;

        let stream = response.bytes_stream();

        let mut buffer: Vec<u8> = Vec::new();

        let stream_of_streams = stream.map(move |chunk_bytes| {
            if let Err(err) = chunk_bytes {
                return futures::stream::iter(vec![Err(Error::ResponseParseError(err))]);
            }

            let chunk_data =
                chunk_bytes.expect("chunk_bytes should cause an early return if it is an error");

            buffer.extend_from_slice(&chunk_data);

            let mut speech_objs: Vec<Result<SpeechResponse, Error>> = Vec::new();
            let mut start = 0;

            // every JSON object ends with a carriage return,
            // except for the last one
            let json_obj_separator = b"\r\n";
            let separator_length = json_obj_separator.len();

            let mut parse_chunk = |chunk: &[u8]| {
                if let Ok(json_object) = serde_json::from_slice::<UnderstandingResponse>(chunk) {
                    speech_objs.push(Ok(SpeechResponse::Understanding(json_object)));
                } else {
                    if let Ok(transcription) =
                        serde_json::from_slice::<TranscriptionResponse>(chunk)
                    {
                        speech_objs.push(Ok(SpeechResponse::Transcription(transcription)));
                    } else {
                        if let Ok(response_str) = from_utf8(chunk) {
                            speech_objs.push(Err(Error::JSONParseError(format!(
                                "{response_str} could not be parsed into JSON"
                            ))));
                        } else {
                            speech_objs.push(Err(Error::JSONParseError(
                                "response could not be parsed into utf8".to_string(),
                            )))
                        }
                    }
                }
            };

            while let Some(end) = buffer[start..]
                .windows(separator_length)
                .position(|w| w == json_obj_separator)
            {
                let json_chunk = &buffer[start..start + end + separator_length];
                start += end + separator_length;

                parse_chunk(json_chunk);
            }

            buffer.drain(..start);

            // the very last JSON object does not end with a carriage return
            if buffer.ends_with(b"\n}") {
                parse_chunk(&buffer);
            }

            // return the successfully deserialized JSON objects
            futures::stream::iter(speech_objs)
        });

        let speech = stream_of_streams.flatten();

        Ok(speech)
    }
}

// curl -XPOST 'https://api.wit.ai/speech?v=20230215' \
//     -i -L \
//     -H "Authorization: Bearer OMIPXMA3DNSX33XNOAUT5AJTJFPULXM6" \
//     -H "Content-Type: audio/mpeg" \
//     --data-binary "@test1.mp3"
