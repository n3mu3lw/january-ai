use serde::{Deserialize, Serialize};

/// Defines the permission scopes available for end-user client tokens.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Scope {
    /// Autocomplete, search, barcode lookup, get a food, alternatives
    #[serde(rename = "foods:read")]
    FoodsRead,

    /// Restaurant search, menu-list search, and a restaurant's menu
    #[serde(rename = "restaurants:read")]
    RestaurantsRead,

    /// Image, text, and corrections
    #[serde(rename = "food_analysis:write")]
    FoodAnalysisWrite,

    /// List, summary, and get
    #[serde(rename = "food_logs:read")]
    FoodLogsRead,

    /// Create, update, and delete
    #[serde(rename = "food_logs:write")]
    FoodLogsWrite,

    /// List daily totals
    #[serde(rename = "water_logs:read")]
    WaterLogsRead,

    /// Create and delete
    #[serde(rename = "water_logs:write")]
    WaterLogsWrite,

    /// List daily weights
    #[serde(rename = "weight_logs:read")]
    WeightLogsRead,

    /// Create
    #[serde(rename = "weight_logs:write")]
    WeightLogsWrite,

    /// Glucose prediction
    #[serde(rename = "glucose:read")]
    GlucoseRead,
}

/// Specific serving size configuration for a food item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FoodServing {
    /// Opaque serving identifier.
    pub id: String,

    /// Serving quantity amount.
    #[serde(default)]
    pub quantity: Option<f64>,

    /// Serving unit label (e.g., `"oz"`).
    #[serde(default)]
    pub unit: Option<String>,

    /// Multiplier applied to base nutrition values for this serving.
    #[serde(default)]
    pub scaling_factor: Option<f64>,

    /// Equivalent weight in grams.
    #[serde(default)]
    pub weight_grams: Option<f64>,

    /// Whether this is the default serving size for the food item.
    #[serde(default)]
    pub is_primary: Option<bool>,
}

/// Measurement value and canonical unit for a single nutrient.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NutrientAmount {
    /// Quantitative measurement value.
    pub value: f64,

    /// Measurement unit (`g`, `mg`, `kcal`, `IU`, `mcg`).
    pub unit: String,
}

/// Nutrient breakdown for a food item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Nutrients {
    /// Energy content. Refer to [`NutrientAmount`].
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub calories: Option<NutrientAmount>,

    /// Total protein content. Refer to [`NutrientAmount`].
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub protein: Option<NutrientAmount>,

    /// Total carbohydrate content. Refer to [`NutrientAmount`].
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub carbohydrates: Option<NutrientAmount>,

    /// Net carbohydrate content (total carbs minus fiber). Refer to [`NutrientAmount`].
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub net_carbohydrates: Option<NutrientAmount>,

    /// Total fat content. Refer to [`NutrientAmount`].
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub total_fat: Option<NutrientAmount>,

    /// Trans fatty acid content. Refer to [`NutrientAmount`].
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub trans_fat: Option<NutrientAmount>,

    /// Saturated fatty acid content. Refer to [`NutrientAmount`].
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub saturated_fat: Option<NutrientAmount>,

    /// Dietary fiber content. Refer to [`NutrientAmount`].
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub fiber: Option<NutrientAmount>,

    /// Total sugars content. Refer to [`NutrientAmount`].
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub total_sugars: Option<NutrientAmount>,

    /// Added sugars content. Refer to [`NutrientAmount`].
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub added_sugars: Option<NutrientAmount>,

    /// Cholesterol content. Refer to [`NutrientAmount`].
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub cholesterol: Option<NutrientAmount>,

    /// Calcium content. Refer to [`NutrientAmount`].
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub calcium: Option<NutrientAmount>,

    /// Iron content. Refer to [`NutrientAmount`].
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub iron: Option<NutrientAmount>,

    /// Potassium content. Refer to [`NutrientAmount`].
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub potassium: Option<NutrientAmount>,

    /// Sodium content. Refer to [`NutrientAmount`].
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub sodium: Option<NutrientAmount>,

    /// Vitamin D content. Refer to [`NutrientAmount`].
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub vitamin_d: Option<NutrientAmount>,
}

/// Selected catalog food, serving, and portion size.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FoodSelection {
    /// Food identifier from a search or food-analysis result.
    pub food_id: String,

    /// Serving identifier associated with the food item.
    pub serving_id: String,

    /// Quantity of the serving consumed (max 10000).
    pub quantity: f64,
}
