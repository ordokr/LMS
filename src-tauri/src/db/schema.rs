// Add this to your existing schema file

table! {
    discussion_mappings (id) {
        id -> Text,
        canvas_discussion_id -> Text,
        discourse_topic_id -> Text,
        course_category_id -> Text,
        title -> Text,
        last_sync -> Timestamp,
        sync_enabled -> Bool,
        sync_posts -> Bool,
        created_at -> Timestamp,
    }
}

table! {
    sync_history (id) {
        id -> Text,
        mapping_id -> Text,
        sync_type -> Text,
        status -> Text,
        message -> Nullable<Text>,
        details -> Nullable<Text>,
        started_at -> Timestamp,
        completed_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
    }
}

use sqlx::{Pool, Sqlite};
use tracing::{info, warn, error};

/// Validates the database schema and creates any missing tables
pub async fn validate_schema(pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
    info!("Validating database schema...");
    
    // Check if tables exist and create them if needed
    create_courses_table(pool).await?;
    create_assignments_table(pool).await?;
    create_submissions_table(pool).await?;
    create_discussion_mappings_table(pool).await?;
    create_sync_history_table(pool).await?;
    
    info!("Database schema validation complete");
    Ok(())
}

async fn create_courses_table(pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS courses (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            description TEXT NOT NULL,
            status TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        )"
    )
    .execute(pool)
    .await?;
    
    Ok(())
}

async fn create_assignments_table(pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS assignments (
            id TEXT PRIMARY KEY,
            course_id TEXT NOT NULL,
            title TEXT NOT NULL,
            description TEXT NOT NULL,
            due_date TEXT,
            points_possible REAL NOT NULL,
            status TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            FOREIGN KEY (course_id) REFERENCES courses(id)
        )"
    )
    .execute(pool)
    .await?;
    
    Ok(())
}

async fn create_submissions_table(pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS submissions (
            id TEXT PRIMARY KEY,
            assignment_id TEXT NOT NULL,
            user_id TEXT NOT NULL,
            content TEXT NOT NULL,
            attachments TEXT NOT NULL,
            status TEXT NOT NULL,
            score REAL,
            feedback TEXT,
            submitted_at TEXT NOT NULL,
            graded_at TEXT,
            FOREIGN KEY (assignment_id) REFERENCES assignments(id),
            FOREIGN KEY (user_id) REFERENCES users(id)
        )"
    )
    .execute(pool)
    .await?;
    
    Ok(())
}

async fn create_users_table(pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (
            id TEXT PRIMARY KEY,
            email TEXT NOT NULL UNIQUE,
            password_hash TEXT NOT NULL,
            first_name TEXT NOT NULL,
            last_name TEXT NOT NULL,
            role TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        )"
    )
    .execute(pool)
    .await?;
    
    Ok(())
}

async fn create_user_profiles_table(pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS user_profiles (
            user_id TEXT PRIMARY KEY,
            bio TEXT NOT NULL,
            avatar_url TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            FOREIGN KEY (user_id) REFERENCES users(id)
        )"
    )
    .execute(pool)
    .await?;
    
    Ok(())
}

async fn create_user_preferences_table(pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS user_preferences (
            user_id TEXT PRIMARY KEY,
            preferences TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            FOREIGN KEY (user_id) REFERENCES users(id)
        )"
    )
    .execute(pool)
    .await?;
    
    Ok(())
}

async fn create_user_integration_settings_table(pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS user_integration_settings (
            user_id TEXT PRIMARY KEY,
            settings TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            FOREIGN KEY (user_id) REFERENCES users(id)
        )"
    )
    .execute(pool)
    .await?;
    
    Ok(())
}

async fn create_course_category_mappings_table(pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS course_category_mappings (
            id TEXT PRIMARY KEY,
            course_id TEXT NOT NULL,
            category_id TEXT NOT NULL,
            sync_topics BOOLEAN NOT NULL,
            sync_assignments BOOLEAN NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            FOREIGN KEY (course_id) REFERENCES courses(id),
            UNIQUE(course_id, category_id)
        )"
    )
    .execute(pool)
    .await?;
    
    Ok(())
}

async fn create_discussions_table(pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS discussions (
            id TEXT PRIMARY KEY,
            course_id TEXT NOT NULL,
            title TEXT NOT NULL,
            content TEXT NOT NULL,
            topic_id TEXT,
            status TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            FOREIGN KEY (course_id) REFERENCES courses(id)
        )"
    )
    .execute(pool)
    .await?;
    
    Ok(())
}

async fn create_notifications_table(pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS notifications (
            id TEXT PRIMARY KEY,
            user_id TEXT NOT NULL,
            title TEXT NOT NULL,
            message TEXT NOT NULL,
            notification_type TEXT NOT NULL,
            status TEXT NOT NULL,
            reference_id TEXT,
            reference_type TEXT,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            FOREIGN KEY (user_id) REFERENCES users(id)
        )"
    )
    .execute(pool)
    .await?;
    
    // Create index on user_id and status for faster queries
    sqlx::query(
        "CREATE INDEX IF NOT EXISTS idx_notifications_user_status 
         ON notifications(user_id, status)"
    )
    .execute(pool)
    .await?;
    
    Ok(())
}

async fn create_modules_table(pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS modules (
            id TEXT PRIMARY KEY,
            course_id TEXT NOT NULL,
            title TEXT NOT NULL,
            position INTEGER NOT NULL,
            items_count INTEGER NOT NULL,
            publish_final_grade BOOLEAN NOT NULL,
            published BOOLEAN NOT NULL,
            status TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            FOREIGN KEY (course_id) REFERENCES courses(id)
        )"
    )
    .execute(pool)
    .await?;
    
    // Create index on course_id for faster queries
    sqlx::query(
        "CREATE INDEX IF NOT EXISTS idx_modules_course_id 
         ON modules(course_id)"
    )
    .execute(pool)
    .await?;
    
    Ok(())
}

async fn create_module_items_table(pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS module_items (
            id TEXT PRIMARY KEY,
            module_id TEXT NOT NULL,
            title TEXT NOT NULL,
            position INTEGER NOT NULL,
            item_type TEXT NOT NULL,
            content_id TEXT,
            content_type TEXT,
            url TEXT,
            page_url TEXT,
            published BOOLEAN NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            FOREIGN KEY (module_id) REFERENCES modules(id)
        )"
    )
    .execute(pool)
    .await?;
    
    // Create index on module_id for faster queries
    sqlx::query(
        "CREATE INDEX IF NOT EXISTS idx_module_items_module_id 
         ON module_items(module_id)"
    )
    .execute(pool)
    .await?;
    
    Ok(())
}