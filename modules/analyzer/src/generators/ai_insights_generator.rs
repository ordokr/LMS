use std::fs;
use std::path::{Path, PathBuf};
use chrono::Local;

use crate::core::analysis_result::AnalysisResult;
use crate::core::ai_insights_analyzer::{AiInsightsAnalyzer, AiInsights, InsightPriority, InsightCategory};

/// Generate AI insights report
pub fn generate_ai_insights_report(result: &AnalysisResult) -> Result<(), String> {
    println!("Generating AI insights report...");
    
    // Ensure docs directory exists
    let docs_dir = Path::new("docs");
    if !docs_dir.exists() {
        fs::create_dir_all(docs_dir)
            .map_err(|e| format!("Failed to create docs directory: {}", e))?;
    }
    
    // Ensure insights directory exists
    let insights_dir = docs_dir.join("insights");
    if !insights_dir.exists() {
        fs::create_dir_all(&insights_dir)
            .map_err(|e| format!("Failed to create insights directory: {}", e))?;
    }
    
    // Create the analyzer
    let analyzer = AiInsightsAnalyzer::new(Path::new(".").to_path_buf());
    
    // Generate insights
    let insights = analyzer.generate_insights(result)?;
    
    // Create the report path
    let report_path = insights_dir.join("ai_insights_report.md");
    
    // Generate the report
    let report = analyzer.generate_report(&insights)?;
    
    // Write to file
    fs::write(&report_path, report)
        .map_err(|e| format!("Failed to write AI insights report: {}", e))?;
    
    println!("AI insights report generated at: {:?}", report_path);
    
    // Create a summary report
    let summary_path = docs_dir.join("ai_insights_summary.md");
    
    // Generate the summary
    let summary = generate_summary(&insights);
    
    // Write to file
    fs::write(&summary_path, summary)
        .map_err(|e| format!("Failed to write AI insights summary: {}", e))?;
    
    println!("AI insights summary generated at: {:?}", summary_path);
    
    Ok(())
}

/// Generate a summary of the AI insights
fn generate_summary(insights: &AiInsights) -> String {
    let mut summary = String::new();
    
    // Header
    summary.push_str("# AI Insights Summary\n\n");
    summary.push_str(&format!("_Generated on: {}_\n\n", Local::now().format("%Y-%m-%d")));
    
    // Summary
    summary.push_str("## Summary\n\n");
    summary.push_str(&format!("**Total Insights: {}**\n\n", insights.insights.len()));
    
    // Insights by priority
    summary.push_str("| Priority | Count |\n");
    summary.push_str("|----------|-------|\n");
    
    let critical_count = insights.insights_by_priority.get(&InsightPriority::Critical).map_or(0, |v| v.len());
    let high_count = insights.insights_by_priority.get(&InsightPriority::High).map_or(0, |v| v.len());
    let medium_count = insights.insights_by_priority.get(&InsightPriority::Medium).map_or(0, |v| v.len());
    let low_count = insights.insights_by_priority.get(&InsightPriority::Low).map_or(0, |v| v.len());
    
    summary.push_str(&format!("| Critical | {} |\n", critical_count));
    summary.push_str(&format!("| High | {} |\n", high_count));
    summary.push_str(&format!("| Medium | {} |\n", medium_count));
    summary.push_str(&format!("| Low | {} |\n\n", low_count));
    
    // Top Insights
    summary.push_str("## Top Insights\n\n");
    
    for insight in &insights.top_insights {
        let priority_str = match insight.priority {
            InsightPriority::Critical => "⚠️ Critical",
            InsightPriority::High => "🔴 High",
            InsightPriority::Medium => "🟠 Medium",
            InsightPriority::Low => "🟢 Low",
        };
        
        summary.push_str(&format!("### {} - {}\n\n", priority_str, insight.title));
        summary.push_str(&format!("{}\n\n", insight.description));
        
        summary.push_str("**Recommendations:**\n\n");
        for recommendation in &insight.recommendations {
            summary.push_str(&format!("- {}\n", recommendation));
        }
        summary.push_str("\n");
    }
    
    // Insights by Category
    summary.push_str("## Insights by Category\n\n");
    
    summary.push_str("| Category | Count |\n");
    summary.push_str("|----------|-------|\n");
    
    for (category, category_insights) in &insights.insights_by_category {
        let category_name = match category {
            InsightCategory::TechnicalDebt => "Technical Debt",
            InsightCategory::CodeQuality => "Code Quality",
            InsightCategory::Architecture => "Architecture",
            InsightCategory::Performance => "Performance",
            InsightCategory::Security => "Security",
            InsightCategory::Testing => "Testing",
            InsightCategory::Documentation => "Documentation",
            InsightCategory::Dependencies => "Dependencies",
            InsightCategory::ProjectProgress => "Project Progress",
        };
        
        summary.push_str(&format!("| {} | {} |\n", category_name, category_insights.len()));
    }
    
    summary.push_str("\n");
    
    // Link to full report
    summary.push_str("For detailed information, see the [full AI insights report](./insights/ai_insights_report.md).\n");
    
    summary
}
