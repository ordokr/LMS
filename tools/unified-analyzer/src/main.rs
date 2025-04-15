mod analyzers;
mod utils;
mod generators;
mod config;

use std::path::PathBuf;
use std::sync::Arc;
use anyhow::Result;

use analyzers::unified_analyzer::UnifiedProjectAnalyzer;
use utils::file_system::FileSystemUtils;
use generators::*;
use config::Config;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Unified Analyzer for LMS Project");

    // Load configuration
    let config = match Config::from_file("config.toml") {
        Ok(config) => {
            println!("Loaded configuration from config.toml");
            config
        },
        Err(e) => {
            println!("Failed to load configuration: {}", e);
            println!("Using default configuration");
            Config::default()
        }
    };

    // Get the current directory or a specified directory
    let args: Vec<String> = std::env::args().collect();
    let base_dir = if args.len() > 1 {
        PathBuf::from(&args[1])
    } else {
        std::env::current_dir()?
    };

    println!("Analyzing project at: {}", base_dir.display());

    // Initialize the file system utilities
    let fs_utils = Arc::new(FileSystemUtils::new());

    // Create the unified analyzer
    let analyzer = UnifiedProjectAnalyzer::new(base_dir.clone(), fs_utils);

    // Run the analysis
    let result = match analyzer.analyze().await {
        Ok(result) => result,
        Err(e) => return Err(anyhow::anyhow!("Analysis failed: {}", e)),
    };

    // Generate documentation based on configuration
    if config.documentation.generate_high_priority {
        // Generate high priority documentation
        println!("Generating high priority documentation...");

        if config.documentation.high_priority.central_reference_hub {
            println!("Generating enhanced central reference hub...");
            if let Err(e) = enhanced_central_hub_generator::generate_enhanced_central_hub(&result, &base_dir) {
                return Err(anyhow::anyhow!("Failed to generate enhanced central reference hub: {}", e));
            }
        }

        if let Err(e) = analyzer.generate_analyzer_reference().await {
            return Err(anyhow::anyhow!("Failed to generate analyzer reference: {}", e));
        }

        if config.documentation.high_priority.api_documentation {
            if let Err(e) = api_doc_generator::generate_api_doc(&result, &base_dir) {
                return Err(anyhow::anyhow!("Failed to generate API documentation: {}", e));
            }
        }

        if config.documentation.high_priority.implementation_details {
            if let Err(e) = implementation_details_generator::generate_implementation_details(&result, &base_dir) {
                return Err(anyhow::anyhow!("Failed to generate implementation details: {}", e));
            }
        }

        if config.documentation.high_priority.testing_documentation {
            if let Err(e) = testing_doc_generator::generate_testing_doc(&result, &base_dir) {
                return Err(anyhow::anyhow!("Failed to generate testing documentation: {}", e));
            }
        }

        if config.documentation.high_priority.technical_debt_report {
            if let Err(e) = tech_debt_report_generator::generate_tech_debt_report(&result, &base_dir) {
                return Err(anyhow::anyhow!("Failed to generate technical debt report: {}", e));
            }
        }

        if config.documentation.high_priority.summary_report {
            if let Err(e) = summary_report_generator::generate_summary_report(&result, &base_dir) {
                return Err(anyhow::anyhow!("Failed to generate summary report: {}", e));
            }
        }
    }

    if config.documentation.generate_medium_priority {
        // Generate medium priority documentation
        println!("Generating medium priority documentation...");

        if config.documentation.medium_priority.synchronization_architecture {
            if let Err(e) = sync_architecture_generator::generate_sync_architecture(&result, &base_dir) {
                return Err(anyhow::anyhow!("Failed to generate synchronization architecture: {}", e));
            }
        }

        if config.documentation.medium_priority.database_architecture {
            if let Err(e) = database_architecture_generator::generate_database_architecture(&result, &base_dir) {
                return Err(anyhow::anyhow!("Failed to generate database architecture: {}", e));
            }
        }
    }

    // Generate metrics visualizations and project dashboard
    if let Err(e) = analyzer.generate_metrics_visualizations().await {
        return Err(anyhow::anyhow!("Failed to generate metrics visualizations: {}", e));
    }

    if let Err(e) = analyzer.generate_project_dashboard().await {
        return Err(anyhow::anyhow!("Failed to generate project dashboard: {}", e));
    }

    println!("Analysis completed successfully!");
    println!("Timestamp: {}", result.timestamp);
    println!("Project status: {}", result.project_status.phase);
    println!("Completion percentage: {}%", result.project_status.completion_percentage);

    Ok(())
}
