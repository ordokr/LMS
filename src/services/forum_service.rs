use leptos::*;
use crate::models::forum::{
    Category, Topic, Post, ForumStats, Group, Site, GroupMembershipLevel,
    CreateTopicRequest, CreatePostRequest, UpdatePostRequest
};
use crate::utils::errors::ApiError;
use crate::utils::api_client::ApiClient;
use crate::sync::{SyncManager, SyncOperation};
use serde_json::Value;
use std::sync::Arc;
use futures::TryStreamExt;
use crate::db::Database;

#[derive(Clone)]
pub struct ForumService {
    client: ApiClient,
    db: Arc<Database>,
    api: Arc<ApiClient>,
    sync_manager: Arc<SyncManager>,
}

impl ForumService {
    pub fn new(db: Arc<Database>, api: Arc<ApiClient>, sync_manager: Arc<SyncManager>) -> Self {
        Self {
            client: ApiClient::new(),
            db,
            api,
            sync_manager,
        }
    }
    
    // Helper method to check if we're offline
    fn is_offline(&self) -> bool {
        // You'd implement this with your actual offline detection
        #[cfg(debug_assertions)]
        let offline = std::env::var("FORCE_OFFLINE").is_ok();
        #[cfg(not(debug_assertions))]
        let offline = !self.api.is_online();
        
        offline
    }

    // Categories
    pub async fn get_categories(&self) -> Result<Vec<Category>, ApiError> {
        self.client.get::<Vec<Category>>("/forum/categories").await
    }
    
    pub async fn get_category(&self, id: i64) -> Result<Category, ApiError> {
        self.client.get::<Category>(&format!("/forum/categories/{}", id)).await
    }
    
    pub async fn get_categories_by_course(&self, course_id: i64) -> Result<Vec<Category>, ApiError> {
        self.client.get::<Vec<Category>>(&format!("/forum/courses/{}/categories", course_id)).await
    }
    
    // Topics
    pub async fn get_topics(&self) -> Result<Vec<Topic>, ApiError> {
        self.client.get::<Vec<Topic>>("/forum/topics").await
    }
    
    pub async fn get_topics_by_category(&self, category_id: i64) -> Result<Vec<Topic>, ApiError> {
        self.client.get::<Vec<Topic>>(&format!("/forum/categories/{}/topics", category_id)).await
    }
    
    pub async fn get_topic(&self, id: i64) -> Result<Topic, ApiError> {
        self.client.get::<Topic>(&format!("/forum/topics/{}", id)).await
    }
    
    pub async fn create_topic(&self, request: CreateTopicRequest) -> Result<Topic, ApiError> {
        self.client.post::<CreateTopicRequest, Topic>("/forum/topics", request).await
    }
    
    // Posts
    pub async fn get_posts_by_topic(&self, topic_id: i64) -> Result<Vec<Post>, ApiError> {
        self.client.get::<Vec<Post>>(&format!("/forum/topics/{}/posts", topic_id)).await
    }
    
    pub async fn create_post(&self, request: CreatePostRequest) -> Result<Post, ApiError> {
        self.client.post::<CreatePostRequest, Post>(
            &format!("/forum/topics/{}/posts", request.topic_id), 
            request
        ).await
    }
    
    pub async fn update_post(&self, id: i64, content: String) -> Result<Post, ApiError> {
        let request = UpdatePostRequest { content };
        self.client.put::<UpdatePostRequest, Post>(&format!("/forum/posts/{}", id), request).await
    }
    
    pub async fn like_post(&self, id: i64) -> Result<Post, ApiError> {
        self.client.post::<(), Post>(&format!("/forum/posts/{}/like", id), ()).await
    }
    
    pub async fn get_stats(&self) -> Result<ForumStats, ApiError> {
        self.client.get::<ForumStats>("/forum/stats").await
    }
    
    /// Get recent forum activity across all categories
    pub async fn get_recent_activity(&self, limit: usize) -> Result<Vec<Topic>, ApiError> {
        let topics = self.client.get::<Vec<Topic>>("/forum/topics/recent").await?;
        
        // Sort by activity and limit
        let mut topics = topics;
        topics.sort_by(|a, b| {
            let a_date = a.last_post_at.unwrap_or(a.created_at);
            let b_date = b.last_post_at.unwrap_or(b.created_at);
            b_date.cmp(&a_date)
        });
        topics.truncate(limit);
        
        Ok(topics)
    }

