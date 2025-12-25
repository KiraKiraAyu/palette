use std::time::Duration;
use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};

pub async fn get_postgres_connection(url: &str) -> Result<DatabaseConnection, DbErr> {
    let mut opt = ConnectOptions::new(url);
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true);
    Database::connect(opt).await
}

pub async fn run_migrations(db: &DatabaseConnection) -> Result<(), DbErr> {
    use migration::Migrator;
    use sea_orm_migration::MigratorTrait;

    Migrator::up(db, None).await
}