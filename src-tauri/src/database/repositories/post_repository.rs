use crate::models::forum::post::Post;
use chrono::{DateTime, Utc};
use rusqlite::{params, Connection, Result, Row};

pub struct PostRepository<'a> {
    conn: &'a Connection,
}

impl<'a> PostRepository<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }
    
    // Convert a database row to a Post model
    fn row_to_post(row: &Row) -> Result<Post> {
        Ok(Post {
            id: Some(row.get(0)?),
            topic_id: row.get(1)?,
            user_id: row.get(2)?,
            content: row.get(3)?,
            content_html: row.get(4)?,
            created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(5)?)
                .map_err(|e| rusqlite::Error::FromSqlConversionFailure(5, rusqlite::types::Type::Text, Box::new(e)))?
                .with_timezone(&Utc),
            updated_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(6)?)
                .map_err(|e| rusqlite::Error::FromSqlConversionFailure(6, rusqlite::types::Type::Text, Box::new(e)))?
                .with_timezone(&Utc),
            is_deleted: row.get(7)?,
        })
    }
    
    // Create a new post
    pub fn create(&self, post: &Post) -> Result<i64> {
        self.conn.execute(
            "INSERT INTO posts 
            (topic_id, user_id, content, content_html, created_at, updated_at, is_deleted) 
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                post.topic_id, post.user_id, post.content, post.content_html,
                post.created_at.to_rfc3339(), post.updated_at.to_rfc3339(), post.is_deleted
            ],
        )?;
        
        Ok(self.conn.last_insert_rowid())
    }
    
    // Get a post by ID
    pub fn find_by_id(&self, id: i64) -> Result<Option<Post>> {
        let mut stmt = self.conn.prepare(
            "SELECT * FROM posts WHERE id = ? AND is_deleted = 0"
        )?;
        
        let mut rows = stmt.query([id])?;
        
        if let Some(row) = rows.next()? {
            Ok(Some(Self::row_to_post(row)?))
        } else {
            Ok(None)
        }
    }
    
    // Get posts by topic ID
    pub fn find_by_topic_id(&self, topic_id: i64, limit: i64, offset: i64) -> Result<Vec<Post>> {
        let mut stmt = self.conn.prepare(
            "SELECT * FROM posts 
             WHERE topic_id = ? AND is_deleted = 0 
             ORDER BY created_at ASC 
             LIMIT ? OFFSET ?"
        )?;
        
        let rows = stmt.query_map(params![topic_id, limit, offset], |row| {
            Self::row_to_post(row)
        })?;
        
        let mut posts = Vec::new();
        for post_result in rows {
            posts.push(post_result?);
        }
        
        Ok(posts)
    }
    
    // Update a post
    pub fn update(&self, post: &Post) -> Result<()> {
        if post.id.is_none() {
            return Err(rusqlite::Error::InvalidParameterName("Post ID is required for update".to_string()));
        }
        
        self.conn.execute(
            "UPDATE posts SET 
            content = ?1, content_html = ?2, updated_at = ?3 
            WHERE id = ?4",
            params![
                post.content, post.content_html, Utc::now().to_rfc3339(), post.id
            ],
        )?;
        
        Ok(())
    }
    
    // Soft delete a post
    pub fn delete(&self, id: i64) -> Result<()> {
        self.conn.execute(
            "UPDATE posts SET is_deleted = 1, updated_at = ? WHERE id = ?",
            params![Utc::now().to_rfc3339(), id],
        )?;
        
        Ok(())
    }
    
    // Count posts in a topic
    pub fn count_by_topic_id(&self, topic_id: i64) -> Result<i64> {
        let mut stmt = self.conn.prepare(
            "SELECT COUNT(*) FROM posts WHERE topic_id = ? AND is_deleted = 0"
        )?;
        
        let count = stmt.query_row([topic_id], |row| row.get(0))?;
        
        Ok(count)
    }
}