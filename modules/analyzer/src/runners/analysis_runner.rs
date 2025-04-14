use std::path::{Path, PathBuf};
use std::fs;

use crate::core::analyzer_config::AnalyzerConfig;
use crate::core::analysis_result::AnalysisResult;
use crate::core::unified_analyzer::UnifiedAnalyzer;

/// Run analysis with the given configuration
pub async fn run_analysis(config: AnalyzerConfig) -> Result<AnalysisResult, String> {
    println!("Running analysis with configuration...");

    // Create analyzer
    let analyzer = UnifiedAnalyzer::new(config);

    // Run analysis
    let result = analyzer.analyze().await?;

    println!("Analysis completed successfully.");

    Ok(result)
}

/// Run analysis with command-line arguments
pub async fn run_analysis_with_args(
    target_dirs: Option<Vec<String>>,
    exclude_patterns: Option<Vec<String>>,
    output_dir: Option<String>,
    update_rag: bool,
    generate_insights: bool,
    analyze_js: bool,
    generate_dashboard: bool,
    analyze_tech_debt: bool,
    analyze_code_quality: bool,
    analyze_models: bool,
) -> Result<AnalysisResult, String> {
    println!("Running analysis with command-line arguments...");

    // Load default configuration
    let mut config = AnalyzerConfig::load(None)?;

    // Update configuration with command-line arguments
    if let Some(dirs) = target_dirs {
        config.target_dirs = dirs.iter().map(PathBuf::from).collect();
    }

    if let Some(patterns) = exclude_patterns {
        config.exclude_patterns = patterns;
    }

    if let Some(dir) = output_dir {
        config.output_dir = PathBuf::from(dir);
    }

    config.update_rag_knowledge_base = update_rag;
    config.generate_ai_insights = generate_insights;
    config.analyze_js_files = analyze_js;
    config.generate_dashboard = generate_dashboard;
    config.analyze_tech_debt = analyze_tech_debt;
    config.analyze_code_quality = analyze_code_quality;
    config.analyze_models = analyze_models;

    // Run analysis
    run_analysis(config).await
}

/// Run quick analysis
pub async fn run_quick_analysis() -> Result<AnalysisResult, String> {
    println!("Running quick analysis...");

    // Load default configuration
    let mut config = AnalyzerConfig::load(None)?;

    // Set quick mode
    config.quick_mode = true;

    // Run analysis
    run_analysis(config).await
}
