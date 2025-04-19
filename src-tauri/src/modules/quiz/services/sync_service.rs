use std::sync::Arc;
use anyhow::{Result, anyhow};
use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::Utc;
use serde::{Serialize, Deserialize};
use tracing::{info, error, warn};
use std::path::{Path, PathBuf};
use std::fs;
use std::io::Write;

use crate::database::repositories::quiz_repository::QuizRepository;
use crate::models::quiz::{
    Quiz, Question, AnswerOption, QuizAttempt, UserAnswer, QuizSettings,
};

/// Sync operation type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SyncOperation {
    Create,
    Update,
    Delete,
}

/// Sync priority
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum SyncPriority {
    Low,
    Medium,
    High,
    Critical,
}

/// Sync item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncItem {
    pub id: String,
    pub entity_type: String,
    pub entity_id: String,
    pub operation: SyncOperation,
    pub data: serde_json::Value,
    pub priority: SyncPriority,
    pub status: SyncStatus,
    pub created_at: String,
    pub updated_at: String,
    pub synced_at: Option<String>,
    pub error: Option<String>,
    pub retry_count: i32,
}

/// Sync status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SyncStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
}

/// Sync service for the quiz module
pub struct QuizSyncService {
    repository: Arc<QuizRepository>,
    sync_dir: PathBuf,
}

impl QuizSyncService {
    /// Create a new QuizSyncService
    pub fn new(repository: Arc<QuizRepository>, data_dir: &Path) -> Result<Self> {
        let sync_dir = data_dir.join("sync").join("quiz");
        
        // Create sync directory if it doesn't exist
        if !sync_dir.exists() {
            fs::create_dir_all(&sync_dir)?;
        }
        
        Ok(Self {
            repository,
            sync_dir,
        })
    }

