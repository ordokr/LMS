use crate::models::unified_models::{
    User, Course, Group, Assignment, Topic, Submission,
    CourseStatus, TopicType, SubmissionStatus, SubmissionContentType
};
use crate::repositories::unified_repositories::Repository;
use super::test_utils::{init_test_db, TestRepositories, cleanup_test_db};

#[tokio::test]
async fn test_model_relationships() {
    // Initialize test database
    let pool = init_test_db().await;
    let repos = TestRepositories::new(pool);
    
    // Create users
    let instructor = User::new(
        None,
        "instructor".to_string(),
        "Test Instructor".to_string(),
        "instructor@example.com".to_string(),
    );
    let instructor = repos.user_repo.create(&instructor).await.expect("Failed to create instructor");
    
    let student1 = User::new(
        None,
        "student1".to_string(),
        "Student One".to_string(),
        "student1@example.com".to_string(),
    );
    let student1 = repos.user_repo.create(&student1).await.expect("Failed to create student1");
    
    let student2 = User::new(
        None,
        "student2".to_string(),
        "Student Two".to_string(),
        "student2@example.com".to_string(),
    );
    let student2 = repos.user_repo.create(&student2).await.expect("Failed to create student2");
    
    // Create a course
    let course = Course::new(
        None,
        "Integrated Course".to_string(),
        Some("This is a course for testing model relationships".to_string()),
        Some(instructor.id.clone()),
    );
    let course = repos.course_repo.create(&course).await.expect("Failed to create course");
    
    // Create a group
    let group = Group::new(
        None,
        "Study Group".to_string(),
        Some("A study group for the integrated course".to_string()),
        Some(student1.id.clone()),
        Some(course.id.clone()),
    );
    let group = repos.group_repo.create(&group).await.expect("Failed to create group");
    
    // Add student2 to the group
    let membership = crate::models::unified_models::GroupMembership {
        id: uuid::Uuid::new_v4().to_string(),
        group_id: group.id.clone(),
        user_id: student2.id.clone(),
        status: crate::models::unified_models::GroupMembershipStatus::Active,
        role: "member".to_string(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };
    repos.group_repo.add_member(&membership).await.expect("Failed to add member to group");
    
    // Create an assignment
    let assignment = Assignment::new(
        None,
        "Research Paper".to_string(),
        Some("Write a research paper on a topic of your choice".to_string()),
        course.id.clone(),
    );
    let mut assignment = assignment;
    assignment.points_possible = Some(100.0);
    assignment.due_date = Some(chrono::Utc::now() + chrono::Duration::days(14));
    let assignment = repos.assignment_repo.create(&assignment).await.expect("Failed to create assignment");
    
    // Publish the assignment
    let assignment = repos.assignment_repo.publish(&assignment.id).await.expect("Failed to publish assignment");
    
    // Create a discussion topic
    let topic = Topic::new(
        None,
        "Research Paper Discussion".to_string(),
        Some("Discuss your research paper topics here".to_string()),
    );
    let mut topic = topic;
    topic.course_id = Some(course.id.clone());
    topic.author_id = Some(instructor.id.clone());
    topic.assignment_id = Some(assignment.id.clone());
    topic.topic_type = TopicType::Assignment;
    let topic = repos.topic_repo.create(&topic).await.expect("Failed to create topic");
    
    // Create a submission for student1
    let submission1 = Submission::new(
        None,
        assignment.id.clone(),
        student1.id.clone(),
    );
    let mut submission1 = submission1;
    submission1.content = Some("This is my research paper on quantum computing".to_string());
    submission1.submission_type = Some(SubmissionContentType::OnlineTextEntry);
    let submission1 = repos.submission_repo.create(&submission1).await.expect("Failed to create submission1");
    
    // Submit the submission
    let submission1 = repos.submission_repo.submit(&submission1.id).await.expect("Failed to submit submission1");
    
    // Create a submission for student2
    let submission2 = Submission::new(
        None,
        assignment.id.clone(),
        student2.id.clone(),
    );
    let mut submission2 = submission2;
    submission2.content = Some("This is my research paper on artificial intelligence".to_string());
    submission2.submission_type = Some(SubmissionContentType::OnlineTextEntry);
    let submission2 = repos.submission_repo.create(&submission2).await.expect("Failed to create submission2");
    
    // Submit the submission
    let submission2 = repos.submission_repo.submit(&submission2.id).await.expect("Failed to submit submission2");
    
    // Grade student1's submission
    let submission1 = repos.submission_repo.grade(&submission1.id, &instructor.id, "A", Some(95.0)).await.expect("Failed to grade submission1");
    
    // Add a comment to student1's submission
    let submission1 = repos.submission_repo.add_comment(&submission1.id, &instructor.id, "Excellent work!").await.expect("Failed to add comment to submission1");
    
    // Test relationships
    
    // Course -> Assignments
    let course_assignments = repos.assignment_repo.find_by_course_id(&course.id).await.expect("Failed to find course assignments");
    assert!(!course_assignments.is_empty());
    assert!(course_assignments.iter().any(|a| a.id == assignment.id));
    
    // Course -> Topics
    let course_topics = repos.topic_repo.find_by_course_id(&course.id).await.expect("Failed to find course topics");
    assert!(!course_topics.is_empty());
    assert!(course_topics.iter().any(|t| t.id == topic.id));
    
    // Course -> Groups
    let course_groups = repos.group_repo.find_by_course_id(&course.id).await.expect("Failed to find course groups");
    assert!(!course_groups.is_empty());
    assert!(course_groups.iter().any(|g| g.id == group.id));
    
    // Assignment -> Submissions
    let assignment_submissions = repos.submission_repo.find_by_assignment_id(&assignment.id).await.expect("Failed to find assignment submissions");
    assert_eq!(assignment_submissions.len(), 2);
    assert!(assignment_submissions.iter().any(|s| s.id == submission1.id));
    assert!(assignment_submissions.iter().any(|s| s.id == submission2.id));
    
    // User -> Submissions
    let student1_submissions = repos.submission_repo.find_by_user_id(&student1.id).await.expect("Failed to find student1 submissions");
    assert!(!student1_submissions.is_empty());
    assert!(student1_submissions.iter().any(|s| s.id == submission1.id));
    
    let student2_submissions = repos.submission_repo.find_by_user_id(&student2.id).await.expect("Failed to find student2 submissions");
    assert!(!student2_submissions.is_empty());
    assert!(student2_submissions.iter().any(|s| s.id == submission2.id));
    
    // User -> Groups
    let student1_groups = repos.group_repo.find_by_user_id(&student1.id).await.expect("Failed to find student1 groups");
    assert!(!student1_groups.is_empty());
    assert!(student1_groups.iter().any(|g| g.id == group.id));
    
    let student2_groups = repos.group_repo.find_by_user_id(&student2.id).await.expect("Failed to find student2 groups");
    assert!(!student2_groups.is_empty());
    assert!(student2_groups.iter().any(|g| g.id == group.id));
    
    // Group -> Members
    let group_members = repos.group_repo.find_members(&group.id).await.expect("Failed to find group members");
    assert_eq!(group_members.len(), 2);
    assert!(group_members.iter().any(|m| m.user_id == student1.id));
    assert!(group_members.iter().any(|m| m.user_id == student2.id));
    
    // Assignment -> Topic
    let assignment_topic = repos.topic_repo.find_by_assignment_id(&assignment.id).await.expect("Failed to find assignment topic");
    assert!(assignment_topic.is_some());
    let assignment_topic = assignment_topic.unwrap();
    assert_eq!(assignment_topic.id, topic.id);
    
    // Clean up
    cleanup_test_db().await;
}
