use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, DeleteResult};
use sea_orm::ActiveValue::Set;
use uuid::Uuid;
use chrono::Utc;
use rust_decimal::Decimal;

use crate::{error::{AppError, Result}, models::provider_model, utils::ToUuidV7};

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

    pub async fn create(
        &self,
        provider_id: Uuid,
        model_id: String,
        name: String,
        input_price: Decimal,
        output_price: Decimal,
    ) -> Result<provider_model::Model> {
        let id = Utc::now().to_uuid_v7();
        let active = provider_model::ActiveModel {
            id: Set(id),
            provider_id: Set(provider_id),
            model_id: Set(model_id),
            name: Set(name),
            input_price_per_million: Set(input_price),
            output_price_per_million: Set(output_price),
            ..Default::default()
        };
        active.insert(&self.pool).await.map_err(AppError::from)
    }

    pub async fn update_model(
        &self,
        id: Uuid,
        model_id: Option<String>,
        name: Option<String>,
        input_price: Option<Decimal>,
        output_price: Option<Decimal>,
    ) -> Result<provider_model::Model> {
        let mut active = provider_model::ActiveModel {
            id: Set(id),
            ..Default::default()
        };
        if let Some(v) = model_id { active.model_id = Set(v); }
        if let Some(v) = name { active.name = Set(v); }
        if let Some(v) = input_price { active.input_price_per_million = Set(v); }
        if let Some(v) = output_price { active.output_price_per_million = Set(v); }
        
        active.update(&self.pool).await.map_err(AppError::from)
    }

    pub async fn delete_by_id(&self, id: Uuid) -> Result<DeleteResult> {
        provider_model::Entity::delete_many()
            .filter(provider_model::Column::Id.eq(id))
            .exec(&self.pool)
            .await
            .map_err(AppError::from)
    }
}