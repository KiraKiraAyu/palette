use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, DeleteResult};
use uuid::Uuid;

use crate::{error::{AppError, Result}, models::user_provider};

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

    pub async fn insert(&self, model: user_provider::ActiveModel) -> Result<user_provider::Model> {
        model.insert(&self.pool).await.map_err(AppError::from)
    }

    pub async fn update(&self, model: user_provider::ActiveModel) -> Result<user_provider::Model> {
        model.update(&self.pool).await.map_err(AppError::from)
    }

    pub async fn delete_by_id_for_user(&self, user_id: Uuid, id: Uuid) -> Result<DeleteResult> {
        user_provider::Entity::delete_many()
            .filter(user_provider::Column::UserId.eq(user_id))
            .filter(user_provider::Column::Id.eq(id))
            .exec(&self.pool)
            .await
            .map_err(AppError::from)
    }
}