use std::sync::Arc;
use crate::models::discussion::{DiscussionMapping, DiscussionSyncSummary};
use crate::models::mapping::SyncDirection;
use crate::api::{canvas::CanvasClient, discourse::DiscourseClient};
use crate::db::Database;
use crate::error::Error;

pub struct DiscussionService {
    db: Arc<Database>,
    canvas_client: Arc<CanvasClient>,
    discourse_client: Arc<DiscourseClient>,
}

impl DiscussionService {
    pub fn new(
        db: Arc<Database>,
        canvas_client: Arc<CanvasClient>,
        discourse_client: Arc<DiscourseClient>,
    ) -> Self {
        Self {
            db,
            canvas_client,
            discourse_client,
        }
    }
    
    pub async fn create_discussion_mapping(
        &self,
        canvas_discussion_id: &str,
        discourse_topic_id: &str,
        course_category_id: &str,
        title: &str,
    ) -> Result<DiscussionMapping, Error> {
        // Verify course-category mapping exists
        let course_mapping = self.db.get_course_category(course_category_id).await?;
        
        // Check if mapping already exists
        if self.db.discussion_mapping_exists(canvas_discussion_id, discourse_topic_id).await? {
            return Err(Error::DuplicateMapping);
        }
        
        // Verify discussion exists in Canvas
        let _ = self.canvas_client.get_discussion(canvas_discussion_id).await?;
        
        // Verify topic exists in Discourse
        let _ = self.discourse_client.get_topic(discourse_topic_id).await?;
        
        // Create new mapping
        let mapping = DiscussionMapping::new(
            canvas_discussion_id,
            discourse_topic_id,
            course_category_id,
            title,
        );
        
        // Save to database
        self.db.save_discussion_mapping(&mapping).await?;
        
        Ok(mapping)
    }
    
    pub async fn get_discussion_mapping(&self, id: &str) -> Result<DiscussionMapping, Error> {
        self.db.get_discussion_mapping(id).await
    }
    
    pub async fn get_mappings_for_course(&self, course_category_id: &str) -> Result<Vec<DiscussionMapping>, Error> {
        self.db.get_discussion_mappings_by_course_category(course_category_id).await
    }
    
    pub async fn update_discussion_mapping(
        &self,
        id: &str,
        title: Option<&str>,
        sync_enabled: Option<bool>,
        sync_posts: Option<bool>,
    ) -> Result<DiscussionMapping, Error> {
        let mut mapping = self.db.get_discussion_mapping(id).await?;
        
        if let Some(title) = title {
            mapping.title = title.to_string();
        }
        
        if let Some(enabled) = sync_enabled {
            mapping.sync_enabled = enabled;
        }
        
        if let Some(sync_posts) = sync_posts {
            mapping.sync_posts = sync_posts;
        }
        
        self.db.save_discussion_mapping(&mapping).await?;
        
        Ok(mapping)
    }
    
    pub async fn sync_discussion_mapping(&self, id: &str) -> Result<DiscussionSyncSummary, Error> {
        let mut mapping = self.db.get_discussion_mapping(id).await?;
        
        mapping.sync(
            &self.db,
            &self.canvas_client,
            &self.discourse_client,
        ).await
    }
    
    pub async fn sync_all_for_course(&self, course_category_id: &str) -> Result<Vec<(String, Result<DiscussionSyncSummary, Error>)>, Error> {
        let mappings = self.db.get_discussion_mappings_by_course_category(course_category_id).await?;
        let mut results = Vec::new();
        
        for mut mapping in mappings {
            let result = mapping.sync(
                &self.db,
                &self.canvas_client,
                &self.discourse_client,
            ).await;
            
            results.push((mapping.id.clone(), result));
        }
        
        Ok(results)
    }
    
    pub async fn delete_discussion_mapping(&self, id: &str) -> Result<(), Error> {
        self.db.delete_discussion_mapping(id).await
    }
    
    // Advanced functionality: automatically create mappings for all discussions in a course
    pub async fn auto_create_mappings_for_course(
        &self,
        course_category_id: &str,
    ) -> Result<Vec<DiscussionMapping>, Error> {
        // Get course mapping
        let course_mapping = self.db.get_course_category(course_category_id).await?;
        
        // Get all discussions in Canvas course
        let canvas_discussions = self.canvas_client
            .get_course_discussions(&course_mapping.canvas_course_id)
            .await?;
        
        let mut created_mappings = Vec::new();
        
        // For each Canvas discussion, create a corresponding Discourse topic and mapping
        for discussion in canvas_discussions {
            // Create Discourse topic
            let discourse_topic = self.discourse_client
                .create_topic(
                    &course_mapping.discourse_category_id,
                    &discussion.title,
                    &discussion.message,
                )
                .await?;
            
            // Create mapping
            let mapping = DiscussionMapping::new(
                &discussion.id,
                &discourse_topic.id,
                &course_mapping.id,
                &discussion.title,
            );
            
            self.db.save_discussion_mapping(&mapping).await?;
            created_mappings.push(mapping);
        }
        
        Ok(created_mappings)
    }
}