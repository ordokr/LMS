use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use std::sync::Arc;

// Database modules
pub mod optimize;
pub mod user_repository;
pub mod redb_error;
pub mod redb_transaction;
pub mod redb;
pub mod test_redb_transactions;

// Test modules
#[cfg(test)]
pub mod tests {
    pub mod redb_transaction_tests;
    pub mod redb_transaction_advanced_tests;
}

// Setup the database pool
pub async fn setup_database(db_path: &str) -> Result<Arc<SqlitePool>, sqlx::Error> {
    let db_url = format!("sqlite:{}?mode=rwc&cache=shared", db_path);

    // Create connection pool with optimized settings
    let pool = SqlitePoolOptions::new()
        .max_connections(16)
        .connect(&db_url)
        .await?;

    // Apply optimizations
    optimize::optimize_db_connection(&pool).await?;

    // Run migrations if you have them
    // sqlx::migrate!("./migrations").run(&pool).await?;

    Ok(Arc::new(pool))
}

pub struct DB {
    pub pool: SqlitePool,
}

impl Clone for DB {
    fn clone(&self) -> Self {
        DB {
            pool: self.pool.clone(),
        }
    }
}

impl DB {
    pub async fn connect(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = SqlitePoolOptions::new()
            .max_connections(10)
            .connect(database_url)
            .await?;

        // Run migrations
        sqlx::migrate!("./migrations")
            .run(&pool)
            .await?;

        Ok(DB { pool })
    }

    pub async fn initialize_tables(&self) -> Result<(), sqlx::Error> {
        // Run our custom migration to create all model tables
        let mut conn = self.pool.acquire().await?;
        crate::db::migrations::create_models_tables::run(&mut conn).await
            .map_err(|e| sqlx::Error::RowNotFound)?;

        Ok(())
    }
}

pub mod migrations {
    pub mod create_models_tables;
}