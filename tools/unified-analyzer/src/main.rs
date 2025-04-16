mod analyzers;
mod config;
mod generators;
mod utils;

use analyzers::unified_analyzer::UnifiedProjectAnalyzer;
use std::path::PathBuf;
use std::sync::Arc;

use crate::analyzers::modules::{file_structure_analyzer, file_structure_analyzer::FileStructureAnalyzer};
use anyhow::Result;
use config::Config;
use generators::*;
use log::info;

use crate::analyzers::modules::file_structure_analyzer::FileStructureAnalyzer;
use crate::analyzers::modules::ruby_rails_analyzer::RubyRailsAnalyzer;
use crate::analyzers::modules::ember_analyzer::EmberAnalyzer;
use crate::analyzers::modules::react_analyzer::ReactAnalyzer;
use crate::analyzers::modules::template_analyzer::TemplateAnalyzer;
use crate::analyzers::modules::route_analyzer::RouteAnalyzer;
use crate::analyzers::modules::api_analyzer::ApiAnalyzer;
use crate::analyzers::modules::dependency_analyzer::DependencyAnalyzer;
use crate::analyzers::modules::auth_flow_analyzer::AuthFlowAnalyzer;
use crate::analyzers::modules::offline_first_readiness_analyzer::OfflineFirstReadinessAnalyzer;
use crate::analyzers::modules::database_schema_analyzer::DatabaseSchemaAnalyzer;
use crate::analyzers::modules::business_logic_analyzer::BusinessLogicAnalyzer;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
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

    info!("Analyzing project at: {}", base_dir.display());

    // Initialize the file system utilities

    // Call File Structure Analyzer
    let file_structure_analyzer = FileStructureAnalyzer::new();
    match file_structure_analyzer.analyze("C:UsersTimDesktopPort") {
        Ok(_) => println!("File Structure Analysis completed successfully!"),
        Err(e) => println!("File Structure Analysis failed: {:?}", e),
    }

    let fs_utils = Arc::new(FileSystemUtils::new());

    file_structure_analyzer::FileStructureAnalyzer::analyze("C:UsersTimDesktopPort").expect("File structure analysis failed");

    println!("---- Starting Unified Analysis ----");
    // Call Unified Analyzer
    println!("---- Starting Unified Analysis ----");
    // Create the unified analyzer
    let analyzer = UnifiedProjectAnalyzer::new(base_dir.clone(), fs_utils);

        // Initialize and run FileStructureAnalyzer
    let file_structure_analyzer = FileStructureAnalyzer::new();
    file_structure_analyzer.analyze("C:UsersTimDesktopPort").expect("File structure analysis failed");

    // Initialize and run RubyRailsAnalyzer
    let ruby_rails_analyzer = RubyRailsAnalyzer::new();
    ruby_rails_analyzer.analyze(&base_dir).expect("Ruby on Rails analysis failed");

    // Initialize and run EmberAnalyzer
    let ember_analyzer = EmberAnalyzer::new();
    ember_analyzer.analyze(&base_dir).expect("Ember analysis failed");

    // Initialize and run ReactAnalyzer
    let react_analyzer = ReactAnalyzer::new();
    react_analyzer.analyze(&base_dir).expect("React analysis failed");

    // Initialize and run TemplateAnalyzer
    let template_analyzer = TemplateAnalyzer::new();
    template_analyzer.analyze(&base_dir).expect("Template analysis failed");

    // Initialize and run RouteAnalyzer
    let route_analyzer = RouteAnalyzer::new();
    route_analyzer.analyze(&base_dir).expect("Route analysis failed");

    // Initialize and run ApiAnalyzer
    let api_analyzer = ApiAnalyzer::new();
    api_analyzer.analyze(&base_dir).expect("API analysis failed");

    // Initialize and run DependencyAnalyzer
    let dependency_analyzer = DependencyAnalyzer::new();
    dependency_analyzer.analyze(&base_dir).expect("Dependency analysis failed");

    // Initialize and run AuthFlowAnalyzer
    let auth_flow_analyzer = AuthFlowAnalyzer::new();
    auth_flow_analyzer.analyze(&base_dir).expect("Authentication flow analysis failed");

    // Initialize and run OfflineFirstReadinessAnalyzer
    let offline_first_readiness_analyzer = OfflineFirstReadinessAnalyzer::new();
    offline_first_readiness_analyzer.analyze(&base_dir).expect("Offline-first readiness analysis failed");

    // Initialize and run DatabaseSchemaAnalyzer
    let database_schema_analyzer = DatabaseSchemaAnalyzer::new();
    database_schema_analyzer.analyze(&base_dir).expect("Database schema analysis failed");

    // Initialize and run BusinessLogicAnalyzer
    let business_logic_analyzer = BusinessLogicAnalyzer::new();
    business_logic_analyzer.analyze(&base_dir).expect("Business logic analysis failed");

    // Run the unified analysis
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
