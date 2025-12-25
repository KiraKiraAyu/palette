use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, DeleteResult};
use sea_orm::ActiveValue::Set;
use uuid::Uuid;
use chrono::Utc;

use crate::{error::{AppError, Result}, models::conversation_session, utils::ToUuidV7};

pub struct ConversationSessionRepo {
    pub pool: DatabaseConnection,
}

impl ConversationSessionRepo {
    pub fn new(pool: DatabaseConnection) -> Self { Self { pool } }

    pub async fn create(&self, user_id: Uuid) -> Result<conversation_session::Model> {
        let id = Utc::now().to_uuid_v7();
        let active = conversation_session::ActiveModel {
            id: Set(id),
            user_id: Set(user_id),
            ..Default::default()
        };
        active.insert(&self.pool).await.map_err(AppError::from)
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<Option<conversation_session::Model>> {
        conversation_session::Entity::find()
            .filter(conversation_session::Column::Id.eq(id))
            .one(&self.pool)
            .await
            .map_err(AppError::from)
    }

    pub async fn list_by_user(&self, user_id: Uuid) -> Result<Vec<conversation_session::Model>> {
        conversation_session::Entity::find()
            .filter(conversation_session::Column::UserId.eq(user_id))
            .all(&self.pool)
            .await
            .map_err(AppError::from)
    }

    pub async fn delete_by_id(&self, id: Uuid) -> Result<DeleteResult> {
        conversation_session::Entity::delete_many()
            .filter(conversation_session::Column::Id.eq(id))
            .exec(&self.pool)
            .await
            .map_err(AppError::from)
    }

    pub async fn update_title(&self, id: Uuid, title: String) -> Result<conversation_session::Model> {
        let active = conversation_session::ActiveModel {
            id: Set(id),
            title: Set(Some(title)),
            ..Default::default()
        };
        active.update(&self.pool).await.map_err(AppError::from)
    }
}