use mockito::Matcher;
use wit_ai_rs::{
    client::WitClient,
    entities::{EntityResponse, EntityRole, NewEntityBuilder},
    DeleteResponse, EntityBasic, EntityKeyword,
};

#[tokio::test]
#[ignore]
async fn get_all_entities() {
    let token = std::env::var("WIT_TOKEN").unwrap();

    let client = WitClient::new(String::from(token), String::from("20231231"));

    let _response = client.get_entities().await.unwrap();
}

#[tokio::test]
#[ignore]
async fn create_entity() {
    let token = std::env::var("WIT_TOKEN").unwrap();

    let client = WitClient::new(String::from(token), String::from("20231231"));

    let new_entity = NewEntityBuilder::new(String::from("wit$contact")).build();

    let _response = client.create_entity(new_entity).await.unwrap();
}

#[tokio::test]
#[ignore]
async fn get_one_entity() {
    let token = std::env::var("WIT_TOKEN").unwrap();

    let client = WitClient::new(String::from(token), String::from("20231231"));

    let _response = client
        .get_entity(String::from("wit$quantity"))
        .await
        .unwrap();
}

#[tokio::test]
#[ignore]
async fn update_entity() {
    let token = std::env::var("WIT_TOKEN").unwrap();

    let client = WitClient::new(String::from(token), String::from("20231231"));

    let updated_entity = NewEntityBuilder::new(String::from("Another_Entity_2")).build();

    let _response = client
        .update_entity("another_entity", updated_entity)
        .await
        .unwrap();
}

#[tokio::test]
#[ignore]
async fn delete_entity() {
    let token = std::env::var("WIT_TOKEN").unwrap();

    let client = WitClient::new(String::from(token), String::from("20231231"));

    let _response = client.delete_entity("wit$quantity").await.unwrap();
}

#[tokio::test]
async fn get_all_entities_mock() {
    let mut server = mockito::Server::new();

    let url = server.url();

    let client =
        WitClient::new(String::from("TEST_TOKEN"), String::from("20231231")).set_api_host(url);

    let mock_entities = server
        .mock("GET", "/entities")
        .with_status(200)
        .with_header("Content-Type", "application/json")
        .with_body_from_file("tests/files/entities/get_all.json") // copied from docs
        .match_header("Authorization", "Bearer TEST_TOKEN")
        .match_query(Matcher::UrlEncoded(
            String::from("v"),
            client.get_version().to_owned(),
        ))
        .create();

    let expected_response = vec![
        EntityBasic {
            id: String::from("2690212494559269"),
            name: String::from("car"),
        },
        EntityBasic {
            id: String::from("254954985556896"),
            name: String::from("color"),
        },
        EntityBasic {
            id: String::from("535a8110-2ea7-414f-a024-cf928b076d17"),
            name: String::from("wit$amount_of_money"),
        },
        EntityBasic {
            id: String::from("233273197778131"),
            name: String::from("wit$reminder"),
        },
        EntityBasic {
            id: String::from("1701608719981711"),
            name: String::from("wit$datetime"),
        },
    ];

    let entities_response = client.get_entities().await.unwrap();

    assert_eq!(entities_response, expected_response);

    mock_entities.assert();
}

#[tokio::test]
async fn create_entity_mock() {
    let mut server = mockito::Server::new();

    let url = server.url();

    let client =
        WitClient::new(String::from("TEST_TOKEN"), String::from("20231231")).set_api_host(url);

    let mock_entities = server
        .mock("POST", "/entities")
        .with_status(200)
        .with_header("Content-Type", "application/json")
        .with_body_from_file("tests/files/entities/create.json") // copied from docs
        .match_header("Authorization", "Bearer TEST_TOKEN")
        .match_query(Matcher::UrlEncoded(
            String::from("v"),
            client.get_version().to_owned(),
        ))
        .create();

    // roles field is incorrect in the docs--it is shown to be an array of strings,
    // but is actually an array of role objects
    let expected_response = EntityResponse {
        id: String::from("5418abc7-cc68-4073-ae9e-3a5c3c81d965"),
        name: String::from("favorite_city"),
        roles: vec![EntityRole {
            id: String::from("3920398382332"),
            name: String::from("favorite_city"),
        }],
        lookups: Some(vec![String::from("free-text"), String::from("keywords")]),
        keywords: Some(vec![]),
    };

    let new_entity = NewEntityBuilder::new(String::from("favorite_city")).build();

    let response = client.create_entity(new_entity).await.unwrap();

    assert_eq!(response, expected_response);

    mock_entities.assert();
}

