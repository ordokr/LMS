use std::sync::Arc;
use crate::models::discussion_mapping::{
    DiscussionMapping, SyncResult, CanvasDiscussionEntry, DiscoursePost
};
use crate::api::unified_clients::{CanvasApiClient, DiscourseApiClient, Result as ApiResult};
use crate::db::{DbPool, discussion_mappings};
use crate::models::course_category::{CourseCategory, SyncDirection};
use crate::error::Error;
use log::{info, error, warn};

/// Discussion synchronization service using unified API clients
pub struct UnifiedDiscussionSyncService {
    pool: Arc<DbPool>,
    canvas_client: Arc<CanvasApiClient>,
    discourse_client: Arc<DiscourseApiClient>,
}

impl UnifiedDiscussionSyncService {
    /// Create a new discussion sync service
    pub fn new(
        pool: Arc<DbPool>,
        canvas_client: Arc<CanvasApiClient>,
        discourse_client: Arc<DiscourseApiClient>,
    ) -> Self {
        Self {
            pool,
            canvas_client,
            discourse_client,
        }
    }
    
    /// Synchronize a discussion between Canvas and Discourse
    pub async fn sync_discussion(
        &self,
        mapping_id: &str,
    ) -> Result<SyncResult, Error> {
        let mapping = discussion_mappings::get_discussion_mapping(&self.pool, mapping_id).await?;
        
        if !mapping.sync_enabled {
            let mut result = SyncResult::new(mapping_id);
            result.status = "skipped".to_string();
            return Ok(result);
        }
        
        // Get parent course mapping to determine sync direction
        let course_category = self.get_course_category(&mapping.course_category_id).await?;
        
        let mut result = SyncResult::new(mapping_id);
        
        // Sync discussion topics (titles, content)
        match self.sync_discussion_topics(&mapping, &course_category, &mut result).await {
            Ok(_) => (),
            Err(e) => {
                result.set_failed(format!("Error syncing discussion topics: {}", e));
                return Ok(result);
            }
        }
        
        // Sync discussion entries/posts if enabled
        if mapping.sync_posts {
            match self.sync_discussion_posts(&mapping, &course_category, &mut result).await {
                Ok(_) => (),
                Err(e) => {
                    result.add_error(format!("Error syncing posts: {}", e));
                }
            }
        }
        
        // Update last sync timestamp
        if let Err(e) = discussion_mappings::update_sync_timestamp(&self.pool, mapping_id).await {
            result.add_error(format!("Failed to update sync timestamp: {}", e));
        }
        
        result.complete();
        Ok(result)
    }
    
    async fn get_course_category(&self, id: &str) -> Result<CourseCategory, Error> {
        // This would call your course_category database function
        // For now, using a placeholder
        // Implement get_course_category (TODO)
    }
    
    async fn sync_discussion_topics(
        &self,
        mapping: &DiscussionMapping,
        course_category: &CourseCategory,
        result: &mut SyncResult,
    ) -> Result<(), Error> {
        match course_category.sync_direction {
            SyncDirection::CanvasToDiscourse => {
                self.sync_canvas_to_discourse_topic(mapping, result).await?;
            }
            SyncDirection::DiscourseToCanvas => {
                self.sync_discourse_to_canvas_topic(mapping, result).await?;
            }
            SyncDirection::Bidirectional => {
                // For bidirectional sync, we need to compare timestamps
                let canvas_discussion = self.canvas_client
                    .get_discussion_topic(&mapping.course_id, &mapping.canvas_discussion_id)
                    .await
                    .map_err(|e| Error::Api(format!("Canvas API error: {}", e)))?;
                
                let discourse_topic = self.discourse_client
                    .get_topic(&mapping.discourse_topic_id)
                    .await
                    .map_err(|e| Error::Api(format!("Discourse API error: {}", e)))?;
                
                // Compare updated_at timestamps
                let canvas_updated = canvas_discussion.updated_at.unwrap_or_else(|| canvas_discussion.created_at.unwrap_or_default());
                let discourse_updated = discourse_topic.updated_at.unwrap_or_else(|| discourse_topic.created_at.unwrap_or_default());
                
                if canvas_updated > discourse_updated {
                    self.sync_canvas_to_discourse_topic(mapping, result).await?;
                } else if discourse_updated > canvas_updated {
                    self.sync_discourse_to_canvas_topic(mapping, result).await?;
                }
            }
        }
        
        Ok(())
    }
    
    async fn sync_canvas_to_discourse_topic(
        &self,
        mapping: &DiscussionMapping,
        result: &mut SyncResult,
    ) -> Result<(), Error> {
        let canvas_discussion = self.canvas_client
            .get_discussion_topic(&mapping.course_id, &mapping.canvas_discussion_id)
            .await
            .map_err(|e| Error::Api(format!("Canvas API error: {}", e)))?;
        
        // Extract title and content from Canvas discussion
        let title = canvas_discussion.title.clone();
        let content = canvas_discussion.message.clone().unwrap_or_default();
        
        // Update Discourse topic
        let update_result = self.discourse_client
            .update_topic(&mapping.discourse_topic_id, &title, &content)
            .await
            .map_err(|e| Error::Api(format!("Discourse API error: {}", e)));
            
        match update_result {
            Ok(_) => {
                result.discourse_updates += 1;
                info!("Updated Discourse topic {} from Canvas discussion {}", 
                      mapping.discourse_topic_id, mapping.canvas_discussion_id);
                Ok(())
            }
            Err(e) => {
                result.add_error(format!("Failed to update Discourse topic: {}", e));
                Err(e)
            }
        }
    }
    
