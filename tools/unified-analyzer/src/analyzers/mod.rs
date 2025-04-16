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

// Function to run all analyzers sequentially
pub fn run_all_analyzers(
    project_path: &str,
) -> Vec<Result<String, String>> {
    vec![
        FileStructureAnalyzer::analyze(project_path),
        RubyRailsAnalyzer::analyze(project_path),
        EmberAnalyzer::analyze(project_path),
        ReactAnalyzer::analyze(project_path),
        TemplateAnalyzer::analyze(project_path),
        RouteAnalyzer::analyze(project_path),
        ApiAnalyzer::analyze(project_path),
        DependencyAnalyzer::analyze(project_path),
        AuthFlowAnalyzer::analyze(project_path),
        OfflineFirstReadinessAnalyzer::analyze(project_path),
        DatabaseSchemaAnalyzer::analyze(project_path),
        BusinessLogicAnalyzer::analyze(project_path),
    ]
}