use futures::Stream;
use futures::StreamExt;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use uuid::Uuid;

use crate::{
    clients::llm_client::{ChatMessagePayload, LlmClient},
    error::{AppError, Result},
    models::{
        conversation_message::{self, ChatRole},
        conversation_session,
    },
    repositories::{
        conversation_message_repo::ConversationMessageRepo,
        conversation_session_repo::ConversationSessionRepo, provider_model_repo::ProviderModelRepo,
        provider_repo::ProviderRepo,
    },
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
        Self {
            session_repo,
            message_repo,
            provider_model_repo,
            provider_repo,
            llm_client,
        }
    }

    pub async fn list_sessions(&self, user_id: Uuid) -> Result<Vec<conversation_session::Model>> {
        self.session_repo.list_by_user(user_id).await
    }

    pub async fn create_session(&self, user_id: Uuid) -> Result<conversation_session::Model> {
        self.session_repo.create(user_id).await
    }

    pub async fn list_messages(
        &self,
        user_id: Uuid,
        session_id: Uuid,
    ) -> Result<Vec<conversation_message::Model>> {
        let session = self
            .session_repo
            .get_by_id(session_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Session not found".to_string()))?;
        if session.user_id != user_id {
            return Err(AppError::Forbidden("Session not accessible".to_string()));
        }
        self.message_repo.list_by_session(session_id).await
    }

    pub async fn send_message(
        &self,
        user_id: Uuid,
        session_id: Uuid,
        content: String,
        provider_model_id: Uuid,
    ) -> Result<impl Stream<Item = Result<String>>> {
        let session = self
            .session_repo
            .get_by_id(session_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Session not found".to_string()))?;
        if session.user_id != user_id {
            return Err(AppError::Forbidden("Session not accessible".to_string()));
        }

        let model = self
            .provider_model_repo
            .get_by_id(provider_model_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Model not found".to_string()))?;
        let provider = self
            .provider_repo
            .get_by_id_for_user(user_id, model.provider_id)
            .await?
            .ok_or_else(|| AppError::Forbidden("Provider not accessible".to_string()))?;

        // Build chat history
        let history = self.message_repo.list_by_session(session.id).await?;
        let mut messages_payload: Vec<ChatMessagePayload> = history
            .into_iter()
            .map(|m| ChatMessagePayload {
                role: m.role.as_str().to_string(),
                content: m.content,
            })
            .collect();
        messages_payload.push(ChatMessagePayload {
            role: ChatRole::User.as_str().to_string(),
            content: content.clone(),
        });

        // Call LLM
        let mut llm_stream = self
            .llm_client
            .chat(&provider, &model.model_id, messages_payload)
            .await?;

        let (tx, rx) = mpsc::unbounded_channel();

        let message_repo = self.message_repo.clone();
        let session_repo = self.session_repo.clone();
        let llm_client_for_title = self.llm_client.clone();
        let provider_for_title = provider.clone();
        let model_id_for_title = model.model_id.clone();
        let content_for_save = content.clone();
        let session_id = session.id;
        let session_title_is_none = session.title.is_none();

        tokio::spawn(async move {
            let mut full_response = String::new();

            while let Some(item) = llm_stream.next().await {
                match item {
                    Ok(chunk) => {
                        full_response.push_str(&chunk);
                        if tx.send(Ok(chunk)).is_err() {
                            // TODO
                        }
                    }
                    Err(e) => {
                        let _ = tx.send(Err(e));
                        // TODO
                    }
                }
            }

            // Save to DB
            if !full_response.is_empty() {
                // We don't need the result here since we are in background task, but log error if needed
                let _ = message_repo
                    .create_pair(session_id, content_for_save.clone(), full_response)
                    .await;
            }

            // Handle Title Generation (after message is done)
            if session_title_is_none {
                let title_messages = vec![
                    ChatMessagePayload { role: "system".to_string(), content: "You are a conversation title assistant. Based on the user's message below, generate a short, clear title (max 20 characters). Do not include quotes or periods.".to_string() },
                    ChatMessagePayload { role: "user".to_string(), content: content_for_save },
                ];
                // We use stream chat but just collect it since we don't have a non-streaming client anymore
                if let Ok(mut stream) = llm_client_for_title
                    .chat(&provider_for_title, &model_id_for_title, title_messages)
                    .await
                {
                    let mut title_text = String::new();
                    while let Some(res) = stream.next().await {
                        if let Ok(chunk) = res {
                            title_text.push_str(&chunk);
                        }
                    }
                    title_text = title_text.trim().to_string();
                    if !title_text.is_empty() {
                        let _ = session_repo.update_title(session_id, title_text).await;
                    }
                }
            }
        });

        Ok(UnboundedReceiverStream::new(rx))
    }

    pub async fn delete_session(&self, user_id: Uuid, session_id: Uuid) -> Result<()> {
        let session = self
            .session_repo
            .get_by_id(session_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Session not found".to_string()))?;
        if session.user_id != user_id {
            return Err(AppError::Forbidden("Session not accessible".to_string()));
        }
        let res = self.session_repo.delete_by_id(session_id).await?;
        if res.rows_affected == 0 {
            Err(AppError::NotFound("Session not found".to_string()))
        } else {
            Ok(())
        }
    }
}
