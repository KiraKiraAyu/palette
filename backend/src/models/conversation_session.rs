use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};

use crate::set_timestamp_before_save;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "conversation_sessions")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: Option<String>,
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
    #[sea_orm(has_many = "crate::models::conversation_message::Entity")]
    Messages,
}

impl Related<crate::models::user::Entity> for Entity {
    fn to() -> RelationDef { Relation::User.def() }
}

impl Related<crate::models::conversation_message::Entity> for Entity {
    fn to() -> RelationDef { Relation::Messages.def() }
}

set_timestamp_before_save!(ActiveModel);