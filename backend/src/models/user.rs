use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};

use crate::set_timestamp_before_save;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(unique)]
    pub email: String,
    #[sea_orm(unique)]
    pub name: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub avatar: Option<String>,
    pub preferences: Option<Json>,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    #[serde(skip_serializing)]
    pub deleted_at: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "crate::models::user_provider::Entity")]
    UserProviders,
}

impl Related<crate::models::user_provider::Entity> for Entity {
    fn to() -> RelationDef { Relation::UserProviders.def() }
}


set_timestamp_before_save!(ActiveModel);