use mockito::Matcher;
use wit_ai_rs::{
    client::WitClientBuilder,
    traits::{NewTrait, TraitResponse, TraitValue},
    DeleteResponse, TraitBasic,
};

#[tokio::test]
#[ignore]
async fn get_all_traits() {
    let token = std::env::var("WIT_TOKEN").unwrap();

    let client = WitClientBuilder::new(String::from(token), String::from("20231231")).build();

    let _response = client.get_traits().await.unwrap();
}

#[tokio::test]
#[ignore]
async fn create_trait() {
    let token = std::env::var("WIT_TOKEN").unwrap();

    let client = WitClientBuilder::new(String::from(token), String::from("20231231")).build();

    let new_trait = NewTrait::new(
        String::from("new_trait"),
        vec![String::from("value1"), String::from("value2")],
    );

    let _response = client.create_trait(new_trait).await.unwrap();
}

#[tokio::test]
#[ignore]
async fn get_trait() {
    let token = std::env::var("WIT_TOKEN").unwrap();

    let client = WitClientBuilder::new(String::from(token), String::from("20231231")).build();

    let trait_name = "new_trait";

    let _response = client.get_trait(trait_name).await.unwrap();
}

#[tokio::test]
#[ignore]
async fn delete_trait() {
    let token = std::env::var("WIT_TOKEN").unwrap();

    let client = WitClientBuilder::new(String::from(token), String::from("20231231")).build();

    let trait_name = "new_trait";

    let _response = client.delete_trait(trait_name).await.unwrap();
}

#[tokio::test]
async fn get_all_traits_mock() {
    let mut server = mockito::Server::new();
    let url = server.url();

    let client = WitClientBuilder::new(String::from("TEST_TOKEN"), String::from("20231231"))
        .api_host(url)
        .build();

    let mock = server
        .mock("GET", "/traits")
        .with_status(200)
        .with_header("Content-Type", "application/json")
        .with_body_from_file("tests/files/traits/get_all.json") // copied from docs
        .match_header("Authorization", "Bearer TEST_TOKEN")
        .match_query(Matcher::UrlEncoded(
            String::from("v"),
            client.get_version().to_owned(),
        ))
        .create();

    let response = client.get_traits().await.unwrap();

    let expected_response = vec![
        TraitBasic {
            id: String::from("2690212494559269"),
            name: String::from("wit$sentiment"),
        },
        TraitBasic {
            id: String::from("254954985556896"),
            name: String::from("faq"),
        },
        TraitBasic {
            id: String::from("233273197778131"),
            name: String::from("politeness"),
        },
    ];

    assert_eq!(response, expected_response);

    mock.assert();
}

#[tokio::test]
async fn create_trait_mock() {
    let mut server = mockito::Server::new();
    let url = server.url();

    let client = WitClientBuilder::new(String::from("TEST_TOKEN"), String::from("20231231"))
        .api_host(url)
        .build();

    let mock = server
        .mock("POST", "/traits")
        .with_status(200)
        .with_header("Content-Type", "application/json")
        .with_body_from_file("tests/files/traits/create.json") // copied from docs
        .match_header("Authorization", "Bearer TEST_TOKEN")
        .match_query(Matcher::UrlEncoded(
            String::from("v"),
            client.get_version().to_owned(),
        ))
        .create();

    let new_trait = NewTrait::new(
        String::from("politeness"),
        vec![String::from("polite"), String::from("rude")],
    );

    let response = client.create_trait(new_trait).await.unwrap();

    let expected_response = TraitResponse {
        id: String::from("13989798788"),
        name: String::from("politeness"),
        values: vec![
            TraitValue {
                id: String::from("97873388"),
                value: String::from("polite"),
            },
            TraitValue {
                id: String::from("54493392772"),
                value: String::from("rude"),
            },
        ],
    };

    assert_eq!(response, expected_response);

    mock.assert();
}

#[tokio::test]
async fn get_trait_mock() {
    let mut server = mockito::Server::new();
    let url = server.url();

    let client = WitClientBuilder::new(String::from("TEST_TOKEN"), String::from("20231231"))
        .api_host(url)
        .build();

    let mock = server
        .mock("GET", "/traits/politeness")
        .with_status(200)
        .with_header("Content-Type", "application/json")
        .with_body_from_file("tests/files/traits/get_one.json") // copied from docs
        .match_header("Authorization", "Bearer TEST_TOKEN")
        .match_query(Matcher::UrlEncoded(
            String::from("v"),
            client.get_version().to_owned(),
        ))
        .create();

    let trait_name = "politeness";

    let response = client.get_trait(trait_name).await.unwrap();

    let expected_response = TraitResponse {
        id: String::from("13989798788"),
        name: String::from("politeness"),
        values: vec![
            TraitValue {
                id: String::from("97873388"),
                value: String::from("polite"),
            },
            TraitValue {
                id: String::from("54493392772"),
                value: String::from("rude"),
            },
        ],
    };

    assert_eq!(response, expected_response);

    mock.assert();
}

#[tokio::test]
async fn delete_trait_mock() {
    let mut server = mockito::Server::new();
    let url = server.url();

    let client = WitClientBuilder::new(String::from("TEST_TOKEN"), String::from("20231231"))
        .api_host(url)
        .build();

    let mock = server
        .mock("DELETE", "/traits/politeness")
        .with_status(200)
        .with_header("Content-Type", "application/json")
        .with_body_from_file("tests/files/traits/delete.json") // copied from docs
        .match_header("Authorization", "Bearer TEST_TOKEN")
        .match_query(Matcher::UrlEncoded(
            String::from("v"),
            client.get_version().to_owned(),
        ))
        .create();

    let trait_name = "politeness";

    let response = client.delete_trait(trait_name).await.unwrap();

    let expected_response = DeleteResponse {
        deleted: String::from("politeness"),
    };

    assert_eq!(response, expected_response);

    mock.assert();
}
