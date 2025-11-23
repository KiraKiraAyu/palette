pub use sea_orm_migration::prelude::*;

mod m20251116_000001_create_users_table;
mod m20251116_000002_create_user_providers_table;
mod m20251116_000003_create_provider_models_table;
mod m20251116_000004_create_conversations_tables;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20251116_000001_create_users_table::Migration),
            Box::new(m20251116_000002_create_user_providers_table::Migration),
            Box::new(m20251116_000003_create_provider_models_table::Migration),
            Box::new(m20251116_000004_create_conversations_tables::Migration),
        ]
    }
}