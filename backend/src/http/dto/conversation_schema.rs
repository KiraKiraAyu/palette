use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::models::{conversation_message, conversation_session};

#[derive(Debug, Clone, Serialize)]
pub struct ConversationSessionsResponse {
    pub items: Vec<conversation_session::Model>,
}

#[derive(Debug, Clone, Validate, Deserialize)]
pub struct CreateConversationRequest {}

#[derive(Debug, Clone, Validate, Deserialize)]
pub struct SendMessageRequest {
    #[validate(length(min = 1))]
    pub content: String,
    pub provider_model_id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct ConversationResponse {
    pub id: Uuid,
    pub items: Vec<conversation_message::Model>,
}