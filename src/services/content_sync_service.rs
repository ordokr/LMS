use anyhow::{Result, anyhow};
use async_trait::async_trait;
use chrono::Utc;
use log::{info, error, warn};
use uuid::Uuid;

use crate::models::forum::mapping::TopicMapping;
use crate::repository::topic_mapping_repository::TopicMappingRepository;
use crate::services::sync_service::SyncService;
use crate::repository::sync_queue_repository::SyncQueueRepository;
use crate::models::sync_queue::SyncQueueItem;

// Define interfaces for platform APIs
#[async_trait]
pub trait CanvasClient {
    async fn get_topic_content(&self, topic_id: &str) -> Result<String>;
    async fn update_topic_content(&self, topic_id: &str, content: &str) -> Result<()>;
}

#[async_trait]
pub trait DiscourseClient {
    async fn get_topic_content(&self, topic_id: &str) -> Result<String>;
    async fn update_topic_content(&self, topic_id: &str, content: &str) -> Result<()>;
}

pub struct ContentSyncService<C: CanvasClient, D: DiscourseClient> {
    sync_service: SyncService,
    topic_mapping_repo: TopicMappingRepository,
    canvas_client: C,
    discourse_client: D,
    sync_queue_repo: SyncQueueRepository,
}

impl<C: CanvasClient, D: DiscourseClient> ContentSyncService<C, D> {
    pub fn new(
        sync_service: SyncService,
        topic_mapping_repo: TopicMappingRepository,
        canvas_client: C,
        discourse_client: D,
        sync_queue_repo: SyncQueueRepository,
    ) -> Self {
        Self {
            sync_service,
            topic_mapping_repo,
            canvas_client,
            discourse_client,
            sync_queue_repo,
        }
    }
    
    pub async fn sync_all_pending_topics(&self) -> Result<usize> {
        // TODO: Get all topic mappings and check which need syncing
        // This is a placeholder implementation
        Ok(0)
    }
    
    pub async fn sync_topic(&self, topic_mapping_id: Uuid) -> Result<()> {
        // Get the mapping
        let mapping = self.topic_mapping_repo.get_topic_mapping_by_id(topic_mapping_id).await?;
        
        if !mapping.sync_enabled {
            return Err(anyhow!("Sync is disabled for topic mapping {}", topic_mapping_id));
        }
        
        // Check sync direction
        let sync_direction = self.sync_service.needs_sync(topic_mapping_id).await?;
        
        match sync_direction {
            Some(direction) if direction == "canvas_to_discourse" => {
                self.sync_from_canvas_to_discourse(&mapping).await?;
            },
            Some(direction) if direction == "discourse_to_canvas" => {
                self.sync_from_discourse_to_canvas(&mapping).await?;
            },
            _ => {
                info!("No sync needed for topic mapping {}", topic_mapping_id);
                return Ok(());
            }
        }
        
        // Update sync time
        self.topic_mapping_repo.update_topic_sync_time(topic_mapping_id).await?;
        
        Ok(())
    }
    
    async fn sync_from_canvas_to_discourse(&self, mapping: &TopicMapping) -> Result<()> {
        info!("Syncing from Canvas to Discourse for topic mapping {}", mapping.id);
        
        // Get content from Canvas
        let content = self.canvas_client.get_topic_content(&mapping.canvas_topic_id).await?;
        
        // Update content in Discourse
        self.discourse_client.update_topic_content(&mapping.discourse_topic_id, &content).await?;
        
        info!("Successfully synced from Canvas to Discourse for topic mapping {}", mapping.id);
        Ok(())
    }
    
    async fn sync_from_discourse_to_canvas(&self, mapping: &TopicMapping) -> Result<()> {
        info!("Syncing from Discourse to Canvas for topic mapping {}", mapping.id);
        
        // Get content from Discourse
        let content = self.discourse_client.get_topic_content(&mapping.discourse_topic_id).await?;
        
        // Update content in Canvas
        self.canvas_client.update_topic_content(&mapping.canvas_topic_id, &content).await?;
        
        info!("Successfully synced from Discourse to Canvas for topic mapping {}", mapping.id);
        Ok(())
    }

    pub async fn queue_sync(&self, topic_mapping_id: Uuid, sync_direction: &str) -> Result<SyncQueueItem> {
        // Default to 3 attempts
        self.sync_queue_repo.enqueue(topic_mapping_id, sync_direction, 3).await
    }
    
    pub async fn process_queued_syncs(&self, batch_size: i64) -> Result<(u32, u32)> {
        let mut success_count = 0u32;
        let mut failure_count = 0u32;
        
        // Get pending items
        let items = self.sync_queue_repo.get_pending_items(batch_size).await?;
        
        for item in items {
            // Mark as processing
            let mut item = self.sync_queue_repo.update_status(
                item.id, 
                "processing", 
                None
            ).await?;
            
            // Increment attempt count
            item = self.sync_queue_repo.increment_attempt(item.id).await?;
            
            // Process the sync
            let result = match item.sync_direction.as_str() {
                "canvas_to_discourse" => {
                    let mapping = self.topic_mapping_repo.get_topic_mapping_by_id(item.topic_mapping_id).await?;
                    self.sync_from_canvas_to_discourse(&mapping).await
                },
                "discourse_to_canvas" => {
                    let mapping = self.topic_mapping_repo.get_topic_mapping_by_id(item.topic_mapping_id).await?;
                    self.sync_from_discourse_to_canvas(&mapping).await
                },
                _ => Err(anyhow!("Invalid sync direction: {}", item.sync_direction)),
            };
            
            match result {
                Ok(_) => {
                    // Mark as completed
                    self.sync_queue_repo.update_status(item.id, "completed", None).await?;
                    success_count += 1;
                    
                    // Update topic mapping's last sync time
                    self.topic_mapping_repo.update_topic_sync_time(item.topic_mapping_id).await?;
                },
                Err(e) => {
                    // Mark as failed or pending for retry
                    self.sync_queue_repo.update_status(
                        item.id, 
                        if item.attempt_count >= item.max_attempts { "failed" } else { "pending" }, 
                        Some(&e.to_string())
                    ).await?;
                    
                    failure_count += 1;
                    error!("Sync failed for queue item {}: {}", item.id, e);
                }
            }
        }
        
        Ok((success_count, failure_count))
    }
    
    // Add maintenance method to clean up old completed items
    pub async fn cleanup_queue(&self, older_than_hours: i64) -> Result<u64> {
        self.sync_queue_repo.clear_completed(older_than_hours).await
    }
}