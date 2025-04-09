use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use sqlx::FromRow;

use crate::db::DB;
use crate::error::Error;
use crate::models::user::user::User;
use crate::models::forum::topic::Topic;
use crate::utils::date_utils::parse_date_string;

/// Post model that harmonizes Canvas DiscussionEntry and Discourse Post
/// 
/// Canvas fields:
///   - id: String
///   - user_id: String
///   - message: String
///   - created_at: DateTime
///   - updated_at: DateTime
///   - parent_id: String (optional)
/// 
/// Discourse fields:
///   - id: Integer
///   - post_number: Integer
///   - user_id: Integer
///   - raw: String (content)
///   - cooked: String (HTML content)
///   - created_at: DateTime
///   - updated_at: DateTime
///   - reply_to_post_number: Integer (optional)
///   - quote_count: Integer
///   - incoming_link_count: Integer
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Post {
    pub id: Uuid,
    pub topic_id: Uuid,
    pub author_id: Uuid,
    pub parent_id: Option<Uuid>,  // For threaded replies
    pub content: String,          // Raw content
    pub html_content: Option<String>, // Rendered HTML content
    
    #[serde(default)]
    pub created_at: DateTime<Utc>,
    
    #[serde(default)]
    pub updated_at: DateTime<Utc>,
    
    pub likes: i32,
    pub is_solution: bool,        // Marks post as solution to a question
    pub score: Option<f32>,       // Canvas-specific: score if this is a graded discussion
    pub read_status: bool,        // Has the current user read this post
    
    #[sqlx(skip)]
    pub attachment_ids: Vec<Uuid>, // Attachments to this post
    
    // Integration-specific fields
    pub canvas_entry_id: Option<String>, // Original Canvas ID if imported
    pub discourse_post_id: Option<i64>,  // Original Discourse ID if imported
    pub sync_status: SyncStatus,
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

// Canvas discussion entry representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanvasDiscussionEntry {
    pub id: String,
    pub user_id: String,
    pub message: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub parent_id: Option<String>,
}

// Discourse post representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoursePost {
    pub id: i64,
    pub post_number: i64,
    pub user_id: i64,
    pub raw: String,
    pub cooked: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub reply_to_post_number: Option<i64>,
    pub reply_to_post_id: Option<i64>,
    pub quote_count: Option<i64>,
    pub incoming_link_count: Option<i64>,
}

impl Post {
    pub fn new(topic_id: Uuid, author_id: Uuid, content: String) -> Self {
        let now = Utc::now();
        
        Post {
            id: Uuid::new_v4(),
            topic_id,
            author_id,
            parent_id: None,
            content,
            html_content: None,
            created_at: now,
            updated_at: now,
            likes: 0,
            is_solution: false,
            score: None,
            read_status: false,
            attachment_ids: Vec::new(),
            canvas_entry_id: None,
            discourse_post_id: None,
            sync_status: SyncStatus::LocalOnly,
        }
    }

    /// Get the author of this post
    pub async fn author(&self, db: &DB) -> Result<User, Error> {
        User::find(db, self.author_id).await
    }
    
    /// Get the topic this post belongs to
    pub async fn topic(&self, db: &DB) -> Result<Topic, Error> {
        Topic::find(db, self.topic_id).await
    }
    
    /// Get the parent post if this is a reply
    pub async fn parent(&self, db: &DB) -> Result<Option<Post>, Error> {
        if let Some(parent_id) = self.parent_id {
            Ok(Some(Post::find(db, parent_id).await?))
        } else {
            Ok(None)
        }
    }
    
    /// Get replies to this post
    pub async fn replies(&self, db: &DB) -> Result<Vec<Post>, Error> {
        Post::find_by_parent_id(db, self.id).await
    }

    // Basic CRUD methods
    pub async fn find(db: &DB, id: Uuid) -> Result<Self, Error> {
        let mut post = sqlx::query_as::<_, Self>(
            "SELECT * FROM posts WHERE id = ?"
        )
        .bind(id)
        .fetch_one(&db.pool)
        .await?;
        
        // Load attachments
        post.attachment_ids = sqlx::query_scalar(
            "SELECT attachment_id FROM post_attachments WHERE post_id = ?"
        )
        .bind(post.id)
        .fetch_all(&db.pool)
        .await?;
        
        Ok(post)
    }
    
