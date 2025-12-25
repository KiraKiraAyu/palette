use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;

use crate::set_timestamp_before_save;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "provider_models")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub provider_id: Uuid,
    pub model_id: String,
    pub name: String,
    pub input_price_per_million: Decimal,
    pub output_price_per_million: Decimal,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "crate::models::user_provider::Entity",
        from = "Column::ProviderId",
        to = "crate::models::user_provider::Column::Id"
    )]
    Provider,
}

impl Related<crate::models::user_provider::Entity> for Entity {
    fn to() -> RelationDef { Relation::Provider.def() }
}

set_timestamp_before_save!(ActiveModel);