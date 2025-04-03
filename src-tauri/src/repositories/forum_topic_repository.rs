use sqlx::{Pool, Sqlite, Transaction, query, query_as};
use chrono::{DateTime, Utc};

use crate::db::DbError;
use crate::models::forum::Topic;

pub struct ForumTopicRepository {
    pool: Pool<Sqlite>,
}

impl ForumTopicRepository {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }
    
    pub async fn get_all(&self, page: usize, per_page: usize) -> Result<Vec<Topic>, DbError> {
        let offset = (page - 1) * per_page;
        
        let topics = query_as!(
            Topic,
            r#"
            SELECT 
                t.id, t.title, t.slug, t.category_id, t.user_id,
                t.pinned, t.locked, t.view_count,
                t.created_at as "created_at: DateTime<Utc>",
                t.updated_at as "updated_at: DateTime<Utc>",
                t.last_post_at as "last_post_at: Option<DateTime<Utc>>",
                t.last_post_id, t.last_poster_id, t.last_poster_name,
                c.name as category_name,
                u.username as author_name,
                (SELECT COUNT(*) FROM forum_posts WHERE topic_id = t.id) - 1 as reply_count,
                (SELECT content FROM forum_posts WHERE topic_id = t.id ORDER BY created_at LIMIT 1) as excerpt
            FROM forum_topics t
            JOIN forum_categories c ON t.category_id = c.id
            JOIN users u ON t.user_id = u.id
            ORDER BY t.pinned DESC, t.last_post_at DESC
            LIMIT ? OFFSET ?
            "#,
            per_page as i64, offset as i64
        )
        .fetch_all(&self.pool)
        .await?;
        
        Ok(topics)
    }
    
    pub async fn get_by_id(&self, id: i64) -> Result<Option<Topic>, DbError> {
        let topic = query_as!(
            Topic,
            r#"
            SELECT 
                t.id, t.title, t.slug, t.category_id, t.user_id,
                t.pinned, t.locked, t.view_count,
                t.created_at as "created_at: DateTime<Utc>",
                t.updated_at as "updated_at: DateTime<Utc>",
                t.last_post_at as "last_post_at: Option<DateTime<Utc>>",
                t.last_post_id, t.last_poster_id, t.last_poster_name,
                c.name as category_name,
                u.username as author_name,
                (SELECT COUNT(*) FROM forum_posts WHERE topic_id = t.id) - 1 as reply_count,
                (SELECT content FROM forum_posts WHERE topic_id = t.id ORDER BY created_at LIMIT 1) as excerpt
            FROM forum_topics t
            JOIN forum_categories c ON t.category_id = c.id
            JOIN users u ON t.user_id = u.id
            WHERE t.id = ?
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;
        
        Ok(topic)
    }
    
    pub async fn get_by_category_id(&self, category_id: i64, page: usize, per_page: usize) -> Result<Vec<Topic>, DbError> {
        let offset = (page - 1) * per_page;
        
        let topics = query_as!(
            Topic,
            r#"
            SELECT 
                t.id, t.title, t.slug, t.category_id, t.user_id,
                t.pinned, t.locked, t.view_count,
                t.created_at as "created_at: DateTime<Utc>",
                t.updated_at as "updated_at: DateTime<Utc>",
                t.last_post_at as "last_post_at: Option<DateTime<Utc>>",
                t.last_post_id, t.last_poster_id, t.last_poster_name,
                c.name as category_name,
                u.username as author_name,
                (SELECT COUNT(*) FROM forum_posts WHERE topic_id = t.id) - 1 as reply_count,
                (SELECT content FROM forum_posts WHERE topic_id = t.id ORDER BY created_at LIMIT 1) as excerpt
            FROM forum_topics t
            JOIN forum_categories c ON t.category_id = c.id
            JOIN users u ON t.user_id = u.id
            WHERE t.category_id = ?
            ORDER BY t.pinned DESC, t.last_post_at DESC
            LIMIT ? OFFSET ?
            "#,
            category_id, per_page as i64, offset as i64
        )
        .fetch_all(&self.pool)
        .await?;
        
        Ok(topics)
    }
    
    pub async fn get_recent(&self, page: usize, per_page: usize) -> Result<Vec<Topic>, DbError> {
        let offset = (page - 1) * per_page;
        
        let topics = query_as!(
            Topic,
            r#"
            SELECT 
                t.id, t.title, t.slug, t.category_id, t.user_id,
                t.pinned, t.locked, t.view_count,
                t.created_at as "created_at: DateTime<Utc>",
                t.updated_at as "updated_at: DateTime<Utc>",
                t.last_post_at as "last_post_at: Option<DateTime<Utc>>",
                t.last_post_id, t.last_poster_id, t.last_poster_name,
                c.name as category_name,
                u.username as author_name,
                (SELECT COUNT(*) FROM forum_posts WHERE topic_id = t.id) - 1 as reply_count,
                (SELECT content FROM forum_posts WHERE topic_id = t.id ORDER BY created_at LIMIT 1) as excerpt
            FROM forum_topics t
            JOIN forum_categories c ON t.category_id = c.id
            JOIN users u ON t.user_id = u.id
            ORDER BY t.last_post_at DESC NULLS LAST, t.created_at DESC
            LIMIT ? OFFSET ?
            "#,
            per_page as i64, offset as i64
        )
        .fetch_all(&self.pool)
        .await?;
        
        Ok(topics)
    }
    
    pub async fn create_with_tx(
        &self,
        tx: &mut Transaction<'_, Sqlite>,
        title: &str,
        slug: &str,
        category_id: i64,
        user_id: i64,
        pinned: bool,
        locked: bool,
    ) -> Result<Topic, DbError> {
        let now = Utc::now();
        
        // Get the username
        let username = query!(
            "SELECT username FROM users WHERE id = ?",
            user_id
        )
        .fetch_one(&self.pool)
        .await?
        .username;
        
        let id = query!(
            r#"
            INSERT INTO forum_topics (
                title, slug, category_id, user_id,
                pinned, locked, view_count,
                created_at, updated_at,
                last_post_at, last_poster_id, last_poster_name
            )
            VALUES (?, ?, ?, ?, ?, ?, 0, ?, ?, ?, ?, ?)
            RETURNING id
            "#,
            title, slug, category_id, user_id,
            pinned, locked, now, now,
            now, user_id, username
        )
        .fetch_one(tx)
        .await?
        .id;
        
        // Increment category counts
        query!(
            r#"
            UPDATE forum_categories
            SET topic_count = topic_count + 1,
                updated_at = ?
            WHERE id = ?
            "#,
            now, category_id
        )
        .execute(tx)
        .await?;
        
        // We need to fetch the full topic outside the transaction
        // to get all the calculated fields, so we create a minimal
        // version to return from this function
        let topic = Topic {
            id,
            title: title.to_string(),
            slug: slug.to_string(),
            category_id,
            user_id,
            pinned,
            locked,
            view_count: 0,
            created_at: now,
            updated_at: now,
            last_post_at: Some(now),
            last_post_id: None,
            last_poster_id: Some(user_id),
            last_poster_name: Some(username),
            category_name: String::new(), // This will be filled when fetched later
            author_name: String::new(),   // This will be filled when fetched later
            reply_count: 0,
            excerpt: None,
        };
        
        Ok(topic)
    }
    
    pub async fn update(
        &self,
        id: i64,
        title: &str,
        slug: &str,
        category_id: i64,
    ) -> Result<Topic, DbError> {
        let now = Utc::now();
        
        // Check if category is changing
        let old_category_id = query!(
            "SELECT category_id FROM forum_topics WHERE id = ?",
            id
        )
        .fetch_one(&self.pool)
        .await?
        .category_id;
        
        // Start transaction
        let mut tx = self.pool.begin().await?;
        
        // Update topic
        query!(
            r#"
            UPDATE forum_topics
            SET title = ?, slug = ?, category_id = ?, updated_at = ?
            WHERE id = ?
            "#,
            title, slug, category_id, now, id
        )
        .execute(&mut tx)
        .await?;
        
        // If category changed, update category counts
        if old_category_id != category_id {
            // Decrement old category
            query!(
                r#"
                UPDATE forum_categories
                SET topic_count = topic_count - 1,
                    updated_at = ?
                WHERE id = ?
                "#,
                now, old_category_id
            )
            .execute(&mut tx)
            .await?;
            
            // Increment new category
            query!(
                r#"
                UPDATE forum_categories
                SET topic_count = topic_count + 1,
                    updated_at = ?
                WHERE id = ?
                "#,
                now, category_id
            )
            .execute(&mut tx)
            .await?;
        }
        
        // Commit transaction
        tx.commit().await?;
        
        self.get_by_id(id)
            .await?
            .ok_or_else(|| DbError::NotFound("Topic not found after update".into()))
    }
    
    pub async fn delete(&self, id: i64) -> Result<(), DbError> {
        // Start transaction
        let mut tx = self.pool.begin().await?;
        
        // Get topic details for category count updates
        let topic = query!(
            "SELECT category_id FROM forum_topics WHERE id = ?",
            id
        )
        .fetch_optional(&mut tx)
        .await?
        .ok_or_else(|| DbError::NotFound("Topic not found".into()))?;
        
        // Count posts in topic
        let post_count = query!(
            "SELECT COUNT(*) as count FROM forum_posts WHERE topic_id = ?",
            id
        )
        .fetch_one(&mut tx)
        .await?
        .count;
        
        // Delete all posts in topic
        query!(
            "DELETE FROM forum_posts WHERE topic_id = ?",
            id
        )
        .execute(&mut tx)
        .await?;
        
        // Delete topic
        query!(
            "DELETE FROM forum_topics WHERE id = ?",
            id
        )
        .execute(&mut tx)
        .await?;
        
        // Update category counts
        query!(
            r#"
            UPDATE forum_categories
            SET topic_count = topic_count - 1,
                post_count = post_count - ?,
                updated_at = ?
            WHERE id = ?
            "#,
            post_count, Utc::now(), topic.category_id
        )
        .execute(&mut tx)
        .await?;
        
        // Commit transaction
        tx.commit().await?;
        
        Ok(())
    }
    
    pub async fn increment_view_count(&self, id: i64) -> Result<(), DbError> {
        query!(
            r#"
            UPDATE forum_topics
            SET view_count = view_count + 1
            WHERE id = ?
            "#,
            id
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    pub async fn update_last_post(
        &self,
        id: i64,
        post_id: i64,
        user_id: i64,
        post_time: DateTime<Utc>,
    ) -> Result<(), DbError> {
        // Get the username
        let username = query!(
            "SELECT username FROM users WHERE id = ?",
            user_id
        )
        .fetch_one(&self.pool)
        .await?
        .username;
        
        query!(
            r#"
            UPDATE forum_topics
            SET last_post_id = ?,
                last_poster_id = ?,
                last_poster_name = ?,
                last_post_at = ?,
                updated_at = ?
            WHERE id = ?
            "#,
            post_id, user_id, username, post_time, Utc::now(), id
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    pub async fn count(&self) -> Result<i64, DbError> {
        let count = query!(
            "SELECT COUNT(*) as count FROM forum_topics"
        )
        .fetch_one(&self.pool)
        .await?
        .count;
        
        Ok(count)
    }
    
    pub async fn search(&self, query: &str, page: usize, per_page: usize) -> Result<Vec<Topic>, DbError> {
        let offset = (page - 1) * per_page;
        let search_term = format!("%{}%", query);
        
        let topics = query_as!(
            Topic,
            r#"
            SELECT 
                t.id, t.title, t.slug, t.category_id, t.user_id,
                t.pinned, t.locked, t.view_count,
                t.created_at as "created_at: DateTime<Utc>",
                t.updated_at as "updated_at: DateTime<Utc>",
                t.last_post_at as "last_post_at: Option<DateTime<Utc>>",
                t.last_post_id, t.last_poster_id, t.last_poster_name,
                c.name as category_name,
                u.username as author_name,
                (SELECT COUNT(*) FROM forum_posts WHERE topic_id = t.id) - 1 as reply_count,
                (SELECT content FROM forum_posts WHERE topic_id = t.id ORDER BY created_at LIMIT 1) as excerpt
            FROM forum_topics t
            JOIN forum_categories c ON t.category_id = c.id
            JOIN users u ON t.user_id = u.id
            WHERE t.title LIKE ?
               OR t.id IN (SELECT topic_id FROM forum_posts WHERE content LIKE ?)
            ORDER BY t.last_post_at DESC NULLS LAST
            LIMIT ? OFFSET ?
            "#,
            search_term, search_term, per_page as i64, offset as i64
        )
        .fetch_all(&self.pool)
        .await?;
        
        Ok(topics)
    }
    
    pub async fn get_by_tag(&self, tag_name: &str, page: usize, per_page: usize) -> Result<Vec<Topic>, DbError> {
        let offset = (page - 1) * per_page;
        
        let topics = query_as!(
            Topic,
            r#"
            SELECT 
                t.id, t.title, t.slug, t.category_id, t.user_id,
                t.pinned, t.locked, t.view_count,
                t.created_at as "created_at: DateTime<Utc>",
                t.updated_at as "updated_at: DateTime<Utc>",
                t.last_post_at as "last_post_at: Option<DateTime<Utc>>",
                t.last_post_id, t.last_poster_id, t.last_poster_name,
                c.name as category_name,
                u.username as author_name,
                (SELECT COUNT(*) FROM forum_posts WHERE topic_id = t.id) - 1 as reply_count,
                (SELECT content FROM forum_posts WHERE topic_id = t.id ORDER BY created_at LIMIT 1) as excerpt
            FROM forum_topics t
            JOIN forum_categories c ON t.category_id = c.id
            JOIN users u ON t.user_id = u.id
            JOIN forum_topic_tags tt ON t.id = tt.topic_id
            JOIN forum_tags tag ON tt.tag_id = tag.id
            WHERE tag.name = ?
            ORDER BY t.last_post_at DESC NULLS LAST
            LIMIT ? OFFSET ?
            "#,
            tag_name, per_page as i64, offset as i64
        )
        .fetch_all(&self.pool)
        .await?;
        
        Ok(topics)
    }
    
    pub async fn get_updated_since(&self, timestamp: i64) -> Result<Vec<Topic>, DbError> {
        let since = DateTime::<Utc>::from_timestamp(timestamp, 0)
            .ok_or_else(|| DbError::InvalidInput("Invalid timestamp".into()))?;
        
        let topics = query_as!(
            Topic,
            r#"
            SELECT 
                t.id, t.title, t.slug, t.category_id, t.user_id,
                t.pinned, t.locked, t.view_count,
                t.created_at as "created_at: DateTime<Utc>",
                t.updated_at as "updated_at: DateTime<Utc>",
                t.last_post_at as "last_post_at: Option<DateTime<Utc>>",
                t.last_post_id, t.last_poster_id, t.last_poster_name,
                c.name as category_name,
                u.username as author_name,
                (SELECT COUNT(*) FROM forum_posts WHERE topic_id = t.id) - 1 as reply_count,
                (SELECT content FROM forum_posts WHERE topic_id = t.id ORDER BY created_at LIMIT 1) as excerpt
            FROM forum_topics t
            JOIN forum_categories c ON t.category_id = c.id
            JOIN users u ON t.user_id = u.id
            WHERE t.updated_at > ?
            ORDER BY t.updated_at DESC
            "#,
            since
        )
        .fetch_all(&self.pool)
        .await?;
        
        Ok(topics)
    }
}