    // Group methods
    pub async fn get_groups(&self) -> Result<Vec<Group>, ApiError> {
        if self.is_offline() {
            // Return from local storage
            self.db.get_groups().await.map_err(Into::into)
        } else {
            // Fetch from API and store locally
            let groups = self.api.get_groups().await?;
            self.db.store_groups(&groups).await?;
            Ok(groups)
        }
    }
    
    pub async fn get_group(&self, group_id: i64) -> Result<Option<Group>, ApiError> {
        if self.is_offline() {
            // Return from local storage
            self.db.get_group(group_id).await.map_err(Into::into)
        } else {
            // Fetch from API and store locally
            let group = self.api.get_group(group_id).await?;
            if let Some(group_data) = &group {
                self.db.store_group(group_data).await?;
            }
            Ok(group)
        }
    }

    pub async fn create_group(&self, group: &Group) -> Result<Group, ApiError> {
        let created_group = if self.is_offline() {
            // Store locally and mark for sync
            let local_group = self.db.store_group(group).await?;
            self.sync_manager.queue_operation(
                SyncOperation::Create { 
                    entity_type: "group".to_string(), 
                    data: serde_json::to_value(group)? 
                }
            ).await?;
            local_group
        } else {
            // Send to API and store locally
            let created = self.api.create_group(group).await?;
            self.db.store_group(&created).await?;
            created
        };
        
        Ok(created_group)
    }

    pub async fn update_group(&self, group: &Group) -> Result<Group, ApiError> {
        let updated_group = if self.is_offline() {
            // Update locally and mark for sync
            let local_group = self.db.update_group(group).await?;
            self.sync_manager.queue_operation(
                SyncOperation::Update { 
                    entity_type: "group".to_string(), 
                    id: group.id.to_string(),
                    fields: serde_json::to_value(group)?.as_object()
                        .ok_or_else(|| ApiError::InvalidData("Cannot serialize group".to_string()))?
                        .clone()
                        .into_iter()
                        .map(|(k, v)| (k, v))
                        .collect()
                }
            ).await?;
            local_group
        } else {
            // Send to API and update locally
            let updated = self.api.update_group(group).await?;
            self.db.update_group(&updated).await?;
            updated
        };
        
        Ok(updated_group)
    }

    pub async fn delete_group(&self, group_id: i64) -> Result<(), ApiError> {
        if self.is_offline() {
            // Mark as deleted locally and queue for sync
            self.db.delete_group(group_id).await?;
            self.sync_manager.queue_operation(
                SyncOperation::Delete { 
                    entity_type: "group".to_string(), 
                    id: group_id.to_string() 
                }
            ).await?;
        } else {
            // Delete from API and locally
            self.api.delete_group(group_id).await?;
            self.db.delete_group(group_id).await?;
        }
        
        Ok(())
    }

    // Site settings methods
    pub async fn get_site_settings(&self) -> Result<Site, ApiError> {
        if self.is_offline() {
            // Return from local storage
            self.db.get_site_settings().await.map_err(Into::into)
        } else {
            // Fetch from API and store locally
            let site = self.api.get_site_settings().await?;
            self.db.store_site_settings(&site).await?;
            Ok(site)
        }
    }

    pub async fn update_site_settings(&self, site: &Site) -> Result<Site, ApiError> {
        if self.is_offline() {
            // Update locally and mark for sync
            let updated_site = self.db.update_site_settings(site).await?;
            self.sync_manager.queue_operation(
                SyncOperation::Update { 
                    entity_type: "site".to_string(), 
                    id: site.id.to_string(),
                    fields: serde_json::to_value(site)?.as_object()
                        .ok_or_else(|| ApiError::InvalidData("Cannot serialize site".to_string()))?
                        .clone()
                        .into_iter()
                        .map(|(k, v)| (k, v))
                        .collect()
                }
            ).await?;
            updated_site
        } else {
            // Send to API and update locally
            let updated = self.api.update_site_settings(site).await?;
            self.db.update_site_settings(&updated).await?;
            updated
        }
    }
}