use crate::models::forum::topic::Topic;
use crate::models::forum::post::Post;
use crate::models::forum::mapping::{TopicMapping, PostMapping, SyncStatus};
use crate::db::DB;
use crate::error::Error;
use uuid::Uuid;
use chrono::Utc;
use std::time::Instant;
use serde_json::Value as JsonValue;
use crate::api::integration_commands::ConflictResolutionStrategy;

use super::canvas_integration::CanvasIntegration;
use super::discourse_integration::DiscourseIntegration;
use super::event_listener::{IntegrationEventDispatcher, IntegrationEventListener};
use super::conflict_resolver::{ConflictResolver, ConflictStrategy};
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
    conflict_resolver: ConflictResolver,
}

impl<C, D> IntegrationSyncService<C, D>
where
    C: CanvasIntegration + Send + Sync,
    D: DiscourseIntegration + Send + Sync,
{    pub fn new(db: DB, canvas: C, discourse: D) -> Self {
        let event_dispatcher = Arc::new(IntegrationEventDispatcher::new());
        let conflict_resolver = ConflictResolver::new(ConflictStrategy::PreferMostRecent);

        IntegrationSyncService {
            db,
            canvas,
            discourse,
            event_dispatcher,
            conflict_resolver,
        }
    }

    /// Set the conflict resolution strategy
    pub fn with_conflict_strategy(mut self, strategy: ConflictStrategy) -> Self {
        self.conflict_resolver = ConflictResolver::new(strategy);
        self
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

    // Get sync conflicts
    pub async fn get_sync_conflicts(&self) -> Result<Vec<JsonValue>, Error> {
        // Query the database for topics with conflict status
        let topic_conflicts = sqlx::query!(r#"
            SELECT
                t.id,
                t.title,
                t.updated_at as local_updated_at,
                tm.id as mapping_id,
                tm.canvas_topic_id,
                tm.discourse_topic_id,
                tm.sync_status
            FROM topics t
            JOIN topic_mappings tm ON t.id = tm.local_topic_id
            WHERE tm.sync_status = 'Conflict'
        "#)
        .fetch_all(&self.db)
        .await?;

        let mut conflicts = Vec::new();

        for conflict in topic_conflicts {
            // Fetch Canvas and Discourse versions
            let canvas_topic = if let Some(canvas_id) = &conflict.canvas_topic_id {
                match self.canvas.fetch_canvas_topic(canvas_id).await {
                    Ok(topic) => Some(topic),
                    Err(_) => None,
                }
            } else {
                None
            };

            let discourse_topic = if let Some(discourse_id) = conflict.discourse_topic_id {
                match self.discourse.fetch_discourse_topic(discourse_id).await {
                    Ok(topic) => Some(topic),
                    Err(_) => None,
                }
            } else {
                None
            };

            // If we have both versions, create a conflict record
            if let (Some(canvas), Some(discourse)) = (&canvas_topic, &discourse_topic) {
                let conflict_json = serde_json::json!({
                    "id": conflict.mapping_id,
                    "entity_type": "Topic",
                    "entity_id": conflict.id,
                    "title": conflict.title,
                    "canvas_updated_at": canvas.updated_at.to_rfc3339(),
                    "discourse_updated_at": discourse.updated_at.to_rfc3339(),
                    "canvas_content": canvas.message.clone().unwrap_or_default(),
                    "discourse_content": discourse.raw.clone().unwrap_or_default(),
                    "detected_at": conflict.local_updated_at,
                });

                conflicts.push(conflict_json);
            }
        }

        // Also query for post conflicts
        let post_conflicts = sqlx::query!(r#"
            SELECT
                p.id,
                p.content as title,
                p.updated_at as local_updated_at,
                pm.id as mapping_id,
                pm.canvas_entry_id,
                pm.discourse_post_id,
                pm.sync_status
            FROM posts p
            JOIN post_mappings pm ON p.id = pm.local_post_id
            WHERE pm.sync_status = 'Conflict'
        "#)
        .fetch_all(&self.db)
        .await?;

        for conflict in post_conflicts {
            // Fetch Canvas and Discourse versions
            let canvas_post = if let Some(canvas_id) = &conflict.canvas_entry_id {
                match self.canvas.fetch_canvas_entry(canvas_id).await {
                    Ok(post) => Some(post),
                    Err(_) => None,
                }
            } else {
                None
            };

            let discourse_post = if let Some(discourse_id) = conflict.discourse_post_id {
                match self.discourse.fetch_discourse_post(discourse_id).await {
                    Ok(post) => Some(post),
                    Err(_) => None,
                }
            } else {
                None
            };

            // If we have both versions, create a conflict record
            if let (Some(canvas), Some(discourse)) = (&canvas_post, &discourse_post) {
                let conflict_json = serde_json::json!({
                    "id": conflict.mapping_id,
                    "entity_type": "Post",
                    "entity_id": conflict.id,
                    "title": conflict.title,
                    "canvas_updated_at": canvas.updated_at.to_rfc3339(),
                    "discourse_updated_at": discourse.updated_at.to_rfc3339(),
                    "canvas_content": canvas.message.clone(),
                    "discourse_content": discourse.raw.clone(),
                    "detected_at": conflict.local_updated_at,
                });

                conflicts.push(conflict_json);
            }
        }

        Ok(conflicts)
    }

    // Get sync status
    pub async fn get_sync_status(&self) -> Result<JsonValue, Error> {
        // Count topics and posts by sync status
        let topic_counts = sqlx::query!(r#"
            SELECT sync_status, COUNT(*) as count
            FROM topic_mappings
            GROUP BY sync_status
        "#)
        .fetch_all(&self.db)
        .await?;

        let post_counts = sqlx::query!(r#"
            SELECT sync_status, COUNT(*) as count
            FROM post_mappings
            GROUP BY sync_status
        "#)
        .fetch_all(&self.db)
        .await?;

        // Get last sync time
        let last_sync = sqlx::query!(r#"
            SELECT MAX(last_sync_at) as last_sync
            FROM topic_mappings
            WHERE sync_status = 'Synced'
        "#)
        .fetch_optional(&self.db)
        .await?;

        // Build status JSON
        let mut status = serde_json::json!({
            "connected": true,
            "last_sync": last_sync.and_then(|r| r.last_sync),
            "topics": {
                "total": 0,
                "synced": 0,
                "pending": 0,
                "conflicts": 0,
                "errors": 0
            },
            "posts": {
                "total": 0,
                "synced": 0,
                "pending": 0,
                "conflicts": 0,
                "errors": 0
            }
        });

        // Update topic counts
        let mut total_topics = 0;
        for row in topic_counts {
            let count = row.count as u64;
            total_topics += count;

            match row.sync_status.as_str() {
                "Synced" => status["topics"]["synced"] = count.into(),
                "PendingToCanvas" | "PendingToDiscourse" => status["topics"]["pending"] = count.into(),
                "Conflict" => status["topics"]["conflicts"] = count.into(),
                "Error" => status["topics"]["errors"] = count.into(),
                _ => {}
            }
        }
        status["topics"]["total"] = total_topics.into();

        // Update post counts
        let mut total_posts = 0;
        for row in post_counts {
            let count = row.count as u64;
            total_posts += count;

            match row.sync_status.as_str() {
                "Synced" => status["posts"]["synced"] = count.into(),
                "PendingToCanvas" | "PendingToDiscourse" => status["posts"]["pending"] = count.into(),
                "Conflict" => status["posts"]["conflicts"] = count.into(),
                "Error" => status["posts"]["errors"] = count.into(),
                _ => {}
            }
        }
        status["posts"]["total"] = total_posts.into();

        Ok(status)
    }

    // Get sync history
    pub async fn get_sync_history(&self) -> Result<Vec<JsonValue>, Error> {
        let history = sqlx::query!(r#"
            SELECT * FROM sync_history
            ORDER BY sync_time DESC
            LIMIT 100
        "#)
        .fetch_all(&self.db)
        .await?;

        let history_json = history.into_iter().map(|row| {
            serde_json::json!({
                "id": row.id,
                "sync_type": row.sync_type,
                "content_id": row.content_id,
                "content_type": row.content_type,
                "sync_time": row.sync_time,
                "success": row.success,
                "error_message": row.error_message,
                "duration_ms": row.duration_ms
            })
        }).collect();

        Ok(history_json)
    }

    // Get sync history stats
    pub async fn get_sync_history_stats(&self) -> Result<JsonValue, Error> {
        // Get success/failure counts
        let counts = sqlx::query!(r#"
            SELECT success, COUNT(*) as count
            FROM sync_history
            GROUP BY success
        "#)
        .fetch_all(&self.db)
        .await?;

        // Get average duration
        let avg_duration = sqlx::query!(r#"
            SELECT AVG(duration_ms) as avg_duration
            FROM sync_history
            WHERE success = 1
        "#)
        .fetch_one(&self.db)
        .await?;

        // Get counts by sync type
        let type_counts = sqlx::query!(r#"
            SELECT sync_type, COUNT(*) as count
            FROM sync_history
            GROUP BY sync_type
        "#)
        .fetch_all(&self.db)
        .await?;

        // Build stats JSON
        let mut stats = serde_json::json!({
            "total": 0,
            "success": 0,
            "failure": 0,
            "avg_duration_ms": avg_duration.avg_duration.unwrap_or(0.0),
            "by_type": {}
        });

        // Update counts
        let mut total = 0;
        for row in counts {
            let count = row.count as u64;
            total += count;

            if row.success == 1 {
                stats["success"] = count.into();
            } else {
                stats["failure"] = count.into();
            }
        }
        stats["total"] = total.into();

        // Update type counts
        for row in type_counts {
            stats["by_type"][row.sync_type] = row.count.into();
        }

        Ok(stats)
    }

    // Clear sync errors
    pub async fn clear_sync_errors(&self) -> Result<(), Error> {
        // Reset error status for topics
        sqlx::query!(r#"
            UPDATE topic_mappings
            SET sync_status = 'PendingToCanvas'
            WHERE sync_status = 'Error'
        "#)
        .execute(&self.db)
        .await?;

        // Reset error status for posts
        sqlx::query!(r#"
            UPDATE post_mappings
            SET sync_status = 'PendingToCanvas'
            WHERE sync_status = 'Error'
        "#)
        .execute(&self.db)
        .await?;

        Ok(())
    }

    // Resolve a conflict
    pub async fn resolve_conflict(&self, conflict_id: &str, strategy: ConflictResolutionStrategy) -> Result<(), Error> {
        // Parse the conflict ID
        let mapping_id = match Uuid::parse_str(conflict_id) {
            Ok(id) => id,
            Err(_) => return Err(Error::InvalidId("Invalid conflict ID format".to_string())),
        };

        // Try to find topic mapping first
        let topic_mapping = TopicMapping::find(&self.db, mapping_id).await;

        if let Ok(mut mapping) = topic_mapping {
            // This is a topic conflict
            if mapping.sync_status != SyncStatus::Conflict {
                return Err(Error::InvalidState("Mapping is not in conflict state".to_string()));
            }

            // Get the topic
            let topic_id = mapping.local_topic_id.ok_or_else(|| {
                Error::InvalidState("Mapping has no local topic ID".to_string())
            })?;

            let topic = Topic::find(&self.db, topic_id).await?;

            // Get Canvas and Discourse versions
            let canvas_topic = if let Some(canvas_id) = &mapping.canvas_topic_id {
                self.canvas.fetch_canvas_topic(canvas_id).await?
            } else {
                return Err(Error::InvalidState("No Canvas topic ID".to_string()));
            };

            let discourse_topic = if let Some(discourse_id) = mapping.discourse_topic_id {
                self.discourse.fetch_discourse_topic(discourse_id).await?
            } else {
                return Err(Error::InvalidState("No Discourse topic ID".to_string()));
            };

            // Apply resolution strategy
            let (resolved_topic, sync_status) = match strategy {
                ConflictResolutionStrategy::PreferCanvas => {
                    // Convert Canvas topic to our model
                    let mut resolved = self.canvas.convert_canvas_topic_to_model(canvas_topic).await?;
                    resolved.id = topic.id; // Preserve our ID
                    (resolved, SyncStatus::PendingToDiscourse)
                },
                ConflictResolutionStrategy::PreferDiscourse => {
                    // Convert Discourse topic to our model
                    let mut resolved = self.discourse.convert_discourse_topic_to_model(discourse_topic).await?;
                    resolved.id = topic.id; // Preserve our ID
                    (resolved, SyncStatus::PendingToCanvas)
                },
                ConflictResolutionStrategy::PreferMostRecent => {
                    if canvas_topic.updated_at > discourse_topic.updated_at {
                        let mut resolved = self.canvas.convert_canvas_topic_to_model(canvas_topic).await?;
                        resolved.id = topic.id; // Preserve our ID
                        (resolved, SyncStatus::PendingToDiscourse)
                    } else {
                        let mut resolved = self.discourse.convert_discourse_topic_to_model(discourse_topic).await?;
                        resolved.id = topic.id; // Preserve our ID
                        (resolved, SyncStatus::PendingToCanvas)
                    }
                },
                ConflictResolutionStrategy::MergePreferCanvas => {
                    // For now, just prefer Canvas (implement actual merging later)
                    let mut resolved = self.canvas.convert_canvas_topic_to_model(canvas_topic).await?;
                    resolved.id = topic.id; // Preserve our ID
                    (resolved, SyncStatus::PendingToDiscourse)
                },
                ConflictResolutionStrategy::MergePreferDiscourse => {
                    // For now, just prefer Discourse (implement actual merging later)
                    let mut resolved = self.discourse.convert_discourse_topic_to_model(discourse_topic).await?;
                    resolved.id = topic.id; // Preserve our ID
                    (resolved, SyncStatus::PendingToCanvas)
                },
            };

            // Update the topic in our database
            resolved_topic.update(&self.db).await?;

            // Update the mapping status
            mapping.sync_status = sync_status;
            mapping.last_sync_at = Utc::now();
            mapping.update(&self.db).await?;

            // Record the resolution in history
            self.record_sync_history(
                "conflict_resolution",
                Some(&topic.id.to_string()),
                Some("topic"),
                true,
                None,
                0
            ).await?;

            return Ok(());
        }

        // If not a topic mapping, try post mapping
        let post_mapping = PostMapping::find(&self.db, mapping_id).await;

        if let Ok(mut mapping) = post_mapping {
            // This is a post conflict
            if mapping.sync_status != SyncStatus::Conflict {
                return Err(Error::InvalidState("Mapping is not in conflict state".to_string()));
            }

            // Get the post
            let post_id = mapping.local_post_id.ok_or_else(|| {
                Error::InvalidState("Mapping has no local post ID".to_string())
            })?;

            let post = Post::find(&self.db, post_id).await?;

            // Get Canvas and Discourse versions
            let canvas_post = if let Some(canvas_id) = &mapping.canvas_entry_id {
                self.canvas.fetch_canvas_entry(canvas_id).await?
            } else {
                return Err(Error::InvalidState("No Canvas entry ID".to_string()));
            };

            let discourse_post = if let Some(discourse_id) = mapping.discourse_post_id {
                self.discourse.fetch_discourse_post(discourse_id).await?
            } else {
                return Err(Error::InvalidState("No Discourse post ID".to_string()));
            };

            // Apply resolution strategy
            let (resolved_post, sync_status) = match strategy {
                ConflictResolutionStrategy::PreferCanvas => {
                    // Convert Canvas post to our model
                    let mut resolved = self.canvas.convert_canvas_entry_to_post(canvas_post, post.topic_id).await?;
                    resolved.id = post.id; // Preserve our ID
                    (resolved, SyncStatus::PendingToDiscourse)
                },
                ConflictResolutionStrategy::PreferDiscourse => {
                    // Convert Discourse post to our model
                    let mut resolved = self.discourse.convert_discourse_post_to_post(discourse_post, post.topic_id).await?;
                    resolved.id = post.id; // Preserve our ID
                    (resolved, SyncStatus::PendingToCanvas)
                },
                ConflictResolutionStrategy::PreferMostRecent => {
                    if canvas_post.updated_at > discourse_post.updated_at {
                        let mut resolved = self.canvas.convert_canvas_entry_to_post(canvas_post, post.topic_id).await?;
                        resolved.id = post.id; // Preserve our ID
                        (resolved, SyncStatus::PendingToDiscourse)
                    } else {
                        let mut resolved = self.discourse.convert_discourse_post_to_post(discourse_post, post.topic_id).await?;
                        resolved.id = post.id; // Preserve our ID
                        (resolved, SyncStatus::PendingToCanvas)
                    }
                },
                ConflictResolutionStrategy::MergePreferCanvas => {
                    // For now, just prefer Canvas (implement actual merging later)
                    let mut resolved = self.canvas.convert_canvas_entry_to_post(canvas_post, post.topic_id).await?;
                    resolved.id = post.id; // Preserve our ID
                    (resolved, SyncStatus::PendingToDiscourse)
                },
                ConflictResolutionStrategy::MergePreferDiscourse => {
                    // For now, just prefer Discourse (implement actual merging later)
                    let mut resolved = self.discourse.convert_discourse_post_to_post(discourse_post, post.topic_id).await?;
                    resolved.id = post.id; // Preserve our ID
                    (resolved, SyncStatus::PendingToCanvas)
                },
            };

            // Update the post in our database
            resolved_post.update(&self.db).await?;

            // Update the mapping status
            mapping.sync_status = sync_status;
            mapping.last_sync_at = Utc::now();
            mapping.update(&self.db).await?;

            // Record the resolution in history
            self.record_sync_history(
                "conflict_resolution",
                Some(&post.id.to_string()),
                Some("post"),
                true,
                None,
                0
            ).await?;

            return Ok(());
        }

        // If we get here, the conflict ID was not found
        Err(Error::NotFound(format!("Conflict with ID {} not found", conflict_id)))
    }

    /// Perform bidirectional sync for a topic
    pub async fn sync_topic_bidirectional(&self, local_topic_id: Uuid) -> Result<Topic, Error> {
        // Get the local topic
        let local_topic = Topic::find(&self.db, local_topic_id).await?;

        // Get Canvas and Discourse IDs
        let canvas_topic_id = local_topic.canvas_topic_id.clone();
        let discourse_topic_id = local_topic.discourse_topic_id;

        // Initialize result
        let mut result_topic = local_topic.clone();

        // If we have both Canvas and Discourse IDs, check for conflicts
        if let (Some(canvas_id), Some(discourse_id)) = (&canvas_topic_id, discourse_topic_id) {
            // 1. Fetch latest data from both systems
            let canvas_topic = self.canvas.sync_topic(canvas_id).await?;
            let discourse_topic = self.discourse.sync_topic(discourse_id).await?;

            // 2. Resolve any conflicts
            result_topic = self.conflict_resolver.resolve_topic_conflict(&canvas_topic, &discourse_topic)?;

            // 3. Update local storage
            result_topic.update(&self.db).await?;

            // 4. Push resolved version back to both systems to ensure consistency
            if canvas_topic.updated_at != result_topic.updated_at {
                self.canvas.push_topic_to_canvas(&result_topic).await?;
            }

            if discourse_topic.updated_at != result_topic.updated_at {
                self.discourse.push_topic_to_discourse(&result_topic).await?;
            }

            // 5. Update sync status
            let mapping = TopicMapping::find_by_local_id(&self.db, local_topic_id).await?;
            let mut updated_mapping = mapping.clone();
            updated_mapping.last_sync_at = Utc::now();
            updated_mapping.sync_status = SyncStatus::Synced;
            updated_mapping.update(&self.db).await?;

            // 6. Now sync all posts for this topic with the same approach
            let posts = Post::find_by_topic_id(&self.db, local_topic_id).await?;

            for post in posts {
                match (post.canvas_entry_id.as_ref(), post.discourse_post_id) {
                    (Some(canvas_id), Some(discourse_id)) => {
                        // Bidirectional sync for posts
                        let _ = self.sync_post_bidirectional(post.id).await;
                    },
                    (Some(canvas_id), None) => {
                        // One-way sync: Canvas to Discourse
                        let _ = self.sync_post_canvas_to_discourse(canvas_id, &updated_mapping).await;
                    },
                    (None, Some(discourse_id)) => {
                        // One-way sync: Discourse to Canvas
                        let _ = self.sync_post_discourse_to_canvas(discourse_id, &updated_mapping).await;
                    },
                    _ => {} // Local only, nothing to sync
                }
            }
        } else if let Some(canvas_id) = canvas_topic_id {
            // Only in Canvas, sync to Discourse
            let mapping = self.sync_topic_canvas_to_discourse(&canvas_id).await?;
            result_topic = Topic::find(&self.db, local_topic_id).await?; // Reload after sync
        } else if let Some(discourse_id) = discourse_topic_id {
            // Only in Discourse, sync to Canvas
            let mapping = self.sync_topic_discourse_to_canvas(discourse_id).await?;
            result_topic = Topic::find(&self.db, local_topic_id).await?; // Reload after sync
        }

        Ok(result_topic)
    }

    /// Perform bidirectional sync for a post
    pub async fn sync_post_bidirectional(&self, local_post_id: Uuid) -> Result<Post, Error> {
        // Get the local post
        let local_post = Post::find(&self.db, local_post_id).await?;

        // Get Canvas and Discourse IDs
        let canvas_entry_id = local_post.canvas_entry_id.clone();
        let discourse_post_id = local_post.discourse_post_id;

        // Initialize result
        let mut result_post = local_post.clone();

        // If we have both Canvas and Discourse IDs, check for conflicts
        if let (Some(canvas_id), Some(discourse_id)) = (&canvas_entry_id, discourse_post_id) {
            // 1. Fetch latest data from both systems
            let canvas_post = self.canvas.sync_post(canvas_id).await?;
            let discourse_post = self.discourse.sync_post(discourse_id).await?;

            // 2. Resolve any conflicts
            result_post = self.conflict_resolver.resolve_post_conflict(&canvas_post, &discourse_post)?;

            // 3. Update local storage
            result_post.update(&self.db).await?;

            // 4. Push resolved version back to both systems to ensure consistency
            if canvas_post.updated_at != result_post.updated_at {
                self.canvas.push_post_to_canvas(&result_post).await?;
            }

            if discourse_post.updated_at != result_post.updated_at {
                self.discourse.push_post_to_discourse(&result_post).await?;
            }

            // 5. Update sync status
            let mapping = PostMapping::find_by_local_id(&self.db, local_post_id).await?;
            let mut updated_mapping = mapping.clone();
            updated_mapping.last_sync_at = Utc::now();
            updated_mapping.sync_status = SyncStatus::Synced;
            updated_mapping.update(&self.db).await?;
        } else if let Some(canvas_id) = canvas_entry_id {
            // Only in Canvas, sync to Discourse
            // First, we need to get the topic mapping
            let topic = Topic::find(&self.db, local_post.topic_id).await?;
            let topic_mapping = TopicMapping::find_by_local_id(&self.db, topic.id).await?;

            let mapping = self.sync_post_canvas_to_discourse(&canvas_id, &topic_mapping).await?;
            result_post = Post::find(&self.db, local_post_id).await?; // Reload after sync
        } else if let Some(discourse_id) = discourse_post_id {
            // Only in Discourse, sync to Canvas
            // First, we need to get the topic mapping
            let topic = Topic::find(&self.db, local_post.topic_id).await?;
            let topic_mapping = TopicMapping::find_by_local_id(&self.db, topic.id).await?;

            let mapping = self.sync_post_discourse_to_canvas(discourse_id, &topic_mapping).await?;
            result_post = Post::find(&self.db, local_post_id).await?; // Reload after sync
        }

        Ok(result_post)
    }
}