    async fn sync_discourse_to_canvas_topic(
        &self,
        mapping: &DiscussionMapping,
        result: &mut SyncResult,
    ) -> Result<(), Error> {
        let discourse_topic = self.discourse_client
            .get_topic(&mapping.discourse_topic_id)
            .await
            .map_err(|e| Error::Api(format!("Discourse API error: {}", e)))?;
        
        // Get the first post which contains the topic content
        let pagination = crate::api::unified_clients::PaginationParams {
            page: Some(1),
            per_page: Some(1),
            cursor: None,
        };
        
        let posts_response = self.discourse_client
            .get_topic_posts(&mapping.discourse_topic_id, &pagination)
            .await
            .map_err(|e| Error::Api(format!("Discourse API error: {}", e)))?;
        
        // For Discourse, the first post is the topic content
        let first_post = posts_response.items.first()
            .ok_or_else(|| Error::NotFound("Discourse topic has no posts".into()))?;
        
        // Extract content from the first post
        let content = first_post.raw.clone().unwrap_or_default();
        
        // Update Canvas discussion
        let update_result = self.canvas_client
            .update_discussion_topic(
                &mapping.course_id,
                &mapping.canvas_discussion_id,
                &discourse_topic.title,
                &content,
            )
            .await
            .map_err(|e| Error::Api(format!("Canvas API error: {}", e)));
            
        match update_result {
            Ok(_) => {
                result.canvas_updates += 1;
                info!("Updated Canvas discussion {} from Discourse topic {}", 
                      mapping.canvas_discussion_id, mapping.discourse_topic_id);
                Ok(())
            }
            Err(e) => {
                result.add_error(format!("Failed to update Canvas discussion: {}", e));
                Err(e)
            }
        }
    }
    
    async fn sync_discussion_posts(
        &self,
        mapping: &DiscussionMapping,
        course_category: &CourseCategory,
        result: &mut SyncResult,
    ) -> Result<(), Error> {
        match course_category.sync_direction {
            SyncDirection::CanvasToDiscourse => {
                self.sync_canvas_to_discourse_posts(mapping, result).await?;
            }
            SyncDirection::DiscourseToCanvas => {
                self.sync_discourse_to_canvas_posts(mapping, result).await?;
            }
            SyncDirection::Bidirectional => {
                // For bidirectional, do both
                if let Err(e) = self.sync_canvas_to_discourse_posts(mapping, result).await {
                    result.add_error(format!("Error syncing Canvas to Discourse posts: {}", e));
                }
                
                if let Err(e) = self.sync_discourse_to_canvas_posts(mapping, result).await {
                    result.add_error(format!("Error syncing Discourse to Canvas posts: {}", e));
                }
            }
        }
        
        Ok(())
    }
    
    async fn sync_canvas_to_discourse_posts(
        &self,
        mapping: &DiscussionMapping,
        result: &mut SyncResult,
    ) -> Result<(), Error> {
        // Get all entries from Canvas discussion
        let entries_response = self.canvas_client
            .get_discussion_entries(&mapping.course_id, &mapping.canvas_discussion_id)
            .await
            .map_err(|e| Error::Api(format!("Canvas API error: {}", e)))?;
        
        // Get all posts from Discourse topic (skip the first one, which is the topic itself)
        let pagination = crate::api::unified_clients::PaginationParams {
            page: Some(1),
            per_page: Some(100), // Adjust as needed
            cursor: None,
        };
        
        let discourse_posts_response = self.discourse_client
            .get_topic_posts(&mapping.discourse_topic_id, &pagination)
            .await
            .map_err(|e| Error::Api(format!("Discourse API error: {}", e)))?;
        
        let discourse_posts = discourse_posts_response.items;
        
        let mut posts_created = 0;
        let mut posts_updated = 0;
        
        for entry in entries_response.items {
            // Create a unique external ID for this Canvas entry
            let entry_id = entry.id.to_string();
            let external_id = format!("canvas-entry-{}", entry_id);
            
            // Check if this entry already exists in Discourse
            let existing_post = discourse_posts.iter()
                .find(|p| p.external_id.as_deref() == Some(&external_id));
            
            if let Some(post) = existing_post {
                // If post exists and content is different, update it
                let post_content = post.raw.clone().unwrap_or_default();
                let entry_message = entry.message.unwrap_or_default();
                
                if post_content != entry_message {
                    match self.discourse_client.update_post(
                        &post.id.to_string(),
                        &entry_message,
                    ).await {
                        Ok(_) => {
                            posts_updated += 1;
                            result.discourse_updates += 1;
                        }
                        Err(e) => {
                            result.add_error(format!("Failed to update Discourse post: {}", e));
                        }
                    }
                }
            } else {
                // Create new post in Discourse
                match self.discourse_client.create_post(
                    &mapping.discourse_topic_id,
                    &entry.message.unwrap_or_default(),
                ).await {
                    Ok(_) => {
                        posts_created += 1;
                        result.discourse_updates += 1;
                    }
                    Err(e) => {
                        result.add_error(format!("Failed to create Discourse post: {}", e));
                    }
                }
            }
        }
        
        info!("Synced {} entries from Canvas to Discourse: {} created, {} updated", 
              entries_response.items.len(), posts_created, posts_updated);
        
        Ok(())
    }
    
