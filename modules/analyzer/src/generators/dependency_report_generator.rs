use std::fs;
use std::path::{Path, PathBuf};
use chrono::Local;
use std::collections::HashMap;

use crate::core::analysis_result::AnalysisResult;
use crate::core::dependency_analyzer::{DependencyAnalyzer, DependencyMetrics};

/// Generate dependency report
pub fn generate_dependency_report(result: &AnalysisResult) -> Result<(), String> {
    println!("Generating dependency report...");
    
    // Ensure docs directory exists
    let docs_dir = Path::new("docs");
    if !docs_dir.exists() {
        fs::create_dir_all(docs_dir)
            .map_err(|e| format!("Failed to create docs directory: {}", e))?;
    }
    
    // Ensure dependencies directory exists
    let deps_dir = docs_dir.join("dependencies");
    if !deps_dir.exists() {
        fs::create_dir_all(&deps_dir)
            .map_err(|e| format!("Failed to create dependencies directory: {}", e))?;
    }
    
    // Create the report path
    let report_path = deps_dir.join("dependency_report.md");
    
    // Create the analyzer
    let analyzer = DependencyAnalyzer::new(Path::new(".").to_path_buf());
    
    // Generate the report
    let report = analyzer.generate_report()?;
    
    // Write to file
    fs::write(&report_path, report)
        .map_err(|e| format!("Failed to write dependency report: {}", e))?;
    
    println!("Dependency report generated at: {:?}", report_path);
    
    // Create a summary report
    let summary_path = docs_dir.join("dependency_summary.md");
    
    // Analyze the codebase
    let metrics = analyzer.analyze_codebase()?;
    
    // Generate the summary
    let summary = generate_summary(&metrics);
    
    // Write to file
    fs::write(&summary_path, summary)
        .map_err(|e| format!("Failed to write dependency summary: {}", e))?;
    
    println!("Dependency summary generated at: {:?}", summary_path);
    
    Ok(())
}

/// Generate a summary of the dependency metrics
fn generate_summary(metrics: &DependencyMetrics) -> String {
    let mut summary = String::new();
    
    // Header
    summary.push_str("# Dependency Summary\n\n");
    summary.push_str(&format!("_Generated on: {}_\n\n", Local::now().format("%Y-%m-%d")));
    
    // Dependency Counts
    summary.push_str("## Dependency Counts\n\n");
    summary.push_str(&format!("**Total Dependencies: {}**\n\n", metrics.total_dependencies));
    summary.push_str("| Type | Count |\n");
    summary.push_str("|------|-------|\n");
    summary.push_str(&format!("| Direct | {} |\n", metrics.direct_dependencies));
    summary.push_str(&format!("| Transitive | {} |\n\n", metrics.transitive_dependencies));
    
    // Files with Dependencies
    summary.push_str("## Files with Dependencies\n\n");
    summary.push_str("| File | Dependencies |\n");
    summary.push_str("|------|-------------|\n");
    
    for (file, deps) in &metrics.dependencies_by_file {
        summary.push_str(&format!("| {} | {} |\n", file, deps.len()));
    }
    summary.push_str("\n");
    
    // Outdated Dependencies
    summary.push_str("## Outdated Dependencies\n\n");
    
    if !metrics.outdated_dependencies.is_empty() {
        summary.push_str("| Name | Current Version | Latest Version |\n");
        summary.push_str("|------|----------------|----------------|\n");
        
        for (dep, latest_version) in &metrics.outdated_dependencies {
            summary.push_str(&format!("| {} | {} | {} |\n",
                dep.name,
                dep.version,
                latest_version));
        }
    } else {
        summary.push_str("No outdated dependencies found.\n");
    }
    summary.push_str("\n");
    
    // Recommendations
    summary.push_str("## Recommendations\n\n");
    
    if !metrics.outdated_dependencies.is_empty() {
        summary.push_str("1. **Update Outdated Dependencies**: Consider updating the outdated dependencies to their latest versions.\n");
    }
    
    if metrics.direct_dependencies > 20 {
        summary.push_str("2. **Reduce Dependencies**: Consider reducing the number of direct dependencies to improve maintainability.\n");
    }
    
    summary.push_str("\nFor detailed information, see the [full dependency report](./dependencies/dependency_report.md).\n");
    
    summary
}
