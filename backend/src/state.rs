use std::sync::Arc;
use axum::extract::FromRef;
use sea_orm::DatabaseConnection;
use crate::{
    config::Config,
    database::{get_postgres_connection, run_migrations},
    repositories::{user_repo::UserRepo, provider_repo::ProviderRepo, provider_model_repo::ProviderModelRepo, conversation_session_repo::ConversationSessionRepo, conversation_message_repo::ConversationMessageRepo},
    services::{auth_service::AuthService, user_provider_service::UserProviderService, provider_model_service::ProviderModelService, conversation_service::ConversationService},
    clients::{model_info_client::{DefaultModelInfoClient, ModelInfoClient}, llm_client::{DefaultLlmClient, LlmClient}},
};

#[derive(Clone)]
pub struct AppState {
    pub database: DatabaseConnection,
    pub auth_service: Arc<AuthService>,
    pub user_provider_service: Arc<UserProviderService>,
    pub provider_model_service: Arc<ProviderModelService>,
    pub conversation_service: Arc<ConversationService>,
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

impl FromRef<AppState> for Arc<ConversationService> {
    fn from_ref(state: &AppState) -> Self {
        state.conversation_service.clone()
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
    let model_info_client: Arc<dyn ModelInfoClient> = Arc::new(DefaultModelInfoClient::default());
    let provider_model_service = Arc::new(ProviderModelService::new(provider_model_repo.clone(), provider_repo.clone(), model_info_client));

    let session_repo = Arc::new(ConversationSessionRepo::new(database.clone()));
    let message_repo = Arc::new(ConversationMessageRepo::new(database.clone()));
    let llm_client: Arc<dyn LlmClient> = Arc::new(DefaultLlmClient::default());
    let conversation_service = Arc::new(ConversationService::new(session_repo, message_repo, provider_model_repo.clone(), provider_repo.clone(), llm_client));

    Ok(AppState {
        database,
        auth_service,
        user_provider_service,
        provider_model_service,
        conversation_service,
    })
}
