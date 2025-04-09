#[cfg(test)]
mod tests {
    use sqlx::SqlitePool;
    use crate::api::module_api::{
        get_course_modules, 
        create_module, 
        get_module, 
        update_module, 
        delete_module, 
        reorder_modules
    };
    use crate::models::module::{Module, ModuleRequest};
    use crate::tests::common::{setup, create_test_course};

    #[tokio::test]
    async fn test_get_course_modules() {
        let pool = setup();
        let course_id = create_test_course(&pool).await;
        
        // Create a few modules
        for i in 1..=3 {
            let request = ModuleRequest {
                course_id: course_id.clone(),
                name: format!("Module {}", i),
                description: Some(format!("Description for module {}", i)),
                position: None,
                prerequisite_module_id: None,
                unlock_at: None,
                published: Some(true),
            };
            
            create_module(request, sqlx::pool::PoolConnection::from(pool.clone()))
                .await
                .expect("Failed to create module");
        }
        
        // Test fetching modules
        let modules = get_course_modules(course_id, sqlx::pool::PoolConnection::from(pool.clone()))
            .await
            .expect("Failed to fetch modules");
            
        assert_eq!(modules.len(), 3);
        assert_eq!(modules[0].name, "Module 1");
        assert_eq!(modules[1].name, "Module 2");
        assert_eq!(modules[2].name, "Module 3");
        
        // Test positions are set correctly
        assert_eq!(modules[0].position, 1);
        assert_eq!(modules[1].position, 2);
        assert_eq!(modules[2].position, 3);
    }
    
    #[tokio::test]
    async fn test_crud_operations() {
        let pool = setup();
        let course_id = create_test_course(&pool).await;
        
        // Create a module
        let request = ModuleRequest {
            course_id: course_id.clone(),
            name: "Test Module".to_string(),
            description: Some("Test Description".to_string()),
            position: None,
            prerequisite_module_id: None,
            unlock_at: None,
            published: Some(true),
        };
        
        let created_module = create_module(request.clone(), sqlx::pool::PoolConnection::from(pool.clone()))
            .await
            .expect("Failed to create module");
            
        assert_eq!(created_module.name, "Test Module");
        assert_eq!(created_module.description, Some("Test Description".to_string()));
        assert_eq!(created_module.published, true);
        
        // Get module
        let fetched_module = get_module(created_module.id.clone(), sqlx::pool::PoolConnection::from(pool.clone()))
            .await
            .expect("Failed to fetch module");
            
        assert_eq!(fetched_module.id, created_module.id);
        assert_eq!(fetched_module.name, created_module.name);
        
        // Update module
        let update_request = ModuleRequest {
            course_id: course_id.clone(),
            name: "Updated Module".to_string(),
            description: Some("Updated Description".to_string()),
            position: Some(1),
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
        
        assert_eq!(updated_module.name, "Updated Module");
        assert_eq!(updated_module.description, Some("Updated Description".to_string()));
        assert_eq!(updated_module.published, false);
        
        // Delete module
        delete_module(created_module.id.clone(), sqlx::pool::PoolConnection::from(pool.clone()))
            .await
            .expect("Failed to delete module");
            
        // Verify module is deleted
        let result = get_module(created_module.id.clone(), sqlx::pool::PoolConnection::from(pool.clone()))
            .await;
            
        assert!(result.is_err());
    }
    
    #[tokio::test]
    async fn test_reorder_modules() {
        let pool = setup();
        let course_id = create_test_course(&pool).await;
        
        // Create modules
        let mut module_ids = Vec::new();
        for i in 1..=3 {
            let request = ModuleRequest {
                course_id: course_id.clone(),
                name: format!("Module {}", i),
                description: None,
                position: None,
                prerequisite_module_id: None,
                unlock_at: None,
                published: Some(true),
            };
            
            let module = create_module(request, sqlx::pool::PoolConnection::from(pool.clone()))
                .await
                .expect("Failed to create module");
                
            module_ids.push(module.id);
        }
        
        // Reorder modules (reverse order)
        let reordered_ids = vec![module_ids[2].clone(), module_ids[1].clone(), module_ids[0].clone()];
        
        reorder_modules(course_id.clone(), reordered_ids, sqlx::pool::PoolConnection::from(pool.clone()))
            .await
            .expect("Failed to reorder modules");
            
        // Fetch modules to check order
        let modules = get_course_modules(course_id, sqlx::pool::PoolConnection::from(pool.clone()))
            .await
            .expect("Failed to fetch modules");
            
        // Verify the new order
        assert_eq!(modules[0].id, module_ids[2]);
        assert_eq!(modules[1].id, module_ids[1]);
        assert_eq!(modules[2].id, module_ids[0]);
        
        // Verify positions are updated
        assert_eq!(modules[0].position, 1);
        assert_eq!(modules[1].position, 2);
        assert_eq!(modules[2].position, 3);
    }
}