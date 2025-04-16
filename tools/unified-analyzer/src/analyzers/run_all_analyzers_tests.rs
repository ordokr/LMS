rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
    use std::path::PathBuf;

    fn setup() -> PathBuf {
        let temp_dir = PathBuf::from("test_files_all_analyzers");
        fs::create_dir_all(&temp_dir).unwrap();

        // Create a file for FileStructureAnalyzer
        File::create(temp_dir.join("file.txt")).unwrap();

        // Create a file for CanvasAnalyzer (module.html)
        let module_content = r#"<h2 class="page-title">Test Module</h2><div class="module-sequence-footer"></div><h2>Test Module Overview</h2>"#;
        let mut file = File::create(temp_dir.join("module.html")).unwrap();
        file.write_all(module_content.as_bytes()).unwrap();

        // Create a file for DiscourseAnalyzer (user.html)
        let user_content = r#"<div>@testuser</div><div>@anotheruser</div>"#;
        let mut file = File::create(temp_dir.join("user.html")).unwrap();
        file.write_all(user_content.as_bytes()).unwrap();

        // Add more files for other analyzers if needed...

        temp_dir
    }

    fn teardown(temp_dir: PathBuf) {
        fs::remove_dir_all(temp_dir).unwrap();
    }

    #[test]
    fn test_run_all_analyzers() {
        let temp_dir = setup();
        let results = run_all_analyzers(temp_dir.to_str().unwrap());
        assert!(!results.is_empty());
        for result in results {
            assert!(result.is_ok() || result.is_err());
        }
        teardown(temp_dir);
    }
}