use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
};
use uuid::Uuid;
use validator::Validate;

use crate::{
    error::Result,
    http::{
        dto::{
            common_schema::ApiResponse,
            provider_schema::{CreateProviderRequest, UpdateProviderRequest, ProviderListResponse, ProviderIdResponse},
        },
        extractors::jwt::AuthUser,
    },
    models::user_provider,
    services::user_provider_service::UserProviderService,
};

pub async fn list_providers(
    AuthUser(claims): AuthUser,
    State(state): State<Arc<UserProviderService>>,
) -> Result<Json<ApiResponse<ProviderListResponse>>> {
    let items = state.list(claims.sub).await?;
    Ok(Json(ApiResponse::success(Some(ProviderListResponse { items }), None::<String>)))
}

pub async fn create_provider(
    AuthUser(claims): AuthUser,
    State(state): State<Arc<UserProviderService>>,
    Json(request): Json<CreateProviderRequest>,
) -> Result<Json<ApiResponse<user_provider::Model>>> {
    request.validate()?;
    let created = state
        .create(claims.sub, request.name, request.provider_type, request.url, request.key)
        .await?;
    Ok(Json(ApiResponse::success(Some(created), Some("Provider created"))))
}

pub async fn get_provider(
    AuthUser(claims): AuthUser,
    State(state): State<Arc<UserProviderService>>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<user_provider::Model>>> {
    let model = state.get(claims.sub, id).await?;
    Ok(Json(ApiResponse::success(Some(model), None::<String>)))
}

pub async fn update_provider(
    AuthUser(claims): AuthUser,
    State(state): State<Arc<UserProviderService>>,
    Path(id): Path<Uuid>,
    Json(request): Json<UpdateProviderRequest>,
) -> Result<Json<ApiResponse<user_provider::Model>>> {
    request.validate()?;
    let updated = state
        .update(claims.sub, id, request.name, request.provider_type, request.url, request.key)
        .await?;
    Ok(Json(ApiResponse::success(Some(updated), Some("Provider updated"))))
}

pub async fn delete_provider(
    AuthUser(claims): AuthUser,
    State(state): State<Arc<UserProviderService>>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<ProviderIdResponse>>> {
    state.delete(claims.sub, id).await?;
    Ok(Json(ApiResponse::success(Some(ProviderIdResponse { id }), Some("Provider deleted"))))
}