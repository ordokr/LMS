use std::path::Path;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};
use once_cell::sync::Lazy;
use log::{info, warn, error};
use serde::{Serialize, Deserialize};
use crate::search::meilisearch::MeiliSearchClient;
use crate::search::embedded::EmbeddedMeilisearch;
use crate::search::setup::setup_meilisearch;

// Global search state
pub static SEARCH_MANAGER: Lazy<SearchManager> = Lazy::new(|| {
    SearchManager::new()
});

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SearchState {
    Uninitialized,
    Initializing,
    Ready,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchStatus {
    pub state: String,
    pub documents_indexed: usize,
    pub is_available: bool,
    pub last_indexed: Option<String>,
}

pub struct SearchManager {
    state: RwLock<SearchState>,
    client: RwLock<Option<Arc<MeiliSearchClient>>>,
    embedded: Mutex<Option<Arc<EmbeddedMeilisearch>>>,
    indexed_count: RwLock<usize>,
    last_indexed: RwLock<Option<chrono::DateTime<chrono::Utc>>>,
}

impl SearchManager {
    fn new() -> Self {
        Self {
            state: RwLock::new(SearchState::Uninitialized),
            client: RwLock::new(None),
            embedded: Mutex::new(None),
            indexed_count: RwLock::new(0),
            last_indexed: RwLock::new(None),
        }
    }
    
    /// Initialize search on demand - doesn't block if already initializing
    pub async fn initialize_if_needed(&self, app_data_dir: &Path, db_pool: Arc<sqlx::SqlitePool>) -> bool {
        // Fast path: already initialized
        {
            let state = *self.state.read().await;
            if state == SearchState::Ready {
                return true;
            }
            
            // Don't try again if already failed
            if state == SearchState::Failed {
                return false;
            }
            
            // Don't duplicate initialization
            if state == SearchState::Initializing {
                return true;
            }
        }
        
        // Try to initialize
        match self.initialize(app_data_dir, db_pool).await {
            Ok(_) => true,
            Err(_) => false,
        }
    }
    
    /// Initialize search system
    async fn initialize(&self, app_data_dir: &Path, db_pool: Arc<sqlx::SqlitePool>) -> Result<(), String> {
        // Update state
        *self.state.write().await = SearchState::Initializing;
        
        info!("Initializing search system...");
        
        // Setup embedded Meilisearch
        let embedded_meili = match setup_meilisearch(app_data_dir).await {
            Ok(meili) => Arc::new(meili),
            Err(e) => {
                error!("Failed to setup embedded Meilisearch: {}", e);
                *self.state.write().await = SearchState::Failed;
                return Err(e);
            }
        };
        
        // Start embedded Meilisearch
        let meili_config = match embedded_meili.start().await {
            Ok(config) => config,
            Err(e) => {
                warn!("Failed to start embedded Meilisearch: {}", e);
                *self.state.write().await = SearchState::Failed;
                return Err(e);
            }
        };
        
        // Initialize client
        let client = Arc::new(MeiliSearchClient::new(
            &meili_config.host,
            meili_config.api_key.as_deref(),
            db_pool,
        ));
        
        // Initialize indexes
        if let Err(e) = client.initialize().await {
            warn!("Failed to initialize search indexes: {}", e);
            *self.state.write().await = SearchState::Failed;
            return Err(e);
        }
        
        // Store references
        *self.embedded.lock().await = Some(embedded_meili);
        *self.client.write().await = Some(client.clone());
        *self.state.write().await = SearchState::Ready;
        
        info!("Search system initialized successfully");
        
        Ok(())
    }
    
    /// Shutdown search system
    pub async fn shutdown(&self) -> Result<(), String> {
        info!("Shutting down search system...");
        
        // Acquire locks
        let mut embedded_guard = self.embedded.lock().await;
        
        // Stop Meilisearch if running
        if let Some(embedded) = embedded_guard.as_ref() {
            if let Err(e) = embedded.stop().await {
                error!("Failed to stop Meilisearch: {}", e);
            }
        }
        
        // Clean up
        *embedded_guard = None;
        *self.client.write().await = None;
        *self.state.write().await = SearchState::Uninitialized;
        
        info!("Search system shutdown complete");
        
        Ok(())
    }
    
    /// Get client if available
    pub async fn get_client(&self) -> Option<Arc<MeiliSearchClient>> {
        self.client.read().await.clone()
    }
    
    /// Check if search is available
    pub async fn is_available(&self) -> bool {
        *self.state.read().await == SearchState::Ready
    }
    
    /// Get search status
    pub async fn get_status(&self) -> SearchStatus {
        let state = *self.state.read().await;
        let state_str = match state {
            SearchState::Uninitialized => "uninitialized",
            SearchState::Initializing => "initializing",
            SearchState::Ready => "ready",
            SearchState::Failed => "failed",
        };
        
        let count = *self.indexed_count.read().await;
        let last = self.last_indexed.read().await.clone();
        
        SearchStatus {
            state: state_str.to_string(),
            documents_indexed: count,
            is_available: state == SearchState::Ready,
            last_indexed: last.map(|dt| dt.to_rfc3339()),
        }
    }
    
    /// Update indexed count
    pub async fn set_indexed_count(&self, count: usize) {
        *self.indexed_count.write().await = count;
        *self.last_indexed.write().await = Some(chrono::Utc::now());
    }
}