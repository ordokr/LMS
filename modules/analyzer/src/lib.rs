// Unified Analyzer Module
//
// This module serves as the central point for all analysis functionality in the LMS project.
// It consolidates various analyzers that were previously scattered throughout the codebase.

pub mod core;
pub mod runners;
pub mod generators;
pub mod utils;

// Re-export key structures for easier access
pub use core::unified_analyzer::UnifiedAnalyzer;
pub use core::analysis_result::AnalysisResult;
pub use core::analyzer_config::AnalyzerConfig;
pub use core::project_analyzer::ProjectAnalyzer;
pub use runners::analysis_runner::run_analysis;
pub use generators::report_generator::{generate_reports, generate_project_analyzer_reports};
pub use generators::project_doc_generator::{generate_project_docs, run_project_analysis_and_generate_docs};

/// Run a complete analysis of the project
pub async fn analyze_project(config_path: Option<&str>) -> Result<AnalysisResult, String> {
    // Load configuration
    let config = AnalyzerConfig::load(config_path)?;

    // Create analyzer
    let analyzer = UnifiedAnalyzer::new(config);

    // Run analysis
    let result = analyzer.analyze().await?;

    // Generate reports
    generate_reports(&result)?;

    Ok(result)
}

/// Run a quick analysis of the project
pub async fn quick_analyze_project() -> Result<AnalysisResult, String> {
    // Load configuration with quick analysis settings
    let mut config = AnalyzerConfig::load(None)?;
    config.quick_mode = true;

    // Create analyzer
    let analyzer = UnifiedAnalyzer::new(config);

    // Run analysis
    analyzer.analyze().await
}

/// Update the central reference hub
pub async fn update_central_reference_hub() -> Result<(), String> {
    // Run a quick analysis
    let result = quick_analyze_project().await?;

    // Generate only the central reference hub
    generators::central_hub_generator::generate_central_reference_hub(&result)
}

/// Run the project analyzer and generate documentation
pub async fn run_project_analyzer(config_path: Option<&str>) -> Result<(), String> {
    // Load configuration
    let config = AnalyzerConfig::load(config_path)?;

    // Generate reports using the project analyzer
    generate_project_analyzer_reports(&config).await
}
