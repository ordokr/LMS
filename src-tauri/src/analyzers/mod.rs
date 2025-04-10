pub mod ast_analyzer;
pub mod project_structure;
pub mod db_schema_analyzer;
pub mod blockchain_analyzer;
pub mod unified_analyzer;
pub mod analysis_runner;
pub mod analysis_commands;
pub mod docs_updater;
pub mod js_migration_analyzer;
pub mod dashboard_generator;
pub mod tech_debt_analyzer;

// Re-export structs for easier access
pub use unified_analyzer::AnalysisResult;
pub use analysis_runner::AnalysisRunner;
pub use docs_updater::DocsUpdater;
pub use js_migration_analyzer::JsMigrationAnalyzer;
pub use dashboard_generator::DashboardGenerator;
pub use tech_debt_analyzer::TechDebtAnalyzer;
pub use analysis_commands::{
    analyze_project,
    quick_analyze_project,
    update_rag_knowledge_base,
    generate_ai_insights,
    cli_analyze_project,
    analyze_js_files
};
