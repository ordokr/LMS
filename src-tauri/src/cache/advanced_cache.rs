use moka::future::Cache;
use std::sync::Arc;
use std::time::{Duration, Instant};
use serde::{Serialize, Deserialize};
use tokio::sync::RwLock;
use std::collections::HashMap;

#[derive(Clone, Debug)]
struct CachedValue<T> {
    value: T,
    inserted_at: Instant,
    access_count: u32,
}

#[derive(Clone, Debug)]
struct HotValue<T> {
    value: T,
    access_count: u32,
}

pub struct AdvancedCache<K, V>
where
    K: Clone + Eq + std::hash::Hash + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    // L1: Hot cache (in-memory, very short TTL, most frequently accessed)
    hot_cache: Cache<K, HotValue<V>>,
    
    // L2: Main cache (in-memory with TTL)
    main_cache: Cache<K, CachedValue<V>>,
    
    // L3: Persistent cache (disk-based for cold items)
    persistent_cache: Arc<RwLock<HashMap<K, V>>>,
    
    // Settings
    promotion_threshold: u32,
    persist_threshold: Duration,
}

impl<K, V> AdvancedCache<K, V>
where
    K: Clone + Eq + std::hash::Hash + Send + Sync + std::fmt::Debug + 'static,
    V: Clone + Send + Sync + 'static + Serialize + for<'de> Deserialize<'de>,
{
    pub fn new(
        hot_capacity: u64,
        main_capacity: u64,
        hot_ttl_secs: u64,
        main_ttl_secs: u64,
    ) -> Self {
        Self {
            hot_cache: Cache::builder()
                .max_capacity(hot_capacity)
                .time_to_live(Duration::from_secs(hot_ttl_secs))
                .build(),
                
            main_cache: Cache::builder()
                .max_capacity(main_capacity)
                .time_to_live(Duration::from_secs(main_ttl_secs))
                .build(),
                
            persistent_cache: Arc::new(RwLock::new(HashMap::new())),
            
            promotion_threshold: 5, // Promote to hot cache after 5 accesses
            persist_threshold: Duration::from_secs(60 * 60), // 1 hour
        }
    }
    
    pub async fn get(&self, key: &K) -> Option<V> {
        // Try hot cache first (fastest)
        if let Some(hot_value) = self.hot_cache.get(key).await {
            // Update access count
            let updated = HotValue {
                value: hot_value.value.clone(),
                access_count: hot_value.access_count + 1,
            };
            self.hot_cache.insert(key.clone(), updated).await;
            return Some(hot_value.value);
        }
        
        // Try main cache next
        if let Some(cached) = self.main_cache.get(key).await {
            let new_access_count = cached.access_count + 1;
            
            // Update with new access count
            let updated = CachedValue {
                value: cached.value.clone(),
                inserted_at: cached.inserted_at,
                access_count: new_access_count,
            };
            self.main_cache.insert(key.clone(), updated.clone()).await;
            
            // Promote to hot cache if accessed frequently enough
            if new_access_count >= self.promotion_threshold {
                self.hot_cache.insert(
                    key.clone(),
                    HotValue {
                        value: cached.value.clone(),
                        access_count: new_access_count,
                    },
                ).await;
            }
            
            return Some(cached.value);
        }
        
        // Finally try persistent cache
        let persistent = self.persistent_cache.read().await;
        if let Some(value) = persistent.get(key) {
            let value_clone = value.clone();
            drop(persistent); // Release the lock before modifying
            
            // Promote back to main cache
            self.main_cache.insert(
                key.clone(),
                CachedValue {
                    value: value_clone.clone(),
                    inserted_at: Instant::now(),
                    access_count: 1,
                },
            ).await;
            
            return Some(value_clone);
        }
        
        None
    }
    
    pub async fn insert(&self, key: K, value: V) {
        // Insert into main cache with tracking
        self.main_cache.insert(
            key.clone(),
            CachedValue {
                value: value.clone(),
                inserted_at: Instant::now(),
                access_count: 0,
            },
        ).await;
    }
    
    pub async fn invalidate(&self, key: &K) {
        self.hot_cache.invalidate(key).await;
        self.main_cache.invalidate(key).await;
        
        let mut persistent = self.persistent_cache.write().await;
        persistent.remove(key);
    }
    
    pub async fn persist_cold_items(&self) {
        let main_entries = self.main_cache.iter().await;
        let now = Instant::now();
        let mut persistent = self.persistent_cache.write().await;
        
        for (key, value) in main_entries {
            // If item is old enough and not frequently accessed
            if now.duration_since(value.inserted_at) > self.persist_threshold 
               && value.access_count < self.promotion_threshold {
                persistent.insert(key.clone(), value.value.clone());
            }
        }
    }
}