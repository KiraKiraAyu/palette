use std::sync::Arc;
use bcrypt::{DEFAULT_COST, hash, verify};
use chrono::Utc;
use jsonwebtoken::{Header, encode};
use crate::{config::JwtConfig, error::{AppError, Result}, http::dto::auth_schema::{AuthResponse, Claims}, repositories::user_repo::UserRepo};

#[derive(Clone)]
pub struct AuthService {
    pub repo: Arc<UserRepo>,
    pub jwt_config: JwtConfig,
}

impl AuthService {
    pub fn new(repo: Arc<UserRepo>, jwt_config: JwtConfig) -> Self {
        Self { repo, jwt_config }
    }

    pub async fn register(&self, email: String, name: String, password: String) -> Result<AuthResponse> {
        let email = email.to_lowercase();

        if self.repo.get_user_by_email(&email).await?.is_some() {
            return Err(AppError::Conflict("Email already registered".to_string()));
        }

        if self.repo.get_user_by_name(&name).await?.is_some() {
            return Err(AppError::Conflict("Username already existed".to_string()));
        }

        let password_hash = tokio::task::spawn_blocking(move || hash(&password, DEFAULT_COST))
            .await
            .map_err(|e| AppError::Internal(e.to_string()))??;

        let created_user = self.repo.create(email, name, password_hash).await?;
        let token = self.generate_token(created_user.id)?;
        
        Ok(AuthResponse { token, user_info: created_user })
    }

    pub async fn login(&self, email: String, password: String) -> Result<AuthResponse> {
        let email = email.to_lowercase();

        let user = self.repo.get_user_by_email(&email).await?;

        if let Some(user) = user {
            let password_hash = user.password_hash.clone();
            let is_password_correct = tokio::task::spawn_blocking(move || verify(&password, &password_hash))
                .await
                .map_err(|e| AppError::Internal(e.to_string()))??;

            if is_password_correct {
                let token = self.generate_token(user.id)?;
                Ok(AuthResponse { token, user_info: user })
            } else {
                Err(AppError::Forbidden("Incorrect email or password".to_string()))
            }
        } else {
            Err(AppError::Forbidden("Incorrect email or password".to_string()))
        }
    }

    pub async fn logout(&self) {}

    fn generate_token(&self, user_id: uuid::Uuid) -> Result<String> {
        let now = Utc::now();
        let exp = now.timestamp() + &self.jwt_config.expires_in;

        let claims = Claims {
            sub: user_id,
            exp,
        };

        encode(&Header::default(), &claims, &self.jwt_config.encoding_key).map_err(AppError::from)
    }
}
