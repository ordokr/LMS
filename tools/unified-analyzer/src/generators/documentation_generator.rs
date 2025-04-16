rust
use anyhow::{{Context, Result}};
use chrono::Local;
use log::info;
use crate::output_schema::UnifiedOutput;
use crate::utils::file_system::FileSystemUtils;
use std::fs;

pub fn generate_documentation(unified_output: &UnifiedOutput, base_dir: &std::path::PathBuf) -> Result<()> {
    // Ensure docs directory exists
    let docs_dir = base_dir.join("docs");
    if !docs_dir.exists() {
        fs::create_dir_all(&docs_dir)
            .map_err(|e| format!("Failed to create docs directory: {}", e))?;
    }

    // Generate central reference hub
    generate_central_hub(unified_output, base_dir)?;

    // Generate architecture documentation
    //architecture_doc_generator::generate_architecture_doc(result)?;

    // Generate models documentation
    //models_doc_generator::generate_models_doc(result)?;

    // Generate API documentation
    //api_doc_generator::generate_api_doc(result)?;

    // Generate technical debt report
    //tech_debt_report_generator::generate_tech_debt_report(result)?;

    // Generate code quality report
    //code_quality_report_generator::generate_code_quality_report(result)?;

    // Generate model report
    //model_report_generator::generate_model_report(result)?;
    Ok(())
}

fn generate_central_hub(unified_output: &UnifiedOutput, base_dir: &std::path::PathBuf) -> Result<()> {
    let mut content = String::from("# Central Reference Hub\n\n");
    content.push_str("## File List\n\n");

    for file in &unified_output.files {
        content.push_str(&format!("- {}\n", file.path));
    }
    let path = FileSystemUtils::create_path(base_dir, "docs/central_reference_hub.md");
    std::fs::write(path, content)?;
    Ok(())
}
