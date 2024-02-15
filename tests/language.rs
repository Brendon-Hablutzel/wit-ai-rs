use mockito::Matcher;
use wit_ai_rs::{
    client::WitClient,
    language::{LanguageRequest, LanguageResponse, Locale},
};

#[tokio::test]
#[ignore]
async fn language() {
    let token = std::env::var("WIT_TOKEN").unwrap();

    let client = WitClient::new(String::from(token), String::from("20231231"));

    let query = "a test of the language endpoint";

    let language_request = LanguageRequest::new(String::from(query), 2).unwrap();

    let response = client.language(language_request).await.unwrap();

    assert!(response.detected_locales.len() <= 2);
}

#[tokio::test]
async fn language_mock() {
    let mut server = mockito::Server::new();

    let url = server.url();

    let mock_language = server
        .mock("GET", "/language")
        .with_status(200)
        .with_header("Content-Type", "application/json")
        .with_body_from_file("tests/files/language.json") // copied from docs
        .match_header("Authorization", "Bearer TEST_TOKEN")
        .match_query(Matcher::AllOf(vec![
            Matcher::UrlEncoded(String::from("q"), String::from("bonjour les amis")),
            Matcher::UrlEncoded(String::from("n"), String::from("2")),
        ]))
        .create();

    let expected_response = LanguageResponse {
        detected_locales: vec![
            Locale {
                locale: String::from("fr_XX"),
                confidence: 0.9986,
            },
            Locale {
                locale: String::from("ar_AR"),
                confidence: 0.0014,
            },
        ],
    };

    let client =
        WitClient::new(String::from("TEST_TOKEN"), String::from("20231231")).set_api_host(url);

    let query = "bonjour les amis";

    let language_request = LanguageRequest::new(String::from(query), 2).unwrap();

    let response = client.language(language_request).await.unwrap();

    assert_eq!(response, expected_response);

    mock_language.assert();
}

// TODO: test language url params
