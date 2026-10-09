mod common;
use common::*;

use january_ai::PlanType;
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn test_get_credits_success() {
    let (mock_server, client) = setup_mock_client().await;
    let fixture_body = include_str!("fixtures/get_credits_success.json");

    Mock::given(method("GET"))
        .and(path("/credits"))
        .and(header("Authorization", AUTH_HEADER))
        .respond_with(ResponseTemplate::new(200).set_body_raw(fixture_body, "application/json"))
        .expect(1)
        .mount(&mock_server)
        .await;

    let res = client
        .get_credits()
        .await
        .expect("Failed to get credits balance");

    assert!(!res.period_start.is_empty());
    assert!(!res.period_end.is_empty());
    assert_eq!(res.plan, PlanType::Free);
}
