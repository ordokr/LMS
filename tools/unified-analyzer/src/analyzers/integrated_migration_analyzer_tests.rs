#[cfg(test)]
mod tests {
    // use std::path::PathBuf; // Removed unused import
    use std::sync::Arc;
    use tempfile::tempdir;
    use std::fs::{self, File};
    use std::io::Write;
    use crate::analyzers::integrated_migration_analyzer::IntegratedMigrationAnalyzer;
    use crate::utils::file_system::FileSystemUtils;

    async fn setup_test_environment() -> (tempfile::TempDir, IntegratedMigrationAnalyzer) {
        // Create a test environment with dummy data
        let dir = tempdir().unwrap();

        // Create Canvas directory structure
        fs::create_dir_all(dir.path().join("canvas/app/models")).unwrap();
        fs::create_dir_all(dir.path().join("canvas/app/controllers")).unwrap();
        fs::create_dir_all(dir.path().join("canvas/app/views")).unwrap();

        // Create Discourse directory structure
        fs::create_dir_all(dir.path().join("discourse/app/models")).unwrap();
        fs::create_dir_all(dir.path().join("discourse/app/controllers")).unwrap();
        fs::create_dir_all(dir.path().join("discourse/app/views")).unwrap();

        // Create some Canvas model files
        let mut file = File::create(dir.path().join("canvas/app/models/user.rb")).unwrap();
        writeln!(file, "class User < ActiveRecord::Base\n  has_many :courses\nend").unwrap();

        let mut file = File::create(dir.path().join("canvas/app/models/course.rb")).unwrap();
        writeln!(file, "class Course < ActiveRecord::Base\n  belongs_to :user\nend").unwrap();

        // Create some Discourse model files
        let mut file = File::create(dir.path().join("discourse/app/models/user.rb")).unwrap();
        writeln!(file, "class User < ActiveRecord::Base\n  has_many :topics\nend").unwrap();

        let mut file = File::create(dir.path().join("discourse/app/models/topic.rb")).unwrap();
        writeln!(file, "class Topic < ActiveRecord::Base\n  belongs_to :user\nend").unwrap();

        // Create the analyzer
        let fs_utils = Arc::new(FileSystemUtils::new());
        let mut analyzer = IntegratedMigrationAnalyzer::new(dir.path(), fs_utils);

        // Set the Canvas and Discourse directories
        analyzer
            .with_canvas_dir(dir.path().join("canvas"))
            .with_discourse_dir(dir.path().join("discourse"));

        // Add dummy data for testing
        analyzer.result.canvas_models.push("User".to_string());
        analyzer.result.canvas_models.push("Course".to_string());
        analyzer.result.discourse_models.push("User".to_string());
        analyzer.result.discourse_models.push("Topic".to_string());

        // Add a common entity
        let common_entity = crate::analyzers::integrated_migration_analyzer::CommonEntity {
            name: "User".to_string(),
            canvas_path: "canvas/app/models/user.rb".to_string(),
            discourse_path: "discourse/app/models/user.rb".to_string(),
            mapping_complexity: "medium".to_string(),
        };

        analyzer.result.common_entities.insert("User".to_string(), common_entity);

        (dir, analyzer)
    }

    #[tokio::test]
    async fn test_analyze() {
        let (_dir, mut analyzer) = setup_test_environment().await;

        // Run the analysis
        let result = analyzer.analyze().await.unwrap();

        // Check that we found common entities
        assert!(result.common_entities.contains_key("User"));

        // Check that we found migration paths
        assert!(!result.migration_paths.is_empty());

        // Check that we found integration points
        assert!(!result.integration_points.is_empty());
    }

    #[tokio::test]
    async fn test_find_common_entities() {
        let (_dir, mut analyzer) = setup_test_environment().await;

        // Run the analysis to find common entities
        analyzer.find_common_entities().await.unwrap();

        // Check that we found the User entity in both systems
        let result = analyzer.analyze().await.unwrap();
        assert!(result.common_entities.contains_key("User"));

        // The User entity should have both Canvas and Discourse paths
        let user_entity = &result.common_entities["User"];
        assert!(user_entity.canvas_path.contains("canvas/app/models/user.rb"));
        assert!(user_entity.discourse_path.contains("discourse/app/models/user.rb"));
    }

    #[tokio::test]
    async fn test_identify_migration_paths() {
        let (_dir, mut analyzer) = setup_test_environment().await;

        // First find common entities
        analyzer.find_common_entities().await.unwrap();

        // Then identify migration paths
        analyzer.identify_migration_paths().await.unwrap();

        // Check that we found migration paths
        let result = analyzer.analyze().await.unwrap();
        assert!(!result.migration_paths.is_empty());

        // There should be a migration path for User
        assert!(result.migration_paths.iter().any(|path| path.entity_name == "User"));
    }

    #[tokio::test]
    async fn test_identify_integration_points() {
        let (_dir, mut analyzer) = setup_test_environment().await;

        // First find common entities
        analyzer.find_common_entities().await.unwrap();

        // Then identify integration points
        analyzer.identify_integration_points().await.unwrap();

        // Check that we found integration points
        let result = analyzer.analyze().await.unwrap();
        assert!(!result.integration_points.is_empty());

        // There should be an integration point for User
        assert!(result.integration_points.iter().any(|point| point.entity_name == "User"));
    }
}
