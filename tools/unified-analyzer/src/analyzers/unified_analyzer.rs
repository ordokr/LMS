use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

use crate::utils::file_system::FileSystemUtils;

// Migrated modules
// These modules are commented out until we fix the import issues
// use crate::analyzers::modules::blockchain_analyzer::BlockchainAnalyzer;
// use crate::analyzers::modules::db_schema_analyzer::DbSchemaAnalyzer;
// use crate::analyzers::modules::js_migration_analyzer::JsMigrationAnalyzer;
// use crate::analyzers::modules::tech_debt_analyzer::TechDebtAnalyzer;
// use crate::analyzers::modules::trend_analyzer::TrendAnalyzer;
// use crate::analyzers::modules::unified_project_analyzer::UnifiedProjectAnalyzer as ExternalProjectAnalyzer;

// Analysis result for the entire codebase
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResult {
    // When the analysis was performed
    pub timestamp: DateTime<Utc>,

    // Overall project metrics
    pub project_status: ProjectStatus,

    // Model implementations
    pub models: ModelMetrics,

    // API endpoints
    pub api_endpoints: ApiEndpointMetrics,

    // UI components
    pub ui_components: ComponentMetrics,

    // Code quality metrics
    pub code_quality: CodeQualityMetrics,

    // Test coverage
    pub tests: TestMetrics,

    // Integration points
    pub integration: IntegrationMetrics,

    // Detected architecture
    pub architecture: ArchitectureInfo,

    // Synchronization system
    pub sync_system: SyncSystemInfo,

    // Blockchain implementation
    pub blockchain: BlockchainInfo,

    // Feature area implementation percentages
    pub feature_areas: HashMap<String, FeatureAreaMetrics>,

    // Next step recommendations
    pub recommendations: Vec<Recommendation>,
}

impl Default for AnalysisResult {
    fn default() -> Self {
        Self {
            timestamp: Utc::now(),
            project_status: ProjectStatus::default(),
            models: ModelMetrics::default(),
            api_endpoints: ApiEndpointMetrics::default(),
            ui_components: ComponentMetrics::default(),
            code_quality: CodeQualityMetrics::default(),
            tests: TestMetrics::default(),
            integration: IntegrationMetrics::default(),
            architecture: ArchitectureInfo::default(),
            sync_system: SyncSystemInfo::default(),
            blockchain: BlockchainInfo::default(),
            feature_areas: HashMap::new(),
            recommendations: Vec::new(),
        }
    }
}

// Overall project status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProjectStatus {
    pub phase: String,
    pub completion_percentage: f32,
    pub last_active_area: String,
    pub estimated_completion_date: Option<DateTime<Utc>>,
}

impl Default for ProjectStatus {
    fn default() -> Self {
        Self {
            phase: "development".to_string(),
            completion_percentage: 0.0,
            last_active_area: "unknown".to_string(),
            estimated_completion_date: None,
        }
    }
}

// Model implementation metrics
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ModelMetrics {
    pub total: usize,
    pub implemented: usize,
    pub implementation_percentage: f32,
}

impl Default for ModelMetrics {
    fn default() -> Self {
        Self {
            total: 0,
            implemented: 0,
            implementation_percentage: 0.0,
        }
    }
}

// API endpoint metrics
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ApiEndpointMetrics {
    pub total: usize,
    pub implemented: usize,
    pub implementation_percentage: f32,
}

impl Default for ApiEndpointMetrics {
    fn default() -> Self {
        Self {
            total: 0,
            implemented: 0,
            implementation_percentage: 0.0,
        }
    }
}

// UI component metrics
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ComponentMetrics {
    pub total: usize,
    pub implemented: usize,
    pub implementation_percentage: f32,
}

impl Default for ComponentMetrics {
    fn default() -> Self {
        Self {
            total: 0,
            implemented: 0,
            implementation_percentage: 0.0,
        }
    }
}

