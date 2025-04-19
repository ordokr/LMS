use crate::models::unified_models::{Topic, TopicStatus, TopicVisibility, TopicType, User, Course};
use crate::repositories::unified_repositories::Repository;
use super::test_utils::{init_test_db, TestRepositories, cleanup_test_db};

#[tokio::test]
async fn test_topic_crud_operations() {
    // Initialize test database
    let pool = init_test_db().await;
    let repos = TestRepositories::new(pool);
    
    // Create a test user (author)
    let author = User::new(
        None,
        "topic_author".to_string(),
        "Topic Author".to_string(),
        "topic_author@example.com".to_string(),
    );
    let author = repos.user_repo.create(&author).await.expect("Failed to create author");
    
    // Create a test course
    let course = Course::new(
        None,
        "Topic Course".to_string(),
        Some("This is a course for topic testing".to_string()),
        Some(author.id.clone()),
    );
    let course = repos.course_repo.create(&course).await.expect("Failed to create course");
    
    // Create a test topic
    let topic = Topic::new(
        None,
        "Test Topic".to_string(),
        Some("This is a test topic".to_string()),
    );
    
    // Set additional properties
    let mut topic = topic;
    topic.course_id = Some(course.id.clone());
    topic.author_id = Some(author.id.clone());
    topic.topic_type = TopicType::Regular;
    
    // Test create
    let created_topic = repos.topic_repo.create(&topic).await.expect("Failed to create topic");
    assert_eq!(created_topic.title, "Test Topic");
    assert_eq!(created_topic.content, Some("This is a test topic".to_string()));
    assert_eq!(created_topic.course_id, Some(course.id.clone()));
    assert_eq!(created_topic.author_id, Some(author.id.clone()));
    assert_eq!(created_topic.status, TopicStatus::Open);
    assert_eq!(created_topic.visibility, TopicVisibility::Private);
    assert_eq!(created_topic.topic_type, TopicType::Regular);
    
    // Test find by ID
    let found_topic = repos.topic_repo.find_by_id(&created_topic.id).await.expect("Failed to find topic");
    assert!(found_topic.is_some());
    let found_topic = found_topic.unwrap();
    assert_eq!(found_topic.id, created_topic.id);
    assert_eq!(found_topic.title, created_topic.title);
    
    // Test find by course
    let found_topics = repos.topic_repo.find_by_course_id(&course.id).await.expect("Failed to find topics by course");
    assert!(!found_topics.is_empty());
    assert!(found_topics.iter().any(|t| t.id == created_topic.id));
    
    // Test find by author
    let found_topics = repos.topic_repo.find_by_author_id(&author.id).await.expect("Failed to find topics by author");
    assert!(!found_topics.is_empty());
    assert!(found_topics.iter().any(|t| t.id == created_topic.id));
    
    // Test update
    let mut updated_topic = found_topic.clone();
    updated_topic.title = "Updated Topic".to_string();
    updated_topic.content = Some("This is an updated topic".to_string());
    updated_topic.visibility = TopicVisibility::Public;
    let updated_topic = repos.topic_repo.update(&updated_topic).await.expect("Failed to update topic");
    assert_eq!(updated_topic.title, "Updated Topic");
    assert_eq!(updated_topic.content, Some("This is an updated topic".to_string()));
    assert_eq!(updated_topic.visibility, TopicVisibility::Public);
    
    // Test topic status operations
    
    // Close
    let closed_topic = repos.topic_repo.close(&created_topic.id).await.expect("Failed to close topic");
    assert_eq!(closed_topic.status, TopicStatus::Closed);
    assert_eq!(closed_topic.is_locked, true);
    assert!(!closed_topic.is_open_for_posting());
    
    // Open
    let opened_topic = repos.topic_repo.open(&created_topic.id).await.expect("Failed to open topic");
    assert_eq!(opened_topic.status, TopicStatus::Open);
    assert_eq!(opened_topic.is_locked, false);
    assert!(opened_topic.is_open_for_posting());
    
    // Archive
    let archived_topic = repos.topic_repo.archive(&created_topic.id).await.expect("Failed to archive topic");
    assert_eq!(archived_topic.status, TopicStatus::Archived);
    assert_eq!(archived_topic.is_locked, true);
    assert!(!archived_topic.is_open_for_posting());
    
    // Test pin/unpin
    
    // Pin
    let pinned_topic = repos.topic_repo.pin(&created_topic.id).await.expect("Failed to pin topic");
    assert_eq!(pinned_topic.is_pinned, true);
    
    // Unpin
    let unpinned_topic = repos.topic_repo.unpin(&created_topic.id).await.expect("Failed to unpin topic");
    assert_eq!(unpinned_topic.is_pinned, false);
    
    // Test tag operations
    
    // Add tag
    let tagged_topic = repos.topic_repo.add_tag(&created_topic.id, "test").await.expect("Failed to add tag");
    assert!(tagged_topic.tags.contains(&"test".to_string()));
    
    // Add another tag
    let tagged_topic = repos.topic_repo.add_tag(&created_topic.id, "topic").await.expect("Failed to add another tag");
    assert!(tagged_topic.tags.contains(&"test".to_string()));
    assert!(tagged_topic.tags.contains(&"topic".to_string()));
    
    // Remove tag
    let untagged_topic = repos.topic_repo.remove_tag(&created_topic.id, "test").await.expect("Failed to remove tag");
    assert!(!untagged_topic.tags.contains(&"test".to_string()));
    assert!(untagged_topic.tags.contains(&"topic".to_string()));
    
    // Test counter operations
    
    // Increment view count
    let viewed_topic = repos.topic_repo.increment_view_count(&created_topic.id).await.expect("Failed to increment view count");
    assert_eq!(viewed_topic.view_count, Some(1));
    
    // Increment view count again
    let viewed_topic = repos.topic_repo.increment_view_count(&created_topic.id).await.expect("Failed to increment view count again");
    assert_eq!(viewed_topic.view_count, Some(2));
    
    // Increment reply count
    let replied_topic = repos.topic_repo.increment_reply_count(&created_topic.id).await.expect("Failed to increment reply count");
    assert_eq!(replied_topic.reply_count, Some(1));
    assert!(replied_topic.last_reply_at.is_some());
    
    // Test find all
    let all_topics = repos.topic_repo.find_all().await.expect("Failed to find all topics");
    assert!(!all_topics.is_empty());
    assert!(all_topics.iter().any(|t| t.id == created_topic.id));
    
    // Test count
    let count = repos.topic_repo.count().await.expect("Failed to count topics");
    assert!(count > 0);
    
    // Test delete
    repos.topic_repo.delete_topic(&created_topic.id).await.expect("Failed to delete topic");
    let deleted_topic = repos.topic_repo.find_by_id(&created_topic.id).await.expect("Failed to check deleted topic");
    assert!(deleted_topic.is_none());
    
    // Clean up
    cleanup_test_db().await;
}
