use std::sync::Arc;

use chrono::Utc;
use sea_orm::ActiveValue::Set;
use uuid::Uuid;

use crate::{
    error::{AppError, Result},
    models::{user_provider::{self, ProviderType}, provider_model},
    repositories::provider_repo::ProviderRepo,
    utils::ToUuidV7,
};

#[derive(Clone)]
pub struct UserProviderService {
    pub repo: Arc<ProviderRepo>,
}

impl UserProviderService {
    pub fn new(repo: Arc<ProviderRepo>) -> Self { Self { repo } }

    pub async fn list(&self, user_id: Uuid) -> Result<Vec<user_provider::Model>> {
        self.repo.list_by_user_id(user_id).await
    }

    pub async fn get(&self, user_id: Uuid, id: Uuid) -> Result<user_provider::Model> {
        match self.repo.get_by_id_for_user(user_id, id).await? {
            Some(model) => Ok(model),
            None => Err(AppError::NotFound("Provider not found".to_string())),
        }
    }

    pub async fn create(
        &self,
        user_id: Uuid,
        name: String,
        provider_type: ProviderType,
        url: String,
        key: Option<String>,
    ) -> Result<user_provider::Model> {
        if self.repo.get_by_name_for_user(user_id, &name).await?.is_some() {
            return Err(AppError::Conflict("Provider name already exists".to_string()));
        }

        let id = Utc::now().to_uuid_v7();

        let active = user_provider::ActiveModel {
            id: Set(id),
            user_id: Set(user_id),
            name: Set(name),
            provider_type: Set(provider_type),
            url: Set(url),
            key: Set(key),
            ..Default::default()
        };

        self.repo.insert(active).await
    }

    pub async fn update(
        &self,
        user_id: Uuid,
        id: Uuid,
        name: Option<String>,
        provider_type: Option<ProviderType>,
        url: Option<String>,
        key: Option<Option<String>>,
    ) -> Result<user_provider::Model> {
        let current = match self.repo.get_by_id_for_user(user_id, id).await? {
            Some(m) => m,
            None => return Err(AppError::NotFound("Provider not found".to_string())),
        };

        if let Some(ref new_name) = name {
            if &current.name != new_name && self.repo.get_by_name_for_user(user_id, new_name).await?.is_some() {
                return Err(AppError::Conflict("Provider name already exists".to_string()));
            }
        }

        let mut active: user_provider::ActiveModel = current.into();
        if let Some(name) = name { active.name = Set(name); }
        if let Some(t) = provider_type { active.provider_type = Set(t); }
        if let Some(url) = url { active.url = Set(url); }
        if let Some(maybe_key) = key { active.key = Set(maybe_key); }

        self.repo.update(active).await
    }

    pub async fn delete(&self, user_id: Uuid, id: Uuid) -> Result<()> {
        let res = self.repo.delete_by_id_for_user(user_id, id).await?;
        if res.rows_affected == 0 {
            Err(AppError::NotFound("Provider not found".to_string()))
        } else {
            Ok(())
        }
    }

    pub async fn list_with_models(&self, user_id: Uuid) -> Result<Vec<(user_provider::Model, Vec<provider_model::Model>)>> {
        self.repo.list_with_models_by_user_id(user_id).await
    }

    pub async fn get_with_models(&self, user_id: Uuid, id: Uuid) -> Result<(user_provider::Model, Vec<provider_model::Model>)> {
        match self.repo.get_with_models_for_user(user_id, id).await? {
            Some(pair) => Ok(pair),
            None => Err(AppError::NotFound("Provider not found".to_string())),
        }
    }
}