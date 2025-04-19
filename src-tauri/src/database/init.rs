use sqlx::{migrate::MigrateDatabase, sqlite::{SqliteConnectOptions, SqlitePoolOptions}, Pool, Sqlite, SqlitePool};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::time::Duration;
use tracing::{info, warn, error};
use anyhow::{Result, Context, anyhow};

/// Initialize the database with optimized settings for the Ordo LMS application.
/// This function creates the database if it doesn't exist, sets up the connection pool,
/// applies migrations, and performs basic schema validation.
///
/// # Arguments
/// * `db_path` - The path to the SQLite database file (default: from DATABASE_URL env var or "lms.db")
/// * `max_connections` - The maximum number of connections in the pool (default: 10)
///
/// # Returns
/// A Result containing the SQLite connection pool or an error
pub async fn init_db(db_path: Option<&str>, max_connections: Option<u32>) -> Result<Pool<Sqlite>> {
    // Get database path from parameter, environment, or use default
    let database_path = match db_path {
        Some(path) => path.to_string(),
        None => std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "sqlite:lms.db".to_string())
    };

    info!("Initializing database at: {}", database_path);

    // Ensure the database directory exists
    let path = database_path.trim_start_matches("sqlite:");
    if let Some(parent) = Path::new(path).parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent)
                .context(format!("Failed to create database directory: {:?}", parent))?;
            info!("Created database directory: {:?}", parent);
        }
    }

    // Create database if it doesn't exist
    if !Sqlite::database_exists(&database_path).await.unwrap_or(false) {
        info!("Database does not exist, creating it now");
        Sqlite::create_database(&database_path).await
            .context("Failed to create database")?;
        info!("Database created successfully");
    }

    // Configure connection options with optimized settings
    let options = SqliteConnectOptions::from_str(&database_path)?
        .create_if_missing(true)
        .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal)
        .synchronous(sqlx::sqlite::SqliteSynchronous::Normal)
        .busy_timeout(Duration::from_secs(10))
        .pragma("cache_size", "-8000")   // 8MB cache
        .pragma("foreign_keys", "ON")
        .pragma("temp_store", "MEMORY")  // Store temp tables in memory
        .pragma("mmap_size", "30000000") // 30MB mmap
        .pragma("page_size", "4096");    // Optimize page size

    // Create connection pool with optimal settings
    let max_conn = max_connections.unwrap_or(10);
    info!("Creating connection pool with max {} connections", max_conn);
    let pool = SqlitePoolOptions::new()
        .min_connections(2)
        .max_connections(max_conn)
        .max_lifetime(Duration::from_secs(1800)) // 30-minute maximum connection lifetime
        .idle_timeout(Duration::from_secs(600))  // 10-minute idle timeout
        .acquire_timeout(Duration::from_secs(5)) // 5-second timeout for connection acquisition
        .test_before_acquire(true) // Ensure connections are valid before use
        .connect_with(options)
        .await
        .context("Failed to create database connection pool")?;

    // Apply migrations
    info!("Applying database migrations");
    apply_migrations(&pool).await?;

    // Validate schema
    info!("Validating database schema");
    validate_schema(&pool).await?;

    info!("Database initialization complete");
    Ok(pool)
}

/// Apply database migrations from the migrations directory
async fn apply_migrations(pool: &SqlitePool) -> Result<()> {
    // Check if migrations directory exists
    let migrations_dir = Path::new("migrations");
    if !migrations_dir.exists() {
        warn!("Migrations directory not found, creating it");
        std::fs::create_dir_all(migrations_dir)
            .context("Failed to create migrations directory")?;
    }

    // Try to apply migrations using SQLx migrate first
    match sqlx::migrate!("./migrations").run(pool).await {
        Ok(_) => {
            info!("Migrations applied successfully using SQLx migrate");
            return Ok(());
        },
        Err(e) => {
            warn!("SQLx migrate failed: {}, falling back to manual migration", e);
            // Continue with manual migration
        }
    }

    // Apply migrations manually by reading SQL files
    info!("Applying migrations manually");
    let migration_files = match std::fs::read_dir(migrations_dir) {
        Ok(files) => files,
        Err(e) => {
            error!("Failed to read migrations directory: {}", e);
            return Err(anyhow!("Failed to read migrations directory: {}", e));
        }
    };

    // Sort migration files by name to ensure they're applied in order
    let mut migration_paths = Vec::new();

    for entry_result in migration_files {
        let entry = match entry_result {
            Ok(entry) => entry,
            Err(e) => {
                error!("Failed to read migration entry: {}", e);
                continue; // Skip this entry but continue with others
            }
        };

        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) == Some("sql") {
            migration_paths.push(path);
        }
    }

    // Sort paths to ensure consistent order
    migration_paths.sort();

    // Apply each migration
    for path in migration_paths {
        info!("Applying migration: {:?}", path);
        let sql = match std::fs::read_to_string(&path) {
            Ok(sql) => sql,
            Err(e) => {
                error!("Failed to read migration file: {}", e);
                continue; // Skip this file but continue with others
            }
        };

        // Execute the SQL
        match sqlx::query(&sql).execute(pool).await {
            Ok(_) => info!("Migration applied successfully: {:?}", path),
            Err(e) => {
                warn!("Error applying migration {:?}: {}", path, e);
                // Continue with next migration, don't fail completely
            }
        }
    }

    info!("Manual migration application complete");
    Ok(())
}

