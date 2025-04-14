use std::fs;
use std::path::{Path, PathBuf};
use chrono::Local;

use crate::core::analysis_result::AnalysisResult;
use crate::core::trend_analyzer::TrendAnalyzer;

/// Generate trend report
pub fn generate_trend_report(result: &AnalysisResult) -> Result<(), String> {
    println!("Generating trend report...");
    
    // Ensure docs directory exists
    let docs_dir = Path::new("docs");
    if !docs_dir.exists() {
        fs::create_dir_all(docs_dir)
            .map_err(|e| format!("Failed to create docs directory: {}", e))?;
    }
    
    // Ensure trends directory exists
    let trends_dir = docs_dir.join("trends");
    if !trends_dir.exists() {
        fs::create_dir_all(&trends_dir)
            .map_err(|e| format!("Failed to create trends directory: {}", e))?;
    }
    
    // Create the analyzer
    let analyzer = TrendAnalyzer::new(Path::new(".").to_path_buf());
    
    // Add the current analysis to the history
    analyzer.add_entry(result)?;
    
    // Create the report path
    let report_path = trends_dir.join("trend_report.md");
    
    // Generate the report
    let report = analyzer.generate_report()?;
    
    // Write to file
    fs::write(&report_path, report)
        .map_err(|e| format!("Failed to write trend report: {}", e))?;
    
    println!("Trend report generated at: {:?}", report_path);
    
    // Create a summary report
    let summary_path = docs_dir.join("trend_summary.md");
    
    // Generate the summary
    let summary = generate_summary(&analyzer)?;
    
    // Write to file
    fs::write(&summary_path, summary)
        .map_err(|e| format!("Failed to write trend summary: {}", e))?;
    
    println!("Trend summary generated at: {:?}", summary_path);
    
    Ok(())
}

/// Generate a summary of the trend metrics
fn generate_summary(analyzer: &TrendAnalyzer) -> Result<String, String> {
    // Analyze trends
    let metrics = analyzer.analyze_trends()?;
    
    let mut summary = String::new();
    
    // Header
    summary.push_str("# Trend Analysis Summary\n\n");
    summary.push_str(&format!("_Generated on: {}_\n\n", Local::now().format("%Y-%m-%d")));
    
    if metrics.history.entries.is_empty() {
        summary.push_str("No history entries found. Run the analyzer multiple times to generate trend data.\n\n");
        return Ok(summary);
    }
    
    // Latest Analysis
    let latest = &metrics.history.entries[0];
    
    summary.push_str("## Latest Analysis\n\n");
    summary.push_str(&format!("**Date: {}**\n\n", latest.timestamp.format("%Y-%m-%d %H:%M:%S")));
    summary.push_str(&format!("- Total Files: {}\n", latest.total_files));
    summary.push_str(&format!("- Lines of Code: {}\n", latest.lines_of_code));
    summary.push_str(&format!("- Overall Progress: {:.1}%\n\n", latest.overall_progress));
    
    // Changes Since Last Analysis
    summary.push_str("## Changes Since Last Analysis\n\n");
    
    if !metrics.changes.is_empty() {
        summary.push_str("| Metric | Change |\n");
        summary.push_str("|--------|--------|\n");
        
        for (metric, change) in &metrics.changes {
            let formatted_metric = match metric.as_str() {
                "total_files" => "Total Files",
                "lines_of_code" => "Lines of Code",
                "rust_files" => "Rust Files",
                "haskell_files" => "Haskell Files",
                "overall_progress" => "Overall Progress",
                "models_percentage" => "Models Implementation",
                "api_endpoints_percentage" => "API Endpoints Implementation",
                "ui_components_percentage" => "UI Components Implementation",
                "tech_debt_issues" => "Technical Debt Issues",
                _ => metric,
            };
            
            let formatted_change = if metric.contains("percentage") || metric == "overall_progress" {
                format!("{:+.1} percentage points", change)
            } else {
                format!("{:+.1}%", change)
            };
            
            summary.push_str(&format!("| {} | {} |\n", formatted_metric, formatted_change));
        }
    } else {
        summary.push_str("No previous analysis found for comparison.\n");
    }
    
    summary.push_str("\n");
    
    // Weekly Changes
    summary.push_str("## Weekly Changes\n\n");
    
    if !metrics.weekly_changes.is_empty() {
        summary.push_str("| Metric | Change |\n");
        summary.push_str("|--------|--------|\n");
        
        for (metric, change) in &metrics.weekly_changes {
            let formatted_metric = match metric.as_str() {
                "total_files" => "Total Files",
                "lines_of_code" => "Lines of Code",
                "rust_files" => "Rust Files",
                "haskell_files" => "Haskell Files",
                "overall_progress" => "Overall Progress",
                "models_percentage" => "Models Implementation",
                "api_endpoints_percentage" => "API Endpoints Implementation",
                "ui_components_percentage" => "UI Components Implementation",
                "tech_debt_issues" => "Technical Debt Issues",
                _ => metric,
            };
            
            let formatted_change = if metric.contains("percentage") || metric == "overall_progress" {
                format!("{:+.1} percentage points", change)
            } else {
                format!("{:+.1}%", change)
            };
            
            summary.push_str(&format!("| {} | {} |\n", formatted_metric, formatted_change));
        }
    } else {
        summary.push_str("No analysis from a week ago found for comparison.\n");
    }
    
    summary.push_str("\n");
    
    // Recommendations
    summary.push_str("## Recommendations\n\n");
    
    // Generate recommendations based on trend data
    let mut recommendations = Vec::new();
    
    if !metrics.changes.is_empty() {
        // Check if overall progress is decreasing
        if let Some(progress_change) = metrics.changes.get("overall_progress") {
            if *progress_change < 0.0 {
                recommendations.push("Overall progress has decreased since the last analysis. Review recent changes to identify issues.".to_string());
            } else if *progress_change < 1.0 {
                recommendations.push("Overall progress is increasing slowly. Consider focusing on high-priority features.".to_string());
            } else {
                recommendations.push("Overall progress is increasing at a good pace. Continue the current development strategy.".to_string());
            }
        }
        
        // Check if technical debt is increasing
        if let Some(tech_debt_change) = metrics.changes.get("tech_debt_issues") {
            if *tech_debt_change > 0.0 {
                recommendations.push("Technical debt has increased since the last analysis. Consider allocating time to address technical debt issues.".to_string());
            } else if *tech_debt_change < 0.0 {
                recommendations.push("Technical debt has decreased since the last analysis. Continue the good work on reducing technical debt.".to_string());
            }
        }
    }
    
    // Add general recommendations
    recommendations.push("Run the analyzer regularly to track progress and identify trends.".to_string());
    recommendations.push("Review the full trend report for detailed information on project progress.".to_string());
    
    for recommendation in recommendations {
        summary.push_str(&format!("- {}\n", recommendation));
    }
    
    summary.push_str("\nFor detailed information, see the [full trend report](./trends/trend_report.md).\n");
    
    Ok(summary)
}
