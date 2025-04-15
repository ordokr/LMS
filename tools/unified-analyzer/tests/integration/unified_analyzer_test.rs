use std::sync::Arc;
use tempfile::TempDir;

use unified_analyzer::analyzers::unified_analyzer::UnifiedProjectAnalyzer;
use unified_analyzer::utils::file_system::FileSystemUtils;
use crate::test_utils::{create_temp_dir, create_test_directory_structure, assert_file_contains};

#[tokio::test]
async fn test_unified_analyzer_end_to_end() {
    // Create a temporary directory for testing
    let temp_dir: TempDir = create_temp_dir();
    let base_dir = temp_dir.path();
    
    // Create a test directory structure
    create_test_directory_structure(base_dir);
    
    // Create the unified analyzer
    let fs_utils = Arc::new(FileSystemUtils::new());
    let analyzer = UnifiedProjectAnalyzer::new(base_dir.to_path_buf(), fs_utils);
    
    // Run the analysis
    let result = analyzer.analyze().await;
    assert!(result.is_ok(), "Failed to run analysis: {:?}", result.err());
    
    // Generate documentation
    let result = result.unwrap();
    
    // Generate enhanced central reference hub
    let generate_result = unified_analyzer::generators::enhanced_central_hub_generator::generate_enhanced_central_hub(&result, base_dir);
    assert!(generate_result.is_ok(), "Failed to generate enhanced central hub: {:?}", generate_result.err());
    
    // Check that the file was created
    let hub_path = base_dir.join("docs").join("central_reference_hub.md");
    assert!(hub_path.exists(), "Central reference hub file was not created");
    
    // Check that the file contains the expected content
    assert_file_contains(&hub_path, "# LMS Project: Central Reference Hub");
    
    // Generate analyzer reference
    let generate_result = analyzer.generate_analyzer_reference().await;
    assert!(generate_result.is_ok(), "Failed to generate analyzer reference: {:?}", generate_result.err());
    
    // Check that the file was created
    let analyzer_reference_path = base_dir.join("docs").join("analyzer_reference.md");
    assert!(analyzer_reference_path.exists(), "Analyzer reference file was not created");
    
    // Generate API documentation
    let generate_result = unified_analyzer::generators::api_doc_generator::generate_api_doc(&result, base_dir);
    assert!(generate_result.is_ok(), "Failed to generate API documentation: {:?}", generate_result.err());
    
    // Check that the file was created
    let api_doc_path = base_dir.join("docs").join("api").join("reference.md");
    assert!(api_doc_path.exists(), "API documentation file was not created");
    
    // Generate implementation details
    let generate_result = unified_analyzer::generators::implementation_details_generator::generate_implementation_details(&result, base_dir);
    assert!(generate_result.is_ok(), "Failed to generate implementation details: {:?}", generate_result.err());
    
    // Check that the file was created
    let implementation_details_path = base_dir.join("docs").join("implementation_details.md");
    assert!(implementation_details_path.exists(), "Implementation details file was not created");
    
    // Generate testing documentation
    let generate_result = unified_analyzer::generators::testing_doc_generator::generate_testing_doc(&result, base_dir);
    assert!(generate_result.is_ok(), "Failed to generate testing documentation: {:?}", generate_result.err());
    
    // Check that the file was created
    let testing_doc_path = base_dir.join("docs").join("tests.md");
    assert!(testing_doc_path.exists(), "Testing documentation file was not created");
    
    // Generate technical debt report
    let generate_result = unified_analyzer::generators::tech_debt_report_generator::generate_tech_debt_report(&result, base_dir);
    assert!(generate_result.is_ok(), "Failed to generate technical debt report: {:?}", generate_result.err());
    
    // Check that the file was created
    let tech_debt_report_path = base_dir.join("docs").join("technical_debt_report.md");
    assert!(tech_debt_report_path.exists(), "Technical debt report file was not created");
    
    // Generate summary report
    let generate_result = unified_analyzer::generators::summary_report_generator::generate_summary_report(&result, base_dir);
    assert!(generate_result.is_ok(), "Failed to generate summary report: {:?}", generate_result.err());
    
    // Check that the file was created
    let summary_report_path = base_dir.join("docs").join("SUMMARY_REPORT.md");
    assert!(summary_report_path.exists(), "Summary report file was not created");
    
    // Generate synchronization architecture
    let generate_result = unified_analyzer::generators::sync_architecture_generator::generate_sync_architecture(&result, base_dir);
    assert!(generate_result.is_ok(), "Failed to generate synchronization architecture: {:?}", generate_result.err());
    
    // Check that the file was created
    let sync_architecture_path = base_dir.join("docs").join("synchronization_architecture.md");
    assert!(sync_architecture_path.exists(), "Synchronization architecture file was not created");
    
    // Generate database architecture
    let generate_result = unified_analyzer::generators::database_architecture_generator::generate_database_architecture(&result, base_dir);
    assert!(generate_result.is_ok(), "Failed to generate database architecture: {:?}", generate_result.err());
    
    // Check that the file was created
    let database_architecture_path = base_dir.join("docs").join("database_architecture.md");
    assert!(database_architecture_path.exists(), "Database architecture file was not created");
}
