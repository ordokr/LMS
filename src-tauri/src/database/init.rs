use sqlx::{migrate::MigrateDatabase, sqlite::SqlitePoolOptions, SqlitePool};
use std::path::Path;
use tracing::{info, error};

pub async fn init_db() -> Result<SqlitePool, sqlx::Error> {
    info!("Initializing database...");
    
    // Get database URL from environment or use default
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:lms.db".to_string());
    
    // Create database if it doesn't exist
    if !sqlx::Sqlite::database_exists(&database_url).await.unwrap_or(false) {
        info!("Creating database...");
        sqlx::Sqlite::create_database(&database_url).await?;
    }
    
    // Connect to the database
    let pool = SqlitePoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await?;
    
    // Apply migrations
    info!("Applying migrations...");
    apply_migrations(&pool).await?;
    
    // Optimize database
    info!("Optimizing database...");
    optimize_db(&pool).await?;
    
    info!("Database initialization complete");
    Ok(pool)
}

async fn apply_migrations(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    // Check if migrations directory exists
    let migrations_dir = Path::new("migrations");
    if !migrations_dir.exists() {
        error!("Migrations directory not found");
        return Ok(());
    }
    
    // Apply migrations from SQL files
    let migration_files = std::fs::read_dir(migrations_dir)
        .map_err(|e| {
            error!("Failed to read migrations directory: {}", e);
            sqlx::Error::Configuration(Box::new(e))
        })?;
    
    for entry in migration_files {
        let entry = entry.map_err(|e| {
            error!("Failed to read migration entry: {}", e);
            sqlx::Error::Configuration(Box::new(e))
        })?;
        
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) == Some("sql") {
            info!("Applying migration: {:?}", path);
            let sql = std::fs::read_to_string(&path).map_err(|e| {
                error!("Failed to read migration file: {}", e);
                sqlx::Error::Configuration(Box::new(e))
            })?;
            
            sqlx::query(&sql).execute(pool).await?;
        }
    }
    
    Ok(())
}

async fn optimize_db(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    // Enable WAL mode for better concurrency
    sqlx::query("PRAGMA journal_mode = WAL;").execute(pool).await?;
    
    // Set synchronous mode to NORMAL for better performance
    sqlx::query("PRAGMA synchronous = NORMAL;").execute(pool).await?;
    
    // Enable foreign keys
    sqlx::query("PRAGMA foreign_keys = ON;").execute(pool).await?;
    
    // Set cache size to 64MB
    sqlx::query("PRAGMA cache_size = -65536;").execute(pool).await?;
    
    Ok(())
}
