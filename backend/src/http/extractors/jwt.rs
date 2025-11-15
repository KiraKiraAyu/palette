use axum::{
    Json, RequestPartsExt, extract::{FromRef, FromRequestParts}, http::{StatusCode, request::Parts}, response::{IntoResponse, Response}
};
use axum_extra::{TypedHeader, headers::{Authorization, authorization::Bearer}};
use jsonwebtoken::{Algorithm, Validation, decode};
use serde::de::DeserializeOwned;
use thiserror::Error; 

use crate::{http::dto::{auth_schema::Claims, common_schema::ApiResponse}, state::AppState};

pub struct AuthUser(Claims);

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Invalid token")]
    InvalidToken,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::InvalidToken => (StatusCode::UNAUTHORIZED, "Token 无效"),
        };

        let body = Json(ApiResponse::failed(Some(error_message)));

        (status, body).into_response()
    }
}

impl<S> FromRequestParts<S> for AuthUser
where
    AppState: FromRef<S>,
    S: Send + Sync,
    Claims: DeserializeOwned,
{
    type Rejection = AuthError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::InvalidToken)?;

        let claims = decode::<Claims>(
            bearer.token(),
            &AppState::from_ref(state).auth_service.jwt_config.decoding_key,
            &Validation::new(Algorithm::RS256),
        )
        .map_err(|_| AuthError::InvalidToken)?
        .claims;

        Ok(AuthUser(claims))
    }
}
