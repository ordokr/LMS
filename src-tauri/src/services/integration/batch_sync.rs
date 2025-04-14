use crate::db::DB;
use crate::models::forum::mapping::{TopicMapping, PostMapping, SyncStatus};
use crate::services::integration::sync_service::IntegrationSyncService;
use crate::error::Error;
use tokio::time::{Duration, sleep};
use std::sync::Arc;
use tokio::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};

pub struct BatchSyncService {
    db: DB,
    sync_service: Arc<IntegrationSyncService>,
    batch_size: usize,
    interval_seconds: u64,
    is_running: Arc<AtomicBool>,
    sync_in_progress: Arc<Mutex<bool>>,
}

impl BatchSyncService {
    pub fn new(
        db: DB, 
        sync_service: Arc<IntegrationSyncService>,
        batch_size: usize,
        interval_seconds: u64,
    ) -> Self {
        Self {
            db,
            sync_service,
            batch_size,
            interval_seconds,
            is_running: Arc::new(AtomicBool::new(false)),
            sync_in_progress: Arc::new(Mutex::new(false)),
        }
    }
    
    pub async fn start_batch_sync_loop(&self) -> Result<(), Error> {
        // Set the running flag
        self.is_running.store(true, Ordering::SeqCst);
        
        while self.is_running.load(Ordering::SeqCst) {
            // Process a batch of pending items
            if let Err(e) = self.process_pending_sync_batch().await {
                log::error!("Error in batch sync: {}", e);
            }
            
            // Wait for the next interval
            sleep(Duration::from_secs(self.interval_seconds)).await;
        }
        
        Ok(())
    }
    
    pub fn stop_batch_sync_loop(&self) {
        self.is_running.store(false, Ordering::SeqCst);
    }
    
    pub async fn is_sync_in_progress(&self) -> bool {
        let guard = self.sync_in_progress.lock().await;
        *guard
    }
    
    pub async fn process_pending_sync_batch(&self) -> Result<(), Error> {
        // Set the sync in progress flag
        {
            let mut guard = self.sync_in_progress.lock().await;
            if *guard {
                // Another sync is already in progress
                return Ok(());
            }
            *guard = true;
        }
        
        // Make sure we reset the flag when we're done
        let _cleanup = scopeguard::guard((), |_| {
            tokio::spawn(async move {
                let mut guard = self.sync_in_progress.lock().await;
                *guard = false;
            });
        });
        
        // Get pending topic mappings
        let pending_topics = TopicMapping::find_pending(&self.db, self.batch_size).await?;
        
        for mapping in pending_topics {
            match mapping.sync_status {
                SyncStatus::PendingToCanvas => {
                    // Sync from Discourse to Canvas
                    if let Some(discourse_id) = mapping.discourse_topic_id {
                        if let Err(e) = self.sync_service.sync_topic_discourse_to_canvas(&discourse_id.to_string()).await {
                            log::error!("Error syncing topic to Canvas: {}", e);
                        }
                    }
                },
                SyncStatus::PendingToDiscourse => {
                    // Sync from Canvas to Discourse
                    if let Some(canvas_id) = &mapping.canvas_topic_id {
                        if let Err(e) = self.sync_service.sync_topic_canvas_to_discourse(canvas_id).await {
                            log::error!("Error syncing topic to Discourse: {}", e);
                        }
                    }
                },
                _ => {}
            }
        }
        
        // Get pending post mappings
        let pending_posts = PostMapping::find_pending(&self.db, self.batch_size).await?;
        
        for mapping in pending_posts {
            match mapping.sync_status {
                SyncStatus::PendingToCanvas => {
                    // Sync from Discourse to Canvas
                    if let Some(discourse_id) = mapping.discourse_post_id {
                        if let Err(e) = self.sync_service.sync_post_discourse_to_canvas(&discourse_id.to_string()).await {
                            log::error!("Error syncing post to Canvas: {}", e);
                        }
                    }
                },
                SyncStatus::PendingToDiscourse => {
                    // Sync from Canvas to Discourse
                    if let Some(canvas_id) = &mapping.canvas_entry_id {
                        if let Err(e) = self.sync_service.sync_post_canvas_to_discourse(canvas_id).await {
                            log::error!("Error syncing post to Discourse: {}", e);
                        }
                    }
                },
                _ => {}
            }
        }
        
        Ok(())
    }
    
    pub async fn sync_all_pending(&self) -> Result<(), Error> {
        // Just call the batch process function
        self.process_pending_sync_batch().await
    }
}
