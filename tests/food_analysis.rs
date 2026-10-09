mod common;
use common::{AUTH_HEADER, setup_mock_client};

use january_ai::{
    AnalyzeImageRequest, AnalyzeTextRequest, CorrectionRequest, FoodAnalysisResponse,
};
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn test_analyze_food_image_success() {
    let (mock_server, client) = setup_mock_client().await;
    let fixture_body = include_str!("fixtures/analyze_food_image_success.json");

    Mock::given(method("POST"))
        .and(path("/food-analysis/image"))
        .and(header("Authorization", AUTH_HEADER))
        .and(header("Content-Type", "application/json"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(fixture_body, "application/json"))
        .expect(1)
        .mount(&mock_server)
        .await;

    let req = AnalyzeImageRequest {
        image: "https://images.unsplash.com/photo-1568901346375-23c9450c58cd?w=500".to_string(),
        reasoning: None,
    };

    let res = client
        .analyze_food_image(&req)
        .await
        .expect("Failed to analyze food image");

    assert_eq!(res.meal_name.as_deref(), Some("Double Cheeseburger"));
    assert!(!res.detections.is_empty());
}

#[tokio::test]
async fn test_analyze_food_text_success() {
    let (mock_server, client) = setup_mock_client().await;
    let fixture_body = include_str!("fixtures/analyze_food_text_success.json");

    Mock::given(method("POST"))
        .and(path("/food-analysis/text"))
        .and(header("Authorization", AUTH_HEADER))
        .and(header("Content-Type", "application/json"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(fixture_body, "application/json"))
        .expect(1)
        .mount(&mock_server)
        .await;

    let req = AnalyzeTextRequest {
        text: "avocado toast with poached egg".to_string(),
    };

    let res = client
        .analyze_food_text(&req)
        .await
        .expect("Failed to analyze food text");

    assert!(!res.detections.is_empty());
    assert!(res.total_nutrients.calories.is_some() || res.total_nutrients.carbohydrates.is_some());
}

#[tokio::test]
async fn test_correct_food_analysis_success() {
    let (mock_server, client) = setup_mock_client().await;
    let fixture_body = include_str!("fixtures/correct_food_analysis_success.json");

    Mock::given(method("POST"))
        .and(path("/food-analysis/corrections"))
        .and(header("Authorization", AUTH_HEADER))
        .and(header("Content-Type", "application/json"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(fixture_body, "application/json"))
        .expect(1)
        .mount(&mock_server)
        .await;

    let previous_analysis: FoodAnalysisResponse =
        serde_json::from_str(include_str!("fixtures/analyze_food_text_success.json"))
            .expect("Failed to parse previous analysis fixture");

    let req = CorrectionRequest {
        analysis: previous_analysis,
        instruction: "Add a glass of orange juice".to_string(),
    };

    let res = client
        .correct_food_analysis(&req)
        .await
        .expect("Failed to correct food analysis");

    assert!(!res.detections.is_empty() || res.total_nutrients.calories.is_some());
}
