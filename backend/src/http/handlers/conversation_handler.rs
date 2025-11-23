use std::sync::Arc;

use axum::{Json, extract::{Path, State}};
use uuid::Uuid;
use validator::Validate;

use crate::{
    error::Result,
    http::{dto::{
        common_schema::ApiResponse,
        conversation_schema::*,
    }, extractors::jwt::AuthUser},
    models::conversation_message,
    services::conversation_service::ConversationService,
};

pub async fn list_conversations(
    AuthUser(claims): AuthUser,
    State(service): State<Arc<ConversationService>>,
) -> Result<Json<ApiResponse<ConversationSessionsResponse>>> {
    let items = service.list_sessions(claims.sub).await?;
    Ok(Json(ApiResponse::success(Some(ConversationSessionsResponse { items }), None::<String>)))
}

pub async fn create_conversation(
    AuthUser(claims): AuthUser,
    State(service): State<Arc<ConversationService>>,
    Json(request): Json<CreateConversationRequest>,
) -> Result<Json<ApiResponse<ConversationResponse>>> {
    let _ = request; // empty body
    let created = service.create_session(claims.sub).await?;
    Ok(Json(ApiResponse::success(Some(ConversationResponse { id: created.id, items: vec![] }), Some("Conversation created"))))
}

pub async fn list_messages(
    AuthUser(claims): AuthUser,
    State(service): State<Arc<ConversationService>>,
    Path(session_id): Path<Uuid>,
) -> Result<Json<ApiResponse<ConversationResponse>>> {
    let items = service.list_messages(claims.sub, session_id).await?;
    Ok(Json(ApiResponse::success(Some(ConversationResponse { id: session_id, items }), None::<String>)))
}

pub async fn send_message(
    AuthUser(claims): AuthUser,
    State(service): State<Arc<ConversationService>>,
    Path(session_id): Path<Uuid>,
    Json(request): Json<SendMessageRequest>,
) -> Result<Json<ApiResponse<conversation_message::Model>>> {
    request.validate()?;
    let saved = service.send_message(claims.sub, session_id, request.content, request.provider_model_id).await?;
    Ok(Json(ApiResponse::success(Some(saved), Some("Message sent"))))
}

pub async fn delete_conversation(
    AuthUser(claims): AuthUser,
    State(service): State<Arc<ConversationService>>,
    Path(session_id): Path<Uuid>,
) -> Result<Json<ApiResponse<ConversationResponse>>> {
    service.delete_session(claims.sub, session_id).await?;
    Ok(Json(ApiResponse::success(Some(ConversationResponse { id: session_id, items: vec![] }), Some("Conversation deleted"))))
}