use std::sync::Arc;
use uuid::Uuid;

use crate::{
    error::{AppError, Result},
    models::{provider_model, user_provider},
    repositories::{provider_model_repo::ProviderModelRepo, provider_repo::ProviderRepo},
    clients::model_info_client::ModelInfoClient,
};

#[derive(Clone)]
pub struct ProviderModelService {
    pub model_repo: Arc<ProviderModelRepo>,
    pub provider_repo: Arc<ProviderRepo>,
    pub model_info_client: Arc<dyn ModelInfoClient>,
}

impl ProviderModelService {
    pub fn new(model_repo: Arc<ProviderModelRepo>, provider_repo: Arc<ProviderRepo>, model_info_client: Arc<dyn ModelInfoClient>) -> Self {
        Self { model_repo, provider_repo, model_info_client }
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

        let provider = self.provider_repo.get_by_id_for_user(user_id, provider_id).await?
            .ok_or_else(|| AppError::NotFound("Provider not found".to_string()))?;
        let (input_price_per_million, output_price_per_million) = self.model_info_client.fetch_prices(&provider, &model_id).await?;

        self.model_repo.create(provider_id, model_id, name, input_price_per_million, output_price_per_million).await
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

        let mut input_price_per_million = None;
        let mut output_price_per_million = None;

        // Refresh price if model ID changed
        if let Some(ref new_model_id) = model_id {
            let provider = self.provider_repo.get_by_id_for_user(user_id, current.provider_id).await?
                .ok_or_else(|| AppError::NotFound("Provider not found".to_string()))?;
            let (in_price, out_price) = self.model_info_client.fetch_prices(&provider, new_model_id).await?;
            input_price_per_million = Some(in_price);
            output_price_per_million = Some(out_price);
        }

        self.model_repo.update_model(id, model_id, name, input_price_per_million, output_price_per_million).await
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