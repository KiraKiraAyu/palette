use axum::{
    Router,
    routing::{get, post, put, delete}
};

use crate::{
    http::handlers::{auth_handler, user_provider_handler, provider_model_handler, conversation_handler},
    state::AppState,
};

pub fn create_routes() -> Router<AppState> {
    Router::new()
        // User Authentication
        .route("/api/auth/register", post(auth_handler::register))
        .route("/api/auth/login", post(auth_handler::login))

        // User Providers
        .route("/api/providers", get(user_provider_handler::list_providers))
        .route("/api/providers", post(user_provider_handler::create_provider))
        .route("/api/providers/{id}", get(user_provider_handler::get_provider))
        .route("/api/providers/{id}", put(user_provider_handler::update_provider))
        .route("/api/providers/{id}", delete(user_provider_handler::delete_provider))
        .route("/api/providers/check/{id}", post(user_provider_handler::check_provider),)

        // Provider Models
        .route("/api/providers/{provider_id}/models", post(provider_model_handler::create_model))
        .route("/api/providers/{provider_id}/models/{id}", put(provider_model_handler::update_model))
        .route("/api/providers/{provider_id}/models/{id}", delete(provider_model_handler::delete_model))

        // Conversations
        .route("/api/conversations", get(conversation_handler::list_conversations))
        .route("/api/conversations", post(conversation_handler::create_conversation))
        .route("/api/conversations/{id}/messages", get(conversation_handler::list_messages))
        .route("/api/conversations/{id}/messages", post(conversation_handler::send_message))
        .route("/api/conversations/{id}", delete(conversation_handler::delete_conversation))
}