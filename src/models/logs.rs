use serde::{Deserialize, Serialize};

use super::common::*;

// Food logs

/// Bucket size for summarizing food logs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SummaryGroupBy {
    /// One bucket per local calendar date.
    Day,
    /// One bucket per week.
    Week,
}

/// Weekday on which a week bucket begins.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SummaryWeekStart {
    /// Week starts on Monday
    Monday,
    /// Week starts on Sunday
    Sunday,
}

/// Request payload to log consumed foods for a user.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFoodLogRequest {
    /// List of foods consumed (1 to 100 items). Refer to [`FoodSelection`].
    pub foods: Vec<FoodSelection>,

    /// ISO 8601 date-time timestamp when the meal was eaten. Omitted defaults to current time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,

    /// Meal name or label (max 256 chars).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Request payload to update an existing food log entry.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateFoodLogRequest {
    /// Updated list of consumed foods (1 to 100 items). Refer to [`FoodSelection`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foods: Option<Vec<FoodSelection>>,

    /// ISO 8601 date-time timestamp when meal was eaten. Omit to leave unchanged.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,

    /// Updated meal name or label (max 256 chars).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Query parameters for retrieving a list of food logs over a date range.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListFoodLogsQuery {
    /// First local calendar date in timezone, inclusive (e.g., `"2024-09-01"`).
    pub start_date: String,

    /// Last local calendar date in timezone, inclusive. Range max 60 calendar days (e.g., `"2024-09-08"`).
    pub end_date: String,

    /// IANA timezone identifier defining the local calendar days (e.g., `"America/Los_Angeles"`).
    pub timezone: String,
}

/// Query parameters for fetching an aggregated food log summary.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FoodLogSummaryQuery {
    /// First local calendar date in timezone, inclusive (e.g., `"2024-09-01"`).
    pub start_date: String,

    /// Last local calendar date in timezone, inclusive. Range max 366 calendar days (e.g., `"2024-09-30"`).
    pub end_date: String,

    /// IANA timezone defining the local calendar days (e.g., `"America/Los_Angeles"`).
    pub timezone: String,

    /// Bucket size (defaults to `Day`). Refer to [`SummaryGroupBy`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_by: Option<SummaryGroupBy>,

    /// Weekday week buckets begin on (defaults to `Monday`). Ignored when `group_by` is `Day`. Refer to [`SummaryWeekStart`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub week_start: Option<SummaryWeekStart>,
}

/// Serving definition for a logged food item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggedServing {
    /// Catalog serving identifier.
    pub id: String,

    /// Positive unit amount represented by this serving (max 10000). Null when no usable size exists.
    pub quantity: Option<f64>,

    /// Serving unit label (1 to 64 chars). Null when unprovided by producer.
    pub unit: Option<String>,

    /// Weight in grams of one catalog serving definition. Null when unknown.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight_grams: Option<f64>,
}

/// Food item entry within a food log.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggedFood {
    /// Food identifier from search or analysis results.
    pub food_id: String,

    /// Food name. Null only when upstream provides none.
    pub name: Option<String>,

    /// Brand name. Null for generic foods.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand_name: Option<String>,

    /// Picture URL of the food item. Null when none.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,

    /// Glycemic index value. Null when unavailable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub glycemic_index: Option<f64>,

    /// Glycemic load value. Null when unavailable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub glycemic_load: Option<f64>,

    /// Nutritional breakdown scaled to consumed portion. Refer to [`Nutrients`].
    pub nutrients: Nutrients,

    /// Number of selected servings consumed. Null when unavailable.
    pub quantity: Option<f64>,

    /// Catalog serving definition. Refer to [`LoggedServing`].
    pub serving: LoggedServing,
}

/// Food log record containing logged foods and metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FoodLog {
    /// Log identifier. Null only when upstream sends a log with no id.
    pub id: Option<String>,

    /// Foods recorded in this log entry. Refer to [`LoggedFood`].
    pub foods: Vec<LoggedFood>,

    /// ISO 8601 date-time timestamp in UTC with milliseconds.
    pub created_at: String,

    /// Log entry name. Null when no name was given.
    pub name: Option<String>,
}

/// Response payload containing food logs for a specified date range.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListFoodLogsResponse {
    /// Logs in the requested range, ordered by timestamp. Empty array is valid. Refer to [`FoodLog`].
    pub items: Vec<FoodLog>,
}

/// Summary bucket covering a single day or week range.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FoodLogSummaryBucket {
    /// First local calendar date covered by this bucket.
    pub start_date: String,

    /// Last local calendar date covered by this bucket, inclusive.
    pub end_date: String,

    /// Number of logs in this bucket.
    pub logs_count: u64,

    /// Number of distinct local calendar dates in this bucket with at least one log.
    pub days_with_logs: u64,

    /// Aggregated nutrient breakdown for this bucket. Refer to [`Nutrients`].
    pub nutrients: Nutrients,
}

/// Overall totals across the entire summarized date range.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FoodLogSummaryTotals {
    /// Total logs in the range.
    pub logs_count: u64,

    /// Total distinct local calendar dates in the range with at least one log.
    pub days_with_logs: u64,

    /// Total aggregated nutrients. Refer to [`Nutrients`].
    pub nutrients: Nutrients,
}

/// Average daily nutrient totals across logged days.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FoodLogSummaryAverage {
    /// Average nutrients per logged day. Refer to [`Nutrients`].
    pub nutrients: Nutrients,
}

