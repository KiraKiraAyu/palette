use axum::{
    Router,
    routing::{get, post, put, delete}
};

use crate::{http::handlers::auth_handler, state::AppState};

pub fn create_routes() -> Router<AppState> {
    Router::new()
        .route("/api/auth/register", post(auth_handler::register))
}