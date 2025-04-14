use std::path::PathBuf;
use std::fs;
use chrono::Local;
use clap::Parser;
use lms_analyzer::{
    core::analyzer_config::AnalyzerConfig,
    runners::analysis_runner,
    generators::report_generator,
};

#[derive(Parser)]
#[command(author, version, about = "Generate a comprehensive analysis report")]
struct Cli {
    /// Output file path
    #[arg(short, long, default_value = "docs/comprehensive_report.md")]
    output: String,
    
    /// Whether to include technical debt analysis
    #[arg(long, default_value = "true")]
    tech_debt: bool,
    
    /// Whether to include code quality analysis
    #[arg(long, default_value = "true")]
    code_quality: bool,
    
    /// Whether to include model analysis
    #[arg(long, default_value = "true")]
    models: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command-line arguments
    let cli = Cli::parse();
    
    println!("Generating comprehensive analysis report...");
    
    // Run analysis
    let mut config = AnalyzerConfig::load(None)?;
    config.analyze_tech_debt = cli.tech_debt;
    config.analyze_code_quality = cli.code_quality;
    config.analyze_models = cli.models;
    
    let result = analysis_runner::run_analysis(config).await?;
    
    // Generate the comprehensive report
    let mut report = String::new();
    
    // Header
    report.push_str("# LMS Project: Comprehensive Analysis Report\n\n");
    report.push_str(&format!("_Generated on: {}_\n\n", Local::now().format("%Y-%m-%d %H:%M:%S")));
    
    // Table of Contents
    report.push_str("## Table of Contents\n\n");
    report.push_str("1. [Project Overview](#project-overview)\n");
    report.push_str("2. [Project Statistics](#project-statistics)\n");
    report.push_str("3. [Overall Progress](#overall-progress)\n");
    report.push_str("4. [Component Progress](#component-progress)\n");
    if cli.tech_debt {
        report.push_str("5. [Technical Debt Analysis](#technical-debt-analysis)\n");
    }
    if cli.code_quality {
        report.push_str("6. [Code Quality Analysis](#code-quality-analysis)\n");
    }
    if cli.models {
        report.push_str("7. [Data Model Analysis](#data-model-analysis)\n");
    }
    report.push_str("8. [Recent Changes](#recent-changes)\n");
    report.push_str("9. [Next Steps](#next-steps)\n");
    report.push_str("10. [Recommendations](#recommendations)\n\n");
    
    // Project Overview
    report.push_str("## Project Overview\n\n");
    report.push_str("The LMS (Learning Management System) project is a migration and integration of Canvas LMS and Discourse forum into a unified Rust/Tauri/Leptos application with Haskell components. The project prioritizes performance, security, and offline-first capabilities.\n\n");
    
    // Project Statistics
    report.push_str("## Project Statistics\n\n");
    report.push_str(&format!("- **Total Files**: {}\n", result.summary.total_files));
    report.push_str(&format!("- **Lines of Code**: {}\n", result.summary.lines_of_code));
    report.push_str(&format!("- **Rust Files**: {}\n", result.summary.rust_files));
    report.push_str(&format!("- **Haskell Files**: {}\n\n", result.summary.haskell_files));
    
    // File Types
    report.push_str("### File Types\n\n");
    report.push_str("| Extension | Count |\n");
    report.push_str("|-----------|-------|\n");
    
    for (ext, count) in &result.summary.file_types {
        report.push_str(&format!("| {} | {} |\n", ext, count));
    }
    report.push_str("\n");
    
    // Overall Progress
    report.push_str("## Overall Progress\n\n");
    report.push_str(&format!("**Project Completion: {:.1}%**\n\n", result.overall_progress));
    
    // Component Progress
    report.push_str("## Component Progress\n\n");
    report.push_str(&format!("- **Models**: {:.1}% ({}/{})\n", 
        result.models.implementation_percentage,
        result.models.implemented,
        result.models.total));
    report.push_str(&format!("- **API Endpoints**: {:.1}% ({}/{})\n", 
        result.api_endpoints.implementation_percentage,
        result.api_endpoints.implemented,
        result.api_endpoints.total));
    report.push_str(&format!("- **UI Components**: {:.1}% ({}/{})\n\n", 
        result.ui_components.implementation_percentage,
        result.ui_components.implemented,
        result.ui_components.total));
    
    // Feature Areas
    report.push_str("### Feature Areas\n\n");
    report.push_str("| Feature Area | Progress | Implemented / Total |\n");
    report.push_str("|--------------|---------|---------------------|\n");
    
    for (area, metrics) in &result.feature_areas {
        report.push_str(&format!("| {} | {:.1}% | {}/{} |\n",
            area,
            metrics.implementation_percentage,
            metrics.implemented,
            metrics.total));
    }
    report.push_str("\n");
    
