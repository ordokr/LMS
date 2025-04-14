use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::path::PathBuf;
use chrono::{DateTime, Local};

/// Result of a project analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResult {
    /// Timestamp of the analysis
    pub timestamp: DateTime<Local>,
    
    /// Project summary statistics
    pub summary: ProjectSummary,
    
    /// Code metrics
    pub code_metrics: CodeMetrics,
    
    /// Model metrics
    pub models: ModelMetrics,
    
    /// API endpoint metrics
    pub api_endpoints: ApiEndpointMetrics,
    
    /// UI component metrics
    pub ui_components: UiComponentMetrics,
    
    /// Feature area metrics
    pub feature_areas: HashMap<String, FeatureAreaMetrics>,
    
    /// Technical debt metrics
    pub tech_debt_metrics: TechDebtMetrics,
    
    /// Recent changes
    pub recent_changes: Vec<String>,
    
    /// Next steps
    pub next_steps: Vec<String>,
    
    /// Overall progress percentage
    pub overall_progress: f32,
}

/// Project summary statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectSummary {
    /// Total number of files
    pub total_files: usize,
    
    /// Total lines of code
    pub lines_of_code: usize,
    
    /// Files by extension
    pub file_types: HashMap<String, usize>,
    
    /// Number of Rust files
    pub rust_files: usize,
    
    /// Number of Haskell files
    pub haskell_files: usize,
}

/// Code metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeMetrics {
    /// Average cyclomatic complexity
    pub avg_complexity: f32,
    
    /// Number of functions
    pub function_count: usize,
    
    /// Number of modules
    pub module_count: usize,
    
    /// Number of structs
    pub struct_count: usize,
    
    /// Number of enums
    pub enum_count: usize,
    
    /// Number of traits
    pub trait_count: usize,
    
    /// Number of implementations
    pub impl_count: usize,
}

/// Model metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelMetrics {
    /// Total number of models
    pub total: usize,
    
    /// Number of implemented models
    pub implemented: usize,
    
    /// Implementation percentage
    pub implementation_percentage: f32,
    
    /// List of models
    pub models: Vec<Model>,
}

/// API endpoint metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiEndpointMetrics {
    /// Total number of API endpoints
    pub total: usize,
    
    /// Number of implemented API endpoints
    pub implemented: usize,
    
    /// Implementation percentage
    pub implementation_percentage: f32,
    
    /// List of API endpoints
    pub endpoints: Vec<ApiEndpoint>,
}

/// UI component metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiComponentMetrics {
    /// Total number of UI components
    pub total: usize,
    
    /// Number of implemented UI components
    pub implemented: usize,
    
    /// Implementation percentage
    pub implementation_percentage: f32,
    
    /// List of UI components
    pub components: Vec<UiComponent>,
}

/// Feature area metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureAreaMetrics {
    /// Total number of features
    pub total: usize,
    
    /// Number of implemented features
    pub implemented: usize,
    
    /// Implementation percentage
    pub implementation_percentage: f32,
    
    /// List of features
    pub features: Vec<Feature>,
}

/// Technical debt metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TechDebtMetrics {
    /// Total number of technical debt issues
    pub total_issues: usize,
    
    /// Number of critical issues
    pub critical_issues: usize,
    
    /// Number of high issues
    pub high_issues: usize,
    
    /// Number of medium issues
    pub medium_issues: usize,
    
    /// Number of low issues
    pub low_issues: usize,
    
    /// List of technical debt items
    pub items: Vec<TechDebtItem>,
}

/// Model information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Model {
    /// Model name
    pub name: String,
    
    /// File path
    pub file_path: String,
    
    /// Source system
    pub source_system: String,
    
    /// Whether the model is implemented
    pub implemented: bool,
    
    /// Model fields
    pub fields: Vec<ModelField>,
    
    /// Model relationships
    pub relationships: Vec<ModelRelationship>,
}

/// Model field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelField {
    /// Field name
    pub name: String,
    
    /// Field type
    pub field_type: String,
    
    /// Whether the field is required
    pub required: bool,
    
    /// Field description
    pub description: Option<String>,
}

/// Model relationship
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelRelationship {
    /// Relationship type
    pub relationship_type: String,
    
    /// Related model
    pub related_model: String,
    
    /// Relationship description
    pub description: Option<String>,
}

/// API endpoint information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiEndpoint {
    /// Endpoint path
    pub path: String,
    
    /// HTTP method
    pub method: String,
    
    /// Endpoint description
    pub description: String,
    
    /// Whether the endpoint is implemented
    pub implemented: bool,
    
    /// Endpoint category
    pub category: Option<String>,
    
    /// Request parameters
    pub parameters: Vec<ApiParameter>,
    
    /// Response fields
    pub response_fields: Vec<ApiResponseField>,
}

/// API parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiParameter {
    /// Parameter name
    pub name: String,
    
    /// Parameter type
    pub param_type: String,
    
    /// Whether the parameter is required
    pub required: bool,
    
    /// Parameter description
    pub description: Option<String>,
}

/// API response field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponseField {
    /// Field name
    pub name: String,
    
    /// Field type
    pub field_type: String,
    
    /// Field description
    pub description: Option<String>,
}

/// UI component information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiComponent {
    /// Component name
    pub name: String,
    
    /// File path
    pub file_path: String,
    
    /// Component description
    pub description: String,
    
    /// Whether the component is implemented
    pub implemented: bool,
    
    /// Component category
    pub category: Option<String>,
}

/// Feature information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Feature {
    /// Feature name
    pub name: String,
    
    /// Feature description
    pub description: String,
    
    /// Whether the feature is implemented
    pub implemented: bool,
    
    /// Feature priority
    pub priority: String,
}

/// Technical debt item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TechDebtItem {
    /// File path
    pub file: String,
    
    /// Line number
    pub line: usize,
    
    /// Category
    pub category: String,
    
    /// Description
    pub description: String,
    
    /// Severity
    pub severity: TechDebtSeverity,
    
    /// Fix suggestion
    pub fix_suggestion: String,
}

/// Technical debt severity
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TechDebtSeverity {
    /// Critical severity
    Critical,
    
    /// High severity
    High,
    
    /// Medium severity
    Medium,
    
    /// Low severity
    Low,
}

impl Default for AnalysisResult {
    fn default() -> Self {
        Self {
            timestamp: Local::now(),
            summary: ProjectSummary {
                total_files: 0,
                lines_of_code: 0,
                file_types: HashMap::new(),
                rust_files: 0,
                haskell_files: 0,
            },
            code_metrics: CodeMetrics {
                avg_complexity: 0.0,
                function_count: 0,
                module_count: 0,
                struct_count: 0,
                enum_count: 0,
                trait_count: 0,
                impl_count: 0,
            },
            models: ModelMetrics {
                total: 0,
                implemented: 0,
                implementation_percentage: 0.0,
                models: Vec::new(),
            },
            api_endpoints: ApiEndpointMetrics {
                total: 0,
                implemented: 0,
                implementation_percentage: 0.0,
                endpoints: Vec::new(),
            },
            ui_components: UiComponentMetrics {
                total: 0,
                implemented: 0,
                implementation_percentage: 0.0,
                components: Vec::new(),
            },
            feature_areas: HashMap::new(),
            tech_debt_metrics: TechDebtMetrics {
                total_issues: 0,
                critical_issues: 0,
                high_issues: 0,
                medium_issues: 0,
                low_issues: 0,
                items: Vec::new(),
            },
            recent_changes: Vec::new(),
            next_steps: Vec::new(),
            overall_progress: 0.0,
        }
    }
}
