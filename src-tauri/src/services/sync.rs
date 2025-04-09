use std::sync::Arc;
use std::time::Duration;
use tokio::time;
use gloo_net::http::Request;
use serde::{Serialize, Deserialize};
use reqwest::Client;
use tracing::{info, warn, error, instrument};

use crate::core::errors::AppError;
use crate::sync::engine::SyncEngine;
use crate::sync::operations::SyncBatch;
use crate::models::discussion::Discussion;
use crate::models::integration::CourseCategoryMapping;
use crate::models::sync::SyncResult;

// Canvas client trait and data structures
#[async_trait::async_trait]
pub trait CanvasClient {
    async fn get_course(&self, course_id: &str) -> Result<CanvasCourse, String>;
    async fn update_course(&self, course_id: &str, name: &str, description: &str) -> Result<(), String>;
}

#[derive(Debug, Clone)]
pub struct CanvasCourse {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
}

// Discourse client trait and data structures
#[async_trait::async_trait]
pub trait DiscourseClient {
    async fn get_category(&self, category_id: &str) -> Result<DiscourseCategory, String>;
    async fn update_category(&self, category_id: &str, name: &str, description: &str) -> Result<(), String>;
}

#[derive(Debug, Clone)]
pub struct DiscourseCategory {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, thiserror::Error)]
pub enum SyncError {
    #[error("HTTP error: {0}")]
    HttpError(String),
    
    #[error("Discourse API error: {0}")]
    DiscourseError(String),
    
    #[error("Canvas API error: {0}")]
    CanvasError(String),
    
    #[error("Configuration error: {0}")]
    ConfigError(String),
}

pub struct SyncService {
    engine: Arc<SyncEngine>,
    sync_endpoint: String,
    sync_interval: Duration,
    http_client: Client,
    discourse_url: String,
    discourse_api_key: String,
    discourse_username: String,
    course_category_repo: Arc<dyn crate::db::course_category_repository::CourseCategoryRepository + Send + Sync>,
}

impl SyncService {
    pub fn new(engine: Arc<SyncEngine>, sync_endpoint: String, sync_interval_secs: u64, discourse_url: String, discourse_api_key: String, discourse_username: String, course_category_repo: Arc<dyn crate::db::course_category_repository::CourseCategoryRepository + Send + Sync>) -> Self {
        Self {
            engine,
            sync_endpoint,
            sync_interval: Duration::from_secs(sync_interval_secs),
            http_client: Client::new(),
            discourse_url,
            discourse_api_key,
            discourse_username,
            course_category_repo,
        }
    }
    
    // Start background sync process
    pub async fn start_background_sync(&self, user_id: i64) -> Result<(), AppError> {
        let engine = self.engine.clone();
        let sync_endpoint = self.sync_endpoint.clone();
        let sync_interval = self.sync_interval;
        
        // Initialize the sync engine
        engine.initialize().await?;
        
        // Start background task
        tokio::spawn(async move {
            let mut interval = time::interval(sync_interval);
            
            loop {
                interval.tick().await;
                
                // Check if there are operations to sync
                match engine.create_sync_batch(user_id, 100).await {
                    Ok(Some(batch)) => {
                        // Try to sync with server
                        if let Err(e) = Self::sync_with_server(&sync_endpoint, &batch).await {
                            tracing::error!("Sync error: {}", e);
                            continue;
                        }
                        
                        // Mark operations as synced
                        let op_ids: Vec<String> = batch.operations.iter()
                            .map(|op| op.id.clone())
                            .collect();
                            
                        if let Err(e) = engine.mark_as_synced(&op_ids).await {
                            tracing::error!("Failed to mark operations as synced: {}", e);
                        }
                    },
                    Ok(None) => {
                        // No operations to sync
                        tracing::debug!("No operations to sync");
                    },
                    Err(e) => {
                        tracing::error!("Failed to create sync batch: {}", e);
                    }
                }
            }
        });
        
        Ok(())
    }
    
