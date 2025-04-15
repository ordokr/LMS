use std::path::PathBuf;
use crate::analyzers::unified_analyzer::{UnifiedProjectAnalyzer, AnalysisResult};
use crate::analyzers::ai_knowledge_enhancer::AiKnowledgeEnhancer;
use crate::analyzers::metrics_visualizer::MetricsVisualizer;
use crate::analyzers::dashboard_generator::DashboardGenerator;

impl UnifiedProjectAnalyzer {
    /// Generate AI knowledge base
    pub async fn generate_ai_knowledge_base(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        println!("Generating AI knowledge base...");
        
        let result = self.result.lock().await.clone();
        
        // Create AI knowledge enhancer
        let knowledge_enhancer = AiKnowledgeEnhancer::new(self.base_dir.clone());
        
        // Generate enhanced knowledge base
        match knowledge_enhancer.enhance_knowledge_base(&result) {
            Ok(path) => println!("AI knowledge base generated at {:?}", path),
            Err(e) => println!("Failed to generate AI knowledge base: {}", e),
        }
        
        // Generate AI agent guidance
        match knowledge_enhancer.generate_agent_guidance(&result) {
            Ok(path) => println!("AI agent guidance generated at {:?}", path),
            Err(e) => println!("Failed to generate AI agent guidance: {}", e),
        }
        
        Ok(())
    }
    
    /// Generate metrics visualizations
    pub async fn generate_metrics_visualizations(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        println!("Generating metrics visualizations...");
        
        let result = self.result.lock().await.clone();
        
        // Create metrics visualizer
        let metrics_visualizer = MetricsVisualizer::new(self.base_dir.clone());
        
        // Generate dashboard
        match metrics_visualizer.generate_dashboard(&result) {
            Ok(path) => println!("Metrics dashboard generated at {:?}", path),
            Err(e) => println!("Failed to generate metrics dashboard: {}", e),
        }
        
        // Generate metrics report
        match metrics_visualizer.generate_metrics_report(&result) {
            Ok(path) => println!("Metrics report generated at {:?}", path),
            Err(e) => println!("Failed to generate metrics report: {}", e),
        }
        
        Ok(())
    }
    
    /// Generate project dashboard
    pub async fn generate_project_dashboard(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        println!("Generating project dashboard...");
        
        let result = self.result.lock().await.clone();
        
        // Create dashboard generator
        let dashboard_generator = DashboardGenerator::new(self.base_dir.clone());
        
        // Generate dashboard
        match dashboard_generator.generate_dashboard(&result) {
            Ok(path) => println!("Project dashboard generated at {:?}", path),
            Err(e) => println!("Failed to generate project dashboard: {}", e),
        }
        
        Ok(())
    }
}
