use std::sync::Arc;

use axum::{Json, extract::{Path, State}};
use uuid::Uuid;
use validator::Validate;

use crate::{
    error::{AppError, Result},
    http::dto::{
        common_schema::ApiResponse,
        provider_models_schema::{CreateProviderModelRequest, UpdateProviderModelRequest, ProviderModelIdResponse},
    },
    models::provider_model,
    services::provider_model_service::ProviderModelService,
    http::extractors::jwt::AuthUser,
};

pub async fn create_model(
    AuthUser(claims): AuthUser,
    State(state): State<Arc<ProviderModelService>>,
    Path(provider_id): Path<Uuid>,
    Json(request): Json<CreateProviderModelRequest>,
) -> Result<Json<ApiResponse<provider_model::Model>>> {
    request.validate()?;
    let created = state
        .create(
            claims.sub,
            provider_id,
            request.model_id,
            request.name,
        )
        .await?;
    Ok(Json(ApiResponse::success(Some(created), Some("Model created"))))
}

pub async fn update_model(
    AuthUser(claims): AuthUser,
    State(state): State<Arc<ProviderModelService>>,
    Path((provider_id, id)): Path<(Uuid, Uuid)>,
    Json(request): Json<UpdateProviderModelRequest>,
) -> Result<Json<ApiResponse<provider_model::Model>>> {
    request.validate()?;
    let updated = state
        .update(
            claims.sub,
            id,
            request.model_id,
            request.name,
        )
        .await?;
    if updated.provider_id != provider_id {
        return Err(AppError::NotFound("Model not found".to_string()));
    }
    Ok(Json(ApiResponse::success(Some(updated), Some("Model updated"))))
}

pub async fn delete_model(
    AuthUser(claims): AuthUser,
    State(state): State<Arc<ProviderModelService>>,
    Path((provider_id, id)): Path<(Uuid, Uuid)>,
) -> Result<Json<ApiResponse<ProviderModelIdResponse>>> {
    let model = state.get(claims.sub, id).await?;
    if model.provider_id != provider_id {
        return Err(AppError::NotFound("Model not found".to_string()));
    }
    state.delete(claims.sub, id).await?;
    Ok(Json(ApiResponse::success(Some(ProviderModelIdResponse { id }), Some("Model deleted"))))
}