/// Optimize the database connection for better performance
pub async fn optimize_db(pool: &SqlitePool) -> Result<()> {
    info!("Optimizing database connection");

    // Enable WAL mode for better concurrency
    sqlx::query("PRAGMA journal_mode = WAL;")
        .execute(pool)
        .await
        .context("Failed to set journal_mode to WAL")?;

    // Set synchronous mode to NORMAL for better performance
    sqlx::query("PRAGMA synchronous = NORMAL;")
        .execute(pool)
        .await
        .context("Failed to set synchronous mode to NORMAL")?;

    // Enable foreign keys
    sqlx::query("PRAGMA foreign_keys = ON;")
        .execute(pool)
        .await
        .context("Failed to enable foreign keys")?;

    // Set cache size to 8MB
    sqlx::query("PRAGMA cache_size = -8000;")
        .execute(pool)
        .await
        .context("Failed to set cache size")?;

    // Store temp tables in memory
    sqlx::query("PRAGMA temp_store = MEMORY;")
        .execute(pool)
        .await
        .context("Failed to set temp_store to MEMORY")?;

    info!("Database optimization complete");
    Ok(())
}

/// Validate the database schema to ensure all required tables exist
async fn validate_schema(pool: &SqlitePool) -> Result<()> {
    info!("Validating database schema");

    // Check if essential tables exist
    let tables = vec![
        "users",
        "courses",
        "quizzes",
        "questions",
        "answers",
        "quiz_attempts",
    ];

    for table in tables {
        let result = sqlx::query(&format!("SELECT name FROM sqlite_master WHERE type='table' AND name='{}'", table))
            .fetch_optional(pool)
            .await
            .context(format!("Failed to check if table exists: {}", table))?;

        if result.is_none() {
            warn!("Essential table missing: {}", table);
        } else {
            info!("Table exists: {}", table);
        }
    }

    // Validate quiz schema specifically
    validate_quiz_schema(pool).await?;

    info!("Schema validation complete");
    Ok(())
}

/// Validate the quiz schema to ensure all required tables and columns exist
async fn validate_quiz_schema(pool: &SqlitePool) -> Result<()> {
    info!("Validating quiz schema");

    // Check if quiz tables exist
    let quiz_tables = vec![
        "quizzes",
        "questions",
        "answers",
        "quiz_attempts",
        "quiz_settings",
    ];

    let mut missing_tables = Vec::new();

    for table in quiz_tables {
        let result = sqlx::query(&format!("SELECT name FROM sqlite_master WHERE type='table' AND name='{}'", table))
            .fetch_optional(pool)
            .await
            .context(format!("Failed to check if table exists: {}", table))?;

        if result.is_none() {
            missing_tables.push(table);
            warn!("Quiz table missing: {}", table);
        } else {
            info!("Quiz table exists: {}", table);
        }
    }

    // If essential quiz tables are missing, try to apply the quiz schema migration
    if !missing_tables.is_empty() {
        warn!("Essential quiz tables are missing, attempting to apply quiz schema migration");
        apply_quiz_schema_migration(pool).await?;
    }

    info!("Quiz schema validation complete");
    Ok(())
}

/// Apply the quiz schema migration specifically
async fn apply_quiz_schema_migration(pool: &SqlitePool) -> Result<()> {
    info!("Applying quiz schema migration");

    // Look for the quiz schema migration file
    let migration_path = Path::new("migrations").join("20240421_ordo_quiz_schema.sql");

    if migration_path.exists() {
        info!("Found quiz schema migration: {:?}", migration_path);
        let sql = std::fs::read_to_string(&migration_path)
            .context(format!("Failed to read quiz schema migration: {:?}", migration_path))?;

        // Execute the SQL
        sqlx::query(&sql)
            .execute(pool)
            .await
            .context("Failed to apply quiz schema migration")?;

        info!("Quiz schema migration applied successfully");
    } else {
        warn!("Quiz schema migration not found: {:?}", migration_path);
        return Err(anyhow!("Quiz schema migration not found"));
    }

    Ok(())
}

/// Initialize the quiz database specifically
pub async fn init_quiz_db(pool: &SqlitePool) -> Result<()> {
    info!("Initializing quiz database");

    // Apply quiz schema migration
    apply_quiz_schema_migration(pool).await?;

    // Validate quiz schema
    validate_quiz_schema(pool).await?;

    info!("Quiz database initialization complete");
    Ok(())
}
