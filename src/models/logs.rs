use serde::{Deserialize, Serialize};

use super::common::*;

// Food logs

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SummaryGroupBy {
    Day,
    Week,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SummaryWeekStart {
    Monday,
    Sunday,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFoodLogRequest {
    pub foods: Vec<FoodSelection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateFoodLogRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foods: Option<Vec<FoodSelection>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListFoodLogsQuery {
    pub start_date: String,
    pub end_date: String,
    pub timezone: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FoodLogSummaryQuery {
    pub start_date: String,
    pub end_date: String,
    pub timezone: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_by: Option<SummaryGroupBy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub week_start: Option<SummaryWeekStart>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggedServing {
    pub id: String,
    pub quantity: f64,
    pub unit: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight_grams: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggedFood {
    pub food_id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub glycemic_index: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub glycemic_load: Option<f64>,
    pub nutrients: Nutrients,
    pub quantity: f64,
    pub serving: LoggedServing,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FoodLog {
    pub id: String,
    pub foods: Vec<LoggedFood>,
    pub created_at: String,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListFoodLogsResponse {
    pub items: Vec<FoodLog>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FoodLogSummaryBucket {
    pub start_date: String,
    pub end_date: String,
    pub logs_count: u64,
    pub days_with_logs: u64,
    pub nutrients: Nutrients,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FoodLogSummaryTotals {
    pub logs_count: u64,
    pub days_with_logs: u64,
    pub nutrients: Nutrients,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FoodLogSummaryAverage {
    pub nutrients: Nutrients,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FoodLogSummaryResponse {
    pub group_by: SummaryGroupBy,
    pub week_start: Option<SummaryWeekStart>,
    pub timezone: String,
    pub start_date: String,
    pub end_date: String,
    pub buckets: Vec<FoodLogSummaryBucket>,
    pub totals: FoodLogSummaryTotals,
    pub average_per_logged_day: FoodLogSummaryAverage,
}

// Water logs

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum WaterUnit {
    #[serde(rename = "fl_oz")]
    FlOz,
    #[serde(rename = "ml")]
    Ml,
    #[serde(rename = "cup")]
    Cup,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaterAmount {
    pub value: f64,
    pub unit: WaterUnit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogWaterRequest {
    pub amount: WaterAmount,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaterLog {
    pub id: String,
    pub amount: WaterAmount,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListWaterLogsQuery {
    pub start_date: String,
    pub end_date: String,
    pub timezone: String,
    pub unit: WaterUnit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyWaterTotal {
    pub date: String,
    pub total: WaterAmount,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListWaterLogsResponse {
    pub items: Vec<DailyWaterTotal>,
}

// Weight logs

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum WeightUnit {
    #[serde(rename = "lb")]
    Lb,
    #[serde(rename = "kg")]
    Kg,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Weight {
    pub value: f64,
    pub unit: WeightUnit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogWeightRequest {
    pub weight: Weight,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogWeightResponse {
    pub weight: Weight,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListWeightLogsQuery {
    pub start_date: String,
    pub end_date: String,
    pub timezone: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyWeight {
    pub date: String,
    pub weight: Weight,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListWeightLogsResponse {
    pub items: Vec<DailyWeight>,
}
