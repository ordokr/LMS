use std::path::{Path, PathBuf};
use std::io::{self, Read, Write};
use std::fs::{self, File};
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use tracing::{info, error, warn};
use base64::{Engine, engine::general_purpose::STANDARD as BASE64};
use sqlx::{Pool, Sqlite};
use mime_guess::from_path;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    pub id: String,
    pub filename: String,
    pub content_type: String,
    pub size: usize,
    pub upload_date: DateTime<Utc>,
    pub entity_type: Option<String>,
    pub entity_id: Option<String>,
    pub user_id: Option<String>,
    pub metadata: Option<HashMap<String, String>>,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileUploadResult {
    pub file_info: FileInfo,
    pub success: bool,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageConfig {
    pub base_path: PathBuf,
    pub max_file_size: usize,
    pub allowed_extensions: Vec<String>,
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            base_path: PathBuf::from("./storage/files"),
            max_file_size: 1024 * 1024 * 50, // 50MB
            allowed_extensions: vec![
                "jpg".to_string(), "jpeg".to_string(), "png".to_string(), 
                "gif".to_string(), "pdf".to_string(), "doc".to_string(), 
                "docx".to_string(), "xls".to_string(), "xlsx".to_string(),
                "ppt".to_string(), "pptx".to_string(), "txt".to_string(),
                "zip".to_string(), "rar".to_string(), "mp3".to_string(),
                "wav".to_string(), "mp4".to_string(), "mov".to_string(),
            ],
        }
    }
}

/// The FileStorageService handles all file operations for the application
#[derive(Clone)]
pub struct FileStorageService {
    config: StorageConfig,
    db_pool: Pool<Sqlite>,
    base_url: String,
}

impl FileStorageService {
    pub fn new(db_pool: Pool<Sqlite>, base_url: String, config: Option<StorageConfig>) -> Self {
        let config = config.unwrap_or_default();
        
        // Ensure storage directory exists
        if let Err(e) = fs::create_dir_all(&config.base_path) {
            error!("Failed to create storage directory: {}", e);
        }
        
        Self {
            config,
            db_pool,
            base_url,
        }
    }
    
    /// Store file data from bytes
    pub async fn store_file(&self, 
        filename: String, 
        content_type: Option<String>, 
        data: Vec<u8>, 
        entity_type: Option<String>,
        entity_id: Option<String>,
        user_id: Option<String>,
        metadata: Option<HashMap<String, String>>
    ) -> Result<FileInfo, String> {
        // Validate file size
        if data.len() > self.config.max_file_size {
            return Err(format!("File size exceeds maximum allowed size of {} bytes", self.config.max_file_size));
        }
        
        // Generate a unique ID for the file
        let file_id = Uuid::new_v4().to_string();
        
        // Determine content type if not provided
        let content_type = content_type.unwrap_or_else(|| {
            from_path(&filename)
                .first_or_octet_stream()
                .to_string()
        });
        
        // Create the directory structure
        let storage_path = self.get_storage_path(&file_id);
        if let Some(parent) = storage_path.parent() {
            if let Err(e) = fs::create_dir_all(parent) {
                return Err(format!("Failed to create directory: {}", e));
            }
        }
        
        // Write the file to disk
        let mut file = match File::create(&storage_path) {
            Ok(file) => file,
            Err(e) => return Err(format!("Failed to create file: {}", e)),
        };
        
        if let Err(e) = file.write_all(&data) {
            return Err(format!("Failed to write file: {}", e));
        }
        
        // Create file metadata
        let file_info = FileInfo {
            id: file_id.clone(),
            filename,
            content_type,
            size: data.len(),
            upload_date: Utc::now(),
            entity_type,
            entity_id,
            user_id,
            metadata,
            url: format!("{}/api/attachments/{}", self.base_url, file_id),
        };
        
        // Store file metadata in database
        if let Err(e) = self.store_file_metadata(&file_info).await {
            // Clean up the file if metadata storage fails
            let _ = fs::remove_file(&storage_path);
            return Err(format!("Failed to store file metadata: {}", e));
        }
        
        Ok(file_info)
    }
    