    pub async fn find_by_author_id(db: &DB, author_id: Uuid) -> Result<Vec<Self>, Error> {
        let posts = sqlx::query_as::<_, Self>(
            "SELECT * FROM posts WHERE author_id = ? ORDER BY created_at DESC"
        )
        .bind(author_id)
        .fetch_all(&db.pool)
        .await?;
        
        // Load attachments for each post
        let mut complete_posts = Vec::with_capacity(posts.len());
        
        for mut post in posts {
            post.attachment_ids = sqlx::query_scalar(
                "SELECT attachment_id FROM post_attachments WHERE post_id = ?"
            )
            .bind(post.id)
            .fetch_all(&db.pool)
            .await?;
            
            complete_posts.push(post);
        }
        
        Ok(complete_posts)
    }
    
    pub async fn find_by_parent_id(db: &DB, parent_id: Uuid) -> Result<Vec<Self>, Error> {
        let posts = sqlx::query_as::<_, Self>(
            "SELECT * FROM posts WHERE parent_id = ? ORDER BY created_at ASC"
        )
        .bind(parent_id)
        .fetch_all(&db.pool)
        .await?;
        
        // Load attachments for each post
        let mut complete_posts = Vec::with_capacity(posts.len());
        
        for mut post in posts {
            post.attachment_ids = sqlx::query_scalar(
                "SELECT attachment_id FROM post_attachments WHERE post_id = ?"
            )
            .bind(post.id)
            .fetch_all(&db.pool)
            .await?;
            
            complete_posts.push(post);
        }
        
        Ok(complete_posts)
    }

    pub async fn find_by_topic_id(db: &DB, topic_id: Uuid) -> Result<Vec<Self>, Error> {
        let posts = sqlx::query_as::<_, Self>(
            "SELECT * FROM posts WHERE topic_id = ? ORDER BY created_at ASC"
        )
        .bind(topic_id)
        .fetch_all(&db.pool)
        .await?;
        
        // Load attachments for each post
        let mut complete_posts = Vec::with_capacity(posts.len());
        
        for mut post in posts {
            post.attachment_ids = sqlx::query_scalar(
                "SELECT attachment_id FROM post_attachments WHERE post_id = ?"
            )
            .bind(post.id)
            .fetch_all(&db.pool)
            .await?;
            
            complete_posts.push(post);
        }
        
        Ok(complete_posts)
    }

    pub async fn find_by_canvas_id(db: &DB, canvas_id: &str) -> Result<Self, Error> {
        let mut post = sqlx::query_as::<_, Self>(
            "SELECT * FROM posts WHERE canvas_entry_id = ?"
        )
        .bind(canvas_id)
        .fetch_one(&db.pool)
        .await?;
        
        // Load attachments
        post.attachment_ids = sqlx::query_scalar(
            "SELECT attachment_id FROM post_attachments WHERE post_id = ?"
        )
        .bind(post.id)
        .fetch_all(&db.pool)
        .await?;
        
        Ok(post)
    }

    pub async fn find_by_discourse_id(db: &DB, discourse_id: i64) -> Result<Self, Error> {
        let mut post = sqlx::query_as::<_, Self>(
            "SELECT * FROM posts WHERE discourse_post_id = ?"
        )
        .bind(discourse_id)
        .fetch_one(&db.pool)
        .await?;
        
        // Load attachments
        post.attachment_ids = sqlx::query_scalar(
            "SELECT attachment_id FROM post_attachments WHERE post_id = ?"
        )
        .bind(post.id)
        .fetch_all(&db.pool)
        .await?;
        
        Ok(post)
    }

