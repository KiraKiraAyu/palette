use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, DeleteResult, QueryOrder};
use uuid::Uuid;

use crate::{error::{AppError, Result}, models::conversation_message};

pub struct ConversationMessageRepo {
    pub pool: DatabaseConnection,
}

impl ConversationMessageRepo {
    pub fn new(pool: DatabaseConnection) -> Self { Self { pool } }

    pub async fn insert(&self, model: conversation_message::ActiveModel) -> Result<conversation_message::Model> {
        model.insert(&self.pool).await.map_err(AppError::from)
    }

    pub async fn list_by_session(&self, session_id: Uuid) -> Result<Vec<conversation_message::Model>> {
        conversation_message::Entity::find()
            .filter(conversation_message::Column::SessionId.eq(session_id))
            .order_by_asc(conversation_message::Column::CreatedAt)
            .all(&self.pool)
            .await
            .map_err(AppError::from)
    }

    pub async fn delete_by_session(&self, session_id: Uuid) -> Result<DeleteResult> {
        conversation_message::Entity::delete_many()
            .filter(conversation_message::Column::SessionId.eq(session_id))
            .exec(&self.pool)
            .await
            .map_err(AppError::from)
    }
}