use async_trait::async_trait;
use std::time::Duration;
use rust_decimal::Decimal;
use serde::Deserialize;

use crate::{
    error::{AppError, Result},
    models::user_provider::Model as ProviderModel,
};

#[async_trait]
pub trait ModelInfoClient: Send + Sync {
    async fn fetch_prices(&self, provider: &ProviderModel, model_id: &str) -> Result<(Decimal, Decimal)>;
}

#[derive(Clone)]
pub struct DefaultModelInfoClient {
    http: reqwest::Client,
}

impl Default for DefaultModelInfoClient {
    fn default() -> Self {
        let http = reqwest::Client::builder()
            .timeout(Duration::from_secs(5))
            .build()
            .unwrap_or_else(|_| reqwest::Client::new());
        Self { http }
    }
}

#[derive(Debug, Deserialize)]
struct PricingResponse {
    input_price_per_million: Decimal,
    output_price_per_million: Decimal,
}

#[async_trait]
impl ModelInfoClient for DefaultModelInfoClient {
    async fn fetch_prices(&self, provider: &ProviderModel, model_id: &str) -> Result<(Decimal, Decimal)> {
        let base = provider.url.trim_end_matches('/');
        let candidates = vec![
            format!("{}/pricing/models/{}", base, model_id),
            format!("{}/v1/pricing/models/{}", base, model_id),
        ];

        let key = provider.key.clone();
        for url in candidates {
            let mut req = self.http.get(&url);
            if let Some(k) = key.clone() {
                req = req.bearer_auth(k);
            }
            let resp = req.send().await;
            if let Ok(r) = resp {
                if r.status().is_success() {
                    let parsed: PricingResponse = r
                        .json()
                        .await
                        .map_err(|e| AppError::Internal(format!("Failed to parse pricing response: {}", e)))?;
                    return Ok((parsed.input_price_per_million, parsed.output_price_per_million));
                }
            }
        }

        Err(AppError::BadRequest("Cannot fetch model prices from provider".to_string()))
    }
}