use serde::{Deserialize, Serialize};

use super::common::*;

#[derive(Debug, Clone, Default, Serialize)]
pub struct RestaurantSearchQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latitude: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longitude: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radius_meters: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct MenuItemSearchQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latitude: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longitude: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radius_meters: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct RestaurantMenuItemsQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Restaurant {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub item_type: Option<String>,
    pub is_chain: Option<bool>,
    pub distance_meters: Option<f64>,
    pub city: Option<String>,
    pub address1: Option<String>,
    pub address2: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuItemNutrients {
    pub calories: Option<NutrientAmount>,
    pub protein: Option<NutrientAmount>,
    pub carbohydrates: Option<NutrientAmount>,
    pub net_carbohydrates: Option<NutrientAmount>,
    pub total_fat: Option<NutrientAmount>,
    pub trans_fat: Option<NutrientAmount>,
    pub saturated_fat: Option<NutrientAmount>,
    pub fiber: Option<NutrientAmount>,
    pub total_sugars: Option<NutrientAmount>,
    pub added_sugars: Option<NutrientAmount>,
    pub cholesterol: Option<NutrientAmount>,
    pub calcium: Option<NutrientAmount>,
    pub iron: Option<NutrientAmount>,
    pub potassium: Option<NutrientAmount>,
    pub sodium: Option<NutrientAmount>,
    pub vitamin_d: Option<NutrientAmount>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuItem {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub item_type: Option<String>,
    pub restaurant_name: Option<String>,
    pub is_chain: Option<bool>,
    pub nutrients: Option<MenuItemNutrients>,
    pub glycemic_index: Option<f64>,
    pub glycemic_load: Option<f64>,
    pub image_url: Option<String>,
    pub distance_meters: Option<f64>,
    #[serde(default)]
    pub servings: Vec<FoodServing>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestaurantMenuItem {
    pub id: String,
    pub name: String,
    pub nutrients: Option<MenuItemNutrients>,
    pub glycemic_index: Option<f64>,
    pub glycemic_load: Option<f64>,
    #[serde(default)]
    pub image_url: Option<String>,
    #[serde(default)]
    pub servings: Vec<FoodServing>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestaurantListResponse {
    pub items: Vec<Restaurant>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuItemListResponse {
    pub items: Vec<MenuItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestaurantMenuItemListResponse {
    pub items: Vec<RestaurantMenuItem>,
}