    // Sync batch with server
    async fn sync_with_server(endpoint: &str, batch: &SyncBatch) -> Result<(), AppError> {
        // In a real implementation, this would use proper HTTP client with error handling
        // For now, we'll just simulate it
        
        tracing::info!("Syncing batch with {} operations to server", batch.operations.len());
        
        // Simulate server sync
        // In a real implementation, you would POST the batch to the server
        // and process the response
        
        // Example of what the actual code might look like:
        /*
        let response = reqwest::Client::new()
            .post(endpoint)
            .json(batch)
            .send()
            .await
            .map_err(|e| AppError::SyncError(format!("Sync request failed: {}", e)))?;
            
        if !response.status().is_success() {
            let error_text = response.text().await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(AppError::SyncError(format!("Sync failed: {}", error_text)));
        }
        
        // Process server response
        let server_batch: SyncBatch = response.json().await
            .map_err(|e| AppError::SyncError(format!("Failed to parse server response: {}", e)))?;
            
        // Apply server operations
        engine.apply_sync_batch(server_batch).await?;
        */
        
        // For now, just simulate success
        Ok(())
    }
    
    #[instrument(skip(self), err)]
    pub async fn find_mapping_for_course(&self, course_id: &str) -> Result<Option<CourseCategoryMapping>, SyncError> {
        match self.course_category_repo.find_by_course(course_id).await {
            Ok(mappings) => {
                if mappings.is_empty() {
                    Ok(None)
                } else {
                    Ok(Some(mappings[0].clone()))
                }
            },
            Err(e) => {
                error!("Failed to find course mapping: {}", e);
                Err(SyncError::ConfigError(e.to_string()))
            }
        }
    }
    
    #[instrument(skip(self), fields(discussion_id = %discussion.id, mapping_id = %mapping.id), err)]
    pub async fn sync_discussion_to_discourse(
        &self, 
        discussion: &Discussion,
        mapping: &CourseCategoryMapping
    ) -> Result<Discussion, SyncError> {
        info!(
            event = "sync_discussion_start", 
            discussion_id = %discussion.id, 
            category_id = %mapping.category_id
        );
        
        let mut updated_discussion = discussion.clone();
        
        // Check if this discussion is already linked to a Discourse topic
        if let Some(topic_id) = &discussion.topic_id {
            // Update existing topic
            self.update_discourse_topic(topic_id, &discussion).await?;
            info!(
                event = "sync_discussion_update", 
                discussion_id = %discussion.id, 
                topic_id = %topic_id
            );
        } else {
            // Create new topic in Discourse
            let topic_id = self.create_discourse_topic(&mapping.category_id, &discussion).await?;
            updated_discussion.topic_id = Some(topic_id.clone());
            info!(
                event = "sync_discussion_create", 
                discussion_id = %discussion.id, 
                topic_id = %topic_id
            );
        }
        
        Ok(updated_discussion)
    }
      // Legacy sync method - deprecated
    #[deprecated(note = "Use the new sync_course_category method with mapping_id parameter instead")]
    #[instrument(skip(self), err)]
    pub async fn sync_course_category_legacy(&self, mapping: &CourseCategoryMapping) -> Result<SyncResult, SyncError> {
        info!(
            event = "sync_course_category_legacy_start", 
            course_id = %mapping.course_id, 
            category_id = %mapping.category_id
        );
        
        // This would synchronize all discussions for a course with a category
        // Implementation depends on specific requirements
        
        // For now, we'll just return a placeholder result
        Ok(SyncResult {
            topics_synced: 0,
            posts_synced: 0,
            assignments_synced: 0,
            errors: vec![],
            completed_at: chrono::Utc::now().to_rfc3339(),
        })
    }
    
