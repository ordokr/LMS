use std::path::{Path, PathBuf};
use std::fs;
use std::io;
use std::time::{SystemTime, UNIX_EPOCH};
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use serde::{Serialize, Deserialize};
use anyhow::{Result, Context, anyhow};
use tokio::fs as tokio_fs;
use chrono::{DateTime, Utc};

use crate::utils::logger::create_logger;

/// Status of a feedback entry
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum FeedbackStatus {
    New,
    InProgress,
    Reviewed,
    Closed,
}

impl Default for FeedbackStatus {
    fn default() -> Self {
        Self::New
    }
}

/// Feedback data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Feedback {
    pub id: String,
    pub user_id: String,
    pub category: String,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rating: Option<u8>,
    pub timestamp: String,
    #[serde(default)]
    pub status: FeedbackStatus,
}

/// Feedback collector configuration
#[derive(Debug, Clone)]
pub struct FeedbackCollectorConfig {
    pub storage_path: PathBuf,
}

impl Default for FeedbackCollectorConfig {
    fn default() -> Self {
        Self {
            storage_path: PathBuf::from("./data/feedback"),
        }
    }
}

/// Feedback collector for storing and retrieving user feedback
pub struct FeedbackCollector {
    config: FeedbackCollectorConfig,
    logger: slog::Logger,
    categories: Vec<String>,
}

impl FeedbackCollector {
    /// Create a new feedback collector
    pub fn new(config: Option<FeedbackCollectorConfig>) -> Self {
        let config = config.unwrap_or_default();
        let logger = create_logger("feedback-collector");
        let categories = vec![
            "ui".to_string(),
            "performance".to_string(),
            "features".to_string(),
            "bugs".to_string(),
            "general".to_string(),
        ];
        
        let collector = Self {
            config,
            logger,
            categories,
        };
        
        // Initialize storage (non-blocking)
        let collector_clone = collector.clone();
        tokio::spawn(async move {
            if let Err(e) = collector_clone.initialize_storage().await {
                slog::error!(collector_clone.logger, "Failed to initialize feedback storage: {}", e);
            }
        });
        
        collector
    }
    
