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
            provider_schema::{CreateProviderRequest, UpdateProviderRequest, ProviderWithModelsListResponse, ProviderWithModels, ProviderIdResponse},
        },
        extractors::jwt::AuthUser,
    },
    models::user_provider,
    services::user_provider_service::UserProviderService,
};

pub async fn list_providers(
    AuthUser(claims): AuthUser,
    State(state): State<Arc<UserProviderService>>,
) -> Result<Json<ApiResponse<ProviderWithModelsListResponse>>> {
    let pairs = state.list_with_models(claims.sub).await?;
    let items = pairs.into_iter().map(|(provider, models)| ProviderWithModels { provider, models }).collect();
    Ok(Json(ApiResponse::success(Some(ProviderWithModelsListResponse { items }), None::<String>)))
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
) -> Result<Json<ApiResponse<ProviderWithModels>>> {
    let (provider, models) = state.get_with_models(claims.sub, id).await?;
    Ok(Json(ApiResponse::success(Some(ProviderWithModels { provider, models }), None::<String>)))
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
    Ok(Json(ApiResponse::success(
        Some(ProviderIdResponse { id }),
        Some("Provider deleted"),
    )))
}

pub async fn check_provider(
    AuthUser(claims): AuthUser,
    State(state): State<Arc<UserProviderService>>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<()>>> {
    state.check(claims.sub, id).await?;
    Ok(Json(ApiResponse::success(None, Some("Provider verified"))))
}
