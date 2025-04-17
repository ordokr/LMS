use crate::services::integration::conflict_resolver::{ConflictResolver, ConflictStrategy};
use crate::sync::version_vector::{VersionVector, CausalRelation};
use crate::models::forum::topic::Topic;
use crate::models::forum::post::Post;
use crate::error::Error;
use chrono::Utc;
use uuid::Uuid;

#[tokio::test]
async fn test_conflict_resolution_strategies() -> Result<(), Error> {
    // Create test data
    let now = Utc::now();
    let earlier = now - chrono::Duration::hours(1);
    let later = now + chrono::Duration::hours(1);
    
    // Create two topics with conflicts
    let canvas_topic = Topic {
        id: Uuid::new_v4(),
        category_id: Uuid::new_v4(),
        title: "Canvas Topic".to_string(),
        content: "Canvas Content".to_string(),
        author_id: Uuid::new_v4(),
        created_at: now,
        updated_at: now,
        last_post_at: Some(now),
        publish_at: None,
        is_pinned: false,
        is_closed: false,
        is_question: false,
        assignment_id: None,
        read_status: false,
        view_count: 10,
        canvas_discussion_id: Some("canvas-1".to_string()),
        discourse_topic_id: None,
        sync_status: crate::models::forum::topic::SyncStatus::SyncedWithCanvas,
        tags: vec!["canvas".to_string(), "shared".to_string()],
        post_ids: vec![],
    };
    
    let discourse_topic = Topic {
        id: Uuid::new_v4(),
        category_id: Uuid::new_v4(),
        title: "Discourse Topic".to_string(),
        content: "Discourse Content".to_string(),
        author_id: Uuid::new_v4(),
        created_at: now,
        updated_at: now,
        last_post_at: Some(now),
        publish_at: None,
        is_pinned: true, // Difference
        is_closed: false,
        is_question: false,
        assignment_id: None,
        read_status: false,
        view_count: 20, // Difference
        canvas_discussion_id: None,
        discourse_topic_id: Some(123),
        sync_status: crate::models::forum::topic::SyncStatus::SyncedWithDiscourse,
        tags: vec!["discourse".to_string(), "shared".to_string()],
        post_ids: vec![],
    };
    
    // Test prefer Canvas strategy
    let resolver = ConflictResolver::new(ConflictStrategy::PreferCanvas);
    let resolved = resolver.resolve_topic_conflict(&canvas_topic, &discourse_topic)?;
    
    assert_eq!(resolved.title, "Canvas Topic");
    assert_eq!(resolved.content, "Canvas Content");
    assert_eq!(resolved.is_pinned, false); // Canvas value
    
    // Test prefer Discourse strategy
    let resolver = ConflictResolver::new(ConflictStrategy::PreferDiscourse);
    let resolved = resolver.resolve_topic_conflict(&canvas_topic, &discourse_topic)?;
    
    assert_eq!(resolved.title, "Discourse Topic");
    assert_eq!(resolved.content, "Discourse Content");
    assert_eq!(resolved.is_pinned, true); // Discourse value
    
    // Test preferring most recent when Canvas is newer
    let canvas_topic_newer = Topic {
        updated_at: later,
        ..canvas_topic.clone()
    };
    
    let discourse_topic_older = Topic {
        updated_at: earlier,
        ..discourse_topic.clone()
    };
    
    let resolver = ConflictResolver::new(ConflictStrategy::PreferMostRecent);
    let resolved = resolver.resolve_topic_conflict(&canvas_topic_newer, &discourse_topic_older)?;
    
    assert_eq!(resolved.title, "Canvas Topic"); // Canvas is newer
    
    // Test preferring most recent when Discourse is newer
    let canvas_topic_older = Topic {
        updated_at: earlier,
        ..canvas_topic.clone()
    };
    
    let discourse_topic_newer = Topic {
        updated_at: later,
        ..discourse_topic.clone()
    };
    
    let resolver = ConflictResolver::new(ConflictStrategy::PreferMostRecent);
    let resolved = resolver.resolve_topic_conflict(&canvas_topic_older, &discourse_topic_newer)?;
    
    assert_eq!(resolved.title, "Discourse Topic"); // Discourse is newer
    
    // Test merge strategies
    // Canvas Topic should have precedence for content but Discourse values for shared fields should be preserved
    let resolver = ConflictResolver::new(ConflictStrategy::MergePreferCanvas);
    let resolved = resolver.resolve_topic_conflict(&canvas_topic, &discourse_topic)?;
    
    assert_eq!(resolved.title, "Canvas Topic"); // Canvas title (content)
    assert_eq!(resolved.content, "Canvas Content"); // Canvas content
    assert_eq!(resolved.view_count, 20); // Discourse view count (metadata)
    assert!(resolved.tags.contains(&"canvas".to_string())); // Should have tags from both
    assert!(resolved.tags.contains(&"discourse".to_string())); // Should have tags from both
    assert!(resolved.tags.contains(&"shared".to_string())); // Should have shared tags
    
    // Test with version vectors
    let mut canvas_vector = VersionVector::new();
    canvas_vector.increment("device1");
    
    let mut discourse_vector = VersionVector::new();
    discourse_vector.increment("device2");
    
    // First test with concurrent modifications (real conflict)
    let resolver = ConflictResolver::new(ConflictStrategy::PreferCanvas);
    let resolved = resolver.resolve_topic_conflict_with_version_vector(
        &canvas_topic,
        &discourse_topic,
        &canvas_vector,
        &discourse_vector
    )?;
    
    // When vectors are concurrent, should fall back to the strategy
    assert_eq!(resolved.title, "Canvas Topic"); // PreferCanvas strategy
    
    // Now test with causal relationship - canvas happens before discourse
    let mut canvas_vector2 = VersionVector::new();
    canvas_vector2.increment("device1");
    
    let mut discourse_vector2 = canvas_vector2.clone(); // Start with canvas vector
    discourse_vector2.increment("device2"); // Add discourse changes
    
    // Here Canvas vector happens before Discourse vector
    assert_eq!(canvas_vector2.causal_relation(&discourse_vector2), CausalRelation::HappensBefore);
    
    let resolved = resolver.resolve_topic_conflict_with_version_vector(
        &canvas_topic,
        &discourse_topic,
        &canvas_vector2,
        &discourse_vector2
    )?;
    
    // When there's a causal relationship, should pick the "after" one regardless of strategy
    assert_eq!(resolved.title, "Discourse Topic"); // Discourse is causally after
    
    Ok(())
}

