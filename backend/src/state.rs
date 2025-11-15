use std::sync::Arc;
use axum::extract::FromRef;
use sea_orm::DatabaseConnection;
use crate::{config::Config, database::{get_postgres_connection, run_migrations}, repositories::user_repo::UserRepo, services::auth_service::AuthService};

#[derive(Clone)]
pub struct AppState {
    pub database: DatabaseConnection,
    pub auth_service: Arc<AuthService>,
}

impl FromRef<AppState> for DatabaseConnection {
    fn from_ref(state: &AppState) -> Self {
        state.database.clone()
    }
}

impl FromRef<AppState> for Arc<AuthService> {
    fn from_ref(state: &AppState) -> Self {
        state.auth_service.clone()
    }
}

pub async fn create_state(config: &Config) -> Result<AppState, Box<dyn std::error::Error>> {
    let database = get_postgres_connection(&config.database_url).await?;
    run_migrations(&database).await?;
    let user_repo = Arc::new(UserRepo::new(database.clone()));
    let auth_service = Arc::new(AuthService::new(user_repo, config.jwt.clone()));

    Ok(AppState {
        database,
        auth_service,
    })
}
