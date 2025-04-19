use sqlx::{Pool, Sqlite};
use chrono::Utc;
use uuid::Uuid;
use std::collections::HashMap;
use log::{info, warn, error};
use crate::models::unified_models::{Course as UnifiedCourse, CourseStatus, CourseVisibility, HomepageType};
use crate::error::Error;

/// Utility for migrating course data from old tables to the new unified table
pub struct CourseMigration {
    pool: Pool<Sqlite>,
}

impl CourseMigration {
    /// Create a new course migration utility
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }
    
    /// Migrate all courses from old tables to the new unified table
    pub async fn migrate_all_courses(&self) -> Result<MigrationStats, Error> {
        info!("Starting course migration...");
        
        let mut stats = MigrationStats::default();
        
        // Migrate from src-tauri/src/models/course/course.rs
        stats += self.migrate_from_course_model().await?;
        
        // Migrate from src-tauri/src/models/course.rs
        stats += self.migrate_from_simple_course_model().await?;
        
        // Migrate from src-tauri/src/models/unified/course.rs
        stats += self.migrate_from_old_unified_course_model().await?;
        
        info!("Course migration completed: {}", stats);
        
        Ok(stats)
    }
    
    /// Migrate courses from the model in src-tauri/src/models/course/course.rs
    async fn migrate_from_course_model(&self) -> Result<MigrationStats, Error> {
        info!("Migrating courses from course model...");
        
        let mut stats = MigrationStats::default();
        
        // Check if the old table exists
        let table_exists = sqlx::query!(
            "SELECT name FROM sqlite_master WHERE type='table' AND name='courses_old'"
        )
        .fetch_optional(&self.pool)
        .await?
        .is_some();
        
        if !table_exists {
            warn!("Old courses table (courses_old) not found, skipping migration");
            return Ok(stats);
        }
        
        // Fetch all courses from the old table
        let old_courses = sqlx::query!(
            r#"
            SELECT 
                id, name, code, description, instructor_id, start_date, end_date,
                status, created_at, updated_at, canvas_course_id, discourse_category_id
            FROM courses_old
            "#
        )
        .fetch_all(&self.pool)
        .await?;
        
        for old_course in old_courses {
            // Parse timestamps
            let created_at = match chrono::DateTime::parse_from_rfc3339(&old_course.created_at) {
                Ok(dt) => dt.with_timezone(&Utc),
                Err(e) => {
                    error!("Failed to parse created_at for course {}: {}", old_course.id, e);
                    Utc::now()
                }
            };
            
            let updated_at = match chrono::DateTime::parse_from_rfc3339(&old_course.updated_at) {
                Ok(dt) => dt.with_timezone(&Utc),
                Err(e) => {
                    error!("Failed to parse updated_at for course {}: {}", old_course.id, e);
                    Utc::now()
                }
            };
            
            let start_date = if let Some(ts) = old_course.start_date {
                match chrono::DateTime::parse_from_rfc3339(&ts) {
                    Ok(dt) => Some(dt.with_timezone(&Utc)),
                    Err(e) => {
                        error!("Failed to parse start_date for course {}: {}", old_course.id, e);
                        None
                    }
                }
            } else {
                None
            };
            
            let end_date = if let Some(ts) = old_course.end_date {
                match chrono::DateTime::parse_from_rfc3339(&ts) {
                    Ok(dt) => Some(dt.with_timezone(&Utc)),
                    Err(e) => {
                        error!("Failed to parse end_date for course {}: {}", old_course.id, e);
                        None
                    }
                }
            } else {
                None
            };
            
            // Parse status
            let status = match old_course.status.as_str() {
                "active" => CourseStatus::Active,
                "archived" => CourseStatus::Archived,
                "deleted" => CourseStatus::Deleted,
                _ => CourseStatus::Draft,
            };
            
            // Create unified course
            let unified_course = UnifiedCourse {
                id: old_course.id.to_string(),
                name: old_course.name,
                code: old_course.code,
                description: old_course.description,
                created_at,
                updated_at,
                status,
                visibility: CourseVisibility::Course,
                is_public: false,
                is_published: status == CourseStatus::Active,
                start_date,
                end_date,
                instructor_id: old_course.instructor_id.map(|id| id.to_string()),
                allow_self_enrollment: false,
                enrollment_code: None,
                enrollment_count: None,
                syllabus_body: None,
                homepage_type: HomepageType::Modules,
                default_view: "modules".to_string(),
                theme_color: None,
                banner_image_url: None,
                timezone: None,
                license: None,
                canvas_id: old_course.canvas_course_id,
                discourse_id: old_course.discourse_category_id,
                category_id: None,
                slug: None,
                color: None,
                position: None,
                parent_id: None,
                last_sync: None,
                source_system: None,
                metadata: HashMap::new(),
            };
            
            // Insert into new table
            if let Err(e) = self.insert_unified_course(&unified_course).await {
                error!("Failed to insert unified course {}: {}", unified_course.id, e);
                stats.errors += 1;
            } else {
                stats.migrated += 1;
            }
        }
        
        info!("Migrated {} courses from course model", stats.migrated);
        
        Ok(stats)
    }
    
    /// Migrate courses from the model in src-tauri/src/models/course.rs
    async fn migrate_from_simple_course_model(&self) -> Result<MigrationStats, Error> {
        info!("Migrating courses from simple course model...");
        
        let mut stats = MigrationStats::default();
        
        // Check if the old table exists
        let table_exists = sqlx::query!(
            "SELECT name FROM sqlite_master WHERE type='table' AND name='courses_simple'"
        )
        .fetch_optional(&self.pool)
        .await?
        .is_some();
        
        if !table_exists {
            warn!("Old courses table (courses_simple) not found, skipping migration");
            return Ok(stats);
        }
        
        // Fetch all courses from the old table
        let old_courses = sqlx::query!(
            r#"
            SELECT 
                id, title, description, instructor_id, created_at, status
            FROM courses_simple
            "#
        )
        .fetch_all(&self.pool)
        .await?;
        
        for old_course in old_courses {
            // Generate a code from the title
            let code = old_course.title.split_whitespace()
                .filter_map(|word| word.chars().next())
                .collect::<String>()
                .to_uppercase();
            
            // Create unified course
            let unified_course = UnifiedCourse {
                id: Uuid::new_v4().to_string(),
                name: old_course.title,
                code,
                description: old_course.description,
                created_at: Utc::now(),
                updated_at: Utc::now(),
                status: CourseStatus::from(old_course.status.as_str()),
                visibility: CourseVisibility::Course,
                is_public: false,
                is_published: old_course.status == "active",
                start_date: None,
                end_date: None,
                instructor_id: old_course.instructor_id.map(|id| id.to_string()),
                allow_self_enrollment: false,
                enrollment_code: None,
                enrollment_count: None,
                syllabus_body: None,
                homepage_type: HomepageType::Modules,
                default_view: "modules".to_string(),
                theme_color: None,
                banner_image_url: None,
                timezone: None,
                license: None,
                canvas_id: None,
                discourse_id: None,
                category_id: None,
                slug: None,
                color: None,
                position: None,
                parent_id: None,
                last_sync: None,
                source_system: None,
                metadata: HashMap::new(),
            };
            
            // Insert into new table
            if let Err(e) = self.insert_unified_course(&unified_course).await {
                error!("Failed to insert unified course {}: {}", unified_course.id, e);
                stats.errors += 1;
            } else {
                stats.migrated += 1;
            }
        }
        
        info!("Migrated {} courses from simple course model", stats.migrated);
        
        Ok(stats)
    }
    
    /// Migrate courses from the model in src-tauri/src/models/unified/course.rs
    async fn migrate_from_old_unified_course_model(&self) -> Result<MigrationStats, Error> {
        info!("Migrating courses from old unified course model...");
        
        let mut stats = MigrationStats::default();
        
        // Check if the old table exists
        let table_exists = sqlx::query!(
            "SELECT name FROM sqlite_master WHERE type='table' AND name='unified_courses'"
        )
        .fetch_optional(&self.pool)
        .await?
        .is_some();
        
        if !table_exists {
            warn!("Old courses table (unified_courses) not found, skipping migration");
            return Ok(stats);
        }
        
        // Fetch all courses from the old table
        let old_courses = sqlx::query!(
            r#"
            SELECT 
                id, title, description, created_at, updated_at, canvas_specific_fields,
                discourse_specific_fields
            FROM unified_courses
            "#
        )
        .fetch_all(&self.pool)
        .await?;
        
        for old_course in old_courses {
            // Parse timestamps
            let created_at = match chrono::DateTime::parse_from_rfc3339(&old_course.created_at) {
                Ok(dt) => dt.with_timezone(&Utc),
                Err(e) => {
                    error!("Failed to parse created_at for course {}: {}", old_course.id, e);
                    Utc::now()
                }
            };
            
            let updated_at = match chrono::DateTime::parse_from_rfc3339(&old_course.updated_at) {
                Ok(dt) => dt.with_timezone(&Utc),
                Err(e) => {
                    error!("Failed to parse updated_at for course {}: {}", old_course.id, e);
                    Utc::now()
                }
            };
            
            // Parse Canvas-specific fields
            let mut canvas_id = None;
            let mut code = String::new();
            let mut start_date = None;
            let mut end_date = None;
            let mut syllabus_body = None;
            
            if let Some(canvas_fields) = old_course.canvas_specific_fields {
                if let Ok(canvas_json) = serde_json::from_str::<serde_json::Value>(&canvas_fields) {
                    canvas_id = canvas_json["id"].as_str().map(|s| s.to_string());
                    code = canvas_json["course_code"].as_str().unwrap_or("").to_string();
                    syllabus_body = canvas_json["syllabus_body"].as_str().map(|s| s.to_string());
                    
                    // Parse dates
                    if let Some(start_str) = canvas_json["start_at"].as_str() {
                        start_date = chrono::DateTime::parse_from_rfc3339(start_str)
                            .ok()
                            .map(|dt| dt.with_timezone(&Utc));
                    }
                    
                    if let Some(end_str) = canvas_json["end_at"].as_str() {
                        end_date = chrono::DateTime::parse_from_rfc3339(end_str)
                            .ok()
                            .map(|dt| dt.with_timezone(&Utc));
                    }
                }
            }
            
            // Parse Discourse-specific fields
            let mut discourse_id = None;
            let mut slug = None;
            let mut color = None;
            let mut position = None;
            let mut parent_id = None;
            
            if let Some(discourse_fields) = old_course.discourse_specific_fields {
                if let Ok(discourse_json) = serde_json::from_str::<serde_json::Value>(&discourse_fields) {
                    discourse_id = discourse_json["id"].as_str().map(|s| s.to_string());
                    slug = discourse_json["slug"].as_str().map(|s| s.to_string());
                    color = discourse_json["color"].as_str().map(|s| s.to_string());
                    position = discourse_json["position"].as_i64().map(|p| p as i32);
                    parent_id = discourse_json["parent_category_id"].as_str().map(|s| s.to_string());
                }
            }
            
            // If code is empty, generate one from the title
            if code.is_empty() {
                code = old_course.title.split_whitespace()
                    .filter_map(|word| word.chars().next())
                    .collect::<String>()
                    .to_uppercase();
            }
            
            // Create unified course
            let unified_course = UnifiedCourse {
                id: old_course.id.unwrap_or_else(|| Uuid::new_v4().to_string()),
                name: old_course.title,
                code,
                description: old_course.description,
                created_at,
                updated_at,
                status: CourseStatus::Active,
                visibility: CourseVisibility::Course,
                is_public: false,
                is_published: true,
                start_date,
                end_date,
                instructor_id: None,
                allow_self_enrollment: false,
                enrollment_code: None,
                enrollment_count: None,
                syllabus_body,
                homepage_type: HomepageType::Modules,
                default_view: "modules".to_string(),
                theme_color: None,
                banner_image_url: None,
                timezone: None,
                license: None,
                canvas_id,
                discourse_id,
                category_id: None,
                slug,
                color,
                position,
                parent_id,
                last_sync: None,
                source_system: None,
                metadata: HashMap::new(),
            };
            
            // Insert into new table
            if let Err(e) = self.insert_unified_course(&unified_course).await {
                error!("Failed to insert unified course {}: {}", unified_course.id, e);
                stats.errors += 1;
            } else {
                stats.migrated += 1;
            }
        }
        
        info!("Migrated {} courses from old unified course model", stats.migrated);
        
        Ok(stats)
    }
    
    /// Insert a unified course into the new table
    async fn insert_unified_course(&self, course: &UnifiedCourse) -> Result<(), Error> {
        // Serialize metadata
        let metadata_json = serde_json::to_string(&course.metadata)
            .map_err(|e| Error::SerializationError(format!("Failed to serialize metadata: {}", e)))?;
        
        // Insert course
        sqlx::query!(
            r#"
            INSERT OR REPLACE INTO courses (
                id, name, code, description, created_at, updated_at, status, visibility,
                is_public, is_published, start_date, end_date, instructor_id,
                allow_self_enrollment, enrollment_code, enrollment_count, syllabus_body,
                homepage_type, default_view, theme_color, banner_image_url, timezone,
                license, canvas_id, discourse_id, category_id, slug, color, position,
                parent_id, last_sync, source_system, metadata
            ) VALUES (
                ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?
            )
            "#,
            course.id,
            course.name,
            course.code,
            course.description,
            course.created_at.to_rfc3339(),
            course.updated_at.to_rfc3339(),
            course.status.to_string(),
            course.visibility.to_string(),
            if course.is_public { 1 } else { 0 },
            if course.is_published { 1 } else { 0 },
            course.start_date.map(|dt| dt.to_rfc3339()),
            course.end_date.map(|dt| dt.to_rfc3339()),
            course.instructor_id,
            if course.allow_self_enrollment { 1 } else { 0 },
            course.enrollment_code,
            course.enrollment_count,
            course.syllabus_body,
            course.homepage_type.to_string(),
            course.default_view,
            course.theme_color,
            course.banner_image_url,
            course.timezone,
            course.license,
            course.canvas_id,
            course.discourse_id,
            course.category_id,
            course.slug,
            course.color,
            course.position,
            course.parent_id,
            course.last_sync.map(|dt| dt.to_rfc3339()),
            course.source_system,
            metadata_json
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
}

/// Statistics for the migration process
#[derive(Debug, Default, Clone, Copy)]
pub struct MigrationStats {
    pub migrated: usize,
    pub errors: usize,
}

impl std::ops::AddAssign for MigrationStats {
    fn add_assign(&mut self, other: Self) {
        self.migrated += other.migrated;
        self.errors += other.errors;
    }
}

impl std::fmt::Display for MigrationStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Migrated: {}, Errors: {}", self.migrated, self.errors)
    }
}
