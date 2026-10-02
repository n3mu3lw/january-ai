use serde::{Deserialize, Serialize};

use super::common::*;

/// Configuration for the vision model reasoning level.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reasoning {
    /// Reasoning depth level (`"none"` for standard analyzer, `"xhigh"` for reasoning-based analyzer).
    pub effort: String,
}

/// Request payload for analyzing a food or label image.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyzeImageRequest {
    /// Image as a publicly fetchable HTTP(S) URL or a base64 data URI (under 5 MB total payload).
    pub image: String,

    /// Optional reasoning configuration to select the analyzer engine. Refer to [`Reasoning`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<Reasoning>,
}

/// Request payload for analyzing a meal description.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyzeTextRequest {
    /// Natural-language description of what was eaten.
    pub text: String,
}

/// Detected catalog food record with calculated portion details.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectedFood {
    /// Matched catalog food identifier.
    pub id: String,

    /// Food item name (1–256 chars).
    pub name: String,

    /// Brand name. Null for generic foods.
    #[serde(default)]
    pub brand_name: Option<String>,

    /// Consumed portion multiplier applied to the selected catalog serving (max 10000).
    #[serde(default)]
    pub quantity: Option<f64>,

    /// Selected serving configuration. Refer to [`FoodServing`].
    #[serde(default)]
    pub serving: Option<FoodServing>,

    /// Nutritional breakdown for the detected food item.
    #[serde(default)]
    pub nutrients: Option<Nutrients>,
}

/// Detection result entry for a recognized food item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Detection {
    /// Vision model confidence level (`"high"`, `"medium"`, `"low"`, or `null`).
    #[serde(default)]
    pub confidence: Option<String>,

    /// Recognized food entry details. Refer to [`DetectedFood`].
    pub food: DetectedFood,
}

/// Response payload containing detected foods and total nutrition analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FoodAnalysisResponse {
    /// Identified overall meal name (max 256 chars).
    #[serde(default)]
    pub meal_name: Option<String>,

    /// Aggregated nutrient totals for all detected foods. Refer to [`Nutrients`].
    pub total_nutrients: Nutrients,

    /// List of recognized food detections (up to 100 items). Refer to [`Detection`].
    #[serde(default)]
    pub detections: Vec<Detection>,
}

/// Request payload to correct an analysis in plain English
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorrectionRequest {
    /// Existing food analysis payload to be corrected. Refer to [`FoodAnalysisResponse`].
    pub analysis: FoodAnalysisResponse,
    /// Plain-English description of what to correct.
    pub instruction: String,
}
