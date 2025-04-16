rust
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use crate::analyzers::modules::file_structure_analyzer::{FileStructureAnalyzer, DirectoryPurpose, FileStructureError};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_metadata_extraction() {
        // Create a temporary directory for testing
        let temp_dir = tempfile::tempdir().unwrap();
        let root_path = temp_dir.path().to_str().unwrap();

        // Create some mock files
        let file1_path = temp_dir.path().join("file1.txt");
        fs::write(&file1_path, "content1").unwrap();

        let file2_path = temp_dir.path().join("file2.txt");
        fs::write(&file2_path, "content2").unwrap();

        // Create a nested directory and a file inside it
        let nested_dir = temp_dir.path().join("nested");
        fs::create_dir(&nested_dir).unwrap();

        let file3_path = nested_dir.join("file3.txt");
        fs::write(&file3_path, "content3").unwrap();

        let analyzer = FileStructureAnalyzer::default();
        let result = analyzer.analyze(root_path);

        assert!(result.is_ok());

        // Clean up the temporary directory
        temp_dir.close().unwrap();
    }

    #[test]
    fn test_directory_categorization() {
        // Create a temporary directory for testing
        let temp_dir = tempfile::tempdir().unwrap();
        let root_path = temp_dir.path().to_str().unwrap();

        // Create mock directories
        fs::create_dir(temp_dir.path().join("models")).unwrap();
        fs::create_dir(temp_dir.path().join("app/controllers")).unwrap();
        fs::create_dir(temp_dir.path().join("db/migrate")).unwrap();
        fs::create_dir(temp_dir.path().join("unknown_dir")).unwrap();

        let analyzer = FileStructureAnalyzer::default();
        analyzer.analyze(root_path).unwrap();

        // Assert that directories are categorized correctly
        let expected_categories = HashMap::from([
            (temp_dir.path().join("models"), DirectoryPurpose::Model),
            (temp_dir.path().join("app/controllers"), DirectoryPurpose::Controller),
            (temp_dir.path().join("db/migrate"), DirectoryPurpose::Migration),
            (temp_dir.path().join("unknown_dir"), DirectoryPurpose::Unknown),
        ]);
        for (dir_path, expected_purpose) in expected_categories {
            let dir_meta = analyzer.directory_metadata.get(&dir_path).unwrap();
            assert_eq!(dir_meta.purpose, expected_purpose);
        }

        // Clean up the temporary directory
        temp_dir.close().unwrap();
    }

    #[test]
    fn test_file_dependency_graph() {
        // Create a temporary directory for testing
        let temp_dir = tempfile::tempdir().unwrap();
        let root_path = temp_dir.path().to_str().unwrap();

        // Create mock files with dependencies
        let file1_path = temp_dir.path().join("file1.rs");
        fs::write(&file1_path, "use crate::file2;").unwrap();

        let file2_path = temp_dir.path().join("file2.rs");
        fs::write(&file2_path, "fn file2_func() {}").unwrap();

        let file3_path = temp_dir.path().join("file3.js");
        fs::write(&file3_path, "const file2 = require('./file2.js');").unwrap();

        let file4_path = temp_dir.path().join("file4.js");
        fs::write(&file4_path, "const file2 = require('./file2');").unwrap();

        let file5_path = temp_dir.path().join("file5.rb");
        fs::write(&file5_path, "require_relative \"./file2.rb\"").unwrap();

        let file6_path = temp_dir.path().join("file6.rb");
        fs::write(&file6_path, "require_relative \"./file5\"").unwrap();

        let analyzer = FileStructureAnalyzer::default();
        analyzer.analyze(root_path).unwrap();

        // Assert that the dependency graph is built correctly
        let file1_relative = PathBuf::from("file1.rs");
        let file2_relative = PathBuf::from("file2.rs");
        let file3_relative = PathBuf::from("file3.js");
        let file4_relative = PathBuf::from("file4.js");
        let file5_relative = PathBuf::from("file5.rb");
        let file6_relative = PathBuf::from("file6.rb");

        assert!(analyzer.file_dependency_graph.contains_key(&file1_relative));
        assert!(analyzer.file_dependency_graph.contains_key(&file3_relative));
        assert!(analyzer.file_dependency_graph.contains_key(&file4_relative));
        assert!(analyzer.file_dependency_graph.contains_key(&file5_relative));
        assert!(analyzer.file_dependency_graph.contains_key(&file6_relative));

        assert_eq!(
            analyzer.file_dependency_graph.get(&file1_relative).unwrap(),
            &vec![file2_relative.clone()]
        );
        assert_eq!(
            analyzer.file_dependency_graph.get(&file3_relative).unwrap(),
            &vec![file2_relative.with_extension("js")]
        );
        assert_eq!(
            analyzer.file_dependency_graph.get(&file4_relative).unwrap(),
            &vec![file2_relative.with_extension("js")]
        );
        assert_eq!(
            analyzer.file_dependency_graph.get(&file5_relative).unwrap(),
            &vec![file2_relative.with_extension("rb")]
        );

        assert_eq!(
            analyzer.file_dependency_graph.get(&file6_relative).unwrap(),
            &vec![file5_relative.with_extension("rb")]
        );

        // Clean up the temporary directory
        temp_dir.close().unwrap();
    }
}