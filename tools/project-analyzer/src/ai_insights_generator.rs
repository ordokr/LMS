use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use serde::{Serialize, Deserialize};
use serde_json::{self, Value};

use crate::ast_analyzer::AstAnalyzer;
use crate::file_system_utils::FileSystemUtils;
use crate::gemini_analyzer::GeminiAnalyzer;

#[derive(Debug, Serialize, Deserialize, Default)]
struct Metrics {
    models: ModelMetrics,
    api_endpoints: ApiEndpointMetrics,
    code_quality: CodeQualityMetrics,
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct ModelMetrics {
    details: Vec<ModelDetail>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct ModelDetail {
    name: String,
    file: String,
    completeness: u8,
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct ApiEndpointMetrics {
    details: Vec<ApiEndpointDetail>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct ApiEndpointDetail {
    path: String,
    method: String,
    handler: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct CodeQualityMetrics {
    complexity: ComplexityMetrics,
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct ComplexityMetrics {
    files: Vec<FileComplexity>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct FileComplexity {
    file: String,
    complexity: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AiInsightOptions {
    pub base_dir: Option<String>,
    pub gemini_api_key: Option<String>,
    pub file: Option<String>,
    pub limit: Option<usize>,
}

impl Default for AiInsightOptions {
    fn default() -> Self {
        Self {
            base_dir: None,
            gemini_api_key: Some("AIzaSyC2bS3qMexvmemLfg-V4ZQ-GbTcg-RgCQ8".to_string()),
            file: None,
            limit: Some(3),
        }
    }
}

/// Generate AI insights for the codebase
pub async fn generate_ai_insights(options: AiInsightOptions) -> Result<(), Box<dyn std::error::Error>> {
    let base_dir = options.base_dir.unwrap_or_else(|| std::env::current_dir()
        .expect("Failed to get current directory")
        .to_string_lossy()
        .to_string());
    
    println!("Generating AI insights for {}...", base_dir);
    
    // Create minimal metrics structure
    let mut metrics = Metrics::default();
    
    // Load saved metrics if available
    let metrics_path = Path::new(&base_dir).join(".analysis_cache").join("metrics.json");
    if metrics_path.exists() {
        match fs::read_to_string(&metrics_path) {
            Ok(content) => {
                match serde_json::from_str::<Metrics>(&content) {
                    Ok(loaded_metrics) => {
                        metrics = loaded_metrics;
                        println!("Loaded existing metrics data");
                    },
                    Err(err) => {
                        eprintln!("Could not parse metrics: {}", err);
                    }
                }
            },
            Err(err) => {
                eprintln!("Could not load metrics: {}", err);
            }
        }
    }
    
    // Initialize file system utils
    let exclude_patterns = vec![
        r"node_modules", r"\.git", r"dist", r"build"
    ];
    let mut fs_utils = FileSystemUtils::new(&base_dir, exclude_patterns);
    
    // Discover files
    fs_utils.discover_files();
    
    // Initialize Gemini analyzer
    let gemini_analyzer = GeminiAnalyzer::new(
        metrics,
        options.gemini_api_key.unwrap_or_else(|| "AIzaSyC2bS3qMexvmemLfg-V4ZQ-GbTcg-RgCQ8".to_string()),
        &base_dir
    );
    
    // Determine files to analyze
    let files_to_analyze = if let Some(file) = options.file {
        let file_path = Path::new(&base_dir).join(file);
        vec![file_path]
    } else {
        fs_utils.get_all_files()
            .iter()
            .filter(|file| {
                if let Some(ext) = file.extension() {
                    let ext_str = ext.to_string_lossy().to_lowercase();
                    [".js", ".ts", ".jsx", ".tsx"].contains(&ext_str.as_str())
                } else {
                    false
                }
            })
            .take(options.limit.unwrap_or(3))
            .cloned()
            .collect()
    };
    
    // Read content only for selected files
    fs_utils.read_file_contents();
    
    // Generate insights
    let insights = gemini_analyzer.generate_code_insights(&files_to_analyze, &fs_utils).await?;
    
    // Generate report
    let overview = gemini_analyzer.generate_project_overview().await?;
    
    // Save insights to report file
    let report_dir = Path::new(&base_dir).join("docs");
    if !report_dir.exists() {
        fs::create_dir_all(&report_dir)?;
    }
    
    let insights_path = report_dir.join("gemini_code_insights.md");
    fs::write(&insights_path, insights)?;
    println!("AI insights saved to {}", insights_path.display());
    
    let overview_path = report_dir.join("gemini_project_assessment.md");
    fs::write(&overview_path, overview)?;
    println!("Project assessment saved to {}", overview_path.display());
    
    Ok(())
}
