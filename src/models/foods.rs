use serde::{Deserialize, Serialize};

use super::common::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FoodType {
    Generic,
    Branded,
    Recipe,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FoodItem {
    pub id: String,
    pub name: String,
    #[serde(default, rename = "brand_name")]
    pub brand: Option<String>,
    #[serde(default, rename = "image_url")]
    pub thumbnail: Option<String>,
    #[serde(default, rename = "type")]
    pub food_type: Option<FoodType>,
    #[serde(default)]
    pub nutrients: Option<Nutrients>,
    #[serde(default, rename = "servings")]
    pub serving_sizes: Option<Vec<FoodServing>>,
    #[serde(default)]
    pub glycemic_index: Option<f64>,
    #[serde(default)]
    pub glycemic_load: Option<f64>,
    #[serde(default)]
    pub barcode: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutocompleteSuggestion {
    pub id: String,
    pub name: String,
    #[serde(default, rename = "brand_name")]
    pub brand: Option<String>,
    #[serde(default, rename = "image_url")]
    pub thumbnail: Option<String>,
    #[serde(default)]
    pub calories: Option<NutrientAmount>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct SearchFoodsQuery {
    pub query: String,
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub food_type: Option<FoodType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct AutocompleteQuery {
    pub query: String,
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub food_type: Option<FoodType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HealthierAlternativesRequest {
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub diet_restrictions: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub diet_preferences: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SearchFoodsResponse {
    pub items: Vec<FoodItem>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AutocompleteResponse {
    pub items: Vec<AutocompleteSuggestion>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct HealthierAlternativesResponse {
    pub alternatives: Vec<FoodItem>,
}
