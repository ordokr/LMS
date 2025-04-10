use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Instant;
use std::error::Error;
use clap::{Parser, Subcommand};
use lms::utils::file_system::FileSystemUtils;
use lms::analyzers::project_structure::ProjectStructure;
use lms::analyzers::ast_analyzer::{AstAnalyzer, CodeMetrics};
use lms::analyzers::analysis_runner::{AnalysisRunner, AnalysisCommand};
use lms::analyzers::docs_updater::DocsUpdater;
use lms::analyzers::js_migration_analyzer::JsMigrationAnalyzer;
use lms::ai::gemini_analyzer::{GeminiAnalyzer, CodeInsight};
use tokio;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Run quick analysis (minimal output)
    #[arg(long)]
    quick: bool,

    /// Update RAG knowledge base
    #[arg(long = "update-rag")]
    update_rag: bool,
    
    /// Generate AI insights
    #[arg(long = "generate-ai")]
    generate_ai: bool,
    
    /// Analyze JavaScript files for Rust migration
    #[arg(long = "analyze-js")]
    analyze_js: bool,
    
    /// Target directories to analyze (defaults to current directory)
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
    let start_time = Instant::now();
    
    // Parse command line arguments
    let cli = Cli::parse();
    
    // Create the analysis command
    let mut command = AnalysisCommand::default();
    
    // Set target directories
    if let Some(dirs) = cli.dirs {
        command.target_dirs = dirs.into_iter()
            .map(|d| PathBuf::from(d))
            .collect();
    }
    
    // Set exclude patterns
    if let Some(patterns) = cli.exclude {
        command.exclude_patterns = patterns;
    }
    
    // Set output directory
    if let Some(output) = cli.output {
        command.output_dir = PathBuf::from(output);
    }
    
    // Set analysis options
    command.update_rag_knowledge_base = cli.update_rag;
    command.generate_ai_insights = cli.generate_ai;
    command.analyze_js_files = cli.analyze_js;
    
    // If quick mode, disable rag and ai insights
    if cli.quick {
        command.update_rag_knowledge_base = false;
        command.generate_ai_insights = false;
        command.analyze_js_files = false;
    }
    
    // Create an analysis runner
    let current_dir = std::env::current_dir()?;
    let mut runner = AnalysisRunner::new(current_dir.clone());
    
    // Setup Gemini analyzer if needed
    if command.generate_ai_insights {
        // Get API key from environment variable or a config file
        let api_key = std::env::var("GEMINI_API_KEY")
            .unwrap_or_else(|_| "".to_string());
        
        if !api_key.is_empty() {
            let gemini = GeminiAnalyzer::new(api_key);
            runner = runner.with_gemini(gemini);
        } else {
            println!("Warning: GEMINI_API_KEY not set, AI insights will not be generated.");
            command.generate_ai_insights = false;
        }
    }
    
    // Run the analysis
    println!("Starting project analysis...");
    let result = runner.run_analysis(&command).await?;
    
    // Print summary
    let elapsed = start_time.elapsed();
    println!("Analysis completed in {:.2?}", elapsed);
    
    // Display key statistics
    println!("\nProject Statistics:");
    println!("-------------------");
    println!("Models: {}/{} implemented ({:.1}%)", 
        result.models.implemented,
        result.models.total,
        result.models.implementation_percentage);
        
    println!("API Endpoints: {}/{} implemented ({:.1}%)", 
        result.api_endpoints.implemented,
        result.api_endpoints.total,
        result.api_endpoints.implementation_percentage);
        
    println!("UI Components: {}/{} implemented ({:.1}%)", 
        result.ui_components.implemented,
        result.ui_components.total,
        result.ui_components.implementation_percentage);
    
    // Display information about generated files
    println!("\nGenerated Files:");
    println!("----------------");
    println!("Central Reference Hub: docs/central_reference_hub.md");
    
    if command.update_rag_knowledge_base {
        println!("RAG Knowledge Base: rag_knowledge_base/");
        println!("AI Agent Documentation: docs/LAST_ANALYSIS_RESULTS.md");
    }
    
    if command.analyze_js_files {
        println!("JS Migration Plan: docs/js_to_rust_migration_plan.md");
        println!("Rust Templates: tools/rust_templates/");
    }
    
    if command.generate_ai_insights {
        println!("AI Insights: docs/ai_code_insights.md");
    }
    
    Ok(())
}
