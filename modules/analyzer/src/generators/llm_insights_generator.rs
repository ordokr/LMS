use std::fs;
use std::path::{Path, PathBuf};
use chrono::Local;

use crate::core::analysis_result::AnalysisResult;
use crate::core::llm_integration::LlmIntegration;

/// Generate LLM insights report
pub async fn generate_llm_insights_report(result: &AnalysisResult) -> Result<(), String> {
    println!("Generating LLM insights report...");
    
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
    
    // Create the LLM integration
    let llm = LlmIntegration::new(Path::new(".").to_path_buf());
    
    // Check if LM Studio is running
    if !llm.check_lm_studio_running() {
        // Try to start LM Studio
        llm.start_lm_studio()?;
        
        // Check again
        if !llm.check_lm_studio_running() {
            return Err("LM Studio is not running. Please start it manually.".to_string());
        }
    }
    
    // Generate insights
    let insights = llm.generate_insights(result).await?;
    
    // Create the report path
    let report_path = insights_dir.join("llm_insights_report.md");
    
    // Add header to the insights
    let mut report = String::new();
    report.push_str("# LLM Insights Report\n\n");
    report.push_str(&format!("_Generated on: {} using LM Studio with Qwen 2.5_\n\n", Local::now().format("%Y-%m-%d")));
    report.push_str(&insights);
    
    // Write to file
    fs::write(&report_path, report)
        .map_err(|e| format!("Failed to write LLM insights report: {}", e))?;
    
    println!("LLM insights report generated at: {:?}", report_path);
    
    Ok(())
}
