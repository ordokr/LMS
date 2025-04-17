#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use std::fs;
    
    use crate::generators::DbSchemaGenerator;
    use crate::output_schema::{UnifiedAnalysisOutput, DatabaseInfo, DatabaseTableInfo, ColumnInfo};

    #[test]
    fn test_db_schema_generator_initialization() {
        let generator = DbSchemaGenerator::new();
        assert!(generator.generate(&create_test_output(), &PathBuf::from("./test_output")).is_err());
    }

    #[test]
    fn test_extract_relationships() {
        let generator = DbSchemaGenerator::new();
        
        // Create test tables
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
        
        // Posts table with foreign key to users
        let mut posts_table = DatabaseTableInfo::default();
        posts_table.name = "posts".to_string();
        posts_table.columns = vec![
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
                name: "title".to_string(),
                data_type: "string".to_string(),
                nullable: false,
                primary_key: false,
                foreign_key: false,
                references: None,
                default_value: None,
                description: None,
            },
            ColumnInfo {
                name: "user_id".to_string(),
                data_type: "integer".to_string(),
                nullable: false,
                primary_key: false,
                foreign_key: true,
                references: Some("users.id".to_string()),
                default_value: None,
                description: None,
            },
        ];
        
        tables.push(users_table);
        tables.push(posts_table);
        
        // Test relationship extraction
        let relationships = generator.extract_relationships(&tables);
        
        assert_eq!(relationships.len(), 1);
        assert_eq!(relationships[0].parent_table, "user");
        assert_eq!(relationships[0].child_table, "posts");
        assert_eq!(relationships[0].relationship_type, "has many");
        assert_eq!(relationships[0].foreign_key, "user_id");
    }

    #[test]
    fn test_generate_markdown() {
        let generator = DbSchemaGenerator::new();
        let output = create_test_output();
        
        let result = generator.generate_markdown(&output);
        assert!(result.is_ok());
        
        let markdown = result.unwrap();
        assert!(markdown.contains("# Database Schema Visualization"));
        assert!(markdown.contains("## Entity-Relationship Diagram"));
        assert!(markdown.contains("## Table Details"));
        assert!(markdown.contains("## Relationships"));
    }

    #[test]
    fn test_template_loading() {
        // Create a temporary template file
        let temp_dir = tempfile::tempdir().unwrap();
        let template_path = temp_dir.path().join("db_schema_template.html");
        
        fs::write(&template_path, r#"<!DOCTYPE html>
<html>
<head>
    <title>Database Schema</title>
</head>
<body>
    <!-- TABLE_LIST_PLACEHOLDER -->
    <!-- RELATIONSHIPS_LIST_PLACEHOLDER -->
    <!-- SCHEMA_DATA_PLACEHOLDER -->
</body>
</html>"#).unwrap();
        
        // Test loading the template
        let template = fs::read_to_string(&template_path).unwrap();
        assert!(template.contains("<!-- TABLE_LIST_PLACEHOLDER -->"));
        assert!(template.contains("<!-- RELATIONSHIPS_LIST_PLACEHOLDER -->"));
        assert!(template.contains("<!-- SCHEMA_DATA_PLACEHOLDER -->"));
    }

    // Helper function to create a test output
    fn create_test_output() -> UnifiedAnalysisOutput {
        let mut output = UnifiedAnalysisOutput::default();
        
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
            ColumnInfo {
                name: "email".to_string(),
                data_type: "string".to_string(),
                nullable: false,
                primary_key: false,
                foreign_key: false,
                references: None,
                default_value: None,
                description: None,
            },
        ];
        
        // Posts table
        let mut posts_table = DatabaseTableInfo::default();
        posts_table.name = "posts".to_string();
        posts_table.columns = vec![
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
                name: "title".to_string(),
                data_type: "string".to_string(),
                nullable: false,
                primary_key: false,
                foreign_key: false,
                references: None,
                default_value: None,
                description: None,
            },
            ColumnInfo {
                name: "content".to_string(),
                data_type: "text".to_string(),
                nullable: false,
                primary_key: false,
                foreign_key: false,
                references: None,
                default_value: None,
                description: None,
            },
            ColumnInfo {
                name: "user_id".to_string(),
                data_type: "integer".to_string(),
                nullable: false,
                primary_key: false,
                foreign_key: true,
                references: Some("users.id".to_string()),
                default_value: None,
                description: None,
            },
        ];
        
        tables.push(users_table);
        tables.push(posts_table);
        
        database.tables = tables;
        database.db_type = Some("PostgreSQL".to_string());
        database.version = Some("14.0".to_string());
        
        output.database = database;
        
        output
    }
}
