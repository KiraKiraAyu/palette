use serde::{Deserialize, Serialize};
use validator::Validate;
use uuid::Uuid;

use crate::models::provider_model;

#[derive(Debug, Clone, Validate, Deserialize)]
pub struct CreateProviderModelRequest {
    #[validate(length(min = 1, max = 128))]
    pub model_id: String,
    #[validate(length(min = 1, max = 64))]
    pub name: String,
}

#[derive(Debug, Clone, Validate, Deserialize)]
pub struct UpdateProviderModelRequest {
    #[validate(length(min = 1, max = 128))]
    pub model_id: Option<String>,
    #[validate(length(min = 1, max = 64))]
    pub name: Option<String>,
}

// #[derive(Debug, Serialize)]
// pub struct ProviderModelListResponse {
//     pub items: Vec<provider_model::Model>,
// }

#[derive(Debug, Serialize)]
pub struct ProviderModelIdResponse {
    pub id: Uuid,
}