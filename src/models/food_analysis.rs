use serde::{Deserialize, Serialize};

use super::common::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reasoning {
    pub effort: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectedFood {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub brand_name: Option<String>,
    #[serde(default)]
    pub quantity: Option<f64>,
    #[serde(default)]
    pub serving: Option<FoodServing>,
    #[serde(default)]
    pub nutrients: Option<Nutrients>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Detection {
    #[serde(default)]
    pub confidence: Option<String>,
    pub food: DetectedFood,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FoodAnalysisResponse {
    #[serde(default)]
    pub meal_name: Option<String>,
    pub total_nutrients: Nutrients,
    #[serde(default)]
    pub detections: Vec<Detection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyzeImageRequest {
    pub image: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<Reasoning>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyzeTextRequest {
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorrectionRequest {
    pub analysis: FoodAnalysisResponse,
    pub instruction: String,
}
