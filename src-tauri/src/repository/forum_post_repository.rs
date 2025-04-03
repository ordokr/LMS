use std::sync::Arc;
use rusqlite::{params, Connection, Result as SqliteResult, Error as SqliteError};
use crate::models::forum::{Post, PostWithUser, CreatePostRequest, UpdatePostRequest};
use crate::repository::RepositoryError;
use chrono::{DateTime, Utc};

pub struct ForumPostRepository {
    conn: Arc<Connection>,
}

impl ForumPostRepository {
    pub fn new(conn: Arc<Connection>) -> Self {
        Self { conn }
    }
    
    // Create a new forum post
    pub fn create_post(&self, request: &CreatePostRequest, user_id: i64) -> Result<Post, RepositoryError> {
        let now = Utc::now();
        
        let result = self.conn.execute(
            "INSERT INTO forum_posts (topic_id, user_id, content, raw_content, parent_id, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                request.topic_id,
                user_id,
                request.content,
                request.content,  // Store same content as raw (we'll process markdown elsewhere)
                request.parent_id,
                now.to_rfc3339(),
                now.to_rfc3339(),
            ],
        ).map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        
        let id = self.conn.last_insert_rowid();
        
        if result == 0 {
            return Err(RepositoryError::InsertFailed);
        }
        
        // Update the post count and last_post_at for the topic
        self.conn.execute(
            "UPDATE forum_topics SET 
             post_count = post_count + 1, 
             last_post_at = ?1,
             last_poster_id = ?2
             WHERE id = ?3",
            params![now.to_rfc3339(), user_id, request.topic_id],
        ).map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        
        // Update category post count
        self.update_category_post_count_for_topic(request.topic_id)?;
        