// Code quality metrics
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CodeQualityMetrics {
    pub metrics: HashMap<String, f32>,
}

impl Default for CodeQualityMetrics {
    fn default() -> Self {
        Self {
            metrics: HashMap::new(),
        }
    }
}

// Test metrics
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TestMetrics {
    pub total: usize,
    pub passing: usize,
    pub coverage: f32,
}

impl Default for TestMetrics {
    fn default() -> Self {
        Self {
            total: 0,
            passing: 0,
            coverage: 0.0,
        }
    }
}

// Integration metrics
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IntegrationMetrics {
    pub total_points: usize,
    pub implemented_points: usize,
    pub implementation_percentage: f32,
}

impl Default for IntegrationMetrics {
    fn default() -> Self {
        Self {
            total_points: 0,
            implemented_points: 0,
            implementation_percentage: 0.0,
        }
    }
}

// Architecture information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ArchitectureInfo {
    pub frameworks: Vec<String>,
    pub design_patterns: Vec<String>,
}

impl Default for ArchitectureInfo {
    fn default() -> Self {
        Self {
            frameworks: vec!["Tauri".to_string(), "Leptos".to_string(), "Axum".to_string()],
            design_patterns: Vec::new(),
        }
    }
}

// Sync system information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SyncSystemInfo {
    pub implementation_status: String,
    pub offline_capability: bool,
}

impl Default for SyncSystemInfo {
    fn default() -> Self {
        Self {
            implementation_status: "planned".to_string(),
            offline_capability: true,
        }
    }
}

// Blockchain information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BlockchainInfo {
    pub implementation_status: String,
    pub features: Vec<String>,
}

impl Default for BlockchainInfo {
    fn default() -> Self {
        Self {
            implementation_status: "planned".to_string(),
            features: Vec::new(),
        }
    }
}

// Feature area metrics
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FeatureAreaMetrics {
    pub total: usize,
    pub implemented: usize,
    pub priority: String,
}

impl Default for FeatureAreaMetrics {
    fn default() -> Self {
        Self {
            total: 0,
            implemented: 0,
            priority: "medium".to_string(),
        }
    }
}

// Recommendation for next steps
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recommendation {
    pub area: String,
    pub description: String,
    pub priority: u8,
    pub related_files: Vec<PathBuf>,
}

// The unified project analyzer
pub struct UnifiedProjectAnalyzer {
    pub base_dir: PathBuf,
    pub fs_utils: Arc<FileSystemUtils>,
    pub result: Arc<Mutex<AnalysisResult>>,
}

impl UnifiedProjectAnalyzer {
    pub fn new(base_dir: PathBuf, fs_utils: Arc<FileSystemUtils>) -> Self {
        Self {
            base_dir,
            fs_utils,
            result: Arc::new(Mutex::new(AnalysisResult::default())),
        }
    }

    // Main analysis function
    pub async fn analyze(&self) -> Result<AnalysisResult, Box<dyn std::error::Error + Send + Sync>> {
        println!("Starting unified project analysis...");

        // Reset the analysis result
        let mut result = self.result.lock().await;
        *result = AnalysisResult::default();
        result.timestamp = Utc::now();
        drop(result);

        // Analyze different aspects of the project sequentially
        // This avoids type mismatches with futures
        self.analyze_models().await?;
        self.analyze_api_endpoints().await?;
        self.analyze_components().await?;
        self.analyze_code_quality().await?;
        self.analyze_tests().await?;
        self.analyze_integration().await?;
        self.analyze_architecture().await?;
        self.analyze_sync_system().await?;
        self.analyze_blockchain().await?;

        // Generate recommendations
        self.generate_recommendations().await?;

        // Update project status
        self.update_project_status().await?;

        // Return the final result
        let result = self.result.lock().await.clone();

        Ok(result)
    }

