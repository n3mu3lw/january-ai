use anyhow::{Result, anyhow};
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE, HeaderMap, HeaderValue};
use serde::Deserialize;
use serde::de::DeserializeOwned;

pub mod models;
pub use models::*;

const BASE_URL: &str = "https://partners.january.ai/v1.2";

#[derive(Debug, Deserialize)]
pub struct APIError {
    pub code: String,
    pub message: String,
}

pub struct JanuaryAI {
    client: reqwest::Client,
    base_url: String,
}

impl JanuaryAI {
    pub fn new(api_key: impl Into<String>) -> Result<Self> {
        let key_str = api_key.into();
        let mut headers = HeaderMap::new();

        let auth_val = format!("Bearer {}", key_str);
        headers.insert(AUTHORIZATION, HeaderValue::from_str(&auth_val)?);
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;

        Ok(Self {
            client,
            base_url: BASE_URL.to_string(),
        })
    }

    async fn handle_response<T: DeserializeOwned>(&self, res: reqwest::Response) -> Result<T> {
        let status = res.status();
        let text = res.text().await?;

        if status.is_success() {
            serde_json::from_str::<T>(&text).map_err(|err| {
                anyhow!("Failed to decode response JSON: {err}. Raw payload: {text}")
            })
        } else {
            let err = serde_json::from_str::<APIError>(&text).unwrap_or(APIError {
                code: "unknown_error".to_string(),
                message: text,
            });
            Err(anyhow!("API Error [{}]: {}", err.code, err.message))
        }
    }

    // Auth Endpoints

    pub async fn mint_client_token(
        &self,
        req: &MintClientTokenRequest,
    ) -> Result<MintClientTokenResponse> {
        let url = format!("{}/auth/client-tokens", self.base_url);
        let res = self.client.post(&url).json(req).send().await?;
        self.handle_response(res).await
    }

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

    // Foods Endpoints

    /// GET /v1.2/foods
    pub async fn search_foods(&self, query: &SearchFoodsQuery) -> Result<SearchFoodsResponse> {
        let url = format!("{}/foods", self.base_url);
        let res = self.client.get(&url).query(query).send().await?;
        self.handle_response(res).await
    }

    /// GET /v1.2/foods/autocomplete
    pub async fn autocomplete(&self, query: &AutocompleteQuery) -> Result<AutocompleteResponse> {
        let url = format!("{}/foods/autocomplete", self.base_url);
        let res = self.client.get(&url).query(query).send().await?;
        self.handle_response(res).await
    }

    /// GET /v1.2/foods/barcode/{barcode}
    pub async fn get_food_by_barcode(&self, barcode: &str) -> Result<FoodItem> {
        let url = format!("{}/foods/barcode/{}", self.base_url, barcode);
        let res = self.client.get(&url).send().await?;
        self.handle_response(res).await
    }

    /// GET /v1.2/foods/{food_id}
    pub async fn get_food_details(&self, food_id: &str) -> Result<FoodItem> {
        let url = format!("{}/foods/{}", self.base_url, food_id);
        let res = self.client.get(&url).send().await?;
        self.handle_response(res).await
    }

    /// POST /v1.2/foods/{food_id}/alternatives
    pub async fn get_healthier_alternatives(
        &self,
        food_id: &str,
        req: &HealthierAlternativesRequest,
    ) -> Result<HealthierAlternativesResponse> {
        let url = format!("{}/foods/{}/alternatives", self.base_url, food_id);
        let res = self.client.post(&url).json(req).send().await?;
        self.handle_response(res).await
    }

    // Food Analysis Endpoints

    /// POST /v1.2/food-analysis/image
    pub async fn analyze_food_image(
        &self,
        req: &AnalyzeImageRequest,
    ) -> Result<FoodAnalysisResponse> {
        let url = format!("{}/food-analysis/image", self.base_url);
        let res = self.client.post(&url).json(req).send().await?;
        self.handle_response(res).await
    }

