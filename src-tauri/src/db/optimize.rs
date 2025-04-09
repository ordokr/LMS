use sqlx::SqlitePool;

/// Applies performance optimizations to SQLite connection
pub async fn optimize_db_connection(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    // WAL journal mode for better concurrency and performance
    sqlx::query("PRAGMA journal_mode = WAL;").execute(pool).await?;
    
    // Normal sync mode offers good durability with better performance
    sqlx::query("PRAGMA synchronous = NORMAL;").execute(pool).await?;
    
    // 64MB cache (negative value means kilobytes)
    sqlx::query("PRAGMA cache_size = -64000;").execute(pool).await?;
    
    // Enforce foreign key constraints
    sqlx::query("PRAGMA foreign_keys = ON;").execute(pool).await?;
    
    // Set a reasonable journal size limit (6MB)
    sqlx::query("PRAGMA journal_size_limit = 6144000;").execute(pool).await?;
    
    // For read-heavy workloads, this can help
    sqlx::query("PRAGMA temp_store = MEMORY;").execute(pool).await?;
    
    // Log optimization complete
    println!("SQLite optimizations applied successfully");
    
    Ok(())
}

// Add to existing optimize_db_connection function
pub async fn configure_memory_limits(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    // Set memory usage limits
    sqlx::query("PRAGMA hard_heap_limit = 67108864").execute(pool).await?; // 64MB limit
    sqlx::query("PRAGMA soft_heap_limit = 50331648").execute(pool).await?; // 48MB soft limit
    
    // Configure shared cache
    sqlx::query("PRAGMA cache_spill = 1").execute(pool).await?;
    
    // Memory-map temp files when memory is constrained
    sqlx::query("PRAGMA temp_store = 2").execute(pool).await?; 
    
    Ok(())
}