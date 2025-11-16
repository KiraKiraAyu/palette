use axum::{
    Router,
    routing::{get, post, put, delete}
};

use crate::{
    http::handlers::{auth_handler, user_provider_handler},
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
        .route("/api/providers/:id", get(user_provider_handler::get_provider))
        .route("/api/providers/:id", put(user_provider_handler::update_provider))
        .route("/api/providers/:id", delete(user_provider_handler::delete_provider))
}