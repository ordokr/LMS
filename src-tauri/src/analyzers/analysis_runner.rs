use std::path::{Path, PathBuf};
use std::sync::Arc;
use serde::{Serialize, Deserialize};
use chrono::Utc;
use tokio::fs;

use crate::analyzers::unified_analyzer::{UnifiedProjectAnalyzer, AnalysisResult};
use crate::utils::file_system::FileSystemUtils;
use crate::ai::gemini_analyzer::GeminiAnalyzer;
use crate::analyzers::docs_updater::DocsUpdater;
use crate::analyzers::js_migration_analyzer::JsMigrationAnalyzer;
use crate::analyzers::dashboard_generator::DashboardGenerator;
use crate::analyzers::tech_debt_analyzer::TechDebtAnalyzer;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisCommand {
    pub target_dirs: Vec<PathBuf>,
    pub exclude_patterns: Vec<String>,
    pub output_dir: PathBuf,
    pub update_rag_knowledge_base: bool,
    pub generate_ai_insights: bool,
    pub analyze_js_files: bool,
    pub generate_dashboard: bool,
    pub analyze_tech_debt: bool,
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
            analyze_js_files: true,
            generate_dashboard: false,
            analyze_tech_debt: false,
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
        
        // Create docs updater
        let docs_updater = DocsUpdater::new(self.base_dir.clone());
        
        // Update the central reference hub
        docs_updater.update_central_reference_hub(&result)?;
        
        // Update the RAG knowledge base if requested
        if command.update_rag_knowledge_base {
            docs_updater.update_rag_knowledge_base(&result)?;
            
            // Generate AI-specific documentation for agents
            docs_updater.generate_last_analysis_results(&result)?;
        }
        
        // Generate dashboard
        if command.generate_dashboard {
            let dashboard_generator = DashboardGenerator::new(self.base_dir.clone());
            dashboard_generator.generate_dashboard(&result)?;
        }
        
        // Analyze technical debt
        if command.analyze_tech_debt {
            let tech_debt_analyzer = TechDebtAnalyzer::new(self.base_dir.clone());
            let report = tech_debt_analyzer.generate_report()?;
            
            // Save the tech debt report
            let report_path = self.base_dir.join("docs").join("technical_debt_report.md");
            fs::write(&report_path, report).await?;
            
            println!("Technical debt report generated: {:?}", report_path);
        }
        
        // Generate JS to Rust migration plan if requested
        if command.analyze_js_files {
            self.analyze_js_files().await?;
        }
        
        // Generate AI insights if requested
        if command.generate_ai_insights && self.gemini.is_some() {
            self.generate_ai_insights(&result).await?;
        }
        
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
    
    async fn analyze_js_files(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        println!("Analyzing JavaScript files for Rust migration...");
        
        // Create the JS migration analyzer
        let mut js_analyzer = JsMigrationAnalyzer::new(self.base_dir.clone());
        
        // Generate migration plan
        let migration_plan = js_analyzer.generate_migration_plan()?;
        
        // Write the migration plan to a file
        let plan_path = self.base_dir.join("docs").join("js_to_rust_migration_plan.md");
        fs::create_dir_all(plan_path.parent().unwrap()).await?;
        fs::write(&plan_path, migration_plan).await?;
        
        // Find high priority files and generate templates
        let js_files = js_analyzer.discover_js_files();
        
        // Create a templates directory
        let templates_dir = self.base_dir.join("tools").join("rust_templates");
        fs::create_dir_all(&templates_dir).await?;
        
        // Process up to 5 high priority files
        let mut processed = 0;
        for js_path in js_files.iter().take(10) {
            match js_analyzer.analyze_js_file(js_path) {
                Ok(analysis) => {
                    // Skip files that aren't high priority or already completed
                    if analysis.port_priority < 8 || !matches!(analysis.port_status, crate::analyzers::js_migration_analyzer::PortStatus::NotStarted) {
                        continue;
                    }
                    
                    // Generate Rust template
                    match js_analyzer.generate_rust_template(js_path) {
                        Ok(template) => {
                            // Create filename for the template
                            let file_name = js_path.file_stem().unwrap_or_default().to_string_lossy();
                            let template_path = templates_dir.join(format!("{}.rs", file_name));
                            
                            // Write the template
                            fs::write(&template_path, template).await?;
                            println!("Generated Rust template for {}: {:?}", file_name, template_path);
                            
                            processed += 1;
                            if processed >= 5 {
                                break;
                            }
                        },
                        Err(e) => {
                            println!("Error generating template for {}: {}", js_path.display(), e);
                        }
                    }
                },
                Err(e) => {
                    println!("Error analyzing {}: {}", js_path.display(), e);
                }
            }
        }
        
        println!("JavaScript migration analysis completed. Plan written to {:?}", plan_path);
        println!("Generated {} Rust templates in {:?}", processed, templates_dir);
        
        Ok(())
    }
}
