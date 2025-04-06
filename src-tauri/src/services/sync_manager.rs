use std::{sync::Arc, time::Duration};
use tokio::{task::JoinHandle, time};
use chrono::Utc;
use log::{info, error, warn};
use serde::{Serialize, Deserialize};
use crate::db::DbPool;
use crate::api::{canvas::CanvasClient, discourse::DiscourseClient};
use crate::services::{
    discussion_sync::DiscussionSyncService,
    course_sync::CourseSyncService,
};
use crate::models::sync_config::SyncConfig;
use crate::error::Error;
use crate::models::discussion_mapping::SyncResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncStatus {
    pub is_running: bool,
    pub last_sync_started: Option<chrono::DateTime<Utc>>,
    pub last_sync_completed: Option<chrono::DateTime<Utc>>,
    pub next_scheduled_sync: Option<chrono::DateTime<Utc>>,
    pub current_operation: Option<String>,
    pub success_count: u32,
    pub error_count: u32,
    pub sync_errors: Vec<String>,
}

impl Default for SyncStatus {
    fn default() -> Self {
        Self {
            is_running: false,
            last_sync_started: None,
            last_sync_completed: None,
            next_scheduled_sync: None,
            current_operation: None,
            success_count: 0,
            error_count: 0,
            sync_errors: Vec::new(),
        }
    }
}

pub struct SyncManager {
    pool: Arc<DbPool>,
    canvas_client: Arc<CanvasClient>,
    discourse_client: Arc<DiscourseClient>,
    status: Arc<std::sync::Mutex<SyncStatus>>,
    sync_handle: Option<JoinHandle<()>>,
    shutdown_signal: Option<tokio::sync::oneshot::Sender<()>>,
}

impl SyncManager {
    pub fn new(
        pool: Arc<DbPool>,
        canvas_client: Arc<CanvasClient>,
        discourse_client: Arc<DiscourseClient>,
    ) -> Self {
        Self {
            pool,
            canvas_client,
            discourse_client,
            status: Arc::new(std::sync::Mutex::new(SyncStatus::default())),
            sync_handle: None,
            shutdown_signal: None,
        }
    }
    
    // Start the synchronization scheduler
    pub async fn start_scheduler(&mut self) -> Result<(), Error> {
        // Check if already running
        if self.sync_handle.is_some() {
            return Ok(());
        }
        
        // Get sync configuration from database
        let config = match self.get_sync_config().await {
            Ok(config) => config,
            Err(_) => {
                // Create default config if not found
                let default_config = SyncConfig::default();
                if let Err(e) = self.save_sync_config(&default_config).await {
                    error!("Failed to save default sync config: {}", e);
                    return Err(e);
                }
                default_config
            }
        };
        
        // Create shutdown channel
        let (tx, mut rx) = tokio::sync::oneshot::channel();
        self.shutdown_signal = Some(tx);
        
        // Clone Arc references for the task
        let pool = self.pool.clone();
        let canvas_client = self.canvas_client.clone();
        let discourse_client = self.discourse_client.clone();
        let status = self.status.clone();
        
        // Start background task
        let handle = tokio::spawn(async move {
            let mut interval = time::interval(Duration::from_secs(config.check_interval_seconds as u64));
            
            loop {
                tokio::select! {
                    _ = interval.tick() => {
                        // Check if it's time to sync
                        let should_sync = {
                            let status_guard = status.lock().unwrap();
                            let now = Utc::now();
                            
                            if let Some(next_sync) = status_guard.next_scheduled_sync {
                                now >= next_sync
                            } else {
                                true // First run
                            }
                        };
                        
                        if should_sync {
                            // Run sync process
                            if let Err(e) = Self::perform_sync(
                                &pool, 
                                &canvas_client, 
                                &discourse_client,
                                &status,
                                &config,
                            ).await {
                                error!("Sync process error: {}", e);
                                
                                // Update status with error
                                let mut status_guard = status.lock().unwrap();
                                status_guard.error_count += 1;
                                status_guard.sync_errors.push(format!("Sync error: {}", e));
                                
                                // Keep only the last 10 errors
                                if status_guard.sync_errors.len() > 10 {
                                    status_guard.sync_errors.remove(0);
                                }
                            }
                            
                            // Schedule next sync
                            {
                                let mut status_guard = status.lock().unwrap();
                                status_guard.next_scheduled_sync = Some(
                                    Utc::now() + chrono::Duration::seconds(config.sync_interval_seconds as i64)
                                );
                            }
                        }
                    }
                    _ = &mut rx => {
                        // Shutdown signal received
                        info!("Sync manager shutting down");
                        break;
                    }
                }
            }
        });
        
        self.sync_handle = Some(handle);
        
        // Update initial status
        {
            let mut status_guard = self.status.lock().unwrap();
            status_guard.next_scheduled_sync = Some(
                Utc::now() + chrono::Duration::seconds(config.sync_interval_seconds as i64)
            );
        }
        
        info!("Sync scheduler started");
        Ok(())
    }
    
