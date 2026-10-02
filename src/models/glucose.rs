use serde::{Deserialize, Serialize};

use super::common::*;

/// Height input value and unit.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeightInput {
    /// Height measurement value.
    pub value: f64,

    /// Height unit label (e.g., `"in"` or `"cm"`).
    pub unit: String,
}

/// Weight input value and unit.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeightInput {
    /// Weight measurement value.
    pub value: f64,

    /// Weight unit label (e.g., `"lb"` or `"kg"`).
    pub unit: String,
}

/// User profile details used for glucose prediction models.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlucoseUserProfile {
    /// User age (1–120).
    pub age: u32,

    /// Biological sex (`"male"` or `"female"`).
    pub sex: String,

    /// User height specification. Refer to [`HeightInput`].
    pub height: HeightInput,

    /// User weight specification. Refer to [`WeightInput`].
    pub weight: WeightInput,

    /// Activity level (`"sedentary"`, `"lightly_active"`, `"moderately_active"`, or `"very_active"`).
    pub activity_level: String,

    /// Health conditions (`"type_2_diabetes"`, `"prediabetes"`, max 2 items). Type 1 diabetes is not supported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_conditions: Option<Vec<String>>,
}

/// Continuous glucose monitor reading entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CgmReading {
    /// Glucose reading value in mg/dL (10–600).
    pub value: f64,

    /// ISO 8601 timestamp with timezone designator when reading was taken.
    pub timestamp: String,
}

/// Recorded food consumption entry corresponding to historical CGM data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsumedFood {
    /// Catalog food identifier.
    pub food_id: String,

    /// Catalog serving identifier.
    pub serving_id: String,

    /// Quantity of selected serving consumed (max 10000).
    pub quantity: f64,

    /// ISO 8601 timestamp with timezone designator when food was eaten.
    pub timestamp: String,
}

/// Request payload for predicting glucose response to a meal.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictGlucoseRequest {
    /// End-user profile specifications. Refer to [`GlucoseUserProfile`].
    pub user_profile: GlucoseUserProfile,

    /// Meal items to predict response for (1–100 items). Refer to [`FoodSelection`].
    pub foods: Vec<FoodSelection>,

    /// ISO 8601 timestamp with timezone designator when meal is eaten.
    pub start_time: String,

    /// IANA timezone identifier for local time calculations (e.g., `"America/New_York"`).
    pub timezone: String,

    /// CGM history to personalize prediction (1–5000 readings). Requires `consumed_foods`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cgm_data: Option<Vec<CgmReading>>,

    /// Meals eaten during CGM history (1–5000 entries). Requires `cgm_data`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_foods: Option<Vec<ConsumedFood>>,
}

/// Predicted glucose curve point at a specific time offset.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlucosePredictionPoint {
    /// Minutes elapsed after `start_time`.
    pub minutes: u32,

    /// Predicted glucose value in mg/dL.
    pub value: f64,
}

/// Suggested Y-axis chart rendering bounds.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlucoseChart {
    /// Suggested Y-axis lower bound in mg/dL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<f64>,

    /// Suggested Y-axis upper bound in mg/dL (180 with Type 2 diabetes, otherwise 140).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,
}

/// Response payload containing predicted glucose curve and impact score.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictGlucoseResponse {
    /// Predicted glucose curve points at 15-minute intervals. Refer to [`GlucosePredictionPoint`].
    pub points: Vec<GlucosePredictionPoint>,

    /// Overall glucose impact rating (`"low"`, `"medium"`, `"high"`, or `null`).
    pub impact_score: Option<String>,

    /// Suggested Y-axis bounds for chart rendering. Refer to [`GlucoseChart`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chart: Option<GlucoseChart>,
}