    #[instrument(skip(self), err)]
    pub async fn sync_course_category(&self, mapping_id: &str) -> Result<SyncResult, SyncError> {
        // Find the mapping
        let mapping = match self.course_category_repo.find_by_id(mapping_id).await {
            Ok(Some(m)) => m,
            Ok(None) => return Err(SyncError::ConfigError(format!("Mapping not found: {}", mapping_id))),
            Err(e) => return Err(SyncError::ConfigError(e.to_string())),
        };
        
        info!(
            event = "sync_course_category_start", 
            course_id = %mapping.course_id, 
            category_id = %mapping.category_id,
            mapping_id = %mapping_id
        );
        
        let mut result = SyncResult {
            topics_synced: 0,
            posts_synced: 0,
            assignments_synced: 0,
            errors: vec![],
            completed_at: chrono::Utc::now().to_rfc3339(),
        };
        
        // Get Canvas and Discourse clients
        let canvas_client = self.get_canvas_client().await?;
        let discourse_client = self.get_discourse_client().await?;
        
        // Decide sync direction based on mapping configuration
        let direction = self.get_sync_direction(mapping_id).await?;
        
        match direction {
            crate::models::integration::SyncDirection::CanvasToDiscourse => {
                // Sync Canvas course to Discourse category
                if let Err(e) = self.sync_canvas_to_discourse(&mapping, &canvas_client, &discourse_client).await {
                    result.errors.push(e.to_string());
                } else {
                    result.topics_synced += 1;
                }
            },
            crate::models::integration::SyncDirection::DiscourseToCanvas => {
                // Sync Discourse category to Canvas course
                if let Err(e) = self.sync_discourse_to_canvas(&mapping, &canvas_client, &discourse_client).await {
                    result.errors.push(e.to_string());
                } else {
                    result.topics_synced += 1;
                }
            },
            crate::models::integration::SyncDirection::Bidirectional => {
                // Sync both ways
                if let Err(e) = self.sync_canvas_to_discourse(&mapping, &canvas_client, &discourse_client).await {
                    result.errors.push(format!("Canvas to Discourse: {}", e));
                } else {
                    result.topics_synced += 1;
                }
                
                if let Err(e) = self.sync_discourse_to_canvas(&mapping, &canvas_client, &discourse_client).await {
                    result.errors.push(format!("Discourse to Canvas: {}", e));
                } else {
                    result.topics_synced += 1;
                }
            }
        }
        
        // Update last_synced_at timestamp
        self.update_last_synced(mapping_id).await?;
        
        info!(
            event = "sync_course_category_complete", 
            mapping_id = %mapping_id,
            topics_synced = result.topics_synced,
            errors = result.errors.len()
        );
        
        Ok(result)
    }
    
    // Get sync direction from the database
    async fn get_sync_direction(&self, mapping_id: &str) -> Result<crate::models::integration::SyncDirection, SyncError> {
        // Parse UUID
        let uuid = uuid::Uuid::parse_str(mapping_id)
            .map_err(|e| SyncError::ConfigError(format!("Invalid UUID: {}", e)))?;
            
        // Find the mapping
        let mapping = match self.course_category_repo.find_by_id(uuid).await {
            Ok(Some(m)) => m,
            Ok(None) => return Err(SyncError::ConfigError(format!("Mapping not found: {}", mapping_id))),
            Err(e) => return Err(SyncError::ConfigError(format!("DB error: {}", e))),
        };
        
        Ok(mapping.sync_direction)
    }
    
    // Update last_synced_at timestamp
    async fn update_last_synced(&self, mapping_id: &str) -> Result<(), SyncError> {
        // Parse UUID
        let uuid = uuid::Uuid::parse_str(mapping_id)
            .map_err(|e| SyncError::ConfigError(format!("Invalid UUID: {}", e)))?;
            
        // Update the mapping
        let update = crate::models::integration::CourseCategoryUpdate {
            sync_enabled: None,
            sync_direction: None,
            last_synced_at: Some(chrono::Utc::now()),
        };
        
        match self.course_category_repo.update(uuid, update).await {
            Ok(_) => Ok(()),
            Err(e) => Err(SyncError::ConfigError(format!("Failed to update mapping: {}", e))),
        }
    }
    
    // Sync from Canvas to Discourse
    async fn sync_canvas_to_discourse(
        &self,
        mapping: &CourseCategoryMapping,
        canvas_client: &impl CanvasClient,
        discourse_client: &impl DiscourseClient,
    ) -> Result<(), SyncError> {
        info!(
            event = "sync_canvas_to_discourse_start", 
            course_id = %mapping.course_id, 
            category_id = %mapping.category_id
        );
        
        // Get course details from Canvas
        let course = canvas_client.get_course(&mapping.course_id).await
            .map_err(|e| SyncError::CanvasError(format!("Failed to get course: {}", e)))?;
            
        // Update Discourse category with Canvas course data
        discourse_client.update_category(
            &mapping.category_id,
            &course.name,
            &course.description.unwrap_or_default(),
        ).await
        .map_err(|e| SyncError::DiscourseError(format!("Failed to update category: {}", e)))?;
        
        info!(
            event = "sync_canvas_to_discourse_complete", 
            course_name = %course.name
        );
        
        Ok(())
    }
    
    // Sync from Discourse to Canvas
    async fn sync_discourse_to_canvas(
        &self,
        mapping: &CourseCategoryMapping,
        canvas_client: &impl CanvasClient,
        discourse_client: &impl DiscourseClient,
    ) -> Result<(), SyncError> {
        info!(
            event = "sync_discourse_to_canvas_start", 
            course_id = %mapping.course_id, 
            category_id = %mapping.category_id
        );
        
        // Get category details from Discourse
        let category = discourse_client.get_category(&mapping.category_id).await
            .map_err(|e| SyncError::DiscourseError(format!("Failed to get category: {}", e)))?;
            
        // Update Canvas course with Discourse category data
        canvas_client.update_course(
            &mapping.course_id,
            &category.name,
            &category.description.unwrap_or_default(),
        ).await
        .map_err(|e| SyncError::CanvasError(format!("Failed to update course: {}", e)))?;
        
        info!(
            event = "sync_discourse_to_canvas_complete", 
            category_name = %category.name
        );
        
        Ok(())
    }
    