    async fn sync_discourse_to_canvas_posts(
        &self,
        mapping: &DiscussionMapping,
        result: &mut SyncResult,
    ) -> Result<(), Error> {
        // Get all posts from Discourse topic
        let pagination = crate::api::unified_clients::PaginationParams {
            page: Some(1),
            per_page: Some(100), // Adjust as needed
            cursor: None,
        };
        
        let discourse_posts_response = self.discourse_client
            .get_topic_posts(&mapping.discourse_topic_id, &pagination)
            .await
            .map_err(|e| Error::Api(format!("Discourse API error: {}", e)))?;
            
        let discourse_posts = discourse_posts_response.items;
        
        if discourse_posts.len() <= 1 {
            // Only the topic post exists, nothing to sync
            return Ok(());
        }
        
        // Skip the first post (topic content)
        let reply_posts = &discourse_posts[1..];
        
        // Get all entries from Canvas discussion
        let entries_response = self.canvas_client
            .get_discussion_entries(&mapping.course_id, &mapping.canvas_discussion_id)
            .await
            .map_err(|e| Error::Api(format!("Canvas API error: {}", e)))?;
        
        let canvas_entries = entries_response.items;
        
        let mut entries_created = 0;
        let mut entries_updated = 0;
        
        for post in reply_posts {
            // Skip posts without external_id as they might be created in Discourse directly
            // and we want to avoid duplicating them in Canvas
            if post.external_id.is_some() && !post.external_id.as_ref().unwrap().starts_with("canvas-entry-") {
                continue;
            }
            
            // For posts created in Discourse, create entries in Canvas
            let canvas_entry = if let Some(external_id) = &post.external_id {
                // This was originally from Canvas, find the matching entry
                let entry_id = external_id.strip_prefix("canvas-entry-").unwrap_or(external_id);
                canvas_entries.iter().find(|e| e.id.to_string() == entry_id).cloned()
            } else {
                None
            };
            
            if let Some(entry) = canvas_entry {
                // If content is different, update Canvas entry
                let post_content = post.raw.clone().unwrap_or_default();
                let entry_message = entry.message.unwrap_or_default();
                
                if entry_message != post_content {
                    match self.canvas_client.update_discussion_entry(
                        &mapping.course_id,
                        &mapping.canvas_discussion_id,
                        &entry.id.to_string(),
                        &post_content,
                    ).await {
                        Ok(_) => {
                            entries_updated += 1;
                            result.canvas_updates += 1;
                        }
                        Err(e) => {
                            result.add_error(format!("Failed to update Canvas entry: {}", e));
                        }
                    }
                }
            } else {
                // Create new entry in Canvas
                match self.canvas_client.create_discussion_entry(
                    &mapping.course_id,
                    &mapping.canvas_discussion_id,
                    &post.raw.clone().unwrap_or_default(),
                ).await {
                    Ok(entry) => {
                        // Update the post with the Canvas entry ID for future reference
                        let external_id = format!("canvas-entry-{}", entry.id);
                        
                        // Update the post's external_id in Discourse
                        if let Ok(_) = self.discourse_client.update_post_external_id(
                            &post.id.to_string(), 
                            &external_id
                        ).await {
                            entries_created += 1;
                            result.canvas_updates += 1;
                        }
                    }
                    Err(e) => {
                        result.add_error(format!("Failed to create Canvas entry: {}", e));
                    }
                }
            }
        }
        
        info!("Synced {} posts from Discourse to Canvas: {} created, {} updated", 
              reply_posts.len(), entries_created, entries_updated);
        
        Ok(())
    }
    
    pub async fn sync_all_for_course(
        &self,
        course_category_id: &str,
    ) -> Result<Vec<SyncResult>, Error> {
        let mappings = discussion_mappings::get_discussion_mappings_by_course(
            &self.pool, course_category_id
        ).await?;
        
        let mut results = Vec::new();
        
        for mapping in mappings {
            let result = self.sync_discussion(&mapping.id).await?;
            results.push(result);
        }
        
        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::{mock, server_url};
    
    #[tokio::test]
    async fn test_sync_discussion() {
        // TODO: Implement tests for the unified discussion sync service
    }
}
