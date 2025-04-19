use crate::models::unified_models::{Submission, SubmissionStatus, SubmissionContentType, User, Course, Assignment};
use crate::repositories::unified_repositories::Repository;
use super::test_utils::{init_test_db, TestRepositories, cleanup_test_db};

#[tokio::test]
async fn test_submission_crud_operations() {
    // Initialize test database
    let pool = init_test_db().await;
    let repos = TestRepositories::new(pool);
    
    // Create a test instructor
    let instructor = User::new(
        None,
        "submission_instructor".to_string(),
        "Submission Instructor".to_string(),
        "submission_instructor@example.com".to_string(),
    );
    let instructor = repos.user_repo.create(&instructor).await.expect("Failed to create instructor");
    
    // Create a test student
    let student = User::new(
        None,
        "submission_student".to_string(),
        "Submission Student".to_string(),
        "submission_student@example.com".to_string(),
    );
    let student = repos.user_repo.create(&student).await.expect("Failed to create student");
    
    // Create a test course
    let course = Course::new(
        None,
        "Submission Course".to_string(),
        Some("This is a course for submission testing".to_string()),
        Some(instructor.id.clone()),
    );
    let course = repos.course_repo.create(&course).await.expect("Failed to create course");
    
    // Create a test assignment
    let assignment = Assignment::new(
        None,
        "Submission Assignment".to_string(),
        Some("This is an assignment for submission testing".to_string()),
        course.id.clone(),
    );
    let assignment = repos.assignment_repo.create(&assignment).await.expect("Failed to create assignment");
    
    // Create a test submission
    let submission = Submission::new(
        None,
        assignment.id.clone(),
        student.id.clone(),
    );
    
    // Test create
    let created_submission = repos.submission_repo.create(&submission).await.expect("Failed to create submission");
    assert_eq!(created_submission.assignment_id, assignment.id);
    assert_eq!(created_submission.user_id, student.id);
    assert_eq!(created_submission.status, SubmissionStatus::NotSubmitted);
    assert_eq!(created_submission.attempt, 1);
    assert_eq!(created_submission.late, false);
    assert_eq!(created_submission.missing, false);
    assert_eq!(created_submission.excused, false);
    
    // Test find by ID
    let found_submission = repos.submission_repo.find_by_id(&created_submission.id).await.expect("Failed to find submission");
    assert!(found_submission.is_some());
    let found_submission = found_submission.unwrap();
    assert_eq!(found_submission.id, created_submission.id);
    assert_eq!(found_submission.assignment_id, created_submission.assignment_id);
    assert_eq!(found_submission.user_id, created_submission.user_id);
    
    // Test find by assignment and user
    let found_submission = repos.submission_repo.find_by_assignment_and_user(&assignment.id, &student.id).await.expect("Failed to find submission by assignment and user");
    assert!(found_submission.is_some());
    let found_submission = found_submission.unwrap();
    assert_eq!(found_submission.id, created_submission.id);
    
    // Test update
    let mut updated_submission = found_submission.clone();
    updated_submission.content = Some("This is my submission content".to_string());
    updated_submission.submission_type = Some(SubmissionContentType::OnlineTextEntry);
    let updated_submission = repos.submission_repo.update(&updated_submission).await.expect("Failed to update submission");
    assert_eq!(updated_submission.content, Some("This is my submission content".to_string()));
    assert_eq!(updated_submission.submission_type, Some(SubmissionContentType::OnlineTextEntry));
    
    // Test submission operations
    
    // Submit
    let submitted_submission = repos.submission_repo.submit(&created_submission.id).await.expect("Failed to submit submission");
    assert_eq!(submitted_submission.status, SubmissionStatus::Submitted);
    assert!(submitted_submission.submitted_at.is_some());
    assert!(submitted_submission.is_submitted());
    
    // Mark late
    let late_submission = repos.submission_repo.mark_late(&created_submission.id).await.expect("Failed to mark submission as late");
    assert_eq!(late_submission.late, true);
    assert!(late_submission.is_late());
    
    // Grade
    let graded_submission = repos.submission_repo.grade(&created_submission.id, &instructor.id, "A", Some(95.0)).await.expect("Failed to grade submission");
    assert_eq!(graded_submission.status, SubmissionStatus::Graded);
    assert_eq!(graded_submission.grade, Some("A".to_string()));
    assert_eq!(graded_submission.score, Some(95.0));
    assert_eq!(graded_submission.grader_id, Some(instructor.id.clone()));
    assert!(graded_submission.graded_at.is_some());
    assert!(graded_submission.is_graded());
    
    // Return to student
    let returned_submission = repos.submission_repo.return_to_student(&created_submission.id).await.expect("Failed to return submission to student");
    assert_eq!(returned_submission.status, SubmissionStatus::Returned);
    assert!(returned_submission.posted_at.is_some());
    assert!(returned_submission.is_graded());
    
    // Test comment operations
    
    // Add comment
    let commented_submission = repos.submission_repo.add_comment(&created_submission.id, &instructor.id, "Good work!").await.expect("Failed to add comment");
    assert!(commented_submission.comments.is_some());
    let comments = commented_submission.comments.unwrap();
    assert_eq!(comments.len(), 1);
    assert_eq!(comments[0].author_id, instructor.id);
    assert_eq!(comments[0].comment, "Good work!");
    
    // Add another comment
    let commented_submission = repos.submission_repo.add_comment(&created_submission.id, &student.id, "Thank you!").await.expect("Failed to add another comment");
    assert!(commented_submission.comments.is_some());
    let comments = commented_submission.comments.unwrap();
    assert_eq!(comments.len(), 2);
    assert_eq!(comments[1].author_id, student.id);
    assert_eq!(comments[1].comment, "Thank you!");
    
    // Test attachment operations
    
    // Add attachment
    let attachment_id = "attachment123";
    let attachment_submission = repos.submission_repo.add_attachment(&created_submission.id, attachment_id).await.expect("Failed to add attachment");
    assert!(attachment_submission.attachment_ids.contains(&attachment_id.to_string()));
    
    // Remove attachment
    let no_attachment_submission = repos.submission_repo.remove_attachment(&created_submission.id, attachment_id).await.expect("Failed to remove attachment");
    assert!(!no_attachment_submission.attachment_ids.contains(&attachment_id.to_string()));
    
    // Test find by assignment
    let assignment_submissions = repos.submission_repo.find_by_assignment_id(&assignment.id).await.expect("Failed to find submissions by assignment");
    assert!(!assignment_submissions.is_empty());
    assert!(assignment_submissions.iter().any(|s| s.id == created_submission.id));
    
    // Test find by user
    let user_submissions = repos.submission_repo.find_by_user_id(&student.id).await.expect("Failed to find submissions by user");
    assert!(!user_submissions.is_empty());
    assert!(user_submissions.iter().any(|s| s.id == created_submission.id));
    
    // Test find by course
    let course_submissions = repos.submission_repo.find_by_course_id(&course.id).await.expect("Failed to find submissions by course");
    assert!(!course_submissions.is_empty());
    assert!(course_submissions.iter().any(|s| s.id == created_submission.id));
    
    // Test find all
    let all_submissions = repos.submission_repo.find_all().await.expect("Failed to find all submissions");
    assert!(!all_submissions.is_empty());
    assert!(all_submissions.iter().any(|s| s.id == created_submission.id));
    
    // Test count
    let count = repos.submission_repo.count().await.expect("Failed to count submissions");
    assert!(count > 0);
    
    // Test delete
    repos.submission_repo.delete(&created_submission.id).await.expect("Failed to delete submission");
    let deleted_submission = repos.submission_repo.find_by_id(&created_submission.id).await.expect("Failed to check deleted submission");
    assert!(deleted_submission.is_none());
    
    // Clean up
    cleanup_test_db().await;
}
