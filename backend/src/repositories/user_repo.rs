use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use uuid::Uuid;

use crate::error::{AppError, Result};
use crate::models::user;

pub struct UserRepo {
    pub pool: DatabaseConnection
}

impl UserRepo {
    pub fn new(pool: DatabaseConnection) -> Self {
        Self { pool }
    }

    pub async fn get_user_by_id(&self, id: Uuid) -> Result<Option<user::Model>> {
        user::Entity::find()
            .filter(user::Column::Id.eq(id))
            .one(&self.pool)
            .await
            .map_err(AppError::from)
    }

    pub async fn get_user_by_email(&self, email: &str) -> Result<Option<user::Model>> {
        user::Entity::find()
            .filter(user::Column::Email.eq(email))
            .one(&self.pool)
            .await
            .map_err(AppError::from)
    }

    pub async fn get_user_by_name(&self, name: &str) -> Result<Option<user::Model>> {
        user::Entity::find()
            .filter(user::Column::Name.eq(name))
            .one(&self.pool)
            .await
            .map_err(AppError::from)
    }

    pub async fn insert(&self, model: user::ActiveModel) -> Result<user::Model> {
        model.insert(&self.pool).await.map_err(AppError::from)
    }

    pub async fn update(&self, model: user::ActiveModel) -> Result<user::Model> {
        model.update(&self.pool).await.map_err(AppError::from)
    }
}