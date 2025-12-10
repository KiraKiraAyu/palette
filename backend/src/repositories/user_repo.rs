use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use sea_orm::ActiveValue::Set;
use uuid::Uuid;
use chrono::Utc;

use crate::error::{AppError, Result};
use crate::models::user;
use crate::utils::ToUuidV7;

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

    pub async fn create(&self, email: String, name: String, password_hash: String) -> Result<user::Model> {
        let id = Utc::now().to_uuid_v7();
        let active_model = user::ActiveModel {
            id: Set(id),
            email: Set(email),
            name: Set(name),
            password_hash: Set(password_hash),
            avatar: Set(None),
            ..Default::default()
        };
        active_model.insert(&self.pool).await.map_err(AppError::from)
    }
}