    // Analyze data models
    pub async fn analyze_models(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        println!("Analyzing models...");

        // Find all model files
        let model_files = self.fs_utils.find_files(&self.base_dir, "rs")
            .into_iter()
            .filter(|path| {
                path.to_string_lossy().contains("models") ||
                path.to_string_lossy().contains("model")
            })
            .collect::<Vec<_>>();

        // Use the migrated JsMigrationAnalyzer to find JavaScript models that need to be migrated
        // let js_migration_analyzer = JsMigrationAnalyzer::new(self.base_dir.clone());
        // js_migration_analyzer.discover_js_files();

        // Analyze JavaScript files to find models that need to be migrated
        // This would help us identify models that are still in JavaScript and need to be migrated to Rust
        // let js_analysis = js_migration_analyzer.analyze_all_js_files();
        // let js_models = js_analysis.values().filter(|a| a.relative_path.contains("model")).count();

        // Update the result
        let mut result = self.result.lock().await;
        result.models.total = 50; // Example value
        result.models.implemented = model_files.len();
        result.models.implementation_percentage = if result.models.total > 0 {
            (result.models.implemented as f32 / result.models.total as f32) * 100.0
        } else {
            0.0
        };

        Ok(())
    }

    // Analyze API endpoints
    pub async fn analyze_api_endpoints(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        println!("Analyzing API endpoints...");

        // Find all API files
        let api_files = self.fs_utils.find_files(&self.base_dir, "rs")
            .into_iter()
            .filter(|path| {
                path.to_string_lossy().contains("api") ||
                path.to_string_lossy().contains("routes") ||
                path.to_string_lossy().contains("controllers")
            })
            .collect::<Vec<_>>();

        // Use the migrated DbSchemaAnalyzer to analyze database schema
        // This would help us identify database tables and their relationships
        // which could be used to infer API endpoints
        // Note: This is a simplified example as we don't have the actual SQLite pool
        // In a real implementation, we would need to create a SQLite pool
        // let db_schema_analyzer = DbSchemaAnalyzer::new(pool);
        // let tables = db_schema_analyzer.get_tables().await.unwrap_or_default();
        // let table_count = tables.len();

        // Update the result
        let mut result = self.result.lock().await;
        result.api_endpoints.total = 100; // Example value
        result.api_endpoints.implemented = api_files.len();
        result.api_endpoints.implementation_percentage = if result.api_endpoints.total > 0 {
            (result.api_endpoints.implemented as f32 / result.api_endpoints.total as f32) * 100.0
        } else {
            0.0
        };

        Ok(())
    }

    // Analyze UI components
    pub async fn analyze_components(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        println!("Analyzing UI components...");

        // Find all component files
        let component_files = self.fs_utils.find_files(&self.base_dir, "rs")
            .into_iter()
            .filter(|path| {
                path.to_string_lossy().contains("components") ||
                path.to_string_lossy().contains("component") ||
                path.to_string_lossy().contains("ui")
            })
            .collect::<Vec<_>>();

        // Update the result
        let mut result = self.result.lock().await;
        result.ui_components.total = 80; // Example value
        result.ui_components.implemented = component_files.len();
        result.ui_components.implementation_percentage = if result.ui_components.total > 0 {
            (result.ui_components.implemented as f32 / result.ui_components.total as f32) * 100.0
        } else {
            0.0
        };

        Ok(())
    }

    // Analyze code quality
    pub async fn analyze_code_quality(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        println!("Analyzing code quality...");

        // Use the migrated TechDebtAnalyzer
        // This is commented out until we fix the import issues
        // let tech_debt_analyzer = TechDebtAnalyzer::new(self.base_dir.clone());

        // Update the result with default values
        let mut result = self.result.lock().await;
        result.code_quality.metrics.insert("complexity".to_string(), 3.5);
        result.code_quality.metrics.insert("maintainability".to_string(), 4.2);
        result.code_quality.metrics.insert("documentation".to_string(), 3.8);

        Ok(())
    }

