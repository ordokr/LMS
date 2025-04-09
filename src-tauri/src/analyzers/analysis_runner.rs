use std::path::{Path, PathBuf};
use std::sync::Arc;
use serde::{Serialize, Deserialize};
use chrono::Utc;
use tokio::fs;

use crate::analyzers::unified_analyzer::{UnifiedProjectAnalyzer, AnalysisResult};
use crate::utils::file_system::FileSystemUtils;
use crate::ai::gemini_analyzer::GeminiAnalyzer;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisCommand {
    pub target_dirs: Vec<PathBuf>,
    pub exclude_patterns: Vec<String>,
    pub output_dir: PathBuf,
    pub update_rag_knowledge_base: bool,
    pub generate_ai_insights: bool,
}

impl Default for AnalysisCommand {
    fn default() -> Self {
        Self {
            target_dirs: vec![PathBuf::from(".")],
            exclude_patterns: vec![
                String::from("node_modules"),
                String::from("target"),
                String::from(".git"),
                String::from("build-output"),
            ],
            output_dir: PathBuf::from("docs"),
            update_rag_knowledge_base: true,
            generate_ai_insights: true,
        }
    }
}

pub struct AnalysisRunner {
    base_dir: PathBuf,
    fs_utils: Arc<FileSystemUtils>,
    gemini: Option<GeminiAnalyzer>,
}

impl AnalysisRunner {
    pub fn new(base_dir: PathBuf) -> Self {
        let fs_utils = Arc::new(FileSystemUtils::new(&base_dir));
        
        Self {
            base_dir,
            fs_utils,
            gemini: None,
        }
    }
    
    pub fn with_gemini(mut self, gemini: GeminiAnalyzer) -> Self {
        self.gemini = Some(gemini);
        self
    }
    
    pub async fn run_analysis(&self, command: &AnalysisCommand) -> Result<AnalysisResult, Box<dyn std::error::Error + Send + Sync>> {
        println!("Starting project analysis...");
        
        // Create the unified project analyzer
        let analyzer = UnifiedProjectAnalyzer::new(self.base_dir.clone(), self.fs_utils.clone());
        
        // Run the analysis
        let result = analyzer.analyze().await?;
        
        // Save the analysis result to a JSON file
        self.save_analysis_result(&result).await?;
        
        // Generate the central reference hub markdown
        analyzer.generate_central_reference_hub().await?;
        
        // Update the RAG knowledge base if requested
        if command.update_rag_knowledge_base {
            self.update_rag_knowledge_base(&result).await?;
        }
        
        // Generate AI insights if requested
        if command.generate_ai_insights && self.gemini.is_some() {
            self.generate_ai_insights(&result).await?;
        }
        
        // Update the LAST_ANALYSIS_RESULTS.md file
        self.update_last_analysis_results(&result).await?;
        
        println!("Project analysis completed successfully.");
        
        Ok(result)
    }
    
    async fn save_analysis_result(&self, result: &AnalysisResult) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let json = serde_json::to_string_pretty(result)?;
        let output_path = self.base_dir.join("audit_report.json");
        
        fs::write(&output_path, json).await?;
        
        println!("Analysis result saved to {:?}", output_path);
        
