//! Unofficial Rust SDK for January AI

#![deny(missing_docs)]

use reqwest::header::{AUTHORIZATION, CONTENT_TYPE, HeaderMap, HeaderValue};
use serde::Deserialize;
use serde::de::DeserializeOwned;
use thiserror::Error;

pub mod models;
pub use models::*;

const BASE_URL: &str = "https://partners.january.ai/v1.2";

/// The primary error type returned by all `january-ai` SDK operations.
#[derive(Error, Debug)]
pub enum Error {
    /// January AI returned an HTTP error response with a JSON error payload.
    #[error("API Error [{code}]: {message}")]
    Api {
        /// Stable error code (e.g., `"invalid_request"`).
        code: String,
        /// Human-readable description.
        message: String,
    },

    /// Network or HTTP transport failure.
    #[error("HTTP transport error: {0}")]
    Reqwest(#[from] reqwest::Error),

    /// Response payload deserialization failure.
    #[error("Failed to decode response JSON: {err}. Raw payload: {payload}")]
    JsonDecode {
        /// Serde error.
        err: serde_json::Error,
        /// Raw body string.
        payload: String,
    },
}

/// Specialized Result type for January AI SDK operations.
pub type Result<T> = std::result::Result<T, Error>;

// internal helper struct for deserializing error payloads
#[derive(Debug, Deserialize)]
struct APIError {
    #[serde(default)]
    code: String,

    #[serde(default)]
    message: String,
}

/// The main SDK entrypoint for interacting with the January AI Partner API.
pub struct JanuaryAI {
    client: reqwest::Client,
    base_url: String,
}

impl JanuaryAI {
    /// Constructs a new `JanuaryAI` client with the specified API key.
    ///
    /// The API key is attached as a `Bearer` authorization token for all HTTP requests.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your January AI secret API key (e.g., `"sk_..."`).
    ///
    /// # Errors
    ///
    /// Returns an error if the API key contains invalid HTTP header characters or if
    /// the underlying `reqwest::Client` fails to build.
    ///
    /// # Example
    ///
    /// ```rust
    /// use january_ai::JanuaryAI;
    ///
    /// let client = JanuaryAI::new("sk_test_123456789");
    /// assert!(client.is_ok());
    /// ```
    pub fn new(api_key: impl Into<String>) -> Result<Self> {
        let key_str = api_key.into();
        let mut headers = HeaderMap::new();

        let auth_val = format!("Bearer {}", key_str);
        let header_val = HeaderValue::from_str(&auth_val).map_err(|e| Error::Api {
            code: "invalid_api_key".to_string(),
            message: format!("Invalid character in API key: {e}"),
        })?;
        headers.insert(AUTHORIZATION, header_val);
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;

        Ok(Self {
            client,
            base_url: BASE_URL.to_string(),
        })
    }

    /// Overrides the default API base URL (useful for testing or staging).
    pub fn with_base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into();
        self
    }

    /// Internal helper method to process HTTP responses and deserialize JSON bodies.
    async fn handle_response<T: DeserializeOwned>(&self, res: reqwest::Response) -> Result<T> {
        let status = res.status();
        let text = res.text().await?;

        if status.is_success() {
            serde_json::from_str::<T>(&text).map_err(|err| Error::JsonDecode { err, payload: text })
        } else {
            let err = serde_json::from_str::<APIError>(&text).unwrap_or(APIError {
                code: "unknown_error".to_string(),
                message: text,
            });

            Err(Error::Api {
                code: err.code,
                message: err.message,
            })
        }
    }

    // ========================================================================
    // Auth Endpoints
    // ========================================================================

    /// Mints a short-lived client token (`ct-...`) for end-user client SDK authentication (`POST /auth/client-tokens`).
    ///
    /// Client tokens allow frontend applications or mobile apps to make requests safely on behalf of a specific user.
    ///
    /// # Arguments
    ///
    /// * `req` - Reference to a [`MintClientTokenRequest`] specifying thetarget `end_user_id` and authorized
    /// * [`Scope`] permissions.
    pub async fn mint_client_token(
        &self,
        req: &MintClientTokenRequest,
    ) -> Result<MintClientTokenResponse> {
        let url = format!("{}/auth/client-tokens", self.base_url);
        let res = self.client.post(&url).json(req).send().await?;
        self.handle_response(res).await
    }

