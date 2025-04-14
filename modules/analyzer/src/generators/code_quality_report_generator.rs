use std::fs;
use std::path::{Path, PathBuf};
use chrono::Local;
use std::collections::HashMap;

use crate::core::analysis_result::AnalysisResult;
use crate::core::code_quality_analyzer::{CodeQualityAnalyzer, CodeQualityMetrics};

/// Generate code quality report
pub fn generate_code_quality_report(result: &AnalysisResult) -> Result<(), String> {
    println!("Generating code quality report...");
    
    // Ensure docs directory exists
    let docs_dir = Path::new("docs");
    if !docs_dir.exists() {
        fs::create_dir_all(docs_dir)
            .map_err(|e| format!("Failed to create docs directory: {}", e))?;
    }
    
    // Ensure quality directory exists
    let quality_dir = docs_dir.join("quality");
    if !quality_dir.exists() {
        fs::create_dir_all(&quality_dir)
            .map_err(|e| format!("Failed to create quality directory: {}", e))?;
    }
    
    // Create the report path
    let report_path = quality_dir.join("code_quality_report.md");
    
    // Create the analyzer
    let analyzer = CodeQualityAnalyzer::new(Path::new(".").to_path_buf());
    
    // Generate the report
    let report = analyzer.generate_report()?;
    
    // Write to file
    fs::write(&report_path, report)
        .map_err(|e| format!("Failed to write code quality report: {}", e))?;
    
    println!("Code quality report generated at: {:?}", report_path);
    
    // Create a summary report
    let summary_path = docs_dir.join("code_quality_summary.md");
    
    // Analyze the codebase
    let metrics = analyzer.analyze_codebase()?;
    
    // Generate the summary
    let summary = generate_summary(&metrics);
    
    // Write to file
    fs::write(&summary_path, summary)
        .map_err(|e| format!("Failed to write code quality summary: {}", e))?;
    
    println!("Code quality summary generated at: {:?}", summary_path);
    
    Ok(())
}

/// Generate a summary of the code quality metrics
fn generate_summary(metrics: &CodeQualityMetrics) -> String {
    let mut summary = String::new();
    
    // Header
    summary.push_str("# Code Quality Summary\n\n");
    summary.push_str(&format!("_Generated on: {}_\n\n", Local::now().format("%Y-%m-%d")));
    
    // SOLID Violations
    summary.push_str("## SOLID Violations\n\n");
    
    let mut solid_counts = HashMap::new();
    for violation in &metrics.solid_violations {
        *solid_counts.entry(violation.principle.clone()).or_insert(0) += 1;
    }
    
    summary.push_str("| Principle | Violations |\n");
    summary.push_str("|-----------|------------|\n");
    
    for principle in &["Single Responsibility", "Open-Closed", "Liskov Substitution", "Interface Segregation", "Dependency Inversion"] {
        let count = solid_counts.get(*principle).unwrap_or(&0);
        summary.push_str(&format!("| {} | {} |\n", principle, count));
    }
    
    summary.push_str(&format!("\n**Total SOLID Violations:** {}\n\n", metrics.solid_violations.len()));
    
    // Design Pattern Implementations
    summary.push_str("## Design Pattern Implementations\n\n");
    
    let mut pattern_counts = HashMap::new();
    for implementation in &metrics.pattern_implementations {
        *pattern_counts.entry(implementation.pattern.clone()).or_insert(0) += 1;
    }
    
    if !pattern_counts.is_empty() {
        summary.push_str("| Pattern | Implementations |\n");
        summary.push_str("|---------|----------------|\n");
        
        for (pattern, count) in &pattern_counts {
            summary.push_str(&format!("| {} | {} |\n", pattern, count));
        }
    } else {
        summary.push_str("No design pattern implementations found.\n");
    }
    
    summary.push_str(&format!("\n**Total Pattern Implementations:** {}\n\n", metrics.pattern_implementations.len()));
    
    // Design Pattern Violations
    summary.push_str("## Design Pattern Violations\n\n");
    
    let mut violation_counts = HashMap::new();
    for violation in &metrics.pattern_violations {
        *violation_counts.entry(violation.pattern.clone()).or_insert(0) += 1;
    }
    
    if !violation_counts.is_empty() {
        summary.push_str("| Pattern | Violations |\n");
        summary.push_str("|---------|------------|\n");
        
        for (pattern, count) in &violation_counts {
            summary.push_str(&format!("| {} | {} |\n", pattern, count));
        }
    } else {
        summary.push_str("No design pattern violations found.\n");
    }
    
    summary.push_str(&format!("\n**Total Pattern Violations:** {}\n\n", metrics.pattern_violations.len()));
    
    // Recommendations
    summary.push_str("## Recommendations\n\n");
    
    if !metrics.solid_violations.is_empty() {
        summary.push_str("1. **Address SOLID Violations**: Focus on fixing the SOLID principle violations, especially those related to Single Responsibility and Dependency Inversion.\n");
    }
    
    if !metrics.pattern_violations.is_empty() {
        summary.push_str("2. **Fix Pattern Violations**: Address the design pattern violations to improve code quality and maintainability.\n");
    }
    
    if metrics.pattern_implementations.is_empty() {
        summary.push_str("3. **Implement Design Patterns**: Consider implementing appropriate design patterns to improve code structure and maintainability.\n");
    }
    
    summary.push_str("\nFor detailed information, see the [full code quality report](./quality/code_quality_report.md).\n");
    
    summary
}
