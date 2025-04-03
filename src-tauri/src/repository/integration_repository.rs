use std::sync::Arc;
use rusqlite::{params, Connection, Result as SqliteResult, Error as SqliteError};
use crate::repository::RepositoryError;
use crate::models::forum::{Category, Topic};
use chrono::Utc;

pub struct IntegrationRepository {
    conn: Arc<Connection>,
}

impl IntegrationRepository {
    pub fn new(conn: Arc<Connection>) -> Self {
        Self { conn }
    }
    
    // Link a course to a forum category
    pub fn link_course_to_category(&self, course_id: i64, category_id: i64) -> Result<(), RepositoryError> {
        // Check if mapping already exists
        let existing: Option<i64> = self.conn.query_row(
            "SELECT 1 FROM course_forum_mappings WHERE course_id = ?1 AND category_id = ?2",
            params![course_id, category_id],
            |row| row.get(0),
        ).optional().map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        
        if existing.is_none() {
            let now = Utc::now().to_rfc3339();
            
            self.conn.execute(
                "INSERT INTO course_forum_mappings (course_id, category_id, created_at, updated_at)
                 VALUES (?1, ?2, ?3, ?4)",
                params![course_id, category_id, now, now],
            ).map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        }
        
        // Also update the category to link it to the course
        self.conn.execute(
            "UPDATE forum_categories SET course_id = ?1 WHERE id = ?2",
            params![course_id, category_id],
        ).map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        
        Ok(())
    }
    