    /// Revokes active client tokens associated with a given end user (`POST /auth/client-token-revocations`).
    ///
    /// # Arguments
    ///
    /// * `end_user_id` - Unique identifier of the end user whose client tokens should be revoked.
    pub async fn revoke_client_tokens(
        &self,
        end_user_id: impl Into<String>,
    ) -> Result<RevokeClientTokensResponse> {
        let url = format!("{}/auth/client-token-revocations", self.base_url);
        let req = RevokeClientTokensRequest {
            end_user_id: end_user_id.into(),
        };

        let res = self.client.post(&url).json(&req).send().await?;
        self.handle_response(res).await
    }

    // ========================================================================
    // Foods Endpoints
    // ========================================================================

    /// Searches the January AI food database using text queries (`GET /foods`).
    ///
    /// # Arguments
    ///
    /// * `query` - Reference to [`SearchFoodsQuery`] containing query string and pagination options.
    pub async fn search_foods(&self, query: &SearchFoodsQuery) -> Result<SearchFoodsResponse> {
        let url = format!("{}/foods", self.base_url);
        let res = self.client.get(&url).query(query).send().await?;
        self.handle_response(res).await
    }

    /// Provides autocompletion suggestions for fast, real-time food search UI inputs (`GET /foods/autocomplete`).
    ///
    /// # Arguments
    ///
    /// * `query` - Reference to [`AutocompleteQuery`] with partial text inputs.
    pub async fn autocomplete(&self, query: &AutocompleteQuery) -> Result<AutocompleteResponse> {
        let url = format!("{}/foods/autocomplete", self.base_url);
        let res = self.client.get(&url).query(query).send().await?;
        self.handle_response(res).await
    }

    /// Fetches detailed nutritional information for a product by its UPC/EAN barcode (`GET /foods/barcode/{barcode}`).
    ///
    /// # Arguments
    ///
    /// * `barcode` - Standard numerical barcode string.
    pub async fn get_food_by_barcode(&self, barcode: &str) -> Result<FoodItem> {
        let url = format!("{}/foods/barcode/{}", self.base_url, barcode);
        let res = self.client.get(&url).send().await?;
        self.handle_response(res).await
    }

    /// Retrieves comprehensive nutritional metadata for a specific food item by its ID (`GET /foods/{food_id}`).
    ///
    /// # Arguments
    ///
    /// * `food_id` - Unique January AI food item identifier.
    pub async fn get_food_details(&self, food_id: &str) -> Result<FoodItem> {
        let url = format!("{}/foods/{}", self.base_url, food_id);
        let res = self.client.get(&url).send().await?;
        self.handle_response(res).await
    }

    /// Suggest healthier alternatives for a food (`POST /foods/{food_id}/alternatives`).
    ///
    /// # Arguments
    ///
    /// * `food_id` - ID of the target food item to find alternatives for.
    /// * `req` - Reference to [`HealthierAlternativesRequest`] specifying user preferences and/or restrictions.
    pub async fn get_healthier_alternatives(
        &self,
        food_id: &str,
        req: &HealthierAlternativesRequest,
    ) -> Result<HealthierAlternativesResponse> {
        let url = format!("{}/foods/{}/alternatives", self.base_url, food_id);
        let res = self.client.post(&url).json(req).send().await?;
        self.handle_response(res).await
    }

    // ========================================================================
    // Food Analysis Endpoints
    // ========================================================================

    /// Analyzes a meal photograph to estimate ingredients, portion sizes, and macronutrient content (`POST /food-analysis/image`).
    ///
    /// # Arguments
    ///
    /// * `req` - Reference to [`AnalyzeImageRequest`] containing base64 image data or image URL.
    pub async fn analyze_food_image(
        &self,
        req: &AnalyzeImageRequest,
    ) -> Result<FoodAnalysisResponse> {
        let url = format!("{}/food-analysis/image", self.base_url);
        let res = self.client.post(&url).json(req).send().await?;
        self.handle_response(res).await
    }

