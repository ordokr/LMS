// Reexports for easier access
pub use crate::analyzers::unified_analyzer::AnalysisResult;
pub use crate::analyzers::unified_analyzer::UnifiedProjectAnalyzer;
pub use crate::analyzers::integrated_migration_analyzer::IntegratedMigrationAnalyzer;
pub use crate::analyzers::modules::canvas_analyzer::CanvasAnalyzer;
pub use crate::analyzers::modules::discourse_analyzer::DiscourseAnalyzer;

mod ast_analyzer;
mod project_structure;
pub mod unified_analyzer;
pub mod integrated_migration_analyzer;
mod unified_analyzer_extensions;
pub mod modules;