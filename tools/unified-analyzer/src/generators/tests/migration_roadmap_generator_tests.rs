#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use std::fs;
    
    use crate::generators::MigrationRoadmapGenerator;
    use crate::output_schema::{UnifiedAnalysisOutput, ApiInfo, ApiEndpointInfo, DatabaseInfo, DatabaseTableInfo, ColumnInfo};

    #[test]
    fn test_migration_roadmap_generator_initialization() {
        let generator = MigrationRoadmapGenerator::new();
        assert!(generator.generate(&create_test_output(), &PathBuf::from("./test_output")).is_err());
    }

    #[test]
    fn test_generate_markdown() {
        let generator = MigrationRoadmapGenerator::new();
        let output = create_test_output();
        
        let result = generator.generate_markdown(&output);
        assert!(result.is_ok());
        
        let markdown = result.unwrap();
        assert!(markdown.contains("# Migration Roadmap"));
        assert!(markdown.contains("## Overview"));
        assert!(markdown.contains("## Phase 1"));
        assert!(markdown.contains("## Phase 2"));
        assert!(markdown.contains("## Phase 3"));
    }

    #[test]
    fn test_generate_html() {
        let generator = MigrationRoadmapGenerator::new();
        let output = create_test_output();
        
        let result = generator.generate_html(&output);
        assert!(result.is_ok());
        
        let html = result.unwrap();
        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("<title>Migration Roadmap</title>"));
        assert!(html.contains("Phase 1"));
        assert!(html.contains("Phase 2"));
        assert!(html.contains("Phase 3"));
    }

    #[test]
    fn test_assign_components_to_phases() {
        let generator = MigrationRoadmapGenerator::new();
        let output = create_test_output();
        
        let phases = generator.generate_phases(&output);
        
        // Check that phases are created
        assert_eq!(phases.len(), 3);
        
        // Check phase names
        assert_eq!(phases[0].name, "Phase 1: Core Functionality");
        assert_eq!(phases[1].name, "Phase 2: Extended Features");
        assert_eq!(phases[2].name, "Phase 3: Advanced Features");
        
        // Check that components are assigned to phases
        assert!(!phases[0].components.is_empty());
        
        // Check that APIs are assigned to phases
        assert!(!phases[0].apis.is_empty());
        
        // Check that database tables are assigned to phases
        assert!(!phases[0].database_tables.is_empty());
    }

    // Helper function to create a test output
    fn create_test_output() -> UnifiedAnalysisOutput {
        let mut output = UnifiedAnalysisOutput::default();
        
        // Add some API endpoints
        let mut endpoints = Vec::new();
        
        endpoints.push(ApiEndpointInfo {
            path: "/api/v1/users".to_string(),
            http_method: "GET".to_string(),
            controller: Some("UsersController".to_string()),
            action: Some("index".to_string()),
            description: Some("Get all users".to_string()),
            request_params: vec![],
            response_format: Some("JSON".to_string()),
            auth_required: true,
            rate_limited: false,
            category: Some("Users".to_string()),
        });
        
        endpoints.push(ApiEndpointInfo {
            path: "/api/v1/users/{id}".to_string(),
            http_method: "GET".to_string(),
            controller: Some("UsersController".to_string()),
            action: Some("show".to_string()),
            description: Some("Get a specific user".to_string()),
            request_params: vec!["id".to_string()],
            response_format: Some("JSON".to_string()),
            auth_required: true,
            rate_limited: false,
            category: Some("Users".to_string()),
        });
        
        output.api = ApiInfo {
            endpoints,
            base_url: Some("https://api.example.com".to_string()),
            version: Some("1.0".to_string()),
        };
        
        // Create database info
        let mut database = DatabaseInfo::default();
        
        // Create tables
        let mut tables = Vec::new();
        
        // Users table
        let mut users_table = DatabaseTableInfo::default();
        users_table.name = "users".to_string();
        users_table.columns = vec![
            ColumnInfo {
                name: "id".to_string(),
                data_type: "integer".to_string(),
                nullable: false,
                primary_key: true,
                foreign_key: false,
                references: None,
                default_value: None,
                description: None,
            },
            ColumnInfo {
                name: "name".to_string(),
                data_type: "string".to_string(),
                nullable: false,
                primary_key: false,
                foreign_key: false,
                references: None,
                default_value: None,
                description: None,
            },
        ];
        
        tables.push(users_table);
        
        database.tables = tables;
        database.db_type = Some("PostgreSQL".to_string());
        database.version = Some("14.0".to_string());
        
        output.database = database;
        
        output
    }
}