    /// Analyzes natural language meal descriptions to extract structured food items and macronutrients (`POST /food-analysis/text`).
    ///
    /// # Arguments
    ///
    /// * `req` - Reference to [`AnalyzeTextRequest`] containing freeform text (e.g., *"2 scrambled eggs with avocado and sourdough toast"*).
    pub async fn analyze_food_text(
        &self,
        req: &AnalyzeTextRequest,
    ) -> Result<FoodAnalysisResponse> {
        let url = format!("{}/food-analysis/text", self.base_url);
        let res = self.client.post(&url).json(req).send().await?;
        self.handle_response(res).await
    }

    /// Submits user corrections to a previously generated AI food analysis to refine accuracy (`POST /food-analysis/corrections`).
    ///
    /// # Arguments
    ///
    /// * `req` - Reference to [`CorrectionRequest`] detailing modified item quantities or replacements.
    pub async fn correct_food_analysis(
        &self,
        req: &CorrectionRequest,
    ) -> Result<FoodAnalysisResponse> {
        let url = format!("{}/food-analysis/corrections", self.base_url);
        let res = self.client.post(&url).json(req).send().await?;
        self.handle_response(res).await
    }

    // ========================================================================
    // Food Logging Endpoints
    // ========================================================================

    /// Creates a new food log entry for a specific end user (`POST /food-logs`).
    ///
    /// Sends the `January-End-User-ID` request header.
    ///
    /// # Arguments
    ///
    /// * `end_user_id` - The unique identifier of the end user.
    /// * `req` - Reference to [`CreateFoodLogRequest`] containing meal components and log timestamp.
    pub async fn create_food_log(
        &self,
        end_user_id: &str,
        req: &CreateFoodLogRequest,
    ) -> Result<FoodLog> {
        let url = format!("{}/food-logs", self.base_url);
        let res = self
            .client
            .post(&url)
            .header("January-End-User-ID", end_user_id)
            .json(req)
            .send()
            .await?;

        self.handle_response(res).await
    }

    /// Lists historical food logs for an end user across a specified date range (`GET /food-logs`).
    ///
    /// # Arguments
    ///
    /// * `end_user_id` - The unique identifier of the end user.
    /// * `query` - Reference to [`ListFoodLogsQuery`] containing start/end timestamps and pagination parameters.
    pub async fn list_food_logs(
        &self,
        end_user_id: &str,
        query: &ListFoodLogsQuery,
    ) -> Result<ListFoodLogsResponse> {
        let url = format!("{}/food-logs", self.base_url);
        let res = self
            .client
            .get(&url)
            .header("January-End-User-ID", end_user_id)
            .query(query)
            .send()
            .await?;

        self.handle_response(res).await
    }

    /// Summarizes a user's food logs over a date range (`GET /food-logs/summary`).
    ///
    /// # Arguments
    ///
    /// * `end_user_id` - The unique identifier of the end user.
    /// * `query` - Reference to [`FoodLogSummaryQuery`] with date filtering options.
    pub async fn get_food_log_summary(
        &self,
        end_user_id: &str,
        query: &FoodLogSummaryQuery,
    ) -> Result<FoodLogSummaryResponse> {
        let url = format!("{}/food-logs/summary", self.base_url);
        let res = self
            .client
            .get(&url)
            .header("January-End-User-ID", end_user_id)
            .query(query)
            .send()
            .await?;

        self.handle_response(res).await
    }

    /// Fetches a specific food log entry by its log ID (`GET /food-logs/{log_id}`).
    ///
    /// # Arguments
    ///
    /// * `end_user_id` - The unique identifier of the end user owning the log.
    /// * `log_id` - Unique identifier of the specific food log.
    pub async fn get_food_log(&self, end_user_id: &str, log_id: &str) -> Result<FoodLog> {
        let url = format!("{}/food-logs/{}", self.base_url, log_id);
        let res = self
            .client
            .get(&url)
            .header("January-End-User-ID", end_user_id)
            .send()
            .await?;

        self.handle_response(res).await
    }

