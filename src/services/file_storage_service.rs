use axum::body::Bytes;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres, query, query_as};
use std::path::{Path, PathBuf};
use std::fs;
use std::io;
use std::sync::Arc;
use tokio::task;
use tokio::fs as tokio_fs;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use uuid::Uuid;
use moka::sync::Cache;
use std::time::Duration;

// File metadata model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMetadata {
    pub id: String,
    pub name: String,
    pub path: String,
    pub content_type: String,
    pub size: i64,
    pub is_public: bool,
    pub user_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub md5_hash: Option<String>,
}

// File attachment model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileAttachment {
    pub id: String,
    pub file_id: String,
    pub entity_type: String,
    pub entity_id: String,
    pub created_at: DateTime<Utc>,
}

// Request models
#[derive(Debug, Deserialize)]
pub struct FileUploadRequest {
    pub name: String,
    pub content_type: String,
    pub content: String, // Base64 encoded content
    pub is_public: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct FileAttachmentRequest {
    pub entity_type: String,
    pub entity_id: String,
}

#[derive(Debug, Deserialize)]
pub struct FileVisibilityRequest {
    pub is_public: bool,
}

// Query parameters for file listing
#[derive(Debug, Deserialize)]
pub struct FileListParams {
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub entity_type: Option<String>,
    pub entity_id: Option<String>,
    pub search: Option<String>,
}

// File storage service
pub struct FileStorageService {
    db: Pool<Postgres>,
    storage_path: PathBuf,
    file_cache: Cache<String, FileMetadata>,
    content_cache: Cache<String, Bytes>,
}

impl FileStorageService {
    // Initialize the file storage service
    pub fn new(db: Pool<Postgres>, storage_path_str: &str) -> Self {
        let storage_path = PathBuf::from(storage_path_str);
        
        // Create storage directory if it doesn't exist
        if !storage_path.exists() {
            fs::create_dir_all(&storage_path).expect("Failed to create storage directory");
        }

        // Configure caches
        let file_cache = Cache::builder()
            .max_capacity(1000)
            .time_to_live(Duration::from_secs(300)) // 5 minutes TTL
            .build();

        let content_cache = Cache::builder()
            .max_capacity(100) // Limit content cache to avoid memory issues
            .time_to_live(Duration::from_secs(120)) // 2 minutes TTL
            .build();

        Self {
            db,
            storage_path,
            file_cache,
            content_cache,
        }
    }
    
    // Initialize database schema
    pub async fn init_schema(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Create file_metadata table
        query!(
            r#"
            CREATE TABLE IF NOT EXISTS file_metadata (
                id VARCHAR(255) PRIMARY KEY,
                name VARCHAR(255) NOT NULL,
                path VARCHAR(1024) NOT NULL,
                content_type VARCHAR(255) NOT NULL,
                size BIGINT NOT NULL,
                is_public BOOLEAN NOT NULL DEFAULT FALSE,
                user_id VARCHAR(255),
                created_at TIMESTAMP WITH TIME ZONE NOT NULL,
                updated_at TIMESTAMP WITH TIME ZONE NOT NULL,
                md5_hash VARCHAR(32),
                CONSTRAINT unique_file_hash UNIQUE (md5_hash, size)
            )
            "#
        )
        .execute(&self.db)
        .await?;
        
        // Create file_attachments table
        query!(
            r#"
            CREATE TABLE IF NOT EXISTS file_attachments (
                id VARCHAR(255) PRIMARY KEY,
                file_id VARCHAR(255) NOT NULL,
                entity_type VARCHAR(100) NOT NULL,
                entity_id VARCHAR(255) NOT NULL,
                created_at TIMESTAMP WITH TIME ZONE NOT NULL,
                CONSTRAINT fk_file_attachment_file FOREIGN KEY (file_id) REFERENCES file_metadata(id) ON DELETE CASCADE,
                CONSTRAINT unique_file_entity UNIQUE (file_id, entity_type, entity_id)
            )
            "#
        )
        .execute(&self.db)
        .await?;
        
        // Create indexes
        query!(
            r#"
            CREATE INDEX IF NOT EXISTS idx_file_metadata_user ON file_metadata(user_id);
            CREATE INDEX IF NOT EXISTS idx_file_metadata_created ON file_metadata(created_at);
            CREATE INDEX IF NOT EXISTS idx_file_metadata_name ON file_metadata(name);
            CREATE INDEX IF NOT EXISTS idx_file_attachments_entity ON file_attachments(entity_type, entity_id);
            "#
        )
        .execute(&self.db)
        .await?;
        
        Ok(())
    }

