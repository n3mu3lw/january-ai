use serde::{Deserialize, Serialize};

use super::common::*;

/// Query parameters for restaurant search near a location.
#[derive(Debug, Clone, Default, Serialize)]
pub struct RestaurantSearchQuery {
    /// Restaurant name to search for (max 256 chars).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,

    /// Latitude of search location (-90 to 90).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latitude: Option<f64>,

    /// Longitude of search location (-180 to 180).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longitude: Option<f64>,

    /// Search radius in meters (1 to 50000, default 8000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radius_meters: Option<f64>,

    /// Maximum number of results to return (1 to 100, default 10).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

/// Query parameters for menu item search.
#[derive(Debug, Clone, Default, Serialize)]
pub struct MenuItemSearchQuery {
    /// Dish or restaurant name to search for (max 256 chars).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,

    /// Latitude of search location (-90 to 90).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latitude: Option<f64>,

    /// Longitude of search location (-180 to 180).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longitude: Option<f64>,

    /// Search radius in meters (1 to 50000, default 8000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radius_meters: Option<f64>,

    /// Maximum number of results to return (1 to 100, default 10).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

/// Query parameters for fetching a restaurant's menu items.
#[derive(Debug, Clone, Default, Serialize)]
pub struct RestaurantMenuItemsQuery {
    /// Maximum number of menu items to return (1 to 500, default 100).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    /// Number of menu items to skip for paging (default 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
}

/// Restaurant location details.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Restaurant {
    /// Restaurant identifier.
    pub id: String,

    /// Restaurant name. Null only when the source has no name.
    pub name: String,

    /// Place classification (always `"restaurant"`).
    #[serde(rename = "type")]
    pub item_type: Option<String>,

    /// Whether location belongs to a chain. Null when unknown.
    pub is_chain: Option<bool>,

    /// Distance from search coordinates in meters. Null when unplaced.
    pub distance_meters: Option<f64>,

    /// City name.
    pub city: Option<String>,

    /// Primary street address line.
    pub address1: Option<String>,

    /// Secondary address line. Null when none.
    pub address2: Option<String>,
}

/// Menu item dish details and nutrition breakdown.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuItem {
    /// Menu item identifier.
    pub id: String,

    /// Dish name. Null only when the menu source has no name.
    pub name: String,

    /// Item classification (always `"menu_item"`).
    #[serde(rename = "type")]
    pub item_type: Option<String>,

    /// Name of the restaurant serving the dish. Null when unnamed by source.
    pub restaurant_name: Option<String>,

    /// Whether the restaurant belongs to a chain. Null when unknown.
    pub is_chain: Option<bool>,

    /// Published nutritional values for the dish. Refer to [`Nutrients`].
    pub nutrients: Option<Nutrients>,

    /// Glycemic index value. Null when unavailable from source.
    pub glycemic_index: Option<f64>,

    /// Glycemic load value. Null when unavailable from source.
    pub glycemic_load: Option<f64>,

    /// Picture URL for the dish. Null when unavailable from source.
    pub image_url: Option<String>,

    /// Distance from search coordinates in meters.
    pub distance_meters: Option<f64>,

    /// Serving sizes available for the dish. Refer to [`FoodServing`].
    #[serde(default)]
    pub servings: Vec<FoodServing>,
}

/// Menu item entry listed under a specific restaurant.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestaurantMenuItem {
    /// Food identifier for the dish. Refer to [`FoodItem`].
    pub id: String,

    /// Dish name. Null only when the menu source has no name.
    pub name: String,

    /// Nutritional breakdown for the dish. Refer to [`Nutrients`].
    pub nutrients: Option<Nutrients>,

    /// Glycemic index value. Null when unavailable from source.
    pub glycemic_index: Option<f64>,

    /// Glycemic load value. Null when unavailable from source.
    pub glycemic_load: Option<f64>,

    /// Picture URL for the dish.
    #[serde(default)]
    pub image_url: Option<String>,

    /// Serving sizes available for the dish. Refer to [`FoodServing`].
    #[serde(default)]
    pub servings: Vec<FoodServing>,
}

/// Response payload containing restaurant search results.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestaurantListResponse {
    /// Matches ranked by proximity. Empty when nothing matches.
    pub items: Vec<Restaurant>,
}

/// Response payload containing menu item search results.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuItemListResponse {
    /// Matching dishes ranked by proximity. Empty when nothing matches.
    pub items: Vec<MenuItem>,
}

/// Response payload containing menu items for a specific restaurant.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestaurantMenuItemListResponse {
    /// List of menu items ordered by name. Empty when no menu is on record or offset is past the end.
    pub items: Vec<RestaurantMenuItem>,
}