    /// Updates an existing food log entry (`PATCH /food-logs/{log_id}`).
    ///
    /// # Arguments
    ///
    /// * `end_user_id` - The unique identifier of the end user.
    /// * `log_id` - Unique identifier of the food log to update.
    /// * `req` - Reference to [`UpdateFoodLogRequest`] containing updated fields.
    pub async fn update_food_log(
        &self,
        end_user_id: &str,
        log_id: &str,
        req: &UpdateFoodLogRequest,
    ) -> Result<FoodLog> {
        let url = format!("{}/food-logs/{}", self.base_url, log_id);
        let res = self
            .client
            .patch(&url)
            .header("January-End-User-ID", end_user_id)
            .json(req)
            .send()
            .await?;

        self.handle_response(res).await
    }

    /// Deletes a food log entry (`DELETE /food-logs/{log_id}`).
    ///
    /// Deleting is idempotent and returns `204 No Content` on success.
    ///
    /// # Arguments
    ///
    /// * `end_user_id` - The unique identifier of the end user.
    /// * `log_id` - Unique identifier of the food log to remove.
    pub async fn delete_food_log(&self, end_user_id: &str, log_id: &str) -> Result<()> {
        let url = format!("{}/food-logs/{}", self.base_url, log_id);
        let res = self
            .client
            .delete(&url)
            .header("January-End-User-ID", end_user_id)
            .send()
            .await?;

        if res.status().is_success() {
            Ok(())
        } else {
            self.handle_response::<serde_json::Value>(res)
                .await
                .map(|_| ())
        }
    }

    // ========================================================================
    // Water Logging Endpoints
    // ========================================================================

    /// Logs water for a user (`POST /water-logs`).
    ///
    /// # Arguments
    ///
    /// * `end_user_id` - The unique identifier of the end user.
    /// * `req` - Reference to [`LogWaterRequest`] containing water volume and unit.
    pub async fn log_water(&self, end_user_id: &str, req: &LogWaterRequest) -> Result<WaterLog> {
        let url = format!("{}/water-logs", self.base_url);
        let res = self
            .client
            .post(&url)
            .header("January-End-User-ID", end_user_id)
            .json(req)
            .send()
            .await?;

        self.handle_response(res).await
    }

    /// Lists daily water totals for a user across a date range (`GET /water-logs`).
    ///
    /// # Arguments
    ///
    /// * `end_user_id` - The unique identifier of the end user.
    /// * `query` - Reference to [`ListWaterLogsQuery`].
    pub async fn list_water_logs(
        &self,
        end_user_id: &str,
        query: &ListWaterLogsQuery,
    ) -> Result<ListWaterLogsResponse> {
        let url = format!("{}/water-logs", self.base_url);
        let res = self
            .client
            .get(&url)
            .header("January-End-User-ID", end_user_id)
            .query(query)
            .send()
            .await?;

        self.handle_response(res).await
    }

    /// Deletes a water log entry by ID (`DELETE /water-logs/{log_id}`).
    ///
    /// # Arguments
    ///
    /// * `end_user_id` - The unique identifier of the end user.
    /// * `log_id` - Unique identifier of the water log entry.
    pub async fn delete_water_log(&self, end_user_id: &str, log_id: &str) -> Result<()> {
        let url = format!("{}/water-logs/{}", self.base_url, log_id);
        let res = self
            .client
            .delete(&url)
            .header("January-End-User-ID", end_user_id)
            .send()
            .await?;

        if res.status().is_success() {
            Ok(())
        } else {
            self.handle_response::<serde_json::Value>(res)
                .await
                .map(|_| ())
        }
    }

    // ========================================================================
    // Weight Logging Endpoints
    // ========================================================================

    /// Logs a weight measurement for an end user (`POST /weight-logs`).
    ///
    /// # Arguments
    ///
    /// * `end_user_id` - The unique identifier of the end user.
    /// * `req` - Reference to [`LogWeightRequest`].
    pub async fn log_weight(
        &self,
        end_user_id: &str,
        req: &LogWeightRequest,
    ) -> Result<LogWeightResponse> {
        let url = format!("{}/weight-logs", self.base_url);
        let res = self
            .client
            .post(&url)
            .header("January-End-User-ID", end_user_id)
            .json(req)
            .send()
            .await?;

        self.handle_response(res).await
    }

