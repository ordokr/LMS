use std::fs;
use std::path::{Path, PathBuf};
use chrono::Local;
use std::collections::HashMap;

use crate::core::analysis_result::AnalysisResult;
use crate::core::model_analyzer::{ModelAnalyzer, ModelMetrics};

/// Generate model report
pub fn generate_model_report(result: &AnalysisResult) -> Result<(), String> {
    println!("Generating model report...");
    
    // Ensure docs directory exists
    let docs_dir = Path::new("docs");
    if !docs_dir.exists() {
        fs::create_dir_all(docs_dir)
            .map_err(|e| format!("Failed to create docs directory: {}", e))?;
    }
    
    // Ensure models directory exists
    let models_dir = docs_dir.join("models");
    if !models_dir.exists() {
        fs::create_dir_all(&models_dir)
            .map_err(|e| format!("Failed to create models directory: {}", e))?;
    }
    
    // Create the report path
    let report_path = models_dir.join("model_report.md");
    
    // Create the analyzer
    let analyzer = ModelAnalyzer::new(Path::new(".").to_path_buf());
    
    // Generate the report
    let report = analyzer.generate_report()?;
    
    // Write to file
    fs::write(&report_path, report)
        .map_err(|e| format!("Failed to write model report: {}", e))?;
    
    println!("Model report generated at: {:?}", report_path);
    
    // Create a summary report
    let summary_path = docs_dir.join("model_summary.md");
    
    // Analyze the codebase
    let metrics = analyzer.analyze_codebase()?;
    
    // Generate the summary
    let summary = generate_summary(&metrics);
    
    // Write to file
    fs::write(&summary_path, summary)
        .map_err(|e| format!("Failed to write model summary: {}", e))?;
    
    println!("Model summary generated at: {:?}", summary_path);
    
    // Generate model diagrams
    generate_model_diagrams(&metrics, &models_dir)?;
    
    Ok(())
}

/// Generate a summary of the model metrics
fn generate_summary(metrics: &ModelMetrics) -> String {
    let mut summary = String::new();
    
    // Header
    summary.push_str("# Data Models Summary\n\n");
    summary.push_str(&format!("_Generated on: {}_\n\n", Local::now().format("%Y-%m-%d")));
    
    // Model Counts
    summary.push_str("## Model Counts\n\n");
    summary.push_str(&format!("**Total Models: {}**\n\n", metrics.total_models));
    summary.push_str("| Source System | Count |\n");
    summary.push_str("|--------------|-------|\n");
    summary.push_str(&format!("| Canvas | {} |\n", metrics.canvas_models));
    summary.push_str(&format!("| Discourse | {} |\n", metrics.discourse_models));
    summary.push_str(&format!("| Native | {} |\n\n", metrics.native_models));
    
    // Model List
    summary.push_str("## Model List\n\n");
    
    for (source, models) in &metrics.models_by_source {
        summary.push_str(&format!("### {} Models\n\n", source));
        
        if !models.is_empty() {
            summary.push_str("| Model | File | Fields | Relationships |\n");
            summary.push_str("|-------|------|--------|---------------|\n");
            
            for model in models {
                summary.push_str(&format!("| {} | {} | {} | {} |\n",
                    model.name,
                    model.file_path,
                    model.fields.len(),
                    model.relationships.len()));
            }
        } else {
            summary.push_str("No models found.\n");
        }
        
        summary.push_str("\n");
    }
    
    // Relationship Summary
    summary.push_str("## Relationship Summary\n\n");
    
    let mut relationship_counts = HashMap::new();
    for relationship in &metrics.relationships {
        *relationship_counts.entry(relationship.relationship_type.clone()).or_insert(0) += 1;
    }
    
    if !relationship_counts.is_empty() {
        summary.push_str("| Relationship Type | Count |\n");
        summary.push_str("|-------------------|-------|\n");
        
        for (relationship_type, count) in &relationship_counts {
            summary.push_str(&format!("| {} | {} |\n", relationship_type, count));
        }
    } else {
        summary.push_str("No relationships found.\n");
    }
    
    summary.push_str("\n");
    
    // Recommendations
    summary.push_str("## Recommendations\n\n");
    
    if metrics.relationships.is_empty() {
        summary.push_str("1. **Define Relationships**: Consider defining relationships between models to improve data integrity and navigation.\n");
    }
    
    if metrics.native_models < 10 {
        summary.push_str("2. **Implement More Native Models**: Consider implementing more native models to reduce dependency on external systems.\n");
    }
    
    summary.push_str("\nFor detailed information, see the [full model report](./models/model_report.md).\n");
    
    summary
}

/// Generate model diagrams
fn generate_model_diagrams(metrics: &ModelMetrics, models_dir: &Path) -> Result<(), String> {
    // Generate a diagram for each source system
    for (source, models) in &metrics.models_by_source {
        // Skip if no models
        if models.is_empty() {
            continue;
        }
        
        // Create the diagram path
        let diagram_path = models_dir.join(format!("{}_models.md", source.to_lowercase()));
        
        // Generate the diagram
        let mut diagram = String::new();
        
        // Header
        diagram.push_str(&format!("# {} Models\n\n", source));
        diagram.push_str(&format!("_Generated on: {}_\n\n", Local::now().format("%Y-%m-%d")));
        
        // Mermaid diagram
        diagram.push_str("```mermaid\nclassDiagram\n");
        
        // Add classes
        for model in models {
            diagram.push_str(&format!("    class {} {{\n", model.name));
            
            for field in &model.fields {
                diagram.push_str(&format!("        {} {}\n",
                    field.field_type,
                    field.name));
            }
            
            diagram.push_str("    }\n");
        }
        
        // Add relationships
        for model in models {
            for relationship in &model.relationships {
                let arrow = match relationship.relationship_type.as_str() {
                    "OneToOne" => "--",
                    "OneToMany" => "-->",
                    "ManyToOne" => "<--",
                    "ManyToMany" => "<-->",
                    _ => "--",
                };
                
                diagram.push_str(&format!("    {} {} {}\n",
                    model.name,
                    arrow,
                    relationship.related_model));
            }
        }
        
        diagram.push_str("```\n");
        
        // Write to file
        fs::write(&diagram_path, diagram)
            .map_err(|e| format!("Failed to write model diagram: {}", e))?;
        
        println!("Model diagram generated at: {:?}", diagram_path);
    }
    
    Ok(())
}
