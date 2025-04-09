use anyhow::{Result, anyhow};
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::repository::topic_mapping_repository::TopicMappingRepository;
use crate::repository::course_category_repository::CourseCategoryRepository;
use crate::models::forum::mapping::{TopicMapping, PostMapping};

pub struct SyncService {
    topic_mapping_repo: TopicMappingRepository,
    course_category_repo: CourseCategoryRepository,
}

impl SyncService {
    pub fn new(
        topic_mapping_repo: TopicMappingRepository,
        course_category_repo: CourseCategoryRepository,
    ) -> Self {
        Self {
            topic_mapping_repo,
            course_category_repo,
        }
    }
    
    // Topic mapping methods
    pub async fn create_topic_mapping(
        &self, 
        canvas_topic_id: &str, 
        discourse_topic_id: &str,
        course_id: &str
    ) -> Result<TopicMapping> {
        // Find the course-category mapping first
        let course_mapping = self.course_category_repo
            .get_by_course_id(course_id)
            .await?;
            
        // Check if topic is already mapped
        if let Ok(_) = self.topic_mapping_repo
            .get_topic_mapping_by_canvas_id(canvas_topic_id).await {
            return Err(anyhow!("Canvas topic ID {} is already mapped", canvas_topic_id));
        }
        
        if let Ok(_) = self.topic_mapping_repo
            .get_topic_mapping_by_discourse_id(discourse_topic_id).await {
            return Err(anyhow!("Discourse topic ID {} is already mapped", discourse_topic_id));
        }
        
        // Create new mapping
        self.topic_mapping_repo
            .create_topic_mapping(canvas_topic_id, discourse_topic_id, course_mapping.id)
            .await
    }
    
    pub async fn get_topic_mappings_for_course(&self, course_id: &str) -> Result<Vec<TopicMapping>> {
        let course_mapping = self.course_category_repo
            .get_by_course_id(course_id)
            .await?;
            
        self.topic_mapping_repo
            .get_topic_mappings_by_course_category_mapping(course_mapping.id)
            .await
    }
    
    pub async fn toggle_topic_sync(&self, topic_mapping_id: Uuid, enabled: bool) -> Result<TopicMapping> {
        self.topic_mapping_repo
            .update_topic_mapping_sync_enabled(topic_mapping_id, enabled)
            .await
    }
    
    pub async fn record_canvas_topic_update(&self, canvas_topic_id: &str, canvas_time: DateTime<Utc>) -> Result<TopicMapping> {
        let mapping = self.topic_mapping_repo
            .get_topic_mapping_by_canvas_id(canvas_topic_id)
            .await?;
            
        self.topic_mapping_repo
            .update_topic_canvas_time(mapping.id, canvas_time)
            .await
    }
    
    pub async fn record_discourse_topic_update(&self, discourse_topic_id: &str, discourse_time: DateTime<Utc>) -> Result<TopicMapping> {
        let mapping = self.topic_mapping_repo
            .get_topic_mapping_by_discourse_id(discourse_topic_id)
            .await?;
            
        self.topic_mapping_repo
            .update_topic_discourse_time(mapping.id, discourse_time)
            .await
    }
    
    // Post mapping methods
    pub async fn create_post_mapping(
        &self,
        canvas_entry_id: &str,
        discourse_post_id: &str,
        canvas_topic_id: &str
    ) -> Result<PostMapping> {
        // Find the topic mapping first
        let topic_mapping = self.topic_mapping_repo
            .get_topic_mapping_by_canvas_id(canvas_topic_id)
            .await?;
            
        // Create the post mapping
        self.topic_mapping_repo
            .create_post_mapping(canvas_entry_id, discourse_post_id, topic_mapping.id)
            .await
    }
    
    pub async fn get_posts_for_topic(&self, topic_mapping_id: Uuid) -> Result<Vec<PostMapping>> {
        self.topic_mapping_repo
            .get_post_mappings_by_topic_mapping(topic_mapping_id)
            .await
    }
    
    // Get sync status - determine if sync is needed by comparing update timestamps
    pub async fn needs_sync(&self, topic_mapping_id: Uuid) -> Result<Option<String>> {
        let mapping = self.topic_mapping_repo
            .get_topic_mapping_by_id(topic_mapping_id)
            .await?;
            
        if !mapping.sync_enabled {
            return Ok(None); // Sync disabled
        }
        
        // If one system has updates more recent than the other or last sync
        match (mapping.canvas_updated_at, mapping.discourse_updated_at, mapping.last_synced_at) {
            // No updates recorded yet
            (None, None, _) => Ok(None),
            
            // Canvas updated, but Discourse not yet (or never)
            (Some(canvas_time), None, last_sync) => {
                if let Some(sync_time) = last_sync {
                    if canvas_time > sync_time {
                        return Ok(Some("canvas_to_discourse".to_string()));
                    }
                } else {
                    return Ok(Some("canvas_to_discourse".to_string()));
                }
                Ok(None)
            },
            
            // Discourse updated, but Canvas not yet (or never)
            (None, Some(discourse_time), last_sync) => {
                if let Some(sync_time) = last_sync {
                    if discourse_time > sync_time {
                        return Ok(Some("discourse_to_canvas".to_string()));
                    }
                } else {
                    return Ok(Some("discourse_to_canvas".to_string()));
                }
                Ok(None)
            },
            
            // Both updated, determine which is more recent
            (Some(canvas_time), Some(discourse_time), last_sync) => {
                if let Some(sync_time) = last_sync {
                    if canvas_time > sync_time && canvas_time > discourse_time {
                        return Ok(Some("canvas_to_discourse".to_string()));
                    } else if discourse_time > sync_time && discourse_time > canvas_time {
                        return Ok(Some("discourse_to_canvas".to_string()));
                    }
                } else {
                    if canvas_time > discourse_time {
                        return Ok(Some("canvas_to_discourse".to_string()));
                    } else {
                        return Ok(Some("discourse_to_canvas".to_string()));
                    }
                }
                Ok(None)
            }
        }
    }
}