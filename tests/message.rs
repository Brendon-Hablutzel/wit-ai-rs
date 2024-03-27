use mockito::Matcher;
use serde_json::Value;
use std::collections::HashMap;
use wit_ai_rs::{
    client::WitClient,
    message::{
        ContextBuilder, Coordinates, IntervalEndpoint, MessageEntity, MessageIntent,
        MessageOptions, MessageOptionsBuilder, MessageResponse,
    },
};

#[tokio::test]
#[ignore]
async fn message() {
    let token = std::env::var("WIT_TOKEN").unwrap();

    let client = WitClient::new(String::from(token), String::from("20231231"));

    let query = "a test query for the message endpoint";

    // it seems that `Context` is not validated by Wit
    let context = ContextBuilder::new()
        .reference_time(String::from("2023-05-01T19:05:00"))
        .timezone(String::from("America/Los_Angeles"))
        .locale(String::from("en_US"))
        .coords(Coordinates::new(37.47104, -122.14703))
        .build();

    let message = MessageOptionsBuilder::new()
        .context(context)
        .limit(1)
        .expect("hardcoded limit should be valid")
        .build();

    let response = client.message(query.to_string(), message).await.unwrap();

    assert!(response.intents.len() <= 1);
}

#[tokio::test]
async fn message_mock() {
    let mut server = mockito::Server::new_async().await;

    let url = server.url();

    let mock_message = server
        .mock("GET", "/message")
        .with_status(200)
        .with_header("Content-Type", "application/json")
        .with_body_from_file("tests/files/message.json") // copied from docs
        .match_header("Authorization", "Bearer TEST_TOKEN")
        .match_query(Matcher::UrlEncoded(
            String::from("q"),
            String::from("how many people between Tuesday and Friday"),
        ))
        .create();

    let client =
        WitClient::new(String::from("TEST_TOKEN"), String::from("20231231")).set_api_host(url);

    let query = "how many people between Tuesday and Friday";

    let response = client
        .message(query.to_string(), MessageOptions::default())
        .await
        .unwrap();

    let mut entities = HashMap::new();

    entities.insert(
        String::from("metric:metric"),
        vec![MessageEntity {
            id: String::from("3701487719281796"),
            name: String::from("metric"),
            role: String::from("metric"),
            start: 9,
            end: 15,
            body: String::from("people"),
            value: Some(serde_json::Value::String(String::from("metric_visitor"))),
            confidence: 0.9231,
            entities: HashMap::new(),
            from: None,
            to: None,
        }],
    );

    entities.insert(
        String::from("wit$datetime:datetime"),
        vec![MessageEntity {
            id: String::from("1701608719981711"),
            name: String::from("wit$datetime"),
            role: String::from("datetime"),
            start: 16,
            end: 42,
            body: String::from("between Tuesday and Friday"),
            value: None,
            confidence: 0.9541,
            entities: HashMap::new(),
            from: Some(IntervalEndpoint {
                unit: None,
                grain: Some(String::from("day")),
                value: Value::String(String::from("2020-05-05T00:00:00.000-07:00")),
            }),
            to: Some(IntervalEndpoint {
                unit: None,
                grain: Some(String::from("day")),
                value: Value::String(String::from("2020-05-09T00:00:00.000-07:00")),
            }),
        }],
    );

    // the docs' example for message doesn't include traits, but
    // testing indicates that a (possibly empty) trait object is always returned
    let traits = HashMap::new();

    let expected_response = MessageResponse {
        text: String::from("how many people between Tuesday and Friday"),
        intents: vec![MessageIntent {
            id: String::from("1701608719981716"),
            name: String::from("inquiry"),
            confidence: 0.8849,
        }],
        entities,
        traits,
    };

    assert_eq!(response, expected_response);

    mock_message.assert();
}

// TODO: test message url params