    /// Clone functionality for the collector
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            logger: self.logger.clone(),
            categories: self.categories.clone(),
        }
    }
    
    /// Initialize the feedback storage directory
    pub async fn initialize_storage(&self) -> Result<()> {
        tokio_fs::create_dir_all(&self.config.storage_path).await
            .with_context(|| format!("Failed to create feedback directory at {:?}", self.config.storage_path))?;
        
        slog::info!(self.logger, "Feedback storage directory created at {:?}", self.config.storage_path);
        
        // Create sample feedback data if directory is empty
        self.create_sample_feedback().await?;
        
        Ok(())
    }
    
    /// Create sample feedback entries for testing
    async fn create_sample_feedback(&self) -> Result<()> {
        // Check if the directory is empty
        let entries = tokio_fs::read_dir(&self.config.storage_path).await
            .with_context(|| format!("Failed to read feedback directory at {:?}", self.config.storage_path))?;
        
        let mut count = 0;
        let mut entry_result = entries.next_entry().await?;
        
        while entry_result.is_some() {
            count += 1;
            entry_result = entries.next_entry().await?;
            
            // If we found at least one file, don't create samples
            if count > 0 {
                return Ok(());
            }
        }
        
        // Create sample feedback if we didn't find any files
        slog::info!(self.logger, "Creating sample feedback data...");
        
        let sample_feedback = vec![
            Feedback {
                id: "feedback_20250404T120000_abc12".to_string(),
                user_id: "user123".to_string(),
                category: "ui".to_string(),
                content: "The discussion integration has greatly improved workflow for students.".to_string(),
                rating: Some(5),
                timestamp: "2025-04-04T12:00:00Z".to_string(),
                status: FeedbackStatus::Reviewed,
            },
            Feedback {
                id: "feedback_20250404T130000_def34".to_string(),
                user_id: "user456".to_string(),
                category: "performance".to_string(),
                content: "Sometimes the forum posts take too long to sync with Canvas.".to_string(),
                rating: Some(3),
                timestamp: "2025-04-04T13:00:00Z".to_string(),
                status: FeedbackStatus::New,
            },
            Feedback {
                id: "feedback_20250404T140000_ghi56".to_string(),
                user_id: "user789".to_string(),
                category: "features".to_string(),
                content: "Would love to see better integration with assignment submissions.".to_string(),
                rating: Some(4),
                timestamp: "2025-04-04T14:00:00Z".to_string(),
                status: FeedbackStatus::InProgress,
            },
        ];
        
        for feedback in sample_feedback {
            let json = serde_json::to_string_pretty(&feedback)?;
            let file_path = self.config.storage_path.join(format!("{}.json", feedback.id));
            tokio_fs::write(&file_path, json).await
                .with_context(|| format!("Failed to write sample feedback to {:?}", file_path))?;
        }
        
        slog::info!(self.logger, "Created {} sample feedback items", sample_feedback.len());
        
        Ok(())
    }
    
    /// Store user feedback
    pub async fn store_feedback(&self, feedback_input: FeedbackInput) -> Result<Feedback> {
        // Validate input
        if feedback_input.user_id.is_empty() {
            return Err(anyhow!("User ID is required"));
        }
        
        if feedback_input.content.is_empty() {
            return Err(anyhow!("Feedback content is required"));
        }
        
        if !self.categories.contains(&feedback_input.category) {
            return Err(anyhow!("Category must be one of: {}", self.categories.join(", ")));
        }
        
        // Create a timestamp
        let now = Utc::now();
        let timestamp = now.to_rfc3339();
        
        // Generate a unique ID
        let random_suffix: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(5)
            .map(char::from)
            .collect();
        
        let id_timestamp = timestamp.replace([':', '.', '+', '-'], "");
        let id = format!("feedback_{}_{}",
            &id_timestamp[0..min(id_timestamp.len(), 15)],
            &random_suffix.to_lowercase()
        );
        
        // Create feedback data
        let feedback = Feedback {
            id,
            user_id: feedback_input.user_id,
            category: feedback_input.category,
            content: feedback_input.content,
            rating: feedback_input.rating,
            timestamp,
            status: FeedbackStatus::New,
        };
        
        // Ensure directory exists
        tokio_fs::create_dir_all(&self.config.storage_path).await
            .with_context(|| format!("Failed to create feedback directory at {:?}", self.config.storage_path))?;
        
        // Write feedback to file
        let json = serde_json::to_string_pretty(&feedback)?;
        let file_path = self.config.storage_path.join(format!("{}.json", feedback.id));
        tokio_fs::write(&file_path, json).await
            .with_context(|| format!("Failed to write feedback to {:?}", file_path))?;
        
        slog::info!(self.logger, "Stored feedback {}", feedback.id; 
            "category" => &feedback.category,
            "user_id" => &feedback.user_id
        );
        
        Ok(feedback)
    }
    
    /// Get all feedback entries
    pub async fn get_all_feedback(&self) -> Result<Vec<Feedback>> {
        let mut feedback_entries = Vec::new();
        
        let mut entries = tokio_fs::read_dir(&self.config.storage_path).await
            .with_context(|| format!("Failed to read feedback directory at {:?}", self.config.storage_path))?;
        
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            
            if path.extension().map_or(false, |ext| ext == "json") {
                match tokio_fs::read_to_string(&path).await {
                    Ok(content) => {
                        match serde_json::from_str::<Feedback>(&content) {
                            Ok(feedback) => feedback_entries.push(feedback),
                            Err(e) => {
                                slog::warn!(self.logger, "Failed to parse feedback file {:?}: {}", path, e);
                            }
                        }
                    },
                    Err(e) => {
                        slog::warn!(self.logger, "Failed to read feedback file {:?}: {}", path, e);
                    }
                }
            }
        }
        
        // Sort by timestamp (newest first)
        feedback_entries.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        
        Ok(feedback_entries)
    }
    
    /// Get feedback by ID
    pub async fn get_feedback_by_id(&self, id: &str) -> Result<Feedback> {
        let file_path = self.config.storage_path.join(format!("{}.json", id));
        
        let content = tokio_fs::read_to_string(&file_path).await
            .with_context(|| format!("Failed to read feedback file {:?}", file_path))?;
            
        let feedback = serde_json::from_str::<Feedback>(&content)
            .with_context(|| format!("Failed to parse feedback file {:?}", file_path))?;
            
        Ok(feedback)
    }
    
    /// Update feedback status
    pub async fn update_feedback_status(&self, id: &str, status: FeedbackStatus) -> Result<Feedback> {
        let mut feedback = self.get_feedback_by_id(id).await?;
        
        feedback.status = status;
        
        let json = serde_json::to_string_pretty(&feedback)?;
        let file_path = self.config.storage_path.join(format!("{}.json", feedback.id));
        tokio_fs::write(&file_path, json).await
            .with_context(|| format!("Failed to write feedback to {:?}", file_path))?;
            
        slog::info!(self.logger, "Updated feedback {} status to {:?}", id, status);
        
        Ok(feedback)
    }
    
    /// Get available feedback categories
    pub fn get_categories(&self) -> &[String] {
        &self.categories
    }
}

/// Input for creating a new feedback entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedbackInput {
    pub user_id: String,
    pub category: String,
    pub content: String,
    pub rating: Option<u8>,
}

/// Create a default feedback collector
pub fn create_feedback_collector() -> FeedbackCollector {
    FeedbackCollector::new(None)
}

/// Utility function to get minimum of two usize values
fn min(a: usize, b: usize) -> usize {
    if a < b { a } else { b }
}
