## 6. Begin Integration API Tests

#[cfg(test)]
mod tests {
    use crate::api::courses::{get_courses, create_course};
    use crate::api::assignments::{get_assignments, create_assignment};
    use crate::api::submissions::{get_submissions, create_submission};
    use crate::api::integration::{create_course_category_mapping, sync_course_category};
    use crate::models::course::{Course, CourseCreate, CourseStatus};
    use crate::models::assignment::{Assignment, AssignmentStatus};
    use crate::models::submission::{SubmissionCreate};
    use crate::db::test_utils::{create_test_db_pool, clean_test_db};
    use uuid::Uuid;
    use std::sync::Arc;
    use sqlx::SqlitePool;
    use crate::api::module_api::{
        get_course_modules, 
        create_module, 
        get_module, 
        update_module,
        delete_module
    };
    use crate::api::module_item_api::{
        create_module_item,
        get_module_items,
        delete_module_item
    };
    use crate::models::module::{Module, ModuleRequest};
    use crate::models::module_item::{ModuleItem, ModuleItemRequest};
    use crate::tests::common::{setup, create_test_course};

    #[tokio::test]
    async fn test_course_assignment_submission_flow() {
        // Set up test DB
        let pool = create_test_db_pool().await;
        clean_test_db(&pool).await;
        
        // Create repositories
        let course_repo = Arc::new(crate::db::course_repository::SqliteCourseRepository::new(pool.clone()));
        let assignment_repo = Arc::new(crate::db::assignment_repository::SqliteAssignmentRepository::new(pool.clone()));
        let submission_repo = Arc::new(crate::db::submission_repository::SqliteSubmissionRepository::new(pool.clone()));
        
        // 1. Create a course
        let course_create = Course {
            id: Uuid::new_v4().to_string(),
            title: "Test Course".to_string(),
            description: "This is a test course".to_string(),
            status: CourseStatus::Active,
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
            modules: None,
        };
        
        let course = create_course(
            course_create.clone(), 
            tauri::State::new(course_repo.clone())
        ).await.expect("Failed to create course");
        
        // 2. Create an assignment for the course
        let assignment = Assignment {
            id: Uuid::new_v4().to_string(),
            course_id: course.id.clone(),
            title: "Test Assignment".to_string(),
            description: "This is a test assignment".to_string(),
            due_date: Some(chrono::Utc::now().to_rfc3339()),
            points_possible: 100.0,
            status: AssignmentStatus::Published,
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        };
        
        let assignment = create_assignment(
            assignment.clone(), 
            tauri::State::new(assignment_repo.clone())
        ).await.expect("Failed to create assignment");
        
        // 3. Create a submission for the assignment
        let submission_create = SubmissionCreate {
            assignment_id: assignment.id.clone(),
            user_id: "test-user-1".to_string(),
            content: "This is my submission".to_string(),
            attachments: vec![],
        };
        
        let submission = create_submission(
            submission_create.clone(), 
            tauri::State::new(submission_repo.clone())
        ).await.expect("Failed to create submission");
        
        // 4. Verify we can retrieve the submission
        let submissions = get_submissions(
            assignment.id.clone(), 
            tauri::State::new(submission_repo.clone())
        ).await.expect("Failed to get submissions");
        
        assert_eq!(submissions.len(), 1);
        assert_eq!(submissions[0].id, submission.id);
    }
    
    #[tokio::test]
    async fn test_module_management_flow() {
        let pool = setup();
        let course_id = create_test_course(&pool).await;
        
        // Step 1: Create a module
        let module_request = ModuleRequest {
            course_id: course_id.clone(),
            name: "Complete Module Flow Test".to_string(),
            description: Some("Testing the full module flow".to_string()),
            position: None,
            prerequisite_module_id: None,
            unlock_at: None,
            published: Some(true),
        };
        
        let created_module = create_module(module_request, sqlx::pool::PoolConnection::from(pool.clone()))
            .await
            .expect("Failed to create module");
            
        // Step 2: Add items to the module
        let item1_request = ModuleItemRequest {
            module_id: created_module.id.clone(),
            title: "Assignment Item".to_string(),
            item_type: "Assignment".to_string(),
            content_id: Some("assignment-123".to_string()),
            external_url: None,
            position: None,
            published: Some(true),
        };
        
        let item2_request = ModuleItemRequest {
            module_id: created_module.id.clone(),
            title: "External URL".to_string(),
            item_type: "ExternalUrl".to_string(),
            content_id: None,
            external_url: Some("https://example.com".to_string()),
            position: None,
            published: Some(true),
        };
        
        let item1 = create_module_item(item1_request, sqlx::pool::PoolConnection::from(pool.clone()))
            .await
            .expect("Failed to create first item");
            
        let item2 = create_module_item(item2_request, sqlx::pool::PoolConnection::from(pool.clone()))
            .await
            .expect("Failed to create second item");
            
        // Step 3: Verify items were added
        let items = get_module_items(created_module.id.clone(), sqlx::pool::PoolConnection::from(pool.clone()))
            .await
            .expect("Failed to get module items");
            
        assert_eq!(items.len(), 2);
        assert_eq!(items[0].title, "Assignment Item");
        assert_eq!(items[1].title, "External URL");
        
        // Step 4: Update the module
        let update_request = ModuleRequest {
            course_id: course_id.clone(),
            name: "Updated Flow Test".to_string(),
            description: Some("Updated description".to_string()),
            position: Some(created_module.position),
            prerequisite_module_id: None,
            unlock_at: None,
            published: Some(false),
        };
        
        let updated_module = update_module(
            created_module.id.clone(), 
            update_request, 
            sqlx::pool::PoolConnection::from(pool.clone())
        )
        .await
        .expect("Failed to update module");
        
        assert_eq!(updated_module.name, "Updated Flow Test");
        assert_eq!(updated_module.published, false);
        
        // Step 5: Delete an item
        delete_module_item(item1.id.clone(), sqlx::pool::PoolConnection::from(pool.clone()))
            .await
            .expect("Failed to delete item");
            
        // Verify item was deleted
        let remaining_items = get_module_items(created_module.id.clone(), sqlx::pool::PoolConnection::from(pool.clone()))
            .await
            .expect("Failed to get remaining items");
            
        assert_eq!(remaining_items.len(), 1);
        assert_eq!(remaining_items[0].id, item2.id);
        
        // Step 6: Delete the module
        delete_module(created_module.id.clone(), sqlx::pool::PoolConnection::from(pool.clone()))
            .await
            .expect("Failed to delete module");
            
        // Verify module was deleted
        let modules = get_course_modules(course_id, sqlx::pool::PoolConnection::from(pool.clone()))
            .await
            .expect("Failed to get course modules");
            
        assert!(modules.is_empty());
        
        // Verify cascade deletion of items
        let items = get_module_items(created_module.id.clone(), sqlx::pool::PoolConnection::from(pool.clone()))
            .await
            .expect("Failed to get module items after module deletion");
            
        assert!(items.is_empty());
    }
}