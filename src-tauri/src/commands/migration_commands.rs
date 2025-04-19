use tauri::{AppHandle, State};
use crate::migrations::{UserMigration, CourseMigration, GroupMigration};
use crate::db::DB;
use log::{info, error};

/// Run the user migration to consolidate all user models into the unified model
#[tauri::command]
pub async fn run_user_migration(app_handle: AppHandle) -> Result<String, String> {
    info!("Starting user migration command...");

    // Get the database connection from the app state
    let db = app_handle.state::<DB>();

    // Create the user migration utility
    let migration = UserMigration::new(db.pool.clone());

    // Run the migration
    match migration.migrate_all_users().await {
        Ok(stats) => {
            let message = format!("User migration completed successfully: {}", stats);
            info!("{}", message);
            Ok(message)
        },
        Err(e) => {
            let error_message = format!("User migration failed: {}", e);
            error!("{}", error_message);
            Err(error_message)
        }
    }
}

/// Run the course migration to consolidate all course models into the unified model
#[tauri::command]
pub async fn run_course_migration(app_handle: AppHandle) -> Result<String, String> {
    info!("Starting course migration command...");

    // Get the database connection from the app state
    let db = app_handle.state::<DB>();

    // Create the course migration utility
    let migration = CourseMigration::new(db.pool.clone());

    // Run the migration
    match migration.migrate_all_courses().await {
        Ok(stats) => {
            let message = format!("Course migration completed successfully: {}", stats);
            info!("{}", message);
            Ok(message)
        },
        Err(e) => {
            let error_message = format!("Course migration failed: {}", e);
            error!("{}", error_message);
            Err(error_message)
        }
    }
}

/// Run the group migration to consolidate all group models into the unified model
#[tauri::command]
pub async fn run_group_migration(app_handle: AppHandle) -> Result<String, String> {
    info!("Starting group migration command...");

    // Get the database connection from the app state
    let db = app_handle.state::<DB>();

    // Create the group migration utility
    let migration = GroupMigration::new(db.pool.clone());

    // Run the migration
    match migration.migrate_all_groups().await {
        Ok(stats) => {
            let message = format!("Group migration completed successfully: {}", stats);
            info!("{}", message);
            Ok(message)
        },
        Err(e) => {
            let error_message = format!("Group migration failed: {}", e);
            error!("{}", error_message);
            Err(error_message)
        }
    }
}
