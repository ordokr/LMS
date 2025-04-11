use std::path::{Path, PathBuf};
use std::fs;
use anyhow::{Result, Context};
use chrono::Local;
use log::{info, warn};

/// Options for the Integration Report Generator
#[derive(Debug, Clone)]
pub struct IntegrationReportOptions {
    pub base_dir: PathBuf,
    pub rag_knowledge_base: String,
    pub output_dir: String,
}

impl Default for IntegrationReportOptions {
    fn default() -> Self {
        Self {
            base_dir: PathBuf::from("."),
            rag_knowledge_base: "rag_knowledge_base".to_string(),
            output_dir: "docs".to_string(),
        }
    }
}

/// Canvas-Discourse Integration Report Generator
/// Generates comprehensive integration documentation
pub struct IntegrationReportGenerator {
    options: IntegrationReportOptions,
}

impl IntegrationReportGenerator {
    /// Create a new integration report generator
    pub fn new(options: Option<IntegrationReportOptions>) -> Self {
        let options = options.unwrap_or_default();
        
        Self { options }
    }
    
    /// Generate the Canvas-Discourse integration report
    pub async fn generate_report(&self) -> Result<PathBuf> {
        info!("Generating Canvas-Discourse integration report...");
        
        // Read integration files from the RAG knowledge base
        let integration_dir = self.options.base_dir.join(&self.options.rag_knowledge_base).join("integration");
        let mut report = self.generate_report_header();
        
        // Load integration points if available
        match self.load_and_format_file(&integration_dir.join("integration_points.md"), "Model Mapping") {
            Ok(content) => report.push_str(&content),
            Err(err) => warn!("Could not load integration points: {}", err),
        }
        
        // Load architecture blueprint if available
        match self.load_and_format_file(&integration_dir.join("architecture-blueprint.md"), "Architecture") {
            Ok(content) => report.push_str(&content),
            Err(err) => warn!("Could not load architecture blueprint: {}", err),
        }
        
        // Add implementation guidance
        report.push_str(&self.generate_implementation_guidance());
        
        // Write the report to file
        let output_dir = self.options.base_dir.join(&self.options.output_dir);
        fs::create_dir_all(&output_dir)
            .context(format!("Failed to create output directory: {:?}", output_dir))?;
            
        let output_path = output_dir.join("canvas_discourse_integration.md");
        fs::write(&output_path, report)
            .context(format!("Failed to write report to: {:?}", output_path))?;
        
        info!("Integration report saved to: {:?}", output_path);
        Ok(output_path)
    }
    
    /// Generate the report header section
    fn generate_report_header(&self) -> String {
        let today = Local::now().format("%Y-%m-%d").to_string();
        
        format!(
            r#"# Canvas-Discourse Integration Reference

_Generated on: {}_

## Overview

This document serves as the central reference for integrating Canvas LMS with Discourse forums.
It provides key information on model mappings, integration architecture, and implementation recommendations.

"#,
            today
        )
    }
    
    /// Load and format content from a file
    fn load_and_format_file(&self, file_path: &Path, section_title: &str) -> Result<String> {
        if !file_path.exists() {
            return Err(anyhow::anyhow!("File not found: {:?}", file_path));
        }
        
        let content = fs::read_to_string(file_path)
            .context(format!("Failed to read file: {:?}", file_path))?;
            
        Ok(self.extract_and_format_content(&content, section_title))
    }
    
    /// Extract and format content from RAG documents
    fn extract_and_format_content(&self, content: &str, section_title: &str) -> String {
        // For this simplified version, just return the content with a section header
        format!("## {}\n\n{}\n\n", section_title, content)
    }
    
    /// Generate implementation guidance section
    fn generate_implementation_guidance(&self) -> String {
        r#"## Implementation Recommendations

Based on the integration architecture and requirements, we recommend:

1. **API-based Integration**: Use REST APIs on both systems as the primary integration method
   - Canvas API for course and assignment data
   - Discourse API for forum interaction

2. **Single Sign-On**: Implement SSO between Canvas and Discourse 
   - Use JWT or OAuth 2.0 for secure authentication
   - Maintain user role synchronization

3. **Synchronization Service**: Create a middle-tier service that:
   - Maps Canvas courses to Discourse categories
   - Synchronizes discussion topics between systems
   - Handles user permission mapping

4. **Error Handling & Resilience**: 
   - Implement proper error handling and retry mechanisms
   - Add logging for synchronization failures
   - Design for eventual consistency

## Testing Strategy

1. Unit test each integration point independently
2. Integration tests for end-to-end flows
3. Load testing to ensure synchronization performance
4. Security testing for authentication flows

## Next Steps

1. Complete detailed technical design document
2. Set up development environment with Canvas and Discourse instances
3. Implement authentication integration (SSO)
4. Develop course-to-category synchronization
5. Implement discussion topic synchronization
"#.to_string()
    }
    
    /// Update the central reference hub with a link to the integration report
    pub fn update_central_reference_hub(&self, hub_path: &Path, report_path: &Path) -> Result<()> {
        if !hub_path.exists() {
            warn!("Central reference hub not found at: {:?}", hub_path);
            return Ok(());
        }
        
        info!("Updating central reference hub: {:?}", hub_path);
        
        // Read the existing content
        let mut content = fs::read_to_string(hub_path)
            .context(format!("Failed to read central reference hub: {:?}", hub_path))?;
        
        // Create the link text
        let relative_path = pathdiff::diff_paths(report_path, hub_path.parent().unwrap_or(Path::new(".")))
            .unwrap_or_else(|| report_path.to_path_buf());
            
        let link_text = format!(
            "- [Canvas-Discourse Integration]({}) - Complete integration reference\n",
            relative_path.to_string_lossy()
        );
        
        // Check if the link already exists
        if !content.contains(&link_text) {
            // Find the appropriate section to add the link
            if let Some(pos) = content.find("## Integration Documents") {
                // Find the end of the section header line
                if let Some(line_end) = content[pos..].find('\n') {
                    let insert_pos = pos + line_end + 1;
                    
                    // Insert the link
                    content.insert_str(insert_pos, &format!("\n{}", link_text));
                    
                    // Write the updated content
                    fs::write(hub_path, content)
                        .context(format!("Failed to update central reference hub: {:?}", hub_path))?;
                        
                    info!("Added integration report link to central reference hub");
                }
            } else {
                // If section doesn't exist, add it
                content.push_str("\n## Integration Documents\n\n");
                content.push_str(&link_text);
                
                // Write the updated content
                fs::write(hub_path, content)
                    .context(format!("Failed to update central reference hub: {:?}", hub_path))?;
                    
                info!("Created Integration Documents section and added link to central reference hub");
            }
        } else {
            info!("Integration report link already exists in central reference hub");
        }
        
        Ok(())
    }
}
