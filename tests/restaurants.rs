mod common;
use common::*;

use january_ai::{MenuItemSearchQuery, RestaurantMenuItemsQuery, RestaurantSearchQuery};
use wiremock::matchers::{header, method, path, query_param};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn test_search_restaurants_success() {
    let (mock_server, client) = setup_mock_client().await;
    let fixture_body = include_str!("fixtures/search_restaurants_success.json");

    Mock::given(method("GET"))
        .and(path("/restaurants"))
        .and(header("Authorization", AUTH_HEADER))
        .and(query_param("query", "Chipotle"))
        .and(query_param("latitude", "37.7749"))
        .and(query_param("longitude", "-122.4194"))
        .and(query_param("limit", "5"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(fixture_body, "application/json"))
        .expect(1)
        .mount(&mock_server)
        .await;

    let query = RestaurantSearchQuery {
        query: Some("Chipotle".to_string()),
        latitude: Some(37.7749),
        longitude: Some(-122.4194),
        limit: Some(5),
        radius_meters: None,
    };

    let res = client
        .search_restaurants(&query)
        .await
        .expect("Failed to search restaurants");

    assert!(res.items.is_empty() || !res.items[0].id.is_empty());
}

#[tokio::test]
async fn test_search_menu_items_success() {
    let (mock_server, client) = setup_mock_client().await;
    let fixture_body = include_str!("fixtures/search_menu_items_success.json");

    Mock::given(method("GET"))
        .and(path("/menu-items"))
        .and(header("Authorization", AUTH_HEADER))
        .and(query_param("query", "burrito"))
        .and(query_param("latitude", "37.7749"))
        .and(query_param("longitude", "-122.4194"))
        .and(query_param("limit", "5"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(fixture_body, "application/json"))
        .expect(1)
        .mount(&mock_server)
        .await;

    let query = MenuItemSearchQuery {
        query: Some("burrito".to_string()),
        latitude: Some(37.7749),
        longitude: Some(-122.4194),
        limit: Some(5),
        radius_meters: None,
    };

    let res = client
        .search_menu_items(&query)
        .await
        .expect("Failed to search menu items");

    assert!(res.items.len() <= 5);
    assert!(
        !res.items.is_empty(),
        "Fixture should contain at least one item"
    );
    assert!(
        !res.items[0].id.is_empty(),
        "First item should have a valid ID"
    );
}

#[tokio::test]
async fn test_get_restaurant_menu_success() {
    let (mock_server, client) = setup_mock_client().await;
    let fixture_body = include_str!("fixtures/get_restaurant_menu_success.json");
    let restaurant_id = "rest_12345";

    Mock::given(method("GET"))
        .and(path(format!("/restaurants/{}/menu-items", restaurant_id)))
        .and(header("Authorization", AUTH_HEADER))
        .and(query_param("limit", "10"))
        .and(query_param("offset", "0"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(fixture_body, "application/json"))
        .expect(1)
        .mount(&mock_server)
        .await;

    let query = RestaurantMenuItemsQuery {
        limit: Some(10),
        offset: Some(0),
    };

    let res = client
        .get_restaurant_menu(restaurant_id, &query)
        .await
        .expect("Failed to get restaurant menu");

    assert!(res.items.len() <= 10);
    assert!(!res.items.is_empty(), "Fixture should contain menu items");
    assert!(
        !res.items[0].id.is_empty(),
        "First menu item should have a valid ID"
    );
}