    pub async fn create(&self, db: &DB) -> Result<Uuid, Error> {
        // Start a transaction
        let mut tx = db.pool.begin().await?;
        
        sqlx::query(
            "INSERT INTO posts 
            (id, topic_id, author_id, parent_id, content, html_content, 
            created_at, updated_at, likes, is_solution, score, read_status,
            canvas_entry_id, discourse_post_id, sync_status)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(self.id)
        .bind(self.topic_id)
        .bind(self.author_id)
        .bind(self.parent_id)
        .bind(&self.content)
        .bind(&self.html_content)
        .bind(self.created_at)
        .bind(self.updated_at)
        .bind(self.likes)
        .bind(self.is_solution)
        .bind(self.score)
        .bind(self.read_status)
        .bind(&self.canvas_entry_id)
        .bind(self.discourse_post_id)
        .bind(self.sync_status as i32)
        .execute(&mut *tx)
        .await?;
        
        // Add attachments if any
        for attachment_id in &self.attachment_ids {
            sqlx::query(
                "INSERT INTO post_attachments (post_id, attachment_id) VALUES (?, ?)"
            )
            .bind(self.id)
            .bind(attachment_id)
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
        
        sqlx::query(
            "UPDATE posts SET
            topic_id = ?, author_id = ?, parent_id = ?, content = ?,
            html_content = ?, updated_at = ?, likes = ?, is_solution = ?,
            score = ?, read_status = ?, canvas_entry_id = ?,
            discourse_post_id = ?, sync_status = ?
            WHERE id = ?"
        )
        .bind(self.topic_id)
        .bind(self.author_id)
        .bind(self.parent_id)
        .bind(&self.content)
        .bind(&self.html_content)
        .bind(Utc::now()) // Update the updated_at timestamp
        .bind(self.likes)
        .bind(self.is_solution)
        .bind(self.score)
        .bind(self.read_status)
        .bind(&self.canvas_entry_id)
        .bind(self.discourse_post_id)
        .bind(self.sync_status as i32)
        .bind(self.id)
        .execute(&mut *tx)
        .await?;
        
        // Update attachments (delete and reinsert)
        sqlx::query("DELETE FROM post_attachments WHERE post_id = ?")
            .bind(self.id)
            .execute(&mut *tx)
            .await?;
            
        for attachment_id in &self.attachment_ids {
            sqlx::query(
                "INSERT INTO post_attachments (post_id, attachment_id) VALUES (?, ?)"
            )
            .bind(self.id)
            .bind(attachment_id)
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
        
        // Delete attachments
        sqlx::query("DELETE FROM post_attachments WHERE post_id = ?")
            .bind(id)
            .execute(&mut *tx)
            .await?;
            
        // Delete post
        sqlx::query("DELETE FROM posts WHERE id = ?")
            .bind(id)
            .execute(&mut *tx)
            .await?;
            
        // Commit transaction
        tx.commit().await?;
        
        Ok(())
    }

    /// Validate post data
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        
        // Content shouldn't be empty
        if self.content.trim().is_empty() {
            errors.push("Post content cannot be empty".to_string());
        }
        
        // Topic ID is required
        if self.topic_id == Uuid::nil() {
            errors.push("Topic ID cannot be empty".to_string());
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
    
    /// Convert from Canvas discussion entry
    pub fn from_canvas_api(
        canvas_entry: &serde_json::Value, 
        topic_id: Uuid,
        author_map: &std::collections::HashMap<String, Uuid>
    ) -> Result<Self, String> {
        // Extract and validate required fields
        let entry_id = canvas_entry["id"]
            .as_str()
            .ok_or("Missing or invalid entry ID")?;
        
        let user_id = canvas_entry["user_id"]
            .as_str()
            .ok_or("Missing or invalid user ID")?;
        
        let author_id = author_map
            .get(user_id)
            .ok_or(format!("Unknown user ID: {}", user_id))?;
        
        let content = canvas_entry["message"]
            .as_str()
            .ok_or("Missing or invalid message")?
            .to_string();
        
        // Extract optional fields
        let parent_id = if let Some(parent_id_str) = canvas_entry["parent_id"].as_str() {
            // In a real implementation, you'd look up the local UUID for this parent ID
            None // Simplified for now
        } else {
            None
        };
        
        // Parse dates
        let created_at = parse_date_string(canvas_entry["created_at"].as_str())
            .unwrap_or_else(Utc::now);
            
        let updated_at = parse_date_string(canvas_entry["updated_at"].as_str())
            .unwrap_or_else(|| created_at);
        
        // Parse other fields
        let html_content = canvas_entry["message"].as_str().map(String::from);
        let score = canvas_entry["score"].as_f64().map(|s| s as f32);
        
        Ok(Self {
            id: Uuid::new_v4(),
            topic_id,
            author_id: *author_id,
            parent_id,
            content,
            html_content,
            created_at,
            updated_at,
            likes: 0,
            is_solution: false,
            score,
            read_status: false,
            attachment_ids: Vec::new(),
            canvas_entry_id: Some(entry_id.to_string()),
            discourse_post_id: None,
            sync_status: SyncStatus::SyncedWithCanvas,
        })
    }
    
    // Discourse API integration method
    pub fn from_discourse_api(
        discourse_post: &serde_json::Value,
        topic_id: Uuid,
        author_map: &std::collections::HashMap<i64, Uuid>
    ) -> Result<Self, String> {
        // Extract and validate required fields
        let post_id = discourse_post["id"]
            .as_i64()
            .ok_or("Missing or invalid post ID")?;
        
        let user_id = discourse_post["user_id"]
            .as_i64()
            .ok_or("Missing or invalid user ID")?;
        
        let author_id = author_map
            .get(&user_id)
            .ok_or(format!("Unknown user ID: {}", user_id))?;
        
        let content = discourse_post["raw"]
            .as_str()
            .ok_or("Missing or invalid post content")?
            .to_string();
            
        // Extract optional fields
        let html_content = discourse_post["cooked"].as_str().map(String::from);
        
        let parent_id = if let Some(reply_id) = discourse_post["reply_to_post_id"].as_i64() {
            // In a real implementation, you'd look up the local UUID for this parent ID
            None // Simplified for now
        } else {
            None
        };
        
        // Parse dates
        let created_at = parse_date_string(discourse_post["created_at"].as_str())
            .unwrap_or_else(Utc::now);
            
        let updated_at = parse_date_string(discourse_post["updated_at"].as_str())
            .unwrap_or_else(|| created_at);
        
        // Parse other fields
        let likes = discourse_post["like_count"].as_i64().unwrap_or(0) as i32;
        let is_solution = discourse_post["accepted_answer"].as_bool().unwrap_or(false);
        
        Ok(Self {
            id: Uuid::new_v4(),
            topic_id,
            author_id: *author_id,
            parent_id,
            content,
            html_content,
            created_at,
            updated_at,
            likes,
            is_solution,
            score: None,
            read_status: false,
            attachment_ids: Vec::new(),
            canvas_entry_id: None,
            discourse_post_id: Some(post_id),
            sync_status: SyncStatus::SyncedWithDiscourse,
        })
    }
}

// Implementation of trait conversions
impl FromCanvas for Post {
    fn from_canvas(canvas_entry: &CanvasDiscussionEntry, topic_id: Uuid, author_id: Uuid) -> Self {
        let now = Utc::now();
        
        // Parse dates
        let created_at = parse_date_string(canvas_entry.created_at.as_deref())
            .unwrap_or_else(|| now);
            
        let updated_at = parse_date_string(canvas_entry.updated_at.as_deref())
            .unwrap_or_else(|| created_at);
        
        // Create post
        Self {
            id: Uuid::new_v4(),
            topic_id,
            author_id,
            parent_id: None, // Would need to map parent IDs separately
            content: canvas_entry.message.clone(),
            html_content: Some(canvas_entry.message.clone()),
            created_at,
            updated_at,
            likes: 0,
            is_solution: false,
            score: None,
            read_status: false,
            attachment_ids: Vec::new(),
            canvas_entry_id: Some(canvas_entry.id.clone()),
            discourse_post_id: None,
            sync_status: SyncStatus::SyncedWithCanvas,
        }
    }
}

impl FromDiscourse for Post {
    fn from_discourse(discourse_post: &DiscoursePost, topic_id: Uuid, author_id: Uuid) -> Self {
        let now = Utc::now();
        
        // Parse dates
        let created_at = parse_date_string(discourse_post.created_at.as_deref())
            .unwrap_or_else(|| now);
            
        let updated_at = parse_date_string(discourse_post.updated_at.as_deref())
            .unwrap_or_else(|| created_at);
        
        // Create post
        Self {
            id: Uuid::new_v4(),
            topic_id,
            author_id,
            parent_id: None, // Would need to map parent IDs separately
            content: discourse_post.raw.clone(),
            html_content: Some(discourse_post.cooked.clone()),
            created_at,
            updated_at,
            likes: 0,
            is_solution: false,
            score: None,
            read_status: false,
            attachment_ids: Vec::new(),
            canvas_entry_id: None,
            discourse_post_id: Some(discourse_post.id),
            sync_status: SyncStatus::SyncedWithDiscourse,
        }
    }
}

// Import conversion traits
pub trait FromCanvas {
    fn from_canvas(canvas_entry: &CanvasDiscussionEntry, topic_id: Uuid, author_id: Uuid) -> Self;
}

pub trait FromDiscourse {
    fn from_discourse(discourse_post: &DiscoursePost, topic_id: Uuid, author_id: Uuid) -> Self;
}