    // Stop the synchronization scheduler
    pub async fn stop_scheduler(&mut self) -> Result<(), Error> {
        if let Some(signal) = self.shutdown_signal.take() {
            let _ = signal.send(());
            
            if let Some(handle) = self.sync_handle.take() {
                let _ = handle.await;
            }
            
            // Update status
            let mut status_guard = self.status.lock().unwrap();
            status_guard.is_running = false;
            status_guard.current_operation = None;
            
            info!("Sync scheduler stopped");
        }
        
        Ok(())
    }
    
    // Get current sync status
    pub fn get_status(&self) -> SyncStatus {
        self.status.lock().unwrap().clone()
    }
    
    // Run a manual synchronization
    pub async fn run_manual_sync(&self) -> Result<(), Error> {
        let config = self.get_sync_config().await?;
        
        Self::perform_sync(
            &self.pool,
            &self.canvas_client, 
            &self.discourse_client,
            &self.status,
            &config,
        ).await
    }
    
    // Main sync process
    async fn perform_sync(
        pool: &DbPool,
        canvas_client: &CanvasClient,
        discourse_client: &DiscourseClient,
        status: &Arc<std::sync::Mutex<SyncStatus>>,
        config: &SyncConfig,
    ) -> Result<(), Error> {
        // Update status to running
        {
            let mut status_guard = status.lock().unwrap();
            status_guard.is_running = true;
            status_guard.last_sync_started = Some(Utc::now());
            status_guard.current_operation = Some("Starting synchronization".to_string());
        }
        
        info!("Starting synchronization process");
        
        // First sync course mappings
        {
            let mut status_guard = status.lock().unwrap();
            status_guard.current_operation = Some("Synchronizing courses".to_string());
        }
        
        let course_sync_service = CourseSyncService::new(
            Arc::new(pool.clone()),
            Arc::new(canvas_client.clone()),
            Arc::new(discourse_client.clone()),
        );
        
        match course_sync_service.sync_all_courses().await {
            Ok(results) => {
                let mut status_guard = status.lock().unwrap();
                status_guard.success_count += results.len() as u32;
                
                // Log results
                for result in results {
                    info!("Course sync result: {:?}", result);
                }
            }
            Err(e) => {
                error!("Failed to sync courses: {}", e);
                let mut status_guard = status.lock().unwrap();
                status_guard.error_count += 1;
                status_guard.sync_errors.push(format!("Course sync error: {}", e));
            }
        }
        
        // Then sync discussion mappings
        {
            let mut status_guard = status.lock().unwrap();
            status_guard.current_operation = Some("Synchronizing discussions".to_string());
        }
        
        let discussion_sync_service = DiscussionSyncService::new(
            Arc::new(pool.clone()),
            Arc::new(canvas_client.clone()),
            Arc::new(discourse_client.clone()),
        );
        
        match discussion_sync_service.sync_all_discussions().await {
            Ok(results) => {
                let mut status_guard = status.lock().unwrap();
                
                // Count successful syncs
                let success_count = results.iter()
                    .filter(|r| r.status == "success" || r.status == "partial")
                    .count() as u32;
                    
                status_guard.success_count += success_count;
                
                // Count and log errors
                for result in results {
                    if !result.errors.is_empty() {
                        status_guard.error_count += 1;
                        
                        for error in result.errors {
                            status_guard.sync_errors.push(format!(
                                "Discussion sync error ({}): {}", result.mapping_id, error
                            ));
                            
                            // Keep only the last 10 errors
                            if status_guard.sync_errors.len() > 10 {
                                status_guard.sync_errors.remove(0);
                            }
                        }
                    }
                }
            }
            Err(e) => {
                error!("Failed to sync discussions: {}", e);
                let mut status_guard = status.lock().unwrap();
                status_guard.error_count += 1;
                status_guard.sync_errors.push(format!("Discussion sync error: {}", e));
            }
        }
        
        // Update completion status
        {
            let mut status_guard = status.lock().unwrap();
            status_guard.is_running = false;
            status_guard.last_sync_completed = Some(Utc::now());
            status_guard.current_operation = None;
        }
        
        info!("Synchronization process completed");
        Ok(())
    }
    
