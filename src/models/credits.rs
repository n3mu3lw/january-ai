use serde::{Deserialize, Serialize};

/// Plan tier associated with an account credit allowance.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PlanType {
    /// Free plan
    Free,

    /// Pro plan
    Pro,

    /// Startup plan
    Startup,

    /// Enterprise plan
    Enterprise,

    /// Unlimited plan: Partner with no ceiling
    Unlimited,
}

/// Account credit balance and billing period details.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditsResponse {
    /// Plan tier associated with the allowance. Refer to [`PlanType`].
    pub plan: PlanType,

    /// First day of the current billing period in UTC (inclusive, ISO 8601 date).
    pub period_start: String,

    /// Last day of the current billing period in UTC (inclusive, ISO 8601 date).
    pub period_end: String,

    /// Instant when the period ends and credits reset (ISO 8601 date-time).
    pub resets_at: String,

    /// Credits consumed during the current billing period.
    pub used_credits: u64,

    /// Total credits included in the plan for this period. Null for uncapped plans.
    pub included_credits: Option<u64>,

    /// Credits remaining in the current period. Null for uncapped plans.
    pub remaining_credits: Option<u64>,
}
