use serde::{Deserialize, Serialize};
use tauri::{command, State};
use sqlx::SqlitePool;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::error::Error;
use crate::services::integration::canvas_integration::CanvasIntegrationService;
use crate::utils::date_utils::{serialize_optional_date, deserialize_optional_date};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseIntegrationSettings {
    pub course_id: String,
    pub canvas_course_id: Option<String>,
    pub auto_sync_enabled: bool,
    pub sync_frequency_hours: Option<i32>,
    pub sync_modules: bool,
    pub sync_assignments: bool,
    pub sync_discussions: bool,
    pub sync_files: bool,
    pub sync_announcements: bool,
    
    #[serde(
        serialize_with = "serialize_optional_date",
        deserialize_with = "deserialize_optional_date"
    )]
    pub last_sync: Option<DateTime<Utc>>,
}

#[command]
pub async fn get_course_integration_settings(
    course_id: String,
    db: State<'_, SqlitePool>
) -> Result<CourseIntegrationSettings, Error> {
    let settings = sqlx::query!(
        r#"
        SELECT 
            course_id,
            canvas_course_id,
            auto_sync_enabled,
            sync_frequency_hours,
            sync_modules,
            sync_assignments,
            sync_discussions,
            sync_files,
            sync_announcements,
            last_sync
        FROM course_integration_settings
        WHERE course_id = ?
        "#,
        course_id
    )
    .fetch_optional(&**db)
    .await?;
    
    // Return existing settings or create default ones
    match settings {
        Some(s) => {
            let last_sync = if let Some(sync_str) = s.last_sync {
                // Parse the datetime string from the database
                if !sync_str.is_empty() {
                    DateTime::parse_from_rfc3339(&sync_str)
                        .map(|dt| Some(dt.with_timezone(&Utc)))
                        .unwrap_or(None)
                } else {
                    None
                }
            } else {
                None
            };
            
            Ok(CourseIntegrationSettings {
                course_id: s.course_id,
                canvas_course_id: s.canvas_course_id,
                auto_sync_enabled: s.auto_sync_enabled,
                sync_frequency_hours: s.sync_frequency_hours,
                sync_modules: s.sync_modules,
                sync_assignments: s.sync_assignments,
                sync_discussions: s.sync_discussions,
                sync_files: s.sync_files,
                sync_announcements: s.sync_announcements,
                last_sync,
            })
        },
        None => {
            // Create default settings
            let default_settings = CourseIntegrationSettings {
                course_id: course_id.clone(),
                canvas_course_id: None,
                auto_sync_enabled: false,
                sync_frequency_hours: Some(24), // Default to daily
                sync_modules: true,
                sync_assignments: true,
                sync_discussions: true,
                sync_files: true,
                sync_announcements: true,
                last_sync: None,
            };
            
            // Insert the default settings
            sqlx::query!(
                r#"
                INSERT INTO course_integration_settings (
                    course_id,
                    canvas_course_id,
                    auto_sync_enabled,
                    sync_frequency_hours,
                    sync_modules,
                    sync_assignments,
                    sync_discussions,
                    sync_files,
                    sync_announcements
                ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
                "#,
                default_settings.course_id,
                default_settings.canvas_course_id,
                default_settings.auto_sync_enabled,
                default_settings.sync_frequency_hours,
                default_settings.sync_modules,
                default_settings.sync_assignments,
                default_settings.sync_discussions,
                default_settings.sync_files,
                default_settings.sync_announcements
            )
            .execute(&**db)
            .await?;
            
            Ok(default_settings)
        }
    }
}

#[command]
pub async fn update_course_integration_settings(
    settings: CourseIntegrationSettings,
    db: State<'_, SqlitePool>
) -> Result<CourseIntegrationSettings, Error> {
    // Update the settings in the database
    sqlx::query!(
        r#"
        UPDATE course_integration_settings SET
            canvas_course_id = ?,
            auto_sync_enabled = ?,
            sync_frequency_hours = ?,
            sync_modules = ?,
            sync_assignments = ?,
            sync_discussions = ?,
            sync_files = ?,
            sync_announcements = ?
        WHERE course_id = ?
        "#,
        settings.canvas_course_id,
        settings.auto_sync_enabled,
        settings.sync_frequency_hours,
        settings.sync_modules,
        settings.sync_assignments,
        settings.sync_discussions,
        settings.sync_files,
        settings.sync_announcements,
        settings.course_id
    )
    .execute(&**db)
    .await?;
    
    // Fetch the updated settings to confirm changes
    let updated_settings = sqlx::query_as!(
        CourseIntegrationSettings,
        r#"
        SELECT 
            course_id,
            canvas_course_id,
            auto_sync_enabled,
            sync_frequency_hours,
            sync_modules,
            sync_assignments,
            sync_discussions,
            sync_files,
            sync_announcements,
            last_sync
        FROM course_integration_settings
        WHERE course_id = ?
        "#,
        settings.course_id
    )
    .fetch_one(&**db)
    .await?;
    
    Ok(updated_settings)
}

#[command]
pub async fn connect_course_to_canvas(
    course_id: String,
    canvas_course_id: String,
    db: State<'_, SqlitePool>,
    canvas_service: State<'_, CanvasIntegrationService>
) -> Result<String, Error> {
    // Verify the Canvas course ID is valid by attempting to fetch it
    let course = canvas_service.fetch_canvas_course(&canvas_course_id).await?;
    
    // Update the settings with the verified Canvas course ID
    sqlx::query!(
        r#"
        UPDATE course_integration_settings SET
            canvas_course_id = ?
        WHERE course_id = ?
        "#,
        canvas_course_id,
        course_id
    )
    .execute(&**db)
    .await?;
    
    // If no settings exist for this course yet, insert default settings
    if sqlx::query!(
        "SELECT COUNT(*) as count FROM course_integration_settings WHERE course_id = ?",
        course_id
    )
    .fetch_one(&**db)
    .await?
    .count == 0 {
        sqlx::query!(
            r#"
            INSERT INTO course_integration_settings (
                course_id,
                canvas_course_id,
                auto_sync_enabled,
                sync_frequency_hours,
                sync_modules,
                sync_assignments,
                sync_discussions,
                sync_files,
                sync_announcements
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
            course_id,
            Some(canvas_course_id),
            false, // auto_sync_enabled default
            24,    // sync_frequency_hours default
            true,  // sync_modules default
            true,  // sync_assignments default
            true,  // sync_discussions default
            true,  // sync_files default
            true   // sync_announcements default
        )
        .execute(&**db)
        .await?;
    }
    
    Ok(format!("Successfully connected to Canvas course: {}", course.name))
}

#[command]
pub async fn disconnect_course_from_canvas(
    course_id: String,
    db: State<'_, SqlitePool>
) -> Result<String, Error> {
    // Update the settings to remove the Canvas course ID
    sqlx::query!(
        r#"
        UPDATE course_integration_settings SET
            canvas_course_id = NULL,
            auto_sync_enabled = false
        WHERE course_id = ?
        "#,
        course_id
    )
    .execute(&**db)
    .await?;
    
    Ok("Successfully disconnected from Canvas".to_string())
}

#[command]
pub async fn sync_course_with_canvas(
    course_id: String,
    sync_modules: bool,
    sync_assignments: bool,
    sync_discussions: bool,
    sync_files: bool,
    sync_announcements: bool,
    db: State<'_, SqlitePool>,
    canvas_service: State<'_, CanvasIntegrationService>
) -> Result<String, Error> {
    // Get the Canvas course ID from settings
    let settings = sqlx::query!(
        r#"
        SELECT canvas_course_id
        FROM course_integration_settings
        WHERE course_id = ?
        "#,
        course_id
    )
    .fetch_optional(&**db)
    .await?
    .ok_or_else(|| Error::NotFound("Course integration settings not found".into()))?;
    
    let canvas_course_id = settings.canvas_course_id
        .ok_or_else(|| Error::NotFound("Canvas course ID not set".into()))?;
    
    // Perform the synchronization with the specified options
    if sync_modules {
        // Sync modules and module items
        canvas_service.sync_modules_from_canvas(&course_id, &canvas_course_id).await?;
    }
    
    if sync_assignments {
        // Sync assignments
        canvas_service.sync_assignments_from_canvas(&course_id, &canvas_course_id).await?;
    }
    
    if sync_discussions {
        // Sync discussions
        canvas_service.sync_discussions_from_canvas(&course_id, &canvas_course_id).await?;
    }
    
    if sync_files {
        // Sync files
        canvas_service.sync_files_from_canvas(&course_id, &canvas_course_id).await?;
    }
    
    if sync_announcements {
        // Sync announcements
        canvas_service.sync_announcements_from_canvas(&course_id, &canvas_course_id).await?;
    }
    
    // Update the last sync time
    let now = Utc::now();
    let now_str = now.to_rfc3339();
    sqlx::query!(
        r#"
        UPDATE course_integration_settings SET
            last_sync = ?
        WHERE course_id = ?
        "#,
        now_str,
        course_id
    )
    .execute(&**db)
    .await?;
    
    Ok("Course synchronization completed successfully".to_string())
}