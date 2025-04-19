use crate::models::unified_models::{Group, GroupJoinLevel, GroupMembership, GroupMembershipStatus, User, Course};
use crate::repositories::unified_repositories::Repository;
use super::test_utils::{init_test_db, TestRepositories, cleanup_test_db};

#[tokio::test]
async fn test_group_crud_operations() {
    // Initialize test database
    let pool = init_test_db().await;
    let repos = TestRepositories::new(pool);
    
    // Create a test user (leader)
    let leader = User::new(
        None,
        "group_leader".to_string(),
        "Group Leader".to_string(),
        "leader@example.com".to_string(),
    );
    let leader = repos.user_repo.create(&leader).await.expect("Failed to create leader");
    
    // Create a test course
    let course = Course::new(
        None,
        "Group Course".to_string(),
        Some("This is a course for group testing".to_string()),
        Some(leader.id.clone()),
    );
    let course = repos.course_repo.create(&course).await.expect("Failed to create course");
    
    // Create a test group
    let group = Group::new(
        None,
        "Test Group".to_string(),
        Some("This is a test group".to_string()),
        Some(leader.id.clone()),
        Some(course.id.clone()),
    );
    
    // Test create
    let created_group = repos.group_repo.create(&group).await.expect("Failed to create group");
    assert_eq!(created_group.name, "Test Group");
    assert_eq!(created_group.description, Some("This is a test group".to_string()));
    assert_eq!(created_group.leader_id, Some(leader.id.clone()));
    assert_eq!(created_group.course_id, Some(course.id.clone()));
    assert_eq!(created_group.join_level, GroupJoinLevel::Invitation);
    
    // Test find by ID
    let found_group = repos.group_repo.find_by_id(&created_group.id).await.expect("Failed to find group");
    assert!(found_group.is_some());
    let found_group = found_group.unwrap();
    assert_eq!(found_group.id, created_group.id);
    assert_eq!(found_group.name, created_group.name);
    
    // Test find by name
    let found_group = repos.group_repo.find_by_name("Test Group").await.expect("Failed to find group by name");
    assert!(found_group.is_some());
    let found_group = found_group.unwrap();
    assert_eq!(found_group.id, created_group.id);
    
    // Test find by course
    let found_groups = repos.group_repo.find_by_course_id(&course.id).await.expect("Failed to find groups by course");
    assert!(!found_groups.is_empty());
    assert!(found_groups.iter().any(|g| g.id == created_group.id));
    
    // Test find by leader
    let found_groups = repos.group_repo.find_by_leader_id(&leader.id).await.expect("Failed to find groups by leader");
    assert!(!found_groups.is_empty());
    assert!(found_groups.iter().any(|g| g.id == created_group.id));
    
    // Test update
    let mut updated_group = found_group.clone();
    updated_group.name = "Updated Group".to_string();
    updated_group.join_level = GroupJoinLevel::Open;
    let updated_group = repos.group_repo.update(&updated_group).await.expect("Failed to update group");
    assert_eq!(updated_group.name, "Updated Group");
    assert_eq!(updated_group.join_level, GroupJoinLevel::Open);
    
    // Create a test member
    let member = User::new(
        None,
        "group_member".to_string(),
        "Group Member".to_string(),
        "member@example.com".to_string(),
    );
    let member = repos.user_repo.create(&member).await.expect("Failed to create member");
    
    // Test add member
    let membership = GroupMembership {
        id: uuid::Uuid::new_v4().to_string(),
        group_id: created_group.id.clone(),
        user_id: member.id.clone(),
        status: GroupMembershipStatus::Active,
        role: "member".to_string(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };
    let created_membership = repos.group_repo.add_member(&membership).await.expect("Failed to add member");
    assert_eq!(created_membership.group_id, created_group.id);
    assert_eq!(created_membership.user_id, member.id);
    assert_eq!(created_membership.status, GroupMembershipStatus::Active);
    
    // Test find members
    let members = repos.group_repo.find_members(&created_group.id).await.expect("Failed to find members");
    assert_eq!(members.len(), 1);
    assert_eq!(members[0].user_id, member.id);
    
    // Test find groups for user
    let user_groups = repos.group_repo.find_by_user_id(&member.id).await.expect("Failed to find groups for user");
    assert!(!user_groups.is_empty());
    assert!(user_groups.iter().any(|g| g.id == created_group.id));
    
    // Test remove member
    repos.group_repo.remove_member(&created_group.id, &member.id).await.expect("Failed to remove member");
    let members_after_remove = repos.group_repo.find_members(&created_group.id).await.expect("Failed to find members after remove");
    assert!(members_after_remove.is_empty());
    
    // Test find all
    let all_groups = repos.group_repo.find_all().await.expect("Failed to find all groups");
    assert!(!all_groups.is_empty());
    assert!(all_groups.iter().any(|g| g.id == created_group.id));
    
    // Test count
    let count = repos.group_repo.count().await.expect("Failed to count groups");
    assert!(count > 0);
    
    // Test delete
    repos.group_repo.delete(&created_group.id).await.expect("Failed to delete group");
    let deleted_group = repos.group_repo.find_by_id(&created_group.id).await.expect("Failed to check deleted group");
    assert!(deleted_group.is_none());
    
    // Clean up
    cleanup_test_db().await;
}