        Ok(())
    }
    
    async fn update_rag_knowledge_base(&self, result: &AnalysisResult) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        println!("Updating RAG knowledge base...");
        
        // Ensure the RAG knowledge base directory exists
        let rag_dir = self.base_dir.join("rag_knowledge_base");
        let integration_dir = rag_dir.join("integration");
        
        if !integration_dir.exists() {
            fs::create_dir_all(&integration_dir).await?;
        }
        
        // Update technical implementation
        let tech_impl_path = integration_dir.join("technical_implementation.md");
        let mut tech_impl_content = format!(
            "# Canvas-Discourse Technical Implementation Details\n\nGenerated on: {}\n\n",
            Utc::now().format("%Y-%m-%d")
        );
        
        // Add overview section
        tech_impl_content.push_str("## Overview\n\n");
        tech_impl_content.push_str("This document details the technical implementation of the Canvas-Discourse integration.\n\n");
        
        // Add implementation status section
        tech_impl_content.push_str("## Implementation Status\n\n");
        tech_impl_content.push_str("| Component | Status | Completeness |\n");
        tech_impl_content.push_str("|-----------|--------|-------------|\n");
        
        for (area, metrics) in &result.feature_areas {
            let percent = if metrics.total > 0 {
                (metrics.implemented as f32 / metrics.total as f32) * 100.0
            } else {
                0.0
            };
            
            tech_impl_content.push_str(&format!(
                "| {} | {} | {:.1}% |\n",
                area,
                if percent > 75.0 {
                    "Complete"
                } else if percent > 25.0 {
                    "In Progress"
                } else {
                    "Planned"
                },
                percent
            ));
        }
        
        tech_impl_content.push_str("\n");
        
        // Add authentication implementation section
        tech_impl_content.push_str("## Authentication Implementation\n\n");
        tech_impl_content.push_str("The authentication system uses JWT tokens for secure user sessions and implements an offline-first approach that works even when connectivity is limited.\n\n");
        
        // Add model synchronization section
        tech_impl_content.push_str("## Model Synchronization\n\n");
        tech_impl_content.push_str("Models are synchronized between Canvas and Discourse using a bidirectional mapping system that preserves data integrity and handles conflicts gracefully.\n\n");
        
        // Add API integration section
        tech_impl_content.push_str("## API Integration\n\n");
        tech_impl_content.push_str("The API layer integrates Canvas and Discourse endpoints into a unified interface, providing consistent access to both systems.\n\n");
        
        // Add synchronization implementation section
        tech_impl_content.push_str("## Synchronization Implementation\n\n");
        tech_impl_content.push_str("The synchronization system uses a vector-clock CRDT approach to ensure data consistency even with offline operations.\n\n");
        
        // Add error handling section
        tech_impl_content.push_str("## Error Handling and Retry Mechanisms\n\n");
        tech_impl_content.push_str("The system implements robust error handling and retry mechanisms to ensure data integrity even in unstable network conditions.\n\n");
        
        // Write the technical implementation file
        fs::write(&tech_impl_path, tech_impl_content).await?;
        
        println!("RAG knowledge base updated.");
        
        Ok(())
    }
    
    async fn generate_ai_insights(&self, result: &AnalysisResult) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        println!("Generating AI insights...");
        
        if let Some(gemini) = &self.gemini {
            // Convert the analysis result to JSON for Gemini
            let analysis_json = serde_json::to_string_pretty(result)?;
            
            // Generate insights using Gemini
            let prompt = format!(
                "Analyze this codebase summary and provide insights about architecture, implementation status, and recommendations for next steps:\n\n{}",
                analysis_json
            );
            
            let insights = gemini.generate_insights(&prompt).await?;
            
            // Write the insights to a file
            let insights_path = self.base_dir.join("docs").join("ai_code_insights.md");
            fs::write(&insights_path, insights).await?;
            
            println!("AI insights generated at {:?}", insights_path);
        }
        
        Ok(())
    }
    
    async fn update_last_analysis_results(&self, result: &AnalysisResult) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        println!("Updating LAST_ANALYSIS_RESULTS.md...");
        
        // Create a summary of the analysis result
        let mut content = format!(
            "# Last Analysis Results\n\n_Generated on: {}_\n\n",
            Utc::now().format("%Y-%m-%d")
        );
        
        // Add overall status
        content.push_str("## Overall Status\n\n");
        content.push_str(&format!("- **Phase**: {}\n", result.project_status.phase));
        content.push_str(&format!("- **Completion**: {:.1}%\n", result.project_status.completion_percentage));
        content.push_str(&format!("- **Last Active Area**: {}\n", result.project_status.last_active_area));
        
        if let Some(date) = &result.project_status.estimated_completion_date {
            content.push_str(&format!("- **Estimated Completion**: {}\n", date.format("%Y-%m-%d")));
        }
        
        content.push_str("\n");
        
        // Add implementation summary
        content.push_str("## Implementation Summary\n\n");
        content.push_str(&format!("- **Models**: {}/{} implemented ({:.1}%)\n", 
            result.models.implemented, 
            result.models.total,
            if result.models.total > 0 {
                (result.models.implemented as f32 / result.models.total as f32) * 100.0
            } else {
                0.0
            }
        ));
        
        content.push_str(&format!("- **API Endpoints**: {}/{} implemented ({:.1}%)\n", 
            result.api_endpoints.implemented, 
            result.api_endpoints.total,
            if result.api_endpoints.total > 0 {
                (result.api_endpoints.implemented as f32 / result.api_endpoints.total as f32) * 100.0
            } else {
                0.0
            }
        ));
        
        content.push_str(&format!("- **UI Components**: {}/{} implemented ({:.1}%)\n", 
            result.ui_components.implemented, 
            result.ui_components.total,
            if result.ui_components.total > 0 {
                (result.ui_components.implemented as f32 / result.ui_components.total as f32) * 100.0
            } else {
                0.0
            }
        ));
        
        content.push_str(&format!("- **Test Coverage**: {:.1}%\n", result.tests.coverage));
        
        content.push_str("\n");
        
        // Add recommendation summary
        content.push_str("## Recommendations\n\n");
        
        for recommendation in &result.recommendations {
            content.push_str(&format!("- **{}**: {} (Priority: {})\n", 
                recommendation.area, 
                recommendation.description,
                recommendation.priority
            ));
        }
        
        // Write the file
        let output_path = self.base_dir.join("LAST_ANALYSIS_RESULTS.md");
        fs::write(&output_path, content).await?;
        
        println!("LAST_ANALYSIS_RESULTS.md updated.");
        
        Ok(())
    }
}
