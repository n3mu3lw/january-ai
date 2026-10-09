mod common;
use common::*;

use january_ai::{
    FoodSelection, GlucoseUserProfile, HeightInput, PredictGlucoseRequest, WeightInput,
};
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn test_predict_glucose_success() {
    let (mock_server, client) = setup_mock_client().await;
    let fixture_body = include_str!("fixtures/predict_glucose_success.json");

    Mock::given(method("POST"))
        .and(path("/glucose/predictions"))
        .and(header("Authorization", AUTH_HEADER))
        .and(header("Content-Type", "application/json"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(fixture_body, "application/json"))
        .expect(1)
        .mount(&mock_server)
        .await;

    let req = PredictGlucoseRequest {
        user_profile: GlucoseUserProfile {
            age: 42,
            sex: "female".to_string(),
            height: HeightInput {
                value: 66.0,
                unit: "in".to_string(),
            },
            weight: WeightInput {
                value: 150.0,
                unit: "lb".to_string(),
            },
            activity_level: "moderately_active".to_string(),
            health_conditions: Some(vec!["prediabetes".to_string()]),
        },
        foods: vec![FoodSelection {
            food_id: "101963552".to_string(),
            serving_id: "68051535".to_string(),
            quantity: 1.4,
        }],
        start_time: "2026-09-29T12:00:00Z".to_string(),
        timezone: "America/New_York".to_string(),
        cgm_data: None,
        consumed_foods: None,
    };

    let res = client
        .predict_glucose(&req)
        .await
        .expect("Failed to predict glucose");

    assert!(!res.points.is_empty());
}