    // Technical Debt Analysis
    if cli.tech_debt {
        report.push_str("## Technical Debt Analysis\n\n");
        report.push_str(&format!("**Total Issues: {}**\n\n", result.tech_debt_metrics.total_issues));
        report.push_str(&format!("- Critical: {}\n", result.tech_debt_metrics.critical_issues));
        report.push_str(&format!("- High: {}\n", result.tech_debt_metrics.high_issues));
        report.push_str(&format!("- Medium: {}\n", result.tech_debt_metrics.medium_issues));
        report.push_str(&format!("- Low: {}\n\n", result.tech_debt_metrics.low_issues));
        
        // Top Technical Debt Issues
        report.push_str("### Top Technical Debt Issues\n\n");
        
        if !result.tech_debt_metrics.items.is_empty() {
            report.push_str("| Severity | File | Line | Description |\n");
            report.push_str("|----------|------|------|-------------|\n");
            
            // Sort by severity
            let mut items = result.tech_debt_metrics.items.clone();
            items.sort_by(|a, b| {
                let a_severity = match a.severity {
                    crate::core::analysis_result::TechDebtSeverity::Critical => 3,
                    crate::core::analysis_result::TechDebtSeverity::High => 2,
                    crate::core::analysis_result::TechDebtSeverity::Medium => 1,
                    crate::core::analysis_result::TechDebtSeverity::Low => 0,
                };
                
                let b_severity = match b.severity {
                    crate::core::analysis_result::TechDebtSeverity::Critical => 3,
                    crate::core::analysis_result::TechDebtSeverity::High => 2,
                    crate::core::analysis_result::TechDebtSeverity::Medium => 1,
                    crate::core::analysis_result::TechDebtSeverity::Low => 0,
                };
                
                b_severity.cmp(&a_severity)
            });
            
            // Show top 10 issues
            for item in items.iter().take(10) {
                let severity = match item.severity {
                    crate::core::analysis_result::TechDebtSeverity::Critical => "Critical",
                    crate::core::analysis_result::TechDebtSeverity::High => "High",
                    crate::core::analysis_result::TechDebtSeverity::Medium => "Medium",
                    crate::core::analysis_result::TechDebtSeverity::Low => "Low",
                };
                
                report.push_str(&format!("| {} | {} | {} | {} |\n",
                    severity,
                    item.file,
                    item.line,
                    item.description));
            }
        } else {
            report.push_str("No technical debt issues found.\n");
        }
        
        report.push_str("\n");
    }
    
    // Code Quality Analysis
    if cli.code_quality {
        report.push_str("## Code Quality Analysis\n\n");
        
        // TODO: Add code quality metrics when available
        report.push_str("Code quality analysis is available in the [Code Quality Report](./quality/code_quality_report.md).\n\n");
    }
    
    // Data Model Analysis
    if cli.models {
        report.push_str("## Data Model Analysis\n\n");
        
        // TODO: Add model metrics when available
        report.push_str("Data model analysis is available in the [Model Report](./models/model_report.md).\n\n");
    }
    
    // Recent Changes
    report.push_str("## Recent Changes\n\n");
    for change in &result.recent_changes {
        report.push_str(&format!("- {}\n", change));
    }
    report.push_str("\n");
    
    // Next Steps
    report.push_str("## Next Steps\n\n");
    for step in &result.next_steps {
        report.push_str(&format!("- {}\n", step));
    }
    report.push_str("\n");
    
    // Recommendations
    report.push_str("## Recommendations\n\n");
    
    // Generate recommendations based on analysis results
    let mut recommendations = Vec::new();
    
    if cli.tech_debt && result.tech_debt_metrics.total_issues > 0 {
        recommendations.push("Address technical debt issues, starting with critical and high severity items.".to_string());
    }
    
    if result.models.implementation_percentage < 100.0 {
        recommendations.push("Complete the implementation of remaining data models.".to_string());
    }
    
    if result.api_endpoints.implementation_percentage < 100.0 {
        recommendations.push("Implement remaining API endpoints.".to_string());
    }
    
    if result.ui_components.implementation_percentage < 100.0 {
        recommendations.push("Complete the implementation of UI components.".to_string());
    }
    
    // Add general recommendations
    recommendations.push("Continue to improve test coverage.".to_string());
    recommendations.push("Regularly update documentation to reflect the current state of the project.".to_string());
    recommendations.push("Run the analyzers regularly to track progress and identify issues.".to_string());
    
    for recommendation in recommendations {
        report.push_str(&format!("- {}\n", recommendation));
    }
    
    // Write the report to file
    fs::write(&cli.output, report)?;
    
    println!("Comprehensive report generated at: {}", cli.output);
    
    Ok(())
}
