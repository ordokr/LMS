use std::path::PathBuf;
use clap::{Parser, Subcommand};
use unified_analyzer::analyzers::enhanced_ruby_analyzer::EnhancedRubyAnalyzer;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Analyze Canvas Ruby code
    Canvas {
        /// Path to Canvas source code
        #[arg(short, long)]
        path: String,
        
        /// Output file path (optional)
        #[arg(short, long)]
        output: Option<String>,
    },
    /// Analyze Discourse Ruby code
    Discourse {
        /// Path to Discourse source code
        #[arg(short, long)]
        path: String,
        
        /// Output file path (optional)
        #[arg(short, long)]
        output: Option<String>,
    },
    /// Analyze generic Ruby code
    Generic {
        /// Path to Ruby source code
        #[arg(short, long)]
        path: String,
        
        /// Output file path (optional)
        #[arg(short, long)]
        output: Option<String>,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Canvas { path, output } => {
            println!("Analyzing Canvas Ruby code at: {}", path);
            let mut analyzer = EnhancedRubyAnalyzer::new("Canvas");
            let result = analyzer.analyze_canvas(path)?;
            
            if let Some(output_path) = output {
                std::fs::write(output_path, result)?;
                println!("Analysis results written to: {}", output_path);
            } else {
                println!("{}", result);
            }
        },
        Commands::Discourse { path, output } => {
            println!("Analyzing Discourse Ruby code at: {}", path);
            let mut analyzer = EnhancedRubyAnalyzer::new("Discourse");
            let result = analyzer.analyze_discourse(path)?;
            
            if let Some(output_path) = output {
                std::fs::write(output_path, result)?;
                println!("Analysis results written to: {}", output_path);
            } else {
                println!("{}", result);
            }
        },
        Commands::Generic { path, output } => {
            println!("Analyzing generic Ruby code at: {}", path);
            let mut analyzer = EnhancedRubyAnalyzer::new("Generic");
            let result = analyzer.analyze(path)?;
            
            if let Some(output_path) = output {
                std::fs::write(output_path, result)?;
                println!("Analysis results written to: {}", output_path);
            } else {
                println!("{}", result);
            }
        },
    }

    Ok(())
}