    // Store file from raw bytes
    pub async fn store_file(
        &self, 
        name: &str, 
        content_type: &str, 
        bytes: Bytes, 
        is_public: bool,
        user_id: Option<String>
    ) -> Result<FileMetadata, Box<dyn std::error::Error + Send + Sync>> {
        let file_id = Uuid::new_v4().to_string();
        let file_path = self.generate_file_path(&file_id);
        
        // Create directory structure if needed
        if let Some(parent) = file_path.parent() {
            tokio_fs::create_dir_all(parent).await?;
        }
        
        // Write file to disk
        tokio_fs::write(&file_path, &bytes).await?;
        
        // Calculate MD5 hash
        let hash = task::spawn_blocking(move || {
            use md5::{Md5, Digest};
            let mut hasher = Md5::new();
            hasher.update(&bytes);
            let result = hasher.finalize();
            format!("{:x}", result)
        }).await?;
        
        let size = bytes.len() as i64;
        let relative_path = self.get_relative_path(&file_id);
        
        // Insert metadata into database
        let file = query_as!(
            FileMetadata,
            r#"
            INSERT INTO file_metadata (id, name, path, content_type, size, is_public, user_id, created_at, updated_at, md5_hash)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            RETURNING id, name, path, content_type, size, is_public, user_id, created_at, updated_at, md5_hash
            "#,
            file_id,
            name,
            relative_path,
            content_type,
            size,
            is_public,
            user_id,
            Utc::now(),
            Utc::now(),
            Some(hash)
        )
        .fetch_one(&self.db)
        .await?;
        
        // Update cache
        self.file_cache.insert(file.id.clone(), file.clone());
        
        Ok(file)
    }
    
    // Store file from base64 encoded content
    pub async fn store_file_from_base64(
        &self,
        name: &str,
        content_type: &str,
        base64_content: &str,
        is_public: bool,
        user_id: Option<String>
    ) -> Result<FileMetadata, Box<dyn std::error::Error + Send + Sync>> {
        // Decode base64 content
        let content = BASE64.decode(base64_content)?;
        let bytes = Bytes::from(content);
        
        self.store_file(name, content_type, bytes, is_public, user_id).await
    }
    
    // Get file metadata by ID
    pub async fn get_file(
        &self,
        file_id: &str
    ) -> Result<FileMetadata, Box<dyn std::error::Error + Send + Sync>> {
        // Check cache first
        if let Some(file) = self.file_cache.get(file_id) {
            return Ok(file);
        }
        
        // Query database if not in cache
        let file = query_as!(
            FileMetadata,
            r#"
            SELECT id, name, path, content_type, size, is_public, user_id, created_at, updated_at, md5_hash
            FROM file_metadata
            WHERE id = $1
            "#,
            file_id
        )
        .fetch_one(&self.db)
        .await?;
        
        // Update cache
        self.file_cache.insert(file.id.clone(), file.clone());
        
        Ok(file)
    }
    
    // Get file content by ID
    pub async fn get_file_content(
        &self,
        file_id: &str
    ) -> Result<Bytes, Box<dyn std::error::Error + Send + Sync>> {
        // Check cache first
        if let Some(content) = self.content_cache.get(file_id) {
            return Ok(content);
        }
        
        // Get file metadata
        let file = self.get_file(file_id).await?;
        let file_path = self.get_absolute_path(&file.path);
        
        // Read file content
        let content = tokio_fs::read(&file_path).await?;
        let bytes = Bytes::from(content);
        
        // Update cache
        self.content_cache.insert(file_id.to_string(), bytes.clone());
        
        Ok(bytes)
    }
    
    // Delete file
    pub async fn delete_file(
        &self,
        file_id: &str
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Get file metadata
        let file = self.get_file(file_id).await?;
        let file_path = self.get_absolute_path(&file.path);
        
        // Delete file from disk
        tokio_fs::remove_file(&file_path).await?;
        
        // Delete metadata from database
        query!(
            r#"
            DELETE FROM file_metadata
            WHERE id = $1
            "#,
            file_id
        )
        .execute(&self.db)
        .await?;
        
        // Delete all attachments for this file
        query!(
            r#"
            DELETE FROM file_attachments
            WHERE file_id = $1
            "#,
            file_id
        )
        .execute(&self.db)
        .await?;
        
        // Remove from caches
        self.file_cache.invalidate(file_id);
        self.content_cache.invalidate(file_id);
        
        Ok(())
    }
    
    // Update file visibility
    pub async fn update_visibility(
        &self,
        file_id: &str,
        is_public: bool
    ) -> Result<FileMetadata, Box<dyn std::error::Error + Send + Sync>> {
        let file = query_as!(
            FileMetadata,
            r#"
            UPDATE file_metadata
            SET is_public = $1, updated_at = $2
            WHERE id = $3
            RETURNING id, name, path, content_type, size, is_public, user_id, created_at, updated_at, md5_hash
            "#,
            is_public,
            Utc::now(),
            file_id
        )
        .fetch_one(&self.db)
        .await?;
        
        // Update cache
        self.file_cache.insert(file.id.clone(), file.clone());
        
        Ok(file)
    }
    
