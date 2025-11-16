use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, DeleteResult};
use uuid::Uuid;

use crate::{error::{AppError, Result}, models::provider_model};

pub struct ProviderModelRepo {
    pub pool: DatabaseConnection,
}

impl ProviderModelRepo {
    pub fn new(pool: DatabaseConnection) -> Self { Self { pool } }

    pub async fn list_by_provider(&self, provider_id: Uuid) -> Result<Vec<provider_model::Model>> {
        provider_model::Entity::find()
            .filter(provider_model::Column::ProviderId.eq(provider_id))
            .all(&self.pool)
            .await
            .map_err(AppError::from)
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<Option<provider_model::Model>> {
        provider_model::Entity::find()
            .filter(provider_model::Column::Id.eq(id))
            .one(&self.pool)
            .await
            .map_err(AppError::from)
    }

    pub async fn get_by_model_id_in_provider(&self, provider_id: Uuid, model_id: &str) -> Result<Option<provider_model::Model>> {
        provider_model::Entity::find()
            .filter(provider_model::Column::ProviderId.eq(provider_id))
            .filter(provider_model::Column::ModelId.eq(model_id))
            .one(&self.pool)
            .await
            .map_err(AppError::from)
    }

    pub async fn insert(&self, model: provider_model::ActiveModel) -> Result<provider_model::Model> {
        model.insert(&self.pool).await.map_err(AppError::from)
    }

    pub async fn update(&self, model: provider_model::ActiveModel) -> Result<provider_model::Model> {
        model.update(&self.pool).await.map_err(AppError::from)
    }

    pub async fn delete_by_id(&self, id: Uuid) -> Result<DeleteResult> {
        provider_model::Entity::delete_many()
            .filter(provider_model::Column::Id.eq(id))
            .exec(&self.pool)
            .await
            .map_err(AppError::from)
    }
}