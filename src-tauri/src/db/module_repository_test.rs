#[cfg(test)]
mod tests {
    use crate::db::module_repository::{ModuleRepository, SqliteModuleRepository};
    use crate::models::module::{Module, ModuleUpdate, ModuleStatus, ModuleItem, ModuleItemType, ModuleItemUpdate};
    use crate::db::test_utils::{create_test_db_pool, clean_test_db};
    use sqlx::Pool;
    use sqlx::sqlite::Sqlite;
    use uuid::Uuid;
    
    async fn setup_test_db() -> Pool<Sqlite> {
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
        
        pool
    }
    
    fn create_test_module() -> Module {
        Module {
            id: Uuid::new_v4().to_string(),
            course_id: Uuid::new_v4().to_string(),
            title: "Test Module".to_string(),
            position: 1,
            items_count: 0,
            publish_final_grade: false,
            published: true,
            status: ModuleStatus::Active,
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        }
    }
    
    fn create_test_module_item(module_id: &str) -> ModuleItem {
        ModuleItem {
            id: Uuid::new_v4().to_string(),
            module_id: module_id.to_string(),
            title: "Test Item".to_string(),
            position: 1,
            item_type: ModuleItemType::Page,
            content_id: Some(Uuid::new_v4().to_string()),
            content_type: Some("page".to_string()),
            url: None,
            page_url: Some("/pages/test".to_string()),
            published: true,
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        }
    }
    
    #[tokio::test]
    async fn test_crud_module() {
        let pool = setup_test_db().await;
        let repo = SqliteModuleRepository::new(pool);
        
        // Create
        let module = create_test_module();
        let module_id = module.id.clone();
        let course_id = module.course_id.clone();
        
        let created = repo.create_module(module.clone()).await.unwrap();
        assert_eq!(created.id, module_id);
        assert_eq!(created.title, module.title);
        
        // Read
        let fetched = repo.get_module(&module_id).await.unwrap().unwrap();
        assert_eq!(fetched.id, module_id);
        assert_eq!(fetched.title, module.title);
        
        // Read by course
        let modules = repo.get_modules_for_course(&course_id).await.unwrap();
        assert_eq!(modules.len(), 1);
        assert_eq!(modules[0].id, module_id);
        
        // Count
        let count = repo.get_modules_count(&course_id).await.unwrap();
        assert_eq!(count, 1);
        
        // Update
        let update = ModuleUpdate {
            title: Some("Updated Module".to_string()),
            position: None,
            publish_final_grade: Some(true),
            published: None,
            status: Some(ModuleStatus::Locked),
        };
        
        let updated = repo.update_module(&module_id, update).await.unwrap();
        assert_eq!(updated.id, module_id);
        assert_eq!(updated.title, "Updated Module");
        assert_eq!(updated.publish_final_grade, true);
        assert_eq!(updated.status, ModuleStatus::Locked);
        
        // Delete
        let deleted = repo.delete_module(&module_id).await.unwrap();
        assert!(deleted);
        
        // Verify deleted
        let not_found = repo.get_module(&module_id).await.unwrap();
        assert!(not_found.is_none());
    }
    
    #[tokio::test]
    async fn test_reorder_modules() {
        let pool = setup_test_db().await;
        let repo = SqliteModuleRepository::new(pool);
        
        // Create modules
        let course_id = Uuid::new_v4().to_string();
        
        let mut module1 = create_test_module();
        module1.course_id = course_id.clone();
        module1.title = "Module 1".to_string();
        module1.position = 1;
        let module1_id = module1.id.clone();
        repo.create_module(module1).await.unwrap();
        
        let mut module2 = create_test_module();
        module2.course_id = course_id.clone();
        module2.title = "Module 2".to_string();
        module2.position = 2;
        let module2_id = module2.id.clone();
        repo.create_module(module2).await.unwrap();
        
        let mut module3 = create_test_module();
        module3.course_id = course_id.clone();
        module3.title = "Module 3".to_string();
        module3.position = 3;
        let module3_id = module3.id.clone();
        repo.create_module(module3).await.unwrap();
        
        // Reorder modules (3, 1, 2)
        let modules = repo.reorder_modules(&course_id, &[module3_id.clone(), module1_id.clone(), module2_id.clone()]).await.unwrap();
        
        assert_eq!(modules.len(), 3);
        assert_eq!(modules[0].id, module3_id);
        assert_eq!(modules[0].position, 1);
        assert_eq!(modules[1].id, module1_id);
        assert_eq!(modules[1].position, 2);
        assert_eq!(modules[2].id, module2_id);
        assert_eq!(modules[2].position, 3);
    }
    
