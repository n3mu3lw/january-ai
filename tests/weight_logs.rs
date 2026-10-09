mod common;
use common::*;

use january_ai::{ListWeightLogsQuery, LogWeightRequest, Weight, WeightUnit};
use wiremock::matchers::{header, method, path, query_param};
use wiremock::{Mock, ResponseTemplate};

const TEST_USER_ID: &str = "testusr";

#[tokio::test]
async fn test_create_weight_log_success() {
    let (mock_server, client) = setup_mock_client().await;
    let fixture_body = include_str!("fixtures/create_weight_log_success.json");

    Mock::given(method("POST"))
        .and(path("/weight-logs"))
        .and(header("Authorization", AUTH_HEADER))
        .and(header("January-End-User-ID", TEST_USER_ID))
        .and(header("Content-Type", "application/json"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(fixture_body, "application/json"))
        .expect(1)
        .mount(&mock_server)
        .await;

    let req = LogWeightRequest {
        weight: Weight {
            value: 75.5,
            unit: WeightUnit::Kg,
        },
        created_at: None,
    };

    let res = client
        .create_weight_log(TEST_USER_ID, &req)
        .await
        .expect("Failed to create weight log");

    assert_eq!(res.weight.value, 75.5);
}

#[tokio::test]
async fn test_list_weight_logs_success() {
    let (mock_server, client) = setup_mock_client().await;
    let fixture_body = include_str!("fixtures/list_weight_logs_success.json");

    Mock::given(method("GET"))
        .and(path("/weight-logs"))
        .and(header("Authorization", AUTH_HEADER))
        .and(header("January-End-User-ID", TEST_USER_ID))
        .and(query_param("start_date", "2026-10-01"))
        .and(query_param("end_date", "2026-10-09"))
        .and(query_param("timezone", "Africa/Nairobi"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(fixture_body, "application/json"))
        .expect(1)
        .mount(&mock_server)
        .await;

    let query = ListWeightLogsQuery {
        start_date: "2026-10-01".to_string(),
        end_date: "2026-10-09".to_string(),
        timezone: "Africa/Nairobi".to_string(),
    };

    let res = client
        .list_weight_logs(TEST_USER_ID, &query)
        .await
        .expect("Failed to list weight logs");

    assert!(res.items.len() == 1);
}
