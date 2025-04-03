use crate::models::{Topic, Post};
use chrono::{DateTime, Utc};
use rusqlite::{params, Connection, Result, Row};

pub struct TopicRepository<'a> {
    conn: &'a Connection,
}

impl<'a> TopicRepository<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }
    
    // Convert a database row to a Topic model
    fn row_to_topic(row: &Row) -> Result<Topic> {
        Ok(Topic {
            id: Some(row.get(0)?),
            title: row.get(1)?,
            slug: row.get(2)?,
            category_id: row.get(3)?,
            user_id: row.get(4)?,
            views: row.get(5)?,
            created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(6)?)
                .map_err(|e| rusqlite::Error::FromSqlConversionFailure(6, rusqlite::types::Type::Text, Box::new(e)))?
                .with_timezone(&Utc),
            updated_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(7)?)
                .map_err(|e| rusqlite::Error::FromSqlConversionFailure(7, rusqlite::types::Type::Text, Box::new(e)))?
                .with_timezone(&Utc),
            last_posted_at: row.get::<_, Option<String>>(8)?
                .map(|dt| DateTime::parse_from_rfc3339(&dt)
                    .map_err(|e| rusqlite::Error::FromSqlConversionFailure(8, rusqlite::types::Type::Text, Box::new(e)))
                    .map(|dt| dt.with_timezone(&Utc)))
                .transpose()?,
            is_closed: row.get(9)?,
            is_pinned: row.get(10)?,
            is_deleted: row.get(11)?,
        })
    }

    // Create a new topic
    pub fn create(&self, topic: &Topic) -> Result<i64> {
        self.conn.execute(
            "INSERT INTO topics 
            (title, slug, category_id, user_id, views, created_at, updated_at, 
             last_posted_at, is_closed, is_pinned, is_deleted) 
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
            params![
                topic.title, topic.slug, topic.category_id, topic.user_id, 
                topic.views, topic.created_at.to_rfc3339(), topic.updated_at.to_rfc3339(),
                topic.last_posted_at.map(|dt| dt.to_rfc3339()),
                topic.is_closed, topic.is_pinned, topic.is_deleted
            ],
        )?;
        
        Ok(self.conn.last_insert_rowid())
    }

    // Get a topic by ID
    pub fn find_by_id(&self, id: i64) -> Result<Option<Topic>> {
        let mut stmt = self.conn.prepare(
            "SELECT * FROM topics WHERE id = ? AND is_deleted = 0"
        )?;
        
        let mut rows = stmt.query([id])?;
        
        if let Some(row) = rows.next()? {
            Ok(Some(Self::row_to_topic(row)?))
        } else {
            Ok(None)
        }
    }
    
    // Get topics by category ID
    pub fn find_by_category_id(&self, category_id: i64, limit: i64, offset: i64) -> Result<Vec<Topic>> {
        let mut stmt = self.conn.prepare(
            "SELECT * FROM topics 
             WHERE category_id = ? AND is_deleted = 0 
             ORDER BY is_pinned DESC, last_posted_at DESC, created_at DESC 
             LIMIT ? OFFSET ?"
        )?;
        
        let rows = stmt.query_map(params![category_id, limit, offset], |row| {
            Self::row_to_topic(row)
        })?;
        
        let mut topics = Vec::new();
        for topic_result in rows {
            topics.push(topic_result?);
        }
        
        Ok(topics)
    }
    
    // Update a topic
    pub fn update(&self, topic: &Topic) -> Result<()> {
        if topic.id.is_none() {
            return Err(rusqlite::Error::InvalidParameterName("Topic ID is required for update".to_string()));
        }
        
        self.conn.execute(
            "UPDATE topics SET 
            title = ?1, slug = ?2, views = ?3, updated_at = ?4, 
            last_posted_at = ?5, is_closed = ?6, is_pinned = ?7 
            WHERE id = ?8",
            params![
                topic.title, topic.slug, topic.views, Utc::now().to_rfc3339(),
                topic.last_posted_at.map(|dt| dt.to_rfc3339()),
                topic.is_closed, topic.is_pinned, topic.id
            ],
        )?;
        
        Ok(())
    }
    
    // Increment view count
    pub fn increment_view_count(&self, id: i64) -> Result<()> {
        self.conn.execute(
            "UPDATE topics SET views = views + 1 WHERE id = ?",
            [id],
        )?;
        
        Ok(())
    }
    
    // Soft delete a topic
    pub fn delete(&self, id: i64) -> Result<()> {
        self.conn.execute(
            "UPDATE topics SET is_deleted = 1, updated_at = ? WHERE id = ?",
            params![Utc::now().to_rfc3339(), id],
        )?;
        
        Ok(())
    }
    
    // Update last posted time
    pub fn update_last_posted(&self, id: i64) -> Result<()> {
        self.conn.execute(
            "UPDATE topics SET last_posted_at = ?, updated_at = ? WHERE id = ?",
            params![Utc::now().to_rfc3339(), Utc::now().to_rfc3339(), id],
        )?;
        
        Ok(())
    }
}