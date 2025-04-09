## 47. Create a Module API Integration Test

```rust
#[cfg(test)]
mod tests {
    use tauri::test::mock_invoke_context;
    use crate::api::modules::{
        create_module, get_modules, get_module, update_module, delete_module, reorder_modules,
        create_module_item, get_module_items, get_module_item, update_module_item, delete_module_item, reorder_module_items
    };
    use crate::models::module::{ModuleCreate, ModuleUpdate, ModuleStatus, ModuleItemCreate, ModuleItemUpdate, ModuleItemType};
    use crate::db::module_repository::{ModuleRepository, SqliteModuleRepository};
    use crate::db::test_utils::{create_test_db_pool, clean_test_db};
    use std::sync::Arc;

    #[tokio::test]
    async fn test_modules_integration() {
        // Setup
        let pool = create_test_db_pool().await;
        clean_test_db(&pool).await;
        
        // Create necessary tables
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS modules (
                id TEXT PRIMARY KEY,
                course_id TEXT NOT NULL,
                title TEXT NOT NULL,
                position INTEGER NOT NULL,
                items_count INTEGER NOT NULL DEFAULT 0,
                publish_final_grade BOOLEAN NOT NULL DEFAULT 0,
                published BOOLEAN NOT NULL DEFAULT 1,
                status TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )"
        ).execute(&pool).await.unwrap();
        
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS module_items (
                id TEXT PRIMARY KEY,
                module_id TEXT NOT NULL,
                title TEXT NOT NULL,
                position INTEGER NOT NULL,
                item_type TEXT NOT NULL,
                content_id TEXT,
                content_type TEXT,
                url TEXT,
                page_url TEXT,
                published BOOLEAN NOT NULL DEFAULT 1,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                FOREIGN KEY (module_id) REFERENCES modules(id)
            )"
        ).execute(&pool).await.unwrap();
        
        let module_repo = Arc::new(SqliteModuleRepository::new(pool));
        
        // Test course_id
        let course_id = "test-course-id".to_string();
        
        // Test create_module
        let module_create = ModuleCreate {
            course_id: course_id.clone(),
            title: "Test Module".to_string(),
            position: None,
            publish_final_grade: None,
            published: None,
        };
        
        let module = create_module(
            module_create, 
            tauri::State::new(module_repo.clone())
        ).await.unwrap();
        
        assert_eq!(module.course_id, course_id);
        assert_eq!(module.title, "Test Module");
        assert_eq!(module.position, 1);
        assert_eq!(module.items_count, 0);
        assert_eq!(module.status, ModuleStatus::Active);
        
        // Test get_module
        let fetched_module = get_module(
            module.id.clone(),
            tauri::State::new(module_repo.clone())
        ).await.unwrap();
        
        assert_eq!(fetched_module.id, module.id);
        assert_eq!(fetched_module.title, module.title);
        
        // Test get_modules
        let modules = get_modules(
            course_id.clone(),
            tauri::State::new(module_repo.clone())
        ).await.unwrap();
        
        assert_eq!(modules.len(), 1);
        assert_eq!(modules[0].id, module.id);
        
        // Test update_module
        let module_update = ModuleUpdate {
            title: Some("Updated Module".to_string()),
            position: None,
            publish_final_grade: Some(true),
            published: None,
            status: Some(ModuleStatus::Locked),
        };
        
        let updated_module = update_module(
            module.id.clone(),
            module_update,
            tauri::State::new(module_repo.clone())
        ).await.unwrap();
        
        assert_eq!(updated_module.id, module.id);
        assert_eq!(updated_module.title, "Updated Module");
        assert_eq!(updated_module.publish_final_grade, true);
        assert_eq!(updated_module.status, ModuleStatus::Locked);
        
        // Test create_module_item
        let item_create = ModuleItemCreate {
            module_id: module.id.clone(),
            title: "Test Item".to_string(),
            position: None,
            item_type: ModuleItemType::Page,
            content_id: None,
            content_type: None,
            url: None,
            page_url: Some("/pages/test".to_string()),
            published: None,
        };
        
        let item = create_module_item(
            item_create,
            tauri::State::new(module_repo.clone())
        ).await.unwrap();
        
        assert_eq!(item.module_id, module.id);
        assert_eq!(item.title, "Test Item");
        assert_eq!(item.position, 1);
        assert_eq!(item.item_type, ModuleItemType::Page);
        assert_eq!(item.page_url, Some("/pages/test".to_string()));
        
        // Test get_module_item
        let fetched_item = get_module_item(
            item.id.clone(),
            tauri::State::new(module_repo.clone())
        ).await.unwrap();
        
        assert_eq!(fetched_item.id, item.id);
        assert_eq!(fetched_item.title, item.title);
        
        // Test get_module_items
        let items = get_module_items(
            module.id.clone(),
            tauri::State::new(module_repo.clone())
        ).await.unwrap();
        
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].id, item.id);
        
        // Test update_module_item
        let item_update = ModuleItemUpdate {
            title: Some("Updated Item".to_string()),
            position: None,
            published: Some(false),
            url: None,
            page_url: Some("/pages/updated".to_string()),
        };
        
        let updated_item = update_module_item(
            item.id.clone(),
            item_update,
            tauri::State::new(module_repo.clone())
        ).await.unwrap();
        
        assert_eq!(updated_item.id, item.id);
        assert_eq!(updated_item.title, "Updated Item");
        assert_eq!(updated_item.published, false);
        assert_eq!(updated_item.page_url, Some("/pages/updated".to_string()));
        
        // Create a second module for reordering test
        let module2_create = ModuleCreate {
            course_id: course_id.clone(),
            title: "Second Module".to_string(),
            position: None,
            publish_final_grade: None,
            published: None,
        };
        
        let module2 = create_module(
            module2_create, 
            tauri::State::new(module_repo.clone())
        ).await.unwrap();
        
        // Test reorder_modules
        let reordered_modules = reorder_modules(
            course_id.clone(),
            vec![module2.id.clone(), module.id.clone()],
            tauri::State::new(module_repo.clone())
        ).await.unwrap();
        
        assert_eq!(reordered_modules.len(), 2);
        assert_eq!(reordered_modules[0].id, module2.id);
        assert_eq!(reordered_modules[0].position, 1);
        assert_eq!(reordered_modules[1].id, module.id);
        assert_eq!(reordered_modules[1].position, 2);
        
        // Create a second item for reordering test
        let item2_create = ModuleItemCreate {
            module_id: module.id.clone(),
            title: "Second Item".to_string(),
            position: None,
            item_type: ModuleItemType::ExternalUrl,
            content_id: None,
            content_type: None,
            url: Some("https://example.com".to_string()),
            page_url: None,
            published: None,
        };
        
        let item2 = create_module_item(
            item2_create,
            tauri::State::new(module_repo.clone())
        ).await.unwrap();
        
        // Test reorder_module_items
        let reordered_items = reorder_module_items(
            module.id.clone(),
            vec![item2.id.clone(), item.id.clone()],
            tauri::State::new(module_repo.clone())
        ).await.unwrap();
        
        assert_eq!(reordered_items.len(), 2);
        assert_eq!(reordered_items[0].id, item2.id);
        assert_eq!(reordered_items[0].position, 1);
        assert_eq!(reordered_items[1].id, item.id);
        assert_eq!(reordered_items[1].position, 2);
        
        // Test delete_module_item
        let item_deleted = delete_module_item(
            item.id.clone(),
            tauri::State::new(module_repo.clone())
        ).await.unwrap();
        
        assert!(item_deleted);
        
        // Verify item is deleted
        let remaining_items = get_module_items(
            module.id.clone(),
            tauri::State::new(module_repo.clone())
        ).await.unwrap();
        
        assert_eq!(remaining_items.len(), 1);
        assert_eq!(remaining_items[0].id, item2.id);
        
        // Test delete_module
        let module_deleted = delete_module(
            module.id.clone(),
            tauri::State::new(module_repo.clone())
        ).await.unwrap();
        
        assert!(module_deleted);
        
        // Verify module is deleted
        let remaining_modules = get_modules(
            course_id.clone(),
            tauri::State::new(module_repo.clone())
        ).await.unwrap();
        
        assert_eq!(remaining_modules.len(), 1);
        assert_eq!(remaining_modules[0].id, module2.id);
    }
}