use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::oneshot;
use log::{info, error};
use crate::search::manager::SEARCH_MANAGER;

/// Start search initialization in a background thread
pub fn initialize_search_in_background(
    app_data_dir: PathBuf, 
    db_pool: Arc<sqlx::SqlitePool>,
    callback: impl FnOnce(bool) + Send + 'static
) {
    // Create a new thread to initialize search
    std::thread::spawn(move || {
        // Create a new runtime for this thread
        let rt = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(1)
            .thread_name("search-init")
            .enable_all()
            .build()
            .expect("Failed to build runtime for search initialization");
            
        // Run the initialization in this thread
        rt.block_on(async {
            info!("Starting search initialization in background thread");
            
            // Initialize search
            let result = SEARCH_MANAGER.initialize_if_needed(&app_data_dir, db_pool).await;
            
            // Call callback with result
            callback(result);
            
            info!("Search initialization thread complete");
        });
    });
}

pub async fn sync_search_data_in_background() -> oneshot::Receiver<Result<usize, String>> {
    let (tx, rx) = oneshot::channel();
    
    tokio::spawn(async move {
        // Get client
        let client = match SEARCH_MANAGER.get_client().await {
            Some(client) => client,
            None => {
                let _ = tx.send(Err("Search client not available".to_string()));
                return;
            }
        };
        
        // Start sync
        info!("Starting background data synchronization");
        match client.sync_data(true).await {
            Ok(count) => {
                // Update stats
                SEARCH_MANAGER.set_indexed_count(count).await;
                let _ = tx.send(Ok(count));
                info!("Synchronized {} documents", count);
            },
            Err(e) => {
                let _ = tx.send(Err(format!("Sync failed: {}", e)));
                error!("Data synchronization failed: {}", e);
            }
        }
    });
    
    rx
}