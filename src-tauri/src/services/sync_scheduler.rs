use std::sync::Arc;
use sqlx::SqlitePool;
use tokio::time::{self, Duration};
use chrono::{Utc, DateTime};
use tauri::async_runtime::Mutex;
use crate::error::Error;
use crate::services::integration::canvas_integration::CanvasIntegrationService;

pub struct SyncScheduler {
    db: SqlitePool,
    canvas_service: Arc<CanvasIntegrationService>,
    running: Arc<Mutex<bool>>,
}

impl SyncScheduler {
    pub fn new(db: SqlitePool, canvas_service: Arc<CanvasIntegrationService>) -> Self {
        SyncScheduler {
            db,
            canvas_service,
            running: Arc::new(Mutex::new(false)),
        }
    }
    
    pub async fn start(&self) -> Result<(), Error> {
        // Make sure we're not already running
        let mut running = self.running.lock().await;
        if *running {
            return Ok(());
        }
        
        *running = true;
        
        // Clone Arc references for the async task
        let db = self.db.clone();
        let canvas_service = self.canvas_service.clone();
        let running_ref = self.running.clone();
        
        // Spawn the background task
        tokio::spawn(async move {
            let mut interval = time::interval(Duration::from_secs(60 * 15)); // Check every 15 minutes
            
            loop {
                interval.tick().await;
                
                // Check if we should stop
                if !*running_ref.lock().await {
                    break;
                }
                
                // Get courses with auto-sync enabled
                let courses_result = sqlx::query!(
                    r#"
                    SELECT 
                        c.id as course_id,
                        c.name as course_name,
                        s.canvas_course_id,
                        s.sync_frequency_hours,
                        s.last_sync,
                        s.sync_modules,
                        s.sync_assignments,
                        s.sync_discussions,
                        s.sync_files,
                        s.sync_announcements
                    FROM courses c
                    JOIN course_integration_settings s ON c.id = s.course_id
                    WHERE s.auto_sync_enabled = true
                    AND s.canvas_course_id IS NOT NULL
                    "#
                )
                .fetch_all(&db)
                .await;
                
                let courses = match courses_result {
                    Ok(c) => c,
                    Err(err) => {
                        eprintln!("Error fetching courses for sync: {}", err);
                        continue;
                    }
                };
                
                for course in courses {
                    // Skip if no Canvas course ID
                    let canvas_id = match &course.canvas_course_id {
                        Some(id) => id,
                        None => continue,
                    };
                    
                    // Check if it's time to sync based on frequency and last sync
                    let should_sync = match course.last_sync {
                        Some(last_sync) => {
                            // Parse the last sync time
                            if let Ok(last_sync_time) = DateTime::parse_from_rfc3339(&last_sync) {
                                let hours_since_sync = Utc::now()
                                    .signed_duration_since(last_sync_time.with_timezone(&Utc))
                                    .num_hours();
                                
                                let sync_frequency = course.sync_frequency_hours.unwrap_or(24);
                                hours_since_sync >= sync_frequency as i64
                            } else {
                                true // If we can't parse the time, sync it anyway
                            }
                        },
                        None => true, // If never synced, do it now
                    };
                    
                    if !should_sync {
                        continue;
                    }
                    
                    // Perform the sync for this course
                    if let Err(err) = sync_course(
                        &db,
                        &canvas_service,
                        &course.course_id,
                        canvas_id,
                        course.sync_modules,
                        course.sync_assignments,
                        course.sync_discussions,
                        course.sync_files,
                        course.sync_announcements,
                    ).await {
                        eprintln!("Error during auto-sync for course {}: {}", course.course_name, err);
                    }
                }
            }
        });
        
        Ok(())
    }
    
    pub async fn stop(&self) {
        let mut running = self.running.lock().await;
        *running = false;
    }
}

async fn sync_course(
    db: &SqlitePool,
    canvas_service: &CanvasIntegrationService,
    course_id: &str,
    canvas_course_id: &str,
    sync_modules: bool,
    sync_assignments: bool,
    sync_discussions: bool,
    sync_files: bool,
    sync_announcements: bool,
) -> Result<(), Error> {
    // Log sync start
    println!("Starting auto-sync for course {}", course_id);
    
    // Perform the synchronization with the specified options
    if sync_modules {
        // Sync modules and module items
        canvas_service.sync_modules_from_canvas(course_id, canvas_course_id).await?;
    }
    
    if sync_assignments {
        // Sync assignments
        canvas_service.sync_assignments_from_canvas(course_id, canvas_course_id).await?;
    }
    
    if sync_discussions {
        // Sync discussions
        canvas_service.sync_discussions_from_canvas(course_id, canvas_course_id).await?;
    }
    
    if sync_files {
        // Sync files
        canvas_service.sync_files_from_canvas(course_id, canvas_course_id).await?;
    }
    
    if sync_announcements {
        // Sync announcements
        canvas_service.sync_announcements_from_canvas(course_id, canvas_course_id).await?;
    }
    
    // Update the last sync time
    let now = Utc::now().to_rfc3339();
    sqlx::query!(
        r#"
        UPDATE course_integration_settings SET
            last_sync = ?
        WHERE course_id = ?
        "#,
        now,
        course_id
    )
    .execute(db)
    .await?;
    
    println!("Auto-sync completed for course {}", course_id);
    
    Ok(())
}