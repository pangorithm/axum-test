use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};
use std::env;
use tokio::time::{sleep, Duration};

pub async fn establish_connection() -> DatabaseConnection {
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

    loop {
        match Database::connect(opt.clone()).await {
            Ok(db) => {
                println!("Connected to database");
                test_connection(db.clone()).await;
                return db;
            }
            Err(e) => {
                eprintln!("Failed to connect to database: {:?}", e);
                println!("wait 10 seconds...");
                sleep(Duration::from_secs(10)).await;
                println!("retry establish connection...");
            }
        }
    }
}

pub async fn test_connection(db: DatabaseConnection) {
    assert!(db.ping().await.is_ok());
    let _ = db.clone().close().await;
    assert!(matches!(db.ping().await, Err(DbErr::ConnectionAcquire(_))));
}
