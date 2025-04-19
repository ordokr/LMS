use sqlx::{sqlite::{SqlitePoolOptions, SqliteConnectOptions}, Pool, Sqlite, SqlitePool};
use std::sync::Arc;
use std::str::FromStr;
use std::time::Duration;
use anyhow::{Result, Context};

// Re-export database initialization functions from database module
pub use crate::database::init::{init_db, optimize_db, init_quiz_db};

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

/// Setup the database pool for the new Ordo application
/// This is a wrapper around the more comprehensive init_db function in database/init.rs
///
/// # Arguments
/// * `db_path` - The path to the SQLite database file
///
/// # Returns
/// A Result containing the SQLite connection pool wrapped in an Arc or an error
pub async fn setup_database(db_path: &str) -> Result<Arc<SqlitePool>, sqlx::Error> {
    // Call the more comprehensive init_db function
    match crate::database::init::init_db(Some(db_path), Some(16)).await {
        Ok(pool) => Ok(Arc::new(pool)),
        Err(e) => Err(sqlx::Error::Configuration(Box::new(e)))
    }
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

        // Run schema migrations to set up the new database structure
        // These migrations create tables for the new application based on source code analysis
        // They do NOT import or migrate data from existing Canvas or Discourse deployments
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