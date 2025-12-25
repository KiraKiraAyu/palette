use axum::{Json, http::StatusCode, response::{IntoResponse, Response}};
use thiserror::Error;

use crate::http::dto::common_schema::ApiResponse;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Forbidden: {0}")]
    Forbidden(String),
    
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Conflict: {0}")]
    Conflict(String),
    
    #[error("Bad request: {0}")]
    BadRequest(String),
    
    #[error("Internal server error: {0}")]
    Internal(String),

    #[error("Authorization error: {0}")]
    Authorization(#[from] crate::http::extractors::jwt::AuthError),

    #[error("Database error: {0}")]
    Database(#[from] sea_orm::DbErr),
    
    #[error("Validation error: {0}")]
    Validation(#[from] validator::ValidationErrors),

    #[error("JWT error: {0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),
    
    #[error("Bcrypt error: {0}")]
    Bcrypt(#[from] bcrypt::BcryptError),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("UUID error: {0}")]
    Uuid(#[from] uuid::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::Authorization(ref msg) => {
                tracing::error!("Authorization error: {}", msg);
                (StatusCode::UNAUTHORIZED, "Failed to authenticate")
            }
            AppError::Forbidden(ref msg) => {
                tracing::error!("Forbidden error: {}", msg);
                (StatusCode::FORBIDDEN, "Forbidden")
            }
            AppError::NotFound(ref msg) => {
                tracing::error!("Not Found error: {}", msg);
                (StatusCode::NOT_FOUND, msg.as_str())
            }
            AppError::Conflict(ref msg) => {
                tracing::error!("Conflict error: {}", msg);
                (StatusCode::CONFLICT, msg.as_str())
            }
            AppError::BadRequest(ref msg) => {
                tracing::error!("Bad Request error: {}", msg);
                (StatusCode::BAD_REQUEST, msg.as_str())
            }
            AppError::Internal(ref msg) => {
                tracing::error!("Internal Server error: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server error")
            }
            AppError::Database(ref err) => {
                tracing::error!("Database error: {:?}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Database error")
            }
            AppError::Validation(ref err) => {
                tracing::error!("Validation error: {:?}", err);
                (StatusCode::BAD_REQUEST, "Failed to validate request data")
            }
            AppError::Jwt(ref err) => {
                tracing::error!("JWT error: {:?}", err);
                (StatusCode::UNAUTHORIZED, "Invalid token")
            }
            AppError::Bcrypt(ref err) => {
                tracing::error!("Bcrypt error: {:?}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server error")
            }
            AppError::Serialization(ref err) => {
                tracing::error!("Serialization error: {:?}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server error")
            }
            AppError::Uuid(ref err) => {
                tracing::error!("UUID error: {:?}", err);
                (StatusCode::BAD_REQUEST, "Invalid id")
            }
        };

        let body = Json(ApiResponse::failed(Some(error_message.to_string())));

        (status, body).into_response()
    }
}

pub type Result<T> = std::result::Result<T, AppError>;