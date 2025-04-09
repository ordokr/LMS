use leptos::*;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};
use wasm_bindgen::prelude::*;

// Cache invalidation strategy
pub struct CacheBuster {
    // Cache version counters for each resource type
    versions: StoredValue<HashMap<String, AtomicU64>>,
    // Time-based cache settings
    ttl_settings: StoredValue<HashMap<String, Duration>>,
    // Last updated timestamps
    last_updated: StoredValue<HashMap<String, Instant>>,
}

impl CacheBuster {
    pub fn new() -> Self {
        let mut ttl_settings = HashMap::new();
        
        // Set default TTLs for different resources
        ttl_settings.insert("categories".to_string(), Duration::from_secs(300));        // 5 minutes
        ttl_settings.insert("topics".to_string(), Duration::from_secs(60));             // 1 minute
        ttl_settings.insert("posts".to_string(), Duration::from_secs(30));              // 30 seconds
        ttl_settings.insert("user_profile".to_string(), Duration::from_secs(600));      // 10 minutes
        ttl_settings.insert("forum_stats".to_string(), Duration::from_secs(300));       // 5 minutes
        
        Self {
            versions: store_value(HashMap::new()),
            ttl_settings: store_value(ttl_settings),
            last_updated: store_value(HashMap::new()),
        }
    }
    
    // Add query parameter for cache busting
    pub fn add_cache_param(&self, url: &str) -> String {
        // Determine the resource type from the URL
        let resource_type = self.get_resource_type_from_url(url);
        let version = self.get_version(&resource_type);
        
        // Add cache buster parameter
        if url.contains('?') {
            format!("{}&_v={}", url, version)
        } else {
            format!("{}?_v={}", url, version)
        }
    }
    
    // Check if cached data should be invalidated
    pub fn should_invalidate(&self, resource_type: &str) -> bool {
        let last_updated = self.last_updated.get_value();
        let ttl_settings = self.ttl_settings.get_value();
        
        if let (Some(last_time), Some(ttl)) = (last_updated.get(resource_type), ttl_settings.get(resource_type)) {
            last_time.elapsed() > *ttl
        } else {
            // If no record, assume it should be invalidated
            true
        }
    }
    
    // Force invalidation of a specific resource type
    pub fn invalidate(&self, resource_type: &str) {
        let versions = self.versions.get_value();
        
        // Increment version counter if it exists
        if let Some(counter) = versions.get(resource_type) {
            counter.fetch_add(1, Ordering::SeqCst);
        } else {
            // Create new counter
            let mut versions = versions.clone();
            versions.insert(resource_type.to_string(), AtomicU64::new(1));
            self.versions.set_value(versions);
        }
        
        // Update last updated time
        let mut last_updated = self.last_updated.get_value();
        last_updated.insert(resource_type.to_string(), Instant::now());
        self.last_updated.set_value(last_updated);
    }
    
    // Set TTL for a specific resource type
    pub fn set_ttl(&self, resource_type: &str, ttl: Duration) {
        let mut ttl_settings = self.ttl_settings.get_value();
        ttl_settings.insert(resource_type.to_string(), ttl);
        self.ttl_settings.set_value(ttl_settings);
    }
    
    // Get current version for a resource type
    fn get_version(&self, resource_type: &str) -> u64 {
        let versions = self.versions.get_value();
        
        if let Some(counter) = versions.get(resource_type) {
            counter.load(Ordering::SeqCst)
        } else {
            // Create new counter
            let mut versions = versions.clone();
            let counter = AtomicU64::new(1);
            let value = counter.load(Ordering::SeqCst);
            versions.insert(resource_type.to_string(), counter);
            self.versions.set_value(versions);
            value
        }
    }
    
    // Determine resource type from URL
    fn get_resource_type_from_url(&self, url: &str) -> String {
        // Extract resource type based on URL patterns
        if url.contains("/api/forum/categories") {
            "categories".to_string()
        } else if url.contains("/api/forum/topics") {
            "topics".to_string()
        } else if url.contains("/api/forum/posts") {
            "posts".to_string()
        } else if url.contains("/api/forum/users") {
            "user_profile".to_string()
        } else if url.contains("/api/forum/stats") {
            "forum_stats".to_string()
        } else {
            // Default to using URL as key
            format!("url:{}", url)
        }
    }
}

// Create global instance
pub static CACHE_BUSTER: Lazy<CacheBuster> = Lazy::new(|| CacheBuster::new());

// Hook to wrap URLs with cache busting parameters
#[hook]
pub fn use_cached_url() -> impl Fn(&str) -> String {
    move |url| {
        CACHE_BUSTER.add_cache_param(url)
    }
}

// Hook to provide cache invalidation function
#[hook]
pub fn use_cache_invalidation() -> impl Fn(&str) {
    move |resource_type| {
        CACHE_BUSTER.invalidate(resource_type);
    }
}