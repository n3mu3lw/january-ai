mod common;
use std::assert_eq;

use common::*;

use january_ai::{ListWaterLogsQuery, LogWaterRequest, WaterAmount, WaterUnit};
use wiremock::matchers::{header, method, path, query_param};
use wiremock::{Mock, ResponseTemplate};

const TEST_USER_ID: &str = "testusr";

#[tokio::test]
async fn test_create_water_log_success() {
    let (mock_server, client) = setup_mock_client().await;
    let fixture_body = include_str!("fixtures/create_water_log_success.json");

    Mock::given(method("POST"))
        .and(path("/water-logs"))
        .and(header("Authorization", AUTH_HEADER))
        .and(header("January-End-User-ID", TEST_USER_ID))
        .and(header("Content-Type", "application/json"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(fixture_body, "application/json"))
        .expect(1)
        .mount(&mock_server)
        .await;

    let req = LogWaterRequest {
        amount: WaterAmount {
            value: 500.0,
            unit: WaterUnit::Ml,
        },
        created_at: None,
    };

    let res = client
        .create_water_log(TEST_USER_ID, &req)
        .await
        .expect("Failed to log water");

    assert!(!res.id.is_empty());
    assert_eq!(res.amount.value, 500.0);
}

#[tokio::test]
async fn test_list_water_logs_success() {
    let (mock_server, client) = setup_mock_client().await;
    let fixture_body = include_str!("fixtures/list_water_logs_success.json");

    Mock::given(method("GET"))
        .and(path("/water-logs"))
        .and(header("Authorization", AUTH_HEADER))
        .and(header("January-End-User-ID", TEST_USER_ID))
        .and(query_param("start_date", "2026-10-01"))
        .and(query_param("end_date", "2026-10-09"))
        .and(query_param("timezone", "Africa/Nairobi"))
        .and(query_param("unit", "ml"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(fixture_body, "application/json"))
        .expect(1)
        .mount(&mock_server)
        .await;

    let query = ListWaterLogsQuery {
        start_date: "2026-10-01".to_string(),
        end_date: "2026-10-09".to_string(),
        timezone: "Africa/Nairobi".to_string(),
        unit: WaterUnit::Ml,
    };

    let res = client
        .list_water_logs(TEST_USER_ID, &query)
        .await
        .expect("Failed to list water logs");

    assert!(res.items[0].total.value == 500.0)
}

#[tokio::test]
async fn test_delete_water_log_success() {
    let (mock_server, client) = setup_mock_client().await;
    let log_id = "240b3042-8503-42e5-9f6a-57162233f573";

    Mock::given(method("DELETE"))
        .and(path(format!("/water-logs/{}", log_id)))
        .and(header("Authorization", AUTH_HEADER))
        .and(header("January-End-User-ID", TEST_USER_ID))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&mock_server)
        .await;

    let result = client.delete_water_log(TEST_USER_ID, log_id).await;

    assert!(result.is_ok(), "Expected Ok(()) on 204 No Content response");
}
