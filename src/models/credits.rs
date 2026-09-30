use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlanType {
    Free,
    Pro,
    Startup,
    Enterprise,
    Unlimited,
    #[serde(untagged)]
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditsResponse {
    pub plan: PlanType,
    pub period_start: String,
    pub period_end: String,
    pub resets_at: String,
    pub used_credits: u64,
    pub included_credits: Option<u64>,
    pub remaining_credits: Option<u64>,
}
