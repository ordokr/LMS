pub mod central_hub_generator;
pub mod architecture_doc_generator;
pub mod models_doc_generator;
pub mod api_doc_generator;
pub mod tech_debt_report_generator;
pub mod code_quality_report_generator;
pub mod model_report_generator;
pub mod dependency_report_generator;
pub mod trend_report_generator;
pub mod statistical_trend_generator;
pub mod llm_insights_generator;
pub mod dashboard_generator;
pub mod enhanced_dashboard_generator;
pub mod report_generator;

// Re-export key functions for easier access
pub use report_generator::generate_reports;
pub use report_generator::generate_summary_report;
pub use central_hub_generator::generate_central_reference_hub;
pub use code_quality_report_generator::generate_code_quality_report;
pub use model_report_generator::generate_model_report;
pub use dependency_report_generator::generate_dependency_report;
pub use trend_report_generator::generate_trend_report;
pub use statistical_trend_generator::generate_statistical_trend_report;
pub use llm_insights_generator::generate_llm_insights_report;
pub use enhanced_dashboard_generator::generate_enhanced_dashboard;