#[tokio::test]
async fn get_one_entity_mock() {
    let mut server = mockito::Server::new();

    let url = server.url();

    let client =
        WitClient::new(String::from("TEST_TOKEN"), String::from("20231231")).set_api_host(url);

    let mock_entities = server
        .mock("GET", "/entities/first_name")
        .with_status(200)
        .with_header("Content-Type", "application/json")
        .with_body_from_file("tests/files/entities/get_one.json") // copied from docs
        .match_header("Authorization", "Bearer TEST_TOKEN")
        .match_query(Matcher::UrlEncoded(
            String::from("v"),
            client.get_version().to_owned(),
        ))
        .create();

    let expected_response = EntityResponse {
        id: String::from("571979db-f6ac-4820-bc28-a1e0787b98fc"),
        name: String::from("first_name"),
        lookups: Some(vec![String::from("keywords"), String::from("free-text")]),
        roles: vec![EntityRole {
            id: String::from("93789208453223"), // docs incorrectly imply that response type for roles is Vec<String>
            name: String::from("first_name"),
        }],
        keywords: Some(vec![
            EntityKeyword {
                keyword: String::from("Willy"),
                synonyms: vec![String::from("Willy")],
            },
            EntityKeyword {
                keyword: String::from("Laurent"),
                synonyms: vec![String::from("Laurent")],
            },
            EntityKeyword {
                keyword: String::from("Julien"),
                synonyms: vec![String::from("Julien")],
            },
            EntityKeyword {
                keyword: String::from("Alex"),
                synonyms: vec![String::from("Alex")],
            },
            EntityKeyword {
                keyword: String::from("Aleka"),
                synonyms: vec![String::from("Aleka")],
            },
            EntityKeyword {
                keyword: String::from("Jason"),
                synonyms: vec![String::from("Jason")],
            },
        ]),
    };

    let response = client.get_entity(String::from("first_name")).await.unwrap();

    assert_eq!(response, expected_response);

    mock_entities.assert();
}

#[tokio::test]
async fn update_entity_mock() {
    let mut server = mockito::Server::new();

    let url = server.url();

    let client =
        WitClient::new(String::from("TEST_TOKEN"), String::from("20231231")).set_api_host(url);

    let mock_entities = server
        .mock("PUT", "/entities/favorite_city")
        .with_status(200)
        .with_header("Content-Type", "application/json")
        .with_body_from_file("tests/files/entities/update.json") // copied from docs
        .match_header("Authorization", "Bearer TEST_TOKEN")
        .match_query(Matcher::UrlEncoded(
            String::from("v"),
            client.get_version().to_owned(),
        ))
        .create();

    // roles field is incorrect in the docs--it is shown to be an array of strings,
    // but is actually an array of role objects
    let expected_response = EntityResponse {
        id: String::from("5418abc7-cc68-4073-ae9e-3a5c3c81d965"),
        name: String::from("Favorite_City"),
        roles: vec![EntityRole {
            id: String::from("3920398382332"),
            name: String::from("Favorite_City"),
        }],
        lookups: Some(vec![String::from("free-text"), String::from("keywords")]),
        keywords: Some(vec![
            EntityKeyword {
                keyword: String::from("Paris"),
                synonyms: vec![
                    String::from("Paris"),
                    String::from("City of Light"),
                    String::from("Capital of France"),
                ],
            },
            EntityKeyword {
                keyword: String::from("Seoul"),
                synonyms: vec![
                    String::from("Seoul"),
                    String::from("서울"),
                    String::from("Kimchi paradise"),
                ],
            },
        ]),
    };

    let updated_entity = NewEntityBuilder::new(String::from("Favorite_City"))
        .keywords(vec![
            EntityKeyword {
                keyword: String::from("Paris"),
                synonyms: vec![
                    String::from("Paris"),
                    String::from("City of Light"),
                    String::from("Capital of France"),
                ],
            },
            EntityKeyword {
                keyword: String::from("Seoul"),
                synonyms: vec![
                    String::from("Seoul"),
                    String::from("서울"),
                    String::from("Kimchi paradise"),
                ],
            },
        ])
        .build();

    let response = client
        .update_entity("favorite_city", updated_entity)
        .await
        .unwrap();

    assert_eq!(response, expected_response);

    mock_entities.assert();
}

#[tokio::test]
async fn delete_entity_mock() {
    let mut server = mockito::Server::new();

    let url = server.url();

    let client =
        WitClient::new(String::from("TEST_TOKEN"), String::from("20231231")).set_api_host(url);

    let mock_entities = server
        .mock("DELETE", "/entities/favorite_city")
        .with_status(200)
        .with_header("Content-Type", "application/json")
        .with_body_from_file("tests/files/entities/delete.json") // copied from docs
        .match_header("Authorization", "Bearer TEST_TOKEN")
        .match_query(Matcher::UrlEncoded(
            String::from("v"),
            client.get_version().to_owned(),
        ))
        .create();

    let expected_response = DeleteResponse {
        deleted: String::from("favorite_city"),
    };

    let response = client.delete_entity("favorite_city").await.unwrap();

    assert_eq!(response, expected_response);

    mock_entities.assert();
}