    /// POST /v1.2/food-analysis/text
    pub async fn analyze_food_text(
        &self,
        req: &AnalyzeTextRequest,
    ) -> Result<FoodAnalysisResponse> {
        let url = format!("{}/food-analysis/text", self.base_url);
        let res = self.client.post(&url).json(req).send().await?;
        self.handle_response(res).await
    }

    /// POST /v1.2/food-analysis/corrections
    pub async fn correct_food_analysis(
        &self,
        req: &CorrectionRequest,
    ) -> Result<FoodAnalysisResponse> {
        let url = format!("{}/food-analysis/corrections", self.base_url);
        let res = self.client.post(&url).json(req).send().await?;
        self.handle_response(res).await
    }

    /// Create a food log entry for an end user (`POST /v1.2/food-logs`).
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

    /// List food logs for an end user in a date range (`GET /v1.2/food-logs`).
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

    /// Aggregate food logs over a date range (`GET /v1.2/food-logs/summary`).
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

    /// Get a specific food log by ID (`GET /v1.2/food-logs/{log_id}`).
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

    /// Update an existing food log entry (`PATCH /v1.2/food-logs/{log_id}`).
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

    /// Delete a food log entry (`DELETE /v1.2/food-logs/{log_id}`).
    /// Deleting is idempotent and returns 204 No Content.
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
            let error_text = res.text().await?;
            Err(anyhow::anyhow!("Failed to delete food log: {}", error_text))
        }
    }

    /// Record an amount of water intake for an end user (`POST /v1.2/water-logs`).
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

    /// List a user's daily water totals in a date range (`GET /v1.2/water-logs`).
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

    /// Delete a water log entry by ID (`DELETE /v1.2/water-logs/{log_id}`).
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
            let error_text = res.text().await?;
            Err(anyhow::anyhow!(
                "Failed to delete water log: {}",
                error_text
            ))
        }
    }

    /// Record a weight measurement for an end user (`POST /v1.2/weight-logs`).
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

    /// List a user's daily weights in a date range (`GET /v1.2/weight-logs`).
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

    /// Predict the glucose response to a meal (`POST /v1.2/glucose/predictions`).
    pub async fn predict_glucose(
        &self,
        req: &PredictGlucoseRequest,
    ) -> Result<PredictGlucoseResponse> {
        let url = format!("{}/glucose/predictions", self.base_url);
        let res = self.client.post(&url).json(req).send().await?;

        self.handle_response(res).await
    }

    /// Search restaurants matching query around (latitude, longitude), ranked by proximity.
    /// Costs 2 credits per successful call.
    pub async fn search_restaurants(
        &self,
        params: &RestaurantSearchQuery,
    ) -> Result<RestaurantListResponse> {
        let url = format!("{}/restaurants", self.base_url);
        let res = self.client.get(&url).query(params).send().await?;

        self.handle_response(res).await
    }

    /// Search dishes across restaurants near (latitude, longitude).
    /// Costs 2 credits per successful call.
    pub async fn search_menu_items(
        &self,
        params: &MenuItemSearchQuery,
    ) -> Result<MenuItemListResponse> {
        let url = format!("{}/menu-items", self.base_url);
        let res = self.client.get(&url).query(params).send().await?;

        self.handle_response(res).await
    }

    /// Get the menu of a single restaurant by `restaurant_id`.
    /// Costs 2 credits per successful call.
    pub async fn get_restaurant_menu(
        &self,
        restaurant_id: &str,
        params: &RestaurantMenuItemsQuery,
    ) -> Result<RestaurantMenuItemListResponse> {
        let url = format!("{}/restaurants/{}/menu-items", self.base_url, restaurant_id);
        let res = self.client.get(&url).query(params).send().await?;

        self.handle_response(res).await
    }

    /// Retrieve current account credit balance and rate limit quota (`GET /v1.2/credits`).
    ///
    /// Note: This endpoint requires an API Key (`sk-...`) and refuses Client Tokens (`ct-...`).
    /// It costs 0 credits and succeeds even if your credit allowance is exhausted.
    pub async fn get_credits(&self) -> Result<CreditsResponse> {
        let url = format!("{}/credits", self.base_url);
        let res = self.client.get(&url).send().await?;

        self.handle_response(res).await
    }
}
