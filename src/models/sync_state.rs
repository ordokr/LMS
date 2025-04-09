use std::collections::HashMap;
use async_trait::async_trait;
use anyhow::{Result, Context};
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use sqlx::{Pool, Sqlite, sqlite::SqlitePoolOptions};

/// Synchronization state storage for entity mappings and status
pub struct SyncState {
    db: Pool<Sqlite>,
}

/// Entity synchronization status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncStatus {
    pub entity_type: String,
    pub entity_id: String,
    pub source_system: String,
    pub target_id: Option<String>,
    pub status: String,
    pub last_synced: Option<DateTime<Utc>>,
    pub last_attempted: Option<DateTime<Utc>>,
    pub last_error: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl SyncState {
    /// Create a new sync state with an SQLite database connection
    pub async fn new() -> Result<Self> {
        // Create or connect to SQLite database
        let db = SqlitePoolOptions::new()
            .max_connections(5)
            .connect("sqlite:sync_state.db")
            .await
            .context("Failed to connect to sync state database")?;
        
        // Initialize database schema
        Self::initialize_schema(&db).await?;
        
        Ok(Self { db })
    }
    
    /// Create a new sync state with a specific database connection
    pub fn with_connection(db: Pool<Sqlite>) -> Self {
        Self { db }
    }
    
    /// Initialize the database schema
    async fn initialize_schema(db: &Pool<Sqlite>) -> Result<()> {
        sqlx::query(r#"
            CREATE TABLE IF NOT EXISTS sync_status (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                entity_type TEXT NOT NULL,
                entity_id TEXT NOT NULL,
                source_system TEXT NOT NULL,
                target_id TEXT,
                status TEXT NOT NULL,
                last_synced TIMESTAMP,
                last_attempted TIMESTAMP,
                last_error TEXT,
                created_at TIMESTAMP NOT NULL,
                updated_at TIMESTAMP NOT NULL,
                UNIQUE(entity_type, entity_id, source_system)
            )
        "#).execute(db).await?;
        
        sqlx::query(r#"
            CREATE INDEX IF NOT EXISTS idx_sync_status_entity 
            ON sync_status(entity_type, entity_id, source_system)
        "#).execute(db).await?;
        
        sqlx::query(r#"
            CREATE INDEX IF NOT EXISTS idx_sync_status_status 
            ON sync_status(status)
        "#).execute(db).await?;
        
        Ok(())
    }
    
    /// Update the synchronization status for an entity
    pub async fn update_sync_status(
        &self,
        entity_type: &str,
        entity_id: &str,
        source_system: &str,
        target_id: Option<&str>,
        status: &str,
        error: Option<&str>,
    ) -> Result<()> {
        let now = Utc::now();
        let last_synced = if status == "SYNCED" { Some(now) } else { None };
        
        // Try to update existing record
        let result = sqlx::query(r#"
            UPDATE sync_status 
            SET 
                target_id = COALESCE($1, target_id),
                status = $2,
                last_synced = COALESCE($3, last_synced),
                last_attempted = $4,
                last_error = $5,
                updated_at = $6
            WHERE 
                entity_type = $7 AND 
                entity_id = $8 AND 
                source_system = $9
        "#)
        .bind(target_id)
        .bind(status)
        .bind(last_synced)
        .bind(now)
        .bind(error)
        .bind(now)
        .bind(entity_type)
        .bind(entity_id)
        .bind(source_system)
        .execute(&self.db)
        .await?;
        
        // If no record was updated, insert a new one
        if result.rows_affected() == 0 {
            sqlx::query(r#"
                INSERT INTO sync_status (
                    entity_type, entity_id, source_system, target_id,
                    status, last_synced, last_attempted, last_error,
                    created_at, updated_at
                ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            "#)
            .bind(entity_type)
            .bind(entity_id)
            .bind(source_system)
            .bind(target_id)
            .bind(status)
            .bind(last_synced)
            .bind(now)
            .bind(error)
            .bind(now)
            .bind(now)
            .execute(&self.db)
            .await?;
        }
        
        Ok(())
    }
    
    /// Get synchronization status for a specific entity
    pub async fn get_sync_status(
        &self,
        entity_type: &str,
        entity_id: &str,
    ) -> Result<SyncStatus> {
        let record = sqlx::query_as!(SyncStatus, r#"
            SELECT 
                entity_type, entity_id, source_system, target_id,
                status, last_synced, last_attempted, last_error,
                created_at, updated_at
            FROM sync_status
            WHERE entity_type = $1 AND entity_id = $2
        "#)
        .bind(entity_type)
        .bind(entity_id)
        .fetch_one(&self.db)
        .await?;
        
        Ok(record)
    }
    
    /// Get entity mapping (target ID) for a source entity
    pub async fn get_target_id(
        &self,
        entity_type: &str,
        entity_id: &str,
        source_system: &str,
    ) -> Result<Option<String>> {
        let record = sqlx::query!(r#"
            SELECT target_id
            FROM sync_status
            WHERE entity_type = $1 AND entity_id = $2 AND source_system = $3
        "#)
        .bind(entity_type)
        .bind(entity_id)
        .bind(source_system)
        .fetch_optional(&self.db)
        .await?;
        
        Ok(record.map(|r| r.target_id).flatten())
    }
    
    /// Get source entity ID from target ID
    pub async fn get_source_id(
        &self,
        entity_type: &str,
        target_id: &str,
        source_system: &str,
    ) -> Result<Option<String>> {
        let record = sqlx::query!(r#"
            SELECT entity_id
            FROM sync_status
            WHERE entity_type = $1 AND target_id = $2 AND source_system = $3
        "#)
        .bind(entity_type)
        .bind(target_id)
        .bind(source_system)
        .fetch_optional(&self.db)
        .await?;
        
        Ok(record.map(|r| r.entity_id))
    }
    
    /// Get all entities with a specific status
    pub async fn get_entities_by_status(
        &self,
        status: &str,
        limit: Option<i32>,
    ) -> Result<Vec<SyncStatus>> {
        let entities = sqlx::query_as!(SyncStatus, r#"
            SELECT 
                entity_type, entity_id, source_system, target_id,
                status, last_synced, last_attempted, last_error,
                created_at, updated_at
            FROM sync_status
            WHERE status = $1
            ORDER BY last_attempted ASC
            LIMIT $2
        "#)
        .bind(status)
        .bind(limit.unwrap_or(100))
        .fetch_all(&self.db)
        .await?;
        
        Ok(entities)
    }
    
    /// Get pending synchronization items
    pub async fn get_pending_syncs(&self, limit: usize) -> Result<Vec<SyncStatus>> {
        self.get_entities_by_status("PENDING", Some(limit as i32)).await
    }
    
    /// Get failed synchronization items
    pub async fn get_failed_syncs(&self, limit: usize) -> Result<Vec<SyncStatus>> {
        self.get_entities_by_status("FAILED", Some(limit as i32)).await
    }
    
    /// Reset synchronization state for an entity
    pub async fn reset_sync_state(
        &self,
        entity_type: &str,
        entity_id: &str,
    ) -> Result<()> {
        sqlx::query(r#"
            UPDATE sync_status
            SET 
                status = 'PENDING',
                last_error = NULL,
                updated_at = $1
            WHERE entity_type = $2 AND entity_id = $3
        "#)
        .bind(Utc::now())
        .bind(entity_type)
        .bind(entity_id)
        .execute(&self.db)
        .await?;
        
        Ok(())
    }
    
    /// Get sync counts by status for an entity type
    pub async fn get_sync_counts_by_status(&self, entity_type: &str) -> Result<HashMap<String, usize>> {
        let records = sqlx::query!(r#"
            SELECT status, COUNT(*) as count
            FROM sync_status
            WHERE entity_type = $1
            GROUP BY status
        "#)
        .bind(entity_type)
        .fetch_all(&self.db)
        .await?;
        
        let mut result = HashMap::new();
        for record in records {
            result.insert(record.status, record.count as usize);
        }
        
        Ok(result)
    }
    
    /// Delete synchronization state for an entity
    pub async fn delete_sync_state(
        &self,
        entity_type: &str,
        entity_id: &str,
        source_system: &str,
    ) -> Result<bool> {
        let result = sqlx::query(r#"
            DELETE FROM sync_status
            WHERE entity_type = $1 AND entity_id = $2 AND source_system = $3
        "#)
        .bind(entity_type)
        .bind(entity_id)
        .bind(source_system)
        .execute(&self.db)
        .await?;
        
        Ok(result.rows_affected() > 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_sync_state_crud() -> Result<()> {
        // Create in-memory database for testing
        let db = SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await?;
        
        // Initialize sync state with test database
        let sync_state = SyncState::with_connection(db);
        SyncState::initialize_schema(&sync_state.db).await?;
        
        // Test creating a sync state
        sync_state.update_sync_status(
            "course", "123", "canvas", Some("456"), "PENDING", None
        ).await?;
        
        // Test retrieving sync state
        let status = sync_state.get_sync_status("course", "123").await?;
        assert_eq!(status.entity_type, "course");
        assert_eq!(status.entity_id, "123");
        assert_eq!(status.status, "PENDING");
        assert_eq!(status.target_id, Some("456".to_string()));
        
        // Test updating sync state
        sync_state.update_sync_status(
            "course", "123", "canvas", None, "SYNCED", None
        ).await?;
        
        // Verify update
        let updated = sync_state.get_sync_status("course", "123").await?;
        assert_eq!(updated.status, "SYNCED");
        assert!(updated.last_synced.is_some());
        
        // Test mapping lookup
        let target_id = sync_state.get_target_id("course", "123", "canvas").await?;
        assert_eq!(target_id, Some("456".to_string()));
        
        // Test reverse mapping
        let source_id = sync_state.get_source_id("course", "456", "canvas").await?;
        assert_eq!(source_id, Some("123".to_string()));
        
        // Test counts by status
        sync_state.update_sync_status(
            "course", "789", "canvas", Some("101"), "FAILED", Some("Test error")
        ).await?;
        
        let counts = sync_state.get_sync_counts_by_status("course").await?;
        assert_eq!(counts.get("SYNCED"), Some(&1));
        assert_eq!(counts.get("FAILED"), Some(&1));
        
        // Test reset
        sync_state.reset_sync_state("course", "789").await?;
        let reset = sync_state.get_sync_status("course", "789").await?;
        assert_eq!(reset.status, "PENDING");
        assert_eq!(reset.last_error, None);
        
        // Test delete
        let deleted = sync_state.delete_sync_state("course", "123", "canvas").await?;
        assert!(deleted);
        
        Ok(())
    }
}
