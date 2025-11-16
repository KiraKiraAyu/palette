use std::sync::Arc;
use axum::{
    Json,
    extract::State,
};

use crate::http::dto::auth_schema::AuthResponse;
use crate::http::dto::common_schema::ApiResponse;
use crate::{http::dto::auth_schema::*, services::auth_service::AuthService};
use crate::error::{Result};
use validator::Validate;

#[axum::debug_handler]
pub async fn register(
    State(state): State<Arc<AuthService>>,
    Json(request): Json<RegisterRequest>
) -> Result<Json<ApiResponse<AuthResponse>>> {
    request.validate()?;
    let response = state.register(request.email, request.name, request.password).await?;
    Ok(Json(ApiResponse::success(Some(response), Some("Registration successful"))))
}

pub async fn login(
    State(state): State<Arc<AuthService>>,
    Json(request): Json<LoginRequest>
) -> Result<Json<ApiResponse<AuthResponse>>> {
    let response = state.login(request.email, request.password).await?;
    Ok(Json(ApiResponse::success(Some(response), Some("Login successful"))))
}