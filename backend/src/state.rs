use std::sync::Arc;
use axum::extract::FromRef;
use sea_orm::DatabaseConnection;
use crate::{
    config::Config,
    database::{get_postgres_connection, run_migrations},
    repositories::{user_repo::UserRepo, provider_repo::ProviderRepo, provider_model_repo::ProviderModelRepo},
    services::{auth_service::AuthService, user_provider_service::UserProviderService, provider_model_service::ProviderModelService},
    clients::pricing_client::{DefaultPricingClient, PricingClient},
};

#[derive(Clone)]
pub struct AppState {
    pub database: DatabaseConnection,
    pub auth_service: Arc<AuthService>,
    pub user_provider_service: Arc<UserProviderService>,
    pub provider_model_service: Arc<ProviderModelService>,
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

impl FromRef<AppState> for Arc<UserProviderService> {
    fn from_ref(state: &AppState) -> Self {
        state.user_provider_service.clone()
    }
}

impl FromRef<AppState> for Arc<ProviderModelService> {
    fn from_ref(state: &AppState) -> Self {
        state.provider_model_service.clone()
    }
}

pub async fn create_state(config: &Config) -> Result<AppState, Box<dyn std::error::Error>> {
    let database = get_postgres_connection(&config.database_url).await?;
    run_migrations(&database).await?;
    
    let user_repo = Arc::new(UserRepo::new(database.clone()));
    let auth_service = Arc::new(AuthService::new(user_repo, config.jwt.clone()));

    let provider_repo = Arc::new(ProviderRepo::new(database.clone()));
    let user_provider_service = Arc::new(UserProviderService::new(provider_repo.clone()));

    let provider_model_repo = Arc::new(ProviderModelRepo::new(database.clone()));
    let pricing_client: Arc<dyn PricingClient> = Arc::new(DefaultPricingClient::default());
    let provider_model_service = Arc::new(ProviderModelService::new(provider_model_repo, provider_repo.clone(), pricing_client));

    Ok(AppState {
        database,
        auth_service,
        user_provider_service,
        provider_model_service,
    })
}