/// Aggregated food log summary response across a date range.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FoodLogSummaryResponse {
    /// Bucket size used for aggregation. Refer to [`SummaryGroupBy`].
    pub group_by: SummaryGroupBy,

    /// Weekday week buckets begin on. Null when `group_by` is `Day`. Refer to [`SummaryWeekStart`].
    pub week_start: Option<SummaryWeekStart>,

    /// Canonical spelling of requested IANA timezone.
    pub timezone: String,

    /// First local calendar date of the summarized range.
    pub start_date: String,

    /// Last local calendar date of the summarized range, inclusive.
    pub end_date: String,

    /// Buckets tiling the range in chronological order. Refer to [`FoodLogSummaryBucket`].
    pub buckets: Vec<FoodLogSummaryBucket>,

    /// Totals for the entire summarized range. Refer to [`FoodLogSummaryTotals`].
    pub totals: FoodLogSummaryTotals,

    /// Average daily nutrition breakdown across logged days. Refer to [`FoodLogSummaryAverage`].
    pub average_per_logged_day: FoodLogSummaryAverage,
}

// Water logs

/// Supported measurement units for logging water intake.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WaterUnit {
    /// Fluid ounces (1 to 811.5 fl oz).
    #[serde(rename = "fl_oz")]
    FlOz,
    /// Milliliters (30 to 24000 ml).
    Ml,
    /// Cups (0.1 to 101.4 cups).
    Cup,
}

/// Quantity and measurement unit of water intake.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaterAmount {
    /// Volume value consumed (0.1 to 24000 depending on unit).
    pub value: f64,

    /// Measurement unit for the volume value. Refer to [`WaterUnit`].
    pub unit: WaterUnit,
}

/// Request payload to log water consumption for a user.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogWaterRequest {
    /// Volume and unit of water consumed. Refer to [`WaterAmount`].
    pub amount: WaterAmount,

    /// ISO 8601 date-time timestamp when water was consumed. Omitted defaults to current time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
}

/// Water intake log entry details.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaterLog {
    /// Log entry identifier used to delete the log.
    pub id: String,

    /// Volume and unit of water logged. Refer to [`WaterAmount`].
    pub amount: WaterAmount,

    /// ISO 8601 date-time timestamp when water was consumed in UTC with milliseconds.
    pub created_at: String,
}

/// Query parameters for listing water intake logs over a date range.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListWaterLogsQuery {
    /// First local calendar date in timezone, inclusive (e.g., `"2026-09-01"`). At most 5 years before today.
    pub start_date: String,

    /// Last local calendar date in timezone, inclusive (e.g., `"2026-09-10"`). Returns up to 100 most recent days with logs.
    pub end_date: String,

    /// IANA timezone identifier defining the local calendar days (e.g., `"America/Los_Angeles"`).
    pub timezone: String,

    /// Measurement unit for returned daily totals. Refer to [`WaterUnit`].
    pub unit: WaterUnit,
}

/// Aggregated total water consumption for a single local calendar date.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyWaterTotal {
    /// Local calendar date in the requested timezone (e.g., `"2026-09-10"`).
    pub date: String,

    /// Total volume consumed on this date in the requested unit. Refer to [`WaterAmount`].
    pub total: WaterAmount,
}

/// Response payload containing daily water intake totals across a requested date range.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListWaterLogsResponse {
    /// Daily totals for days with water logged, ordered oldest first. Days with no logs are omitted. Refer to [`DailyWaterTotal`].
    pub items: Vec<DailyWaterTotal>,
}

// Weight logs

/// Supported measurement units for logging weight.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WeightUnit {
    /// Pounds (2 to 1500 lb).
    #[serde(rename = "lb")]
    Lb,
    /// Kilograms (1 to 700 kg).
    #[serde(rename = "kg")]
    Kg,
}

/// Weight quantity and measurement unit.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Weight {
    /// Weight value (1 to 1500 depending on unit).
    pub value: f64,

    /// Measurement unit for the weight value. Refer to [`WeightUnit`].
    pub unit: WeightUnit,
}

/// Request payload to log a weight measurement for a user.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogWeightRequest {
    /// Weight measurement value and unit. Refer to [`Weight`].
    pub weight: Weight,

    /// ISO 8601 date-time timestamp when weight was measured. Omitted defaults to current time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
}

/// Response payload for a logged weight entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogWeightResponse {
    /// Stored weight measurement and unit. Refer to [`Weight`].
    pub weight: Weight,

    /// ISO 8601 date-time timestamp when weight was measured in UTC with milliseconds.
    pub created_at: String,
}

/// Query parameters for retrieving weight logs over a date range.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListWeightLogsQuery {
    /// First local calendar date in timezone, inclusive (e.g., `"2024-09-01"`). At most 5 years before today.
    pub start_date: String,

    /// Last local calendar date in timezone, inclusive (e.g., `"2024-09-08"`). Returns up to 100 most recent days with weight logs.
    pub end_date: String,

    /// IANA timezone identifier defining the local calendar days (e.g., `"America/Los_Angeles"`).
    pub timezone: String,
}

/// Recorded weight measurement for a single local calendar date.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyWeight {
    /// Local calendar date in the requested timezone (e.g., `"2026-09-10"`).
    pub date: String,

    /// Weight measurement value and unit recorded on this date. Refer to [`Weight`].
    pub weight: Weight,
}

/// Response payload containing daily weight records across a requested date range.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListWeightLogsResponse {
    /// Daily weight entries for days with recorded measurements, ordered oldest first. Days with no records are omitted. Refer to [`DailyWeight`].
    pub items: Vec<DailyWeight>,
}
