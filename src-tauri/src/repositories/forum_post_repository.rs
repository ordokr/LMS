use sqlx::{Pool, Sqlite, Transaction, query, query_as};
use chrono::{DateTime, Utc};

use crate::db::DbError;
use crate::models::forum::Post;

pub struct ForumPostRepository {
    pool: Pool<Sqlite>,
}

impl ForumPostRepository {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }
    
    pub async fn get_by_id(&self, id: i64) -> Result<Option<Post>, DbError> {
        let post = query_as!(
            Post,
            r#"
            SELECT 
                p.id, p.topic_id, p.user_id, p.content, p.parent_id,
                p.created_at as "created_at: DateTime<Utc>",
                p.updated_at as "updated_at: DateTime<Utc>",
                p.is_solution,
                u.username as author_name,
                COALESCE(u.role, 'user') as author_role,
                (SELECT COUNT(*) FROM forum_post_likes WHERE post_id = p.id) as like_count
            FROM forum_posts p
            JOIN users u ON p.user_id = u.id
            WHERE p.id = ?
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;
        
        Ok(post)
    }
    
    pub async fn get_by_topic_id(&self, topic_id: i64, page: usize, per_page: usize) -> Result<Vec<Post>, DbError> {
        let offset = (page - 1) * per_page;
        
        let posts = query_as!(
            Post,
            r#"
            SELECT 
                p.id, p.topic_id, p.user_id, p.content, p.parent_id,
                p.created_at as "created_at: DateTime<Utc>",
                p.updated_at as "updated_at: DateTime<Utc>",
                p.is_solution,
                u.username as author_name,
                COALESCE(u.role, 'user') as author_role,
                (SELECT COUNT(*) FROM forum_post_likes WHERE post_id = p.id) as like_count
            FROM forum_posts p
            JOIN users u ON p.user_id = u.id
            WHERE p.topic_id = ?
            ORDER BY p.created_at
            LIMIT ? OFFSET ?
            "#,
            topic_id, per_page as i64, offset as i64
        )
        .fetch_all(&self.pool)
        .await?;
        
        Ok(posts)
    }
    
    pub async fn create_with_tx(
        &self,
        tx: &mut Transaction<'_, Sqlite>,
        topic_id: i64,
        user_id: i64,
        content: &str,
        parent_id: Option<i64>,
    ) -> Result<Post, DbError> {
        let now = Utc::now();
        
        let id = query!(
            r#"
            INSERT INTO forum_posts (
                topic_id, user_id, content, parent_id,
                created_at, updated_at, is_solution
            )
            VALUES (?, ?, ?, ?, ?, ?, FALSE)
            RETURNING id
            "#,
            topic_id, user_id, content, parent_id, now, now
        )
        .fetch_one(tx)
        .await?
        .id;
        
        // Get topic's category_id
        let category_id = query!(
            "SELECT category_id FROM forum_topics WHERE id = ?",
            topic_id
        )
        .fetch_one(tx)
        .await?
        .category_id;
        
        // Increment forum_categories post_count
        query!(
            r#"
            UPDATE forum_categories
            SET post_count = post_count + 1,
                updated_at = ?
            WHERE id = ?
            "#,
            now, category_id
        )
        .execute(tx)
        .await?;
        
        // Create a minimal post to return
        // The full post with calculated fields will be retrieved later
        let username = query!(
            "SELECT username FROM users WHERE id = ?",
            user_id
        )
        .fetch_one(tx)
        .await?
        .username;
        
        let role = query!(
            "SELECT role FROM users WHERE id = ?",
            user_id
        )
        .fetch_one(tx)
        .await?
        .role
        .unwrap_or_else(|| "user".to_string());
        
        let post = Post {
            id,
            topic_id,
            user_id,
            content: content.to_string(),
            parent_id,
            created_at: now,
            updated_at: now,
            is_solution: false,
            author_name: username,
            author_role: role,
            like_count: 0,
        };
        
        Ok(post)
    }
    
    pub async fn create(
        &self,
        topic_id: i64,
        user_id: i64,
        content: &str,
        parent_id: Option<i64>,
    ) -> Result<Post, DbError> {
        // Start transaction
        let mut tx = self.pool.begin().await?;
        
        let post = self.create_with_tx(
            &mut tx,
            topic_id,
            user_id,
            content,
            parent_id,
        ).await?;
        
        // Update topic's last post information
        query!(
            r#"
            UPDATE forum_topics
            SET last_post_id = ?,
                last_poster_id = ?,
                last_poster_name = (SELECT username FROM users WHERE id = ?),
                last_post_at = ?,
                updated_at = ?
            WHERE id = ?
            "#,
            post.id, user_id, user_id, post.created_at, post.created_at, topic_id
        )
        .execute(&mut tx)
        .await?;
        
        // Commit transaction
        tx.commit().await?;
        
        // Fetch the full post with calculated fields
        self.get_by_id(post.id)
            .await?
            .ok_or_else(|| DbError::NotFound("Post not found after creation".into()))
    }
    
    pub async fn update(
        &self,
        id: i64,
        content: &str,
    ) -> Result<Post, DbError> {
        let now = Utc::now();
        
        query!(
            r#"
            UPDATE forum_posts
            SET content = ?, updated_at = ?
            WHERE id = ?
            "#,
            content, now, id
        )
        .execute(&self.pool)
        .await?;
        
        self.get_by_id(id)
            .await?
            .ok_or_else(|| DbError::NotFound("Post not found after update".into()))
    }
    
    pub async fn delete(&self, id: i64) -> Result<(), DbError> {
        // Get post details
        let post = self.get_by_id(id).await?
            .ok_or_else(|| DbError::NotFound("Post not found".into()))?;
        
        // Check if this is the initial post of a topic
        let is_initial_post = query!(
            r#"
            SELECT MIN(id) as first_post_id FROM forum_posts
            WHERE topic_id = ?
            "#,
            post.topic_id
        )
        .fetch_one(&self.pool)
        .await?
        .first_post_id == id;
        
        // Start transaction
        let mut tx = self.pool.begin().await?;
        
        if is_initial_post {
            // If this is the initial post, delete the entire topic
            query!(
                "DELETE FROM forum_topics WHERE id = ?",
                post.topic_id
            )
            .execute(&mut tx)
            .await?;
            
            // Delete all posts
            query!(
                "DELETE FROM forum_posts WHERE topic_id = ?",
                post.topic_id
            )
            .execute(&mut tx)
            .await?;
            
            // Get topic's category_id
            let category_id = query!(
                "SELECT category_id FROM forum_topics WHERE id = ?",
                post.topic_id
            )
            .fetch_one(&mut tx)
            .await?
            .category_id;
            
            // Update category counts
            let post_count = query!(
                "SELECT COUNT(*) as count FROM forum_posts WHERE topic_id = ?",
                post.topic_id
            )
            .fetch_one(&mut tx)
            .await?
            .count;
            
            query!(
                r#"
                UPDATE forum_categories
                SET topic_count = topic_count - 1,
                    post_count = post_count - ?,
                    updated_at = ?
                WHERE id =// filepath: c:\Users\Tim\Desktop\LMS\src-tauri\src\repositories\forum_post_repository.rs
use sqlx::{Pool, Sqlite, Transaction, query, query_as};
use chrono::{DateTime, Utc};

use crate::db::DbError;
use crate::models::forum::Post;

pub struct ForumPostRepository {
    pool: Pool<Sqlite>,
}

impl ForumPostRepository {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }
    
    pub async fn get_by_id(&self, id: i64) -> Result<Option<Post>, DbError> {
        let post = query_as!(
            Post,
            r#"
            SELECT 
                p.id, p.topic_id, p.user_id, p.content, p.parent_id,
                p.created_at as "created_at: DateTime<Utc>",
                p.updated_at as "updated_at: DateTime<Utc>",
                p.is_solution,
                u.username as author_name,
                COALESCE(u.role, 'user') as author_role,
                (SELECT COUNT(*) FROM forum_post_likes WHERE post_id = p.id) as like_count
            FROM forum_posts p
            JOIN users u ON p.user_id = u.id
            WHERE p.id = ?
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;
        
        Ok(post)
    }
    
    pub async fn get_by_topic_id(&self, topic_id: i64, page: usize, per_page: usize) -> Result<Vec<Post>, DbError> {
        let offset = (page - 1) * per_page;
        
        let posts = query_as!(
            Post,
            r#"
            SELECT 
                p.id, p.topic_id, p.user_id, p.content, p.parent_id,
                p.created_at as "created_at: DateTime<Utc>",
                p.updated_at as "updated_at: DateTime<Utc>",
                p.is_solution,
                u.username as author_name,
                COALESCE(u.role, 'user') as author_role,
                (SELECT COUNT(*) FROM forum_post_likes WHERE post_id = p.id) as like_count
            FROM forum_posts p
            JOIN users u ON p.user_id = u.id
            WHERE p.topic_id = ?
            ORDER BY p.created_at
            LIMIT ? OFFSET ?
            "#,
            topic_id, per_page as i64, offset as i64
        )
        .fetch_all(&self.pool)
        .await?;
        
        Ok(posts)
    }
    
    pub async fn create_with_tx(
        &self,
        tx: &mut Transaction<'_, Sqlite>,
        topic_id: i64,
        user_id: i64,
        content: &str,
        parent_id: Option<i64>,
    ) -> Result<Post, DbError> {
        let now = Utc::now();
        
        let id = query!(
            r#"
            INSERT INTO forum_posts (
                topic_id, user_id, content, parent_id,
                created_at, updated_at, is_solution
            )
            VALUES (?, ?, ?, ?, ?, ?, FALSE)
            RETURNING id
            "#,
            topic_id, user_id, content, parent_id, now, now
        )
        .fetch_one(tx)
        .await?
        .id;
        
        // Get topic's category_id
        let category_id = query!(
            "SELECT category_id FROM forum_topics WHERE id = ?",
            topic_id
        )
        .fetch_one(tx)
        .await?
        .category_id;
        
        // Increment forum_categories post_count
        query!(
            r#"
            UPDATE forum_categories
            SET post_count = post_count + 1,
                updated_at = ?
            WHERE id = ?
            "#,
            now, category_id
        )
        .execute(tx)
        .await?;
        
        // Create a minimal post to return
        // The full post with calculated fields will be retrieved later
        let username = query!(
            "SELECT username FROM users WHERE id = ?",
            user_id
        )
        .fetch_one(tx)
        .await?
        .username;
        
        let role = query!(
            "SELECT role FROM users WHERE id = ?",
            user_id
        )
        .fetch_one(tx)
        .await?
        .role
        .unwrap_or_else(|| "user".to_string());
        
        let post = Post {
            id,
            topic_id,
            user_id,
            content: content.to_string(),
            parent_id,
            created_at: now,
            updated_at: now,
            is_solution: false,
            author_name: username,
            author_role: role,
            like_count: 0,
        };
        
        Ok(post)
    }
    
    pub async fn create(
        &self,
        topic_id: i64,
        user_id: i64,
        content: &str,
        parent_id: Option<i64>,
    ) -> Result<Post, DbError> {
        // Start transaction
        let mut tx = self.pool.begin().await?;
        
        let post = self.create_with_tx(
            &mut tx,
            topic_id,
            user_id,
            content,
            parent_id,
        ).await?;
        
        // Update topic's last post information
        query!(
            r#"
            UPDATE forum_topics
            SET last_post_id = ?,
                last_poster_id = ?,
                last_poster_name = (SELECT username FROM users WHERE id = ?),
                last_post_at = ?,
                updated_at = ?
            WHERE id = ?
            "#,
            post.id, user_id, user_id, post.created_at, post.created_at, topic_id
        )
        .execute(&mut tx)
        .await?;
        
        // Commit transaction
        tx.commit().await?;
        
        // Fetch the full post with calculated fields
        self.get_by_id(post.id)
            .await?
            .ok_or_else(|| DbError::NotFound("Post not found after creation".into()))
    }
    
    pub async fn update(
        &self,
        id: i64,
        content: &str,
    ) -> Result<Post, DbError> {
        let now = Utc::now();
        
        query!(
            r#"
            UPDATE forum_posts
            SET content = ?, updated_at = ?
            WHERE id = ?
            "#,
            content, now, id
        )
        .execute(&self.pool)
        .await?;
        
        self.get_by_id(id)
            .await?
            .ok_or_else(|| DbError::NotFound("Post not found after update".into()))
    }
    
    pub async fn delete(&self, id: i64) -> Result<(), DbError> {
        // Get post details
        let post = self.get_by_id(id).await?
            .ok_or_else(|| DbError::NotFound("Post not found".into()))?;
        
        // Check if this is the initial post of a topic
        let is_initial_post = query!(
            r#"
            SELECT MIN(id) as first_post_id FROM forum_posts
            WHERE topic_id = ?
            "#,
            post.topic_id
        )
        .fetch_one(&self.pool)
        .await?
        .first_post_id == id;
        
        // Start transaction
        let mut tx = self.pool.begin().await?;
        
        if is_initial_post {
            // If this is the initial post, delete the entire topic
            query!(
                "DELETE FROM forum_topics WHERE id = ?",
                post.topic_id
            )
            .execute(&mut tx)
            .await?;
            
            // Delete all posts
            query!(
                "DELETE FROM forum_posts WHERE topic_id = ?",
                post.topic_id
            )
            .execute(&mut tx)
            .await?;
            
            // Get topic's category_id
            let category_id = query!(
                "SELECT category_id FROM forum_topics WHERE id = ?",
                post.topic_id
            )
            .fetch_one(&mut tx)
            .await?
            .category_id;
            
            // Update category counts
            let post_count = query!(
                "SELECT COUNT(*) as count FROM forum_posts WHERE topic_id = ?",
                post.topic_id
            )
            .fetch_one(&mut tx)
            .await?
            .count;
            
            query!(
                r#"
                UPDATE forum_categories
                SET topic_count = topic_count - 1,
                    post_count = post_count - ?,
                    updated_at = ?
                WHERE id =