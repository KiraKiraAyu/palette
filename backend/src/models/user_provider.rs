use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};

use crate::set_timestamp_before_save;

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(
    rs_type = "String",
    db_type = "Text",
    enum_name = "user_type"
)]
pub enum ProviderType {
    #[sea_orm(string_value = "OpenAI")]
    OpenAI,
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "user_providers")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub provider_type: ProviderType,
    pub url: String,
    pub key: Option<String>,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "crate::models::user::Entity",
        from = "Column::UserId",
        to = "crate::models::user::Column::Id"
    )]
    User,
    #[sea_orm(has_many = "crate::models::provider_model::Entity")]
    ProviderModels,
}

impl Related<crate::models::user::Entity> for Entity {
    fn to() -> RelationDef { Relation::User.def() }
}

impl Related<crate::models::provider_model::Entity> for Entity {
    fn to() -> RelationDef { Relation::ProviderModels.def() }
}

set_timestamp_before_save!(ActiveModel);