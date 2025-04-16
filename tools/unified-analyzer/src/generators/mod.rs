// Error handling
pub mod error;

// High Priority Generators
pub mod api_doc_generator;
pub mod implementation_details_generator;
pub mod testing_doc_generator;
pub mod tech_debt_report_generator;
pub mod summary_report_generator;
pub mod enhanced_central_hub_generator;
pub mod migration_roadmap_generator;
pub mod component_tree_generator;
pub mod api_map_generator;

pub mod documentation_generator;
// Medium Priority Generators
pub mod sync_architecture_generator;
pub mod database_architecture_generator;

// Re-export key types for easier access
pub use error::{GeneratorError, GeneratorResult};

// Export generator functions for easier access
pub use api_doc_generator::generate_api_doc;
pub use implementation_details_generator::generate_implementation_details;
pub use testing_doc_generator::generate_testing_doc;
pub use tech_debt_report_generator::generate_tech_debt_report;
pub use summary_report_generator::generate_summary_report;
pub use enhanced_central_hub_generator::generate_enhanced_central_hub;
pub use migration_roadmap_generator::MigrationRoadmapGenerator;
pub use component_tree_generator::ComponentTreeGenerator;
pub use api_map_generator::ApiMapGenerator;
pub use sync_architecture_generator::generate_sync_architecture;
pub use database_architecture_generator::generate_database_architecture;