    // Attach file to an entity
    pub async fn attach_file(
        &self,
        file_id: &str,
        entity_type: &str,
        entity_id: &str
    ) -> Result<FileAttachment, Box<dyn std::error::Error + Send + Sync>> {
        let attachment_id = Uuid::new_v4().to_string();
        let now = Utc::now();
        
        let attachment = query_as!(
            FileAttachment,
            r#"
            INSERT INTO file_attachments (id, file_id, entity_type, entity_id, created_at)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (file_id, entity_type, entity_id) DO NOTHING
            RETURNING id, file_id, entity_type, entity_id, created_at
            "#,
            attachment_id,
            file_id,
            entity_type,
            entity_id,
            now
        )
        .fetch_one(&self.db)
        .await?;
        
        Ok(attachment)
    }
    
    // Detach file from an entity
    pub async fn detach_file(
        &self,
        file_id: &str,
        entity_type: &str,
        entity_id: &str
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        query!(
            r#"
            DELETE FROM file_attachments
            WHERE file_id = $1 AND entity_type = $2 AND entity_id = $3
            "#,
            file_id,
            entity_type,
            entity_id
        )
        .execute(&self.db)
        .await?;
        
        Ok(())
    }
    
    // List files with pagination and filters
    pub async fn list_files(
        &self,
        params: &FileListParams
    ) -> Result<Vec<FileMetadata>, Box<dyn std::error::Error + Send + Sync>> {
        let page = params.page.unwrap_or(1);
        let limit = params.limit.unwrap_or(20);
        let offset = (page - 1) * limit;
        
        // Build dynamic query based on filters
        let mut query_string = String::from(
            r#"
            SELECT f.id, f.name, f.path, f.content_type, f.size, f.is_public, f.user_id, f.created_at, f.updated_at, f.md5_hash
            FROM file_metadata f
            "#
        );
        
        let mut params_vec: Vec<String> = Vec::new();
        let mut param_count = 1;
        
        // Add entity filters if provided
        if params.entity_type.is_some() || params.entity_id.is_some() {
            query_string.push_str("JOIN file_attachments a ON f.id = a.file_id ");
            
            if let Some(entity_type) = &params.entity_type {
                query_string.push_str(&format!("AND a.entity_type = ${} ", param_count));
                params_vec.push(entity_type.clone());
                param_count += 1;
            }
            
            if let Some(entity_id) = &params.entity_id {
                query_string.push_str(&format!("AND a.entity_id = ${} ", param_count));
                params_vec.push(entity_id.clone());
                param_count += 1;
            }
        }
        
        // Add search filter if provided
        if let Some(search) = &params.search {
            if param_count == 1 {
                query_string.push_str("WHERE ");
            } else {
                query_string.push_str("AND ");
            }
            
            query_string.push_str(&format!("(f.name ILIKE ${} OR f.content_type ILIKE ${}) ", 
                param_count, param_count));
            params_vec.push(format!("%{}%", search));
            param_count += 1;
        }
        
        // Add pagination
        query_string.push_str(&format!("ORDER BY f.created_at DESC LIMIT ${} OFFSET ${}", 
            param_count, param_count + 1));
        params_vec.push(limit.to_string());
        params_vec.push(offset.to_string());
        
        // Execute the dynamic query
        let mut query = sqlx::query_as::<_, FileMetadata>(&query_string);
        
        // Add parameters to the query
        for param in params_vec {
            query = query.bind(param);
        }
        
        let files = query.fetch_all(&self.db).await?;
        
        Ok(files)
    }
    
    // Get files for a specific entity
    pub async fn get_files_for_entity(
        &self,
        entity_type: &str,
        entity_id: &str
    ) -> Result<Vec<FileMetadata>, Box<dyn std::error::Error + Send + Sync>> {
        let files = query_as!(
            FileMetadata,
            r#"
            SELECT f.id, f.name, f.path, f.content_type, f.size, f.is_public, f.user_id, f.created_at, f.updated_at, f.md5_hash
            FROM file_metadata f
            JOIN file_attachments a ON f.id = a.file_id
            WHERE a.entity_type = $1 AND a.entity_id = $2
            ORDER BY f.created_at DESC
            "#,
            entity_type,
            entity_id
        )
        .fetch_all(&self.db)
        .await?;
        
        Ok(files)
    }
    
