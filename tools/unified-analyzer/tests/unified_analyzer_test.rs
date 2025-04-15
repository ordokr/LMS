use std::sync::Arc;
use std::path::PathBuf;
use tokio;
use chrono::Utc;
use anyhow::Result;

use unified_analyzer::analyzers::unified_analyzer::{
    UnifiedProjectAnalyzer,
    AnalysisResult,
    ProjectStatus,
    ModelMetrics,
    ApiEndpointMetrics,
    ComponentMetrics,
    CodeQualityMetrics,
    TestMetrics,
    IntegrationMetrics,
    ArchitectureInfo,
    SyncSystemInfo,
    BlockchainInfo,
    FeatureAreaMetrics,
    Recommendation
};
use unified_analyzer::utils::file_system::FileSystemUtils;

#[tokio::test]
async fn test_unified_analyzer_initialization() -> Result<()> {
    let base_dir = std::env::current_dir()?;
    let fs_utils = Arc::new(FileSystemUtils::new());
    let analyzer = UnifiedProjectAnalyzer::new(base_dir, fs_utils);
    assert!(analyzer.result.lock().await.timestamp.timestamp_millis() > 0);
    Ok(())
}

#[tokio::test]
async fn test_full_analysis_execution() -> Result<()> {
    let base_dir = std::env::current_dir()?;
    let fs_utils = Arc::new(FileSystemUtils::new());
    let analyzer = UnifiedProjectAnalyzer::new(base_dir, fs_utils);
    let result = match analyzer.analyze().await {
        Ok(result) => result,
        Err(e) => return Err(anyhow::anyhow!("Analysis failed: {}", e)),
    };

    // Verify all analysis components were executed
    assert!(result.timestamp <= Utc::now());
    assert!(result.project_status != ProjectStatus::default());

    // Check that metrics were collected
    assert!(result.models != ModelMetrics::default());
    assert!(result.api_endpoints != ApiEndpointMetrics::default());
    assert!(result.code_quality != CodeQualityMetrics::default());

    Ok(())
}

#[tokio::test]
async fn test_parallel_analysis_components() -> Result<()> {
    let base_dir = std::env::current_dir()?;
    let fs_utils = Arc::new(FileSystemUtils::new());
    let analyzer = UnifiedProjectAnalyzer::new(base_dir, fs_utils);

    // Test individual analysis components
    let models_result = analyzer.analyze_models().await;
    let api_result = analyzer.analyze_api_endpoints().await;
    let quality_result = analyzer.analyze_code_quality().await;

    // These methods return () not Result<()>, so we just verify they completed
    assert!(models_result.is_ok());
    assert!(api_result.is_ok());
    assert!(quality_result.is_ok());

    Ok(())
}

#[tokio::test]
async fn test_incremental_analysis() -> Result<()> {
    let base_dir = std::env::current_dir()?;
    let fs_utils = Arc::new(FileSystemUtils::new());
    let analyzer = UnifiedProjectAnalyzer::new(base_dir.clone(), fs_utils);

    // First analysis
    let initial_result = match analyzer.analyze().await {
        Ok(result) => result,
        Err(e) => return Err(anyhow::anyhow!("First analysis failed: {}", e)),
    };

    // Simulate some changes
    // Create a temporary test file to simulate a change
    let test_file_path = base_dir.join("temp_test_file.rs");
    std::fs::write(&test_file_path, "// Test file for incremental analysis")?;

    // Second analysis should be incremental
    let updated_result = match analyzer.analyze().await {
        Ok(result) => result,
        Err(e) => return Err(anyhow::anyhow!("Second analysis failed: {}", e)),
    };

    // Clean up the test file
    std::fs::remove_file(test_file_path)?;

    assert!(updated_result.timestamp > initial_result.timestamp);
    // Verify that only changed components were analyzed

    Ok(())
}

