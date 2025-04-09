use std::sync::Arc;
use std::time::Duration;
use tokio::time;
use log::{info, error, warn};
use uuid::Uuid;

use crate::services::content_sync_service::ContentSyncService;
use crate::clients::canvas_client::CanvasApiClient;
use crate::clients::discourse_client::DiscourseApiClient;

pub struct SyncScheduler<C, D> 
where 
    C: CanvasClient + Send + Sync + 'static,
    D: DiscourseClient + Send + Sync + 'static,
{
    content_sync_service: Arc<ContentSyncService<C, D>>,
    interval_minutes: u64,
}

impl<C, D> SyncScheduler<C, D> 
where 
    C: CanvasClient + Send + Sync + 'static,
    D: DiscourseClient + Send + Sync + 'static,
{
    pub fn new(content_sync_service: ContentSyncService<C, D>, interval_minutes: u64) -> Self {
        Self {
            content_sync_service: Arc::new(content_sync_service),
            interval_minutes,
        }
    }
    
    pub async fn start(&self) {
        let service = self.content_sync_service.clone();
        let interval = self.interval_minutes;
        
        tokio::spawn(async move {
            let mut interval = time::interval(Duration::from_secs(interval * 60));
            
            loop {
                interval.tick().await;
                info!("Running scheduled sync job...");
                
                match service.sync_all_pending_topics().await {
                    Ok(count) => {
                        info!("Scheduled sync complete. Processed {} topics", count);
                    },
                    Err(e) => {
                        error!("Error during scheduled sync: {}", e);
                    }
                }
            }
        });
        
        info!("Sync scheduler started with interval of {} minutes", self.interval_minutes);
    }
}

// Example of how to initialize and start the scheduler
pub async fn init_sync_scheduler(app_state: &AppState) -> SyncScheduler<CanvasApiClient, DiscourseApiClient> {
    // Get configuration from environment or config file
    let canvas_url = std::env::var("CANVAS_API_URL").expect("CANVAS_API_URL must be set");
    let canvas_token = std::env::var("CANVAS_API_TOKEN").expect("CANVAS_API_TOKEN must be set");
    
    let discourse_url = std::env::var("DISCOURSE_API_URL").expect("DISCOURSE_API_URL must be set");
    let discourse_key = std::env::var("DISCOURSE_API_KEY").expect("DISCOURSE_API_KEY must be set");
    let discourse_username = std::env::var("DISCOURSE_API_USERNAME").expect("DISCOURSE_API_USERNAME must be set");
    
    // Default to 30 minutes if not specified
    let sync_interval = std::env::var("SYNC_INTERVAL_MINUTES")
        .unwrap_or_else(|_| "30".to_string())
        .parse::<u64>()
        .unwrap_or(30);
    
    // Initialize API clients
    let canvas_client = CanvasApiClient::new(&canvas_url, &canvas_token);
    let discourse_client = DiscourseApiClient::new(&discourse_url, &discourse_key, &discourse_username);
    
    // Create content sync service
    let content_sync_service = ContentSyncService::new(
        app_state.sync_service.clone(),
        app_state.topic_mapping_repo.clone(),
        canvas_client,
        discourse_client
    );
    
    // Create and start scheduler
    let scheduler = SyncScheduler::new(content_sync_service, sync_interval);
    scheduler.start().await;
    
    scheduler
}