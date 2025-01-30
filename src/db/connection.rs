use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};
use std::{env, time::Duration};

pub async fn establish_connection() -> Result<DatabaseConnection, DbErr> {
    let database_url =
        env::var("POSTGRES_DATABASE_URL").expect("POSTGRES_DATABASE_URL must be set");
    let mut opt = ConnectOptions::new(database_url);
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Info)
        .set_schema_search_path("public"); // Setting default PostgreSQL schema

    let db: Result<DatabaseConnection, DbErr> = Database::connect(opt).await;

    db
}

pub async fn test_connection(db: DatabaseConnection) {
    assert!(db.ping().await.is_ok());
    let _ = db.clone().close().await;
    assert!(matches!(db.ping().await, Err(DbErr::ConnectionAcquire(_))));
}
