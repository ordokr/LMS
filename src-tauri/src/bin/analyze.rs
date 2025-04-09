use std::path::PathBuf;
use clap::{Parser, Subcommand};
use lms::analyzers::analysis_commands;

#[derive(Parser)]
#[command(name = "analyze")]
#[command(about = "LMS Project Analysis CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run a full project analysis
    Full {
        /// Target directories to analyze
        #[arg(short, long, num_args = 1..)]
        target_dirs: Option<Vec<String>>,
        
        /// Patterns to exclude from analysis
        #[arg(short, long, num_args = 1..)]
        exclude: Option<Vec<String>>,
        
        /// Output directory for analysis results
        #[arg(short, long)]
        output: Option<String>,
        
        /// Whether to update the RAG knowledge base
        #[arg(short, long, default_value_t = true)]
        update_rag: bool,
        
        /// Whether to generate AI insights
        #[arg(short, long, default_value_t = true)]
        generate_insights: bool,
    },
    
    /// Run a quick analysis
    Quick,
    
    /// Update the RAG knowledge base
    UpdateRag,
    
    /// Generate AI insights
    GenerateInsights,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command line arguments
    let cli = Cli::parse();
    
    // Execute the appropriate command
    match cli.command {
        Commands::Full {
            target_dirs,
            exclude,
            output,
            update_rag,
            generate_insights,
        } => {
            println!("Running full project analysis...");
            analysis_commands::cli_analyze_project(
                target_dirs,
                exclude,
                output,
                update_rag,
                generate_insights,
            ).await?;
        }
        
        Commands::Quick => {
            println!("Running quick project analysis...");
            // Call the quick analysis command
            let _ = analysis_commands::quick_analyze_project().await
                .map_err(|e| format!("Analysis failed: {}", e))?;
        }
        
        Commands::UpdateRag => {
            println!("Updating RAG knowledge base...");
            // Call the update RAG command
            let _ = analysis_commands::update_rag_knowledge_base().await
                .map_err(|e| format!("RAG update failed: {}", e))?;
        }
        
        Commands::GenerateInsights => {
            println!("Generating AI insights...");
            // Call the generate insights command
            let _ = analysis_commands::generate_ai_insights().await
                .map_err(|e| format!("AI insights generation failed: {}", e))?;
        }
    }
    
    Ok(())
}
