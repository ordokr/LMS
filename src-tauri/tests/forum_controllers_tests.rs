#[cfg(test)]
mod tests {
    use sqlx::SqlitePool;
    use crate::controllers::forum::{
        // Topics controller
        list_topics,
        get_topic,
        create_topic,
        update_topic,
        delete_topic,
        
        // Posts controller
        get_post,
        create_post,
        update_post,
        delete_post,
        
        // Categories controller
        list_categories,
        create_category,
        update_category,
        delete_category
    };
    use crate::models::forum::topic::TopicRequest;
    use crate::models::forum::post::PostRequest;
    use crate::models::forum::category::CategoryRequest;
    use crate::tests::common::{setup, create_test_user};
    
    #[tokio::test]
    async fn test_category_crud() {
        let pool = setup();
        let user_id = create_test_user(&pool).await;
        
        // Create a category
        let category_request = CategoryRequest {
            name: "Test Category".to_string(),
            description: Some("Test Description".to_string()),
            parent_id: None,
        };
        
        let category = create_category(
            user_id.clone(), 
            category_request.clone(), 
            sqlx::pool::PoolConnection::from(pool.clone())
        )
        .await
        .expect("Failed to create category");
        
        assert_eq!(category.name, "Test Category");
        assert_eq!(category.description, Some("Test Description".to_string()));
        assert_eq!(category.parent_id, None);
        
        // Update the category
        let update_request = CategoryRequest {
            name: "Updated Category".to_string(),
            description: Some("Updated Description".to_string()),
            parent_id: None,
        };
        
        let updated = update_category(
            category.id.clone(),
            user_id.clone(),
            update_request,
            sqlx::pool::PoolConnection::from(pool.clone())
        )
        .await
        .expect("Failed to update category");
        
        assert_eq!(updated.name, "Updated Category");
        assert_eq!(updated.description, Some("Updated Description".to_string()));
        
        // List categories
        let categories = list_categories(
            None, 
            sqlx::pool::PoolConnection::from(pool.clone())
        )
        .await
        .expect("Failed to list categories");
        
        assert_eq!(categories.len(), 1);
        assert_eq!(categories[0].id, category.id);
        
        // Create child category
        let child_request = CategoryRequest {
            name: "Child Category".to_string(),
            description: None,
            parent_id: Some(category.id.clone()),
        };
        
        let child = create_category(
            user_id.clone(),
            child_request,
            sqlx::pool::PoolConnection::from(pool.clone())
        )
        .await
        .expect("Failed to create child category");
        
        // Test parent-child relationship
        assert_eq!(child.parent_id, Some(category.id.clone()));
        
        // List with parent filter
        let children = list_categories(
            Some(category.id.clone()),
            sqlx::pool::PoolConnection::from(pool.clone())
        )
        .await
        .expect("Failed to list child categories");
        
        assert_eq!(children.len(), 1);
        assert_eq!(children[0].id, child.id);
        
        // Cannot delete parent category with children
        let delete_result = delete_category(
            category.id.clone(),
            user_id.clone(),
            sqlx::pool::PoolConnection::from(pool.clone())
        )
        .await;
        
        assert!(delete_result.is_err());
        
        // Delete child category
        delete_category(
            child.id.clone(),
            user_id.clone(),
            sqlx::pool::PoolConnection::from(pool.clone())
        )
        .await
        .expect("Failed to delete child category");
        
        // Now can delete parent
        delete_category(
            category.id.clone(),
            user_id.clone(),
            sqlx::pool::PoolConnection::from(pool.clone())
        )
        .await
        .expect("Failed to delete parent category");
        
        // Verify deletion
        let categories_after = list_categories(
            None,
            sqlx::pool::PoolConnection::from(pool.clone())
        )
        .await
        .expect("Failed to list categories after deletion");
        
        assert_eq!(categories_after.len(), 0);
    }
    
