use sqlx::{Pool, Sqlite, query, query_as};
use std::sync::Arc;
use log::{debug, error, trace};
use std::time::{Duration, Instant};

pub struct ForumRepository {
    pool: Arc<Pool<Sqlite>>,
    // Add prepared statement cache
    statement_cache: tokio::sync::Mutex<std::collections::HashMap<String, sqlx::query::Query<'static, Sqlite, sqlx::sqlite::SqliteArguments<'static>>>>,
    // Add query timing metrics
    query_times: Arc<tokio::sync::Mutex<Vec<(String, Duration)>>>,
}

impl ForumRepository {
    pub fn new(pool: Arc<Pool<Sqlite>>) -> Self {
        Self {
            pool,
            statement_cache: tokio::sync::Mutex::new(std::collections::HashMap::new()),
            query_times: Arc::new(tokio::sync::Mutex::new(Vec::with_capacity(100))),
        }
    }
    
    // Get cached statement to reduce parsing overhead
    async fn get_statement(&self, key: &str, sql: &'static str) -> sqlx::query::Query<'static, Sqlite, sqlx::sqlite::SqliteArguments<'static>> {
        let mut cache = self.statement_cache.lock().await;
        
        if let Some(stmt) = cache.get(key) {
            return stmt.clone();
        }
        
        // Create new statement
        let stmt = sqlx::query(sql);
        cache.insert(key.to_string(), stmt.clone());
        stmt
    }
    
    // Get topics with optimized query
    pub async fn get_topics_by_category(&self, category_id: i64, page: u32, per_page: u32) -> Result<Vec<Topic>, sqlx::Error> {
        let start = Instant::now();
        
        // Use prepared statement with pagination
        let offset = (page - 1) * per_page;
        let topics = self.get_statement(
            "get_topics_by_category",
            r#"
            SELECT 
                t.id, t.title, t.slug, t.content, t.category_id, 
                t.user_id, t.created_at, t.updated_at,
                c.name as category_name, 
                u.username as author_name,
                (SELECT COUNT(*) FROM posts p WHERE p.topic_id = t.id) as post_count,
                (SELECT MAX(created_at) FROM posts p WHERE p.topic_id = t.id) as last_post_at
            FROM topics t
            JOIN categories c ON t.category_id = c.id
            JOIN users u ON t.user_id = u.id
            WHERE t.category_id = ?
            ORDER BY t.pinned DESC, t.updated_at DESC
            LIMIT ? OFFSET ?
            "#
        )
        .await
        .bind(category_id)
        .bind(per_page)
        .bind(offset)
        .fetch_all(&*self.pool)
        .await
        .map(|rows| {
            rows.into_iter().map(|row| {
                Topic {
                    id: row.get("id"),
                    title: row.get("title"),
                    slug: row.get("slug"),
                    content: row.get("content"),
                    category_id: row.get("category_id"),
                    category_name: row.get("category_name"),
                    user_id: row.get("user_id"),
                    author_name: row.get("author_name"),
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                    post_count: row.get("post_count"),
                    last_post_at: row.get("last_post_at"),
                }
            }).collect()
        });
        
        // Record query time
        let duration = start.elapsed();
        let mut times = self.query_times.lock().await;
        times.push(("get_topics_by_category".to_string(), duration));
        if times.len() > 100 {
            times.remove(0);
        }
        
        if duration > Duration::from_millis(50) {
            debug!("Slow query get_topics_by_category: {:?}", duration);
        }
        
        topics
    }
    
    // Get query performance metrics
    pub async fn get_query_metrics(&self) -> Vec<(String, Duration)> {
        let times = self.query_times.lock().await;
        times.clone()
    }
    
    // Add bulk operations for better performance
    pub async fn bulk_create_topics(&self, topics: Vec<NewTopic>) -> Result<Vec<i64>, sqlx::Error> {
        // Use a transaction for multiple inserts
        let mut tx = self.pool.begin().await?;
        let mut ids = Vec::with_capacity(topics.len());
        
        for topic in topics {
            let id = sqlx::query(
                "INSERT INTO topics (title, slug, content, category_id, user_id, created_at, updated_at, pinned) 
                VALUES (?, ?, ?, ?, ?, datetime('now'), datetime('now'), ?)"
            )
            .bind(topic.title)
            .bind(topic.slug)
            .bind(topic.content)
            .bind(topic.category_id)
            .bind(topic.user_id)
            .bind(topic.pinned.unwrap_or(false))
            .execute(&mut tx)
            .await?
            .last_insert_rowid();
            
            ids.push(id);
        }
        
        tx.commit().await?;
        
        Ok(ids)
    }
}