    /// Store a base64 encoded file
    pub async fn store_base64_file(&self,
        filename: String,
        content_type: Option<String>,
        base64_data: String,
        entity_type: Option<String>,
        entity_id: Option<String>,
        user_id: Option<String>,
        metadata: Option<HashMap<String, String>>
    ) -> Result<FileInfo, String> {
        // Remove data URL prefix if present
        let base64_data = if base64_data.starts_with("data:") {
            let parts: Vec<&str> = base64_data.split(",").collect();
            if parts.len() < 2 {
                return Err("Invalid base64 data URL format".to_string());
            }
            parts[1].to_string()
        } else {
            base64_data
        };
        
        // Decode base64 data
        let data = match BASE64.decode(base64_data) {
            Ok(data) => data,
            Err(e) => return Err(format!("Failed to decode base64 data: {}", e)),
        };
        
        // Store the decoded data
        self.store_file(filename, content_type, data, entity_type, entity_id, user_id, metadata).await
    }
    
    /// Get file info by ID
    pub async fn get_file_info(&self, file_id: &str) -> Result<FileInfo, String> {
        let query = "SELECT * FROM file_attachments WHERE id = ?";
        
        // Query the database for file metadata
        match sqlx::query_as::<_, FileInfoDb>(query)
            .bind(file_id)
            .fetch_optional(&self.db_pool)
            .await {
                Ok(Some(file_info_db)) => {
                    let metadata = if let Some(json) = file_info_db.metadata {
                        match serde_json::from_str::<HashMap<String, String>>(&json) {
                            Ok(map) => Some(map),
                            Err(_) => None,
                        }
                    } else {
                        None
                    };
                    
                    Ok(FileInfo {
                        id: file_info_db.id,
                        filename: file_info_db.filename,
                        content_type: file_info_db.content_type,
                        size: file_info_db.size as usize,
                        upload_date: file_info_db.upload_date,
                        entity_type: file_info_db.entity_type,
                        entity_id: file_info_db.entity_id,
                        user_id: file_info_db.user_id,
                        metadata,
                        url: format!("{}/api/attachments/{}", self.base_url, file_info_db.id),
                    })
                },
                Ok(None) => Err(format!("File with ID {} not found", file_id)),
                Err(e) => Err(format!("Database error: {}", e)),
            }
    }
    
    /// Read file data by ID
    pub async fn read_file(&self, file_id: &str) -> Result<(Vec<u8>, FileInfo), String> {
        // Get file metadata
        let file_info = self.get_file_info(file_id).await?;
        
        // Get file path
        let file_path = self.get_storage_path(file_id);
        
        // Read file data
        let mut file = match File::open(&file_path) {
            Ok(file) => file,
            Err(e) => return Err(format!("Failed to open file: {}", e)),
        };
        
        let mut data = Vec::new();
        if let Err(e) = file.read_to_end(&mut data) {
            return Err(format!("Failed to read file: {}", e));
        }
        
        Ok((data, file_info))
    }
    
    /// Delete file by ID
    pub async fn delete_file(&self, file_id: &str) -> Result<(), String> {
        // Get file path
        let file_path = self.get_storage_path(file_id);
        
        // Delete file from disk
        if let Err(e) = fs::remove_file(&file_path) {
            if e.kind() != io::ErrorKind::NotFound {
                return Err(format!("Failed to delete file: {}", e));
            }
        }
        
        // Delete metadata from database
        let query = "DELETE FROM file_attachments WHERE id = ?";
        
        if let Err(e) = sqlx::query(query)
            .bind(file_id)
            .execute(&self.db_pool)
            .await {
            return Err(format!("Failed to delete file metadata: {}", e));
        }
        
        Ok(())
    }
    
    /// List files for an entity
    pub async fn list_files_for_entity(&self, 
        entity_type: &str, 
        entity_id: &str
    ) -> Result<Vec<FileInfo>, String> {
        let query = "SELECT * FROM file_attachments WHERE entity_type = ? AND entity_id = ?";
        
        match sqlx::query_as::<_, FileInfoDb>(query)
            .bind(entity_type)
            .bind(entity_id)
            .fetch_all(&self.db_pool)
            .await {
                Ok(files) => {
                    let mut result = Vec::new();
                    for file_info_db in files {
                        let metadata = if let Some(json) = file_info_db.metadata {
                            match serde_json::from_str::<HashMap<String, String>>(&json) {
                                Ok(map) => Some(map),
                                Err(_) => None,
                            }
                        } else {
                            None
                        };
                        
                        result.push(FileInfo {
                            id: file_info_db.id,
                            filename: file_info_db.filename,
                            content_type: file_info_db.content_type,
                            size: file_info_db.size as usize,
                            upload_date: file_info_db.upload_date,
                            entity_type: file_info_db.entity_type,
                            entity_id: file_info_db.entity_id,
                            user_id: file_info_db.user_id,
                            metadata,
                            url: format!("{}/api/attachments/{}", self.base_url, file_info_db.id),
                        });
                    }
                    Ok(result)
                },
                Err(e) => Err(format!("Database error: {}", e)),
            }
    }
    
