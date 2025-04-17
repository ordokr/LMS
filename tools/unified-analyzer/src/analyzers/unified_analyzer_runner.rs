use std::path::Path;
use std::sync::Arc;
use crate::analyzers::unified_analyzer::UnifiedProjectAnalyzer;
use crate::utils::file_system::FileSystemUtils;

/// Run the unified project analyzer on a project
#[allow(dead_code)]
pub async fn run_unified_project_analyzer(project_path: &Path) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("Running unified project analyzer on {:?}", project_path);

    // Create a new file system utils
    let fs_utils = Arc::new(FileSystemUtils::new());

    // Create a new unified project analyzer
    let analyzer = UnifiedProjectAnalyzer::new(project_path.to_path_buf(), fs_utils);

    // Analyze the project
    let _result = analyzer.analyze().await?;

    // Print analysis results
    println!("Unified project analysis completed successfully");

    // Generate central reference hub
    analyzer.generate_central_reference_hub().await?;

    // Generate analyzer reference
    analyzer.generate_analyzer_reference().await?;

    // Generate AI knowledge base
    analyzer.generate_ai_knowledge_base().await?;

    // Generate metrics visualizations
    analyzer.generate_metrics_visualizations().await?;

    // Generate project dashboard
    analyzer.generate_project_dashboard().await?;

    // Analyze models
    analyzer.analyze_models().await?;

    // Analyze API endpoints
    analyzer.analyze_api_endpoints().await?;

    // Analyze components
    analyzer.analyze_components().await?;

    // Analyze code quality
    analyzer.analyze_code_quality().await?;

    // Analyze tests
    analyzer.analyze_tests().await?;

    // Analyze integration
    analyzer.analyze_integration().await?;

    // Analyze architecture
    analyzer.analyze_architecture().await?;

    // Analyze sync system
    analyzer.analyze_sync_system().await?;

    // Analyze blockchain
    analyzer.analyze_blockchain().await?;

    // Generate recommendations
    analyzer.generate_recommendations().await?;

    // Update project status
    analyzer.update_project_status().await?;

    Ok(())
}
