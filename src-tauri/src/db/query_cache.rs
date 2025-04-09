use std::sync::Arc;
use std::collections::HashMap;
use sqlx::{Sqlite, SqlitePool, query::Query};
use tokio::sync::Mutex;
use std::hash::{Hash, Hasher};
use std::sync::{Arc, Mutex as StdMutex};
use std::time::{Duration, Instant};
use sqlx::{Executor, FromRow, Pool};
use serde::Serialize;
use lru::LruCache;

// Simple query cache for read-heavy operations
pub struct QueryCache {
    cache: StdMutex<LruCache<QueryKey, CachedResult>>,
}

struct QueryKey {
    query: String,
    params_hash: u64,
}

struct CachedResult {
    data: Vec<u8>, // Serialized result
    expiry: Instant,
}

impl QueryKey {
    fn new<T: Serialize>(query: &str, params: &T) -> Result<Self, String> {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        
        // Attempt to serialize and hash the parameters
        let serialized = bincode::serialize(params)
            .map_err(|e| format!("Failed to serialize params: {}", e))?;
            
        serialized.hash(&mut hasher);
        let params_hash = hasher.finish();
        
        Ok(Self {
            query: query.to_string(),
            params_hash,
        })
    }
}

impl PartialEq for QueryKey {
    fn eq(&self, other: &Self) -> bool {
        self.query == other.query && self.params_hash == other.params_hash
    }
}

impl Eq for QueryKey {}

impl Hash for QueryKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.query.hash(state);
        self.params_hash.hash(state);
    }
}

impl QueryCache {
    pub fn new(max_size: usize) -> Self {
        Self {
            cache: StdMutex::new(LruCache::new(max_size)),
        }
    }
    
    // Execute a query with caching for read operations
    pub async fn cached_query<T, P>(
        &self,
        pool: &Pool<Sqlite>,
        query: &str,
        params: P,
        ttl: Duration,
    ) -> Result<Vec<T>, sqlx::Error>
    where
        T: FromRow + Send + Unpin + Serialize + for<'de> serde::de::Deserialize<'de>,
        P: Serialize + Send + Sync,
    {
        // For write operations, bypass cache
        if !query.trim().to_lowercase().starts_with("select") {
            return sqlx::query_as::<_, T>(query)
                .bind(params)
                .fetch_all(pool)
                .await;
        }
        
        // For read operations, try cache first
        let query_key = match QueryKey::new(query, &params) {
            Ok(key) => key,
            Err(_) => {
                // If we can't create a cache key, just execute the query directly
                return sqlx::query_as::<_, T>(query)
                    .bind(params)
                    .fetch_all(pool)
                    .await;
            }
        };
        
        // Check cache
        {
            let mut cache = self.cache.lock().unwrap();
            if let Some(cached) = cache.get(&query_key) {
                if cached.expiry > Instant::now() {
                    // Cache hit
                    return match bincode::deserialize(&cached.data) {
                        Ok(data) => Ok(data),
                        Err(_) => {
                            // Cache corrupted, remove and continue to database
                            cache.pop(&query_key);
                            drop(cache); // Release mutex lock before async call
                            
                            // Execute query
                            sqlx::query_as::<_, T>(query)
                                .bind(params)
                                .fetch_all(pool)
                                .await
                        }
                    };
                } else {
                    // Expired, remove from cache
                    cache.pop(&query_key);
                }
            }
        }
        
        // Cache miss, execute query
        let results = sqlx::query_as::<_, T>(query)
            .bind(params)
            .fetch_all(pool)
            .await?;
            
        // Cache results
        match bincode::serialize(&results) {
            Ok(data) => {
                let cached = CachedResult {
                    data,
                    expiry: Instant::now() + ttl,
                };
                
                let mut cache = self.cache.lock().unwrap();
                cache.put(query_key, cached);
            },
            Err(e) => {
                log::warn!("Failed to cache query results: {}", e);
            }
        }
        
        Ok(results)
    }
    
    // Invalidate cache entries matching a table name
    pub fn invalidate_table(&self, table_name: &str) {
        let mut cache = self.cache.lock().unwrap();
        
        // Collect keys to remove (can't modify while iterating)
        let mut keys_to_remove = Vec::new();
        
        // Simple heuristic - if query contains table name, invalidate it
        // A better approach would be to parse the SQL and check tables properly
        for key in cache.iter().map(|(k, _)| k) {
            if key.query.to_lowercase().contains(&table_name.to_lowercase()) {
                keys_to_remove.push(QueryKey {
                    query: key.query.clone(),
                    params_hash: key.params_hash,
                });
            }
        }
        
        // Remove all matched keys
        for key in keys_to_remove {
            cache.pop(&key);
        }
    }
    
    // Get cache stats
    pub fn get_stats(&self) -> (usize, usize) {
        let cache = self.cache.lock().unwrap();
        (cache.len(), cache.cap())
    }
}

// Create a global query cache
lazy_static::lazy_static! {
    static ref QUERY_CACHE: QueryCache = QueryCache::new(1000); // Cache up to 1000 queries
}

pub fn get_query_cache() -> &'static QueryCache {
    &QUERY_CACHE
}