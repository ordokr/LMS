use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use sqlx::FromRow;

use crate::db::DB;
use crate::error::Error;
use crate::models::user::user::User;
use crate::models::forum::post::Post;
use crate::models::forum::category::Category;
use crate::utils::date_utils::parse_date_string;

/// Topic model that harmonizes Canvas Discussion and Discourse Topic
/// 
/// Canvas Discussion fields:
///   - id: String
///   - title: String
///   - message: String
///   - assignment_id: String (optional)
///   - posted_at: DateTime
///   - delayed_post_at: DateTime (optional)
///   - locked: Boolean
/// 
/// Discourse Topic fields:
///   - id: Integer
///   - title: String
///   - category_id: Integer
///   - pinned: Boolean
///   - closed: Boolean
///   - created_at: DateTime
///   - last_posted_at: DateTime
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Topic {
    pub id: Uuid,
    pub category_id: Uuid,
    pub title: String,
    pub content: String,
    pub author_id: Uuid,
    pub is_pinned: bool,
    pub is_closed: bool,
    pub is_question: bool,    // Canvas-specific: marks topic as a Q&A discussion
    pub assignment_id: Option<Uuid>,  // Canvas-specific: optional linked assignment
    
    #[serde(default)]
    pub created_at: DateTime<Utc>,
    
    #[serde(default)]
    pub updated_at: DateTime<Utc>,
    
    #[serde(default)]
    pub last_post_at: Option<DateTime<Utc>>,
    
    #[serde(default)]
    pub publish_at: Option<DateTime<Utc>>, // Optional scheduled publish time
    
    pub read_status: bool,        // Has the current user read this topic
    pub view_count: i32,
    
    // Integration-specific fields
    pub canvas_discussion_id: Option<String>,
    pub discourse_topic_id: Option<i64>,
    pub sync_status: SyncStatus,
    
    #[sqlx(skip)]
    pub tags: Vec<String>,     // Topic tags
    
    #[sqlx(skip)]
    pub post_ids: Vec<Uuid>,   // IDs of posts in this topic (for efficient loading)
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SyncStatus {
    LocalOnly,
    SyncedWithCanvas,
    SyncedWithDiscourse,
    SyncedWithBoth,
    PendingSync,
    SyncError,
}

impl Topic {
    pub fn new(category_id: Uuid, author_id: Uuid, title: String, content: String) -> Self {
        let now = Utc::now();
        
        Topic {
            id: Uuid::new_v4(),
            category_id,
            title,
            content,
            author_id,
            is_pinned: false,
            is_closed: false,
            is_question: false,
            assignment_id: None,
            created_at: now,
            updated_at: now,
            last_post_at: Some(now),
            publish_at: Some(now),
            read_status: false,
            view_count: 0,
            canvas_discussion_id: None,
            discourse_topic_id: None,
            sync_status: SyncStatus::LocalOnly,
            tags: Vec::new(),
            post_ids: Vec::new(),
        }
    }
    
    /// Validate topic data
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        
        // Title shouldn't be empty
        if self.title.trim().is_empty() {
            errors.push("Topic title cannot be empty".to_string());
        }
        
        // Content shouldn't be empty
        if self.content.trim().is_empty() {
            errors.push("Topic content cannot be empty".to_string());
        }
        
        // Category ID is required
        if self.category_id == Uuid::nil() {
            errors.push("Category ID cannot be empty".to_string());
        }
        
        // Author ID is required
        if self.author_id == Uuid::nil() {
            errors.push("Author ID cannot be empty".to_string());
        }
        
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Get the author of this topic
    pub async fn author(&self, db: &DB) -> Result<User, Error> {
        User::find(db, self.author_id).await
    }
    
    /// Get the category this topic belongs to
    pub async fn category(&self, db: &DB) -> Result<Category, Error> {
        Category::find(db, self.category_id).await
    }
    
    /// Get posts in this topic
    pub async fn posts(&self, db: &DB) -> Result<Vec<Post>, Error> {
        Post::find_by_topic_id(db, self.id).await
    }
    
    /// Get the first (root) post of the topic
    pub async fn first_post(&self, db: &DB) -> Result<Post, Error> {
        // Get the earliest post in the topic
        let posts = sqlx::query_as::<_, Post>(
            "SELECT * FROM posts 
            WHERE topic_id = ? AND parent_id IS NULL
            ORDER BY created_at ASC LIMIT 1"
        )
        .bind(self.id)
        .fetch_all(&db.pool)
        .await?;
        
        if posts.is_empty() {
            // Create a post from the topic content if none exists
            let post = Post::new(self.id, self.author_id, self.content.clone());
            let _ = post.create(db).await?;
            Ok(post)
        } else {
            Ok(posts[0].clone())
        }
    }

    // Basic CRUD methods
    pub async fn find(db: &DB, id: Uuid) -> Result<Self, Error> {
        let mut topic = sqlx::query_as::<_, Self>(
            "SELECT * FROM topics WHERE id = ?"
        )
        .bind(id)
        .fetch_one(&db.pool)
        .await?;
        
        // Load tags
        topic.tags = sqlx::query_scalar(
            "SELECT t.name FROM topic_tags tt
            JOIN tags t ON tt.tag_id = t.id
            WHERE tt.topic_id = ?"
        )
        .bind(topic.id)
        .fetch_all(&db.pool)
        .await?;
        
        // Load post IDs for efficient loading
        topic.post_ids = sqlx::query_scalar(
            "SELECT id FROM posts WHERE topic_id = ? ORDER BY created_at ASC"
        )
        .bind(topic.id)
        .fetch_all(&db.pool)
        .await?;
        
        Ok(topic)
    }
    
    pub async fn find_by_category(db: &DB, category_id: Uuid) -> Result<Vec<Self>, Error> {
        let topics = sqlx::query_as::<_, Self>(
            "SELECT * FROM topics WHERE category_id = ? ORDER BY is_pinned DESC, last_post_at DESC"
        )
        .bind(category_id)
        .fetch_all(&db.pool)
        .await?;
        
        // Load related data for each topic
        let mut complete_topics = Vec::with_capacity(topics.len());
        
        for mut topic in topics {
            // Load tags
            topic.tags = sqlx::query_scalar(
                "SELECT t.name FROM topic_tags tt
                JOIN tags t ON tt.tag_id = t.id
                WHERE tt.topic_id = ?"
            )
            .bind(topic.id)
            .fetch_all(&db.pool)
            .await?;
            
            // Load post IDs
            topic.post_ids = sqlx::query_scalar(
                "SELECT id FROM posts WHERE topic_id = ? ORDER BY created_at ASC"
            )
            .bind(topic.id)
            .fetch_all(&db.pool)
            .await?;
            
            complete_topics.push(topic);
        }
        
        Ok(complete_topics)
    }
    
    pub async fn find_by_author(db: &DB, author_id: Uuid) -> Result<Vec<Self>, Error> {
        let topics = sqlx::query_as::<_, Self>(
            "SELECT * FROM topics WHERE author_id = ? ORDER BY created_at DESC"
        )
        .bind(author_id)
        .fetch_all(&db.pool)
        .await?;
        
        // Load related data for each topic
        let mut complete_topics = Vec::with_capacity(topics.len());
        
        for mut topic in topics {
            // Load tags
            topic.tags = sqlx::query_scalar(
                "SELECT t.name FROM topic_tags tt
                JOIN tags t ON tt.tag_id = t.id
                WHERE tt.topic_id = ?"
            )
            .bind(topic.id)
            .fetch_all(&db.pool)
            .await?;
            
            // Load post IDs
            topic.post_ids = sqlx::query_scalar(
                "SELECT id FROM posts WHERE topic_id = ? ORDER BY created_at ASC"
            )
            .bind(topic.id)
            .fetch_all(&db.pool)
            .await?;
            
            complete_topics.push(topic);
        }
        
        Ok(complete_topics)
    }
    
    pub async fn find_by_tag(db: &DB, tag_name: &str) -> Result<Vec<Self>, Error> {
        let topics = sqlx::query_as::<_, Self>(
            "SELECT t.* FROM topics t
            JOIN topic_tags tt ON t.id = tt.topic_id
            JOIN tags tag ON tt.tag_id = tag.id
            WHERE tag.name = ?
            ORDER BY t.is_pinned DESC, t.last_post_at DESC"
        )
        .bind(tag_name)
        .fetch_all(&db.pool)
        .await?;
        
        // Load related data for each topic
        let mut complete_topics = Vec::with_capacity(topics.len());
        
        for mut topic in topics {
            // Load tags
            topic.tags = sqlx::query_scalar(
                "SELECT t.name FROM topic_tags tt
                JOIN tags t ON tt.tag_id = t.id
                WHERE tt.topic_id = ?"
            )
            .bind(topic.id)
            .fetch_all(&db.pool)
            .await?;
            
            // Load post IDs
            topic.post_ids = sqlx::query_scalar(
                "SELECT id FROM posts WHERE topic_id = ? ORDER BY created_at ASC"
            )
            .bind(topic.id)
            .fetch_all(&db.pool)
            .await?;
            
            complete_topics.push(topic);
        }
        
        Ok(complete_topics)
    }
    
    pub async fn find_by_canvas_id(db: &DB, canvas_id: &str) -> Result<Self, Error> {
        let mut topic = sqlx::query_as::<_, Self>(
            "SELECT * FROM topics WHERE canvas_discussion_id = ?"
        )
        .bind(canvas_id)
        .fetch_one(&db.pool)
        .await?;
        
        // Load tags
        topic.tags = sqlx::query_scalar(
            "SELECT t.name FROM topic_tags tt
            JOIN tags t ON tt.tag_id = t.id
            WHERE tt.topic_id = ?"
        )
        .bind(topic.id)
        .fetch_all(&db.pool)
        .await?;
        
        // Load post IDs
        topic.post_ids = sqlx::query_scalar(
            "SELECT id FROM posts WHERE topic_id = ? ORDER BY created_at ASC"
        )
        .bind(topic.id)
        .fetch_all(&db.pool)
        .await?;
        
        Ok(topic)
    }
    
    pub async fn find_by_discourse_id(db: &DB, discourse_id: i64) -> Result<Self, Error> {
        let mut topic = sqlx::query_as::<_, Self>(
            "SELECT * FROM topics WHERE discourse_topic_id = ?"
        )
        .bind(discourse_id)
        .fetch_one(&db.pool)
        .await?;
        
        // Load tags
        topic.tags = sqlx::query_scalar(
            "SELECT t.name FROM topic_tags tt
            JOIN tags t ON tt.tag_id = t.id
            WHERE tt.topic_id = ?"
        )
        .bind(topic.id)
        .fetch_all(&db.pool)
        .await?;
        
        // Load post IDs
        topic.post_ids = sqlx::query_scalar(
            "SELECT id FROM posts WHERE topic_id = ? ORDER BY created_at ASC"
        )
        .bind(topic.id)
        .fetch_all(&db.pool)
        .await?;
        
        Ok(topic)
    }

    pub async fn create(&self, db: &DB) -> Result<Uuid, Error> {
        // Start a transaction
        let mut tx = db.pool.begin().await?;
        
        // Insert topic
        sqlx::query(
            "INSERT INTO topics 
            (id, category_id, title, content, author_id, is_pinned, is_closed,
            is_question, assignment_id, created_at, updated_at, last_post_at,
            publish_at, read_status, view_count, canvas_discussion_id,
            discourse_topic_id, sync_status)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(self.id)
        .bind(self.category_id)
        .bind(&self.title)
        .bind(&self.content)
        .bind(self.author_id)
        .bind(self.is_pinned)
        .bind(self.is_closed)
        .bind(self.is_question)
        .bind(self.assignment_id)
        .bind(self.created_at)
        .bind(self.updated_at)
        .bind(self.last_post_at)
        .bind(self.publish_at)
        .bind(self.read_status)
        .bind(self.view_count)
        .bind(&self.canvas_discussion_id)
        .bind(self.discourse_topic_id)
        .bind(self.sync_status as i32)
        .execute(&mut *tx)
        .await?;
        
        // Create the initial post from the topic content
        let post = Post {
            id: Uuid::new_v4(),
            topic_id: self.id,
            author_id: self.author_id,
            parent_id: None,
            content: self.content.clone(),
            html_content: None,
            created_at: self.created_at,
            updated_at: self.updated_at,
            likes: 0,
            is_solution: false,
            score: None,
            read_status: false,
            attachment_ids: Vec::new(),
            canvas_entry_id: None,
            discourse_post_id: None,
            sync_status: self.sync_status,
        };
        
        // Insert the post
        sqlx::query(
            "INSERT INTO posts 
            (id, topic_id, author_id, parent_id, content, html_content, 
            created_at, updated_at, likes, is_solution, score, read_status,
            canvas_entry_id, discourse_post_id, sync_status)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(post.id)
        .bind(post.topic_id)
        .bind(post.author_id)
        .bind(post.parent_id)
        .bind(&post.content)
        .bind(&post.html_content)
        .bind(post.created_at)
        .bind(post.updated_at)
        .bind(post.likes)
        .bind(post.is_solution)
        .bind(post.score)
        .bind(post.read_status)
        .bind(&post.canvas_entry_id)
        .bind(post.discourse_post_id)
        .bind(post.sync_status as i32)
        .execute(&mut *tx)
        .await?;
        
        // Add tags
        for tag in &self.tags {
            // Find or create the tag
            let tag_id: Uuid = match sqlx::query_scalar::<_, Uuid>("SELECT id FROM tags WHERE name = ?")
                .bind(tag)
                .fetch_optional(&mut *tx)
                .await?
            {
                Some(id) => id,
                None => {
                    let tag_id = Uuid::new_v4();
                    sqlx::query("INSERT INTO tags (id, name) VALUES (?, ?)")
                        .bind(tag_id)
                        .bind(tag)
                        .execute(&mut *tx)
                        .await?;
                    tag_id
                }
            };
            
            // Associate tag with topic
            sqlx::query("INSERT INTO topic_tags (topic_id, tag_id) VALUES (?, ?)")
                .bind(self.id)
                .bind(tag_id)
                .execute(&mut *tx)
                .await?;
        }
        
        // Commit transaction
        tx.commit().await?;
        
        Ok(self.id)
    }

    pub async fn update(&self, db: &DB) -> Result<(), Error> {
        // Start a transaction
        let mut tx = db.pool.begin().await?;
        
        // Update topic
        sqlx::query(
            "UPDATE topics SET
            category_id = ?, title = ?, content = ?, author_id = ?,
            is_pinned = ?, is_closed = ?, is_question = ?, assignment_id = ?,
            updated_at = ?, last_post_at = ?, publish_at = ?, read_status = ?,
            view_count = ?, canvas_discussion_id = ?, discourse_topic_id = ?,
            sync_status = ?
            WHERE id = ?"
        )
        .bind(self.category_id)
        .bind(&self.title)
        .bind(&self.content)
        .bind(self.author_id)
        .bind(self.is_pinned)
        .bind(self.is_closed)
        .bind(self.is_question)
        .bind(self.assignment_id)
        .bind(Utc::now()) // Update the updated_at timestamp
        .bind(self.last_post_at)
        .bind(self.publish_at)
        .bind(self.read_status)
        .bind(self.view_count)
        .bind(&self.canvas_discussion_id)
        .bind(self.discourse_topic_id)
        .bind(self.sync_status as i32)
        .bind(self.id)
        .execute(&mut *tx)
        .await?;
        
        // Update first post with the new content
        sqlx::query(
            "UPDATE posts SET
            content = ?, updated_at = ?
            WHERE topic_id = ? AND parent_id IS NULL
            ORDER BY created_at ASC LIMIT 1"
        )
        .bind(&self.content)
        .bind(Utc::now())
        .bind(self.id)
        .execute(&mut *tx)
        .await?;
        
        // Update tags (delete and reinsert)
        sqlx::query("DELETE FROM topic_tags WHERE topic_id = ?")
            .bind(self.id)
            .execute(&mut *tx)
            .await?;
            
        for tag in &self.tags {
            // Find or create the tag
            let tag_id: Uuid = match sqlx::query_scalar::<_, Uuid>("SELECT id FROM tags WHERE name = ?")
                .bind(tag)
                .fetch_optional(&mut *tx)
                .await?
            {
                Some(id) => id,
                None => {
                    let tag_id = Uuid::new_v4();
                    sqlx::query("INSERT INTO tags (id, name) VALUES (?, ?)")
                        .bind(tag_id)
                        .bind(tag)
                        .execute(&mut *tx)
                        .await?;
                    tag_id
                }
            };
            
            // Associate tag with topic
            sqlx::query("INSERT INTO topic_tags (topic_id, tag_id) VALUES (?, ?)")
                .bind(self.id)
                .bind(tag_id)
                .execute(&mut *tx)
                .await?;
        }
        
        // Commit transaction
        tx.commit().await?;
        
        Ok(())
    }
    
    pub async fn delete(db: &DB, id: Uuid) -> Result<(), Error> {
        // Start a transaction
        let mut tx = db.pool.begin().await?;
        
        // Delete post attachments
        sqlx::query(
            "DELETE FROM post_attachments WHERE post_id IN 
            (SELECT id FROM posts WHERE topic_id = ?)"
        )
        .bind(id)
        .execute(&mut *tx)
        .await?;
        
        // Delete posts
        sqlx::query("DELETE FROM posts WHERE topic_id = ?")
            .bind(id)
            .execute(&mut *tx)
            .await?;
            
        // Delete topic tags
        sqlx::query("DELETE FROM topic_tags WHERE topic_id = ?")
            .bind(id)
            .execute(&mut *tx)
            .await?;
            
        // Delete topic
        sqlx::query("DELETE FROM topics WHERE id = ?")
            .bind(id)
            .execute(&mut *tx)
            .await?;
            
        // Commit transaction
        tx.commit().await?;
        
        Ok(())
    }

    // Canvas API integration method
    pub fn from_canvas_api(
        canvas_discussion: &serde_json::Value,
        category_id: Uuid,
        author_map: &std::collections::HashMap<String, Uuid>
    ) -> Result<Self, String> {
        // Extract and validate required fields
        let title = canvas_discussion["title"]
            .as_str()
            .ok_or("Missing or invalid discussion title")?
            .to_string();
        
        let content = canvas_discussion["message"]
            .as_str()
            .ok_or("Missing or invalid discussion message")?
            .to_string();
            
        let user_id = canvas_discussion["user_id"]
            .as_str()
            .ok_or("Missing or invalid user ID")?;
            
        let author_id = author_map
            .get(user_id)
            .ok_or(format!("Unknown user ID: {}", user_id))?;
            
        // Extract optional fields
        let discussion_id = canvas_discussion["id"].as_str().map(String::from);
        
        let assignment_id = if let Some(assignment_id_str) = canvas_discussion["assignment_id"].as_str() {
            // In a real implementation, you'd look up the local UUID for this assignment ID
            None // Simplified for now
        } else {
            None
        };
        
        // Parse dates
        let posted_at = parse_date_string(canvas_discussion["posted_at"].as_str())
            .unwrap_or_else(Utc::now);
            
        let delayed_post_at = parse_date_string(canvas_discussion["delayed_post_at"].as_str());
        
        // Parse other fields
        let locked = canvas_discussion["locked"].as_bool().unwrap_or(false);
        let is_question = canvas_discussion["discussion_type"]
            .as_str()
            .map(|t| t == "threaded_question")
            .unwrap_or(false);
            
        Ok(Self {
            id: Uuid::new_v4(),
            category_id,
            title,
            content,
            author_id: *author_id,
            is_pinned: false,
            is_closed: locked,
            is_question,
            assignment_id,
            created_at: posted_at,
            updated_at: posted_at,
            last_post_at: Some(posted_at),
            publish_at: delayed_post_at,
            read_status: false,
            view_count: 0,
            canvas_discussion_id: discussion_id,
            discourse_topic_id: None,
            sync_status: SyncStatus::SyncedWithCanvas,
            tags: Vec::new(),
            post_ids: Vec::new(),
        })
    }
    
    // Discourse API integration method
    pub fn from_discourse_api(
        discourse_topic: &serde_json::Value,
        category_id: Uuid,
        author_map: &std::collections::HashMap<i64, Uuid>
    ) -> Result<Self, String> {
        // Extract and validate required fields
        let topic_id = discourse_topic["id"]
            .as_i64()
            .ok_or("Missing or invalid topic ID")?;
            
        let title = discourse_topic["title"]
            .as_str()
            .ok_or("Missing or invalid topic title")?
            .to_string();
            
        // Extract the first post's content
        let content = discourse_topic["post_stream"]["posts"][0]["raw"]
            .as_str()
            .ok_or("Missing or invalid topic content")?
            .to_string();
            
        let user_id = discourse_topic["post_stream"]["posts"][0]["user_id"]
            .as_i64()
            .ok_or("Missing or invalid user ID")?;
            
        let author_id = author_map
            .get(&user_id)
            .ok_or(format!("Unknown user ID: {}", user_id))?;
            
        // Parse dates
        let created_at = parse_date_string(discourse_topic["created_at"].as_str())
            .unwrap_or_else(Utc::now);
            
        let last_posted_at = parse_date_string(discourse_topic["last_posted_at"].as_str());
        
        // Parse other fields
        let is_pinned = discourse_topic["pinned"].as_bool().unwrap_or(false);
        let is_closed = discourse_topic["closed"].as_bool().unwrap_or(false);
        
        Ok(Self {
            id: Uuid::new_v4(),
            category_id,
            title,
            content,
            author_id: *author_id,
            is_pinned,
            is_closed,
            is_question: false,
            assignment_id: None,
            created_at,
            updated_at: created_at,
            last_post_at: last_posted_at,
            publish_at: None,
            read_status: false,
            view_count: 0,
            canvas_discussion_id: None,
            discourse_topic_id: Some(topic_id),
            sync_status: SyncStatus::SyncedWithDiscourse,
            tags: Vec::new(),
            post_ids: Vec::new(),
        })
    }
}