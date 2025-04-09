#[test]
fn test_category_with_parent() {
    let parent_id = Uuid::new_v4();
    let mut category = Category::new("Child Category".to_string(), None);
    category.parent_id = Some(parent_id);
    
    assert_eq!(category.parent_id, Some(parent_id));
    assert_eq!(category.name, "Child Category");
    assert_eq!(category.sync_status, SyncStatus::LocalOnly);
}

#[test]
fn test_category_update_from_discourse() {
    let mut category = Category::new("Original Name".to_string(), None);
    category.discourse_category_id = Some(200);
    
    let discourse_update = json!({
        "id": 200,
        "name": "Updated Name",
        "slug": "updated-slug",
        "description": "Updated description",
        "color": "FF5733",
        "text_color": "000000",
        "position": 3,
        "updated_at": "2025-05-15T12:00:00Z"
    });
    
    // Assume we have a method to update from Discourse data
    category.update_from_discourse(&discourse_update).unwrap();
    
    assert_eq!(category.name, "Updated Name");
    assert_eq!(category.slug, "updated-slug");
    assert_eq!(category.description, Some("Updated description".to_string()));
    assert_eq!(category.color, "#FF5733");
    assert_eq!(category.text_color, Some("#000000".to_string()));
    assert_eq!(category.position, 3);
    assert_eq!(category.sync_status, SyncStatus::SyncedWithDiscourse);
    
    // Verify updated_at was updated correctly
    assert_eq!(category.updated_at.year(), 2025);
    assert_eq!(category.updated_at.month(), 5);
    assert_eq!(category.updated_at.day(), 15);
}

#[test]
fn test_category_subcategories() {
    let mut parent = Category::new("Parent Category".to_string(), None);
    let child1 = Category::new("Child 1".to_string(), None);
    let child2 = Category::new("Child 2".to_string(), None);
    
    parent.subcategories.push(child1);
    parent.subcategories.push(child2);
    
    assert_eq!(parent.subcategories.len(), 2);
    assert_eq!(parent.subcategories[0].name, "Child 1");
    assert_eq!(parent.subcategories[1].name, "Child 2");
}

#[test]
fn test_category_color_manipulation() {
    let mut category = Category::new("Test Category".to_string(), None);
    
    // Test setting a valid color
    category.color = "#123456".to_string();
    assert!(category.validate().is_ok());
    
    // Test setting color without hash prefix (auto-corrected)
    category.color = "ABCDEF".to_string();
    assert_eq!(category.color, "#ABCDEF");
    assert!(category.validate().is_ok());
    
    // Test setting invalid color (too short)
    category.color = "#ABC".to_string();
    assert!(category.validate().is_err());
    
    // Test setting invalid color (non-hex characters)
    category.color = "#XYZABC".to_string();
    assert!(category.validate().is_err());
}

