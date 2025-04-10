use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use sqlx::{Pool, Sqlite};
use uuid::Uuid;
use std::time::Instant;
use log::{info, error};
use thiserror::Error;

/// Error type for transaction operations
#[derive(Debug, Error)]
pub enum TransactionError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    
    #[error("Transaction error: {0}")]
    TransactionError(String),
}

/// Type alias for result with TransactionError
pub type Result<T> = std::result::Result<T, TransactionError>;

/// The status of a sync transaction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TransactionStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
    RolledBack,
}

impl ToString for TransactionStatus {
    fn to_string(&self) -> String {
        match self {
            TransactionStatus::Pending => "pending".to_string(),
            TransactionStatus::InProgress => "in_progress".to_string(),
            TransactionStatus::Completed => "completed".to_string(),
            TransactionStatus::Failed => "failed".to_string(),
            TransactionStatus::RolledBack => "rolled_back".to_string(),
        }
    }
}

/// A synchronization event triggering a transaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncEvent {
    pub transaction_id: Option<String>,
    pub entity_type: String,
    pub entity_id: String,
    pub operation: String,
    pub source_system: String,
    pub target_system: String,
    pub timestamp: DateTime<Utc>,
    pub data: serde_json::Value,
}

/// A step in a synchronization transaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionStep {
    pub transaction_id: String,
    pub timestamp: DateTime<Utc>,
    pub description: String,
    pub data: serde_json::Value,
}

/// Manages the lifecycle of synchronization transactions
#[derive(Debug)]
pub struct SyncTransactionHandler {
    transaction_id: String,
    entity_type: String,
    operation: String,
    source_system: String,
    target_system: String,
    start_time: DateTime<Utc>,
    status: TransactionStatus,
    steps: Vec<TransactionStep>,
    db_pool: Pool<Sqlite>,
    timer: Instant,
}

impl SyncTransactionHandler {
    /// Create a new sync transaction handler
    pub fn new(event: SyncEvent, db_pool: Pool<Sqlite>) -> Self {
        let transaction_id = event.transaction_id
            .unwrap_or_else(|| Uuid::new_v4().to_string());
        
        Self {
            transaction_id,
            entity_type: event.entity_type,
            operation: event.operation,
            source_system: event.source_system,
            target_system: event.target_system,
            start_time: Utc::now(),
            status: TransactionStatus::Pending,
            steps: Vec::new(),
            db_pool,
            timer: Instant::now(),
        }
    }
    
