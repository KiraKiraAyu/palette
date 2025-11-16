use sea_orm_migration::prelude::*;

use crate::m20251116_000002_create_user_providers_table::UserProviders;

#[derive(DeriveIden)]
enum ProviderModels {
    Table,
    Id,
    ProviderId,
    ModelId,
    Name,
    InputPricePerMillion,
    OutputPricePerMillion,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ProviderModels::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(ProviderModels::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(ProviderModels::ProviderId).uuid().not_null())
                    .col(ColumnDef::new(ProviderModels::ModelId).string().not_null())
                    .col(ColumnDef::new(ProviderModels::Name).string().not_null())
                    .col(ColumnDef::new(ProviderModels::InputPricePerMillion).decimal().not_null())
                    .col(ColumnDef::new(ProviderModels::OutputPricePerMillion).decimal().not_null())
                    .col(ColumnDef::new(ProviderModels::CreatedAt).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(ProviderModels::UpdatedAt).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_provider_models_provider_id")
                            .from(ProviderModels::Table, ProviderModels::ProviderId)
                            .to(UserProviders::Table, UserProviders::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_provider_models_provider_id")
                    .table(ProviderModels::Table)
                    .col(ProviderModels::ProviderId)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("uq_provider_models_provider_id_model_id")
                    .table(ProviderModels::Table)
                    .col(ProviderModels::ProviderId)
                    .col(ProviderModels::ModelId)
                    .unique()
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(Index::drop().name("uq_provider_models_provider_id_model_id").table(ProviderModels::Table).to_owned())
            .await?;
        manager
            .drop_index(Index::drop().name("idx_provider_models_provider_id").table(ProviderModels::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(ProviderModels::Table).to_owned())
            .await
    }
}