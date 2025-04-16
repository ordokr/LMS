mod analyzers;
mod config;
mod generators;
mod output_schema;
mod utils;

use crate::analyzers::modules::{{
    business_logic_analyzer::BusinessLogicAnalyzer, dependency_analyzer::DependencyAnalyzer,
    ember_analyzer::EmberAnalyzer, file_structure_analyzer::FileStructureAnalyzer,
    offline_first_readiness_analyzer::OfflineFirstReadinessAnalyzer, react_analyzer::ReactAnalyzer,
    route_analyzer::RouteAnalyzer, template_analyzer::TemplateAnalyzer,
}};
use anyhow::Result;
use config::Config;
use generators::*;
use log::info;
use std::fs::File;
use std::path::PathBuf;
use crate::{{analyzers::modules::*, generators::documentation_generator::generate_documentation, integrator::integrate_analysis_results, utils::file_system::FileSystemUtils}};

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

    run_analysis(&base_dir).await?;

async fn run_analysis(base_dir: &PathBuf) -> Result<()> {
    println!("---- Starting Unified Analysis ----");

    // Initialize and run FileStructureAnalyzer
    let file_structure_analyzer = FileStructureAnalyzer::new();
    let file_structure_result = file_structure_analyzer.analyze(&base_dir).expect("File structure analysis failed");

    // Initialize and run RubyRailsAnalyzer
    let ruby_rails_analyzer = RubyRailsAnalyzer::new();
    let ruby_rails_result = ruby_rails_analyzer.analyze(&base_dir).expect("Ruby on Rails analysis failed");

    // Initialize and run EmberAnalyzer
    let ember_analyzer = EmberAnalyzer::new();
    let ember_result = ember_analyzer.analyze(&base_dir).expect("Ember analysis failed");

    // Initialize and run ReactAnalyzer
    let react_analyzer = ReactAnalyzer::new();
    let react_result = react_analyzer.analyze(&base_dir).expect("React analysis failed");

    // Initialize and run TemplateAnalyzer
    let template_analyzer = TemplateAnalyzer::new();
    let template_result = template_analyzer.analyze(&base_dir).expect("Template analysis failed");

    // Initialize and run RouteAnalyzer
    let route_analyzer = RouteAnalyzer::new();
    let route_result = route_analyzer.analyze(&base_dir).expect("Route analysis failed");

    // Initialize and run ApiAnalyzer
    let api_analyzer = ApiAnalyzer::new();
    let api_result = api_analyzer.analyze(&base_dir).expect("API analysis failed");

    // Initialize and run DependencyAnalyzer
    let dependency_analyzer = DependencyAnalyzer::new();
    let dependency_result = dependency_analyzer.analyze(&base_dir).expect("Dependency analysis failed");

    // Initialize and run AuthFlowAnalyzer
    let auth_flow_analyzer = AuthFlowAnalyzer::new();
    let auth_flow_result = auth_flow_analyzer.analyze(&base_dir).expect("Authentication flow analysis failed");

    // Initialize and run OfflineFirstReadinessAnalyzer
    let offline_first_readiness_analyzer = OfflineFirstReadinessAnalyzer::new();
    let offline_first_readiness_result = offline_first_readiness_analyzer.analyze(&base_dir).expect("Offline-first readiness analysis failed");

    // Initialize and run DatabaseSchemaAnalyzer
    let database_schema_analyzer = DatabaseSchemaAnalyzer::new();
    let database_schema_result = database_schema_analyzer.analyze(&base_dir).expect("Database schema analysis failed");

    // Initialize and run BusinessLogicAnalyzer
    let business_logic_analyzer = BusinessLogicAnalyzer::new();
    let business_logic_result = business_logic_analyzer.analyze(&base_dir).expect("Business logic analysis failed");

    // Integrate analysis results
    let unified_output = integrate_analysis_results(
        file_structure_result,
        ruby_rails_result,
        ember_result,
        react_result,
        template_result,
        route_result,
        api_result,
        dependency_result,
        auth_flow_result,
        offline_first_readiness_result,
        database_schema_result,
        business_logic_result,
    );

    // Write the unified output to a JSON file
    let output_path = base_dir.join("unified_output.json");
    let file = File::create(output_path).expect("Failed to create output file");
    serde_json::to_writer_pretty(file, &unified_output).expect("Failed to write unified output");

    generate_documentation(&unified_output, base_dir)?;
    println!("Documentation generation completed.");


    println!("Unified analysis completed and output written to unified_output.json");

    // Generate documentation based on configuration
    // if config.documentation.generate_high_priority {
    //     // Generate high priority documentation
    //     println!("Generating high priority documentation...");
    //
    //     if config.documentation.high_priority.central_reference_hub {
    //         println!("Generating enhanced central reference hub...");
    //         if let Err(e) = enhanced_central_hub_generator::generate_enhanced_central_hub(&result, &base_dir) {
    //             return Err(anyhow::anyhow!("Failed to generate enhanced central reference hub: {}", e));
    //         }
    //     }
    //
    //     if let Err(e) = analyzer.generate_analyzer_reference().await {
    //         return Err(anyhow::anyhow!("Failed to generate analyzer reference: {}", e));
    //     }
    //
    // }

        Ok(())
}
