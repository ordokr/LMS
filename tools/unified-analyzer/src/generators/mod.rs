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
pub mod db_schema_generator;
pub mod db_schema_visualization;
pub mod simple_db_schema_viz;
pub mod simple_source_db_viz;
pub mod rust_source_db_viz;

pub mod documentation_generator;
// Medium Priority Generators
pub mod sync_architecture_generator;
pub mod database_architecture_generator;

// All generators in one place
pub mod all_generators;

// Re-export key types for easier access
// These are used in all_generators.rs
// pub use error::GeneratorError;

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
pub use db_schema_generator::DbSchemaGenerator;
// pub use db_schema_visualization::generate_db_schema_visualization;
pub use simple_db_schema_viz::generate_simple_db_schema_visualization;
// pub use simple_source_db_viz::generate_simple_source_db_visualization;
pub use rust_source_db_viz::generate_rust_source_db_visualization;
pub use sync_architecture_generator::generate_sync_architecture;
pub use database_architecture_generator::generate_database_architecture;