    #[tokio::test]
    async fn test_topic_post_flow() {
        let pool = setup();
        let user_id = create_test_user(&pool).await;
        
        // Create a category for topics
        let category_request = CategoryRequest {
            name: "Topic Test Category".to_string(),
            description: None,
            parent_id: None,
        };
        
        let category = create_category(
            user_id.clone(),
            category_request,
            sqlx::pool::PoolConnection::from(pool.clone())
        )
        .await
        .expect("Failed to create category for topics");
        
        // Create a topic
        let topic_request = TopicRequest {
            title: "Test Topic".to_string(),
            category_id: category.id.clone(),
            raw: "This is the content of the first post".to_string(),
            tags: Some(vec!["test".to_string(), "example".to_string()]),
        };
        
        let topic = create_topic(
            user_id.clone(),
            topic_request,
            sqlx::pool::PoolConnection::from(pool.clone())
        )
        .await
        .expect("Failed to create topic");
        
        assert_eq!(topic.title, "Test Topic");
        assert_eq!(topic.category_id, category.id);
        assert_eq!(topic.user_id, user_id);
        
        // Topic should have one post
        assert!(topic.posts.is_some());
        assert_eq!(topic.posts.as_ref().unwrap().len(), 1);
        
        // Add a second post to the topic
        let post_request = PostRequest {
            topic_id: topic.id.clone(),
            raw: "This is a reply to the topic".to_string(),
            reply_to_post_id: Some(topic.posts.as_ref().unwrap()[0].id.clone()),
        };
        
        let post = create_post(
            user_id.clone(),
            post_request,
            sqlx::pool::PoolConnection::from(pool.clone())
        )
        .await
        .expect("Failed to create post");
        
        assert_eq!(post.topic_id, topic.id);
        assert_eq!(post.user_id, user_id);
        assert_eq!(post.post_number, 2); // Second post
        
        // Get the topic to verify post count
        let updated_topic = get_topic(
            topic.id.clone(),
            sqlx::pool::PoolConnection::from(pool.clone())
        )
        .await
        .expect("Failed to get updated topic");
        
        assert_eq!(updated_topic.posts_count, 2);
        assert_eq!(updated_topic.posts.as_ref().unwrap().len(), 2);
        
        // Update the post
        let updated_content = "This is the updated reply content";
        update_post(
            post.id.clone(),
            user_id.clone(),
            updated_content.to_string(),
            sqlx::pool::PoolConnection::from(pool.clone())
        )
        .await
        .expect("Failed to update post");
        
        // Verify the update
        let updated_post = get_post(
            post.id.clone(),
            sqlx::pool::PoolConnection::from(pool.clone())
        )
        .await
        .expect("Failed to get updated post");
        
        assert_eq!(updated_post.raw, updated_content);
        
        // Delete the post
        delete_post(
            post.id.clone(),
            user_id.clone(),
            sqlx::pool::PoolConnection::from(pool.clone())
        )
        .await
        .expect("Failed to delete post");
        
        // Topic should now have 1 post again
        let final_topic = get_topic(
            topic.id.clone(),
            sqlx::pool::PoolConnection::from(pool.clone())
        )
        .await
        .expect("Failed to get final topic");
        
        assert_eq!(final_topic.posts_count, 1);
        
        // Check topic listing
        let topics = list_topics(
            Some(category.id.clone()),
            None,
            None,
            sqlx::pool::PoolConnection::from(pool.clone())
        )
        .await
        .expect("Failed to list topics");
        
        assert_eq!(topics.len(), 1);
        assert_eq!(topics[0].id, topic.id);
        
        // Delete the topic
        delete_topic(
            topic.id.clone(),
            user_id.clone(),
            sqlx::pool::PoolConnection::from(pool.clone())
        )
        .await
        .expect("Failed to delete topic");
        
        // Verify topic is deleted
        let topics_after = list_topics(
            Some(category.id.clone()),
            None,
            None,
            sqlx::pool::PoolConnection::from(pool.clone())
        )
        .await
        .expect("Failed to list topics after deletion");
        
        assert_eq!(topics_after.len(), 0);
    }
}