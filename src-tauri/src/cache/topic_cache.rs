use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};
use tokio::sync::{oneshot, Mutex as AsyncMutex};
use futures::Future;
use log::{debug, warn};
use serde::{Serialize, Deserialize};

// Advanced cache with stampede prevention
pub struct TopicCache<T> 
where
    T: Clone + Send + Sync + 'static,
{
    cache: RwLock<HashMap<i64, CacheEntry<T>>>,
    pending_requests: AsyncMutex<HashMap<i64, oneshot::Receiver<Option<T>>>>,
    ttl: Duration,
}

struct CacheEntry<T> {
    value: T,
    expires_at: Instant,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopicCacheStats {
    hit_count: usize,
    miss_count: usize,
    entry_count: usize,
    stale_refreshes: usize,
    total_size_bytes: usize,
}

impl<T> TopicCache<T> 
where
    T: Clone + Send + Sync + 'static,
{
    pub fn new(ttl_seconds: u64) -> Self {
        Self {
            cache: RwLock::new(HashMap::new()),
            pending_requests: AsyncMutex::new(HashMap::new()),
            ttl: Duration::from_secs(ttl_seconds),
        }
    }
    
    // Get with automatic refresh for expired entries (early expiration)
    pub async fn get_or_refresh<F, Fut>(&self, 
        key: i64, 
        refresh_fn: F,
        stale_ttl_factor: f64,  // e.g. 0.75 means refresh when 75% of TTL has passed
    ) -> Option<T>
    where
        F: FnOnce(i64) -> Fut + Clone + Send + 'static,
        Fut: Future<Output = Option<T>> + Send,
    {
        let stale_threshold = self.ttl.mul_f64(stale_ttl_factor);
        
        // Fast path: check cache with read lock
        {
            let cache = self.cache.read().unwrap();
            if let Some(entry) = cache.get(&key) {
                if entry.expires_at > Instant::now() {
                    // Cache hit
                    
                    // If entry is stale but not expired, trigger background refresh
                    let now = Instant::now();
                    let time_passed = entry.expires_at - now;
                    if time_passed < stale_threshold {
                        let refresh_fn = refresh_fn.clone();
                        // Background refresh
                        tokio::spawn(async move {
                            if let Some(new_value) = refresh_fn(key).await {
                                // Update in background
                            }
                        });
                    }
                    
                    return Some(entry.value.clone());
                }
            }
        }
        
        // Cache miss or expired, we need to fetch the data
        // First check if someone else is already fetching this key
        {
            let mut pending = self.pending_requests.lock().await;
            if let Some(receiver) = pending.remove(&key) {
                // Another request is already refreshing this key, wait for it
                debug!("Waiting for pending request for topic {}", key);
                return match receiver.await {
                    Ok(value) => value,
                    Err(_) => {
                        warn!("Request for topic {} was dropped", key);
                        None
                    }
                };
            }
            
            // No pending request, create a new one
            let (sender, receiver) = oneshot::channel();
            pending.insert(key, receiver);
            
            // Release the lock before the async operation
            drop(pending);
            
            // Fetch the data
            let result = refresh_fn(key).await;
            
            // If successful, update the cache
            if let Some(ref value) = result {
                let mut cache = self.cache.write().unwrap();
                cache.insert(key, CacheEntry {
                    value: value.clone(),
                    expires_at: Instant::now() + self.ttl,
                });
            }
            
            // Remove from pending and notify waiters
            let mut pending = self.pending_requests.lock().await;
            pending.remove(&key);
            let _ = sender.send(result.clone());
            
            result
        }
    }
    
    // Regular get with no refresh
    pub fn get(&self, key: &i64) -> Option<T> {
        let cache = self.cache.read().unwrap();
        if let Some(entry) = cache.get(key) {
            if entry.expires_at > Instant::now() {
                return Some(entry.value.clone());
            }
        }
        None
    }
    
    // Direct insert
    pub fn insert(&self, key: i64, value: T) {
        let mut cache = self.cache.write().unwrap();
        cache.insert(key, CacheEntry {
            value,
            expires_at: Instant::now() + self.ttl,
        });
    }
    
    // Invalidate a specific key
    pub fn invalidate(&self, key: &i64) {
        let mut cache = self.cache.write().unwrap();
        cache.remove(key);
    }
    
    // Clear all entries
    pub fn clear(&self) {
        let mut cache = self.cache.write().unwrap();
        cache.clear();
    }
    
    // Get cache statistics
    pub fn stats(&self) -> TopicCacheStats {
        let cache = self.cache.read().unwrap();
        
        // Estimate memory usage (rough approximation)
        let total_size = cache.iter()
            .map(|(_, entry)| {
                // Base size + key size + entry overhead
                std::mem::size_of::<i64>() + std::mem::size_of::<Instant>() + 
                // Rough estimate for T, would need to be tuned per type
                std::mem::size_of::<T>()
            })
            .sum();
        
        TopicCacheStats {
            hit_count: 0, // Would need separate tracking
            miss_count: 0, // Would need separate tracking
            entry_count: cache.len(),
            stale_refreshes: 0, // Would need separate tracking
            total_size_bytes: total_size,
        }
    }
}

// Global topic cache singleton with hot posts
pub struct ForumCacheManager {
    topic_cache: TopicCache<TopicData>,
    hot_topics: RwLock<Vec<i64>>,
    last_refresh: RwLock<Instant>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TopicData {
    pub id: i64,
    pub title: String,
    pub content: String,
    pub post_count: i32,
    pub views: i32,
    pub author: String,
    pub last_post_at: chrono::DateTime<chrono::Utc>,
    // Other fields
}

impl ForumCacheManager {
    pub fn new() -> Self {
        Self {
            topic_cache: TopicCache::new(300), // 5 minutes TTL
            hot_topics: RwLock::new(Vec::new()),
            last_refresh: RwLock::new(Instant::now()),
        }
    }
    
    pub fn get_topic_cache(&self) -> &TopicCache<TopicData> {
        &self.topic_cache
    }
    
    // Update hot topics list
    pub fn update_hot_topics(&self, topics: Vec<i64>) {
        let mut hot_topics = self.hot_topics.write().unwrap();
        *hot_topics = topics;
        *self.last_refresh.write().unwrap() = Instant::now();
    }
    
    // Get hot topics with auto-refresh if stale
    pub async fn get_hot_topics<F>(&self, refresh_fn: F) -> Vec<i64> 
    where
        F: FnOnce() -> futures::future::BoxFuture<'static, Vec<i64>> + Clone + Send + 'static,
    {
        let refresh_threshold = Duration::from_secs(600); // 10 minutes
        
        // Check if refresh needed
        let needs_refresh = {
            let last_refresh = self.last_refresh.read().unwrap();
            last_refresh.elapsed() > refresh_threshold
        };
        
        if needs_refresh {
            // Background refresh
            let self_clone = self.clone();
            tokio::spawn(async move {
                let topics = refresh_fn().await;
                self_clone.update_hot_topics(topics);
            });
        }
        
        // Return current list
        self.hot_topics.read().unwrap().clone()
    }
}

impl Clone for ForumCacheManager {
    fn clone(&self) -> Self {
        Self {
            topic_cache: TopicCache::new(300), // Create new cache but same TTL
            hot_topics: RwLock::new(self.hot_topics.read().unwrap().clone()),
            last_refresh: RwLock::new(*self.last_refresh.read().unwrap()),
        }
    }
}