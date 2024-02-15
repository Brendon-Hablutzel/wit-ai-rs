use mockito::Matcher;
use wit_ai_rs::{
    client::WitClient,
    utterances::{
        CreateUtteranceResponse, DeleteUtteranceResponse, GetUtterancesRequestBuilder,
        NewUtterance, NewUtteranceEntity, UtteranceResponse, UtteranceResponseEntity,
        UtteranceResponseTrait,
    },
    IntentBasic,
};

#[tokio::test]
#[ignore]
async fn get_utterances() {
    let token = std::env::var("WIT_TOKEN").unwrap();

    let client = WitClient::new(String::from(token), String::from("20231231"));

    let request = GetUtterancesRequestBuilder::new(1000)
        .unwrap()
        .offset(0)
        .intents(vec![String::from("play"), String::from("pause")])
        .build();

    let _response = client.get_utterances(request).await.unwrap();
}

#[tokio::test]
#[ignore]
async fn create_utterances() {
    let token = std::env::var("WIT_TOKEN").unwrap();

    let client = WitClient::new(String::from(token), String::from("20231231"));

    let new_utterances = vec![NewUtterance::new(
        String::from("make the volume 30"),
        vec![NewUtteranceEntity::new(
            String::from("wit$number:number"),
            16,
            17,
            String::from("30"),
            vec![],
        )],
        vec![],
        Some(String::from("set_volume")),
    )];

    let _response = client.create_utterances(new_utterances).await.unwrap();
}

#[tokio::test]
#[ignore]
async fn delete_utterances() {
    let token = std::env::var("WIT_TOKEN").unwrap();

    let client = WitClient::new(String::from(token), String::from("20231231"));

    let utterances = vec![String::from("make the volume 30")];

    let _response = client.delete_utterances(utterances).await.unwrap();
}

#[tokio::test]
async fn get_utterances_mock() {
    let mut server = mockito::Server::new();

    let url = server.url();

    let client =
        WitClient::new(String::from("TEST_TOKEN"), String::from("20231231")).set_api_host(url);

    let mock_utterances = server
        .mock("GET", "/utterances")
        .with_status(200)
        .with_header("Content-Type", "application/json")
        .with_body_from_file("tests/files/utterances/get_all.json") // copied from docs and modified because
        // docs are incorrect--intent is not a string, it is an object
        .match_header("Authorization", "Bearer TEST_TOKEN")
        .match_query(Matcher::AllOf(vec![
            Matcher::UrlEncoded(String::from("v"), client.get_version().to_owned()),
            Matcher::UrlEncoded(String::from("limit"), 100.to_string()),
        ]))
        .create();

    let expected_response = vec![UtteranceResponse {
        text: String::from("I want to fly SFO"),
        intent: IntentBasic {
            id: String::from("928398303890"),
            name: String::from("flight_request"),
        },
        entities: vec![UtteranceResponseEntity {
            id: String::from("120890890090903"),
            name: String::from("wit$location"),
            role: String::from("destination"),
            start: 17,
            end: 20,
            body: String::from("SFO"),
            entities: vec![],
        }],
        traits: vec![UtteranceResponseTrait {
            id: String::from("198982399822"),
            name: String::from("wit$sentiment"),
            value: String::from("neutral"),
        }],
    }];

    let request = GetUtterancesRequestBuilder::new(100).unwrap().build();

    let response = client.get_utterances(request).await.unwrap();

    assert_eq!(response, expected_response);

    mock_utterances.assert();
}

#[tokio::test]
async fn create_utterances_mock() {
    let mut server = mockito::Server::new();

    let url = server.url();

    let client =
        WitClient::new(String::from("TEST_TOKEN"), String::from("20231231")).set_api_host(url);

    let mock_utterances = server
        .mock("POST", "/utterances")
        .with_status(200)
        .with_header("Content-Type", "application/json")
        .with_body_from_file("tests/files/utterances/create.json") // copied from docs and modified because
        // docs are incorrect--intent is not a string, it is an object
        .match_header("Authorization", "Bearer TEST_TOKEN")
        .match_query(Matcher::UrlEncoded(
            String::from("v"),
            client.get_version().to_owned(),
        ))
        .create();

    let new_utterances = vec![NewUtterance::new(
        String::from("make the volume 30"),
        vec![NewUtteranceEntity::new(
            String::from("wit$number:number"),
            16,
            17,
            String::from("30"),
            vec![],
        )],
        vec![],
        Some(String::from("set_volume")),
    )];

    let response = client.create_utterances(new_utterances).await.unwrap();

    let expected_response = CreateUtteranceResponse { sent: true, n: 1 };

    assert_eq!(response, expected_response);

    mock_utterances.assert();
}

#[tokio::test]
async fn delete_utterances_mock() {
    let mut server = mockito::Server::new();

    let url = server.url();

    let client =
        WitClient::new(String::from("TEST_TOKEN"), String::from("20231231")).set_api_host(url);

    let mock_utterances = server
        .mock("DELETE", "/utterances")
        .with_status(200)
        .with_header("Content-Type", "application/json")
        .with_body_from_file("tests/files/utterances/delete.json") // copied from docs and modified because
        // docs are incorrect--intent is not a string, it is an object
        .match_header("Authorization", "Bearer TEST_TOKEN")
        .match_query(Matcher::UrlEncoded(
            String::from("v"),
            client.get_version().to_owned(),
        ))
        .create();

    let utterances = vec![String::from("utterance1"), String::from("utterances2")];

    let response = client.delete_utterances(utterances).await.unwrap();

    let expected_response = DeleteUtteranceResponse { sent: true, n: 2 };

    assert_eq!(response, expected_response);

    mock_utterances.assert();
}
