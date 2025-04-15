use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;
use std::time::Instant;
use serde::{Serialize, Deserialize};
use regex::Regex;

use crate::file_system_utils::FileSystemUtils;
use crate::analysis_utils::AnalysisUtils;
use crate::ast_analyzer::AstAnalyzer;
use crate::project_predictor::ProjectPredictor;

// Import modular analyzers
use crate::modules::solid_analyzer::SolidAnalyzer;
use crate::modules::pattern_analyzer::PatternAnalyzer;
use crate::modules::source_analyzer::SourceAnalyzer;
use crate::modules::report_generator::ReportGenerator;
use crate::modules::ml_analyzer::MLAnalyzer;
use crate::modules::visual_report_generator::VisualReportGenerator;
use crate::modules::github_analyzer::GitHubAnalyzer;
use crate::modules::gemini_analyzer::GeminiAnalyzer;
use crate::modules::vector_database_adapter::VectorDatabaseAdapter;
use crate::modules::rag_retriever::RagRetriever;
use crate::modules::integration_report_generator::IntegrationReportGenerator;
use crate::modules::technical_docs_generator::TechnicalDocsGenerator;

/// Options for the UnifiedProjectAnalyzer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyzerOptions {
    pub use_cache: bool,
    pub implementation_threshold: u8,
    pub gemini_api_key: Option<String>,
    pub incremental: bool,
    pub skip_reports: bool,
    pub skip_relationship_maps: bool,
    pub skip_performance_analysis: bool,
}

impl Default for AnalyzerOptions {
    fn default() -> Self {
        Self {
            use_cache: true,
            implementation_threshold: 35,
            gemini_api_key: Some("AIzaSyC2bS3qMexvmemLfg-V4ZQ-GbTcg-RgCQ8".to_string()),
            incremental: false,
            skip_reports: false,
            skip_relationship_maps: false, 
            skip_performance_analysis: false,
        }
    }
}

/// Source system configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceSystem {
    pub name: String,
    pub path: PathBuf,
}