    /// Begin the transaction
    pub async fn begin(&mut self) -> Result<&mut Self> {
        info!("Beginning sync transaction: {}", self.transaction_id);
        
        self.status = TransactionStatus::InProgress;
        
        // Record transaction start in database
        sqlx::query!(
            r#"
            INSERT INTO sync_transactions (
                transaction_id, entity_type, operation, source_system, target_system, 
                start_time, status, event_data
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            "#,
            self.transaction_id,
            self.entity_type,
            self.operation,
            self.source_system,
            self.target_system,
            self.start_time,
            self.status.to_string(),
            serde_json::to_string(&self.steps)?
        )
        .execute(&self.db_pool)
        .await?;
        
        Ok(self)
    }
    
    /// Record a step in the transaction
    pub async fn record_step(&mut self, description: &str, data: serde_json::Value) -> Result<&mut Self> {
        let step = TransactionStep {
            transaction_id: self.transaction_id.clone(),
            timestamp: Utc::now(),
            description: description.to_string(),
            data,
        };
        
        self.steps.push(step.clone());
        
        // Update transaction in database
        sqlx::query!(
            r#"
            INSERT INTO sync_transaction_steps (
                transaction_id, timestamp, description, step_data
            ) VALUES (?, ?, ?, ?)
            "#,
            step.transaction_id,
            step.timestamp,
            step.description,
            serde_json::to_string(&step.data)?
        )
        .execute(&self.db_pool)
        .await?;
        
        Ok(self)
    }
    
    /// Commit the transaction
    pub async fn commit(&mut self) -> Result<bool> {
        info!("Committing sync transaction: {}", self.transaction_id);
        
        self.status = TransactionStatus::Completed;
        let end_time = Utc::now();
        let duration_ms = self.timer.elapsed().as_millis() as i64;
        
        // Update transaction in database
        sqlx::query!(
            r#"
            UPDATE sync_transactions 
            SET status = ?, end_time = ?, duration_ms = ?
            WHERE transaction_id = ?
            "#,
            self.status.to_string(),
            end_time,
            duration_ms,
            self.transaction_id
        )
        .execute(&self.db_pool)
        .await?;
        
        Ok(true)
    }
    
    /// Roll back the transaction
    pub async fn rollback(&mut self, error_message: &str) -> Result<bool> {
        error!("Rolling back sync transaction: {}, Error: {}", self.transaction_id, error_message);
        
        self.status = TransactionStatus::RolledBack;
        let end_time = Utc::now();
        let duration_ms = self.timer.elapsed().as_millis() as i64;
        
        // Update transaction in database
        sqlx::query!(
            r#"
            UPDATE sync_transactions 
            SET status = ?, end_time = ?, duration_ms = ?, error_message = ?
            WHERE transaction_id = ?
            "#,
            self.status.to_string(),
            end_time,
            duration_ms,
            error_message,
            self.transaction_id
        )
        .execute(&self.db_pool)
        .await?;
        
        // Add rollback step
        self.record_step("Transaction rolled back", serde_json::json!({
            "error": error_message
        }))
        .await?;
        
        Ok(true)
    }
    
    /// Mark the transaction as failed
    pub async fn fail(&mut self, error_message: &str) -> Result<bool> {
        error!("Marking sync transaction as failed: {}, Error: {}", self.transaction_id, error_message);
        
        self.status = TransactionStatus::Failed;
        let end_time = Utc::now();
        let duration_ms = self.timer.elapsed().as_millis() as i64;
        
        // Update transaction in database
        sqlx::query!(
            r#"
            UPDATE sync_transactions 
            SET status = ?, end_time = ?, duration_ms = ?, error_message = ?
            WHERE transaction_id = ?
            "#,
            self.status.to_string(),
            end_time,
            duration_ms,
            error_message,
            self.transaction_id
        )
        .execute(&self.db_pool)
        .await?;
        
        // Add failure step
        self.record_step("Transaction failed", serde_json::json!({
            "error": error_message
        }))
        .await?;
        
        Ok(true)
    }
    
    /// Get the transaction ID
    pub fn transaction_id(&self) -> &str {
        &self.transaction_id
    }
    
    /// Get the transaction status
    pub fn status(&self) -> TransactionStatus {
        self.status
    }
    
    /// Get the transaction steps
    pub fn steps(&self) -> &Vec<TransactionStep> {
        &self.steps
    }
    
    /// Create database tables required for transaction handling
    pub async fn create_tables(db_pool: &Pool<Sqlite>) -> Result<()> {
        // Create transactions table
        sqlx::query!(
            r#"
            CREATE TABLE IF NOT EXISTS sync_transactions (
                transaction_id TEXT PRIMARY KEY,
                entity_type TEXT NOT NULL,
                operation TEXT NOT NULL,
                source_system TEXT NOT NULL,
                target_system TEXT NOT NULL,
                start_time TEXT NOT NULL,
                end_time TEXT,
                status TEXT NOT NULL,
                duration_ms INTEGER,
                error_message TEXT,
                event_data TEXT
            )
            "#
        )
        .execute(db_pool)
        .await?;
        
        // Create transaction steps table
        sqlx::query!(
            r#"
            CREATE TABLE IF NOT EXISTS sync_transaction_steps (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                transaction_id TEXT NOT NULL,
                timestamp TEXT NOT NULL,
                description TEXT NOT NULL,
                step_data TEXT,
                FOREIGN KEY (transaction_id) REFERENCES sync_transactions (transaction_id)
            )
            "#
        )
        .execute(db_pool)
        .await?;
        
        Ok(())
    }
    
    /// Get a transaction by ID
    pub async fn get_by_id(db_pool: &Pool<Sqlite>, transaction_id: &str) -> Result<Option<serde_json::Value>> {
        let transaction = sqlx::query!(
            r#"
            SELECT * FROM sync_transactions
            WHERE transaction_id = ?
            "#,
            transaction_id
        )
        .fetch_optional(db_pool)
        .await?;
        
        let transaction = match transaction {
            Some(record) => {
                // Get steps for this transaction
                let steps = sqlx::query!(
                    r#"
                    SELECT * FROM sync_transaction_steps
                    WHERE transaction_id = ?
                    ORDER BY timestamp
                    "#,
                    transaction_id
                )
                .fetch_all(db_pool)
                .await?;
                
                let steps = steps.iter().map(|step| {
                    serde_json::json!({
                        "transaction_id": step.transaction_id,
                        "timestamp": step.timestamp,
                        "description": step.description,
                        "data": serde_json::from_str::<serde_json::Value>(&step.step_data.clone().unwrap_or_else(|| "{}".to_string())).unwrap_or_else(|_| serde_json::json!({}))
                    })
                }).collect::<Vec<_>>();
                
                Some(serde_json::json!({
                    "transaction_id": record.transaction_id,
                    "entity_type": record.entity_type,
                    "operation": record.operation,
                    "source_system": record.source_system,
                    "target_system": record.target_system,
                    "start_time": record.start_time,
                    "end_time": record.end_time,
                    "status": record.status,
                    "duration_ms": record.duration_ms,
                    "error_message": record.error_message,
                    "steps": steps
                }))
            },
            None => None,
        };
        
        Ok(transaction)
    }
    
    /// List recent transactions
    pub async fn list_recent(db_pool: &Pool<Sqlite>, limit: i64) -> Result<Vec<serde_json::Value>> {
        let transactions = sqlx::query!(
            r#"
            SELECT * FROM sync_transactions
            ORDER BY start_time DESC
            LIMIT ?
            "#,
            limit
        )
        .fetch_all(db_pool)
        .await?;
        
        let result = transactions.iter().map(|record| {
            serde_json::json!({
                "transaction_id": record.transaction_id,
                "entity_type": record.entity_type,
                "operation": record.operation,
                "source_system": record.source_system,
                "target_system": record.target_system,
                "start_time": record.start_time,
                "end_time": record.end_time,
                "status": record.status,
                "duration_ms": record.duration_ms,
                "error_message": record.error_message
            })
        }).collect();
        
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::sqlite::SqlitePoolOptions;
    
    async fn setup_test_db() -> Pool<Sqlite> {
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect("sqlite::memory:")
            .await
            .expect("Failed to connect to in-memory SQLite");
            
        SyncTransactionHandler::create_tables(&pool)
            .await
            .expect("Failed to create tables");
            
        pool
    }
    
    #[tokio::test]
    async fn test_transaction_lifecycle() {
        let pool = setup_test_db().await;
        
        let event = SyncEvent {
            transaction_id: None,
            entity_type: "course".to_string(),
            entity_id: "course-123".to_string(),
            operation: "create".to_string(),
            source_system: "canvas".to_string(),
            target_system: "discourse".to_string(),
            timestamp: Utc::now(),
            data: serde_json::json!({ "name": "Test Course" }),
        };
        
        let mut tx = SyncTransactionHandler::new(event, pool.clone());
        
        // Begin transaction
        tx.begin().await.expect("Failed to begin transaction");
        assert_eq!(tx.status(), TransactionStatus::InProgress);
        
        // Record steps
        tx.record_step("Preparing data", serde_json::json!({ "step": 1 }))
            .await
            .expect("Failed to record step");
            
        tx.record_step("Processing data", serde_json::json!({ "step": 2 }))
            .await
            .expect("Failed to record step");
            
        assert_eq!(tx.steps().len(), 2);
        
        // Commit transaction
        tx.commit().await.expect("Failed to commit transaction");
        assert_eq!(tx.status(), TransactionStatus::Completed);
        
        // Verify transaction exists in database
        let loaded_tx = SyncTransactionHandler::get_by_id(&pool, tx.transaction_id())
            .await
            .expect("Failed to get transaction by ID");
            
        assert!(loaded_tx.is_some());
        
        let loaded_tx = loaded_tx.unwrap();
        assert_eq!(loaded_tx["status"], "completed");
        assert_eq!(loaded_tx["entity_type"], "course");
        assert_eq!(loaded_tx["operation"], "create");
        
        let steps = loaded_tx["steps"].as_array().unwrap();
        assert_eq!(steps.len(), 2);
        assert_eq!(steps[0]["description"], "Preparing data");
        assert_eq!(steps[1]["description"], "Processing data");
    }
    
    #[tokio::test]
    async fn test_transaction_rollback() {
        let pool = setup_test_db().await;
        
        let event = SyncEvent {
            transaction_id: None,
            entity_type: "user".to_string(),
            entity_id: "user-456".to_string(),
            operation: "update".to_string(),
            source_system: "canvas".to_string(),
            target_system: "discourse".to_string(),
            timestamp: Utc::now(),
            data: serde_json::json!({ "username": "Test User" }),
        };
        
        let mut tx = SyncTransactionHandler::new(event, pool.clone());
        
        // Begin transaction
        tx.begin().await.expect("Failed to begin transaction");
        
        // Record a step
        tx.record_step("Starting update", serde_json::json!({ "username": "Test User" }))
            .await
            .expect("Failed to record step");
            
        // Rollback transaction
        tx.rollback("Test error occurred").await.expect("Failed to rollback transaction");
        assert_eq!(tx.status(), TransactionStatus::RolledBack);
        
        // Verify transaction status in database
        let loaded_tx = SyncTransactionHandler::get_by_id(&pool, tx.transaction_id())
            .await
            .expect("Failed to get transaction by ID")
            .unwrap();
            
        assert_eq!(loaded_tx["status"], "rolled_back");
        assert_eq!(loaded_tx["error_message"], "Test error occurred");
        
        let steps = loaded_tx["steps"].as_array().unwrap();
        assert_eq!(steps.len(), 2); // Original step + rollback step
        assert_eq!(steps[1]["description"], "Transaction rolled back");
    }
    
    #[tokio::test]
    async fn test_list_recent_transactions() {
        let pool = setup_test_db().await;
        
        // Create several transactions
        for i in 0..5 {
            let event = SyncEvent {
                transaction_id: None,
                entity_type: "test".to_string(),
                entity_id: format!("entity-{}", i),
                operation: "test".to_string(),
                source_system: "source".to_string(),
                target_system: "target".to_string(),
                timestamp: Utc::now(),
                data: serde_json::json!({}),
            };
            
            let mut tx = SyncTransactionHandler::new(event, pool.clone());
            tx.begin().await.expect("Failed to begin transaction");
            tx.commit().await.expect("Failed to commit transaction");
        }
        
        // List recent transactions
        let transactions = SyncTransactionHandler::list_recent(&pool, 3)
            .await
            .expect("Failed to list recent transactions");
            
        assert_eq!(transactions.len(), 3);
        assert_eq!(transactions[0]["entity_id"], "entity-4");
        assert_eq!(transactions[1]["entity_id"], "entity-3");
        assert_eq!(transactions[2]["entity_id"], "entity-2");
    }
}
