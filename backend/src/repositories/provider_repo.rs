use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, DeleteResult};
use sea_orm::ActiveValue::Set;
use uuid::Uuid;
use chrono::Utc;

use crate::{error::{AppError, Result}, models::{user_provider::{self, ProviderType}, provider_model}, utils::ToUuidV7};

pub struct ProviderRepo {
    pub pool: DatabaseConnection,
}

impl ProviderRepo {
    pub fn new(pool: DatabaseConnection) -> Self { Self { pool } }

    pub async fn list_by_user_id(&self, user_id: Uuid) -> Result<Vec<user_provider::Model>> {
        user_provider::Entity::find()
            .filter(user_provider::Column::UserId.eq(user_id))
            .all(&self.pool)
            .await
            .map_err(AppError::from)
    }

    pub async fn get_by_id_for_user(&self, user_id: Uuid, id: Uuid) -> Result<Option<user_provider::Model>> {
        user_provider::Entity::find()
            .filter(user_provider::Column::UserId.eq(user_id))
            .filter(user_provider::Column::Id.eq(id))
            .one(&self.pool)
            .await
            .map_err(AppError::from)
    }

    pub async fn get_by_name_for_user(&self, user_id: Uuid, name: &str) -> Result<Option<user_provider::Model>> {
        user_provider::Entity::find()
            .filter(user_provider::Column::UserId.eq(user_id))
            .filter(user_provider::Column::Name.eq(name))
            .one(&self.pool)
            .await
            .map_err(AppError::from)
    }

    pub async fn create(
        &self,
        user_id: Uuid,
        name: String,
        provider_type: ProviderType,
        url: String,
        key: Option<String>,
    ) -> Result<user_provider::Model> {
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
        active.insert(&self.pool).await.map_err(AppError::from)
    }

    pub async fn update_provider(
        &self,
        id: Uuid,
        name: Option<String>,
        provider_type: Option<ProviderType>,
        url: Option<String>,
        key: Option<Option<String>>,
    ) -> Result<user_provider::Model> {
        let mut active = user_provider::ActiveModel {
            id: Set(id),
            ..Default::default()
        };
        if let Some(v) = name { active.name = Set(v); }
        if let Some(v) = provider_type { active.provider_type = Set(v); }
        if let Some(v) = url { active.url = Set(v); }
        if let Some(v) = key { active.key = Set(v); }

        active.update(&self.pool).await.map_err(AppError::from)
    }

    pub async fn delete_by_id_for_user(&self, user_id: Uuid, id: Uuid) -> Result<DeleteResult> {
        user_provider::Entity::delete_many()
            .filter(user_provider::Column::UserId.eq(user_id))
            .filter(user_provider::Column::Id.eq(id))
            .exec(&self.pool)
            .await
            .map_err(AppError::from)
    }

    pub async fn list_with_models_by_user_id(&self, user_id: Uuid) -> Result<Vec<(user_provider::Model, Vec<provider_model::Model>)>> {
        user_provider::Entity::find()
            .filter(user_provider::Column::UserId.eq(user_id))
            .find_with_related(provider_model::Entity)
            .all(&self.pool)
            .await
            .map_err(AppError::from)
    }

    pub async fn get_with_models_for_user(&self, user_id: Uuid, id: Uuid) -> Result<Option<(user_provider::Model, Vec<provider_model::Model>)>> {
        let mut items = user_provider::Entity::find()
            .filter(user_provider::Column::UserId.eq(user_id))
            .filter(user_provider::Column::Id.eq(id))
            .find_with_related(provider_model::Entity)
            .all(&self.pool)
            .await
            .map_err(AppError::from)?;
        Ok(items.pop())
    }
}