use sea_orm_migration::prelude::*;


#[derive(DeriveIden)]
pub enum ConversationSessions {
    Table,
    Id,
    UserId,
    Title,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
pub enum ConversationMessages {
    Table,
    Id,
    SessionId,
    Role,
    Content,
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
                    .table(ConversationSessions::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(ConversationSessions::Id).uuid().not_null().primary_key(),)
                    .col(ColumnDef::new(ConversationSessions::UserId).uuid().not_null())
                    .col(ColumnDef::new(ConversationSessions::Title).string().null())
                    .col(ColumnDef::new(ConversationSessions::CreatedAt).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(ConversationSessions::UpdatedAt).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_conversation_sessions_user")
                    .from(ConversationSessions::Table, ConversationSessions::UserId)
                    .to(crate::m20251116_000001_create_users_table::Users::Table, crate::m20251116_000001_create_users_table::Users::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_conversation_sessions_user_id")
                    .table(ConversationSessions::Table)
                    .col(ConversationSessions::UserId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(ConversationMessages::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(ConversationMessages::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(ConversationMessages::SessionId).uuid().not_null())
                    .col(ColumnDef::new(ConversationMessages::Role).string().not_null())
                    .col(ColumnDef::new(ConversationMessages::Content).text().not_null())
                    .col(ColumnDef::new(ConversationMessages::CreatedAt).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(ConversationMessages::UpdatedAt).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_conversation_messages_session")
                    .from(ConversationMessages::Table, ConversationMessages::SessionId)
                    .to(ConversationSessions::Table, ConversationSessions::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_conversation_messages_session_id_created_at")
                    .table(ConversationMessages::Table)
                    .col(ConversationMessages::SessionId)
                    .col(ConversationMessages::CreatedAt)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ConversationMessages::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(ConversationSessions::Table).to_owned())
            .await?;
        Ok(())
    }
}