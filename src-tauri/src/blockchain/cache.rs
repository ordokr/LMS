use moka::sync::Cache;
use std::sync::Arc;
use std::time::Duration;
use lazy_static::lazy_static;
use crate::models::Certificate;
use crate::blockchain::HybridChain;

// Define certificate cache with TTL of 1 hour
lazy_static! {
    static ref CERT_CACHE: Cache<String, Option<Certificate>> = {
        Cache::builder()
            .time_to_live(Duration::from_secs(3600))
            .build()
    };
}

pub struct CachedVerifier {
    chain: Arc<HybridChain>,
}

impl CachedVerifier {
    pub fn new(chain: Arc<HybridChain>) -> Self {
        Self { chain }
    }
    
    pub async fn get_certificate(&self, user_id: &str, course_id: &str) -> Option<Certificate> {
        // Create a composite key
        let cache_key = format!("{}:{}", user_id, course_id);
        
        // Try to get from cache first
        if let Some(cert) = CERT_CACHE.get(&cache_key) {
            return cert;
        }
        
        // Not in cache, fetch from blockchain
        let cert = self.fetch_from_blockchain(user_id, course_id).await;
        
        // Store in cache (even if None, to prevent repeated lookups)
        CERT_CACHE.insert(cache_key, cert.clone());
        
        cert
    }
    
    async fn fetch_from_blockchain(&self, user_id: &str, course_id: &str) -> Option<Certificate> {
        // In a real implementation, this would be a potentially expensive blockchain lookup
        // For now, just a placeholder implementation
        
        // Simulate blockchain verification latency (20-100ms)
        tokio::time::sleep(Duration::from_millis(50)).await;
        
        // Mock certificate for demonstration
        Some(Certificate {
            id: "cert-123".to_string(),
            user_id: user_id.to_string(),
            course_id: course_id.to_string(),
            issue_date: chrono::Utc::now(),
            metadata: "{}".to_string(),
        })
    }
    
    pub fn invalidate_cache(&self, user_id: &str, course_id: &str) {
        let cache_key = format!("{}:{}", user_id, course_id);
        CERT_CACHE.invalidate(&cache_key);
    }
}