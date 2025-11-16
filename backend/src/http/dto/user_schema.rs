use serde::{Serialize, Deserialize};
use validator::Validate;


#[derive(Debug, Clone, Validate, Deserialize)]
pub struct UpdateProfileRequest {
    #[validate(length(min = 1, max = 64))]
    pub name: Option<String>,
}