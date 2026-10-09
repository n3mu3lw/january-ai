mod common;
use std::assert_eq;

use common::*;

use january_ai::{
    CreateFoodLogRequest, FoodLog, FoodLogSummaryQuery, FoodSelection, ListFoodLogsQuery,
    ListFoodLogsResponse, UpdateFoodLogRequest,
};
use wiremock::matchers::{header, method, path, query_param};
use wiremock::{Mock, ResponseTemplate};

const TEST_USER_ID: &str = "testusr";

#[tokio::test]
async fn test_create_food_log() {
    let (mock_server, client) = setup_mock_client().await;
    let fixture_body = include_str!("fixtures/create_food_log_success.json");

    Mock::given(method("POST"))
        .and(path("/food-logs"))
        .and(header("Authorization", AUTH_HEADER))
        .and(header("January-End-User-ID", TEST_USER_ID))
        .and(header("Content-Type", "application/json"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(fixture_body, "application/json"))
        .expect(1)
        .mount(&mock_server)
        .await;

    let req = CreateFoodLogRequest {
        name: Some("Test Log".to_string()),
        created_at: None,
        foods: vec![FoodSelection {
            food_id: "67835190".to_string(),
            serving_id: "66948544".to_string(),
            quantity: 1.0,
        }],
    };

    let res: FoodLog = client
        .create_food_log(TEST_USER_ID, &req)
        .await
        .expect("Failed to create food log");

    assert_eq!(
        res.id.as_deref(),
        Some("13ad8e2a-fcdf-4768-946d-2d13e65e9c9b")
    );
    assert_eq!(res.name.as_deref(), Some("Test Log"));
    assert_eq!(res.foods.len(), 1);
    assert_eq!(res.foods[0].food_id, "67835190");
    assert_eq!(res.foods[0].name, Some("Cola".to_string()));
}

#[tokio::test]
async fn test_list_food_logs_success() {
    let (mock_server, client) = setup_mock_client().await;
    let fixture_body = include_str!("fixtures/list_food_logs_success.json");

    Mock::given(method("GET"))
        .and(path("/food-logs"))
        .and(header("Authorization", AUTH_HEADER))
        .and(header("January-End-User-ID", TEST_USER_ID))
        .and(query_param("start_date", "2026-10-01"))
        .and(query_param("end_date", "2026-10-09"))
        .and(query_param("timezone", "Africa/Nairobi"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(fixture_body, "application/json"))
        .expect(1)
        .mount(&mock_server)
        .await;

    let query = ListFoodLogsQuery {
        start_date: "2026-10-01".to_string(),
        end_date: "2026-10-09".to_string(),
        timezone: "Africa/Nairobi".to_string(),
    };

    let res: ListFoodLogsResponse = client
        .list_food_logs(TEST_USER_ID, &query)
        .await
        .expect("Failed to list food logs");

    assert!(res.items.is_empty() || res.items[0].id.is_some());
    assert_eq!(res.items[0].foods[0].food_id, "67835190");
}

#[tokio::test]
async fn test_get_food_log_summary_success() {
    let (mock_server, client) = setup_mock_client().await;
    let fixture_body = include_str!("fixtures/get_food_log_summary_success.json");

    Mock::given(method("GET"))
        .and(path("/food-logs/summary"))
        .and(header("Authorization", AUTH_HEADER))
        .and(header("January-End-User-ID", TEST_USER_ID))
        .and(query_param("start_date", "2026-10-01"))
        .and(query_param("end_date", "2026-10-09"))
        .and(query_param("timezone", "Africa/Nairobi"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(fixture_body, "application/json"))
        .expect(1)
        .mount(&mock_server)
        .await;

    let query = FoodLogSummaryQuery {
        start_date: "2026-10-01".to_string(),
        end_date: "2026-10-09".to_string(),
        timezone: "Africa/Nairobi".to_string(),
        group_by: None,
        week_start: None,
    };

    let res = client
        .get_food_log_summary(TEST_USER_ID, &query)
        .await
        .expect("Failed to get food log summary");

    assert_eq!(res.timezone, "Africa/Nairobi");
    assert_eq!(res.start_date, "2026-10-01");
    assert_eq!(res.end_date, "2026-10-09");
    assert!(res.buckets[8].logs_count == 1);
}

#[tokio::test]
async fn test_get_food_log_success() {
    let (mock_server, client) = setup_mock_client().await;
    let fixture_body = include_str!("fixtures/get_food_log_success.json");
    let log_id = "13ad8e2a-fcdf-4768-946d-2d13e65e9c9b";

    Mock::given(method("GET"))
        .and(path(format!("/food-logs/{}", log_id)))
        .and(header("Authorization", AUTH_HEADER))
        .and(header("January-End-User-ID", TEST_USER_ID))
        .respond_with(ResponseTemplate::new(200).set_body_raw(fixture_body, "application/json"))
        .expect(1)
        .mount(&mock_server)
        .await;

    let res = client
        .get_food_log(TEST_USER_ID, log_id)
        .await
        .expect("Failed to get food log");

    assert_eq!(res.id.as_deref(), Some(log_id));
    assert!(!res.foods.is_empty());
}

#[tokio::test]
async fn test_update_food_log_success() {
    let (mock_server, client) = setup_mock_client().await;
    let fixture_body = include_str!("fixtures/update_food_log_success.json");
    let end_user_id = "testusr";
    let log_id = "13ad8e2a-fcdf-4768-946d-2d13e65e9c9b";

    Mock::given(method("PATCH"))
        .and(path(format!("/food-logs/{}", log_id)))
        .and(header("Authorization", AUTH_HEADER))
        .and(header("January-End-User-ID", end_user_id))
        .and(header("Content-Type", "application/json"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(fixture_body, "application/json"))
        .expect(1)
        .mount(&mock_server)
        .await;

    let req = UpdateFoodLogRequest {
        name: Some("Updated Breakfast Log".to_string()),
        created_at: None,
        foods: Some(vec![FoodSelection {
            food_id: "67835190".to_string(),
            serving_id: "66948544".to_string(),
            quantity: 2.0,
        }]),
    };

    let res = client
        .update_food_log(end_user_id, log_id, &req)
        .await
        .expect("Failed to update food log");

    assert!(res.foods[0].quantity == Some(2.0));
}

#[tokio::test]
async fn test_delete_food_log_success() {
    let (mock_server, client) = setup_mock_client().await;
    let log_id = "13ad8e2a-fcdf-4768-946d-2d13e65e9c9b";

    Mock::given(method("DELETE"))
        .and(path(format!("/food-logs/{}", log_id)))
        .and(header("Authorization", AUTH_HEADER))
        .and(header("January-End-User-ID", TEST_USER_ID))
        .respond_with(ResponseTemplate::new(204)) // or 200 with empty body
        .expect(1)
        .mount(&mock_server)
        .await;

    let res = client.delete_food_log(TEST_USER_ID, log_id).await;
    assert!(res.is_ok(), "Expected Ok(()) on 204 No Content response");
}
