use std::sync::Arc;

use chrono::Utc;
use rust_decimal::Decimal;
use sea_orm::ActiveValue::Set;
use uuid::Uuid;

use crate::{
    error::{AppError, Result},
    models::{provider_model, user_provider},
    repositories::{provider_model_repo::ProviderModelRepo, provider_repo::ProviderRepo},
    utils::ToUuidV7,
    clients::pricing_client::PricingClient,
};

#[derive(Clone)]
pub struct ProviderModelService {
    pub model_repo: Arc<ProviderModelRepo>,
    pub provider_repo: Arc<ProviderRepo>,
    pub pricing_client: Arc<dyn PricingClient>,
}

impl ProviderModelService {
    pub fn new(model_repo: Arc<ProviderModelRepo>, provider_repo: Arc<ProviderRepo>, pricing_client: Arc<dyn PricingClient>) -> Self {
        Self { model_repo, provider_repo, pricing_client }
    }

    pub async fn list(&self, user_id: Uuid, provider_id: Uuid) -> Result<Vec<provider_model::Model>> {
        self.ensure_provider_owned_by(user_id, provider_id).await?;
        self.model_repo.list_by_provider(provider_id).await
    }

    pub async fn get(&self, user_id: Uuid, id: Uuid) -> Result<provider_model::Model> {
        let model = self.model_repo.get_by_id(id).await?;
        let Some(model) = model else { return Err(AppError::NotFound("Model not found".to_string())) };
        self.ensure_provider_owned_by(user_id, model.provider_id).await?;
        Ok(model)
    }

    pub async fn create(
        &self,
        user_id: Uuid,
        provider_id: Uuid,
        model_id: String,
        name: String,
    ) -> Result<provider_model::Model> {
        self.ensure_provider_owned_by(user_id, provider_id).await?;

        if self.model_repo.get_by_model_id_in_provider(provider_id, &model_id).await?.is_some() {
            return Err(AppError::Conflict("Model ID already exists in provider".to_string()));
        }

        // 获取价格
        let provider = self.provider_repo.get_by_id_for_user(user_id, provider_id).await?
            .ok_or_else(|| AppError::NotFound("Provider not found".to_string()))?;
        let (input_price_per_million, output_price_per_million) = self.pricing_client.fetch_prices(&provider, &model_id).await?;

        let id = Utc::now().to_uuid_v7();
        let active = provider_model::ActiveModel {
            id: Set(id),
            provider_id: Set(provider_id),
            model_id: Set(model_id),
            name: Set(name),
            input_price_per_million: Set(input_price_per_million),
            output_price_per_million: Set(output_price_per_million),
            ..Default::default()
        };
        self.model_repo.insert(active).await
    }

    pub async fn update(
        &self,
        user_id: Uuid,
        id: Uuid,
        model_id: Option<String>,
        name: Option<String>,
    ) -> Result<provider_model::Model> {
        let current = self.get(user_id, id).await?;

        if let Some(ref new_model_id) = model_id {
            if new_model_id != &current.model_id {
                if self.model_repo.get_by_model_id_in_provider(current.provider_id, new_model_id).await?.is_some() {
                    return Err(AppError::Conflict("Model ID already exists in provider".to_string()));
                }
            }
        }

        let mut active: provider_model::ActiveModel = current.into();
        let new_model_id_opt = model_id.clone();
        if let Some(v) = new_model_id_opt.clone() { active.model_id = Set(v); }
        if let Some(v) = name { active.name = Set(v); }

        // 如果模型ID发生变化，刷新价格
        if let Some(ref new_model_id) = new_model_id_opt {
            let provider = self.provider_repo.get_by_id_for_user(user_id, active.provider_id.clone().unwrap()).await?
                .ok_or_else(|| AppError::NotFound("Provider not found".to_string()))?;
            let (in_price, out_price) = self.pricing_client.fetch_prices(&provider, new_model_id).await?;
            active.input_price_per_million = Set(in_price);
            active.output_price_per_million = Set(out_price);
        }

        self.model_repo.update(active).await
    }

    pub async fn delete(&self, user_id: Uuid, id: Uuid) -> Result<()> {
        let model = self.get(user_id, id).await?;
        let res = self.model_repo.delete_by_id(model.id).await?;
        if res.rows_affected == 0 { Err(AppError::NotFound("Model not found".to_string())) } else { Ok(()) }
    }

    async fn ensure_provider_owned_by(&self, user_id: Uuid, provider_id: Uuid) -> Result<user_provider::Model> {
        let provider = self.provider_repo.get_by_id_for_user(user_id, provider_id).await?;
        match provider { Some(p) => Ok(p), None => Err(AppError::Forbidden("Provider not accessible".to_string())) }
    }
}