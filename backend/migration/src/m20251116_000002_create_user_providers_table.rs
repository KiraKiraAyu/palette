use sea_orm_migration::prelude::*;

use crate::m20251116_000001_create_users_table::Users;

#[derive(DeriveIden)]
pub enum UserProviders {
    Table,
    Id,
    UserId,
    Name,
    ProviderType,
    Url,
    Key,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(
            Table::create()
                .table(UserProviders::Table)
                .if_not_exists()
                .col(ColumnDef::new(UserProviders::Id).uuid().not_null().primary_key())
                .col(ColumnDef::new(UserProviders::UserId).uuid().not_null())
                .col(ColumnDef::new(UserProviders::Name).string().not_null())
                .col(ColumnDef::new(UserProviders::ProviderType).string().not_null())
                .col(ColumnDef::new(UserProviders::Url).string().not_null())
                .col(ColumnDef::new(UserProviders::Key).string().null())
                .col(ColumnDef::new(UserProviders::CreatedAt).timestamp_with_time_zone().not_null()
                    .default(Expr::current_timestamp()))
                .col(ColumnDef::new(UserProviders::UpdatedAt).timestamp_with_time_zone().not_null()
                    .default(Expr::current_timestamp()))
                .foreign_key(
                    ForeignKey::create()
                        .name("fk_user_providers_user_id")
                        .from(UserProviders::Table, UserProviders::UserId)
                        .to(Users::Table, Users::Id)
                        .on_delete(ForeignKeyAction::Cascade)
                        .on_update(ForeignKeyAction::Cascade)
                )
                .to_owned(),
        ).await?;

        manager.create_index(
            Index::create()
                .name("idx_user_providers_user_id_name")
                .table(UserProviders::Table)
                .col(UserProviders::UserId)
                .col(UserProviders::Name)
                .unique()
                .to_owned()
        ).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_index(
            Index::drop()
                .name("idx_user_providers_user_id_name")
                .table(UserProviders::Table)
                .to_owned(),
        ).await?;
        manager.drop_table(Table::drop().table(UserProviders::Table).to_owned()).await
    }
}
