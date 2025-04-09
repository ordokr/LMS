use sqlx::Error;
use crate::models::{DiscussionTopic, NewDiscussionTopic, UpdateDiscussionTopic};
use async_trait::async_trait;

pub struct DiscussionTopicRepositoryImpl {
    pool: sqlx::PgPool,
}

impl DiscussionTopicRepositoryImpl {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }

    async fn internal_find_by_category_id(&self, category_id: &str, limit: i64, offset: i64) -> Result<Vec<DiscussionTopic>, Error> {
        let pool = self.pool.clone();
        
        let topics = sqlx::query_as!(
            DiscussionTopic,
            r#"
            SELECT id, title, content, author_id, category_id, is_pinned, is_locked, 
                   view_count, reply_count, created_at, updated_at
            FROM discussion_topics
            WHERE category_id = $1
            ORDER BY updated_at DESC
            LIMIT $2 OFFSET $3
            "#,
            category_id,
            limit,
            offset
        )
        .fetch_all(&*pool)
        .await?;
        
        Ok(topics)
    }

    async fn internal_find_by_author_id(&self, author_id: &str, limit: i64, offset: i64) -> Result<Vec<DiscussionTopic>, Error> {
        let pool = self.pool.clone();
        
        let topics = sqlx::query_as!(
            DiscussionTopic,
            r#"
            SELECT id, title, content, author_id, category_id, is_pinned, is_locked, 
                   view_count, reply_count, created_at, updated_at
            FROM discussion_topics
            WHERE author_id = $1
            ORDER BY updated_at DESC
            LIMIT $2 OFFSET $3
            "#,
            author_id,
            limit,
            offset
        )
        .fetch_all(&*pool)
        .await?;
        
        Ok(topics)
    }

    async fn internal_search(&self, query: &str, category_id: Option<&str>, limit: i64, offset: i64) -> Result<Vec<DiscussionTopic>, Error> {
        let pool = self.pool.clone();
        
        let search_query = format!("%{}%", query);
        
        let topics = match category_id {
            Some(cat_id) => {
                sqlx::query_as!(
                    DiscussionTopic,
                    r#"
                    SELECT id, title, content, author_id, category_id, is_pinned, is_locked, 
                           view_count, reply_count, created_at, updated_at
                    FROM discussion_topics
                    WHERE (title ILIKE $1 OR content ILIKE $1)
                    AND category_id = $2
                    ORDER BY updated_at DESC
                    LIMIT $3 OFFSET $4
                    "#,
                    search_query,
                    cat_id,
                    limit,
                    offset
                )
                .fetch_all(&*pool)
                .await?
            },
            None => {
                sqlx::query_as!(
                    DiscussionTopic,
                    r#"
                    SELECT id, title, content, author_id, category_id, is_pinned, is_locked, 
                           view_count, reply_count, created_at, updated_at
                    FROM discussion_topics
                    WHERE title ILIKE $1 OR content ILIKE $1
                    ORDER BY updated_at DESC
                    LIMIT $2 OFFSET $3
                    "#,
                    search_query,
                    limit,
                    offset
                )
                .fetch_all(&*pool)
                .await?
            }
        };
        
        Ok(topics)
    }

    async fn internal_find_recent(&self, since: chrono::DateTime<chrono::Utc>, limit: i64) -> Result<Vec<DiscussionTopic>, Error> {
        let pool = self.pool.clone();
        
        let topics = sqlx::query_as!(
            DiscussionTopic,
            r#"
            SELECT id, title, content, author_id, category_id, is_pinned, is_locked, 
                   view_count, reply_count, created_at, updated_at
            FROM discussion_topics
            WHERE created_at >= $1
            ORDER BY created_at DESC
            LIMIT $2
            "#,
            since,
            limit
        )
        .fetch_all(&*pool)
        .await?;
        
        Ok(topics)
    }

    async fn internal_find_most_active(&self, limit: i64) -> Result<Vec<DiscussionTopic>, Error> {
        let pool = self.pool.clone();
        
        let topics = sqlx::query_as!(
            DiscussionTopic,
            r#"
            SELECT id, title, content, author_id, category_id, is_pinned, is_locked, 
                   view_count, reply_count, created_at, updated_at
            FROM discussion_topics
            ORDER BY reply_count DESC, view_count DESC
            LIMIT $1
            "#,
            limit
        )
        .fetch_all(&*pool)
        .await?;
        
        Ok(topics)
    }
}