/// Metrics structure for project analysis
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Metrics {
    pub models: ModelMetrics,
    pub api_endpoints: ApiEndpointMetrics,
    pub ui_components: UiComponentMetrics,
    pub code_quality: CodeQualityMetrics,
    pub tests: TestMetrics,
    pub overall_status: OverallStatus,
    pub overall_phase: String,
    pub tech_debt: TechDebtMetrics,
    pub relationships: Vec<Relationship>,
    pub performance: Option<PerformanceMetrics>,
    pub source_systems: HashMap<String, SourceSystemMetrics>,
    pub source_to_target: SourceToTargetMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModelMetrics {
    pub total: usize,
    pub implemented: usize,
    pub details: Vec<ModelDetail>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelDetail {
    pub name: String,
    pub file: String,
    pub completeness: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApiEndpointMetrics {
    pub total: usize,
    pub implemented: usize,
    pub details: Vec<ApiEndpointDetail>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiEndpointDetail {
    pub path: String,
    pub method: String,
    pub handler: String,
    pub completeness: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UiComponentMetrics {
    pub total: usize,
    pub implemented: usize,
    pub details: Vec<UiComponentDetail>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiComponentDetail {
    pub name: String,
    pub file: String,
    pub completeness: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CodeQualityMetrics {
    pub complexity: ComplexityMetrics,
    pub tech_debt: Vec<TechDebtItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComplexityMetrics {
    pub files: Vec<FileComplexity>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileComplexity {
    pub file: String,
    pub complexity: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TestMetrics {
    pub total: usize,
    pub passing: usize,
    pub coverage: f32,
    pub details: Vec<TestDetail>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestDetail {
    pub name: String,
    pub file: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OverallStatus {
    pub models: String,
    pub api: String,
    pub ui: String,
    pub tests: String,
    pub tech_debt: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TechDebtMetrics {
    pub total_items: usize,
    pub high_priority: usize,
    pub medium_priority: usize,
    pub low_priority: usize,
    pub details: Vec<TechDebtItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TechDebtItem {
    pub file: String,
    pub description: String,
    pub priority: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relationship {
    pub from: String,
    pub to: String,
    pub r#type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PerformanceMetrics {
    pub start_time: u64,
    pub steps: HashMap<String, u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SourceSystemMetrics {
    pub models: ModelMetrics,
    pub controllers: ControllerMetrics,
    pub files_by_type: HashMap<String, Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ControllerMetrics {
    pub total: usize,
    pub details: Vec<ControllerDetail>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControllerDetail {
    pub name: String,
    pub file: String,
    pub actions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SourceToTargetMetrics {
    pub models: Vec<ModelMapping>,
    pub controllers: Vec<ControllerMapping>,
    pub components: Vec<ComponentMapping>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelMapping {
    pub source_model: String,
    pub target_model: String,
    pub completeness: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControllerMapping {
    pub source_controller: String,
    pub target_handler: String,
    pub completeness: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentMapping {
    pub source_component: String,
    pub target_component: String,
    pub completeness: u8,
}

/// Unified Project Analyzer
/// Consolidates all analyzer functionality into a single tool
pub struct UnifiedProjectAnalyzer {
    base_dir: PathBuf,
    source_systems: HashMap<String, SourceSystem>,
    options: AnalyzerOptions,
    metrics: Metrics,
    fs_utils: FileSystemUtils,
    analysis_utils: AnalysisUtils,
    ast_analyzer: AstAnalyzer,
    predictor: ProjectPredictor,
    
    // Modular analyzers
    solid_analyzer: SolidAnalyzer,
    pattern_analyzer: PatternAnalyzer,
    source_analyzer: SourceAnalyzer,
    report_generator: ReportGenerator,
    ml_analyzer: MLAnalyzer,
    visual_report_generator: VisualReportGenerator,
    github_analyzer: GitHubAnalyzer,
    gemini_analyzer: GeminiAnalyzer,
    vector_db_adapter: VectorDatabaseAdapter,
    rag_retriever: RagRetriever,
    integration_report_generator: IntegrationReportGenerator,
    technical_docs_generator: TechnicalDocsGenerator,
}

impl UnifiedProjectAnalyzer {
    /// Create a new UnifiedProjectAnalyzer instance
    pub fn new(
        base_dir: String, 
        source_systems: Option<HashMap<String, SourceSystem>>,
        use_cache: bool,
    ) -> Self {
        let options = AnalyzerOptions {
            use_cache,
            ..Default::default()
        };
        
        let sources = source_systems.unwrap_or_default();
        let metrics = Self::initialize_metrics();
        let exclude_patterns = Self::get_exclude_patterns();
        
        let fs_utils = FileSystemUtils::new(&base_dir, exclude_patterns.clone());
        let analysis_utils = AnalysisUtils::new(&base_dir, &fs_utils, &options, &metrics);
        let ast_analyzer = AstAnalyzer::new();
        let predictor = ProjectPredictor::new(&metrics);
        
        // Initialize modular analyzers
        let solid_analyzer = SolidAnalyzer::new(&metrics);
        let pattern_analyzer = PatternAnalyzer::new(&metrics);
        let source_analyzer = SourceAnalyzer::new(&metrics, &exclude_patterns);
        let report_generator = ReportGenerator::new(&metrics, &base_dir);
        let ml_analyzer = MLAnalyzer::new(&metrics);
        let visual_report_generator = VisualReportGenerator::new(&metrics, &base_dir);
        let github_analyzer = GitHubAnalyzer::new(&metrics);
        let gemini_analyzer = GeminiAnalyzer::new(
            metrics.clone(),
            options.gemini_api_key.clone().unwrap_or_default(),
            &base_dir,
        );
        let vector_db_adapter = VectorDatabaseAdapter::new(&base_dir);
        let rag_retriever = RagRetriever::new(&base_dir);
        let integration_report_generator = IntegrationReportGenerator::new(&metrics, &base_dir);
        let technical_docs_generator = TechnicalDocsGenerator::new(&metrics, &base_dir);
        
        Self {
            base_dir: Path::new(&base_dir).to_path_buf(),
            source_systems: sources,
            options,
            metrics,
            fs_utils,
            analysis_utils,
            ast_analyzer,
            predictor,
            solid_analyzer,
            pattern_analyzer,
            source_analyzer,
            report_generator,
            ml_analyzer,
            visual_report_generator,
            github_analyzer,
            gemini_analyzer,
            vector_db_adapter,
            rag_retriever,
            integration_report_generator,
            technical_docs_generator,
        }
    }
    
    /// Initialize metrics structure
    fn initialize_metrics() -> Metrics {
        Metrics {
            models: ModelMetrics::default(),
            api_endpoints: ApiEndpointMetrics::default(),
            ui_components: UiComponentMetrics::default(),
            code_quality: CodeQualityMetrics::default(),
            tests: TestMetrics::default(),
            overall_status: OverallStatus {
                models: "Planning".to_string(),
                api: "Planning".to_string(),
                ui: "Planning".to_string(),
                tests: "Planning".to_string(),
                tech_debt: "Low".to_string(),
            },
            overall_phase: "Planning".to_string(),
            tech_debt: TechDebtMetrics::default(),
            relationships: Vec::new(),
            performance: Some(PerformanceMetrics {
                start_time: chrono::Utc::now().timestamp_millis() as u64,
                steps: HashMap::new(),
            }),
            source_systems: HashMap::new(),
            source_to_target: SourceToTargetMetrics::default(),
        }
    }
    
    /// Get patterns to exclude from analysis
    fn get_exclude_patterns() -> Vec<&'static str> {
        vec![
            r"node_modules",
            r"\.git",
            r"\.github",
            r"build",
            r"dist",
            r"coverage",
            r"\.DS_Store",
            r"\.vscode",
            r"\.idea",
            r"target",
            r"\.next",
            r"\.nuxt",
        ]
    }
    
    /// Enhanced run method with incremental analysis and performance tracking
    pub async fn analyze(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Starting analysis of {}...", self.base_dir.display());
        let start_time = Instant::now();
        
        // Initialize performance tracking
        let mut metrics = self.metrics.clone();
        metrics.performance = Some(PerformanceMetrics {
            start_time: chrono::Utc::now().timestamp_millis() as u64,
            steps: HashMap::new(),
        });
        
        // Check if this is an incremental analysis
        // let changed_files = if self.options.incremental {
        //     self.perform_incremental_analysis()?
        // } else {
        //     None
        // };
        
        // File discovery (with tracking)
        let start_file_discovery = Instant::now();
        // if changed_files.is_none() {
        //     self.fs_utils.discover_files();
        //     self.fs_utils.read_file_contents();
        // }
        if let Some(perf) = &mut metrics.performance {
            perf.steps.insert(
                "fileDiscovery".to_string(), 
                start_file_discovery.elapsed().as_millis() as u64
            );
        }
        
        // Core analysis (with tracking)
        let start_model_analysis = Instant::now();
        // await self.analysis_utils.analyze_models(changed_files.as_ref());
        if let Some(perf) = &mut metrics.performance {
            perf.steps.insert(
                "modelAnalysis".to_string(),
                start_model_analysis.elapsed().as_millis() as u64
            );
        }
        
        // API endpoint analysis
        let start_api_analysis = Instant::now();
        // await self.analysis_utils.analyze_api_endpoints(changed_files.as_ref());
        if let Some(perf) = &mut metrics.performance {
            perf.steps.insert(
                "apiAnalysis".to_string(),
                start_api_analysis.elapsed().as_millis() as u64
            );
        }
        
        // UI component analysis
        let start_ui_analysis = Instant::now();
        // await self.analysis_utils.analyze_ui_components(changed_files.as_ref());
        if let Some(perf) = &mut metrics.performance {
            perf.steps.insert(
                "uiAnalysis".to_string(),
                start_ui_analysis.elapsed().as_millis() as u64
            );
        }
        
        // Test analysis
        let start_test_analysis = Instant::now();
        // await self.analysis_utils.analyze_tests(changed_files.as_ref());
        if let Some(perf) = &mut metrics.performance {
            perf.steps.insert(
                "testAnalysis".to_string(),
                start_test_analysis.elapsed().as_millis() as u64
            );
        }
        
        // Find relationships
        if !self.options.skip_relationship_maps {
            let start_relationships = Instant::now();
            // await self.analysis_utils.find_model_relationships();
            if let Some(perf) = &mut metrics.performance {
                perf.steps.insert(
                    "relationships".to_string(),
                    start_relationships.elapsed().as_millis() as u64
                );
            }
        }
        
        // Quality & tech debt analysis
        let start_quality_analysis = Instant::now();
        // await self.analysis_utils.analyze_code_quality(changed_files.as_ref());
        if let Some(perf) = &mut metrics.performance {
            perf.steps.insert(
                "qualityAnalysis".to_string(),
                start_quality_analysis.elapsed().as_millis() as u64
            );
        }
        
        // Generate reports
        if !self.options.skip_reports {
            let start_reports = Instant::now();
            // await self.generate_all_reports();
            if let Some(perf) = &mut metrics.performance {
                perf.steps.insert(
                    "reports".to_string(),
                    start_reports.elapsed().as_millis() as u64
                );
            }
        }
        
        // Generate summary
        // self.print_summary();
        
        // Save analysis results for future runs
        // await self.save_analysis_results();
        
        let elapsed = start_time.elapsed();
        println!("Analysis completed in {:.2} seconds", elapsed.as_secs_f32());
        
        Ok(())
    }
    
    /// Only analyze files that have changed since the last analysis
    async fn perform_incremental_analysis(&self) -> Result<Option<Vec<PathBuf>>, Box<dyn std::error::Error>> {
        let cache_dir = self.base_dir.join(".analysis_cache");
        if !cache_dir.exists() {
            fs::create_dir_all(&cache_dir)?;
        }
        
        let last_run_file = cache_dir.join("last_run.json");
        let last_run = if last_run_file.exists() {
            match fs::read_to_string(&last_run_file) {
                Ok(content) => {
                    match serde_json::from_str::<serde_json::Value>(&content) {
                        Ok(json) => Some(json),
                        Err(err) => {
                            eprintln!("Could not parse last run data: {}", err);
                            None
                        }
                    }
                },
                Err(err) => {
                    eprintln!("Could not read last run data: {}", err);
                    None
                }
            }
        } else {
            None
        };
        
        // Discover files
        // self.fs_utils.discover_files();
        
        // Filter for changed/new files only
        let changed_files = if let Some(last_run) = last_run {
            let files = vec![]; // TODO: Implement file comparison logic
            println!("Found {} changed/new files out of {} total files", 
                files.len(), 
                0 // self.fs_utils.get_all_files().len()
            );
            
            // Only read contents of changed files
            // self.fs_utils.read_file_contents(&files);
            
            // Record this run's file timestamps
            let current_run = serde_json::json!({
                "timestamp": chrono::Utc::now().timestamp_millis(),
                "files": {}
            });
            
            // Save current run data
            match fs::write(&last_run_file, serde_json::to_string_pretty(&current_run)?) {
                Ok(_) => {}
                Err(err) => {
                    eprintln!("Could not save run data: {}", err);
                }
            }
            
            Some(files)
        } else {
            None
        };
        
        Ok(changed_files)
    }
    
    /// Generate all reports
    async fn generate_all_reports(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Generating reports...");
        
        // Create reports directory if it doesn't exist
        let reports_dir = self.base_dir.join("reports");
        if !reports_dir.exists() {
            fs::create_dir_all(&reports_dir)?;
        }
        
        // Generate model report
        let models_report_path = reports_dir.join("models_report.md");
        // let models_report = self.report_generator.generate_models_report();
        // fs::write(&models_report_path, models_report)?;
        println!("Model report saved to {}", models_report_path.display());
        
        // Generate API report
        let api_report_path = reports_dir.join("api_report.md");
        // let api_report = self.report_generator.generate_api_report();
        // fs::write(&api_report_path, api_report)?;
        println!("API report saved to {}", api_report_path.display());
        
        // Generate UI components report
        let ui_report_path = reports_dir.join("ui_components_report.md");
        // let ui_report = self.report_generator.generate_ui_report();
        // fs::write(&ui_report_path, ui_report)?;
        println!("UI components report saved to {}", ui_report_path.display());
        
        // Generate code quality report
        let quality_report_path = reports_dir.join("code_quality_report.md");
        // let quality_report = self.report_generator.generate_code_quality_report();
        // fs::write(&quality_report_path, quality_report)?;
        println!("Code quality report saved to {}", quality_report_path.display());
        
        // Generate tech debt report
        let tech_debt_report_path = reports_dir.join("tech_debt_report.md");
        // let tech_debt_report = self.report_generator.generate_tech_debt_report();
        // fs::write(&tech_debt_report_path, tech_debt_report)?;
        println!("Tech debt report saved to {}", tech_debt_report_path.display());
        
        // Generate tests report
        let tests_report_path = reports_dir.join("tests_report.md");
        // let tests_report = self.report_generator.generate_tests_report();
        // fs::write(&tests_report_path, tests_report)?;
        println!("Tests report saved to {}", tests_report_path.display());
        
        // Generate relationship map
        if !self.options.skip_relationship_maps {
            await self.generate_relationship_maps()?;
        }
        
        // Generate central reference hub
        let central_hub_path = self.base_dir.join("docs").join("central_reference_hub.md");
        // let central_hub = self.integration_report_generator.generate_central_reference_hub();
        // fs::write(&central_hub_path, central_hub)?;
        println!("Central reference hub saved to {}", central_hub_path.display());
        
        // Generate visual dashboard
        let dashboard_path = self.base_dir.join("docs").join("dashboard.html");
        // let dashboard = self.visual_report_generator.generate_dashboard();
        // fs::write(&dashboard_path, dashboard)?;
        println!("Visual dashboard saved to {}", dashboard_path.display());
        
        // Generate AI-based reports if Gemini API key is available
        if self.options.gemini_api_key.is_some() {
            // Generate Gemini code insights
            let gemini_insights_path = self.base_dir.join("docs").join("gemini_code_insights.md");
            // let gemini_insights = await self.gemini_analyzer.generate_code_insights();
            // fs::write(&gemini_insights_path, gemini_insights)?;
            println!("Gemini code insights saved to {}", gemini_insights_path.display());
            
            // Generate Gemini project assessment
            let gemini_assessment_path = self.base_dir.join("docs").join("gemini_project_assessment.md");
            // let gemini_assessment = await self.gemini_analyzer.generate_project_assessment();
            // fs::write(&gemini_assessment_path, gemini_assessment)?;
            println!("Gemini project assessment saved to {}", gemini_assessment_path.display());
            
            // Generate Gemini mapping analysis
            let gemini_mapping_path = self.base_dir.join("docs").join("gemini_mapping_analysis.md");
            // let gemini_mapping = await self.gemini_analyzer.generate_mapping_analysis();
            // fs::write(&gemini_mapping_path, gemini_mapping)?;
            println!("Gemini mapping analysis report saved to {}", gemini_mapping_path.display());
        }
        
        // Generate index.html to link all reports
        // self.generate_report_index();
        
        // Update integration documentation
        // await self.update_integration_documentation();
        
        println!("All reports generated successfully");
        
        Ok(())
    }
    
    /// Generate relationship maps using Mermaid syntax
    async fn generate_relationship_maps(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Generating relationship maps...");
        
        let docs_dir = self.base_dir.join("docs");
        if !docs_dir.exists() {
            fs::create_dir_all(&docs_dir)?;
        }
        
        // Delegate the relationship detection to analysis_utils
        // if self.analysis_utils.find_model_relationships().is_ok() {
        //     println!("Model relationships identified");
        // }
        
        // Generate the diagram
        let mut mermaid_diagram = String::from("graph LR\n");
        // let nodes = HashSet::new();
        // for rel in &self.metrics.relationships {
        //     nodes.insert(&rel.from);
        //     nodes.insert(&rel.to);
        //     let arrow = if rel.r#type == "OneToMany" { "-->|1..*|" } else { "-->" };
        //     mermaid_diagram.push_str(&format!("  {}{}{}\n", rel.from, arrow, rel.to));
        // }
        
        // Add nodes that might not have relationships yet
        // for model in &self.metrics.models.details {
        //     nodes.insert(&model.name);
        // }
        
        // Add styles for nodes
        // for node in nodes {
        //     let model = self.metrics.models.details.iter().find(|m| m.name == *node);
        //     let completeness = model.map(|m| m.completeness).unwrap_or(0);
        //     let style = if completeness >= 75 {
        //         "fill:#c8e6c9,stroke:#2e7d32,stroke-width:2px" // Green
        //     } else if completeness >= 40 {
        //         "fill:#fff9c4,stroke:#fbc02d,stroke-width:1px" // Yellow
        //     } else if completeness > 0 {
        //         "fill:#ffcdd2,stroke:#c62828,stroke-width:1px" // Red
        //     } else {
        //         "fill:#eee,stroke:#333,stroke-width:1px" // Gray
        //     };
        //     mermaid_diagram.push_str(&format!("  style {} {}\n", node, style));
        // }
        
        // Save to a file
        let map_content = format!("# Model Relationship Map\n\n```mermaid\n{}\n```\n", mermaid_diagram);
        let map_path = docs_dir.join("relationship_map.md");
        fs::write(&map_path, map_content)?;
        println!("Relationship map saved to docs/relationship_map.md");
        
        Ok(())
    }
    
    /// Print analysis summary to console
    fn print_summary(&self) {
        println!(
            "Project Status: Models={}, API={}, UI={}, Tests={}, Debt={}",
            self.metrics.overall_status.models,
            self.metrics.overall_status.api,
            self.metrics.overall_status.ui,
            self.metrics.overall_status.tests,
            self.metrics.overall_status.tech_debt
        );
        println!("Overall Phase: {}", self.metrics.overall_phase);
        
        // Print file statistics
        // let file_stats = self.fs_utils.get_file_stats();
        // println!(
        //     "Processed {} files ({} JS/TS, {} Rust, {} other)",
        //     file_stats.total, file_stats.js, file_stats.rust, file_stats.other
        // );
        
        // Print model statistics
        println!(
            "Found {} models, {} API endpoints, {} UI components",
            self.metrics.models.total,
            self.metrics.api_endpoints.total,
            self.metrics.ui_components.total
        );
        
        // Print performance stats
        if let Some(performance) = &self.metrics.performance {
            let total_time = chrono::Utc::now().timestamp_millis() as u64 - performance.start_time;
            println!("Analysis completed in {:.2} seconds", total_time as f32 / 1000.0);
            
            if !performance.steps.is_empty() {
                println!("Performance breakdown:");
                for (step, time) in &performance.steps {
                    println!("  - {}: {:.2} seconds", step, *time as f32 / 1000.0);
                }
            }
        }
    }
}