    /// Queue an item for sync
    pub async fn queue_sync_item(
        &self,
        entity_type: &str,
        entity_id: &str,
        operation: SyncOperation,
        data: serde_json::Value,
        priority: SyncPriority,
    ) -> Result<String> {
        let sync_id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();
        
        let sync_item = SyncItem {
            id: sync_id.clone(),
            entity_type: entity_type.to_string(),
            entity_id: entity_id.to_string(),
            operation,
            data,
            priority,
            status: SyncStatus::Pending,
            created_at: now.clone(),
            updated_at: now,
            synced_at: None,
            error: None,
            retry_count: 0,
        };
        
        // Save to sync file
        let sync_file = self.sync_dir.join(format!("{}.json", sync_id));
        let sync_data = serde_json::to_string_pretty(&sync_item)?;
        let mut file = fs::File::create(sync_file)?;
        file.write_all(sync_data.as_bytes())?;
        
        // Also save to database for faster querying
        sqlx::query!(
            r#"
            INSERT INTO quiz_sync_items (
                id, entity_type, entity_id, operation, data, priority, status,
                created_at, updated_at, synced_at, error, retry_count
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
            sync_item.id,
            sync_item.entity_type,
            sync_item.entity_id,
            format!("{:?}", sync_item.operation).to_lowercase(),
            sync_data,
            format!("{:?}", sync_item.priority).to_lowercase(),
            format!("{:?}", sync_item.status).to_lowercase(),
            sync_item.created_at,
            sync_item.updated_at,
            sync_item.synced_at,
            sync_item.error,
            sync_item.retry_count
        )
        .execute(self.repository.get_pool())
        .await?;
        
        info!("Queued sync item: {} - {} - {}", entity_type, entity_id, format!("{:?}", operation));
        
        Ok(sync_id)
    }

    /// Get pending sync items
    pub async fn get_pending_sync_items(&self) -> Result<Vec<SyncItem>> {
        let rows = sqlx::query!(
            r#"
            SELECT id, data
            FROM quiz_sync_items
            WHERE status = 'pending'
            ORDER BY 
                CASE priority
                    WHEN 'critical' THEN 1
                    WHEN 'high' THEN 2
                    WHEN 'medium' THEN 3
                    WHEN 'low' THEN 4
                    ELSE 5
                END,
                created_at ASC
            "#
        )
        .fetch_all(self.repository.get_pool())
        .await?;
        
        let mut items = Vec::new();
        for row in rows {
            let item: SyncItem = serde_json::from_str(&row.data)?;
            items.push(item);
        }
        
        Ok(items)
    }

    /// Process sync items
    pub async fn process_sync_items(&self) -> Result<usize> {
        let items = self.get_pending_sync_items().await?;
        let mut processed_count = 0;
        
        for item in items {
            match self.process_sync_item(&item).await {
                Ok(_) => {
                    processed_count += 1;
                    self.mark_sync_item_completed(&item.id).await?;
                },
                Err(e) => {
                    error!("Failed to process sync item {}: {}", item.id, e);
                    self.mark_sync_item_failed(&item.id, &e.to_string()).await?;
                }
            }
        }
        
        Ok(processed_count)
    }

    /// Process a single sync item
    async fn process_sync_item(&self, item: &SyncItem) -> Result<()> {
        // Mark as in progress
        self.mark_sync_item_in_progress(&item.id).await?;
        
        // Process based on entity type and operation
        match (item.entity_type.as_str(), &item.operation) {
            ("quiz", SyncOperation::Create) => {
                let quiz: Quiz = serde_json::from_value(item.data.clone())?;
                // Handle quiz creation sync
                info!("Syncing quiz creation: {}", quiz.id);
                // Implementation would depend on how quizzes are stored in the main app
            },
            ("quiz", SyncOperation::Update) => {
                let quiz: Quiz = serde_json::from_value(item.data.clone())?;
                // Handle quiz update sync
                info!("Syncing quiz update: {}", quiz.id);
            },
            ("quiz_attempt", SyncOperation::Create) | ("quiz_attempt", SyncOperation::Update) => {
                let attempt: QuizAttempt = serde_json::from_value(item.data.clone())?;
                // Handle attempt sync
                info!("Syncing quiz attempt: {}", attempt.id);
                // This is particularly important for tracking study activity
            },
            ("user_answer", SyncOperation::Create) => {
                let answer: UserAnswer = serde_json::from_value(item.data.clone())?;
                // Handle user answer sync
                info!("Syncing user answer: {}", answer.id);
            },
            _ => {
                warn!("Unsupported sync item: {} - {}", item.entity_type, format!("{:?}", item.operation));
            }
        }
        
        Ok(())
    }

    /// Mark a sync item as in progress
    async fn mark_sync_item_in_progress(&self, id: &str) -> Result<()> {
        let now = Utc::now().to_rfc3339();
        
        sqlx::query!(
            r#"
            UPDATE quiz_sync_items
            SET status = 'in_progress', updated_at = ?
            WHERE id = ?
            "#,
            now,
            id
        )
        .execute(self.repository.get_pool())
        .await?;
        
        Ok(())
    }

    /// Mark a sync item as completed
    async fn mark_sync_item_completed(&self, id: &str) -> Result<()> {
        let now = Utc::now().to_rfc3339();
        
        sqlx::query!(
            r#"
            UPDATE quiz_sync_items
            SET status = 'completed', updated_at = ?, synced_at = ?
            WHERE id = ?
            "#,
            now,
            now,
            id
        )
        .execute(self.repository.get_pool())
        .await?;
        
        Ok(())
    }

    /// Mark a sync item as failed
    async fn mark_sync_item_failed(&self, id: &str, error: &str) -> Result<()> {
        let now = Utc::now().to_rfc3339();
        
        sqlx::query!(
            r#"
            UPDATE quiz_sync_items
            SET 
                status = 'failed', 
                updated_at = ?, 
                error = ?,
                retry_count = retry_count + 1
            WHERE id = ?
            "#,
            now,
            error,
            id
        )
        .execute(self.repository.get_pool())
        .await?;
        
        Ok(())
    }

    /// Export sync data to a file
    pub async fn export_sync_data(&self, path: &Path) -> Result<()> {
        // Get all sync items
        let rows = sqlx::query!(
            r#"
            SELECT data
            FROM quiz_sync_items
            WHERE status = 'pending' OR status = 'failed'
            ORDER BY created_at ASC
            "#
        )
        .fetch_all(self.repository.get_pool())
        .await?;
        
        let mut items = Vec::new();
        for row in rows {
            let item: SyncItem = serde_json::from_str(&row.data)?;
            items.push(item);
        }
        
        // Export to file
        let sync_data = serde_json::to_string_pretty(&items)?;
        let mut file = fs::File::create(path)?;
        file.write_all(sync_data.as_bytes())?;
        
        info!("Exported {} sync items to {}", items.len(), path.display());
        
        Ok(())
    }

    /// Import sync data from a file
    pub async fn import_sync_data(&self, path: &Path) -> Result<usize> {
        // Read file
        let sync_data = fs::read_to_string(path)?;
        let items: Vec<SyncItem> = serde_json::from_str(&sync_data)?;
        
        let mut imported_count = 0;
        for item in items {
            // Check if item already exists
            let existing = sqlx::query!(
                r#"
                SELECT id FROM quiz_sync_items
                WHERE id = ?
                "#,
                item.id
            )
            .fetch_optional(self.repository.get_pool())
            .await?;
            
            if existing.is_none() {
                // Save to sync file
                let sync_file = self.sync_dir.join(format!("{}.json", item.id));
                let sync_data = serde_json::to_string_pretty(&item)?;
                let mut file = fs::File::create(sync_file)?;
                file.write_all(sync_data.as_bytes())?;
                
                // Save to database
                sqlx::query!(
                    r#"
                    INSERT INTO quiz_sync_items (
                        id, entity_type, entity_id, operation, data, priority, status,
                        created_at, updated_at, synced_at, error, retry_count
                    )
                    VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                    "#,
                    item.id,
                    item.entity_type,
                    item.entity_id,
                    format!("{:?}", item.operation).to_lowercase(),
                    sync_data,
                    format!("{:?}", item.priority).to_lowercase(),
                    format!("{:?}", item.status).to_lowercase(),
                    item.created_at,
                    item.updated_at,
                    item.synced_at,
                    item.error,
                    item.retry_count
                )
                .execute(self.repository.get_pool())
                .await?;
                
                imported_count += 1;
            }
        }
        
        info!("Imported {} sync items from {}", imported_count, path.display());
        
        Ok(imported_count)
    }

    /// Initialize the sync database
    pub async fn init_sync_db(&self) -> Result<()> {
        // Create sync tables if they don't exist
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS quiz_sync_items (
                id TEXT PRIMARY KEY,
                entity_type TEXT NOT NULL,
                entity_id TEXT NOT NULL,
                operation TEXT NOT NULL,
                data TEXT NOT NULL,
                priority TEXT NOT NULL,
                status TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                synced_at TEXT,
                error TEXT,
                retry_count INTEGER NOT NULL DEFAULT 0
            );
            
            CREATE INDEX IF NOT EXISTS idx_quiz_sync_items_status ON quiz_sync_items(status);
            CREATE INDEX IF NOT EXISTS idx_quiz_sync_items_entity ON quiz_sync_items(entity_type, entity_id);
            "#
        )
        .execute(self.repository.get_pool())
        .await?;
        
        Ok(())
    }
}
