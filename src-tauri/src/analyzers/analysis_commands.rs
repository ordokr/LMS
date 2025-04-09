use std::path::PathBuf;
use std::sync::Arc;
use serde::{Serialize, Deserialize};
use tauri::{command, State};

use crate::analyzers::analysis_runner::{AnalysisRunner, AnalysisCommand};
use crate::analyzers::unified_analyzer::AnalysisResult;
use crate::utils::file_system::FileSystemUtils;
use crate::ai::gemini_analyzer::GeminiAnalyzer;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisRequest {
    pub target_dirs: Option<Vec<String>>,
    pub exclude_patterns: Option<Vec<String>>,
    pub output_dir: Option<String>,
    pub update_rag_knowledge_base: Option<bool>,
    pub generate_ai_insights: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisProgress {
    pub stage: String,
    pub progress: f32,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResponse {
    pub success: bool,
    pub message: String,
    pub central_reference_hub: String,
    pub last_analysis_results: String,
}

// Tauri command for analyzing the project
#[command]
pub async fn analyze_project(
    request: AnalysisRequest,
) -> Result<AnalysisResponse, String> {
    println!("Received analysis request: {:?}", request);
    
    // Get the current working directory
    let current_dir = std::env::current_dir()
        .map_err(|e| format!("Failed to get current directory: {}", e))?;
    
    // Create the analysis command
    let mut command = AnalysisCommand::default();
    
    // Override defaults with request values if provided
    if let Some(target_dirs) = request.target_dirs {
        command.target_dirs = target_dirs.into_iter()
            .map(|dir| current_dir.join(dir))
            .collect();
    } else {
        command.target_dirs = vec![current_dir.clone()];
    }
    
    if let Some(exclude_patterns) = request.exclude_patterns {
        command.exclude_patterns = exclude_patterns;
    }
    
    if let Some(output_dir) = request.output_dir {
        command.output_dir = current_dir.join(output_dir);
    } else {
        command.output_dir = current_dir.join("docs");
    }
    
    if let Some(update_rag) = request.update_rag_knowledge_base {
        command.update_rag_knowledge_base = update_rag;
    }
    
    if let Some(generate_insights) = request.generate_ai_insights {
        command.generate_ai_insights = generate_insights;
    }
    
    // Create file system utils
    let fs_utils = Arc::new(FileSystemUtils::new(&current_dir));
    
    // Create Gemini analyzer if needed
    let mut runner = AnalysisRunner::new(current_dir.clone());
    
    if command.generate_ai_insights {
        // Get API key from environment variable or a config file
        let api_key = std::env::var("GEMINI_API_KEY")
            .unwrap_or_else(|_| "".to_string());
        
        if !api_key.is_empty() {
            let gemini = GeminiAnalyzer::new(api_key);
            runner = runner.with_gemini(gemini);
        } else {
            println!("Warning: GEMINI_API_KEY not set, AI insights will not be generated.");
            command.generate_ai_insights = false;
        }
    }
    
    // Run the analysis
    match runner.run_analysis(&command).await {
        Ok(_) => {
            // Return success response
            Ok(AnalysisResponse {
                success: true,
                message: "Analysis completed successfully".to_string(),
                central_reference_hub: format!("file://{}/docs/central_reference_hub.md", current_dir.display()),
                last_analysis_results: format!("file://{}/LAST_ANALYSIS_RESULTS.md", current_dir.display()),
            })
        },
        Err(e) => {
            // Return error response
            Err(format!("Analysis failed: {}", e))
        }
    }
}

// Tauri command for running a quick analysis that only updates the LAST_ANALYSIS_RESULTS.md file
#[command]
pub async fn quick_analyze_project() -> Result<AnalysisResponse, String> {
    // Create a simplified analysis request
    let request = AnalysisRequest {
        target_dirs: None,
        exclude_patterns: None,
        output_dir: None,
        update_rag_knowledge_base: Some(false),
        generate_ai_insights: Some(false),
    };
    
    // Run the analysis
    analyze_project(request).await
}

// Tauri command for updating the RAG knowledge base
#[command]
pub async fn update_rag_knowledge_base() -> Result<AnalysisResponse, String> {
    // Create a specialized analysis request for updating the RAG knowledge base
    let request = AnalysisRequest {
        target_dirs: None,
        exclude_patterns: None,
        output_dir: None,
        update_rag_knowledge_base: Some(true),
        generate_ai_insights: Some(false),
    };
    
    // Run the analysis
    analyze_project(request).await
}

// Tauri command for generating AI insights
#[command]
pub async fn generate_ai_insights() -> Result<AnalysisResponse, String> {
    // Create a specialized analysis request for generating AI insights
    let request = AnalysisRequest {
        target_dirs: None,
        exclude_patterns: None,
        output_dir: None,
        update_rag_knowledge_base: Some(false),
        generate_ai_insights: Some(true),
    };
    
    // Run the analysis
    analyze_project(request).await
}

// CLI command for analyzing the project
pub async fn cli_analyze_project(
    target_dirs: Option<Vec<String>>,
    exclude_patterns: Option<Vec<String>>,
    output_dir: Option<String>,
    update_rag: bool,
    generate_insights: bool,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Create the analysis request
    let request = AnalysisRequest {
        target_dirs,
        exclude_patterns,
        output_dir,
        update_rag_knowledge_base: Some(update_rag),
        generate_ai_insights: Some(generate_insights),
    };
    
    // Run the analysis
    match analyze_project(request).await {
        Ok(response) => {
            println!("{}", response.message);
            println!("Central reference hub: {}", response.central_reference_hub);
            println!("Last analysis results: {}", response.last_analysis_results);
            Ok(())
        },
        Err(e) => {
            Err(e.into())
        }
    }
}
