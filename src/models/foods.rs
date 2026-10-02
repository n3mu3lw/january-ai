use serde::{Deserialize, Serialize};

use super::common::*;

/// Food classification category filter.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FoodType {
    /// Generic or unbranded whole foods.
    Generic,

    /// Branded commercial food products.
    Branded,

    /// Prepared multi-ingredient recipes.
    Recipe,
}
/// Detailed food item record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FoodItem {
    /// Opaque food identifier.
    pub id: String,

    /// Name of the food item.
    pub name: String,

    /// Brand name. Null for generic foods and recipes.
    #[serde(default, rename = "brand_name")]
    pub brand: Option<String>,

    /// URL of a picture of the food.
    #[serde(default, rename = "image_url")]
    pub thumbnail: Option<String>,

    /// Classification category (generic, branded, or recipe).
    #[serde(default, rename = "type")]
    pub food_type: Option<FoodType>,

    /// Nutritional values for this item.
    #[serde(default)]
    pub nutrients: Option<Nutrients>,

    /// Available serving sizes. Search/barcode results carry the default serving only.
    #[serde(default, rename = "servings")]
    pub serving_sizes: Option<Vec<FoodServing>>,

    /// Glycemic index value.
    #[serde(default)]
    pub glycemic_index: Option<f64>,

    /// Glycemic load value.
    #[serde(default)]
    pub glycemic_load: Option<f64>,

    /// Product barcode (UPC, EAN, GTIN). Normalized form; do not string-compare directly.
    #[serde(default)]
    pub barcode: Option<String>,
}

/// Type-ahead suggestion entry for a food item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutocompleteSuggestion {
    /// Opaque food identifier.
    pub id: String,

    /// Food item name. Generic foods are lowercase; branded items retain product casing.
    pub name: String,

    /// Brand name. Null for generic foods.
    #[serde(default, rename = "brand_name")]
    pub brand: Option<String>,

    /// URL thumbnail image for the food item.
    #[serde(default, rename = "image_url")]
    pub thumbnail: Option<String>,

    /// Caloric content per default serving size. Refer to [`NutrientAmount`].
    #[serde(default)]
    pub calories: Option<NutrientAmount>,
}

/// Query parameters for searching foods.
#[derive(Debug, Clone, Default, Serialize)]
pub struct SearchFoodsQuery {
    /// Food name search query (max 256 chars).
    pub query: String,

    /// Filter by food classification category. Refer to [`FoodType`].
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub food_type: Option<FoodType>,

    /// Maximum results to return (1–50, defaults to 10).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    /// Number of results to skip for pagination (defaults to 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
}

/// Query parameters for food name autocomplete suggestions.
#[derive(Debug, Clone, Default, Serialize)]
pub struct AutocompleteQuery {
    /// Partial text input typed so far (max 64 chars; yields no results if under 2 chars).
    pub query: String,

    /// Filter suggestions by food classification. Refer to [`FoodType`].
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub food_type: Option<FoodType>,

    /// Maximum suggestions to return (1–20, defaults to 8).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

/// Response containing food search matches.
#[derive(Debug, Clone, Deserialize)]
pub struct SearchFoodsResponse {
    /// Ranked matches, best first. Empty when nothing matches.
    pub items: Vec<FoodItem>,
}

/// Response payload containing food autocomplete suggestions.
#[derive(Debug, Clone, Deserialize)]
pub struct AutocompleteResponse {
    /// Ranked suggestions, generic foods before branded. Empty when under 2 chars or no matches.
    pub items: Vec<AutocompleteSuggestion>,
}

/// Request constraints for finding healthier food alternatives.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HealthierAlternativesRequest {
    /// Allergens and ingredients to avoid (up to 17 items; e.g., `"gluten"`, `"peanuts"`).
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub diet_restrictions: Vec<String>,

    /// Dietary patterns to match (up to 9 items; e.g., `"vegetarian"`, `"keto"`).
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub diet_preferences: Vec<String>,
}

/// Response payload containing recommended healthier food alternatives.
#[derive(Debug, Clone, Deserialize)]
pub struct HealthierAlternativesResponse {
    /// List of alternative food items matching specified restrictions and preferences. Empty when none found.
    pub alternatives: Vec<FoodItem>,
}
