use leptos::*;
use std::hash::Hash;
use serde::{de::DeserializeOwned, Serialize};
use gloo_storage::LocalStorage;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

// Optimized resource loading pattern with caching, suspense and retry logic
#[hook]
pub fn use_cached_resource<T, S, Fu, Fn>(
    source: S,
    fetcher: Fn,
    options: ResourceOptions,
) -> Resource<T>
where
    S: Clone + Hash + 'static,
    T: Clone + DeserializeOwned + Serialize + 'static,
    Fu: Future<Output = Result<T, String>> + 'static,
    Fn: Fn(S) -> Fu + 'static,
{
    let (is_fresh, set_is_fresh) = create_signal(false);
    
    let resource = create_resource(
        move || (source.clone(), is_fresh.get()),
        move |(source, _)| {
            let options = options.clone();
            let fetcher = fetcher.clone();
            
            async move {
                let cache_key = format!("resource_cache:{:?}", std::hash::Hash::hash(&source));
                
                // Try to get from cache first if enabled
                if options.use_cache {
                    if let Ok(CachedData { data, timestamp }) = LocalStorage::get::<CachedData<T>>(&cache_key) {
                        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
                        
                        // Check if cache is still valid
                        if now - timestamp < options.cache_ttl_seconds {
                            set_is_fresh.set(false);
                            
                            // Return cached data immediately
                            if !options.refresh_cache {
                                return Ok(data);
                            }
                            
                            // Update in background if refresh_cache is true
                            spawn_local({
                                let cache_key = cache_key.clone();
                                let fetcher = fetcher.clone();
                                let source = source.clone();
                                
                                async move {
                                    if let Ok(fresh_data) = fetcher(source).await {
                                        let _ = LocalStorage::set(
                                            &cache_key,
                                            &CachedData {
                                                data: fresh_data,
                                                timestamp: SystemTime::now()
                                                    .duration_since(UNIX_EPOCH)
                                                    .unwrap()
                                                    .as_secs(),
                                            },
                                        );
                                        set_is_fresh.set(true);
                                    }
                                }
                            });
                            
                            return Ok(data);
                        }
                    }
                }
                
                // Fetch with retry logic
                let mut attempts = 0;
                let mut last_error = None;
                
                while attempts < options.max_retries {
                    match fetcher(source.clone()).await {
                        Ok(data) => {
                            // Cache the data if caching is enabled
                            if options.use_cache {
                                let _ = LocalStorage::set(
                                    &cache_key,
                                    &CachedData {
                                        data: data.clone(),
                                        timestamp: SystemTime::now()
                                            .duration_since(UNIX_EPOCH)
                                            .unwrap()
                                            .as_secs(),
                                    },
                                );
                            }
                            
                            set_is_fresh.set(true);
                            return Ok(data);
                        }
                        Err(e) => {
                            attempts += 1;
                            last_error = Some(e);
                            
                            if attempts < options.max_retries {
                                // Exponential backoff
                                let delay = 2u64.pow(attempts as u32) * 100;
                                gloo_timers::future::TimeoutFuture::new(delay as u32).await;
                            }
                        }
                    }
                }
                
                Err(last_error.unwrap_or_else(|| "Failed to load resource".to_string()))
            }
        },
    );
    
    resource
}

#[derive(Clone)]
pub struct ResourceOptions {
    pub use_cache: bool,
    pub cache_ttl_seconds: u64,
    pub refresh_cache: bool,
    pub max_retries: usize,
}

impl Default for ResourceOptions {
    fn default() -> Self {
        Self {
            use_cache: true,
            cache_ttl_seconds: 300, // 5 minutes
            refresh_cache: true,
            max_retries: 3,
        }
    }
}

#[derive(Serialize, Deserialize)]
struct CachedData<T> {
    data: T,
    timestamp: u64,
}