    // Get Canvas client
    async fn get_canvas_client(&self) -> Result<impl CanvasClient, SyncError> {
        // This would normally initialize and return a Canvas API client
        // For now, we'll use a placeholder implementation
        
        struct MockCanvasClient;
        
        impl CanvasClient for MockCanvasClient {
            async fn get_course(&self, course_id: &str) -> Result<CanvasCourse, String> {
                Ok(CanvasCourse {
                    id: course_id.to_string(),
                    name: format!("Canvas Course {}", course_id),
                    description: Some(format!("Description for course {}", course_id)),
                })
            }
            
            async fn update_course(&self, course_id: &str, name: &str, description: &str) -> Result<(), String> {
                // In real implementation, this would make an API call
                Ok(())
            }
        }
        
        Ok(MockCanvasClient)
    }
    
    // Get Discourse client
    async fn get_discourse_client(&self) -> Result<impl DiscourseClient, SyncError> {
        // This would normally initialize and return a Discourse API client
        // For now, we'll use a placeholder implementation
        
        struct MockDiscourseClient;
        
        impl DiscourseClient for MockDiscourseClient {
            async fn get_category(&self, category_id: &str) -> Result<DiscourseCategory, String> {
                Ok(DiscourseCategory {
                    id: category_id.to_string(),
                    name: format!("Discourse Category {}", category_id),
                    description: Some(format!("Description for category {}", category_id)),
                })
            }
            
            async fn update_category(&self, category_id: &str, name: &str, description: &str) -> Result<(), String> {
                // In real implementation, this would make an API call
                Ok(())
            }
        }
        
        Ok(MockDiscourseClient)
    }

    // Helper methods for Discourse API interaction
    
    async fn create_discourse_topic(
        &self, 
        category_id: &str, 
        discussion: &Discussion
    ) -> Result<String, SyncError> {
        #[derive(Serialize)]
        struct CreateTopicRequest {
            title: String,
            raw: String,
            category: String,
            api_key: String,
            api_username: String,
        }
        
        #[derive(Deserialize)]
        struct TopicResponse {
            id: u64,
            topic_id: u64,
        }
        
        let request = CreateTopicRequest {
            title: discussion.title.clone(),
            raw: discussion.content.clone(),
            category: category_id.to_string(),
            api_key: self.discourse_api_key.clone(),
            api_username: self.discourse_username.clone(),
        };
        
        let response = self.http_client.post(format!("{}/posts.json", self.discourse_url))
            .json(&request)
            .send()
            .await
            .map_err(|e| SyncError::HttpError(e.to_string()))?;
            
        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            error!(
                event = "discourse_api_error", 
                status = %status, 
                error = %error_text
            );
            return Err(SyncError::DiscourseError(format!("API error {}: {}", status, error_text)));
        }
        
        let topic_response: TopicResponse = response.json()
            .await
            .map_err(|e| SyncError::DiscourseError(format!("Failed to parse response: {}", e)))?;
            
        Ok(topic_response.topic_id.to_string())
    }
    
    async fn update_discourse_topic(
        &self, 
        topic_id: &str, 
        discussion: &Discussion
    ) -> Result<(), SyncError> {
        #[derive(Serialize)]
        struct UpdateTopicRequest {
            title: String,
            api_key: String,
            api_username: String,
        }
        
        // Update title
        let title_request = UpdateTopicRequest {
            title: discussion.title.clone(),
            api_key: self.discourse_api_key.clone(),
            api_username: self.discourse_username.clone(),
        };
        
        let response = self.http_client.put(format!("{}/t/{}.json", self.discourse_url, topic_id))
            .json(&title_request)
            .send()
            .await
            .map_err(|e| SyncError::HttpError(e.to_string()))?;
            
        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            warn!(
                event = "discourse_api_warning", 
                status = %status, 
                error = %error_text
            );
            // We'll continue even if title update fails
        }
        
        // For the first post content, we'd need additional API calls
        // Updating the first post would require looking up the post ID
        // This is a simplified implementation
        
        Ok(())
    }
}