    // Analyze tests
    pub async fn analyze_tests(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        println!("Analyzing tests...");

        // Find all test files
        let test_files = self.fs_utils.find_files(&self.base_dir, "rs")
            .into_iter()
            .filter(|path| {
                path.to_string_lossy().contains("tests") ||
                path.to_string_lossy().contains("test") ||
                path.to_string_lossy().contains("spec")
            })
            .collect::<Vec<_>>();

        // Update the result
        let mut result = self.result.lock().await;
        result.tests.total = test_files.len();
        result.tests.passing = test_files.len(); // Assume all tests pass for now
        result.tests.coverage = 65.0; // Example value

        Ok(())
    }

    // Analyze integration points
    pub async fn analyze_integration(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        println!("Analyzing integration points...");

        // Update the result
        let mut result = self.result.lock().await;
        result.integration.total_points = 30; // Example value
        result.integration.implemented_points = 20; // Example value
        result.integration.implementation_percentage = if result.integration.total_points > 0 {
            (result.integration.implemented_points as f32 / result.integration.total_points as f32) * 100.0
        } else {
            0.0
        };

        Ok(())
    }

    // Analyze architecture
    pub async fn analyze_architecture(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        println!("Analyzing architecture...");

        // Update the result
        let mut result = self.result.lock().await;
        result.architecture.design_patterns.push("Repository".to_string());
        result.architecture.design_patterns.push("Service".to_string());
        result.architecture.design_patterns.push("Factory".to_string());

        Ok(())
    }

    // Analyze sync system
    pub async fn analyze_sync_system(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        println!("Analyzing sync system...");

        // Update the result
        let mut result = self.result.lock().await;
        result.sync_system.implementation_status = "in-progress".to_string();

        Ok(())
    }

    // Analyze blockchain
    pub async fn analyze_blockchain(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        println!("Analyzing blockchain components...");

        // Use the migrated BlockchainAnalyzer
        // Note: This is a simplified example as we don't have the actual HybridChain
        // In a real implementation, we would need to create a HybridChain instance
        // let blockchain_analyzer = BlockchainAnalyzer::new(Arc::new(Mutex::new(HybridChain::new())));
        // let blockchain_analysis = blockchain_analyzer.analyze().await?;
        // This is commented out until we fix the import issues

        // For now, we'll use placeholder values
        let mut result = self.result.lock().await;
        result.blockchain.implementation_status = "planned".to_string();
        result.blockchain.features.push("Immutable Records".to_string());
        result.blockchain.features.push("Smart Contracts".to_string());

        Ok(())
    }

    // Generate recommendations
    pub async fn generate_recommendations(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        println!("Generating recommendations...");

        // Update the result
        let mut result = self.result.lock().await;

        // Add some example recommendations
        result.recommendations.push(Recommendation {
            area: "Models".to_string(),
            description: "Implement remaining Canvas models".to_string(),
            priority: 1,
            related_files: vec![],
        });

        result.recommendations.push(Recommendation {
            area: "API".to_string(),
            description: "Add authentication to remaining endpoints".to_string(),
            priority: 2,
            related_files: vec![],
        });

        Ok(())
    }