#[async_trait]
pub trait DiscussionTopicRepository {
    // Existing methods
    async fn create(&self, topic: &NewDiscussionTopic) -> Result<DiscussionTopic, Error>;
    async fn find_by_id(&self, id: &str) -> Result<Option<DiscussionTopic>, Error>;
    async fn find_all(&self, limit: i64, offset: i64) -> Result<Vec<DiscussionTopic>, Error>;
    async fn update(&self, id: &str, topic: &UpdateDiscussionTopic) -> Result<Option<DiscussionTopic>, Error>;
    async fn delete(&self, id: &str) -> Result<bool, Error>;
    
    // New methods
    async fn find_by_category_id(&self, category_id: &str, limit: i64, offset: i64) -> Result<Vec<DiscussionTopic>, Error>;
    async fn find_by_author_id(&self, author_id: &str, limit: i64, offset: i64) -> Result<Vec<DiscussionTopic>, Error>;
    async fn search(&self, query: &str, category_id: Option<&str>, limit: i64, offset: i64) -> Result<Vec<DiscussionTopic>, Error>;
    async fn find_recent(&self, since: chrono::DateTime<chrono::Utc>, limit: i64) -> Result<Vec<DiscussionTopic>, Error>;
    async fn find_most_active(&self, limit: i64) -> Result<Vec<DiscussionTopic>, Error>;
}

#[async_trait]
impl DiscussionTopicRepository for DiscussionTopicRepositoryImpl {
    // Existing implementations (assuming these already exist)
    async fn create(&self, topic: &NewDiscussionTopic) -> Result<DiscussionTopic, Error> {
        // Existing implementation - you'd need to keep this
        unimplemented!()
    }
    
    async fn find_by_id(&self, id: &str) -> Result<Option<DiscussionTopic>, Error> {
        // Existing implementation - you'd need to keep this
        unimplemented!()
    }
    
    async fn find_all(&self, limit: i64, offset: i64) -> Result<Vec<DiscussionTopic>, Error> {
        // Existing implementation - you'd need to keep this
        unimplemented!()
    }
    
    async fn update(&self, id: &str, topic: &UpdateDiscussionTopic) -> Result<Option<DiscussionTopic>, Error> {
        // Existing implementation - you'd need to keep this
        unimplemented!()
    }
    
    async fn delete(&self, id: &str) -> Result<bool, Error> {
        // Existing implementation - you'd need to keep this
        unimplemented!()
    }
    
    // New implementations
    async fn find_by_category_id(&self, category_id: &str, limit: i64, offset: i64) -> Result<Vec<DiscussionTopic>, Error> {
        self.internal_find_by_category_id(category_id, limit, offset).await
    }
    
    async fn find_by_author_id(&self, author_id: &str, limit: i64, offset: i64) -> Result<Vec<DiscussionTopic>, Error> {
        self.internal_find_by_author_id(author_id, limit, offset).await
    }
    
    async fn search(&self, query: &str, category_id: Option<&str>, limit: i64, offset: i64) -> Result<Vec<DiscussionTopic>, Error> {
        self.internal_search(query, category_id, limit, offset).await
    }
    
    async fn find_recent(&self, since: chrono::DateTime<chrono::Utc>, limit: i64) -> Result<Vec<DiscussionTopic>, Error> {
        self.internal_find_recent(since, limit).await
    }
    
    async fn find_most_active(&self, limit: i64) -> Result<Vec<DiscussionTopic>, Error> {
        self.internal_find_most_active(limit).await
    }
}