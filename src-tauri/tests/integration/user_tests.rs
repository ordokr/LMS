use crate::models::unified_models::User;
use crate::repositories::unified_repositories::Repository;
use super::test_utils::{init_test_db, TestRepositories, cleanup_test_db};

#[tokio::test]
async fn test_user_crud_operations() {
    // Initialize test database
    let pool = init_test_db().await;
    let repos = TestRepositories::new(pool);
    
    // Create a test user
    let user = User::new(
        None,
        "test_user".to_string(),
        "Test User".to_string(),
        "test@example.com".to_string(),
    );
    
    // Test create
    let created_user = repos.user_repo.create(&user).await.expect("Failed to create user");
    assert_eq!(created_user.username, "test_user");
    assert_eq!(created_user.name, "Test User");
    assert_eq!(created_user.email, "test@example.com");
    
    // Test find by ID
    let found_user = repos.user_repo.find_by_id(&created_user.id).await.expect("Failed to find user");
    assert!(found_user.is_some());
    let found_user = found_user.unwrap();
    assert_eq!(found_user.id, created_user.id);
    assert_eq!(found_user.username, created_user.username);
    
    // Test find by username
    let found_user = repos.user_repo.find_by_username("test_user").await.expect("Failed to find user by username");
    assert!(found_user.is_some());
    let found_user = found_user.unwrap();
    assert_eq!(found_user.id, created_user.id);
    
    // Test find by email
    let found_user = repos.user_repo.find_by_email("test@example.com").await.expect("Failed to find user by email");
    assert!(found_user.is_some());
    let found_user = found_user.unwrap();
    assert_eq!(found_user.id, created_user.id);
    
    // Test update
    let mut updated_user = found_user.clone();
    updated_user.name = "Updated User".to_string();
    let updated_user = repos.user_repo.update(&updated_user).await.expect("Failed to update user");
    assert_eq!(updated_user.name, "Updated User");
    
    // Test find all
    let all_users = repos.user_repo.find_all().await.expect("Failed to find all users");
    assert!(!all_users.is_empty());
    assert!(all_users.iter().any(|u| u.id == created_user.id));
    
    // Test count
    let count = repos.user_repo.count().await.expect("Failed to count users");
    assert!(count > 0);
    
    // Test delete
    repos.user_repo.delete(&created_user.id).await.expect("Failed to delete user");
    let deleted_user = repos.user_repo.find_by_id(&created_user.id).await.expect("Failed to check deleted user");
    assert!(deleted_user.is_none());
    
    // Clean up
    cleanup_test_db().await;
}