    // Generate a file path from ID
    fn generate_file_path(&self, file_id: &str) -> PathBuf {
        // Create a directory structure based on the first characters of the ID
        // This prevents having too many files in a single directory
        let mut path = self.storage_path.clone();
        if file_id.len() >= 4 {
            path.push(&file_id[0..2]);
            path.push(&file_id[2..4]);
        } else {
            path.push("misc");
        }
        path.push(file_id);
        path
    }
    
    // Get relative path from ID
    fn get_relative_path(&self, file_id: &str) -> String {
        let absolute_path = self.generate_file_path(file_id);
        let relative_path = absolute_path.strip_prefix(&self.storage_path)
            .unwrap_or(Path::new(file_id))
            .to_string_lossy()
            .to_string();
        
        // Convert backslashes to forward slashes for cross-platform compatibility
        relative_path.replace('\\', "/")
    }
    
    // Get absolute path from relative path
    fn get_absolute_path(&self, relative_path: &str) -> PathBuf {
        self.storage_path.join(relative_path)
    }
    
    // Bulk operations
    
    // Store multiple files at once
    pub async fn store_files_batch(
        &self,
        files: Vec<(String, String, Bytes, bool, Option<String>)>, // (name, content_type, bytes, is_public, user_id)
    ) -> Result<Vec<FileMetadata>, Box<dyn std::error::Error + Send + Sync>> {
        let mut results = Vec::with_capacity(files.len());
        
        // Process files concurrently with a limit on parallelism
        use futures::stream::{self, StreamExt};
        
        let results = stream::iter(files)
            .map(|(name, content_type, bytes, is_public, user_id)| {
                let this = self.clone();
                async move {
                    this.store_file(&name, &content_type, bytes, is_public, user_id).await
                }
            })
            .buffer_unordered(4) // Process up to 4 files concurrently
            .collect::<Vec<_>>()
            .await;
            
        // Gather results and handle errors
        let mut successful_files = Vec::new();
        let mut errors = Vec::new();
        
        for (i, result) in results.into_iter().enumerate() {
            match result {
                Ok(file) => successful_files.push(file),
                Err(e) => errors.push(format!("Error storing file {}: {}", i, e)),
            }
        }
        
        if !errors.is_empty() {
            // Log errors but continue with successful files
            log::error!("Errors during batch file upload: {}", errors.join("; "));
        }
        
        Ok(successful_files)
    }
    
    // Attach multiple files to an entity at once
    pub async fn attach_files_batch(
        &self,
        file_ids: Vec<String>,
        entity_type: &str,
        entity_id: &str
    ) -> Result<Vec<FileAttachment>, Box<dyn std::error::Error + Send + Sync>> {
        let mut attachments = Vec::with_capacity(file_ids.len());
        
        // Use a transaction to ensure all attachments are created or none
        let mut tx = self.db.begin().await?;
        
        for file_id in file_ids {
            let attachment_id = Uuid::new_v4().to_string();
            let now = Utc::now();
            
            let attachment = query_as!(
                FileAttachment,
                r#"
                INSERT INTO file_attachments (id, file_id, entity_type, entity_id, created_at)
                VALUES ($1, $2, $3, $4, $5)
                ON CONFLICT (file_id, entity_type, entity_id) DO NOTHING
                RETURNING id, file_id, entity_type, entity_id, created_at
                "#,
                attachment_id,
                file_id,
                entity_type,
                entity_id,
                now
            )
            .fetch_one(&mut *tx)
            .await?;
            
            attachments.push(attachment);
        }
        
        // Commit transaction
        tx.commit().await?;
        
        Ok(attachments)
    }
    
    // Delete multiple files at once
    pub async fn delete_files_batch(
        &self,
        file_ids: Vec<String>
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Use a transaction to ensure atomicity
        let mut tx = self.db.begin().await?;
        
        for file_id in &file_ids {
            // Get file metadata
            let file = query_as!(
                FileMetadata,
                r#"
                SELECT id, name, path, content_type, size, is_public, user_id, created_at, updated_at, md5_hash
                FROM file_metadata
                WHERE id = $1
                "#,
                file_id
            )
            .fetch_optional(&mut *tx)
            .await?;
            
            if let Some(file) = file {
                // Delete from database
                query!(
                    r#"
                    DELETE FROM file_metadata
                    WHERE id = $1
                    "#,
                    file_id
                )
                .execute(&mut *tx)
                .await?;
                
                // Remove from caches
                self.file_cache.invalidate(file_id);
                self.content_cache.invalidate(file_id);
                
                // Schedule file deletion (will be done after transaction commits)
                let file_path = self.get_absolute_path(&file.path);
                tokio::spawn(async move {
                    if let Err(e) = tokio_fs::remove_file(&file_path).await {
                        log::error!("Failed to delete file {}: {}", file_path.display(), e);
                    }
                });
            }
        }
        
        // Commit transaction
        tx.commit().await?;
        
        Ok(())
    }
}
