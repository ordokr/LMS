use std::path::PathBuf;
use clap::{Parser, Subcommand};
use lms_analyzer::{
    core::analyzer_config::AnalyzerConfig,
    runners::analysis_runner,
    generators::report_generator,
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run a full project analysis
    Full {
        /// Target directories to analyze (comma separated)
        #[arg(short, long, value_delimiter = ',')]
        target_dirs: Option<Vec<String>>,

        /// Patterns to exclude (comma separated)
        #[arg(short, long, value_delimiter = ',')]
        exclude: Option<Vec<String>>,

        /// Output directory for documentation
        #[arg(short, long)]
        output: Option<String>,

        /// Update RAG knowledge base
        #[arg(long)]
        update_rag: bool,

        /// Generate AI insights
        #[arg(long)]
        generate_insights: bool,

        /// Analyze JavaScript files for Rust migration
        #[arg(long)]
        analyze_js: bool,

        /// Generate visual dashboard
        #[arg(long)]
        dashboard: bool,

        /// Analyze technical debt
        #[arg(long)]
        tech_debt: bool,

        /// Analyze code quality
        #[arg(long)]
        code_quality: bool,

        /// Analyze data models
        #[arg(long)]
        models: bool,
    },

    /// Run a quick project analysis
    Quick,

    /// Update the central reference hub
    UpdateHub,

    /// Generate a summary report
    Summary,

    /// Update the RAG knowledge base
    UpdateRag,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command-line arguments
    let cli = Cli::parse();

    // Execute the appropriate command
    match cli.command {
        Commands::Full {
            target_dirs,
            exclude,
            output,
            update_rag,
            generate_insights,
            analyze_js,
            dashboard,
            tech_debt,
            code_quality,
            models,
        } => {
            println!("Running full project analysis...");

            // Run analysis
            let result = analysis_runner::run_analysis_with_args(
                target_dirs,
                exclude,
                output,
                update_rag,
                generate_insights,
                analyze_js,
                dashboard,
                tech_debt,
                code_quality,
                models,
            ).await?;

            // Generate reports
            report_generator::generate_reports(&result)?;

            println!("Analysis complete. Reports generated.");
        }

        Commands::Quick => {
            println!("Running quick project analysis...");

            // Run quick analysis
            let result = analysis_runner::run_quick_analysis().await?;

            // Generate summary report
            report_generator::generate_summary_report(&result)?;

            println!("Quick analysis complete. Summary report generated.");
        }

        Commands::UpdateHub => {
            println!("Updating central reference hub...");

            // Run quick analysis
            let result = analysis_runner::run_quick_analysis().await?;

            // Generate central reference hub
            lms_analyzer::generators::central_hub_generator::generate_central_reference_hub(&result)?;

            println!("Central reference hub updated.");
        }

        Commands::Summary => {
            println!("Generating summary report...");

            // Run quick analysis
            let result = analysis_runner::run_quick_analysis().await?;

            // Generate summary report
            report_generator::generate_summary_report(&result)?;

            println!("Summary report generated.");
        }

        Commands::UpdateRag => {
            println!("Updating RAG knowledge base...");

            // Load configuration
            let mut config = AnalyzerConfig::load(None)?;
            config.update_rag_knowledge_base = true;

            // Run analysis
            let result = analysis_runner::run_analysis(config).await?;

            println!("RAG knowledge base updated.");
        }
    }

    Ok(())
}
