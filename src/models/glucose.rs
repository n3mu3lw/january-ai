use serde::{Deserialize, Serialize};

use super::common::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeightInput {
    pub value: f64,
    pub unit: String, // e.g. "in" or "cm"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeightInput {
    pub value: f64,
    pub unit: String, // e.g. "lb" or "kg"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlucoseUserProfile {
    pub age: u32,
    pub sex: String, // "male", "female", "other"
    pub height: HeightInput,
    pub weight: WeightInput,
    pub activity_level: String, // "sedentary", "lightly_active", "moderately_active", "very_active"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_conditions: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CgmReading {
    pub value: f64,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsumedFood {
    pub food_id: String,
    pub serving_id: String,
    pub quantity: f64,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictGlucoseRequest {
    pub user_profile: GlucoseUserProfile,
    pub foods: Vec<FoodSelection>,
    pub start_time: String,
    pub timezone: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cgm_data: Option<Vec<CgmReading>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_foods: Option<Vec<ConsumedFood>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlucosePredictionPoint {
    pub minutes: u32,
    pub value: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlucoseChart {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictGlucoseResponse {
    pub points: Vec<GlucosePredictionPoint>,
    pub impact_score: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chart: Option<GlucoseChart>,
}
