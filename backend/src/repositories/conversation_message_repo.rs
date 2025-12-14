use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, DeleteResult, QueryOrder, TransactionTrait};
use sea_orm::ActiveValue::Set;
use uuid::Uuid;
use chrono::Utc;

use crate::{error::{AppError, Result}, models::conversation_message::{self, ChatRole}, utils::ToUuidV7};

pub struct ConversationMessageRepo {
    pub pool: DatabaseConnection,
}

impl ConversationMessageRepo {
    pub fn new(pool: DatabaseConnection) -> Self { Self { pool } }

    pub async fn create_pair(&self, session_id: Uuid, user_content: String, assistant_content: String) -> Result<conversation_message::Model> {
        let txn = self.pool.begin().await.map_err(AppError::from)?;
        
        let user_msg_id = Utc::now().to_uuid_v7();
        let user_msg = conversation_message::ActiveModel {
            id: Set(user_msg_id),
            session_id: Set(session_id),
            role: Set(ChatRole::User),
            content: Set(user_content),
            ..Default::default()
        };
        let _ = user_msg.insert(&txn).await.map_err(AppError::from)?;

        let asst_id = Utc::now().to_uuid_v7();
        let asst_msg = conversation_message::ActiveModel {
            id: Set(asst_id),
            session_id: Set(session_id),
            role: Set(ChatRole::Assistant),
            content: Set(assistant_content),
            ..Default::default()
        };
        let saved = asst_msg.insert(&txn).await.map_err(AppError::from)?;
        
        txn.commit().await.map_err(AppError::from)?;
        Ok(saved)
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