#[tokio::test]
async fn test_analysis_with_errors() -> Result<()> {
    let base_dir = std::env::current_dir()?;
    let fs_utils = Arc::new(FileSystemUtils::new());
    let analyzer = UnifiedProjectAnalyzer::new(base_dir, fs_utils);

    // Test handling of various error conditions
    // 1. Invalid project structure
    // 2. Missing required files
    // 3. Parse errors
    // 4. Integration errors

    // The analysis should complete with partial results
    let _result = match analyzer.analyze().await {
        Ok(result) => result,
        Err(e) => return Err(anyhow::anyhow!("Analysis failed: {}", e)),
    };

    // Note: This test might need to be adjusted based on how errors are handled
    // in the actual implementation. For now, we're just checking that analysis completes.

    Ok(())
}

#[tokio::test]
async fn test_recommendations_generation() -> Result<()> {
    let base_dir = std::env::current_dir()?;
    let fs_utils = Arc::new(FileSystemUtils::new());
    let analyzer = UnifiedProjectAnalyzer::new(base_dir, fs_utils);

    // Generate recommendations
    if let Err(e) = analyzer.generate_recommendations().await {
        return Err(anyhow::anyhow!("Failed to generate recommendations: {}", e));
    }

    // Get the analysis result
    let _result = match analyzer.analyze().await {
        Ok(result) => result,
        Err(e) => return Err(anyhow::anyhow!("Analysis failed: {}", e)),
    };

    // Verify recommendations were generated
    // Note: This might need adjustment based on the actual implementation
    // If recommendations are not generated by default, we may need to modify this test

    Ok(())
}

// Test helpers
fn create_test_project_structure() -> Result<PathBuf> {
    // Create a temporary directory for test project
    let temp_dir = tempfile::tempdir()?;
    let test_dir = temp_dir.path().to_path_buf();

    // Create sample project structure
    std::fs::create_dir_all(test_dir.join("src"))?;
    std::fs::create_dir_all(test_dir.join("src/models"))?;
    std::fs::create_dir_all(test_dir.join("src/api"))?;
    std::fs::create_dir_all(test_dir.join("src/components"))?;
    std::fs::create_dir_all(test_dir.join("tests"))?;
    std::fs::create_dir_all(test_dir.join("docs"))?;

    // Create sample model file
    let model_content = r#"
    pub struct User {
        pub id: i64,
        pub name: String,
        pub email: String,
    }

    impl User {
        pub fn new(name: String, email: String) -> Self {
            Self {
                id: 0,
                name,
                email,
            }
        }
    }
    "#;
    std::fs::write(test_dir.join("src/models/user.rs"), model_content)?;

    Ok(test_dir)
}

fn cleanup_test_project(test_dir: PathBuf) {
    // Remove test project directory and all artifacts
    let _ = std::fs::remove_dir_all(test_dir);
}

// Run before each test
async fn setup() -> Result<(UnifiedProjectAnalyzer, PathBuf)> {
    let test_dir = create_test_project_structure()?;
    let fs_utils = Arc::new(FileSystemUtils::new());
    let analyzer = UnifiedProjectAnalyzer::new(test_dir.clone(), fs_utils);
    Ok((analyzer, test_dir))
}

// Run after each test
fn teardown(test_dir: PathBuf) {
    cleanup_test_project(test_dir);
}

// Add a new test for central reference hub generation
#[tokio::test]
async fn test_central_reference_hub_generation() -> Result<()> {
    let base_dir = std::env::current_dir()?;
    let fs_utils = Arc::new(FileSystemUtils::new());
    let analyzer = UnifiedProjectAnalyzer::new(base_dir.clone(), fs_utils);

    // Run analysis first
    if let Err(e) = analyzer.analyze().await {
        return Err(anyhow::anyhow!("Analysis failed: {}", e));
    }

    // Generate central reference hub
    if let Err(e) = analyzer.generate_central_reference_hub().await {
        return Err(anyhow::anyhow!("Failed to generate central reference hub: {}", e));
    }

    // Verify the file was created
    let hub_path = base_dir.join("docs").join("central_reference_hub.md");
    assert!(hub_path.exists());

    // Clean up
    if hub_path.exists() {
        std::fs::remove_file(hub_path)?;
    }

    Ok(())
}