#[tokio::test]
async fn test_post_conflict_resolution() -> Result<(), Error> {
    // Create test data
    let now = Utc::now();
    
    // Create two posts with conflicts
    let canvas_post = Post {
        id: Uuid::new_v4(),
        topic_id: Uuid::new_v4(),
        author_id: Uuid::new_v4(),
        parent_id: None,
        content: "Canvas Post Content".to_string(),
        html_content: None,
        created_at: now,
        updated_at: now,
        likes: 5,
        is_solution: false,
        score: None,
        read_status: false,
        attachment_ids: vec![],
        canvas_entry_id: Some("canvas-entry-1".to_string()),
        discourse_post_id: None,
        sync_status: crate::models::forum::post::SyncStatus::SyncedWithCanvas,
    };
    
    let discourse_post = Post {
        id: Uuid::new_v4(),
        topic_id: Uuid::new_v4(),
        author_id: Uuid::new_v4(),
        parent_id: None,
        content: "Discourse Post Content".to_string(),
        html_content: Some("<p>Discourse Post Content</p>".to_string()),
        created_at: now,
        updated_at: now,
        likes: 10, // More likes in Discourse
        is_solution: true, // Marked as solution in Discourse
        score: None,
        read_status: false,
        attachment_ids: vec![],
        canvas_entry_id: None,
        discourse_post_id: Some(456),
        sync_status: crate::models::forum::post::SyncStatus::SyncedWithDiscourse,
    };
    
    // Test merge strategies for posts
    let resolver = ConflictResolver::new(ConflictStrategy::MergePreferCanvas);
    let resolved = resolver.resolve_post_conflict(&canvas_post, &discourse_post)?;
    
    assert_eq!(resolved.content, "Canvas Post Content"); // Canvas content (preferred)
    assert_eq!(resolved.html_content, Some("<p>Discourse Post Content</p>".to_string())); // Discourse HTML
    assert_eq!(resolved.likes, 10); // Higher like count from Discourse
    assert_eq!(resolved.is_solution, true); // Solution status from Discourse
    
    // Test merge with version vectors
    let mut canvas_vector = VersionVector::new();
    canvas_vector.increment("device1");
    canvas_vector.increment("device1");
    
    let mut discourse_vector = canvas_vector.clone();
    discourse_vector.increment("device2");
    
    // Canvas happens before Discourse
    assert_eq!(canvas_vector.causal_relation(&discourse_vector), CausalRelation::HappensBefore);
    
    // Even with PreferCanvas strategy, should pick Discourse because it's causally after
    let resolver = ConflictResolver::new(ConflictStrategy::PreferCanvas);
    let resolved = resolver.resolve_post_conflict_with_version_vector(
        &canvas_post,
        &discourse_post,
        &canvas_vector,
        &discourse_vector
    )?;
    
    assert_eq!(resolved.content, "Discourse Post Content"); // Discourse content
    
    Ok(())
}