#[test]
fn test_category_topic_count_tracking() {
    let mut category = Category::new("Test Category".to_string(), None);
    assert_eq!(category.topic_count, 0);
    
    // Simulate adding topics
    category.topic_count = 5;
    assert_eq!(category.topic_count, 5);
    
    // Simulate increasing topic count
    category.increment_topic_count();
    assert_eq!(category.topic_count, 6);
    
    // Simulate decreasing topic count
    category.decrement_topic_count();
    assert_eq!(category.topic_count, 5);
    
    // Ensure we don't go below 0
    category.topic_count = 0;
    category.decrement_topic_count();
    assert_eq!(category.topic_count, 0);
}
        assert_eq!(category.topic_count, 0);
        assert!(category.subcategories.is_empty());
        
        // Verify created_at and updated_at are set
        let now = Utc::now();
        let diff_created = (category.created_at - now).num_seconds().abs();
        assert!(diff_created < 5); // Within 5 seconds
        
        let diff_updated = (category.updated_at - now).num_seconds().abs();
        assert!(diff_updated < 5); // Within 5 seconds
    }

    #[test]
    fn test_category_slug_generation() {
        // Test with spaces
        let category = Category::new("Test Category Name".to_string(), None);
        assert_eq!(category.slug, "test-category-name");
        
        // Test with special characters
        let category = Category::new("Test & Category @ Name!".to_string(), None);
        assert_eq!(category.slug, "test-category-name");
        
        // Test with multiple spaces
        let category = Category::new("Test   Category   Name".to_string(), None);
        assert_eq!(category.slug, "test---category---name");
        
        // Test regenerating slug
        let mut category = Category::new("Old Name".to_string(), None);
        category.name = "New Name".to_string();
        category.regenerate_slug();
        assert_eq!(category.slug, "new-name");
    }

    #[test]
    fn test_category_validation() {
        // Valid category
        let category = Category::new(
            "Test Category".to_string(),
            Some("Description".to_string())
        );
        assert!(category.validate().is_ok());
        
        // Invalid: Empty name
        let mut invalid_category = Category::new(
            "".to_string(),
            Some("Description".to_string())
        );
        assert!(invalid_category.validate().is_err());
        
        // Invalid: Empty slug
        let mut invalid_category = Category::new(
            "Test".to_string(),
            Some("Description".to_string())
        );
        invalid_category.slug = "".to_string();
        assert!(invalid_category.validate().is_err());
        
        // Invalid: Invalid color format
        let mut invalid_category = Category::new(
            "Test".to_string(),
            Some("Description".to_string())
        );
        invalid_category.color = "invalid".to_string();
        assert!(invalid_category.validate().is_err());
        
        // Invalid: Invalid text color format
        let mut invalid_category = Category::new(
            "Test".to_string(),
            Some("Description".to_string())
        );
        invalid_category.text_color = Some("invalid".to_string());
        assert!(invalid_category.validate().is_err());
    }

    #[test]
    fn test_category_from_discourse_api() {
        // Create a map of discourse parent category IDs to our UUIDs
        let mut parent_map = HashMap::new();
        let parent_uuid = Uuid::new_v4();
        parent_map.insert(100, parent_uuid);
        
        let discourse_json = json!({
            "id": 200,
            "name": "Test Discourse Category",
            "slug": "test-discourse",
            "description": "This is a category from Discourse",
            "color": "3498DB",
            "text_color": "FFFFFF",
            "parent_category_id": 100,
            "position": 5,
            "created_at": "2025-04-01T10:30:00Z",
            "updated_at": "2025-04-05T15:45:30Z"
        });
        
        let category = Category::from_discourse_api(&discourse_json, &parent_map).unwrap();
        
        // Verify basic fields
        assert_eq!(category.name, "Test Discourse Category");
        assert_eq!(category.slug, "test-discourse");
        assert_eq!(category.description, Some("This is a category from Discourse".to_string()));
        assert_eq!(category.color, "#3498DB");
        assert_eq!(category.text_color, Some("#FFFFFF".to_string()));
        assert_eq!(category.parent_id, Some(parent_uuid));
        assert_eq!(category.position, 5);
        assert_eq!(category.discourse_category_id, Some(200));
        assert_eq!(category.sync_status, SyncStatus::SyncedWithDiscourse);
        
        // Verify dates were parsed correctly
        assert_eq!(category.created_at.year(), 2025);
        assert_eq!(category.created_at.month(), 4);
        assert_eq!(category.created_at.day(), 1);
        
        assert_eq!(category.updated_at.year(), 2025);
        assert_eq!(category.updated_at.month(), 4);
        assert_eq!(category.updated_at.day(), 5);
    }

    #[test]
    fn test_category_serialization() {
        let name = "Test Category".to_string();
        let description = Some("This is a test category".to_string());
        
        let category = Category::new(name, description);
        
        // Serialize to JSON
        let json = serde_json::to_string(&category).unwrap();
        
        // Deserialize from JSON
        let deserialized: Category = serde_json::from_str(&json).unwrap();
        
        // Verify fields
        assert_eq!(category.id, deserialized.id);
        assert_eq!(category.name, deserialized.name);
        assert_eq!(category.slug, deserialized.slug);
        assert_eq!(category.description, deserialized.description);
        
        // Verify dates are properly serialized/deserialized
        assert_eq!(
            category.created_at.timestamp(),
            deserialized.created_at.timestamp()
        );
        assert_eq!(
            category.updated_at.timestamp(),
            deserialized.updated_at.timestamp()
        );
    }
}