        self.get_post(id)
    }
    
    // Get a single post by ID
    pub fn get_post(&self, id: i64) -> Result<Post, RepositoryError> {
        let post = self.conn.query_row(
            "SELECT id, topic_id, user_id, content, raw_content, parent_id, is_solution, 
             like_count, created_at, updated_at 
             FROM forum_posts 
             WHERE id = ?1 AND deleted_at IS NULL",
            params![id],
            |row| {
                let created_at_str: String = row.get(8)?;
                let updated_at_str: String = row.get(9)?;
                
                let created_at = DateTime::parse_from_rfc3339(&created_at_str)
                    .map_err(|_| SqliteError::InvalidParameterName("Invalid date format".to_string()))?;
                
                let updated_at = DateTime::parse_from_rfc3339(&updated_at_str)
                    .map_err(|_| SqliteError::InvalidParameterName("Invalid date format".to_string()))?;
                
                Ok(Post {
                    id: row.get(0)?,
                    topic_id: row.get(1)?,
                    user_id: row.get(2)?,
                    content: row.get(3)?,
                    raw_content: row.get(4)?,
                    parent_id: row.get(5)?,
                    is_solution: row.get(6)?,
                    like_count: row.get(7)?,
                    created_at: created_at.with_timezone(&Utc),
                    updated_at: updated_at.with_timezone(&Utc),
                })
            },
        ).map_err(|e| match e {
            SqliteError::QueryReturnedNoRows => RepositoryError::NotFound,
            e => RepositoryError::DatabaseError(e.to_string()),
        })?;
        
        Ok(post)
    }
    
    // Get a post with user information
    pub fn get_post_with_user(&self, id: i64) -> Result<PostWithUser, RepositoryError> {
        let post_with_user = self.conn.query_row(
            "SELECT p.id, p.topic_id, p.user_id, p.content, p.raw_content, p.parent_id, 
             p.is_solution, p.like_count, p.created_at, p.updated_at,
             u.username, u.display_name, u.avatar_url, u.role
             FROM forum_posts p
             JOIN users u ON p.user_id = u.id
             WHERE p.id = ?1 AND p.deleted_at IS NULL",
            params![id],
            |row| {
                let created_at_str: String = row.get(8)?;
                let updated_at_str: String = row.get(9)?;
                
                let created_at = DateTime::parse_from_rfc3339(&created_at_str)
                    .map_err(|_| SqliteError::InvalidParameterName("Invalid date format".to_string()))?;
                
                let updated_at = DateTime::parse_from_rfc3339(&updated_at_str)
                    .map_err(|_| SqliteError::InvalidParameterName("Invalid date format".to_string()))?;
                
                Ok(PostWithUser {
                    id: row.get(0)?,
                    topic_id: row.get(1)?,
                    user_id: row.get(2)?,
                    content: row.get(3)?,
                    raw_content: row.get(4)?,
                    parent_id: row.get(5)?,
                    is_solution: row.get(6)?,
                    like_count: row.get(7)?,
                    created_at: created_at.with_timezone(&Utc),
                    updated_at: updated_at.with_timezone(&Utc),
                    author_username: row.get(10)?,
                    author_display_name: row.get(11)?,
                    author_avatar_url: row.get(12)?,
                    author_role: row.get(13)?,
                })
            },
        ).map_err(|e| match e {
            SqliteError::QueryReturnedNoRows => RepositoryError::NotFound,
            e => RepositoryError::DatabaseError(e.to_string()),
        })?;
        
        Ok(post_with_user)
    }
    
    // Get posts for a topic (with pagination)
    pub fn get_posts_for_topic(
        &self, 
        topic_id: i64, 
        page: Option<usize>, 
        per_page: Option<usize>
    ) -> Result<Vec<PostWithUser>, RepositoryError> {
        let page = page.unwrap_or(1);
        let per_page = per_page.unwrap_or(20);
        let offset = (page - 1) * per_page;
        
        let mut stmt = self.conn.prepare(
            "SELECT p.id, p.topic_id, p.user_id, p.content, p.raw_content, p.parent_id, 
             p.is_solution, p.like_count, p.created_at, p.updated_at,
             u.username, u.display_name, u.avatar_url, u.role
             FROM forum_posts p
             JOIN users u ON p.user_id = u.id
             WHERE p.topic_id = ?1 AND p.deleted_at IS NULL
             ORDER BY p.created_at ASC
             LIMIT ?2 OFFSET ?3"
        ).map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        
        let rows = stmt.query_map(
            params![topic_id, per_page as i64, offset as i64],
            |row| {
                let created_at_str: String = row.get(8)?;
                let updated_at_str: String = row.get(9)?;
                
                let created_at = DateTime::parse_from_rfc3339(&created_at_str)
                    .map_err(|_| SqliteError::InvalidParameterName("Invalid date format".to_string()))?;
                
                let updated_at = DateTime::parse_from_rfc3339(&updated_at_str)
                    .map_err(|_| SqliteError::InvalidParameterName("Invalid date format".to_string()))?;
                
                Ok(PostWithUser {
                    id: row.get(0)?,
                    topic_id: row.get(1)?,
                    user_id: row.get(2)?,
                    content: row.get(3)?,
                    raw_content: row.get(4)?,
                    parent_id: row.get(5)?,
                    is_solution: row.get(6)?,
                    like_count: row.get(7)?,
                    created_at: created_at.with_timezone(&Utc),
                    updated_at: updated_at.with_timezone(&Utc),
                    author_username: row.get(10)?,
                    author_display_name: row.get(11)?,
                    author_avatar_url: row.get(12)?,
                    author_role: row.get(13)?,
                })
            },
        ).map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        
        let mut posts = Vec::new();
        for row in rows {
            posts.push(row.map_err(|e| RepositoryError::DatabaseError(e.to_string()))?);
        }
        
        Ok(posts)
    }
    
    // Update a post
    pub fn update_post(&self, id: i64, request: &UpdatePostRequest) -> Result<Post, RepositoryError> {
        let now = Utc::now();
        
        let result = self.conn.execute(
            "UPDATE forum_posts SET 
             content = ?1, 
             raw_content = ?2,
             updated_at = ?3
             WHERE id = ?4",
            params![
                request.content,
                request.content,
                now.to_rfc3339(),
                id
            ],
        ).map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        
        if result == 0 {
            return Err(RepositoryError::NotFound);
        }
        
        self.get_post(id)
    }
    
    // Mark a post as a solution
    pub fn mark_as_solution(&self, post_id: i64) -> Result<Post, RepositoryError> {
        // Get the post to check which topic it belongs to
        let post = self.get_post(post_id)?;
        
        let now = Utc::now();
        
        // Begin transaction
        let tx = self.conn.transaction()
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        
        // Clear any existing solution for this topic
        tx.execute(
            "UPDATE forum_posts SET 
             is_solution = 0,
             updated_at = ?1
             WHERE topic_id = ?2 AND is_solution = 1",
            params![now.to_rfc3339(), post.topic_id],
        ).map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        
        // Mark this post as the solution
        tx.execute(
            "UPDATE forum_posts SET 
             is_solution = 1,
             updated_at = ?1
             WHERE id = ?2",
            params![now.to_rfc3339(), post_id],
        ).map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        
        // Update topic to indicate it has a solution
        tx.execute(
            "UPDATE forum_topics SET 
             has_solution = 1,
             updated_at = ?1
             WHERE id = ?2",
            params![now.to_rfc3339(), post.topic_id],
        ).map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        
        // Commit transaction
        tx.commit()
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        
        self.get_post(post_id)
    }
    
    // Add a like to a post
    pub fn like_post(&self, post_id: i64, user_id: i64) -> Result<Post, RepositoryError> {
        let now = Utc::now();
        
        // Begin transaction
        let tx = self.conn.transaction()
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        
        // Check if user already liked this post
        let existing_like: Option<i64> = tx.query_row(
            "SELECT 1 FROM forum_post_likes WHERE post_id = ?1 AND user_id = ?2",
            params![post_id, user_id],
            |row| row.get(0),
        ).optional().map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        
        if existing_like.is_none() {
            // Add the like
            tx.execute(
                "INSERT INTO forum_post_likes (post_id, user_id, created_at)
                 VALUES (?1, ?2, ?3)",
                params![post_id, user_id, now.to_rfc3339()],
            ).map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
            
            // Increment like count on post
            tx.execute(
                "UPDATE forum_posts SET 
                 like_count = like_count + 1,
                 updated_at = ?1
                 WHERE id = ?2",
                params![now.to_rfc3339(), post_id],
            ).map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        }
        
        // Commit transaction
        tx.commit()
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        
        self.get_post(post_id)
    }
    
    // Unlike a post
    pub fn unlike_post(&self, post_id: i64, user_id: i64) -> Result<Post, RepositoryError> {
        let now = Utc::now();
        
        // Begin transaction
        let tx = self.conn.transaction()
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        
        // Remove the like
        let result = tx.execute(
            "DELETE FROM forum_post_likes WHERE post_id = ?1 AND user_id = ?2",
            params![post_id, user_id],
        ).map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        
        if result > 0 {
            // Decrement like count on post
            tx.execute(
                "UPDATE forum_posts SET 
                 like_count = like_count - 1,
                 updated_at = ?1
                 WHERE id = ?2",
                params![now.to_rfc3339(), post_id],
            ).map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        }
        
        // Commit transaction
        tx.commit()
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        
        self.get_post(post_id)
    }
    
    // Soft delete a post
    pub fn delete_post(&self, id: i64) -> Result<(), RepositoryError> {
        let now = Utc::now();
        
        // Get the post to check which topic it belongs to
        let post = self.get_post(id)?;
        
        // Begin transaction
        let tx = self.conn.transaction()
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        
        // Mark post as deleted
        tx.execute(
            "UPDATE forum_posts SET 
             deleted_at = ?1
             WHERE id = ?2",
            params![now.to_rfc3339(), id],
        ).map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        
        // Update the post count for the topic
        tx.execute(
            "UPDATE forum_topics SET 
             post_count = post_count - 1
             WHERE id = ?1",
            params![post.topic_id],
        ).map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        
        // Update category post count
        tx.execute(
            "UPDATE forum_categories c
             SET post_count = post_count - 1
             FROM forum_topics t
             WHERE t.id = ?1 AND t.category_id = c.id",
            params![post.topic_id],
        ).map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        
        // Commit transaction
        tx.commit()
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        
        Ok(())
    }
    
    // Get recent posts across topics (for activity feeds)
    pub fn get_recent_posts(&self, limit: usize) -> Result<Vec<PostWithUser>, RepositoryError> {
        let mut stmt = self.conn.prepare(
            "SELECT p.id, p.topic_id, p.user_id, p.content, p.raw_content, p.parent_id, 
             p.is_solution, p.like_count, p.created_at, p.updated_at,
             u.username, u.display_name, u.avatar_url, u.role,
             t.title as topic_title, c.id as category_id, c.name as category_name
             FROM forum_posts p
             JOIN users u ON p.user_id = u.id
             JOIN forum_topics t ON p.topic_id = t.id
             JOIN forum_categories c ON t.category_id = c.id
             WHERE p.deleted_at IS NULL
             ORDER BY p.created_at DESC
             LIMIT ?1"
        ).map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        
        let rows = stmt.query_map(
            params![limit as i64],
            |row| {
                let created_at_str: String = row.get(8)?;
                let updated_at_str: String = row.get(9)?;
                
                let created_at = DateTime::parse_from_rfc3339(&created_at_str)
                    .map_err(|_| SqliteError::InvalidParameterName("Invalid date format".to_string()))?;
                
                let updated_at = DateTime::parse_from_rfc3339(&updated_at_str)
                    .map_err(|_| SqliteError::InvalidParameterName("Invalid date format".to_string()))?;
                
                Ok(PostWithUser {
                    id: row.get(0)?,
                    topic_id: row.get(1)?,
                    user_id: row.get(2)?,
                    content: row.get(3)?,
                    raw_content: row.get(4)?,
                    parent_id: row.get(5)?,
                    is_solution: row.get(6)?,
                    like_count: row.get(7)?,
                    created_at: created_at.with_timezone(&Utc),
                    updated_at: updated_at.with_timezone(&Utc),
                    author_username: row.get(10)?,
                    author_display_name: row.get(11)?,
                    author_avatar_url: row.get(12)?,
                    author_role: row.get(13)?,
                    topic_title: Some(row.get(14)?),
                    category_id: Some(row.get(15)?),
                    category_name: Some(row.get(16)?),
                })
            },
        ).map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        
        let mut posts = Vec::new();
        for row in rows {
            posts.push(row.map_err(|e| RepositoryError::DatabaseError(e.to_string()))?);
        }
        
        Ok(posts)
    }
    
    // Count posts for a topic
    pub fn count_posts_for_topic(&self, topic_id: i64) -> Result<i64, RepositoryError> {
        let count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM forum_posts WHERE topic_id = ?1 AND deleted_at IS NULL",
            params![topic_id],
            |row| row.get(0),
        ).map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        
        Ok(count)
    }
    
    // Get posts created since a timestamp (for sync)
    pub fn get_posts_created_since(&self, timestamp: DateTime<Utc>) -> Result<Vec<Post>, RepositoryError> {
        let mut stmt = self.conn.prepare(
            "SELECT id, topic_id, user_id, content, raw_content, parent_id, 
             is_solution, like_count, created_at, updated_at 
             FROM forum_posts 
             WHERE created_at >= ?1 AND deleted_at IS NULL
             ORDER BY created_at ASC"
        ).map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        
        let rows = stmt.query_map(
            params![timestamp.to_rfc3339()],
            |row| {
                let created_at_str: String = row.get(8)?;
                let updated_at_str: String = row.get(9)?;
                
                let created_at = DateTime::parse_from_rfc3339(&created_at_str)
                    .map_err(|_| SqliteError::InvalidParameterName("Invalid date format".to_string()))?;
                
                let updated_at = DateTime::parse_from_rfc3339(&updated_at_str)
                    .map_err(|_| SqliteError::InvalidParameterName("Invalid date format".to_string()))?;
                
                Ok(Post {
                    id: row.get(0)?,
                    topic_id: row.get(1)?,
                    user_id: row.get(2)?,
                    content: row.get(3)?,
                    raw_content: row.get(4)?,
                    parent_id: row.get(5)?,
                    is_solution: row.get(6)?,
                    like_count: row.get(7)?,
                    created_at: created_at.with_timezone(&Utc),
                    updated_at: updated_at.with_timezone(&Utc),
                })
            },
        ).map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        
        let mut posts = Vec::new();
        for row in rows {
            posts.push(row.map_err(|e| RepositoryError::DatabaseError(e.to_string()))?);
        }
        
        Ok(posts)
    }
    
    // Get posts updated since a timestamp (for sync)
    pub fn get_posts_updated_since(&self, timestamp: DateTime<Utc>) -> Result<Vec<Post>, RepositoryError> {
        let mut stmt = self.conn.prepare(
            "SELECT id, topic_id, user_id, content, raw_content, parent_id, 
             is_solution, like_count, created_at, updated_at 
             FROM forum_posts 
             WHERE updated_at >= ?1 AND created_at < ?1 AND deleted_at IS NULL
             ORDER BY updated_at ASC"
        ).map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        
        let rows = stmt.query_map(
            params![timestamp.to_rfc3339()],
            |row| {
                let created_at_str: String = row.get(8)?;
                let updated_at_str: String = row.get(9)?;
                
                let created_at = DateTime::parse_from_rfc3339(&created_at_str)
                    .map_err(|_| SqliteError::InvalidParameterName("Invalid date format".to_string()))?;
                
                let updated_at = DateTime::parse_from_rfc3339(&updated_at_str)
                    .map_err(|_| SqliteError::InvalidParameterName("Invalid date format".to_string()))?;
                
                Ok(Post {
                    id: row.get(0)?,
                    topic_id: row.get(1)?,
                    user_id: row.get(2)?,
                    content: row.get(3)?,
                    raw_content: row.get(4)?,
                    parent_id: row.get(5)?,
                    is_solution: row.get(6)?,
                    like_count: row.get(7)?,
                    created_at: created_at.with_timezone(&Utc),
                    updated_at: updated_at.with_timezone(&Utc),
                })
            },
        ).map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        
        let mut posts = Vec::new();
        for row in rows {
            posts.push(row.map_err(|e| RepositoryError::DatabaseError(e.to_string()))?);
        }
        
        Ok(posts)
    }
    
    // Helper: Update category post count for a topic
    fn update_category_post_count_for_topic(&self, topic_id: i64) -> Result<(), RepositoryError> {
        self.conn.execute(
            "UPDATE forum_categories c
             SET post_count = (
                 SELECT COUNT(*) 
                 FROM forum_posts p 
                 JOIN forum_topics t ON p.topic_id = t.id
                 WHERE t.category_id = c.id AND p.deleted_at IS NULL
             )
             FROM forum_topics t
             WHERE t.id = ?1 AND t.category_id = c.id",
            params![topic_id],
        ).map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        
        Ok(())
    }
}