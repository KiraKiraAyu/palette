use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};

use crate::set_timestamp_before_save;

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(
    rs_type = "String",
    db_type = "Text",
    enum_name = "chat_role"
)]
pub enum ChatRole {
    #[sea_orm(string_value = "system")]
    System,
    #[sea_orm(string_value = "user")]
    User,
    #[sea_orm(string_value = "assistant")]
    Assistant,
}

impl ChatRole {
    pub fn as_str(&self) -> &'static str {
        match self {
            ChatRole::System => "system",
            ChatRole::User => "user",
            ChatRole::Assistant => "assistant",
        }
    }
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "conversation_messages")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub session_id: Uuid,
    pub role: ChatRole,
    pub content: String,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "crate::models::conversation_session::Entity",
        from = "Column::SessionId",
        to = "crate::models::conversation_session::Column::Id"
    )]
    Session,
}

impl Related<crate::models::conversation_session::Entity> for Entity {
    fn to() -> RelationDef { Relation::Session.def() }
}

set_timestamp_before_save!(ActiveModel);