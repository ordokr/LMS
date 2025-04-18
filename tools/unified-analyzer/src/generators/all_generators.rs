use std::path::Path;
use crate::analyzers::unified_analyzer::AnalysisResult;
use crate::generators::error::{GeneratorError, GeneratorResult};

/// Generate all documentation
pub fn generate_all_documentation(result: &AnalysisResult, base_dir: &Path) -> GeneratorResult<()> {
    // Generate API documentation
    if let Err(e) = crate::generators::generate_api_doc(result, base_dir) {
        return Err(GeneratorError::ContentGeneration(format!("Failed to generate API documentation: {}", e)));
    }

    // Generate implementation details
    if let Err(e) = crate::generators::generate_implementation_details(result, base_dir) {
        return Err(GeneratorError::ContentGeneration(format!("Failed to generate implementation details: {}", e)));
    }

    // Generate testing documentation
    if let Err(e) = crate::generators::generate_testing_doc(result, base_dir) {
        return Err(GeneratorError::ContentGeneration(format!("Failed to generate testing documentation: {}", e)));
    }

    // Generate tech debt report
    if let Err(e) = crate::generators::generate_tech_debt_report(result, base_dir) {
        return Err(GeneratorError::ContentGeneration(format!("Failed to generate tech debt report: {}", e)));
    }

    // Generate summary report
    if let Err(e) = crate::generators::generate_summary_report(result, base_dir) {
        return Err(GeneratorError::ContentGeneration(format!("Failed to generate summary report: {}", e)));
    }

    // Generate enhanced central hub
    if let Err(e) = crate::generators::generate_enhanced_central_hub(result, base_dir) {
        return Err(GeneratorError::ContentGeneration(format!("Failed to generate enhanced central hub: {}", e)));
    }

    // Generate sync architecture
    if let Err(e) = crate::generators::generate_sync_architecture(result, base_dir) {
        return Err(GeneratorError::ContentGeneration(format!("Failed to generate sync architecture: {}", e)));
    }

    // Generate database architecture
    if let Err(e) = crate::generators::generate_database_architecture(result, base_dir) {
        return Err(GeneratorError::ContentGeneration(format!("Failed to generate database architecture: {}", e)));
    }

    // Generate migration guide
    if let Err(e) = crate::generators::generate_migration_guide(result, base_dir) {
        return Err(GeneratorError::ContentGeneration(format!("Failed to generate migration guide: {}", e)));
    }

    // Create output directory if it doesn't exist
    let docs_dir = base_dir.join("docs");
    if !docs_dir.exists() {
        std::fs::create_dir_all(&docs_dir).map_err(|e| {
            GeneratorError::DirectoryCreation(format!("Failed to create docs directory: {}", e))
        })?;
    }

    // Parse some data for demonstration
    let data = r#"{"key": "value"}"#;
    let _parsed: serde_json::Value = serde_json::from_str(data).map_err(|e| {
        GeneratorError::DataParsing(format!("Failed to parse JSON data: {}", e))
    })?;

    Ok(())
}