    // Get category for a course
    pub fn get_category_for_course(&self, course_id: i64) -> Result<Option<Category>, RepositoryError> {
        let result = self.conn.query_row(
            "SELECT c.id, c.name, c.slug, c.description, c.parent_id, c.course_id,
                  c.color, c.text_color, c.topic_count, c.post_count, 
                  c.created_at, c.updated_at
             FROM forum_categories c
             WHERE c.course_id = ?1",
            params![course_id],
            |row| {
                let created_at_str: String = row.get(10)?;
                let updated_at_str: String = row.get(11)?;
                
                Ok(Category {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    slug: row.get(2)?,
                    description: row.get(3)?,
                    parent_id: row.get(4)?,
                    course_id: row.get(5)?,
                    color: row.get(6)?,
                    text_color: row.get(7)?,
                    topic_count: row.get(8)?,
                    post_count: row.get(9)?,
                    created_at: chrono::DateTime::parse_from_rfc3339(&created_at_str)
                        .map_err(|_| SqliteError::InvalidParameterName("Invalid date format".to_string()))?
                        .with_timezone(&chrono::Utc),
                    updated_at: chrono::DateTime::parse_from_rfc3339(&updated_at_str)
                        .map_err(|_| SqliteError::InvalidParameterName("Invalid date format".to_string()))?
                        .with_timezone(&chrono::Utc),
                })
            },
        );
        
        match result {
            Ok(category) => Ok(Some(category)),
            Err(SqliteError::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(RepositoryError::DatabaseError(e.to_string())),
        }
    }
    
    // Link module to forum topic
    pub fn link_module_to_topic(&self, module_id: i64, topic_id: i64) -> Result<(), RepositoryError> {
        // Check if mapping already exists
        let existing: Option<i64> = self.conn.query_row(
            "SELECT 1 FROM module_forum_mappings WHERE module_id = ?1 AND topic_id = ?2",
            params![module_id, topic_id],
            |row| row.get(0),
        ).optional().map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        
        if existing.is_none() {
            let now = Utc::now().to_rfc3339();
            
            self.conn.execute(
                "INSERT INTO module_forum_mappings (module_id, topic_id, created_at, updated_at)
                 VALUES (?1, ?2, ?3, ?4)",
                params![module_id, topic_id, now, now],
            ).map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        }
        
        Ok(())
    }
    
    // Get topic for a module
    pub fn get_topic_for_module(&self, module_id: i64) -> Result<Option<Topic>, RepositoryError> {
        let result = self.conn.query_row(
            "SELECT t.id, t.title, t.slug, t.category_id, t.user_id, t.pinned, t.locked,
                  t.post_count, t.view_count, t.has_solution, t.last_post_at,
                  t.created_at, t.updated_at, u.username as author_name, c.name as category_name
             FROM forum_topics t
             JOIN module_forum_mappings m ON t.id = m.topic_id
             JOIN users u ON t.user_id = u.id
             JOIN forum_categories c ON t.category_id = c.id
             WHERE m.module_id = ?1",
            params![module_id],
            |row| {
                let last_post_at: Option<String> = row.get(10)?;
                let created_at_str: String = row.get(11)?;
                let updated_at_str: String = row.get(12)?;
                
                Ok(Topic {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    slug: row.get(2)?,
                    category_id: row.get(3)?,
                    user_id: row.get(4)?,
                    pinned: row.get(5)?,
                    locked: row.get(6)?,
                    post_count: row.get(7)?,
                    view_count: row.get(8)?,
                    has_solution: row.get(9)?,
                    last_post_at: last_post_at.map(|s| 
                        chrono::DateTime::parse_from_rfc3339(&s)
                            .map_err(|_| SqliteError::InvalidParameterName("Invalid date format".to_string()))
                            .unwrap()
                            .with_timezone(&chrono::Utc)
                    ),
                    created_at: chrono::DateTime::parse_from_rfc3339(&created_at_str)
                        .map_err(|_| SqliteError::InvalidParameterName("Invalid date format".to_string()))?
                        .with_timezone(&chrono::Utc),
                    updated_at: chrono::DateTime::parse_from_rfc3339(&updated_at_str)
                        .map_err(|_| SqliteError::InvalidParameterName("Invalid date format".to_string()))?
                        .with_timezone(&chrono::Utc),
                    author_name: row.get(13)?,
                    category_name: row.get(14)?,
                    // These are for UI display only, not needed here
                    reply_count: 0,
                    excerpt: None,
                })
            },
        );
        
        match result {
            Ok(topic) => Ok(Some(topic)),
            Err(SqliteError::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(RepositoryError::DatabaseError(e.to_string())),
        }
    }
    
    // Link assignment to forum topic
    pub fn link_assignment_to_topic(&self, assignment_id: i64, topic_id: i64) -> Result<(), RepositoryError> {
        // Check if mapping already exists
        let existing: Option<i64> = self.conn.query_row(
            "SELECT 1 FROM assignment_forum_mappings WHERE assignment_id = ?1 AND topic_id = ?2",
            params![assignment_id, topic_id],
            |row| row.get(0),
        ).optional().map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        
        if existing.is_none() {
            let now = Utc::now().to_rfc3339();
            
            self.conn.execute(
                "INSERT INTO assignment_forum_mappings (assignment_id, topic_id, created_at, updated_at)
                 VALUES (?1, ?2, ?3, ?4)",
                params![assignment_id, topic_id, now, now],
            ).map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        }
        
        Ok(())
    }
    
    // Get topic for an assignment
    pub fn get_topic_for_assignment(&self, assignment_id: i64) -> Result<Option<Topic>, RepositoryError> {
        let result = self.conn.query_row(
            "SELECT t.id, t.title, t.slug, t.category_id, t.user_id, t.pinned, t.locked,
                  t.post_count, t.view_count, t.has_solution, t.last_post_at,
                  t.created_at, t.updated_at, u.username as author_name, c.name as category_name
             FROM forum_topics t
             JOIN assignment_forum_mappings a ON t.id = a.topic_id
             JOIN users u ON t.user_id = u.id
             JOIN forum_categories c ON t.category_id = c.id
             WHERE a.assignment_id = ?1",
            params![assignment_id],
            |row| {
                let last_post_at: Option<String> = row.get(10)?;
                let created_at_str: String = row.get(11)?;
                let updated_at_str: String = row.get(12)?;
                
                Ok(Topic {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    slug: row.get(2)?,
                    category_id: row.get(3)?,
                    user_id: row.get(4)?,
                    pinned: row.get(5)?,
                    locked: row.get(6)?,
                    post_count: row.get(7)?,
                    view_count: row.get(8)?,
                    has_solution: row.get(9)?,
                    last_post_at: last_post_at.map(|s| 
                        chrono::DateTime::parse_from_rfc3339(&s)
                            .map_err(|_| SqliteError::InvalidParameterName("Invalid date format".to_string()))
                            .unwrap()
                            .with_timezone(&chrono::Utc)
                    ),
                    created_at: chrono::DateTime::parse_from_rfc3339(&created_at_str)
                        .map_err(|_| SqliteError::InvalidParameterName("Invalid date format".to_string()))?
                        .with_timezone(&chrono::Utc),
                    updated_at: chrono::DateTime::parse_from_rfc3339(&updated_at_str)
                        .map_err(|_| SqliteError::InvalidParameterName("Invalid date format".to_string()))?
                        .with_timezone(&chrono::Utc),
                    author_name: row.get(13)?,
                    category_name: row.get(14)?,
                    // These are for UI display only, not needed here
                    reply_count: 0,
                    excerpt: None,
                })
            },
        );
        
        match result {
            Ok(topic) => Ok(Some(topic)),
            Err(SqliteError::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(RepositoryError::DatabaseError(e.to_string())),
        }
    }
    
    // Get recent forum activity for a course
    pub fn get_recent_course_activity(
        &self, 
        course_id: i64,
        limit: usize
    ) -> Result<Vec<Topic>, RepositoryError> {
        // First get the category for the course
        let category = match self.get_category_for_course(course_id)? {
            Some(category) => category,
            None => return Ok(Vec::new()), // No category, so no activity
        };
        
        // Get recent topics from this category
        let mut stmt = self.conn.prepare(
            "SELECT t.id, t.title, t.slug, t.category_id, t.user_id, t.pinned, t.locked,
                  t.post_count, t.view_count, t.has_solution, t.last_post_at,
                  t.created_at, t.updated_at, u.username as author_name, c.name as category_name,
                  (t.post_count - 1) as reply_count,
                  (SELECT SUBSTR(p.content, 1, 100) FROM forum_posts p WHERE p.topic_id = t.id ORDER BY p.created_at ASC LIMIT 1) as excerpt
             FROM forum_topics t
             JOIN users u ON t.user_id = u.id
             JOIN forum_categories c ON t.category_id = c.id
             WHERE t.category_id = ?1 AND t.deleted_at IS NULL
             ORDER BY COALESCE(t.last_post_at, t.created_at) DESC
             LIMIT ?2"
        ).map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        
        let rows = stmt.query_map(
            params![category.id, limit as i64],
            |row| {
                let last_post_at: Option<String> = row.get(10)?;
                let created_at_str: String = row.get(11)?;
                let updated_at_str: String = row.get(12)?;
                let reply_count: i32 = row.get(15)?;
                let excerpt: Option<String> = row.get(16)?;
                
                Ok(Topic {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    slug: row.get(2)?,
                    category_id: row.get(3)?,
                    user_id: row.get(4)?,
                    pinned: row.get(5)?,
                    locked: row.get(6)?,
                    post_count: row.get(7)?,
                    view_count: row.get(8)?,
                    has_solution: row.get(9)?,
                    last_post_at: last_post_at.map(|s| 
                        chrono::DateTime::parse_from_rfc3339(&s)
                            .map_err(|_| SqliteError::InvalidParameterName("Invalid date format".to_string()))
                            .unwrap()
                            .with_timezone(&chrono::Utc)
                    ),
                    created_at: chrono::DateTime::parse_from_rfc3339(&created_at_str)
                        .map_err(|_| SqliteError::InvalidParameterName("Invalid date format".to_string()))?
                        .with_timezone(&chrono::Utc),
                    updated_at: chrono::DateTime::parse_from_rfc3339(&updated_at_str)
                        .map_err(|_| SqliteError::InvalidParameterName("Invalid date format".to_string()))?
                        .with_timezone(&chrono::Utc),
                    author_name: row.get(13)?,
                    category_name: row.get(14)?,
                    reply_count,
                    excerpt,
                })
            },
        ).map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        
        let mut topics = Vec::new();
        for row in rows {
            topics.push(row.map_err(|e| RepositoryError::DatabaseError(e.to_string()))?);
        }
        
        Ok(topics)
    }
}