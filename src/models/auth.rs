use serde::{Deserialize, Serialize};

use super::common::*;

#[derive(Debug, Serialize)]
pub struct MintClientTokenRequest {
    pub end_user_id: String,
    pub scopes: Vec<Scope>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl_seconds: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct MintClientTokenResponse {
    pub token: String,
    pub expires_in: u32,
    pub expires_at: String,
    pub end_user_id: String,
    pub scopes: Vec<Scope>,
}

#[derive(Debug, Serialize)]
pub struct RevokeClientTokensRequest {
    pub end_user_id: String,
}

#[derive(Debug, Deserialize)]
pub struct RevokeClientTokensResponse {
    pub revoked_count: u32,
}