    // Get sync configuration
    async fn get_sync_config(&self) -> Result<SyncConfig, Error> {
        // This would fetch from database
        // For now, return a default config
        Ok(SyncConfig::default())
    }
    
    // Save sync configuration
    async fn save_sync_config(&self, config: &SyncConfig) -> Result<(), Error> {
        // This would save to database
        // For now, just log
        info!("Would save sync config: {:?}", config);
        Ok(())
    }
    
    // Update sync configuration
    pub async fn update_sync_config(&self, config: SyncConfig) -> Result<SyncConfig, Error> {
        self.save_sync_config(&config).await?;
        Ok(config)
    }

    // Add discussion sync to the sync process
    
    // Method to sync all discussions across all course mappings
    pub async fn sync_all_discussions(&self) -> Result<Vec<SyncResult>, Error> {
        let discussion_service = DiscussionSyncService::new(
            self.db_pool.clone(),
            self.canvas_client.clone(),
            self.discourse_client.clone(),
        );
        
        // Get all course category mappings
        let course_mappings = match self.db.get_all_course_categories().await {
            Ok(mappings) => mappings,
            Err(e) => {
                error!("Failed to get course mappings: {}", e);
                return Err(e);
            }
        };
        
        let mut all_results = Vec::new();
        
        // For each course, sync its discussions
        for course in course_mappings {
            if !course.sync_enabled {
                continue;
            }
            
            info!("Syncing discussions for course mapping: {}", course.id);
            
            match discussion_service.sync_all_for_course(&course.id).await {
                Ok(results) => {
                    all_results.extend(results);
                },
                Err(e) => {
                    error!("Error syncing discussions for course {}: {}", course.id, e);
                }
            }
        }
        
        Ok(all_results)
    }
    
    // Add this to your existing run_sync method
    pub async fn run_sync(&self) -> Result<(), Error> {
        // Set sync status to running
        self.set_status(SyncStatus::Running);
        
        // Log sync start
        info!("Starting synchronization");
        let start_time = Utc::now();
        
        // First sync course-category mappings
        let course_results = match self.sync_all_courses().await {
            Ok(results) => results,
            Err(e) => {
                error!("Error syncing courses: {}", e);
                self.set_status(SyncStatus::Failed);
                return Err(e);
            }
        };
        
        // Then sync discussion mappings
        let discussion_results = match self.sync_all_discussions().await {
            Ok(results) => results,
            Err(e) => {
                error!("Error syncing discussions: {}", e);
                self.set_status(SyncStatus::Failed);
                return Err(e);
            }
        };
        
        // Calculate statistics
        let course_count = course_results.len();
        let discussion_count = discussion_results.len();
        let successful_courses = course_results.iter().filter(|r| r.status == "success").count();
        let successful_discussions = discussion_results.iter().filter(|r| r.status == "success").count();
        
        // Log completion
        let duration = Utc::now() - start_time;
        info!("Sync completed in {} seconds. Courses: {}/{} successful, Discussions: {}/{} successful", 
              duration.num_seconds(), successful_courses, course_count, 
              successful_discussions, discussion_count);
        
        // Update sync status
        if successful_courses == course_count && successful_discussions == discussion_count {
            self.set_status(SyncStatus::Success);
        } else {
            self.set_status(SyncStatus::PartialSuccess);
        }
        
        // Save last sync time and info
        self.update_last_sync_info(start_time, duration, 
                                  successful_courses, course_count, 
                                  successful_discussions, discussion_count).await?;
                                  
        Ok(())
    }
}