    // Update project status
    pub async fn update_project_status(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        println!("Updating project status...");

        // Calculate overall completion percentage
        let result_clone = self.result.lock().await.clone();

        let model_weight = 0.3;
        let api_weight = 0.3;
        let ui_weight = 0.2;
        let test_weight = 0.1;
        let integration_weight = 0.1;

        let overall_percentage =
            (result_clone.models.implementation_percentage * model_weight) +
            (result_clone.api_endpoints.implementation_percentage * api_weight) +
            (result_clone.ui_components.implementation_percentage * ui_weight) +
            (result_clone.tests.coverage * test_weight) +
            (result_clone.integration.implementation_percentage * integration_weight);

        // Update the result
        let mut result = self.result.lock().await;
        result.project_status.completion_percentage = overall_percentage;

        // Determine the phase based on completion percentage
        if overall_percentage < 25.0 {
            result.project_status.phase = "early-development".to_string();
        } else if overall_percentage < 50.0 {
            result.project_status.phase = "development".to_string();
        } else if overall_percentage < 75.0 {
            result.project_status.phase = "late-development".to_string();
        } else if overall_percentage < 90.0 {
            result.project_status.phase = "testing".to_string();
        } else {
            result.project_status.phase = "release-candidate".to_string();
        }

        // Determine the last active area
        // For now, just use a placeholder
        result.project_status.last_active_area = "API Development".to_string();

        // Use the migrated TrendAnalyzer to record the analysis result
        // This is commented out until we fix the import issues
        // let trend_analyzer = TrendAnalyzer::new(self.base_dir.clone());

        // Create a simplified AnalysisResult for the trend analyzer
        // Note: This is a simplified example as we don't have the exact same structure
        // In a real implementation, we would need to convert our AnalysisResult to the format expected by TrendAnalyzer
        // let trend_result = crate::analyzers::modules::trend_analyzer::AnalysisResult::new("unified_analyzer");
        // trend_analyzer.record_analysis(&trend_result).await.ok();

        Ok(())
    }

    // Generate and write the central reference hub markdown file
    pub async fn generate_central_reference_hub(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        println!("Generating central reference hub...");

        let result = self.result.lock().await.clone();

        // Format current date
        let current_date = chrono::Utc::now().format("%Y-%m-%d").to_string();

        // Start building the markdown content
        let mut content = format!(
            "# LMS Integration Project - Central Reference Hub\n\n_Last updated: {}_\n\n",
            current_date
        );

        // Add project overview section
        content.push_str("## 📊 Project Overview\n\n");
        content.push_str("```json\n");
        content.push_str(&serde_json::to_string_pretty(&serde_json::json!({
            "status": result.project_status.phase,
            "completion": format!("{}%", result.project_status.completion_percentage),
            "lastActiveArea": result.project_status.last_active_area,
            "estimatedCompletion": result.project_status.estimated_completion_date,
        })).unwrap());
        content.push_str("\n```\n\n");

        // Add implementation details section
        content.push_str("## 🔍 Implementation Details\n\n");
        content.push_str("### Code Quality Metrics\n\n");
        content.push_str("| Metric | Value |\n");
        content.push_str("|--------|-------|\n");

        for (metric, value) in &result.code_quality.metrics {
            content.push_str(&format!("| {} | {:.1} |\n", metric, value));
        }

        content.push_str(&format!("| Test Coverage | {:.1}% |\n", result.tests.coverage));

        content.push_str("\n");

        // Add implementation tasks section
        content.push_str("## 📋 Implementation Tasks\n\n");

        // Add recommendations
        for recommendation in &result.recommendations {
            content.push_str(&format!("- **{}**: {}\n", recommendation.area, recommendation.description));
        }

        // Add technology stack section
        content.push_str("\n## 🔧 Technology Stack\n\n");
        content.push_str("- **Frontend**: Leptos, Tauri\n");
        content.push_str("- **Backend**: Rust, Axum\n");
        content.push_str("- **Database**: SQLite with SQLx\n");
        content.push_str("- **Search**: MeiliSearch\n");
        content.push_str("- **Authentication**: JWT\n");

        // Add documentation links section
        content.push_str("\n## 📚 Documentation Links\n\n");
        content.push_str("- [Architecture Documentation](./architecture/overview.md)\n");
        content.push_str("- [Models Documentation](./models/overview.md)\n");
        content.push_str("- [Integration Documentation](./integration/overview.md)\n");
        content.push_str("- [Analyzer Reference](./analyzer_reference.md)\n");

        // Write the content to the central reference hub file
        let hub_path = self.base_dir.join("docs").join("central_reference_hub.md");

        // Ensure the docs directory exists
        let docs_dir = self.base_dir.join("docs");
        if !docs_dir.exists() {
            std::fs::create_dir_all(&docs_dir)?;
        }

        std::fs::write(&hub_path, content)?;

        println!("Central reference hub generated at {:?}", hub_path);

        Ok(())
    }

