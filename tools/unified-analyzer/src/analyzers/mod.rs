// Reexports for easier access
mod ast_analyzer;
mod project_structure;
#[cfg(test)]
mod project_structure_tests;
pub mod unified_analyzer;
pub mod integrated_migration_analyzer;
#[cfg(test)]
mod integrated_migration_analyzer_tests;
mod unified_analyzer_extensions;
pub mod modules;
pub mod codebase_scanner;
pub mod simple_source_db_analyzer;
pub mod rust_source_db_analyzer;
pub mod enhanced_ruby_analyzer;
mod ast_analyzer_runner;
mod project_structure_runner;
mod integrated_migration_runner;
mod unified_analyzer_runner;

// Export the analyzer runners
pub use ast_analyzer_runner::run_ast_analyzer;
pub use project_structure_runner::run_project_structure_analyzer;
// These are commented out to avoid unused import warnings
// pub use integrated_migration_runner::run_integrated_migration_analyzer;
// pub use unified_analyzer_runner::run_unified_project_analyzer;

// Re-export AnalysisResult for external use
// This is used in all_generators.rs
// pub use unified_analyzer::AnalysisResult;

// Import the modules that will house the analyzers
use modules::file_structure_analyzer::FileStructureAnalyzer;
use modules::ruby_rails_analyzer::RubyRailsAnalyzer;
use modules::ember_analyzer::EmberAnalyzer;
use modules::react_analyzer::ReactAnalyzer;
use modules::template_analyzer::TemplateAnalyzer;
use modules::route_analyzer::RouteAnalyzer;
use modules::enhanced_api_analyzer::ApiAnalyzer as EnhancedApiAnalyzer;
use modules::api_analyzer::ApiAnalyzer;
use modules::dependency_analyzer::DependencyAnalyzer;
use modules::auth_flow_analyzer::AuthFlowAnalyzer;
use modules::offline_first_readiness_analyzer::OfflineFirstReadinessAnalyzer;
use modules::database_schema_analyzer::DatabaseSchemaAnalyzer;
use modules::business_logic_analyzer::BusinessLogicAnalyzer;
use modules::haskell_analyzer::HaskellAnalyzer;
use modules::canvas_analyzer::CanvasAnalyzer;
use modules::discourse_analyzer::DiscourseAnalyzer;
use enhanced_ruby_analyzer::EnhancedRubyAnalyzer;

// These exports are used in main.rs

// Function to run all analyzers sequentially
pub fn run_all_analyzers(
    project_path: &str,
) -> Vec<Result<String, String>> {
    let path_buf = std::path::PathBuf::from(project_path);

    let file_structure_analyzer = FileStructureAnalyzer::default();
    let ruby_rails_analyzer = RubyRailsAnalyzer::new();
    let ember_analyzer = EmberAnalyzer::new();
    let react_analyzer = ReactAnalyzer::new();
    let template_analyzer = TemplateAnalyzer::new();
    let route_analyzer = RouteAnalyzer::new();
    let api_analyzer = ApiAnalyzer::new();
    let enhanced_api_analyzer = EnhancedApiAnalyzer::new();
    let dependency_analyzer = DependencyAnalyzer::new();
    let auth_flow_analyzer = AuthFlowAnalyzer::new();
    let offline_first_readiness_analyzer = OfflineFirstReadinessAnalyzer::new();
    let database_schema_analyzer = DatabaseSchemaAnalyzer::new();
    let business_logic_analyzer = BusinessLogicAnalyzer::new();
    let canvas_analyzer = CanvasAnalyzer::new();
    let discourse_analyzer = DiscourseAnalyzer::new();
    let haskell_analyzer = HaskellAnalyzer::new();
    let mut enhanced_ruby_analyzer = EnhancedRubyAnalyzer::new("Generic");

    vec![
        file_structure_analyzer.analyze(project_path).map_err(|e| e.to_string()),
        ruby_rails_analyzer.analyze(project_path).map_err(|e| e.to_string()),
        ember_analyzer.analyze(&path_buf),
        react_analyzer.analyze(&path_buf),
        template_analyzer.analyze(&path_buf),
        route_analyzer.analyze(&path_buf),
        api_analyzer.analyze(&path_buf),
        enhanced_api_analyzer.analyze(&path_buf),
        dependency_analyzer.analyze(&path_buf),
        auth_flow_analyzer.analyze(&path_buf),
        offline_first_readiness_analyzer.analyze(&path_buf),
        database_schema_analyzer.analyze(&path_buf),
        business_logic_analyzer.analyze(&path_buf),
        haskell_analyzer.analyze(&path_buf),
        canvas_analyzer.analyze(project_path).map_err(|e| e.to_string()),
        discourse_analyzer.analyze(project_path).map_err(|e| e.to_string()),
        enhanced_ruby_analyzer.analyze(project_path).map_err(|e| e.to_string()),
    ]
}