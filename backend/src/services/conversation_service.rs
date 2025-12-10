use std::sync::Arc;
use uuid::Uuid;

use crate::{
    error::{AppError, Result},
    models::{conversation_session, conversation_message::{self, ChatRole}},
    repositories::{
        conversation_session_repo::ConversationSessionRepo,
        conversation_message_repo::ConversationMessageRepo,
        provider_model_repo::ProviderModelRepo,
        provider_repo::ProviderRepo,
    },
    clients::llm_client::{ChatMessagePayload, LlmClient},
};

#[derive(Clone)]
pub struct ConversationService {
    pub session_repo: Arc<ConversationSessionRepo>,
    pub message_repo: Arc<ConversationMessageRepo>,
    pub provider_model_repo: Arc<ProviderModelRepo>,
    pub provider_repo: Arc<ProviderRepo>,
    pub llm_client: Arc<dyn LlmClient>,
}

impl ConversationService {
    pub fn new(
        session_repo: Arc<ConversationSessionRepo>,
        message_repo: Arc<ConversationMessageRepo>,
        provider_model_repo: Arc<ProviderModelRepo>,
        provider_repo: Arc<ProviderRepo>,
        llm_client: Arc<dyn LlmClient>,
    ) -> Self {
        Self { session_repo, message_repo, provider_model_repo, provider_repo, llm_client }
    }

    pub async fn list_sessions(&self, user_id: Uuid) -> Result<Vec<conversation_session::Model>> {
        self.session_repo.list_by_user(user_id).await
    }

    pub async fn create_session(&self, user_id: Uuid) -> Result<conversation_session::Model> {
        self.session_repo.create(user_id).await
    }

    pub async fn list_messages(&self, user_id: Uuid, session_id: Uuid) -> Result<Vec<conversation_message::Model>> {
        let session = self.session_repo.get_by_id(session_id).await?
            .ok_or_else(|| AppError::NotFound("Session not found".to_string()))?;
        if session.user_id != user_id { return Err(AppError::Forbidden("Session not accessible".to_string())); }
        self.message_repo.list_by_session(session_id).await
    }

    pub async fn send_message(&self, user_id: Uuid, session_id: Uuid, content: String, provider_model_id: Uuid) -> Result<conversation_message::Model> {
        let session = self.session_repo.get_by_id(session_id).await?
            .ok_or_else(|| AppError::NotFound("Session not found".to_string()))?;
        if session.user_id != user_id { return Err(AppError::Forbidden("Session not accessible".to_string())); }

        let model = self.provider_model_repo.get_by_id(provider_model_id).await?
            .ok_or_else(|| AppError::NotFound("Model not found".to_string()))?;
        let provider = self.provider_repo.get_by_id_for_user(user_id, model.provider_id).await?
            .ok_or_else(|| AppError::Forbidden("Provider not accessible".to_string()))?;

        // Build chat history
        let history = self.message_repo.list_by_session(session.id).await?;
        let mut messages_payload: Vec<ChatMessagePayload> = history.into_iter()
            .map(|m| ChatMessagePayload { role: m.role.as_str().to_string(), content: m.content })
            .collect();
        messages_payload.push(ChatMessagePayload { role: ChatRole::User.as_str().to_string(), content: content.clone() });

        // Call LLM
        let assistant_text = self.llm_client.chat(&provider, &model.model_id, messages_payload).await?;

        // Persist user & assistant message atomically
        let saved = self.message_repo.create_pair(session.id, content.clone(), assistant_text).await?;

        // If this is the first message and session has no title, generate a title
        if session.title.is_none() {
            let title_messages = vec![
                ChatMessagePayload { role: "system".to_string(), content: "You are a conversation title assistant. Based on the user's message below, generate a short, clear title (max 20 characters). Do not include quotes or periods.".to_string() },
                ChatMessagePayload { role: "user".to_string(), content: content.clone() },
            ];
            if let Ok(mut title_text) = self.llm_client.chat(&provider, &model.model_id, title_messages).await {
                title_text = title_text.trim().to_string();
                let _ = self.session_repo.update_title(session.id, title_text).await?;
            }
        }
        Ok(saved)
    }

    pub async fn delete_session(&self, user_id: Uuid, session_id: Uuid) -> Result<()> {
        let session = self.session_repo.get_by_id(session_id).await?
            .ok_or_else(|| AppError::NotFound("Session not found".to_string()))?;
        if session.user_id != user_id { return Err(AppError::Forbidden("Session not accessible".to_string())); }
        let res = self.session_repo.delete_by_id(session_id).await?;
        if res.rows_affected == 0 { Err(AppError::NotFound("Session not found".to_string())) } else { Ok(()) }
    }
}