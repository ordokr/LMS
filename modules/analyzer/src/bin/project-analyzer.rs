use std::error::Error;
use clap::{Parser, Subcommand};
use lms_analyzer::{AnalyzerConfig, run_project_analyzer};

#[derive(Parser)]
#[command(name = "project-analyzer")]
#[command(about = "LMS Project Analyzer", long_about = None)]
struct Cli {
    /// Configuration file path
    #[arg(short, long)]
    config: Option<String>,
    
    /// Target directories to analyze
    #[arg(short, long, value_delimiter = ',')]
    dirs: Option<Vec<String>>,
    
    /// Patterns to exclude (comma separated)
    #[arg(short, long, value_delimiter = ',')]
    exclude: Option<Vec<String>>,
    
    /// Output directory for documentation
    #[arg(short, long)]
    output: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    
    println!("Running Project Analyzer...");
    
    // Run the project analyzer
    run_project_analyzer(cli.config.as_deref()).await?;
    
    println!("Project Analyzer completed successfully.");
    Ok(())
}
