use async_trait::async_trait;
use sqlx::{Pool, Sqlite};
use chrono::Utc;
use std::collections::HashMap;
use crate::error::Error;
use crate::models::unified_models::{Topic, TopicStatus, TopicVisibility, TopicType};
use super::repository::Repository;
use super::topic_repository::TopicRepository;

/// SQLite implementation of the topic repository
pub struct SqliteTopicRepository {
    pool: Pool<Sqlite>,
}

impl SqliteTopicRepository {
    /// Create a new SQLite topic repository
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }
    
    /// Helper method to convert a row to a Topic
    async fn row_to_topic(&self, id: &str) -> Result<Option<Topic>, Error> {
        let topic_row = sqlx::query!(
            r#"
            SELECT 
                id, title, content, created_at, updated_at, course_id, category_id,
                group_id, author_id, assignment_id, status, visibility, topic_type,
                is_pinned, is_locked, allow_rating, require_initial_post, posted_at,
                last_reply_at, delayed_post_at, view_count, reply_count, participant_count,
                canvas_id, discourse_id, slug, tags, source_system, metadata
            FROM topics
            WHERE id = ?
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;
        
        if let Some(row) = topic_row {
            // Parse timestamps
            let created_at = chrono::DateTime::parse_from_rfc3339(&row.created_at)
                .map_err(|e| Error::ParseError(format!("Failed to parse created_at: {}", e)))?
                .with_timezone(&Utc);
                
            let updated_at = chrono::DateTime::parse_from_rfc3339(&row.updated_at)
                .map_err(|e| Error::ParseError(format!("Failed to parse updated_at: {}", e)))?
                .with_timezone(&Utc);
            
            // Parse optional dates
            let posted_at = if let Some(date_str) = row.posted_at {
                Some(chrono::DateTime::parse_from_rfc3339(&date_str)
                    .map_err(|e| Error::ParseError(format!("Failed to parse posted_at: {}", e)))?
                    .with_timezone(&Utc))
            } else {
                None
            };
            
            let last_reply_at = if let Some(date_str) = row.last_reply_at {
                Some(chrono::DateTime::parse_from_rfc3339(&date_str)
                    .map_err(|e| Error::ParseError(format!("Failed to parse last_reply_at: {}", e)))?
                    .with_timezone(&Utc))
            } else {
                None
            };
            
            let delayed_post_at = if let Some(date_str) = row.delayed_post_at {
                Some(chrono::DateTime::parse_from_rfc3339(&date_str)
                    .map_err(|e| Error::ParseError(format!("Failed to parse delayed_post_at: {}", e)))?
                    .with_timezone(&Utc))
            } else {
                None
            };
            
            // Parse status
            let status = TopicStatus::from(row.status.as_str());
            
            // Parse visibility
            let visibility = TopicVisibility::from(row.visibility.as_str());
            
            // Parse topic type
            let topic_type = TopicType::from(row.topic_type.as_str());
            
            // Parse tags
            let tags: Vec<String> = if let Some(tags_str) = row.tags {
                serde_json::from_str(&tags_str)
                    .map_err(|e| Error::ParseError(format!("Failed to parse tags: {}", e)))?
            } else {
                Vec::new()
            };
            
            // Parse metadata
            let metadata: HashMap<String, serde_json::Value> = if let Some(meta_str) = row.metadata {
                serde_json::from_str(&meta_str)
                    .map_err(|e| Error::ParseError(format!("Failed to parse metadata: {}", e)))?
            } else {
                HashMap::new()
            };
            
            // Create topic
            let topic = Topic {
                id: row.id,
                title: row.title,
                content: row.content,
                created_at,
                updated_at,
                course_id: row.course_id,
                category_id: row.category_id,
                group_id: row.group_id,
                author_id: row.author_id,
                assignment_id: row.assignment_id,
                status,
                visibility,
                topic_type,
                is_pinned: row.is_pinned != 0,
                is_locked: row.is_locked != 0,
                allow_rating: row.allow_rating != 0,
                require_initial_post: row.require_initial_post != 0,
                posted_at,
                last_reply_at,
                delayed_post_at,
                view_count: row.view_count,
                reply_count: row.reply_count,
                participant_count: row.participant_count,
                canvas_id: row.canvas_id,
                discourse_id: row.discourse_id,
                slug: row.slug,
                tags,
                source_system: row.source_system,
                metadata,
            };
            
            Ok(Some(topic))
        } else {
            Ok(None)
        }
    }
}

