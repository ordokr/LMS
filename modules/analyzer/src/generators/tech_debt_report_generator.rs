use std::fs;
use std::path::{Path, PathBuf};
use chrono::Local;
use std::collections::HashMap;

use crate::core::analysis_result::{AnalysisResult, TechDebtSeverity};

/// Generate technical debt report
pub fn generate_tech_debt_report(result: &AnalysisResult) -> Result<(), String> {
    println!("Generating technical debt report...");
    
    // Ensure docs directory exists
    let docs_dir = Path::new("docs");
    if !docs_dir.exists() {
        fs::create_dir_all(docs_dir)
            .map_err(|e| format!("Failed to create docs directory: {}", e))?;
    }
    
    // Create the report path
    let report_path = docs_dir.join("technical_debt_report.md");
    
    // Generate the content
    let mut content = String::new();
    
    // Header
    content.push_str("# Technical Debt Report\n\n");
    content.push_str(&format!("_Generated on: {}_\n\n", Local::now().format("%Y-%m-%d")));
    
    // Summary
    content.push_str("## Summary\n\n");
    content.push_str(&format!("**Total Issues: {}**\n\n", result.tech_debt_metrics.total_issues));
    content.push_str("| Severity | Count |\n");
    content.push_str("|----------|-------|\n");
    content.push_str(&format!("| Critical | {} |\n", result.tech_debt_metrics.critical_issues));
    content.push_str(&format!("| High | {} |\n", result.tech_debt_metrics.high_issues));
    content.push_str(&format!("| Medium | {} |\n", result.tech_debt_metrics.medium_issues));
    content.push_str(&format!("| Low | {} |\n\n", result.tech_debt_metrics.low_issues));
    
    // Critical Issues
    content.push_str("## Critical Issues\n\n");
    if result.tech_debt_metrics.critical_issues > 0 {
        content.push_str("| File | Line | Description | Fix Suggestion |\n");
        content.push_str("|------|------|-------------|---------------|\n");
        
        for item in &result.tech_debt_metrics.items {
            if item.severity == TechDebtSeverity::Critical {
                content.push_str(&format!("| {} | {} | {} | {} |\n",
                    item.file,
                    item.line,
                    item.description,
                    item.fix_suggestion));
            }
        }
    } else {
        content.push_str("No critical issues found.\n");
    }
    content.push_str("\n");
    
    // High Issues
    content.push_str("## High Issues\n\n");
    if result.tech_debt_metrics.high_issues > 0 {
        content.push_str("| File | Line | Description | Fix Suggestion |\n");
        content.push_str("|------|------|-------------|---------------|\n");
        
        for item in &result.tech_debt_metrics.items {
            if item.severity == TechDebtSeverity::High {
                content.push_str(&format!("| {} | {} | {} | {} |\n",
                    item.file,
                    item.line,
                    item.description,
                    item.fix_suggestion));
            }
        }
    } else {
        content.push_str("No high issues found.\n");
    }
    content.push_str("\n");
    
    // Medium Issues
    content.push_str("## Medium Issues\n\n");
    if result.tech_debt_metrics.medium_issues > 0 {
        content.push_str("| File | Line | Description | Fix Suggestion |\n");
        content.push_str("|------|------|-------------|---------------|\n");
        
        for item in &result.tech_debt_metrics.items {
            if item.severity == TechDebtSeverity::Medium {
                content.push_str(&format!("| {} | {} | {} | {} |\n",
                    item.file,
                    item.line,
                    item.description,
                    item.fix_suggestion));
            }
        }
    } else {
        content.push_str("No medium issues found.\n");
    }
    content.push_str("\n");
    
    // Low Issues
    content.push_str("## Low Issues\n\n");
    if result.tech_debt_metrics.low_issues > 0 {
        content.push_str("| File | Line | Description | Fix Suggestion |\n");
        content.push_str("|------|------|-------------|---------------|\n");
        
        for item in &result.tech_debt_metrics.items {
            if item.severity == TechDebtSeverity::Low {
                content.push_str(&format!("| {} | {} | {} | {} |\n",
                    item.file,
                    item.line,
                    item.description,
                    item.fix_suggestion));
            }
        }
    } else {
        content.push_str("No low issues found.\n");
    }
    
    // Write to file
    fs::write(&report_path, content)
        .map_err(|e| format!("Failed to write technical debt report: {}", e))?;
    
    println!("Technical debt report generated at: {:?}", report_path);
    
    Ok(())
}