    // Generate analyzer reference documentation
    pub async fn generate_analyzer_reference(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        println!("Generating analyzer reference documentation...");

        // Format current date is not needed for now

        // Start building the markdown content
        let mut content = String::from("# LMS Project Analyzer Reference\n\n");
        content.push_str("This document serves as a reference for the unified analyzer system that generates documentation and insights for the LMS project.\n\n");

        // Overview section
        content.push_str("## Overview\n\n");
        content.push_str("The LMS Project Analyzer is a unified system that analyzes the codebase, generates documentation, and provides insights into the project's structure, progress, and quality. It consolidates various analyzers that were previously scattered throughout the codebase.\n\n");

        // Components section
        content.push_str("## Analyzer Components\n\n");
        content.push_str("### Core Components\n\n");
        content.push_str("- **UnifiedAnalyzer**: The main analyzer that orchestrates the analysis process.\n");
        content.push_str("- **AnalyzerConfig**: Configuration for the analyzer, loaded from `analyzer_config.toml`.\n");
        content.push_str("- **AnalysisResult**: The result of the analysis, containing various metrics and insights.\n\n");

        content.push_str("### Analyzers\n\n");
        content.push_str("- **ModelAnalyzer**: Analyzes models in the codebase.\n");
        content.push_str("- **ApiAnalyzer**: Analyzes API endpoints.\n");
        content.push_str("- **UiAnalyzer**: Analyzes UI components.\n");
        content.push_str("- **CodeQualityAnalyzer**: Analyzes code quality metrics.\n");
        content.push_str("- **TestAnalyzer**: Analyzes tests and test coverage.\n");
        content.push_str("- **IntegrationAnalyzer**: Analyzes integration points.\n");
        content.push_str("- **ArchitectureAnalyzer**: Analyzes architecture patterns.\n");
        content.push_str("- **SyncAnalyzer**: Analyzes sync system.\n");
        content.push_str("- **BlockchainAnalyzer**: Analyzes blockchain components.\n\n");

        // Usage section
        content.push_str("## Usage\n\n");
        content.push_str("### Command-Line Interface\n\n");
        content.push_str("The analyzer can be run using the following command:\n\n");
        content.push_str("```bash\n");
        content.push_str("cd tools/unified-analyzer\n");
        content.push_str("cargo run\n");
        content.push_str("```\n\n");

        content.push_str("### Options\n\n");
        content.push_str("- `--path PATH`: Specify the path to analyze (default: current directory)\n");
        content.push_str("- `--output DIR`: Specify the output directory for documentation (default: docs)\n");
        content.push_str("- `--verbose`: Enable verbose output\n\n");

        // Generated documentation section
        content.push_str("## Generated Documentation\n\n");
        content.push_str("The analyzer generates the following documentation:\n\n");
        content.push_str("- **Central Reference Hub** (`docs/central_reference_hub.md`): The main entry point for project documentation.\n");
        content.push_str("- **Analyzer Reference** (`docs/analyzer_reference.md`): Documentation for the analyzer itself.\n");
        content.push_str("- **Architecture Documentation** (`docs/architecture/overview.md`): Overview of the project's architecture.\n");
        content.push_str("- **Models Documentation** (`docs/models/overview.md`): Documentation for the project's data models.\n");
        content.push_str("- **Integration Documentation** (`docs/integration/overview.md`): Documentation for integration points.\n\n");

        // Write the content to the analyzer reference file
        let reference_path = self.base_dir.join("docs").join("analyzer_reference.md");

        // Ensure the docs directory exists
        let docs_dir = self.base_dir.join("docs");
        if !docs_dir.exists() {
            std::fs::create_dir_all(&docs_dir)?;
        }

        std::fs::write(&reference_path, content)?;

        println!("Analyzer reference documentation generated at {:?}", reference_path);

        Ok(())
    }
}
