use std::sync::Arc;
use std::time::{Duration, Instant};
use moka::future::{Cache, CacheBuilder};
use serde::{Serialize, Deserialize};
use sqlx::{query_as, query, SqlitePool};
use tokio::sync::RwLock;
use log::{debug, warn, info};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub topic_count: i64, // Denormalized for performance
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Topic {
    pub id: i64,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub category_id: i64,
    pub user_id: i64,
    pub created_at: String,
    pub updated_at: String,
    pub view_count: i64,
    pub reply_count: i64, // Denormalized for performance
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewTopic {
    pub title: String,
    pub slug: String,
    pub content: String,
    pub category_id: i64,
    pub user_id: i64,
}

/// Advanced repository with multi-level caching and denormalization
pub struct OptimizedForumRepository {
    pool: Arc<SqlitePool>,
    
    // Two-tier cache system
    // L1: Hot cache (small, fast)
    category_hot_cache: Cache<i64, Arc<Category>>,
    topic_hot_cache: Cache<i64, Arc<Topic>>,
    
    // L2: Bulk cache (larger, slightly slower)
    categories_cache: RwLock<Vec<Category>>,
    topics_by_category: Cache<i64, Arc<Vec<Topic>>>,
    
    // Cache invalidation tracking
    last_category_update: RwLock<Instant>,
    last_topic_update: RwLock<Instant>,
    
    // Prepared statements
    get_category_stmt: RwLock<Option<sqlx::sqlite::SqliteStatement>>,
    get_topic_stmt: RwLock<Option<sqlx::sqlite::SqliteStatement>>,
    get_topics_by_category_stmt: RwLock<Option<sqlx::sqlite::SqliteStatement>>,
}

impl OptimizedForumRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        // L1 cache config - very fast, limited size
        let category_hot_cache = CacheBuilder::new(100)   // Max 100 categories
            .time_to_live(Duration::from_secs(30))       // 30 second TTL
            .time_to_idle(Duration::from_secs(15))       // 15 second TTI
            .build();
            
        let topic_hot_cache = CacheBuilder::new(1_000)   // Max 1000 topics
            .time_to_live(Duration::from_secs(30))       // 30 second TTL
            .time_to_idle(Duration::from_secs(15))       // 15 second TTI
            .build();
            
        // L2 cache for topics by category
        let topics_by_category = CacheBuilder::new(50)   // Max 50 categories
            .time_to_live(Duration::from_secs(60))       // 60 second TTL
            .build();
            
        Self {
            pool,
            category_hot_cache,
            topic_hot_cache,
            categories_cache: RwLock::new(Vec::new()),
            topics_by_category,
            last_category_update: RwLock::new(Instant::now()),
            last_topic_update: RwLock::new(Instant::now()),
            get_category_stmt: RwLock::new(None),
            get_topic_stmt: RwLock::new(None),
            get_topics_by_category_stmt: RwLock::new(None),
        }
    }
    
    /// Initialize prepared statements
    pub async fn initialize(&self) -> Result<(), sqlx::Error> {
        // Prepare common statements for reuse
        let mut get_category_stmt = self.get_category_stmt.write().await;
        *get_category_stmt = Some(
            sqlx::query("SELECT c.*, COUNT(t.id) as topic_count 
                        FROM categories c 
                        LEFT JOIN topics t ON c.id = t.category_id 
                        WHERE c.id = ? 
                        GROUP BY c.id")
                .prepare(&*self.pool).await?
        );
        
        let mut get_topic_stmt = self.get_topic_stmt.write().await;
        *get_topic_stmt = Some(
            sqlx::query("SELECT t.*, COUNT(r.id) as reply_count 
                        FROM topics t 
                        LEFT JOIN replies r ON t.id = r.topic_id 
                        WHERE t.id = ? 
                        GROUP BY t.id")
                .prepare(&*self.pool).await?
        );
        
        let mut get_topics_by_category_stmt = self.get_topics_by_category_stmt.write().await;
        *get_topics_by_category_stmt = Some(
            sqlx::query("SELECT t.*, COUNT(r.id) as reply_count 
                        FROM topics t 
                        LEFT JOIN replies r ON t.id = r.topic_id 
                        WHERE t.category_id = ? 
                        GROUP BY t.id 
                        ORDER BY t.created_at DESC 
                        LIMIT ? OFFSET ?")
                .prepare(&*self.pool).await?
        );
        
        Ok(())
    }
    
    /// Get all categories with optimized caching
    pub async fn get_categories(&self) -> Result<Vec<Category>, sqlx::Error> {
        // Check if cache needs refresh
        let should_refresh = {
            let categories = self.categories_cache.read().await;
            categories.is_empty() || self.last_category_update.read().await.elapsed() > Duration::from_secs(60)
        };
        
        if should_refresh {
            debug!("Refreshing categories cache");
            
            // Get categories with topic counts in a single query
            let categories = query_as!(
                Category,
                r#"
                SELECT 
                    c.*,
                    COUNT(t.id) as topic_count
                FROM 
                    categories c
                LEFT JOIN 
                    topics t ON c.id = t.category_id
                GROUP BY 
                    c.id
                ORDER BY 
                    c.name
                "#
            )
            .fetch_all(&*self.pool)
            .await?;
            
            // Update cache
            {
                let mut categories_cache = self.categories_cache.write().await;
                *categories_cache = categories.clone();
                let mut last_update = self.last_category_update.write().await;
                *last_update = Instant::now();
            }
            
            // Also update individual category hot cache
            for category in &categories {
                self.category_hot_cache.insert(category.id, Arc::new(category.clone())).await;
            }
            
            Ok(categories)
        } else {
            debug!("Serving categories from cache");
            let categories = self.categories_cache.read().await;
            Ok(categories.clone())
        }
    }
    
    /// Get a single category by ID with efficient caching
    pub async fn get_category(&self, id: i64) -> Result<Category, sqlx::Error> {
        // Try L1 cache first
        if let Some(category) = self.category_hot_cache.get(&id).await {
            debug!("Category {} served from hot cache", id);
            return Ok(category.as_ref().clone());
        }
        
        // Try L2 cache
        {
            let categories = self.categories_cache.read().await;
            if let Some(category) = categories.iter().find(|c| c.id == id) {
                // Found in L2 cache, promote to L1
                let category_clone = category.clone();
                self.category_hot_cache.insert(id, Arc::new(category_clone.clone())).await;
                debug!("Category {} served from bulk cache and promoted", id);
                return Ok(category_clone);
            }
        }
        
        // Not in cache, fetch from DB using prepared statement if available
        debug!("Category {} cache miss, fetching from database", id);
        let category = if let Some(stmt) = &*self.get_category_stmt.read().await {
            query_as!(
                Category,
                "SELECT c.*, COUNT(t.id) as topic_count 
                FROM categories c 
                LEFT JOIN topics t ON c.id = t.category_id 
                WHERE c.id = ? 
                GROUP BY c.id",
                id
            )
            .fetch_optional(&*self.pool)
            .await?
            .ok_or_else(|| sqlx::Error::RowNotFound)?
        } else {
            // Fallback if prepared statement not available
            query_as!(
                Category,
                "SELECT c.*, COUNT(t.id) as topic_count 
                FROM categories c 
                LEFT JOIN topics t ON c.id = t.category_id 
                WHERE c.id = ? 
                GROUP BY c.id",
                id
            )
            .fetch_optional(&*self.pool)
            .await?
            .ok_or_else(|| sqlx::Error::RowNotFound)?
        };
        
        // Update hot cache
        self.category_hot_cache.insert(id, Arc::new(category.clone())).await;
        
        Ok(category)
    }
    
    /// Get topics by category with efficient pagination and caching
    pub async fn get_topics_by_category(&self, category_id: i64, page: i64, per_page: i64) -> Result<Vec<Topic>, sqlx::Error> {
        let offset = (page - 1) * per_page;
        
        // Generate cache key for this specific pagination request
        let cache_key = category_id;
        
        // Check cache first (only for first page to ensure freshness of most visible content)
        if page == 1 {
            if let Some(topics) = self.topics_by_category.get(&cache_key).await {
                debug!("Topics for category {} served from cache", category_id);
                return Ok(topics.as_ref().clone());
            }
        }
        
        // Not in cache or not first page, fetch from DB
        debug!("Topics for category {} page {} cache miss, fetching from database", category_id, page);
        
        let topics = query_as!(
            Topic,
            r#"
            SELECT 
                t.*,
                COUNT(r.id) as reply_count
            FROM 
                topics t
            LEFT JOIN 
                replies r ON t.id = r.topic_id
            WHERE 
                t.category_id = ?
            GROUP BY 
                t.id
            ORDER BY 
                t.created_at DESC
            LIMIT ? OFFSET ?
            "#,
            category_id,
            per_page,
            offset
        )
        .fetch_all(&*self.pool)
        .await?;
        
        // Cache first page results
        if page == 1 {
            self.topics_by_category.insert(cache_key, Arc::new(topics.clone())).await;
        }
        
        // Also update individual topic hot cache
        for topic in &topics {
            self.topic_hot_cache.insert(topic.id, Arc::new(topic.clone())).await;
        }
        
        Ok(topics)
    }
    
    /// Get a single topic with efficient caching
    pub async fn get_topic(&self, id: i64) -> Result<Topic, sqlx::Error> {
        // Try hot cache first
        if let Some(topic) = self.topic_hot_cache.get(&id).await {
            debug!("Topic {} served from hot cache", id);
            return Ok(topic.as_ref().clone());
        }
        
        // Not in cache, fetch from DB
        debug!("Topic {} cache miss, fetching from database", id);
        
        // Use prepared statement if available
        let topic = if let Some(stmt) = &*self.get_topic_stmt.read().await {
            query_as!(
                Topic,
                "SELECT t.*, COUNT(r.id) as reply_count 
                FROM topics t 
                LEFT JOIN replies r ON t.id = r.topic_id 
                WHERE t.id = ? 
                GROUP BY t.id",
                id
            )
            .fetch_optional(&*self.pool)
            .await?
            .ok_or_else(|| sqlx::Error::RowNotFound)?
        } else {
            // Fallback query
            query_as!(
                Topic,
                "SELECT t.*, COUNT(r.id) as reply_count 
                FROM topics t 
                LEFT JOIN replies r ON t.id = r.topic_id 
                WHERE t.id = ? 
                GROUP BY t.id",
                id
            )
            .fetch_optional(&*self.pool)
            .await?
            .ok_or_else(|| sqlx::Error::RowNotFound)?
        };
            
        // Update view count atomically in background
        let pool = self.pool.clone();
        let topic_id = topic.id;
        tokio::spawn(async move {
            if let Err(e) = query!(
                "UPDATE topics SET view_count = view_count + 1 WHERE id = ?",
                topic_id
            )
            .execute(&*pool)
            .await {
                warn!("Failed to update view count for topic {}: {}", topic_id, e);
            }
        });
        
        // Update hot cache
        self.topic_hot_cache.insert(id, Arc::new(topic.clone())).await;
        
        Ok(topic)
    }
    
    /// Create a new topic with optimized transaction and cache invalidation
    pub async fn create_topic(&self, new_topic: NewTopic) -> Result<Topic, sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        
        let now = chrono::Utc::now().to_rfc3339();
        
        // Insert the topic
        let topic_id = query!(
            r#"
            INSERT INTO topics (title, slug, content, category_id, user_id, created_at, updated_at, view_count)
            VALUES (?, ?, ?, ?, ?, ?, ?, 0)
            "#,
            new_topic.title,
            new_topic.slug,
            new_topic.content,
            new_topic.category_id,
            new_topic.user_id,
            now,
            now,
        )
        .execute(&mut *tx)
        .await?
        .last_insert_rowid();
        
        // Commit transaction
        tx.commit().await?;
        
        // Invalidate caches
        *self.last_topic_update.write().await = Instant::now();
        self.topics_by_category.invalidate(&new_topic.category_id).await;
        
        // Get the created topic to return
        let topic = self.get_topic(topic_id).await?;
        
        Ok(topic)
    }
    
    // Force refresh all caches
    pub async fn refresh_caches(&self) -> Result<(), sqlx::Error> {
        info!("Refreshing all forum caches");
        
        // Refresh categories
        let categories = query_as!(
            Category,
            r#"
            SELECT 
                c.*,
                COUNT(t.id) as topic_count
            FROM 
                categories c
            LEFT JOIN 
                topics t ON c.id = t.category_id
            GROUP BY 
                c.id
            ORDER BY 
                c.name
            "#
        )
        .fetch_all(&*self.pool)
        .await?;
        
        // Update cache
        {
            let mut categories_cache = self.categories_cache.write().await;
            *categories_cache = categories.clone();
            let mut last_update = self.last_category_update.write().await;
            *last_update = Instant::now();
        }
        
        // Also update individual category hot cache
        for category in &categories {
            self.category_hot_cache.insert(category.id, Arc::new(category.clone())).await;
            
            // And refresh topics for each category (first page only)
            let topics = self.get_topics_by_category(category.id, 1, 20).await?;
            self.topics_by_category.insert(category.id, Arc::new(topics)).await;
        }
        
        info!("All forum caches refreshed");
        Ok(())
    }
}