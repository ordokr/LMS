use std::path::{Path, PathBuf};
use std::fs;
use tempfile::TempDir;

/// Create a temporary directory for testing
pub fn create_temp_dir() -> TempDir {
    tempfile::tempdir().expect("Failed to create temporary directory")
}

/// Create a test file with the given content
pub fn create_test_file(dir: &Path, filename: &str, content: &str) -> PathBuf {
    let file_path = dir.join(filename);
    fs::write(&file_path, content).expect("Failed to write test file");
    file_path
}

/// Create a test directory structure
pub fn create_test_directory_structure(base_dir: &Path) -> PathBuf {
    // Create src directory
    let src_dir = base_dir.join("src");
    fs::create_dir_all(&src_dir).expect("Failed to create src directory");

    // Create src-tauri directory
    let src_tauri_dir = base_dir.join("src-tauri");
    fs::create_dir_all(&src_tauri_dir.join("src")).expect("Failed to create src-tauri/src directory");

    // Create docs directory
    let docs_dir = base_dir.join("docs");
    fs::create_dir_all(&docs_dir).expect("Failed to create docs directory");

    // Create some test files
    create_test_file(&src_dir, "main.rs", "fn main() { println!(\"Hello, world!\"); }");
    create_test_file(&src_tauri_dir.join("src"), "main.rs", "fn main() { println!(\"Hello from Tauri!\"); }");
    create_test_file(&base_dir, "Cargo.toml", "[package]\nname = \"test-project\"\nversion = \"0.1.0\"\nedition = \"2021\"");

    base_dir.to_path_buf()
}

/// Create a mock analysis result for testing
pub fn create_mock_analysis_result() -> unified_analyzer::analyzers::unified_analyzer::AnalysisResult {
    use unified_analyzer::analyzers::unified_analyzer::{
        AnalysisResult, ProjectStatus, ModelMetrics, ApiEndpointMetrics,
        ComponentMetrics, CodeQualityMetrics, TestMetrics, IntegrationMetrics,
        ArchitectureInfo, SyncSystemInfo, BlockchainInfo, Recommendation
    };
    use std::collections::HashMap;

    use chrono::Utc;

    let mut code_quality_metrics = HashMap::new();
    code_quality_metrics.insert("documentation".to_string(), 3.8);
    code_quality_metrics.insert("maintainability".to_string(), 4.2);
    code_quality_metrics.insert("complexity".to_string(), 3.5);

    let recommendations = vec![
        Recommendation {
            area: "Models".to_string(),
            description: "Implement remaining Canvas models".to_string(),
            priority: 1,
            related_files: vec![],
        },
        Recommendation {
            area: "API".to_string(),
            description: "Add authentication to remaining endpoints".to_string(),
            priority: 2,
            related_files: vec![],
        },
    ];

    AnalysisResult {
        timestamp: Utc::now(),
        project_status: ProjectStatus {
            phase: "early-development".to_string(),
            completion_percentage: 13.5,
            last_active_area: "API Development".to_string(),
            estimated_completion_date: None,
        },
        models: ModelMetrics {
            implemented: 0,
            total: 50,
            implementation_percentage: 0.0,
        },
        api_endpoints: ApiEndpointMetrics {
            implemented: 1,
            total: 100,
            implementation_percentage: 1.0,
        },
        ui_components: ComponentMetrics {
            implemented: 0,
            total: 80,
            implementation_percentage: 0.0,
        },
        code_quality: CodeQualityMetrics {
            metrics: code_quality_metrics,
        },
        tests: TestMetrics {
            total: 3,
            passing: 3,
            coverage: 65.0,
        },
        integration: IntegrationMetrics {
            total_points: 30,
            implemented_points: 20,
            implementation_percentage: 66.7,
        },
        architecture: ArchitectureInfo {
            frameworks: vec!["Tauri".to_string(), "Leptos".to_string(), "Axum".to_string()],
            design_patterns: vec!["Repository".to_string(), "Service".to_string(), "Factory".to_string()],
        },
        sync_system: SyncSystemInfo {
            implementation_status: "in-progress".to_string(),
            offline_capability: true,
        },
        blockchain: BlockchainInfo {
            implementation_status: "planned".to_string(),
            features: vec!["Immutable Records".to_string(), "Smart Contracts".to_string()],
        },
        feature_areas: HashMap::new(),
        recommendations,
    }
}

/// Assert that a file exists and contains the expected content
pub fn assert_file_contains(path: &Path, expected_content: &str) {
    assert!(path.exists(), "File does not exist: {:?}", path);
    let content = fs::read_to_string(path).expect("Failed to read file");
    assert!(content.contains(expected_content), "File does not contain expected content: {:?}", path);
}
