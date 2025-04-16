// Reexports for easier access
mod ast_analyzer;
mod project_structure;
pub mod unified_analyzer;
pub mod integrated_migration_analyzer;
mod unified_analyzer_extensions;
pub mod modules;

// Re-export AnalysisResult and UnifiedProjectAnalyzer for external use
pub use unified_analyzer::{AnalysisResult, UnifiedProjectAnalyzer};
pub use integrated_migration_analyzer::IntegratedMigrationAnalyzer;

// Import the modules that will house the analyzers
use modules::file_structure_analyzer::FileStructureAnalyzer;
use modules::ruby_rails_analyzer::RubyRailsAnalyzer;
use modules::ember_analyzer::EmberAnalyzer;
use modules::react_analyzer::ReactAnalyzer;
use modules::template_analyzer::TemplateAnalyzer;
use modules::route_analyzer::RouteAnalyzer;
use modules::api_analyzer::ApiAnalyzer;
use modules::dependency_analyzer::DependencyAnalyzer;
use modules::auth_flow_analyzer::AuthFlowAnalyzer;
use modules::offline_first_readiness_analyzer::OfflineFirstReadinessAnalyzer;
use modules::database_schema_analyzer::DatabaseSchemaAnalyzer;
use modules::business_logic_analyzer::BusinessLogicAnalyzer;
use modules::canvas_analyzer::CanvasAnalyzer;
use modules::discourse_analyzer::DiscourseAnalyzer;

// Re-export individual analyzers for use in UnifiedProjectAnalyzer
pub use modules::file_structure_analyzer::FileStructureAnalyzer as ExposedFileStructureAnalyzer;
pub use modules::ruby_rails_analyzer::RubyRailsAnalyzer as ExposedRubyRailsAnalyzer;
pub use modules::ember_analyzer::EmberAnalyzer as ExposedEmberAnalyzer;
pub use modules::react_analyzer::ReactAnalyzer as ExposedReactAnalyzer;
pub use modules::template_analyzer::TemplateAnalyzer as ExposedTemplateAnalyzer;
pub use modules::route_analyzer::RouteAnalyzer as ExposedRouteAnalyzer;
pub use modules::api_analyzer::ApiAnalyzer as ExposedApiAnalyzer;
pub use modules::dependency_analyzer::DependencyAnalyzer as ExposedDependencyAnalyzer;
pub use modules::auth_flow_analyzer::AuthFlowAnalyzer as ExposedAuthFlowAnalyzer;
pub use modules::offline_first_readiness_analyzer::OfflineFirstReadinessAnalyzer as ExposedOfflineFirstReadinessAnalyzer;
pub use modules::database_schema_analyzer::DatabaseSchemaAnalyzer as ExposedDatabaseSchemaAnalyzer;
pub use modules::business_logic_analyzer::BusinessLogicAnalyzer as ExposedBusinessLogicAnalyzer;
pub use modules::canvas_analyzer::CanvasAnalyzer as ExposedCanvasAnalyzer;
pub use modules::discourse_analyzer::DiscourseAnalyzer as ExposedDiscourseAnalyzer;

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
    let dependency_analyzer = DependencyAnalyzer::new();
    let auth_flow_analyzer = AuthFlowAnalyzer::new();
    let offline_first_readiness_analyzer = OfflineFirstReadinessAnalyzer::new();
    let database_schema_analyzer = DatabaseSchemaAnalyzer::new();
    let business_logic_analyzer = BusinessLogicAnalyzer::new();
    let canvas_analyzer = CanvasAnalyzer::new();
    let discourse_analyzer = DiscourseAnalyzer::new();

    vec![
        file_structure_analyzer.analyze(project_path).map_err(|e| e.to_string()),
        ruby_rails_analyzer.analyze(project_path).map_err(|e| e.to_string()),
        ember_analyzer.analyze(&path_buf),
        react_analyzer.analyze(&path_buf),
        template_analyzer.analyze(&path_buf),
        route_analyzer.analyze(&path_buf),
        api_analyzer.analyze(&path_buf),
        dependency_analyzer.analyze(&path_buf),
        auth_flow_analyzer.analyze(&path_buf),
        offline_first_readiness_analyzer.analyze(&path_buf),
        database_schema_analyzer.analyze(&path_buf),
        business_logic_analyzer.analyze(&path_buf),
        canvas_analyzer.analyze(project_path).map_err(|e| e.to_string()),
        discourse_analyzer.analyze(project_path).map_err(|e| e.to_string()),
    ]
}