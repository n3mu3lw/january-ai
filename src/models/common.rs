use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Scope {
    #[serde(rename = "foods:read")]
    FoodsRead,
    #[serde(rename = "restaurants:read")]
    RestaurantsRead,
    #[serde(rename = "food_analysis:write")]
    FoodAnalysisWrite,
    #[serde(rename = "food_logs:read")]
    FoodLogsRead,
    #[serde(rename = "food_logs:write")]
    FoodLogsWrite,
    #[serde(rename = "water_logs:read")]
    WaterLogsRead,
    #[serde(rename = "water_logs:write")]
    WaterLogsWrite,
    #[serde(rename = "weight_logs:read")]
    WeightLogsRead,
    #[serde(rename = "weight_logs:write")]
    WeightLogsWrite,
    #[serde(rename = "glucose:read")]
    GlucoseRead,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FoodServing {
    pub id: String,
    #[serde(default)]
    pub quantity: Option<f64>,
    #[serde(default)]
    pub unit: Option<String>,
    #[serde(default)]
    pub scaling_factor: Option<f64>,
    #[serde(default)]
    pub weight_grams: Option<f64>,
    #[serde(default)]
    pub is_primary: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NutrientAmount {
    pub value: f64,
    pub unit: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Nutrients {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub calories: Option<NutrientAmount>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub protein: Option<NutrientAmount>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub carbohydrates: Option<NutrientAmount>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub net_carbohydrates: Option<NutrientAmount>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub total_fat: Option<NutrientAmount>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub trans_fat: Option<NutrientAmount>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub saturated_fat: Option<NutrientAmount>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub fiber: Option<NutrientAmount>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub total_sugars: Option<NutrientAmount>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub added_sugars: Option<NutrientAmount>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub cholesterol: Option<NutrientAmount>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub calcium: Option<NutrientAmount>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub iron: Option<NutrientAmount>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub potassium: Option<NutrientAmount>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub sodium: Option<NutrientAmount>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub vitamin_d: Option<NutrientAmount>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FoodSelection {
    pub food_id: String,
    pub serving_id: String,
    pub quantity: f64,
}
