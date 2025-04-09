use std::future::Future;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use log::info;

pub struct RefreshableCache<K, V, E> 
where
    K: Eq + std::hash::Hash + Clone + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
    E: std::fmt::Debug + Send + Sync + 'static,
{
    data: Arc<RwLock<std::collections::HashMap<K, (V, Instant)>>>,
    ttl: Duration,
    refresh_ahead: Duration,
}

impl<K, V, E> RefreshableCache<K, V, E> 
where
    K: Eq + std::hash::Hash + Clone + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
    E: std::fmt::Debug + Send + Sync + 'static,
{
    pub fn new(ttl_seconds: u64, refresh_ahead_seconds: u64) -> Self {
        Self {
            data: Arc::new(RwLock::new(std::collections::HashMap::new())),
            ttl: Duration::from_secs(ttl_seconds),
            refresh_ahead: Duration::from_secs(refresh_ahead_seconds),
        }
    }
    
    pub async fn get_or_load<F, Fut>(&self, key: K, loader: F) -> Result<V, E>
    where
        F: Fn(K) -> Fut + Send + Sync,
        Fut: Future<Output = Result<V, E>> + Send,
    {
        // Check if we have a valid cached value
        let needs_refresh = {
            let cache = self.data.read().await;
            if let Some((value, timestamp)) = cache.get(&key) {
                let age = Instant::now().duration_since(*timestamp);
                
                // If nearing expiry, initiate background refresh
                if age > self.ttl.saturating_sub(self.refresh_ahead) {
                    true
                } else {
                    return Ok(value.clone());
                }
            } else {
                // Cache miss
                drop(cache);
                let value = loader(key.clone()).await?;
                
                // Insert into cache
                let mut cache = self.data.write().await;
                cache.insert(key, (value.clone(), Instant::now()));
                
                return Ok(value);
            }
        };
        
        if needs_refresh {
            // We have a value but it's nearing expiry, schedule refresh
            let data_clone = self.data.clone();
            let key_clone = key.clone();
            
            tokio::spawn(async move {
                match loader(key_clone.clone()).await {
                    Ok(fresh_value) => {
                        let mut cache = data_clone.write().await;
                        info!("Background refresh successful for {:?}", key_clone);
                        cache.insert(key_clone, (fresh_value, Instant::now()));
                    }
                    Err(e) => {
                        // Just log error, but keep serving old value
                        log::error!("Background refresh failed for {:?}: {:?}", key_clone, e);
                    }
                }
            });
            
            // Return current value while refresh happens in background
            let cache = self.data.read().await;
            let (value, _) = cache.get(&key).unwrap();
            Ok(value.clone())
        } else {
            // Normal cache miss (should not happen due to earlier check)
            let value = loader(key.clone()).await?;
            
            let mut cache = self.data.write().await;
            cache.insert(key, (value.clone(), Instant::now()));
            
            Ok(value)
        }
    }
}