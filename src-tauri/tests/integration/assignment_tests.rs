use crate::models::unified_models::{Assignment, SubmissionType, GradingType, AssignmentStatus, User, Course};
use crate::repositories::unified_repositories::Repository;
use super::test_utils::{init_test_db, TestRepositories, cleanup_test_db};

#[tokio::test]
async fn test_assignment_crud_operations() {
    // Initialize test database
    let pool = init_test_db().await;
    let repos = TestRepositories::new(pool);
    
    // Create a test user (instructor)
    let instructor = User::new(
        None,
        "assignment_instructor".to_string(),
        "Assignment Instructor".to_string(),
        "assignment_instructor@example.com".to_string(),
    );
    let instructor = repos.user_repo.create(&instructor).await.expect("Failed to create instructor");
    
    // Create a test course
    let course = Course::new(
        None,
        "Assignment Course".to_string(),
        Some("This is a course for assignment testing".to_string()),
        Some(instructor.id.clone()),
    );
    let course = repos.course_repo.create(&course).await.expect("Failed to create course");
    
    // Create a test assignment
    let assignment = Assignment::new(
        None,
        "Test Assignment".to_string(),
        Some("This is a test assignment".to_string()),
        course.id.clone(),
    );
    
    // Test create
    let created_assignment = repos.assignment_repo.create(&assignment).await.expect("Failed to create assignment");
    assert_eq!(created_assignment.title, "Test Assignment");
    assert_eq!(created_assignment.description, Some("This is a test assignment".to_string()));
    assert_eq!(created_assignment.course_id, course.id);
    assert_eq!(created_assignment.status, AssignmentStatus::Draft);
    assert_eq!(created_assignment.grading_type, GradingType::Points);
    assert!(created_assignment.submission_types.contains(&SubmissionType::OnlineTextEntry));
    
    // Test find by ID
    let found_assignment = repos.assignment_repo.find_by_id(&created_assignment.id).await.expect("Failed to find assignment");
    assert!(found_assignment.is_some());
    let found_assignment = found_assignment.unwrap();
    assert_eq!(found_assignment.id, created_assignment.id);
    assert_eq!(found_assignment.title, created_assignment.title);
    
    // Test find by course
    let found_assignments = repos.assignment_repo.find_by_course_id(&course.id).await.expect("Failed to find assignments by course");
    assert!(!found_assignments.is_empty());
    assert!(found_assignments.iter().any(|a| a.id == created_assignment.id));
    
    // Test update
    let mut updated_assignment = found_assignment.clone();
    updated_assignment.title = "Updated Assignment".to_string();
    updated_assignment.status = AssignmentStatus::Published;
    updated_assignment.points_possible = Some(100.0);
    updated_assignment.due_date = Some(chrono::Utc::now() + chrono::Duration::days(7));
    updated_assignment.grading_type = GradingType::Percentage;
    let updated_assignment = repos.assignment_repo.update(&updated_assignment).await.expect("Failed to update assignment");
    assert_eq!(updated_assignment.title, "Updated Assignment");
    assert_eq!(updated_assignment.status, AssignmentStatus::Published);
    assert_eq!(updated_assignment.points_possible, Some(100.0));
    assert_eq!(updated_assignment.grading_type, GradingType::Percentage);
    
    // Test publish
    let published_assignment = repos.assignment_repo.publish(&created_assignment.id).await.expect("Failed to publish assignment");
    assert_eq!(published_assignment.status, AssignmentStatus::Published);
    
    // Test unpublish
    let unpublished_assignment = repos.assignment_repo.unpublish(&created_assignment.id).await.expect("Failed to unpublish assignment");
    assert_eq!(unpublished_assignment.status, AssignmentStatus::Draft);
    
    // Test find all
    let all_assignments = repos.assignment_repo.find_all().await.expect("Failed to find all assignments");
    assert!(!all_assignments.is_empty());
    assert!(all_assignments.iter().any(|a| a.id == created_assignment.id));
    
    // Test count
    let count = repos.assignment_repo.count().await.expect("Failed to count assignments");
    assert!(count > 0);
    
    // Test delete
    repos.assignment_repo.delete(&created_assignment.id).await.expect("Failed to delete assignment");
    let deleted_assignment = repos.assignment_repo.find_by_id(&created_assignment.id).await.expect("Failed to check deleted assignment");
    assert!(deleted_assignment.is_none());
    
    // Clean up
    cleanup_test_db().await;
}
