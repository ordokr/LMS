use meilisearch_sdk::{client::Client, search::SearchResults, indexes::Index};
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use log::{info, error, warn, debug};
use sqlx::SqlitePool;
use futures::future::join_all;

// Models for search
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TopicDocument {
    pub id: i64,
    pub title: String,
    pub content: String,
    pub category_id: i64,
    pub category_name: String,
    pub user_id: i64,
    pub created_at: String,
    pub slug: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CategoryDocument {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub slug: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchOptions {
    pub limit: usize,
    pub offset: usize,
    pub filter: Option<String>,
    pub sort: Option<Vec<String>>,
}

impl Default for SearchOptions {
    fn default() -> Self {
        Self {
            limit: 20,
            offset: 0,
            filter: None,
            sort: None,
        }
    }
}

// Structure to track sync status
#[derive(Debug, Default)]
struct SyncStats {
    last_sync: Option<chrono::DateTime<chrono::Utc>>,
    last_sync_duration: Option<Duration>,
    topics_count: usize,
    categories_count: usize,
    in_progress: bool,
}

pub struct MeiliSearchClient {
    client: Client,
    topics_index: RwLock<Option<Index>>,
    categories_index: RwLock<Option<Index>>,
    pool: Arc<SqlitePool>,
    stats: RwLock<SyncStats>,
    // Cache for frequent queries
    query_cache: tokio::sync::Mutex<lru::LruCache<String, Arc<SearchResults<serde_json::Value>>>>,
}

impl MeiliSearchClient {
    pub fn new(host: &str, api_key: Option<&str>, pool: Arc<SqlitePool>) -> Self {
        let client = if let Some(key) = api_key {
            Client::new(host, key)
        } else {
            Client::new(host, "")
        };

        Self {
            client,
            topics_index: RwLock::new(None),
            categories_index: RwLock::new(None),
            pool,
            stats: RwLock::new(SyncStats::default()),
            query_cache: tokio::sync::Mutex::new(lru::LruCache::new(100)), // Cache 100 most recent queries
        }
    }
    
    pub async fn initialize(&self) -> Result<(), String> {
        debug!("Initializing Meilisearch indexes");
        
        // Set up topics index with optimized settings
        let topics_index = match self.client.get_or_create("topics").await {
            Ok(index) => index,
            Err(e) => return Err(format!("Failed to create topics index: {}", e)),
        };
        
        // Set up categories index
        let categories_index = match self.client.get_or_create("categories").await {
            Ok(index) => index,
            Err(e) => return Err(format!("Failed to create categories index: {}", e)),
        };
        
        // Optimize performance with appropriate settings
        let settings_futures = vec![
            // Topics index settings
            topics_index.set_searchable_attributes(&["title", "content", "category_name", "slug"]),
            topics_index.set_filterable_attributes(&["category_id", "user_id", "created_at"]),
            topics_index.set_sortable_attributes(&["created_at"]),
            topics_index.set_pagination_options(1000, 500), // Optimize for up to 1000 results per page
            topics_index.set_typo_tolerance(true),
            
            // Categories index settings
            categories_index.set_searchable_attributes(&["name", "description", "slug"]),
            categories_index.set_filterable_attributes(&["created_at"]),
            categories_index.set_pagination_options(1000, 100),
        ];
        
        // Apply all settings in parallel for better performance
        let results = join_all(settings_futures).await;
        for result in results {
            if let Err(e) = result {
                warn!("Failed to apply some Meilisearch settings: {}", e);
                // Continue anyway - these are optimizations, not critical failures
            }
        }
        
        // Store indexes
        *self.topics_index.write().await = Some(topics_index);
        *self.categories_index.write().await = Some(categories_index);
        
        debug!("Meilisearch indexes initialized successfully");
        
        Ok(())
    }
    
    // Sync data with optimized batching and parallelism
    pub async fn sync_data(&self, force: bool) -> Result<(), String> {
        // Check if sync already in progress
        {
            let stats = self.stats.read().await;
            if stats.in_progress && !force {
                return Ok(());
            }
            
            // Rate limit syncs unless forced
            if !force && stats.last_sync.is_some() {
                let elapsed = chrono::Utc::now() - stats.last_sync.unwrap();
                if elapsed < chrono::Duration::minutes(10) {
                    return Ok(());
                }
            }
        }
        
        // Mark sync as in progress
        {
            let mut stats = self.stats.write().await;
            stats.in_progress = true;
        }
        
        let start_time = Instant::now();
        info!("Starting optimized Meilisearch data sync");
        
        // Execute sync operations
        let sync_result = self.execute_sync().await;
        
        // Update stats
        {
            let mut stats = self.stats.write().await;
            stats.in_progress = false;
            stats.last_sync = Some(chrono::Utc::now());
            stats.last_sync_duration = Some(start_time.elapsed());
            
            match &sync_result {
                Ok((topics_count, categories_count)) => {
                    stats.topics_count = *topics_count;
                    stats.categories_count = *categories_count;
                    info!(
                        "Meilisearch sync completed in {:?}: {} topics, {} categories indexed", 
                        start_time.elapsed(), topics_count, categories_count
                    );
                },
                Err(e) => {
                    error!("Meilisearch sync failed: {}", e);
                }
            }
        }
        
        // Clear cache after sync
        {
            let mut cache = self.query_cache.lock().await;
            cache.clear();
        }
        
        sync_result.map(|_| ())
    }
    
    async fn execute_sync(&self) -> Result<(usize, usize), String> {
        // Get indexes
        let topics_index = match self.topics_index.read().await.as_ref() {
            Some(index) => index.clone(),
            None => return Err("Topics index not initialized".to_string()),
        };
        
        let categories_index = match self.categories_index.read().await.as_ref() {
            Some(index) => index.clone(),
            None => return Err("Categories index not initialized".to_string()),
        };
        
        // Run sync operations in parallel for better performance
        let (topics_result, categories_result) = tokio::join!(
            self.sync_topics(&topics_index),
            self.sync_categories(&categories_index)
        );
        
        let topics_count = topics_result?;
        let categories_count = categories_result?;
        
        Ok((topics_count, categories_count))
    }
    
    async fn sync_topics(&self, index: &Index) -> Result<usize, String> {
        // Find last modification time to only sync changed topics
        let last_sync_time = {
            let stats = self.stats.read().await;
            stats.last_sync
        };
        
        // Build query to get only updated topics
        let query = if let Some(sync_time) = last_sync_time {
            let formatted_time = sync_time.format("%Y-%m-%d %H:%M:%S").to_string();
            format!(
                r#"
                SELECT 
                    t.id,
                    t.title,
                    t.content,
                    t.category_id,
                    c.name as category_name,
                    t.user_id,
                    t.created_at as "created_at: String",
                    t.slug
                FROM topics t
                JOIN categories c ON t.category_id = c.id
                WHERE t.updated_at > '{}'
                ORDER BY t.id
                "#, 
                formatted_time
            )
        } else {
            // First sync, get all topics
            r#"
            SELECT 
                t.id,
                t.title,
                t.content,
                t.category_id,
                c.name as category_name,
                t.user_id,
                t.created_at as "created_at: String",
                t.slug
            FROM topics t
            JOIN categories c ON t.category_id = c.id
            ORDER BY t.id
            "#.to_string()
        };
        
        // Fetch topics from database
        let topics: Vec<TopicDocument> = match sqlx::query_as(&query)
            .fetch_all(&*self.pool)
            .await {
                Ok(topics) => topics,
                Err(e) => return Err(format!("Failed to fetch topics: {}", e)),
            };
        
        let topics_count = topics.len();
        
        if topics_count == 0 {
            info!("No topics to index");
            return Ok(0);
        }
        
        info!("Indexing {} topics in Meilisearch", topics_count);
        
        // Optimize indexing with batches
        const BATCH_SIZE: usize = 1000;
        let mut futures = Vec::new();
        
        for chunk in topics.chunks(BATCH_SIZE) {
            futures.push(index.add_documents(chunk, Some("id")));
        }
        
        // Process batches in parallel, but with a limit to avoid overwhelming the server
        const PARALLEL_LIMIT: usize = 3;
        for chunk in futures.chunks(PARALLEL_LIMIT) {
            let _ = join_all(chunk.iter().cloned()).await;
        }
        
        Ok(topics_count)
    }
    
    async fn sync_categories(&self, index: &Index) -> Result<usize, String> {
        // Similar pattern as sync_topics, but optimized for categories
        let last_sync_time = {
            let stats = self.stats.read().await;
            stats.last_sync
        };
        
        let query = if let Some(sync_time) = last_sync_time {
            let formatted_time = sync_time.format("%Y-%m-%d %H:%M:%S").to_string();
            format!(
                r#"
                SELECT 
                    id,
                    name,
                    description,
                    slug,
                    created_at as "created_at: String"
                FROM categories
                WHERE updated_at > '{}'
                ORDER BY id
                "#, 
                formatted_time
            )
        } else {
            r#"
            SELECT 
                id,
                name,
                description,
                slug,
                created_at as "created_at: String"
            FROM categories
            ORDER BY id
            "#.to_string()
        };
        
        // Fetch categories
        let categories: Vec<CategoryDocument> = match sqlx::query_as(&query)
            .fetch_all(&*self.pool)
            .await {
                Ok(categories) => categories,
                Err(e) => return Err(format!("Failed to fetch categories: {}", e)),
            };
        
        let categories_count = categories.len();
        
        if categories_count == 0 {
            info!("No categories to index");
            return Ok(0);
        }
        
        info!("Indexing {} categories in Meilisearch", categories_count);
        
        // Categories are typically few, so we can index them in a single batch
        if let Err(e) = index.add_documents(&categories, Some("id")).await {
            return Err(format!("Failed to index categories: {}", e));
        }
        
        Ok(categories_count)
    }
    
    // Search with caching for better performance
    pub async fn search_topics(
        &self, 
        query: &str, 
        options: SearchOptions
    ) -> Result<SearchResults<serde_json::Value>, String> {
        // Build cache key
        let cache_key = format!(
            "topics:{}:{}:{}:{}:{}",
            query,
            options.limit,
            options.offset,
            options.filter.as_deref().unwrap_or(""),
            options.sort.as_ref().map_or("".to_string(), |s| s.join(","))
        );
        
        // Check cache first
        {
            let mut cache = self.query_cache.lock().await;
            if let Some(result) = cache.get(&cache_key) {
                return Ok(result.as_ref().clone());
            }
        }
        
        // Not in cache, perform actual search
        let topics_index = match self.topics_index.read().await.as_ref() {
            Some(index) => index.clone(),
            None => return Err("Topics index not initialized".to_string()),
        };
        
        let mut search_query = topics_index.search();
        search_query.with_query(query)
            .with_limit(options.limit)
            .with_offset(options.offset);
        
        if let Some(filter) = options.filter {
            search_query.with_filter(&filter);
        }
        
        if let Some(sort) = options.sort {
            search_query.with_sort(sort.as_slice());
        }
        
        match search_query.execute::<serde_json::Value>().await {
            Ok(results) => {
                // Store in cache
                let results_arc = Arc::new(results.clone());
                let mut cache = self.query_cache.lock().await;
                cache.put(cache_key, results_arc);
                
                Ok(results)
            },
            Err(e) => Err(format!("Search failed: {}", e)),
        }
    }
    
    // Start background sync with adaptive interval based on database activity
    pub fn start_background_sync(self: Arc<Self>) {
        tokio::spawn(async move {
            let mut adaptive_interval = 600; // Start with 10 minutes (600 seconds)
            let mut consecutive_empty = 0;
            
            loop {
                tokio::time::sleep(Duration::from_secs(adaptive_interval)).await;
                
                // Check if anything changed
                let now = chrono::Utc::now();
                let last_sync = {
                    let stats = self.stats.read().await;
                    stats.last_sync
                };
                
                // Count changes since last sync
                let changes_count = match Self::count_changes_since_last_sync(&self.pool, last_sync).await {
                    Ok(count) => count,
                    Err(e) => {
                        error!("Failed to check for changes: {}", e);
                        0
                    }
                };
                
                if changes_count > 0 {
                    info!("Detected {} changes, syncing Meilisearch", changes_count);
                    consecutive_empty = 0;
                    
                    // Adjust interval based on activity level
                    if changes_count > 100 {
                        // High activity - sync more frequently
                        adaptive_interval = adaptive_interval.saturating_sub(60).max(300); // Min 5 minutes
                    }
                    
                    if let Err(e) = self.sync_data(false).await {
                        error!("Background sync failed: {}", e);
                    }
                } else {
                    consecutive_empty += 1;
                    
                    // If no changes for multiple checks, gradually increase interval
                    if consecutive_empty > 2 {
                        adaptive_interval = (adaptive_interval + 300).min(3600); // Max 1 hour
                    }
                }
            }
        });
    }
    
    // Utility to check database changes since last sync
    async fn count_changes_since_last_sync(
        pool: &SqlitePool, 
        last_sync: Option<chrono::DateTime<chrono::Utc>>
    ) -> Result<i64, sqlx::Error> {
        if let Some(sync_time) = last_sync {
            let formatted_time = sync_time.format("%Y-%m-%d %H:%M:%S").to_string();
            
            // Count topics and categories updated since last sync
            let result = sqlx::query!(
                r#"
                SELECT 
                    (SELECT COUNT(*) FROM topics WHERE updated_at > $1) +
                    (SELECT COUNT(*) FROM categories WHERE updated_at > $1) as count
                "#,
                formatted_time
            )
            .fetch_one(pool)
            .await?;
            
            Ok(result.count)
        } else {
            // No previous sync, return positive number to trigger initial sync
            Ok(1)
        }
    }
    
    // Check health status with timeout
    pub async fn health_check(&self) -> bool {
        // Create a timeout future
        let timeout_future = tokio::time::sleep(Duration::from_secs(3));
        
        // Create the health check future
        let health_future = self.client.health();
        
        // Race the futures
        match tokio::select! {
            result = health_future => Some(result),
            _ = timeout_future => None,
        } {
            Some(Ok(health)) => health.status == "available",
            _ => false,
        }
    }
    
    // Delete document from index
    pub async fn delete_topic(&self, id: i64) -> Result<(), String> {
        let topics_index = match self.topics_index.read().await.as_ref() {
            Some(index) => index.clone(),
            None => return Err("Topics index not initialized".to_string()),
        };
        
        if let Err(e) = topics_index.delete_document(id).await {
            return Err(format!("Failed to delete topic: {}", e));
        }
        
        Ok(())
    }
    
    // Search categories with similar optimizations
    pub async fn search_categories(&self, query: &str, options: SearchOptions) -> Result<SearchResults<CategoryDocument>, String> {
        let categories_index = match self.categories_index.read().await.as_ref() {
            Some(index) => index.clone(),
            None => return Err("Categories index not initialized".to_string()),
        };
        
        let mut search_query = categories_index.search();
        search_query.with_query(query)
            .with_limit(options.limit)
            .with_offset(options.offset);
        
        if let Some(filter) = options.filter {
            if !filter.is_empty() {
                search_query.with_filter(&filter);
            }
        }
        
        if let Some(sort) = options.sort {
            if !sort.is_empty() {
                search_query.with_sort(sort.as_slice());
            }
        }
        
        match search_query.execute::<CategoryDocument>().await {
            Ok(results) => Ok(results),
            Err(e) => Err(format!("Search failed: {}", e)),
        }
    }
}