#[async_trait]
impl Repository<Topic, String> for SqliteTopicRepository {
    async fn find_by_id(&self, id: &String) -> Result<Option<Topic>, Error> {
        self.row_to_topic(id).await
    }
    
    async fn find_all(&self) -> Result<Vec<Topic>, Error> {
        let topic_ids = sqlx::query!(
            "SELECT id FROM topics"
        )
        .fetch_all(&self.pool)
        .await?;
        
        let mut topics = Vec::new();
        for row in topic_ids {
            if let Some(topic) = self.row_to_topic(&row.id).await? {
                topics.push(topic);
            }
        }
        
        Ok(topics)
    }
    
    async fn create(&self, topic: &Topic) -> Result<Topic, Error> {
        // Serialize tags
        let tags_json = serde_json::to_string(&topic.tags)
            .map_err(|e| Error::SerializationError(format!("Failed to serialize tags: {}", e)))?;
        
        // Serialize metadata
        let metadata_json = serde_json::to_string(&topic.metadata)
            .map_err(|e| Error::SerializationError(format!("Failed to serialize metadata: {}", e)))?;
        
        // Insert topic
        sqlx::query!(
            r#"
            INSERT INTO topics (
                id, title, content, created_at, updated_at, course_id, category_id,
                group_id, author_id, assignment_id, status, visibility, topic_type,
                is_pinned, is_locked, allow_rating, require_initial_post, posted_at,
                last_reply_at, delayed_post_at, view_count, reply_count, participant_count,
                canvas_id, discourse_id, slug, tags, source_system, metadata
            ) VALUES (
                ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?
            )
            "#,
            topic.id,
            topic.title,
            topic.content,
            topic.created_at.to_rfc3339(),
            topic.updated_at.to_rfc3339(),
            topic.course_id,
            topic.category_id,
            topic.group_id,
            topic.author_id,
            topic.assignment_id,
            topic.status.to_string(),
            topic.visibility.to_string(),
            topic.topic_type.to_string(),
            if topic.is_pinned { 1 } else { 0 },
            if topic.is_locked { 1 } else { 0 },
            if topic.allow_rating { 1 } else { 0 },
            if topic.require_initial_post { 1 } else { 0 },
            topic.posted_at.map(|dt| dt.to_rfc3339()),
            topic.last_reply_at.map(|dt| dt.to_rfc3339()),
            topic.delayed_post_at.map(|dt| dt.to_rfc3339()),
            topic.view_count,
            topic.reply_count,
            topic.participant_count,
            topic.canvas_id,
            topic.discourse_id,
            topic.slug,
            tags_json,
            topic.source_system,
            metadata_json
        )
        .execute(&self.pool)
        .await?;
        
        // Return the created topic
        Ok(topic.clone())
    }
    
