use sqlx::{migrate::MigrateDatabase, sqlite::{SqlitePool, SqlitePoolOptions}, Sqlite};
use std::fs;

pub async fn initialize_db() -> Result<SqlitePool, sqlx::Error> {
    // Define database path
    let db_path = "sqlite:ordo.db";

    // Create database if it doesn't exist
    if !Sqlite::database_exists(db_path).await.unwrap_or(false) {
        Sqlite::create_database(db_path).await?;
    }

    // Create pool
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(db_path)
        .await?;

    // Run migrations
    sqlx::migrate!("./migrations").run(&pool).await?;

    Ok(pool)
}