use std::path::PathBuf;
use clap::Parser;
use lms_analyzer::{
    core::analyzer_config::AnalyzerConfig,
    runners::analysis_runner,
    generators::llm_insights_generator,
};

#[derive(Parser)]
#[command(author, version, about = "Generate LLM insights")]
struct Cli {
    /// Model to use
    #[arg(long, default_value = "qwen2.5")]
    model: String,
    
    /// API endpoint
    #[arg(long, default_value = "http://localhost:1234/v1/completions")]
    api_endpoint: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command-line arguments
    let cli = Cli::parse();
    
    println!("Generating LLM insights...");
    println!("Using model: {}", cli.model);
    println!("API endpoint: {}", cli.api_endpoint);
    
    // Run analysis
    let config = AnalyzerConfig::load(None)?;
    let result = analysis_runner::run_analysis(config).await?;
    
    // Generate LLM insights
    llm_insights_generator::generate_llm_insights_report(&result).await?;
    
    println!("LLM insights generated successfully.");
    
    Ok(())
}