    async fn update(&self, topic: &Topic) -> Result<Topic, Error> {
        // Serialize tags
        let tags_json = serde_json::to_string(&topic.tags)
            .map_err(|e| Error::SerializationError(format!("Failed to serialize tags: {}", e)))?;
        
        // Serialize metadata
        let metadata_json = serde_json::to_string(&topic.metadata)
            .map_err(|e| Error::SerializationError(format!("Failed to serialize metadata: {}", e)))?;
        
        // Update topic
        sqlx::query!(
            r#"
            UPDATE topics SET
                title = ?, content = ?, updated_at = ?, course_id = ?, category_id = ?,
                group_id = ?, author_id = ?, assignment_id = ?, status = ?, visibility = ?,
                topic_type = ?, is_pinned = ?, is_locked = ?, allow_rating = ?,
                require_initial_post = ?, posted_at = ?, last_reply_at = ?, delayed_post_at = ?,
                view_count = ?, reply_count = ?, participant_count = ?, canvas_id = ?,
                discourse_id = ?, slug = ?, tags = ?, source_system = ?, metadata = ?
            WHERE id = ?
            "#,
            topic.title,
            topic.content,
            topic.updated_at.to_rfc3339(),
            topic.course_id,
            topic.category_id,
            topic.group_id,
            topic.author_id,
            topic.assignment_id,
            topic.status.to_string(),
            topic.visibility.to_string(),
            topic.topic_type.to_string(),
            if topic.is_pinned { 1 } else { 0 },
            if topic.is_locked { 1 } else { 0 },
            if topic.allow_rating { 1 } else { 0 },
            if topic.require_initial_post { 1 } else { 0 },
            topic.posted_at.map(|dt| dt.to_rfc3339()),
            topic.last_reply_at.map(|dt| dt.to_rfc3339()),
            topic.delayed_post_at.map(|dt| dt.to_rfc3339()),
            topic.view_count,
            topic.reply_count,
            topic.participant_count,
            topic.canvas_id,
            topic.discourse_id,
            topic.slug,
            tags_json,
            topic.source_system,
            metadata_json,
            topic.id
        )
        .execute(&self.pool)
        .await?;
        
        // Return the updated topic
        Ok(topic.clone())
    }
    
