pub mod central_hub_generator;
pub mod architecture_doc_generator;
pub mod models_doc_generator;
pub mod api_doc_generator;
pub mod tech_debt_report_generator;
pub mod code_quality_report_generator;
pub mod model_report_generator;
pub mod dashboard_generator;
pub mod report_generator;

// Re-export key functions for easier access
pub use report_generator::generate_reports;
pub use report_generator::generate_summary_report;
pub use central_hub_generator::generate_central_reference_hub;
pub use code_quality_report_generator::generate_code_quality_report;
pub use model_report_generator::generate_model_report;