    /// List files for a user
    pub async fn list_files_for_user(&self, user_id: &str) -> Result<Vec<FileInfo>, String> {
        let query = "SELECT * FROM file_attachments WHERE user_id = ?";
        
        match sqlx::query_as::<_, FileInfoDb>(query)
            .bind(user_id)
            .fetch_all(&self.db_pool)
            .await {
                Ok(files) => {
                    let mut result = Vec::new();
                    for file_info_db in files {
                        let metadata = if let Some(json) = file_info_db.metadata {
                            match serde_json::from_str::<HashMap<String, String>>(&json) {
                                Ok(map) => Some(map),
                                Err(_) => None,
                            }
                        } else {
                            None
                        };
                        
                        result.push(FileInfo {
                            id: file_info_db.id,
                            filename: file_info_db.filename,
                            content_type: file_info_db.content_type,
                            size: file_info_db.size as usize,
                            upload_date: file_info_db.upload_date,
                            entity_type: file_info_db.entity_type,
                            entity_id: file_info_db.entity_id,
                            user_id: file_info_db.user_id,
                            metadata,
                            url: format!("{}/api/attachments/{}", self.base_url, file_info_db.id),
                        });
                    }
                    Ok(result)
                },
                Err(e) => Err(format!("Database error: {}", e)),
            }
    }
    
    // Helper methods
    
    fn get_storage_path(&self, file_id: &str) -> PathBuf {
        // Create a hierarchical structure based on the first characters of the ID
        // This helps avoid having too many files in a single directory
        let id_chars: Vec<char> = file_id.chars().collect();
        let dir1 = if id_chars.len() > 0 { id_chars[0].to_string() } else { "0".to_string() };
        let dir2 = if id_chars.len() > 1 { id_chars[1].to_string() } else { "0".to_string() };
        
        self.config.base_path.join(dir1).join(dir2).join(file_id)
    }
    
    async fn store_file_metadata(&self, file_info: &FileInfo) -> Result<(), sqlx::Error> {
        let metadata_json = if let Some(ref metadata) = file_info.metadata {
            Some(serde_json::to_string(metadata).unwrap_or_default())
        } else {
            None
        };
        
        let query = r#"
            INSERT INTO file_attachments (
                id, filename, content_type, size, upload_date, 
                entity_type, entity_id, user_id, metadata
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#;
        
        sqlx::query(query)
            .bind(&file_info.id)
            .bind(&file_info.filename)
            .bind(&file_info.content_type)
            .bind(file_info.size as i64)
            .bind(file_info.upload_date)
            .bind(&file_info.entity_type)
            .bind(&file_info.entity_id)
            .bind(&file_info.user_id)
            .bind(metadata_json)
            .execute(&self.db_pool)
            .await?;
        
        Ok(())
    }
}

// Database model for FileInfo
#[derive(sqlx::FromRow)]
struct FileInfoDb {
    id: String,
    filename: String,
    content_type: String,
    size: i64,
    upload_date: DateTime<Utc>,
    entity_type: Option<String>,
    entity_id: Option<String>,
    user_id: Option<String>,
    metadata: Option<String>,
}

/// Initialize the file attachments table in the database
pub async fn init_file_attachments_table(pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
    sqlx::query(r#"
        CREATE TABLE IF NOT EXISTS file_attachments (
            id TEXT PRIMARY KEY,
            filename TEXT NOT NULL,
            content_type TEXT NOT NULL,
            size INTEGER NOT NULL,
            upload_date TIMESTAMP NOT NULL,
            entity_type TEXT,
            entity_id TEXT,
            user_id TEXT,
            metadata TEXT,
            FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE SET NULL
        )
    "#)
    .execute(pool)
    .await?;
    
    // Create indexes
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_file_attachments_entity ON file_attachments (entity_type, entity_id)")
        .execute(pool)
        .await?;
    
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_file_attachments_user ON file_attachments (user_id)")
        .execute(pool)
        .await?;
    
    Ok(())
}
