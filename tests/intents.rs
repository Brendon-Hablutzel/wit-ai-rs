use mockito::Matcher;
use wit_ai_rs::{
    client::WitClientBuilder, intents::IntentResponse, DeleteResponse, EntityBasic, IntentBasic,
};

#[tokio::test]
#[ignore]
async fn get_all_intents() {
    let token = std::env::var("WIT_TOKEN").unwrap();

    let client = WitClientBuilder::new(String::from(token), String::from("20231231")).build();

    let _response = client.get_intents().await.unwrap();
}

#[tokio::test]
#[ignore]
async fn create_intent() {
    let token = std::env::var("WIT_TOKEN").unwrap();

    let client = WitClientBuilder::new(String::from(token), String::from("20231231")).build();

    let new_intent_name = "new_intent";

    let _response = client.create_intent(new_intent_name).await.unwrap();
}

#[tokio::test]
#[ignore]
async fn get_intent() {
    let token = std::env::var("WIT_TOKEN").unwrap();

    let client = WitClientBuilder::new(String::from(token), String::from("20231231")).build();

    let intent_name = "new_intent";

    let _response = client.get_intent(intent_name).await.unwrap();
}

#[tokio::test]
#[ignore]
async fn delete_intent() {
    let token = std::env::var("WIT_TOKEN").unwrap();

    let client = WitClientBuilder::new(String::from(token), String::from("20231231")).build();

    let intent_name = "new_intent";

    let _response = client.delete_intent(intent_name).await.unwrap();
}

#[tokio::test]
async fn get_all_intents_mock() {
    let mut server = mockito::Server::new();
    let url = server.url();

    let client = WitClientBuilder::new(String::from("TEST_TOKEN"), String::from("20231231"))
        .api_host(url)
        .build();

    let mock = server
        .mock("GET", "/intents")
        .with_status(200)
        .with_header("Content-Type", "application/json")
        .with_body_from_file("tests/files/intents/get_all.json") // copied from docs
        .match_header("Authorization", "Bearer TEST_TOKEN")
        .match_query(Matcher::UrlEncoded(
            String::from("v"),
            client.get_version().to_owned(),
        ))
        .create();

    let response = client.get_intents().await.unwrap();

    let expected_response = vec![
        IntentBasic {
            id: String::from("2690212494559269"),
            name: String::from("buy_car"),
        },
        IntentBasic {
            id: String::from("233273197778131"),
            name: String::from("make_call"),
        },
        IntentBasic {
            id: String::from("708611983192814"),
            name: String::from("wit$get_weather"),
        },
        IntentBasic {
            id: String::from("854486315384573"),
            name: String::from("wit$play_music"),
        },
    ];

    assert_eq!(response, expected_response);

    mock.assert();
}

#[tokio::test]
async fn create_intent_mock() {
    let mut server = mockito::Server::new();
    let url = server.url();

    let client = WitClientBuilder::new(String::from("TEST_TOKEN"), String::from("20231231"))
        .api_host(url)
        .build();

    let mock = server
        .mock("POST", "/intents")
        .with_status(200)
        .with_header("Content-Type", "application/json")
        .with_body_from_file("tests/files/intents/create.json") // copied from docs
        .match_header("Authorization", "Bearer TEST_TOKEN")
        .match_query(Matcher::UrlEncoded(
            String::from("v"),
            client.get_version().to_owned(),
        ))
        .create();

    let new_intent_name = "buy_flowers";

    let response = client.create_intent(new_intent_name).await.unwrap();

    let expected_response = IntentBasic {
        id: String::from("13989798788"),
        name: String::from("buy_flowers"),
    };

    assert_eq!(response, expected_response);

    mock.assert();
}

#[tokio::test]
async fn get_intent_mock() {
    let mut server = mockito::Server::new();
    let url = server.url();

    let client = WitClientBuilder::new(String::from("TEST_TOKEN"), String::from("20231231"))
        .api_host(url)
        .build();

    let mock = server
        .mock("GET", "/intents/buy_flowers")
        .with_status(200)
        .with_header("Content-Type", "application/json")
        .with_body_from_file("tests/files/intents/get_one.json") // copied from docs
        .match_header("Authorization", "Bearer TEST_TOKEN")
        .match_query(Matcher::UrlEncoded(
            String::from("v"),
            client.get_version().to_owned(),
        ))
        .create();

    let intent_name = "buy_flowers";

    let response = client.get_intent(intent_name).await.unwrap();

    let expected_response = IntentResponse {
        id: String::from("13989798788"),
        name: String::from("buy_flowers"),
        entities: vec![
            EntityBasic {
                id: String::from("9078938883"),
                name: String::from("flower:flower"),
            },
            EntityBasic {
                id: String::from("11223229984"),
                name: String::from("wit$contact:contact"),
            },
        ],
    };

    assert_eq!(response, expected_response);

    mock.assert();
}

#[tokio::test]
async fn delete_intent_mock() {
    let mut server = mockito::Server::new();
    let url = server.url();

    let client = WitClientBuilder::new(String::from("TEST_TOKEN"), String::from("20231231"))
        .api_host(url)
        .build();

    let mock = server
        .mock("DELETE", "/intents/buy_flowers")
        .with_status(200)
        .with_header("Content-Type", "application/json")
        .with_body_from_file("tests/files/intents/delete.json") // copied from docs
        .match_header("Authorization", "Bearer TEST_TOKEN")
        .match_query(Matcher::UrlEncoded(
            String::from("v"),
            client.get_version().to_owned(),
        ))
        .create();

    let intent_name = "buy_flowers";

    let response = client.delete_intent(intent_name).await.unwrap();

    let expected_response = DeleteResponse {
        deleted: String::from("buy_flowers"),
    };

    assert_eq!(response, expected_response);

    mock.assert();
}
