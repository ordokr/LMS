use std::error::Error;
use lms_analyzer::{AnalyzerConfig, run_project_analyzer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Testing Project Analyzer Integration");
    
    // Run the project analyzer with default configuration
    run_project_analyzer(None).await?;
    
    println!("Test completed successfully!");
    Ok(())
}
