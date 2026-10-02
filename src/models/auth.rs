use serde::{Deserialize, Serialize};

use super::common::*;

/// Request payload for minting a short-lived end-user client token.
///
/// Follow the least privilege principle and request only the minimum required [`Scope`]s.
#[derive(Debug, Serialize)]
pub struct MintClientTokenRequest {
    /// Stable unique identifier for the end user (max 64 chars).
    pub end_user_id: String,

    /// Permission scopes granted to the token (1 to 10 items). Refer to [`Scope`].
    pub scopes: Vec<Scope>,

    /// Token validity duration in seconds (300–7200, defaults to 1800).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl_seconds: Option<u32>,
}

/// Response returned after successfully minting a client token.
#[derive(Debug, Deserialize)]
pub struct MintClientTokenResponse {
    /// The client credential string (e.g., `"ct-..."`). Shown **only once**.
    pub token: String,

    /// Validity remaining in seconds. Prefer calculating expiration from this over `expires_at` to avoid clock drift.
    pub expires_in: u32,

    /// Absolute expiration timestamp in UTC (ISO 8601).
    pub expires_at: String,

    /// Echoed end user ID bound to this token.
    pub end_user_id: String,

    /// List of granted permission [`Scope`]s.
    pub scopes: Vec<Scope>,
}

/// Request payload for revoking all active client tokens for a user.
#[derive(Debug, Serialize)]
pub struct RevokeClientTokensRequest {
    /// End user ID whose outstanding tokens should be revoked (max 64 chars).
    pub end_user_id: String,
}

/// Response payload from revoking client tokens.
#[derive(Debug, Deserialize)]
pub struct RevokeClientTokensResponse {
    /// Number of active tokens revoked by this request. Already-expired or revoked tokens are excluded.
    pub revoked_count: u32,
}
