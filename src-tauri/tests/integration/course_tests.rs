use crate::models::unified_models::{Course, CourseStatus, CourseVisibility, User};
use crate::repositories::unified_repositories::Repository;
use super::test_utils::{init_test_db, TestRepositories, cleanup_test_db};

#[tokio::test]
async fn test_course_crud_operations() {
    // Initialize test database
    let pool = init_test_db().await;
    let repos = TestRepositories::new(pool);
    
    // Create a test user (instructor)
    let instructor = User::new(
        None,
        "instructor".to_string(),
        "Test Instructor".to_string(),
        "instructor@example.com".to_string(),
    );
    let instructor = repos.user_repo.create(&instructor).await.expect("Failed to create instructor");
    
    // Create a test course
    let course = Course::new(
        None,
        "Test Course".to_string(),
        Some("This is a test course".to_string()),
        Some(instructor.id.clone()),
    );
    
    // Test create
    let created_course = repos.course_repo.create(&course).await.expect("Failed to create course");
    assert_eq!(created_course.title, "Test Course");
    assert_eq!(created_course.description, Some("This is a test course".to_string()));
    assert_eq!(created_course.instructor_id, Some(instructor.id.clone()));
    assert_eq!(created_course.status, CourseStatus::Active);
    assert_eq!(created_course.visibility, CourseVisibility::Private);
    
    // Test find by ID
    let found_course = repos.course_repo.find_by_id(&created_course.id).await.expect("Failed to find course");
    assert!(found_course.is_some());
    let found_course = found_course.unwrap();
    assert_eq!(found_course.id, created_course.id);
    assert_eq!(found_course.title, created_course.title);
    
    // Test find by title
    let found_course = repos.course_repo.find_by_title("Test Course").await.expect("Failed to find course by title");
    assert!(found_course.is_some());
    let found_course = found_course.unwrap();
    assert_eq!(found_course.id, created_course.id);
    
    // Test find by instructor
    let found_courses = repos.course_repo.find_by_instructor_id(&instructor.id).await.expect("Failed to find courses by instructor");
    assert!(!found_courses.is_empty());
    assert!(found_courses.iter().any(|c| c.id == created_course.id));
    
    // Test update
    let mut updated_course = found_course.clone();
    updated_course.title = "Updated Course".to_string();
    updated_course.status = CourseStatus::Completed;
    updated_course.visibility = CourseVisibility::Public;
    let updated_course = repos.course_repo.update(&updated_course).await.expect("Failed to update course");
    assert_eq!(updated_course.title, "Updated Course");
    assert_eq!(updated_course.status, CourseStatus::Completed);
    assert_eq!(updated_course.visibility, CourseVisibility::Public);
    
    // Test find all
    let all_courses = repos.course_repo.find_all().await.expect("Failed to find all courses");
    assert!(!all_courses.is_empty());
    assert!(all_courses.iter().any(|c| c.id == created_course.id));
    
    // Test count
    let count = repos.course_repo.count().await.expect("Failed to count courses");
    assert!(count > 0);
    
    // Test delete
    repos.course_repo.delete(&created_course.id).await.expect("Failed to delete course");
    let deleted_course = repos.course_repo.find_by_id(&created_course.id).await.expect("Failed to check deleted course");
    assert!(deleted_course.is_none());
    
    // Clean up
    cleanup_test_db().await;
}
