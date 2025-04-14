use std::fs;
use std::path::{Path, PathBuf};
use chrono::Local;

use crate::core::analysis_result::AnalysisResult;
use crate::core::statistical_trend_analyzer::StatisticalTrendAnalyzer;

/// Generate statistical trend report
pub fn generate_statistical_trend_report(result: &AnalysisResult) -> Result<(), String> {
    println!("Generating statistical trend report...");
    
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
    let analyzer = StatisticalTrendAnalyzer::new(Path::new(".").to_path_buf());
    
    // Create the report path
    let report_path = trends_dir.join("statistical_trend_report.md");
    
    // Generate the report
    let report = match analyzer.generate_report() {
        Ok(report) => report,
        Err(e) => {
            if e.contains("Not enough data") {
                let mut report = String::new();
                report.push_str("# Statistical Trend Analysis Report\n\n");
                report.push_str(&format!("_Generated on: {}_\n\n", Local::now().format("%Y-%m-%d")));
                report.push_str("## Insufficient Data\n\n");
                report.push_str("Not enough data is available for statistical trend analysis. At least 3 data points are required.\n\n");
                report.push_str("Please run the analyzer multiple times over a period of time to collect enough data for trend analysis.\n\n");
                report.push_str("## Recommendations\n\n");
                report.push_str("- Run the analyzer regularly (e.g., daily or weekly) to collect trend data.\n");
                report.push_str("- Use the trend report generator after collecting at least 3 data points.\n");
                report.push_str("- Consider automating the analysis using the scheduled task feature.\n");
                report
            } else {
                return Err(e);
            }
        }
    };
    
    // Write to file
    fs::write(&report_path, report)
        .map_err(|e| format!("Failed to write statistical trend report: {}", e))?;
    
    println!("Statistical trend report generated at: {:?}", report_path);
    
    // Create a summary report
    let summary_path = docs_dir.join("statistical_trend_summary.md");
    
    // Generate the summary
    let summary = generate_summary(&analyzer)?;
    
    // Write to file
    fs::write(&summary_path, summary)
        .map_err(|e| format!("Failed to write statistical trend summary: {}", e))?;
    
    println!("Statistical trend summary generated at: {:?}", summary_path);
    
    Ok(())
}

/// Generate a summary of the statistical trend metrics
fn generate_summary(analyzer: &StatisticalTrendAnalyzer) -> Result<String, String> {
    let mut summary = String::new();
    
    // Header
    summary.push_str("# Statistical Trend Analysis Summary\n\n");
    summary.push_str(&format!("_Generated on: {}_\n\n", Local::now().format("%Y-%m-%d")));
    
    // Try to analyze trends
    let metrics = match analyzer.analyze_trends() {
        Ok(metrics) => metrics,
        Err(e) => {
            if e.contains("Not enough data") {
                summary.push_str("## Insufficient Data\n\n");
                summary.push_str("Not enough data is available for statistical trend analysis. At least 3 data points are required.\n\n");
                summary.push_str("Please run the analyzer multiple times over a period of time to collect enough data for trend analysis.\n\n");
                return Ok(summary);
            } else {
                return Err(e);
            }
        }
    };
    
    // Project Velocity
    summary.push_str("## Project Velocity\n\n");
    summary.push_str(&format!("- **Current Velocity**: {:.2} percentage points per week\n", metrics.velocity));
    summary.push_str(&format!("- **Acceleration**: {:.2} percentage points per week²\n\n", metrics.acceleration));
    
    // Estimated Completion Date
    summary.push_str("## Estimated Completion Date\n\n");
    
    if let Some(completion_date) = metrics.completion_date_estimate {
        summary.push_str(&format!("**Estimated Completion Date**: {}\n", completion_date.format("%Y-%m-%d")));
        
        if let Some((lower_date, upper_date)) = metrics.completion_date_confidence {
            summary.push_str(&format!("**95% Confidence Interval**: {} to {}\n", lower_date.format("%Y-%m-%d"), upper_date.format("%Y-%m-%d")));
        }
        
        // Calculate days until completion
        let days_until_completion = (completion_date - Local::now()).num_days();
        
        summary.push_str(&format!("\nAt the current pace, the project will be completed in approximately {} days.\n", days_until_completion));
    } else {
        summary.push_str("**Estimated Completion Date**: Unable to estimate completion date due to insufficient progress.\n");
    }
    
    summary.push_str("\n");
    
    // Top Forecasts
    summary.push_str("## Key Forecasts (1 Month)\n\n");
    summary.push_str("| Metric | Current | Forecast | Growth Rate |\n");
    summary.push_str("|--------|---------|----------|-------------|\n");
    
    for (metric_name, forecast) in &metrics.forecasts {
        let formatted_metric = match metric_name.as_str() {
            "overall_progress" => "Overall Progress",
            "models_percentage" => "Models Implementation",
            "api_endpoints_percentage" => "API Endpoints Implementation",
            "ui_components_percentage" => "UI Components Implementation",
            "tech_debt_issues" => "Technical Debt Issues",
            _ => metric_name,
        };
        
        summary.push_str(&format!("| {} | {:.2} | {:.2} | {:.2}% per week |\n",
            formatted_metric,
            forecast.current_value,
            forecast.forecast_1_month,
            forecast.growth_rate));
    }
    
    summary.push_str("\n");
    
    // Recommendations
    summary.push_str("## Key Recommendations\n\n");
    
    // Generate recommendations based on the analysis
    let mut recommendations = Vec::new();
    
    // Velocity recommendations
    if metrics.velocity < 1.0 {
        recommendations.push("Increase development velocity by focusing on high-impact features.".to_string());
    } else if metrics.velocity > 10.0 {
        recommendations.push("Maintain the current high velocity, but ensure quality is not being sacrificed.".to_string());
    }
    
    // Acceleration recommendations
    if metrics.acceleration < -0.5 {
        recommendations.push("Address the decreasing velocity trend by identifying and removing bottlenecks.".to_string());
    } else if metrics.acceleration > 0.5 {
        recommendations.push("Capitalize on the increasing velocity trend by continuing current practices.".to_string());
    }
    
    // Technical debt recommendations
    if let Some(tech_debt_forecast) = metrics.forecasts.get("tech_debt_issues") {
        if tech_debt_forecast.trend_direction == crate::core::statistical_trend_analyzer::TrendDirection::Increasing {
            recommendations.push("Address the increasing technical debt trend by allocating time for debt reduction.".to_string());
        }
    }
    
    // Add general recommendations
    recommendations.push("Continue collecting project metrics to improve forecast accuracy.".to_string());
    
    for recommendation in recommendations {
        summary.push_str(&format!("- {}\n", recommendation));
    }
    
    summary.push_str("\nFor detailed information, see the [full statistical trend report](./trends/statistical_trend_report.md).\n");
    
    Ok(summary)
}
