use anyhow::{Result, Context};
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use sqlx::{Pool, Sqlite, sqlite::SqlitePoolOptions};
use uuid::Uuid;

/// Synchronization transaction model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: String,
    pub entity_type: String,
    pub entity_id: String,
    pub operation: String,
    pub source_system: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

/// Transaction step for detailed logging
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionStep {
    pub id: String,
    pub transaction_id: String,
    pub step_name: String,
    pub details: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// Sync transaction manager
pub struct SyncTransaction {
    db: Pool<Sqlite>,
}

impl SyncTransaction {
    /// Create a new transaction manager with an SQLite database connection
    pub async fn new() -> Result<Self> {
        // Create or connect to SQLite database
        let db = SqlitePoolOptions::new()
            .max_connections(5)
            .connect("sqlite:sync_transactions.db")
            .await
            .context("Failed to connect to transaction database")?;
        
        // Initialize database schema
        Self::initialize_schema(&db).await?;
        
        Ok(Self { db })
    }
    
    /// Create with a specific database connection
    pub fn with_connection(db: Pool<Sqlite>) -> Self {
        Self { db }
    }
    
    /// Initialize the database schema
    async fn initialize_schema(db: &Pool<Sqlite>) -> Result<()> {
        // Create transactions table
        sqlx::query(r#"
            CREATE TABLE IF NOT EXISTS transactions (
                id TEXT PRIMARY KEY,
                entity_type TEXT NOT NULL,
                entity_id TEXT NOT NULL,
                operation TEXT NOT NULL,
                source_system TEXT NOT NULL,
                status TEXT NOT NULL,
                created_at TIMESTAMP NOT NULL,
                completed_at TIMESTAMP
            )
        "#).execute(db).await?;
        
        // Create transaction steps table
        sqlx::query(r#"
            CREATE TABLE IF NOT EXISTS transaction_steps (
                id TEXT PRIMARY KEY,
                transaction_id TEXT NOT NULL,
                step_name TEXT NOT NULL,
                details TEXT,
                created_at TIMESTAMP NOT NULL,
                FOREIGN KEY (transaction_id) REFERENCES transactions (id)
            )
        "#).execute(db).await?;
        
        // Create indices
        sqlx::query(r#"
            CREATE INDEX IF NOT EXISTS idx_transactions_entity 
            ON transactions(entity_type, entity_id)
        "#).execute(db).await?;
        
        sqlx::query(r#"
            CREATE INDEX IF NOT EXISTS idx_transaction_steps_tx 
            ON transaction_steps(transaction_id)
        "#).execute(db).await?;
        
        Ok(())
    }
    
    /// Begin a new transaction and return the transaction object
    pub async fn begin_transaction(
        &self,
        entity_type: &str,
        entity_id: &str,
        operation: &str,
        source_system: &str,
    ) -> Result<Transaction> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now();
        
        let transaction = Transaction {
            id: id.clone(),
            entity_type: entity_type.to_string(),
            entity_id: entity_id.to_string(),
            operation: operation.to_string(),
            source_system: source_system.to_string(),
            status: "STARTED".to_string(),
            created_at: now,
            completed_at: None,
        };
        
        // Insert into database
        sqlx::query(r#"
            INSERT INTO transactions (
                id, entity_type, entity_id, operation, source_system, status, created_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7)
        "#)
        .bind(&transaction.id)
        .bind(&transaction.entity_type)
        .bind(&transaction.entity_id)
        .bind(&transaction.operation)
        .bind(&transaction.source_system)
        .bind(&transaction.status)
        .bind(&transaction.created_at)
        .execute(&self.db)
        .await
        .context("Failed to insert transaction")?;
        
        Ok(transaction)
    }
    
    /// Record a step in the transaction
    pub async fn record_step(
        &self,
        transaction_id: &str,
        step_name: &str,
        details: &str,
    ) -> Result<TransactionStep> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now();
        
        let step = TransactionStep {
            id: id.clone(),
            transaction_id: transaction_id.to_string(),
            step_name: step_name.to_string(),
            details: Some(details.to_string()),
            created_at: now,
        };
        
        // Insert into database
        sqlx::query(r#"
            INSERT INTO transaction_steps (
                id, transaction_id, step_name, details, created_at
            ) VALUES ($1, $2, $3, $4, $5)
        "#)
        .bind(&step.id)
        .bind(&step.transaction_id)
        .bind(&step.step_name)
        .bind(&step.details)
        .bind(&step.created_at)
        .execute(&self.db)
        .await
        .context("Failed to insert transaction step")?;
        
        Ok(step)
    }
    
    /// Mark a transaction as committed (successful)
    pub async fn commit(&self, transaction_id: &str) -> Result<()> {
        let now = Utc::now();
        
        sqlx::query(r#"
            UPDATE transactions 
            SET status = 'COMMITTED', completed_at = $1
            WHERE id = $2
        "#)
        .bind(now)
        .bind(transaction_id)
        .execute(&self.db)
        .await?;
        
        // Record commit step
        self.record_step(
            transaction_id,
            "COMMIT",
            "Transaction completed successfully"
        ).await?;
        
        Ok(())
    }
    
    /// Mark a transaction as rolled back (failed)
    pub async fn rollback(&self, transaction_id: &str, reason: &str) -> Result<()> {
        let now = Utc::now();
        
        sqlx::query(r#"
            UPDATE transactions 
            SET status = 'ROLLED_BACK', completed_at = $1
            WHERE id = $2
        "#)
        .bind(now)
        .bind(transaction_id)
        .execute(&self.db)
        .await?;
        
        // Record rollback step
        self.record_step(
            transaction_id,
            "ROLLBACK",
            &format!("Transaction failed: {}", reason)
        ).await?;
        
        Ok(())
    }
    
    /// Get a transaction by ID
    pub async fn get_transaction(&self, transaction_id: &str) -> Result<Transaction> {
        let transaction = sqlx::query_as!(Transaction, r#"
            SELECT 
                id, entity_type, entity_id, operation, 
                source_system, status, created_at, completed_at
            FROM transactions
            WHERE id = $1
        "#)
        .bind(transaction_id)
        .fetch_one(&self.db)
        .await
        .context("Transaction not found")?;
        
        Ok(transaction)
    }
    
    /// Get steps for a transaction
    pub async fn get_transaction_steps(&self, transaction_id: &str) -> Result<Vec<TransactionStep>> {
        let steps = sqlx::query_as!(TransactionStep, r#"
            SELECT 
                id, transaction_id, step_name, details, created_at
            FROM transaction_steps
            WHERE transaction_id = $1
            ORDER BY created_at ASC
        "#)
        .bind(transaction_id)
        .fetch_all(&self.db)
        .await
        .context("Failed to fetch transaction steps")?;
        
        Ok(steps)
    }
    
    /// Get recent transactions for an entity
    pub async fn get_entity_transactions(
        &self,
        entity_type: &str,
        entity_id: &str,
        limit: i32,
    ) -> Result<Vec<Transaction>> {
        let transactions = sqlx::query_as!(Transaction, r#"
            SELECT 
                id, entity_type, entity_id, operation, 
                source_system, status, created_at, completed_at
            FROM transactions
            WHERE entity_type = $1 AND entity_id = $2
            ORDER BY created_at DESC
            LIMIT $3
        "#)
        .bind(entity_type)
        .bind(entity_id)
        .bind(limit)
        .fetch_all(&self.db)
        .await
        .context("Failed to fetch entity transactions")?;
        
        Ok(transactions)
    }
    
    /// Get failed transactions within a time period
    pub async fn get_failed_transactions(
        &self,
        hours: i64,
        limit: i32,
    ) -> Result<Vec<Transaction>> {
        let cutoff = Utc::now() - chrono::Duration::hours(hours);
        
        let transactions = sqlx::query_as!(Transaction, r#"
            SELECT 
                id, entity_type, entity_id, operation, 
                source_system, status, created_at, completed_at
            FROM transactions
            WHERE status = 'ROLLED_BACK' AND created_at > $1
            ORDER BY created_at DESC
            LIMIT $2
        "#)
        .bind(cutoff)
        .bind(limit)
        .fetch_all(&self.db)
        .await
        .context("Failed to fetch failed transactions")?;
        
        Ok(transactions)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_transaction_lifecycle() -> Result<()> {
        // Create in-memory database for testing
        let db = SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await?;
        
        // Initialize transaction manager with test database
        let tx_manager = SyncTransaction::with_connection(db);
        SyncTransaction::initialize_schema(&tx_manager.db).await?;
        
        // Begin a transaction
        let transaction = tx_manager.begin_transaction(
            "course", "123", "update", "canvas"
        ).await?;
        
        assert_eq!(transaction.entity_type, "course");
        assert_eq!(transaction.entity_id, "123");
        assert_eq!(transaction.status, "STARTED");
        
        // Record some steps
        let step1 = tx_manager.record_step(
            &transaction.id, "VALIDATION", "Validating course data"
        ).await?;
        
        assert_eq!(step1.step_name, "VALIDATION");
        
        let step2 = tx_manager.record_step(
            &transaction.id, "PROCESSING", "Processing course data"
        ).await?;
        
        // Commit the transaction
        tx_manager.commit(&transaction.id).await?;
        
        // Verify transaction status
        let updated_tx = tx_manager.get_transaction(&transaction.id).await?;
        assert_eq!(updated_tx.status, "COMMITTED");
        assert!(updated_tx.completed_at.is_some());
        
        // Verify steps
        let steps = tx_manager.get_transaction_steps(&transaction.id).await?;
        assert_eq!(steps.len(), 3); // Initial two steps plus COMMIT step
        assert_eq!(steps[2].step_name, "COMMIT");
        
        // Begin another transaction that will fail
        let tx2 = tx_manager.begin_transaction(
            "user", "456", "create", "canvas"
        ).await?;
        
        // Record a step
        tx_manager.record_step(
            &tx2.id, "VALIDATION", "Validating user data"
        ).await?;
        
        // Roll back the transaction
        tx_manager.rollback(&tx2.id, "Invalid user data").await?;
        
        // Verify transaction status
        let updated_tx2 = tx_manager.get_transaction(&tx2.id).await?;
        assert_eq!(updated_tx2.status, "ROLLED_BACK");
        
        // Get entity transactions
        let course_txs = tx_manager.get_entity_transactions("course", "123", 10).await?;
        assert_eq!(course_txs.len(), 1);
        assert_eq!(course_txs[0].id, transaction.id);
        
        // Get failed transactions
        let failed_txs = tx_manager.get_failed_transactions(24, 10).await?;
        assert_eq!(failed_txs.len(), 1);
        assert_eq!(failed_txs[0].id, tx2.id);
        
        Ok(())
    }
}
