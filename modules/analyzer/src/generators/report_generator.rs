use std::fs;
use std::path::{Path, PathBuf};
use chrono::Local;

use crate::core::analysis_result::AnalysisResult;
use crate::core::analyzer_config::AnalyzerConfig;
use crate::generators::central_hub_generator;
use crate::generators::architecture_doc_generator;
use crate::generators::models_doc_generator;
use crate::generators::api_doc_generator;
use crate::generators::tech_debt_report_generator;
use crate::generators::code_quality_report_generator;
use crate::generators::model_report_generator;
use crate::generators::dependency_report_generator;
use crate::generators::trend_report_generator;
use crate::generators::dashboard_generator;
use crate::generators::enhanced_dashboard_generator;
use crate::generators::project_doc_generator;
use crate::generators::statistical_trend_generator;

/// Generate all reports based on the analysis result
pub fn generate_reports(result: &AnalysisResult) -> Result<(), String> {
    println!("Generating reports...");

    // Ensure docs directory exists
    let docs_dir = Path::new("docs");
    if !docs_dir.exists() {
        fs::create_dir_all(docs_dir)
            .map_err(|e| format!("Failed to create docs directory: {}", e))?;
    }

    // Generate central reference hub
    central_hub_generator::generate_central_reference_hub(result)?;

    // Generate architecture documentation
    architecture_doc_generator::generate_architecture_doc(result)?;

    // Generate models documentation
    models_doc_generator::generate_models_doc(result)?;

    // Generate API documentation
    api_doc_generator::generate_api_doc(result)?;

    // Generate technical debt report
    tech_debt_report_generator::generate_tech_debt_report(result)?;

    // Generate code quality report
    code_quality_report_generator::generate_code_quality_report(result)?;

    // Generate model report
    model_report_generator::generate_model_report(result)?;

    // Generate dependency report
    dependency_report_generator::generate_dependency_report(result)?;

    // Generate trend reports
    trend_report_generator::generate_trend_report(result)?;
    statistical_trend_generator::generate_statistical_trend_report(result)?;

    // Generate dashboards
    dashboard_generator::generate_dashboard(result)?;
    enhanced_dashboard_generator::generate_enhanced_dashboard(result)?;

    println!("All reports generated successfully.");

    Ok(())
}

/// Generate reports using the project analyzer
pub async fn generate_project_analyzer_reports(config: &AnalyzerConfig) -> Result<(), String> {
    println!("Generating reports using project analyzer...");

    // Run project analysis and generate documentation
    project_doc_generator::run_project_analysis_and_generate_docs(config).await?;

    println!("Project analyzer reports generated successfully.");

    Ok(())
}

/// Generate a summary report
pub fn generate_summary_report(result: &AnalysisResult) -> Result<(), String> {
    println!("Generating summary report...");

    // Ensure docs directory exists
    let docs_dir = Path::new("docs");
    if !docs_dir.exists() {
        fs::create_dir_all(docs_dir)
            .map_err(|e| format!("Failed to create docs directory: {}", e))?;
    }

    // Create the summary report path
    let report_path = docs_dir.join("SUMMARY_REPORT.md");

    // Generate the content
    let mut content = String::new();

    // Header
    content.push_str("# LMS Project: Summary Report\n\n");
    content.push_str(&format!("_Generated on: {}_\n\n", Local::now().format("%Y-%m-%d")));

    // Overall Progress
    content.push_str("## Overall Progress\n\n");
    content.push_str(&format!("**Project Completion: {:.1}%**\n\n", result.overall_progress));

    // Component Progress
    content.push_str("## Component Progress\n\n");
    content.push_str(&format!("- **Models**: {:.1}% ({}/{})\n",
        result.models.implementation_percentage,
        result.models.implemented,
        result.models.total));
    content.push_str(&format!("- **API Endpoints**: {:.1}% ({}/{})\n",
        result.api_endpoints.implementation_percentage,
        result.api_endpoints.implemented,
        result.api_endpoints.total));
    content.push_str(&format!("- **UI Components**: {:.1}% ({}/{})\n\n",
        result.ui_components.implementation_percentage,
        result.ui_components.implemented,
        result.ui_components.total));

    // Feature Areas
    content.push_str("## Feature Areas\n\n");
    content.push_str("| Feature Area | Progress | Implemented / Total |\n");
    content.push_str("|--------------|---------|---------------------|\n");

    for (area, metrics) in &result.feature_areas {
        content.push_str(&format!("| {} | {:.1}% | {}/{} |\n",
            area,
            metrics.implementation_percentage,
            metrics.implemented,
            metrics.total));
    }
    content.push_str("\n");

    // Technical Debt
    content.push_str("## Technical Debt\n\n");
    content.push_str(&format!("**Total Issues: {}**\n\n", result.tech_debt_metrics.total_issues));
    content.push_str(&format!("- Critical: {}\n", result.tech_debt_metrics.critical_issues));
    content.push_str(&format!("- High: {}\n", result.tech_debt_metrics.high_issues));
    content.push_str(&format!("- Medium: {}\n", result.tech_debt_metrics.medium_issues));
    content.push_str(&format!("- Low: {}\n\n", result.tech_debt_metrics.low_issues));

    // Recent Changes
    content.push_str("## Recent Changes\n\n");
    for change in &result.recent_changes {
        content.push_str(&format!("- {}\n", change));
    }
    content.push_str("\n");

    // Next Steps
    content.push_str("## Next Steps\n\n");
    for step in &result.next_steps {
        content.push_str(&format!("- {}\n", step));
    }

    // Write to file
    fs::write(&report_path, content)
        .map_err(|e| format!("Failed to write summary report: {}", e))?;

    println!("Summary report generated at: {:?}", report_path);

    Ok(())
}
