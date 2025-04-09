pub mod ast_analyzer;
pub mod project_structure;
pub mod db_schema_analyzer;
pub mod blockchain_analyzer;
pub mod unified_analyzer;
pub mod analysis_runner;
pub mod analysis_commands;

// Re-export structs for easier access
pub use unified_analyzer::AnalysisResult;
pub use analysis_runner::AnalysisRunner;
pub use analysis_commands::{
    analyze_project,
    quick_analyze_project,
    update_rag_knowledge_base,
    generate_ai_insights,
    cli_analyze_project
};