    async fn delete(&self, id: &String) -> Result<(), Error> {
        sqlx::query!(
            "DELETE FROM topics WHERE id = ?",
            id
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    async fn count(&self) -> Result<i64, Error> {
        let result = sqlx::query!("SELECT COUNT(*) as count FROM topics")
            .fetch_one(&self.pool)
            .await?;
            
        Ok(result.count)
    }
}

#[async_trait]
impl TopicRepository for SqliteTopicRepository {
    async fn find_by_course_id(&self, course_id: &str) -> Result<Vec<Topic>, Error> {
        let topic_rows = sqlx::query!(
            "SELECT id FROM topics WHERE course_id = ? ORDER BY is_pinned DESC, posted_at DESC",
            course_id
        )
        .fetch_all(&self.pool)
        .await?;
        
        let mut topics = Vec::new();
        for row in topic_rows {
            if let Some(topic) = self.row_to_topic(&row.id).await? {
                topics.push(topic);
            }
        }
        
        Ok(topics)
    }
    
    async fn find_by_course_id_and_status(&self, course_id: &str, status: TopicStatus) -> Result<Vec<Topic>, Error> {
        let status_str = status.to_string();
        
        let topic_rows = sqlx::query!(
            "SELECT id FROM topics WHERE course_id = ? AND status = ? ORDER BY is_pinned DESC, posted_at DESC",
            course_id,
            status_str
        )
        .fetch_all(&self.pool)
        .await?;
        
        let mut topics = Vec::new();
        for row in topic_rows {
            if let Some(topic) = self.row_to_topic(&row.id).await? {
                topics.push(topic);
            }
        }
        
        Ok(topics)
    }
    
    async fn find_by_group_id(&self, group_id: &str) -> Result<Vec<Topic>, Error> {
        let topic_rows = sqlx::query!(
            "SELECT id FROM topics WHERE group_id = ? ORDER BY is_pinned DESC, posted_at DESC",
            group_id
        )
        .fetch_all(&self.pool)
        .await?;
        
        let mut topics = Vec::new();
        for row in topic_rows {
            if let Some(topic) = self.row_to_topic(&row.id).await? {
                topics.push(topic);
            }
        }
        
        Ok(topics)
    }
    
    async fn find_by_category_id(&self, category_id: &str) -> Result<Vec<Topic>, Error> {
        let topic_rows = sqlx::query!(
            "SELECT id FROM topics WHERE category_id = ? ORDER BY is_pinned DESC, posted_at DESC",
            category_id
        )
        .fetch_all(&self.pool)
        .await?;
        
        let mut topics = Vec::new();
        for row in topic_rows {
            if let Some(topic) = self.row_to_topic(&row.id).await? {
                topics.push(topic);
            }
        }
        
        Ok(topics)
    }
    
    async fn find_by_author_id(&self, author_id: &str) -> Result<Vec<Topic>, Error> {
        let topic_rows = sqlx::query!(
            "SELECT id FROM topics WHERE author_id = ? ORDER BY posted_at DESC",
            author_id
        )
        .fetch_all(&self.pool)
        .await?;
        
        let mut topics = Vec::new();
        for row in topic_rows {
            if let Some(topic) = self.row_to_topic(&row.id).await? {
                topics.push(topic);
            }
        }
        
        Ok(topics)
    }
    
    async fn find_by_assignment_id(&self, assignment_id: &str) -> Result<Option<Topic>, Error> {
        let topic_row = sqlx::query!(
            "SELECT id FROM topics WHERE assignment_id = ?",
            assignment_id
        )
        .fetch_optional(&self.pool)
        .await?;
        
        if let Some(row) = topic_row {
            self.row_to_topic(&row.id).await
        } else {
            Ok(None)
        }
    }
    
    async fn find_by_canvas_id(&self, canvas_id: &str) -> Result<Option<Topic>, Error> {
        let topic_row = sqlx::query!(
            "SELECT id FROM topics WHERE canvas_id = ?",
            canvas_id
        )
        .fetch_optional(&self.pool)
        .await?;
        
        if let Some(row) = topic_row {
            self.row_to_topic(&row.id).await
        } else {
            Ok(None)
        }
    }
    
    async fn find_by_discourse_id(&self, discourse_id: &str) -> Result<Option<Topic>, Error> {
        let topic_row = sqlx::query!(
            "SELECT id FROM topics WHERE discourse_id = ?",
            discourse_id
        )
        .fetch_optional(&self.pool)
        .await?;
        
        if let Some(row) = topic_row {
            self.row_to_topic(&row.id).await
        } else {
            Ok(None)
        }
    }
    
    async fn find_by_type(&self, topic_type: TopicType) -> Result<Vec<Topic>, Error> {
        let topic_type_str = topic_type.to_string();
        
        let topic_rows = sqlx::query!(
            "SELECT id FROM topics WHERE topic_type = ? ORDER BY posted_at DESC",
            topic_type_str
        )
        .fetch_all(&self.pool)
        .await?;
        
        let mut topics = Vec::new();
        for row in topic_rows {
            if let Some(topic) = self.row_to_topic(&row.id).await? {
                topics.push(topic);
            }
        }
        
        Ok(topics)
    }
    
    async fn find_by_tag(&self, tag: &str) -> Result<Vec<Topic>, Error> {
        // This is a bit tricky with SQLite since we're storing tags as JSON
        // We'll fetch all topics and filter in memory
        let topic_rows = sqlx::query!(
            "SELECT id, tags FROM topics"
        )
        .fetch_all(&self.pool)
        .await?;
        
        let mut matching_ids = Vec::new();
        
        for row in topic_rows {
            if let Some(tags_json) = row.tags {
                if let Ok(tags) = serde_json::from_str::<Vec<String>>(&tags_json) {
                    if tags.contains(&tag.to_string()) {
                        matching_ids.push(row.id);
                    }
                }
            }
        }
        
        let mut topics = Vec::new();
        for id in matching_ids {
            if let Some(topic) = self.row_to_topic(&id).await? {
                topics.push(topic);
            }
        }
        
        Ok(topics)
    }
    
    async fn find_pinned_by_course_id(&self, course_id: &str) -> Result<Vec<Topic>, Error> {
        let topic_rows = sqlx::query!(
            "SELECT id FROM topics WHERE course_id = ? AND is_pinned = 1 ORDER BY posted_at DESC",
            course_id
        )
        .fetch_all(&self.pool)
        .await?;
        
        let mut topics = Vec::new();
        for row in topic_rows {
            if let Some(topic) = self.row_to_topic(&row.id).await? {
                topics.push(topic);
            }
        }
        
        Ok(topics)
    }
    
    async fn find_recent_by_course_id(&self, course_id: &str, limit: i64) -> Result<Vec<Topic>, Error> {
        let topic_rows = sqlx::query!(
            "SELECT id FROM topics WHERE course_id = ? AND status != 'deleted' ORDER BY posted_at DESC LIMIT ?",
            course_id,
            limit
        )
        .fetch_all(&self.pool)
        .await?;
        
        let mut topics = Vec::new();
        for row in topic_rows {
            if let Some(topic) = self.row_to_topic(&row.id).await? {
                topics.push(topic);
            }
        }
        
        Ok(topics)
    }
    
    async fn open(&self, id: &str) -> Result<Topic, Error> {
        // Get the topic
        let mut topic = self.find_by_id(&id.to_string()).await?
            .ok_or_else(|| Error::NotFound(format!("Topic with ID {} not found", id)))?;
        
        // Open the topic
        topic.open();
        
        // Update the topic
        self.update(&topic).await
    }
    
    async fn close(&self, id: &str) -> Result<Topic, Error> {
        // Get the topic
        let mut topic = self.find_by_id(&id.to_string()).await?
            .ok_or_else(|| Error::NotFound(format!("Topic with ID {} not found", id)))?;
        
        // Close the topic
        topic.close();
        
        // Update the topic
        self.update(&topic).await
    }
    
    async fn archive(&self, id: &str) -> Result<Topic, Error> {
        // Get the topic
        let mut topic = self.find_by_id(&id.to_string()).await?
            .ok_or_else(|| Error::NotFound(format!("Topic with ID {} not found", id)))?;
        
        // Archive the topic
        topic.archive();
        
        // Update the topic
        self.update(&topic).await
    }
    
    async fn delete_topic(&self, id: &str) -> Result<(), Error> {
        // Get the topic
        let mut topic = self.find_by_id(&id.to_string()).await?
            .ok_or_else(|| Error::NotFound(format!("Topic with ID {} not found", id)))?;
        
        // Mark the topic as deleted
        topic.delete();
        
        // Update the topic
        self.update(&topic).await?;
        
        Ok(())
    }
    
    async fn pin(&self, id: &str) -> Result<Topic, Error> {
        // Get the topic
        let mut topic = self.find_by_id(&id.to_string()).await?
            .ok_or_else(|| Error::NotFound(format!("Topic with ID {} not found", id)))?;
        
        // Pin the topic
        topic.pin();
        
        // Update the topic
        self.update(&topic).await
    }
    
    async fn unpin(&self, id: &str) -> Result<Topic, Error> {
        // Get the topic
        let mut topic = self.find_by_id(&id.to_string()).await?
            .ok_or_else(|| Error::NotFound(format!("Topic with ID {} not found", id)))?;
        
        // Unpin the topic
        topic.unpin();
        
        // Update the topic
        self.update(&topic).await
    }
    
    async fn add_tag(&self, id: &str, tag: &str) -> Result<Topic, Error> {
        // Get the topic
        let mut topic = self.find_by_id(&id.to_string()).await?
            .ok_or_else(|| Error::NotFound(format!("Topic with ID {} not found", id)))?;
        
        // Add the tag
        topic.add_tag(tag);
        
        // Update the topic
        self.update(&topic).await
    }
    
    async fn remove_tag(&self, id: &str, tag: &str) -> Result<Topic, Error> {
        // Get the topic
        let mut topic = self.find_by_id(&id.to_string()).await?
            .ok_or_else(|| Error::NotFound(format!("Topic with ID {} not found", id)))?;
        
        // Remove the tag
        topic.remove_tag(tag);
        
        // Update the topic
        self.update(&topic).await
    }
    
    async fn increment_view_count(&self, id: &str) -> Result<Topic, Error> {
        // Get the topic
        let mut topic = self.find_by_id(&id.to_string()).await?
            .ok_or_else(|| Error::NotFound(format!("Topic with ID {} not found", id)))?;
        
        // Increment the view count
        topic.increment_view_count();
        
        // Update the topic
        self.update(&topic).await
    }
    
    async fn increment_reply_count(&self, id: &str) -> Result<Topic, Error> {
        // Get the topic
        let mut topic = self.find_by_id(&id.to_string()).await?
            .ok_or_else(|| Error::NotFound(format!("Topic with ID {} not found", id)))?;
        
        // Increment the reply count
        topic.increment_reply_count();
        
        // Update the topic
        self.update(&topic).await
    }
}
