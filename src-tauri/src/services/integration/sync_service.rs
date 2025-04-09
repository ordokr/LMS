use crate::models::forum::topic::Topic;
use crate::models::forum::post::Post;
use crate::models::forum::mapping::{TopicMapping, PostMapping, SyncStatus};
use crate::db::DB;
use crate::error::Error;
use uuid::Uuid;
use chrono::Utc;
use std::time::Instant;

use super::canvas_integration::CanvasIntegration;
use super::discourse_integration::DiscourseIntegration;
use super::event_listener::{IntegrationEventDispatcher, IntegrationEventListener};
use std::sync::Arc;

pub struct IntegrationSyncService<C, D>
where
    C: CanvasIntegration + Send + Sync,
    D: DiscourseIntegration + Send + Sync,
{
    db: DB,
    canvas: C,
    discourse: D,
    event_dispatcher: Arc<IntegrationEventDispatcher>,
}

impl<C, D> IntegrationSyncService<C, D> 
where
    C: CanvasIntegration + Send + Sync,
    D: DiscourseIntegration + Send + Sync,
{
    pub fn new(db: DB, canvas: C, discourse: D) -> Self {
        let event_dispatcher = Arc::new(IntegrationEventDispatcher::new());
        
        IntegrationSyncService {
            db,
            canvas,
            discourse,
            event_dispatcher,
        }
    }
    
    // Add method to register event listeners
    pub fn register_event_listener(&self, listener: Arc<dyn IntegrationEventListener>) {
        let mut dispatcher = Arc::clone(&self.event_dispatcher);
        let dispatcher_ptr = Arc::get_mut(&mut dispatcher).unwrap();
        dispatcher_ptr.register_listener(listener);
    }
    
    // Sync a topic from Canvas to Discourse
    pub async fn sync_topic_canvas_to_discourse(&self, canvas_topic_id: &str) -> Result<TopicMapping, Error> {
        // Step 1: Sync topic from Canvas to our local model
        let local_topic = self.canvas.sync_topic(canvas_topic_id).await?;
        
        // Step 2: Push topic to Discourse
        let discourse_topic_id = self.discourse.push_topic_to_discourse(&local_topic).await?;
        
        // Step 3: Create or update mapping
        let mapping = match TopicMapping::find_by_canvas_id(&self.db, canvas_topic_id).await {
            Ok(mut existing) => {
                // Update existing mapping
                existing.discourse_topic_id = discourse_topic_id;
                existing.local_topic_id = Some(local_topic.id);
                existing.last_sync_at = Utc::now();
                existing.sync_status = SyncStatus::Synced;
                
                existing.update(&self.db).await?;
                existing
            },
            Err(_) => {
                // Create new mapping
                let mapping = TopicMapping {
                    id: Uuid::new_v4(),
                    canvas_topic_id: canvas_topic_id.to_string(),
                    discourse_topic_id,
                    local_topic_id: Some(local_topic.id),
                    last_sync_at: Utc::now(),
                    sync_status: SyncStatus::Synced,
                };
                
                mapping.create(&self.db).await?;
                mapping
            }
        };
        
        // Step 4: Now sync all posts for this topic
        let posts = Post::find_by_topic_id(&self.db, local_topic.id).await?;
        
        for post in posts {
            if let Some(canvas_id) = &post.canvas_entry_id {
                let _ = self.sync_post_canvas_to_discourse(canvas_id, &mapping).await;
                // Continue even if individual post sync fails
            }
        }
        
        self.event_dispatcher.topic_synced_to_discourse(&local_topic, &mapping).await?;
        
        Ok(mapping)
    }
    
    // Sync a post from Canvas to Discourse
    pub async fn sync_post_canvas_to_discourse(&self, canvas_entry_id: &str, topic_mapping: &TopicMapping) 
        -> Result<PostMapping, Error> {
        // Step 1: Get the post from our local model
        let local_post = Post::find_by_canvas_id(&self.db, canvas_entry_id).await?;
        
        // Step 2: Push post to Discourse
        let discourse_post_id = self.discourse.push_post_to_discourse(&local_post).await?;
        
        // Step 3: Create or update mapping
        let mapping = match PostMapping::find_by_canvas_id(&self.db, canvas_entry_id).await {
            Ok(mut existing) => {
                // Update existing mapping
                existing.discourse_post_id = discourse_post_id;
                existing.local_post_id = Some(local_post.id);
                existing.last_sync_at = Utc::now();
                existing.sync_status = SyncStatus::Synced;
                
                existing.update(&self.db).await?;
                existing
            },
            Err(_) => {
                // Create new mapping
                let mapping = PostMapping {
                    id: Uuid::new_v4(),
                    canvas_entry_id: canvas_entry_id.to_string(),
                    discourse_post_id,
                    topic_mapping_id: topic_mapping.id,
                    local_post_id: Some(local_post.id),
                    last_sync_at: Utc::now(),
                    sync_status: SyncStatus::Synced,
                };
                
                mapping.create(&self.db).await?;
                mapping
            }
        };
        
        self.event_dispatcher.post_synced_to_discourse(&local_post, &mapping).await?;
        
        Ok(mapping)
    }
    
    // Sync a topic from Discourse to Canvas
    pub async fn sync_topic_discourse_to_canvas(&self, discourse_topic_id: i64) -> Result<TopicMapping, Error> {
        // Step 1: Sync topic from Discourse to our local model
        let local_topic = self.discourse.sync_topic(discourse_topic_id).await?;
        
        // Step 2: Push topic to Canvas
        let canvas_topic_id = self.canvas.push_topic_to_canvas(&local_topic).await?;
        
        // Step 3: Create or update mapping
        let mapping = match TopicMapping::find_by_discourse_id(&self.db, discourse_topic_id).await {
            Ok(mut existing) => {
                // Update existing mapping
                existing.canvas_topic_id = canvas_topic_id;
                existing.local_topic_id = Some(local_topic.id);
                existing.last_sync_at = Utc::now();
                existing.sync_status = SyncStatus::Synced;
                
                existing.update(&self.db).await?;
                existing
            },
            Err(_) => {
                // Create new mapping
                let mapping = TopicMapping {
                    id: Uuid::new_v4(),
                    canvas_topic_id,
                    discourse_topic_id,
                    local_topic_id: Some(local_topic.id),
                    last_sync_at: Utc::now(),
                    sync_status: SyncStatus::Synced,
                };
                
                mapping.create(&self.db).await?;
                mapping
            }
        };
        
        // Step 4: Now sync all posts for this topic
        let posts = Post::find_by_topic_id(&self.db, local_topic.id).await?;
        
        for post in posts {
            if let Some(discourse_id) = post.discourse_post_id {
                let _ = self.sync_post_discourse_to_canvas(discourse_id, &mapping).await;
                // Continue even if individual post sync fails
            }
        }
        
        self.event_dispatcher.topic_synced_to_canvas(&local_topic, &mapping).await?;
        
        Ok(mapping)
    }
    
    // Sync a post from Discourse to Canvas
    pub async fn sync_post_discourse_to_canvas(&self, discourse_post_id: i64, topic_mapping: &TopicMapping) 
        -> Result<PostMapping, Error> {
        // Step 1: Get the post from our local model
        let local_post = Post::find_by_discourse_id(&self.db, discourse_post_id).await?;
        
        // Step 2: Push post to Canvas
        let canvas_entry_id = self.canvas.push_post_to_canvas(&local_post).await?;
        
        // Step 3: Create or update mapping
        let mapping = match PostMapping::find_by_discourse_id(&self.db, discourse_post_id).await {
            Ok(mut existing) => {
                // Update existing mapping
                existing.canvas_entry_id = canvas_entry_id;
                existing.local_post_id = Some(local_post.id);
                existing.last_sync_at = Utc::now();
                existing.sync_status = SyncStatus::Synced;
                
                existing.update(&self.db).await?;
                existing
            },
            Err(_) => {
                // Create new mapping
                let mapping = PostMapping {
                    id: Uuid::new_v4(),
                    canvas_entry_id,
                    discourse_post_id,
                    topic_mapping_id: topic_mapping.id,
                    local_post_id: Some(local_post.id),
                    last_sync_at: Utc::now(),
                    sync_status: SyncStatus::Synced,
                };
                
                mapping.create(&self.db).await?;
                mapping
            }
        };
        
        self.event_dispatcher.post_synced_to_canvas(&local_post, &mapping).await?;
        
        Ok(mapping)
    }
    
    // Sync all pending topics (those marked for sync)
    pub async fn sync_all_pending(&self) -> Result<(), Error> {
        // Find all topics marked for sync
        let pending_topics = Topic::find_by_sync_status(&self.db, crate::models::forum::topic::SyncStatus::PendingSync).await?;
        
        for topic in pending_topics {
            match (topic.canvas_topic_id.as_ref(), topic.discourse_topic_id) {
                (Some(canvas_id), None) => {
                    // Topic exists in Canvas but not Discourse - sync to Discourse
                    let _ = self.sync_topic_canvas_to_discourse(canvas_id).await;
                },
                (None, Some(discourse_id)) => {
                    // Topic exists in Discourse but not Canvas - sync to Canvas
                    let _ = self.sync_topic_discourse_to_canvas(discourse_id).await;
                },
                (Some(canvas_id), Some(_)) => {
                    // Topic exists in both - sync from Canvas to Discourse (arbitrary choice)
                    let _ = self.sync_topic_canvas_to_discourse(canvas_id).await;
                },
                (None, None) => {
                    // Topic is local only - nothing to sync
                    // Mark as no longer pending
                    let mut updated_topic = topic.clone();
                    updated_topic.sync_status = crate::models::forum::topic::SyncStatus::LocalOnly;
                    let _ = updated_topic.update(&self.db).await;
                }
            }
        }
        
        Ok(())
    }

    // Add this method to record sync history
    async fn record_sync_history(
        &self,
        sync_type: &str,
        content_id: Option<&str>,
        content_type: Option<&str>,
        success: bool,
        error_message: Option<&str>,
        duration_ms: i64,
    ) -> Result<(), Error> {
        sqlx::query!(
            r#"
            INSERT INTO sync_history (
                sync_type,
                content_id,
                content_type,
                sync_time,
                success,
                error_message,
                duration_ms
            ) VALUES (?, ?, ?, ?, ?, ?, ?)
            "#,
            sync_type,
            content_id,
            content_type,
            Utc::now().to_rfc3339(),
            success,
            error_message,
            duration_ms
        )
        .execute(&self.db)
        .await?;
        
        Ok(())
    }
    
    // Modify your sync methods to record history, for example:
    pub async fn sync_topic(&self, topic_id: &str) -> Result<(), Error> {
        let start_time = Instant::now();
        let sync_type = "topic";
        let content_type = "forum_topic";
        
        let result = self.sync_topic_impl(topic_id).await;
        
        let duration = start_time.elapsed().as_millis() as i64;
        
        // Record sync history
        match &result {
            Ok(_) => {
                let _ = self.record_sync_history(
                    sync_type, 
                    Some(topic_id), 
                    Some(content_type),
                    true,
                    None,
                    duration
                ).await;
            },
            Err(e) => {
                let _ = self.record_sync_history(
                    sync_type, 
                    Some(topic_id), 
                    Some(content_type),
                    false,
                    Some(&e.to_string()),
                    duration
                ).await;
            }
        }
        
        result
    }
    
    // Apply similar pattern to other sync methods
}