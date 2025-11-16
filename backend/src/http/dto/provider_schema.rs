use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::models::user_provider::{self, ProviderType};

#[derive(Debug, Clone, Validate, Deserialize)]
pub struct CreateProviderRequest {
    #[validate(length(min = 1, max = 64))]
    pub name: String,
    pub provider_type: ProviderType,
    #[validate(url)]
    pub url: String,
    #[validate(length(min = 1, max = 4096))]
    pub key: Option<String>,
}

#[derive(Debug, Clone, Validate, Deserialize)]
pub struct UpdateProviderRequest {
    #[validate(length(min = 1, max = 64))]
    pub name: Option<String>,
    pub provider_type: Option<ProviderType>,
    #[validate(url)]
    pub url: Option<String>,
    pub key: Option<Option<String>>,
}

#[derive(Debug, Serialize)]
pub struct ProviderListResponse {
    pub items: Vec<user_provider::Model>,
}

#[derive(Debug, Serialize)]
pub struct ProviderIdResponse {
    pub id: Uuid,
}