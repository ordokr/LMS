use sqlx::SqlitePool;
use log::{info, warn};

/// Analyze a SQL query and log its execution plan
pub async fn analyze_query(pool: &SqlitePool, sql: &str) -> Result<(), sqlx::Error> {
    let explain = format!("EXPLAIN QUERY PLAN {}", sql);
    let rows = sqlx::query(&explain).fetch_all(pool).await?;
    
    info!("Query plan for: {}", sql);
    
    for row in rows {
        let detail: String = row.try_get("detail")?;
        let using_index = detail.to_lowercase().contains("using index");
        
        if using_index {
            info!("  ✓ {}", detail);
        } else {
            warn!("  ⚠ {}", detail);
        }
    }
    
    Ok(())
}

/// Run ANALYZE on the database to update statistics
pub async fn update_statistics(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    info!("Updating SQLite statistics...");
    sqlx::query("ANALYZE").execute(pool).await?;
    info!("Statistics updated successfully");
    Ok(())
}