    #[tokio::test]
    async fn test_crud_module_item() {
        let pool = setup_test_db().await;
        let repo = SqliteModuleRepository::new(pool);
        
        // Create a module first
        let module = create_test_module();
        let module_id = module.id.clone();
        repo.create_module(module).await.unwrap();
        
        // Create item
        let item = create_test_module_item(&module_id);
        let item_id = item.id.clone();
        
        let created = repo.create_module_item(item.clone()).await.unwrap();
        assert_eq!(created.id, item_id);
        assert_eq!(created.title, item.title);
        
        // Check module items_count updated
        let module = repo.get_module(&module_id).await.unwrap().unwrap();
        assert_eq!(module.items_count, 1);
        
        // Read
        let fetched = repo.get_module_item(&item_id).await.unwrap().unwrap();
        assert_eq!(fetched.id, item_id);
        assert_eq!(fetched.title, item.title);
        
        // Read by module
        let items = repo.get_module_items(&module_id).await.unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].id, item_id);
        
        // Count
        let count = repo.get_module_items_count(&module_id).await.unwrap();
        assert_eq!(count, 1);
        
        // Update
        let update = ModuleItemUpdate {
            title: Some("Updated Item".to_string()),
            position: Some(2),
            published: Some(false),
            url: Some("https://example.com".to_string()),
            page_url: None,
        };
        
        let updated = repo.update_module_item(&item_id, update).await.unwrap();
        assert_eq!(updated.id, item_id);
        assert_eq!(updated.title, "Updated Item");
        assert_eq!(updated.position, 2);
        assert_eq!(updated.published, false);
        assert_eq!(updated.url, Some("https://example.com".to_string()));
        assert!(updated.page_url.is_none());
        
        // Delete
        let deleted = repo.delete_module_item(&item_id).await.unwrap();
        assert!(deleted);
        
        // Check module items_count updated after delete
        let module = repo.get_module(&module_id).await.unwrap().unwrap();
        assert_eq!(module.items_count, 0);
        
        // Verify deleted
        let not_found = repo.get_module_item(&item_id).await.unwrap();
        assert!(not_found.is_none());
    }
    
    #[tokio::test]
    async fn test_reorder_module_items() {
        let pool = setup_test_db().await;
        let repo = SqliteModuleRepository::new(pool);
        
        // Create a module first
        let module = create_test_module();
        let module_id = module.id.clone();
        repo.create_module(module).await.unwrap();
        
        // Create items
        let mut item1 = create_test_module_item(&module_id);
        item1.title = "Item 1".to_string();
        item1.position = 1;
        let item1_id = item1.id.clone();
        repo.create_module_item(item1).await.unwrap();
        
        let mut item2 = create_test_module_item(&module_id);
        item2.title = "Item 2".to_string();
        item2.position = 2;
        let item2_id = item2.id.clone();
        repo.create_module_item(item2).await.unwrap();
        
        let mut item3 = create_test_module_item(&module_id);
        item3.title = "Item 3".to_string();
        item3.position = 3;
        let item3_id = item3.id.clone();
        repo.create_module_item(item3).await.unwrap();
        
        // Reorder items (3, 1, 2)
        let items = repo.reorder_module_items(&module_id, &[item3_id.clone(), item1_id.clone(), item2_id.clone()]).await.unwrap();
        
        assert_eq!(items.len(), 3);
        assert_eq!(items[0].id, item3_id);
        assert_eq!(items[0].position, 1);
        assert_eq!(items[1].id, item1_id);
        assert_eq!(items[1].position, 2);
        assert_eq!(items[2].id, item2_id);
        assert_eq!(items[2].position, 3);
    }
}