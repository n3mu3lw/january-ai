mod common;
use common::*;

use january_ai::{AutocompleteQuery, HealthierAlternativesRequest, SearchFoodsQuery};
use wiremock::matchers::{header, method, path, query_param};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn test_search_foods_success() {
    let (mock_server, client) = setup_mock_client().await;
    let fixture_body = include_str!("fixtures/search_foods_success.json");

    Mock::given(method("GET"))
        .and(path("/foods"))
        .and(query_param("query", "apple"))
        .and(query_param("limit", "5"))
        .and(header("Authorization", AUTH_HEADER))
        .respond_with(ResponseTemplate::new(200).set_body_raw(fixture_body, "application/json"))
        .expect(1)
        .mount(&mock_server)
        .await;

    let query = SearchFoodsQuery {
        query: "apple".to_string(),
        limit: Some(5),
        ..Default::default()
    };

    let res = client
        .search_foods(&query)
        .await
        .expect("Failed to search foods");

    assert!(!res.items.is_empty());
    assert!(res.items.len() <= 5);
}

#[tokio::test]
async fn test_autocomplete_success() {
    let (mock_server, client) = setup_mock_client().await;
    let fixture_body = include_str!("fixtures/foods_autocomplete_success.json");

    Mock::given(method("GET"))
        .and(path("/foods/autocomplete"))
        .and(query_param("query", "app"))
        .and(query_param("limit", "5"))
        .and(header("Authorization", AUTH_HEADER))
        .respond_with(ResponseTemplate::new(200).set_body_raw(fixture_body, "application/json"))
        .expect(1)
        .mount(&mock_server)
        .await;

    let query = AutocompleteQuery {
        query: "app".to_string(),
        limit: Some(5),
        ..Default::default()
    };

    let res = client
        .autocomplete(&query)
        .await
        .expect("Failed to get autocomplete suggestions");

    assert!(!res.items.is_empty());
    assert!(res.items.len() <= 5);
}

#[tokio::test]
async fn test_get_food_by_barcode_success() {
    let (mock_server, client) = setup_mock_client().await;
    let fixture_body = include_str!("fixtures/get_food_by_barcode_success.json");
    let test_barcode = "49000006346";

    Mock::given(method("GET"))
        .and(path(format!("/foods/barcode/{}", test_barcode)))
        .and(header("Authorization", AUTH_HEADER))
        .respond_with(ResponseTemplate::new(200).set_body_raw(fixture_body, "application/json"))
        .expect(1)
        .mount(&mock_server)
        .await;

    let res = client
        .get_food_by_barcode(test_barcode)
        .await
        .expect("Failed to fetch food by barcode");

    assert_eq!(res.barcode.as_deref(), Some(test_barcode));
}

#[tokio::test]
async fn test_get_food_details_success() {
    let (mock_server, client) = setup_mock_client().await;
    let fixture_body = include_str!("fixtures/get_food_details_success.json");
    let food_id = "101963552";

    Mock::given(method("GET"))
        .and(path(format!("/foods/{}", food_id)))
        .and(header("Authorization", AUTH_HEADER))
        .respond_with(ResponseTemplate::new(200).set_body_raw(fixture_body, "application/json"))
        .expect(1)
        .mount(&mock_server)
        .await;

    let res = client
        .get_food_details(food_id)
        .await
        .expect("Failed to fetch food details");

    assert_eq!(res.id, food_id);
    assert!(!res.name.is_empty());
}

#[tokio::test]
async fn test_get_healthier_alternatives_success() {
    let (mock_server, client) = setup_mock_client().await;
    let fixture_body = include_str!("fixtures/get_healthier_alternatives_success.json");
    let food_id = "70372230";

    Mock::given(method("POST"))
        .and(path(format!("/foods/{}/alternatives", food_id)))
        .and(header("Authorization", AUTH_HEADER))
        .and(header("Content-Type", "application/json"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(fixture_body, "application/json"))
        .expect(1)
        .mount(&mock_server)
        .await;

    let req = HealthierAlternativesRequest {
        diet_restrictions: vec!["gluten".to_string()],
        diet_preferences: vec!["vegetarian".to_string()],
    };

    let res = client
        .get_healthier_alternatives(food_id, &req)
        .await
        .expect("Failed to get healthier alternatives");

    assert!(!res.alternatives.is_empty());
    assert_eq!(res.alternatives.len(), 12);
}
