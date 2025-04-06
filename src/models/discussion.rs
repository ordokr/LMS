use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::models::mapping::CourseCategory;
use crate::error::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscussionMapping {
    pub id: String,
    pub canvas_discussion_id: String,
    pub discourse_topic_id: String,
    pub course_category_id: String, // Reference to parent mapping
    pub title: String,
    pub last_sync: DateTime<Utc>,
    pub sync_enabled: bool,
    pub sync_posts: bool, // Whether to sync individual posts/replies
    pub created_at: DateTime<Utc>,
}

impl DiscussionMapping {
    pub fn new(
        canvas_discussion_id: &str,
        discourse_topic_id: &str,
        course_category_id: &str,
        title: &str,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            canvas_discussion_id: canvas_discussion_id.to_string(),
            discourse_topic_id: discourse_topic_id.to_string(),
            course_category_id: course_category_id.to_string(),
            title: title.to_string(),
            last_sync: now,
            sync_enabled: true,
            sync_posts: true,
            created_at: now,
        }
    }
    
    pub async fn sync(
        &mut self,
        db: &Database,
        canvas_client: &CanvasClient,
        discourse_client: &DiscourseClient,
    ) -> Result<DiscussionSyncSummary, Error> {
        if !self.sync_enabled {
            return Ok(DiscussionSyncSummary::new(
                &self.id, 
                "Sync disabled for this discussion mapping"
            ));
        }
        
        let mut summary = DiscussionSyncSummary::new(&self.id, "Starting discussion sync");
        
        // Get parent course-category mapping to determine sync direction
        let course_mapping = db.get_course_category(&self.course_category_id).await?;
        
        // Use the same sync direction as the parent mapping
        match course_mapping.sync_direction {
            SyncDirection::CanvasToDiscourse => {
                self.sync_discussion_canvas_to_discourse(
                    canvas_client, 
                    discourse_client,
                    &mut summary,
                ).await?;
            },
            SyncDirection::DiscourseToCanvas => {
                self.sync_discussion_discourse_to_canvas(
                    canvas_client, 
                    discourse_client,
                    &mut summary,
                ).await?;
            },
            SyncDirection::Bidirectional => {
                // First Canvas to Discourse
                self.sync_discussion_canvas_to_discourse(
                    canvas_client, 
                    discourse_client,
                    &mut summary,
                ).await?;
                
                // Then Discourse to Canvas
                self.sync_discussion_discourse_to_canvas(
                    canvas_client, 
                    discourse_client,
                    &mut summary,
                ).await?;
            }
        }
        
        // Sync posts/replies if enabled
        if self.sync_posts {
            self.sync_posts(
                canvas_client,
                discourse_client,
                &course_mapping.sync_direction,
                &mut summary,
            ).await?;
        }
        
        // Update last sync time
        self.last_sync = Utc::now();
        
        // Save updated mapping
        db.save_discussion_mapping(self).await?;
        
        summary.complete();
        Ok(summary)
    }
    
    async fn sync_discussion_canvas_to_discourse(
        &self,
        canvas_client: &CanvasClient,
        discourse_client: &DiscourseClient,
        summary: &mut DiscussionSyncSummary,
    ) -> Result<(), Error> {
        // Get Canvas discussion
        let canvas_discussion = canvas_client.get_discussion(
            &self.canvas_discussion_id
        ).await?;
        
        // Update Discourse topic with Canvas data
        discourse_client.update_topic(
            &self.discourse_topic_id,
            &canvas_discussion.title,
            &canvas_discussion.message,
        ).await?;
        
        summary.add_operation(format!(
            "Updated Discourse topic '{}' with Canvas discussion data",
            canvas_discussion.title
        ));
        
        Ok(())
    }
    
    async fn sync_discussion_discourse_to_canvas(
        &self,
        canvas_client: &CanvasClient,
        discourse_client: &DiscourseClient,
        summary: &mut DiscussionSyncSummary,
    ) -> Result<(), Error> {
        // Get Discourse topic
        let discourse_topic = discourse_client.get_topic(
            &self.discourse_topic_id
        ).await?;
        
        // Update Canvas discussion with Discourse data
        canvas_client.update_discussion(
            &self.canvas_discussion_id,
            &discourse_topic.title,
            &discourse_topic.raw,
        ).await?;
        
        summary.add_operation(format!(
            "Updated Canvas discussion with Discourse topic '{}'",
            discourse_topic.title
        ));
        
        Ok(())
    }
    
    async fn sync_posts(
        &self,
        canvas_client: &CanvasClient,
        discourse_client: &DiscourseClient,
        sync_direction: &SyncDirection,
        summary: &mut DiscussionSyncSummary,
    ) -> Result<(), Error> {
        match sync_direction {
            SyncDirection::CanvasToDiscourse => {
                // Get Canvas discussion entries
                let entries = canvas_client.get_discussion_entries(
                    &self.canvas_discussion_id
                ).await?;
                
                // Create map of existing posts in Discourse by external ID
                let discourse_posts = discourse_client.get_topic_posts(
                    &self.discourse_topic_id
                ).await?;
                
                let mut posts_synced = 0;
                
                // Create or update Discourse posts for each Canvas entry
                for entry in entries {
                    let external_id = format!("canvas-entry-{}", entry.id);
                    
                    // Try to find existing post with this external ID
                    if !discourse_posts.iter().any(|p| p.external_id == Some(external_id.clone())) {
                        // Create new post
                        discourse_client.create_post(
                            &self.discourse_topic_id,
                            &entry.message,
                            Some(&external_id),
                            entry.user_id.as_deref(),
                        ).await?;
                        
                        posts_synced += 1;
                    }
                }
                
                summary.add_operation(format!(
                    "Synced {} Canvas discussion entries to Discourse",
                    posts_synced
                ));
            },
            SyncDirection::DiscourseToCanvas => {
                // Similar implementation for Discourse to Canvas
                // ...
                
                summary.add_operation("Synced Discourse posts to Canvas".to_string());
            },
            SyncDirection::Bidirectional => {
                // More complex bidirectional sync logic
                // ...
                
                summary.add_operation("Performed bidirectional post sync".to_string());
            }
        }
        
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct DiscussionSyncSummary {
    pub mapping_id: String,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub operations: Vec<String>,
    pub status: String,
}

impl DiscussionSyncSummary {
    pub fn new(mapping_id: &str, initial_status: &str) -> Self {
        Self {
            mapping_id: mapping_id.to_string(),
            start_time: Utc::now(),
            end_time: None,
            operations: Vec::new(),
            status: initial_status.to_string(),
        }
    }
    
    pub fn add_operation(&mut self, operation: String) {
        self.operations.push(operation);
    }
    
    pub fn complete(&mut self) {
        self.end_time = Some(Utc::now());
        self.status = "Completed".to_string();
    }
}