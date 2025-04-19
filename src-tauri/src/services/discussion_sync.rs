use std::sync::Arc;
use crate::models::discussion_mapping::{
    DiscussionMapping, SyncResult, CanvasDiscussionEntry, DiscoursePost
};
// Import both old and new clients for backward compatibility
use crate::api::{canvas::CanvasClient, discourse::DiscourseClient};
use crate::api::unified_clients::{CanvasApiClient, DiscourseApiClient};
use crate::db::{DbPool, discussion_mappings};
use crate::models::course_category::{CourseCategory, SyncDirection};
use crate::error::Error;
use log::{info, error, warn};

pub struct DiscussionSyncService {
    pool: Arc<DbPool>,
    canvas_client: Arc<CanvasClient>,
    discourse_client: Arc<DiscourseClient>,
}

impl DiscussionSyncService {
    pub fn new(
        pool: Arc<DbPool>,
        canvas_client: Arc<CanvasClient>,
        discourse_client: Arc<DiscourseClient>,
    ) -> Self {
        Self {
            pool,
            canvas_client,
            discourse_client,
        }
    }

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
                    .get_discussion(&mapping.canvas_discussion_id)
                    .await?;

                let discourse_topic = self.discourse_client
                    .get_topic(&mapping.discourse_topic_id)
                    .await?;

                if canvas_discussion.updated_at > discourse_topic.updated_at {
                    self.sync_canvas_to_discourse_topic(mapping, result).await?;
                } else if discourse_topic.updated_at > canvas_discussion.updated_at {
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
            .get_discussion(&mapping.canvas_discussion_id)
            .await?;

        let update_result = self.discourse_client
            .update_topic(
                &mapping.discourse_topic_id,
                &canvas_discussion.title,
                &canvas_discussion.message,
            )
            .await;

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
            .get_topic_with_posts(&mapping.discourse_topic_id)
            .await?;

        // For Discourse, the first post is the topic content
        let first_post = discourse_topic.posts.first()
            .ok_or_else(|| Error::NotFound("Discourse topic has no posts".into()))?;

        let update_result = self.canvas_client
            .update_discussion(
                &mapping.canvas_discussion_id,
                &discourse_topic.title,
                &first_post.content,
            )
            .await;

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
        let entries = self.canvas_client
            .get_discussion_entries(&mapping.canvas_discussion_id)
            .await?;

        // Get all posts from Discourse topic (skip the first one, which is the topic itself)
        let discourse_posts = self.discourse_client
            .get_topic_posts(&mapping.discourse_topic_id)
            .await?;

        let mut posts_created = 0;
        let mut posts_updated = 0;

        for entry in entries {
            // Create a unique external ID for this Canvas entry
            let external_id = format!("canvas-entry-{}", entry.id);

            // Check if this entry already exists in Discourse
            let existing_post = discourse_posts.iter()
                .find(|p| p.external_id.as_deref() == Some(&external_id));

            if let Some(post) = existing_post {
                // If post exists and content is different, update it
                if post.content != entry.message {
                    match self.discourse_client.update_post(
                        &post.id,
                        &entry.message,
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
                    &entry.message,
                    Some(&external_id),
                    entry.user_id.as_deref(),
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
              entries.len(), posts_created, posts_updated);

        Ok(())
    }

    async fn sync_discourse_to_canvas_posts(
        &self,
        mapping: &DiscussionMapping,
        result: &mut SyncResult,
    ) -> Result<(), Error> {
        // Get all posts from Discourse topic (skip the first one, which is the topic itself)
        let discourse_posts = self.discourse_client
            .get_topic_posts(&mapping.discourse_topic_id)
            .await?;

        if discourse_posts.len() <= 1 {
            // Only the topic post exists, nothing to sync
            return Ok(());
        }

        // Skip the first post (topic content)
        let reply_posts = &discourse_posts[1..];

        // Get all entries from Canvas discussion
        let canvas_entries = self.canvas_client
            .get_discussion_entries(&mapping.canvas_discussion_id)
            .await?;

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
                canvas_entries.iter().find(|e| e.id == entry_id).cloned()
            } else {
                None
            };

            if let Some(entry) = canvas_entry {
                // If content is different, update Canvas entry
                if entry.message != post.content {
                    match self.canvas_client.update_discussion_entry(
                        &mapping.canvas_discussion_id,
                        &entry.id,
                        &post.content,
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
                    &mapping.canvas_discussion_id,
                    &post.content,
                    post.user_id.as_deref(),
                ).await {
                    Ok(entry_id) => {
                        // Update the post with the Canvas entry ID for future reference
                        let external_id = format!("canvas-entry-{}", entry_id);
                        if self.discourse_client.update_post_external_id(
                            &post.id,
                            &external_id
                        ).await.is_ok() {
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