    /// Lists a user's weight measurement history in a date range (`GET /weight-logs`).
    ///
    /// # Arguments
    ///
    /// * `end_user_id` - The unique identifier of the end user.
    /// * `query` - Reference to [`ListWeightLogsQuery`].
    pub async fn list_weight_logs(
        &self,
        end_user_id: &str,
        query: &ListWeightLogsQuery,
    ) -> Result<ListWeightLogsResponse> {
        let url = format!("{}/weight-logs", self.base_url);
        let res = self
            .client
            .get(&url)
            .header("January-End-User-ID", end_user_id)
            .query(query)
            .send()
            .await?;

        self.handle_response(res).await
    }

    // ========================================================================
    // Glucose Prediction Endpoints
    // ========================================================================

    /// Predicts the postprandial glucose curve response for a planned meal (`POST /glucose/predictions`).
    ///
    /// # Arguments
    ///
    /// * `req` - Reference to [`PredictGlucoseRequest`] containing food item lists or macro compositions.
    pub async fn predict_glucose(
        &self,
        req: &PredictGlucoseRequest,
    ) -> Result<PredictGlucoseResponse> {
        let url = format!("{}/glucose/predictions", self.base_url);
        let res = self.client.post(&url).json(req).send().await?;

        self.handle_response(res).await
    }

    // ========================================================================
    // Restaurant & Menu Endpoints
    // ========================================================================

    /// Searches restaurants around geographic coordinates, ranked by distance (`GET /restaurants`).
    ///
    /// **Costs 2 credits per successful request.**
    ///
    /// # Arguments
    ///
    /// * `params` - Reference to [`RestaurantSearchQuery`] containing latitude, longitude, and search terms.
    pub async fn search_restaurants(
        &self,
        params: &RestaurantSearchQuery,
    ) -> Result<RestaurantListResponse> {
        let url = format!("{}/restaurants", self.base_url);
        let res = self.client.get(&url).query(params).send().await?;

        self.handle_response(res).await
    }

    /// Searches menu items near a location (`GET /menu-items`).
    ///
    /// **Costs 2 credits per successful request.**
    ///
    /// # Arguments
    ///
    /// * `params` - Reference to [`MenuItemSearchQuery`].
    pub async fn search_menu_items(
        &self,
        params: &MenuItemSearchQuery,
    ) -> Result<MenuItemListResponse> {
        let url = format!("{}/menu-items", self.base_url);
        let res = self.client.get(&url).query(params).send().await?;

        self.handle_response(res).await
    }

    /// Lists a restaurant's menu items (`GET /restaurants/{restaurant_id}/menu-items`).
    ///
    /// **Costs 2 credits per successful request.**
    ///
    /// # Arguments
    ///
    /// * `restaurant_id` - Unique identifier of the target restaurant.
    /// * `params` - Reference to [`RestaurantMenuItemsQuery`].
    pub async fn get_restaurant_menu(
        &self,
        restaurant_id: &str,
        params: &RestaurantMenuItemsQuery,
    ) -> Result<RestaurantMenuItemListResponse> {
        let url = format!("{}/restaurants/{}/menu-items", self.base_url, restaurant_id);
        let res = self.client.get(&url).query(params).send().await?;

        self.handle_response(res).await
    }

    // ========================================================================
    // Credit Account Endpoints
    // ========================================================================

    /// Retrieves current account credit balance and API rate limits (`GET /credits`).
    ///
    /// **Note:** Requires a master API Key (`sk-...`); refuses Client Tokens (`ct-...`).
    /// Costs **0 credits** and succeeds even if your credit allocation is depleted.
    pub async fn get_credits(&self) -> Result<CreditsResponse> {
        let url = format!("{}/credits", self.base_url);
        let res = self.client.get(&url).send().await?;

        self.handle_response(res).await
    }
}
