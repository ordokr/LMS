use super::common::TestContext;
use crate::controllers::course_controller;
use crate::models::course::{Course, CourseStatus, IntegrationStatus};
use sqlx::SqlitePool;
use uuid::Uuid;

#[tokio::test]
async fn test_course_creation_and_update() -> Result<(), Box<dyn std::error::Error>> {
    // Set up test context
    let ctx = TestContext::new().await?;
    let pool = ctx.db_pool.clone();
    
    // Create test user as instructor
    let instructor_id = create_test_user(&pool, "instructor", "Course Instructor").await?;
    
    // Test: Create a course
    let course = course_controller::create_course(
        "CS101".to_string(),
        "Introduction to Computer Science".to_string(),
        Some("Learn the basics of programming".to_string()),
        instructor_id.clone(),
        Some("2025-01-01T00:00:00Z".to_string()),
        Some("2025-05-01T00:00:00Z".to_string()),
        "active".to_string(),
        Some("enroll123".to_string()),
        tauri::State::new(pool.clone())
    ).await?;
    
    // Verify course was created correctly
    assert_eq!(course.code, "CS101");
    assert_eq!(course.name, "Introduction to Computer Science");
    assert_eq!(course.description, Some("Learn the basics of programming".to_string()));
    assert_eq!(course.instructor_id, instructor_id);
    assert_eq!(course.status.to_string(), "active");
    assert_eq!(course.integration_status.to_string(), "notintegrated");
    
    // Test: Update a course
    let updated_course = course_controller::update_course(
        course.id.clone(),
        None,
        Some("CS101: Programming Fundamentals".to_string()),
        None,
        None,
        None,
        None,
        Some("upcoming".to_string()),
        None,
        tauri::State::new(pool.clone())
    ).await?;
    
    // Verify course was updated correctly
    assert_eq!(updated_course.name, "CS101: Programming Fundamentals");
    assert_eq!(updated_course.status.to_string(), "upcoming");
    
    // Test: List courses
    let courses = course_controller::list_courses(
        Some(1),
        Some(10),
        None,
        None,
        tauri::State::new(pool.clone())
    ).await?;
    
    assert!(!courses.is_empty());
    assert!(courses.iter().any(|c| c.id == course.id));
    
    // Test: Filter courses by status
    let filtered_courses = course_controller::list_courses(
        Some(1),
        Some(10),
        Some("upcoming".to_string()),
        None,
        tauri::State::new(pool.clone())
    ).await?;
    
    assert!(!filtered_courses.is_empty());
    assert!(filtered_courses.iter().all(|c| c.status.to_string() == "upcoming"));
    
    // Test: Search courses
    let searched_courses = course_controller::list_courses(
        Some(1),
        Some(10),
        None,
        Some("Fundamentals".to_string()),
        tauri::State::new(pool.clone())
    ).await?;
    
    assert!(!searched_courses.is_empty());
    assert!(searched_courses[0].name.contains("Fundamentals"));
    
    Ok(())
}

async fn create_test_user(pool: &SqlitePool, username: &str, display_name: &str) -> Result<String, sqlx::Error> {
    let user_id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();
    
    sqlx::query!(
        "INSERT INTO users (id, username, email, display_name, created_at, updated_at)
         VALUES (?, ?, ?, ?, ?, ?)",
        user_id,
        username,
        format!("{}@example.com", username),
        display_name,
        now,
        now
    )
    .execute(pool)
    .await?;
    
    Ok(user_id)
}