#[cfg(test)]
mod tests {
    // use std::path::Path; // Removed unused import
    use tempfile::tempdir;
    use std::fs::{self, File};
    use std::io::Write;
    use crate::analyzers::project_structure::ProjectStructure;

    fn create_test_project() -> tempfile::TempDir {
        let dir = tempdir().unwrap();

        // Create some directories
        fs::create_dir_all(dir.path().join("src/models")).unwrap();
        fs::create_dir_all(dir.path().join("src/controllers")).unwrap();
        fs::create_dir_all(dir.path().join("src/views")).unwrap();
        fs::create_dir_all(dir.path().join("tests")).unwrap();

        // Create some files
        let mut file = File::create(dir.path().join("src/models/user.rs")).unwrap();
        writeln!(file, "struct User {{ name: String, age: u32 }}").unwrap();

        let mut file = File::create(dir.path().join("src/controllers/user_controller.rs")).unwrap();
        writeln!(file, "fn get_user() -> User {{ User::new() }}").unwrap();

        let mut file = File::create(dir.path().join("src/views/user_view.html")).unwrap();
        writeln!(file, "<div>User View</div>").unwrap();

        let mut file = File::create(dir.path().join("tests/user_test.rs")).unwrap();
        writeln!(file, "#[test] fn test_user() {{ assert!(true); }}").unwrap();

        dir
    }

    #[test]
    fn test_project_structure_analysis() {
        let temp_dir = create_test_project();
        let analyzer = ProjectStructure::new(temp_dir.path());

        // Test file count
        assert_eq!(analyzer.get_file_count(), 4);

        // Test directory count
        assert_eq!(analyzer.get_directory_count(), 6); // including root dir and parent temp dir

        // Test files by extension
        let rs_files = analyzer.get_files_by_extension("rs");
        assert_eq!(rs_files.len(), 3);

        let html_files = analyzer.get_files_by_extension("html");
        assert_eq!(html_files.len(), 1);
    }

    #[test]
    fn test_get_directory_structure() {
        let temp_dir = create_test_project();
        let analyzer = ProjectStructure::new(temp_dir.path());

        let structure = analyzer.get_directory_structure();

        // Check that the structure contains our directories
        // The structure contains the full paths, so we need to check if any path contains "/src"
        assert!(structure.iter().any(|path| path.contains("src")));
        // Handle both Unix and Windows path separators
        assert!(structure.iter().any(|path| path.contains("src\\models") || path.contains("src/models")));
        assert!(structure.iter().any(|path| path.contains("src\\controllers") || path.contains("src/controllers")));
        assert!(structure.iter().any(|path| path.contains("src\\views") || path.contains("src/views")));
        assert!(structure.iter().any(|path| path.contains("tests")));
    }

    #[test]
    fn test_get_file_paths() {
        let temp_dir = create_test_project();
        let analyzer = ProjectStructure::new(temp_dir.path());

        let file_paths = analyzer.get_file_paths();

        // Check that the file paths contain our files
        // Handle both Unix and Windows path separators
        assert!(file_paths.iter().any(|path| {
            let path_str = path.to_string_lossy();
            path_str.contains("src\\models\\user.rs") || path_str.contains("src/models/user.rs")
        }));
        assert!(file_paths.iter().any(|path| {
            let path_str = path.to_string_lossy();
            path_str.contains("src\\controllers\\user_controller.rs") || path_str.contains("src/controllers/user_controller.rs")
        }));
        assert!(file_paths.iter().any(|path| {
            let path_str = path.to_string_lossy();
            path_str.contains("src\\views\\user_view.html") || path_str.contains("src/views/user_view.html")
        }));
        assert!(file_paths.iter().any(|path| {
            let path_str = path.to_string_lossy();
            path_str.contains("tests\\user_test.rs") || path_str.contains("tests/user_test.rs")
        }));
    }
}
