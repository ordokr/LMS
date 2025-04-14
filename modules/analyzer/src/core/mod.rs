pub mod analyzer_config;
pub mod analysis_result;
pub mod unified_analyzer;
pub mod tech_debt_analyzer;
pub mod code_quality_analyzer;
pub mod model_analyzer;
pub mod dependency_analyzer;
pub mod trend_analyzer;
pub mod statistical_trend_analyzer;
pub mod ai_insights_analyzer;
pub mod llm_integration;

// Re-export key structures for easier access
pub use analyzer_config::AnalyzerConfig;
pub use analysis_result::AnalysisResult;
pub use unified_analyzer::UnifiedAnalyzer;
pub use tech_debt_analyzer::TechDebtAnalyzer;
pub use code_quality_analyzer::CodeQualityAnalyzer;
pub use model_analyzer::ModelAnalyzer;
pub use dependency_analyzer::DependencyAnalyzer;
pub use trend_analyzer::TrendAnalyzer;
pub use statistical_trend_analyzer::StatisticalTrendAnalyzer;
pub use ai_insights_analyzer::AiInsightsAnalyzer;
pub use llm_integration::LlmIntegration;
