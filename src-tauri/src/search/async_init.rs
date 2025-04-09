use std::sync::Arc;
use tokio::sync::{oneshot, Mutex};
use log::{info, warn};

use super::embedded::EmbeddedMeilisearch;
use super::meilisearch::MeiliSearchClient;
use sqlx::SqlitePool;

pub struct AsyncSearchInitializer {
    initialization_status: Mutex<InitStatus>,
    meilisearch: Arc<EmbeddedMeilisearch>,
    search_client: Arc<Mutex<Option<Arc<MeiliSearchClient>>>>,
}

enum InitStatus {
    NotStarted,
    InProgress(oneshot::Receiver<Result<(), String>>),
    Completed(Result<(), String>),
}

impl AsyncSearchInitializer {
    pub fn new(meilisearch: Arc<EmbeddedMeilisearch>) -> Self {
        Self {
            initialization_status: Mutex::new(InitStatus::NotStarted),
            meilisearch,
            search_client: Arc::new(Mutex::new(None)),
        }
    }
    
    // Initialize search in background without blocking app startup
    pub async fn start_initialization(&self, pool: Arc<SqlitePool>) {
        let mut status = self.initialization_status.lock().await;
        
        match &*status {
            InitStatus::NotStarted => {
                // Create channel for receiving completion status
                let (tx, rx) = oneshot::channel();
                
                // Start initialization in background
                let meilisearch = self.meilisearch.clone();
                let search_client_mutex = self.search_client.clone();
                
                tokio::spawn(async move {
                    info!("Starting Meilisearch initialization in background");
                    
                    // Start embedded Meilisearch
                    let result = match meilisearch.start().await {
                        Ok(config) => {
                            // Create Meilisearch client
                            let client = Arc::new(MeiliSearchClient::new(
                                &config.host,
                                config.api_key.as_deref(),
                                pool,
                            ));
                            
                            // Initialize indexes
                            if let Err(e) = client.initialize().await {
                                warn!("Failed to initialize Meilisearch indexes: {}", e);
                                Err(format!("Index initialization failed: {}", e))
                            } else {
                                // Store client
                                let mut client_guard = search_client_mutex.lock().await;
                                *client_guard = Some(client.clone());
                                
                                // Start background sync
                                client.clone().start_background_sync();
                                
                                Ok(())
                            }
                        },
                        Err(e) => {
                            warn!("Failed to start Meilisearch: {}", e);
                            Err(e)
                        }
                    };
                    
                    // Send result
                    let _ = tx.send(result);
                });
                
                // Update status
                *status = InitStatus::InProgress(rx);
                info!("Meilisearch initialization started in background");
            },
            InitStatus::InProgress(_) => {
                info!("Meilisearch initialization already in progress");
            },
            InitStatus::Completed(result) => {
                match result {
                    Ok(_) => info!("Meilisearch already initialized"),
                    Err(e) => warn!("Meilisearch initialization previously failed: {}", e),
                }
            }
        }
    }
    
    // Get initialization status
    pub async fn is_ready(&self) -> bool {
        let mut status = self.initialization_status.lock().await;
        
        // Check if we need to update status from InProgress to Completed
        if let InitStatus::InProgress(ref mut rx) = &mut *status {
            // Poll receiver (non-blocking)
            if rx.is_terminated() {
                // Receiver completed, get result
                match rx.try_recv() {
                    Ok(result) => {
                        *status = InitStatus::Completed(result);
                    },
                    Err(_) => {
                        // Channel closed without value - treat as error
                        *status = InitStatus::Completed(Err("Initialization failed".to_string()));
                    }
                }
            }
        }
        
        // Return true if successfully completed
        matches!(*status, InitStatus::Completed(Ok(())))
    }
    
    // Get client if available
    pub async fn get_client(&self) -> Option<Arc<MeiliSearchClient>> {
        let client_guard = self.search_client.lock().await;
        client_guard.clone()
    }
}