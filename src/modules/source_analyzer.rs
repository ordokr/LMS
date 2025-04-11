use std::path::{Path, PathBuf};
use std::collections::HashMap;
use std::fs;
use anyhow::{Result, Context};
use serde::{Serialize, Deserialize};
use regex::Regex;
use log::info;

/// Source system types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SourceSystem {
    Canvas,
    Discourse,
    Other,
}

impl std::fmt::Display for SourceSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SourceSystem::Canvas => write!(f, "Canvas"),
            SourceSystem::Discourse => write!(f, "Discourse"),
            SourceSystem::Other => write!(f, "Other"),
        }
    }
}

/// HTTP methods for controller actions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Patch,
    Delete,
    Head,
    Options,
    Any,
}

/// Represents a property in a model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelProperty {
    pub name: String,
    pub property_type: String,
    pub required: bool,
    pub default_value: Option<String>,
    pub description: Option<String>,
}

/// Represents a model in a source system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceModel {
    pub name: String,
    pub file_path: PathBuf,
    pub properties: Vec<ModelProperty>,
    pub associations: Vec<ModelAssociation>,
    pub system: SourceSystem,
}

/// Represents an association between models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelAssociation {
    pub association_type: String, // "belongs_to", "has_many", etc.
    pub target_model: String,
    pub foreign_key: Option<String>,
}

/// Represents a controller action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControllerAction {
    pub name: String,
    pub http_method: HttpMethod,
    pub path: Option<String>,
    pub parameters: Vec<String>,
    pub description: Option<String>,
}

/// Represents a controller in a source system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceController {
    pub name: String,
    pub file_path: PathBuf,
    pub actions: Vec<ControllerAction>,
    pub system: SourceSystem,
}

/// Source analysis metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceMetrics {
    pub files_by_type: HashMap<String, usize>,
    pub models: HashMap<String, usize>,
    pub controllers: HashMap<String, usize>,
    pub total_routes: usize,
    pub file_count: usize,
    pub total_lines: usize,
}

/// Module for analyzing source systems (Canvas, Discourse)
pub struct SourceAnalyzer<M> {
    metrics: M,
    exclude_patterns: Vec<Regex>,
}

impl<M> SourceAnalyzer<M> {
    /// Create a new source analyzer
    pub fn new(metrics: M, exclude_patterns: Option<Vec<String>>) -> Result<Self> {
        // Convert string patterns to compiled regular expressions
        let exclude_patterns = if let Some(patterns) = exclude_patterns {
            patterns.into_iter()
                .map(|pattern| Regex::new(&pattern))
                .collect::<std::result::Result<Vec<Regex>, regex::Error>>()
                .context("Failed to compile exclude patterns")?
        } else {
            Vec::new()
        };
        
        Ok(Self {
            metrics,
            exclude_patterns,
        })
    }
    
    /// Analyze source systems
    pub fn analyze_source_systems(
        &self,
        base_dir: &Path,
        systems: &HashMap<String, PathBuf>,
    ) -> Result<HashMap<SourceSystem, SourceMetrics>> {
        info!("Analyzing source systems: {}", 
             systems.keys().cloned().collect::<Vec<_>>().join(", "));
        
        let mut results = HashMap::new();
        
        for (system_name, system_path) in systems {
            let system_type = match system_name.to_lowercase().as_str() {
                "canvas" => SourceSystem::Canvas,
                "discourse" => SourceSystem::Discourse,
                _ => SourceSystem::Other,
            };
            
            let files_by_type = self.count_source_files_by_type(system_path)?;
            let models = self.analyze_source_models(system_path, system_type)?;
            let controllers = self.analyze_source_controllers(system_path, system_type)?;
            
            let metrics = SourceMetrics {
                files_by_type: files_by_type.clone(),
                models: models.iter()
                    .map(|m| (m.name.clone(), 1))
                    .collect(),
                controllers: controllers.iter()
                    .map(|c| (c.name.clone(), 1))
                    .collect(),
                total_routes: controllers.iter()
                    .map(|c| c.actions.len())
                    .sum(),
                file_count: files_by_type.values().sum(),
                total_lines: 0, // Would need to calculate by reading files
            };
            
            results.insert(system_type, metrics);
        }
        
        // Generate comparison report
        if results.len() > 1 {
            self.generate_source_comparison_report(base_dir, &results)?;
        }
        
        Ok(results)
    }
    
    /// Count source files by type
    fn count_source_files_by_type(&self, system_path: &Path) -> Result<HashMap<String, usize>> {
        let mut counts = HashMap::new();
        
        // This would recursively walk the directory and count files by extension
        // For simplicity, we'll just return a placeholder
        
        counts.insert("rb".to_string(), 100);  // Ruby files
        counts.insert("js".to_string(), 50);   // JavaScript files
        counts.insert("html.erb".to_string(), 30); // ERB templates
        
        Ok(counts)
    }
    
    /// Analyze source models
    fn analyze_source_models(
        &self,
        system_path: &Path,
        system_type: SourceSystem,
    ) -> Result<Vec<SourceModel>> {
        info!("Analyzing models for {}", system_type);
        
        let mut models = Vec::new();
        
        // This would find and parse model files based on system type
        // For Ruby on Rails projects like Canvas and Discourse, we'd look for
        // app/models/*.rb files and extract model properties
        
        // For now, return a placeholder
        
        Ok(models)
    }
    
    /// Extract Ruby model properties
    fn extract_ruby_model_properties(&self, file_path: &Path) -> Result<Vec<ModelProperty>> {
        // This would parse a Ruby file and extract model properties
        // It would look for things like:
        // - attr_accessor :name
        // - validates_presence_of :email
        // - has_many :posts
        // etc.
        
        // For now, return a placeholder
        Ok(Vec::new())
    }
    
    /// Analyze source controllers
    fn analyze_source_controllers(
        &self,
        system_path: &Path,
        system_type: SourceSystem,
    ) -> Result<Vec<SourceController>> {
        info!("Analyzing controllers for {}", system_type);
        
        let mut controllers = Vec::new();
        
        // This would find and parse controller files based on system type
        // For Ruby on Rails projects like Canvas and Discourse, we'd look for
        // app/controllers/*.rb files and extract controller actions
        
        // For now, return a placeholder
        
        Ok(controllers)
    }
    
    /// Extract Ruby controller actions
    fn extract_ruby_controller_actions(&self, file_path: &Path) -> Result<Vec<ControllerAction>> {
        // This would parse a Ruby file and extract controller actions
        // It would look for methods in the controller class and try to determine
        // the HTTP method based on Rails conventions or explicit routes
        
        // For now, return a placeholder
        Ok(Vec::new())
    }
    
    /// Map source model to target model
    pub fn map_source_to_target(
        &self,
        source_model: &SourceModel,
        target_models: &[SourceModel],
    ) -> Result<Option<(String, f64)>> {
        // This would attempt to find the best match between a source model
        // and potential target models based on name and property similarity
        
        let mut best_match = None;
        let mut best_score = 0.0;
        
        for target_model in target_models {
            // Skip models from the same system
            if source_model.system == target_model.system {
                continue;
            }
            
            // Calculate property match score
            let score = self.calculate_property_match_score(source_model, target_model);
            
            if score > best_score {
                best_score = score;
                best_match = Some((target_model.name.clone(), score));
            }
        }
        
        // Require a minimum score to consider it a match
        if best_score >= 0.5 {
            Ok(best_match)
        } else {
            Ok(None)
        }
    }
    
    /// Calculate property match score between models
    fn calculate_property_match_score(
        &self,
        source_model: &SourceModel,
        target_model: &SourceModel,
    ) -> f64 {
        // This would compare properties between models and return a similarity score
        // It would consider exact matches, similar names, and similar types
        
        // For now, return a placeholder
        0.7
    }
    
    /// Check if HTTP methods match
    fn http_method_matches(&self, method1: HttpMethod, method2: HttpMethod) -> bool {
        method1 == method2 || method2 == HttpMethod::Any || method1 == HttpMethod::Any
    }
    
    /// Pluralize a word (very simplistic implementation)
    fn pluralize(&self, word: &str) -> String {
        if word.ends_with('s') {
            word.to_string()
        } else {
            format!("{}s", word)
        }
    }
    
    /// Singularize a word (very simplistic implementation)
    fn singularize(&self, word: &str) -> String {
        if word.ends_with('s') {
            word[0..word.len()-1].to_string()
        } else {
            word.to_string()
        }
    }
    
    /// Generate cache key for a model or controller
    fn generate_cache_key(&self, name: &str, system: SourceSystem) -> String {
        format!("{}_{}", system, name)
    }
    
    /// Generate source comparison report
    fn generate_source_comparison_report(
        &self,
        base_dir: &Path,
        metrics: &HashMap<SourceSystem, SourceMetrics>,
    ) -> Result<PathBuf> {
        info!("Generating source comparison report");
        
        let report_dir = base_dir.join("analysis_summary");
        fs::create_dir_all(&report_dir).context("Failed to create report directory")?;
        
        let report_path = report_dir.join("source_comparison.md");
        
        // Generate a markdown report comparing the source systems
        let mut report = String::new();
        
        report.push_str("# Source System Comparison Report\n\n");
        report.push_str(&format!("Generated: {}\n\n", chrono::Local::now().format("%Y-%m-%d")));
        
        report.push_str("## File Counts\n\n");
        report.push_str("| File Type | ");
        
        // Add a column for each system
        for system in metrics.keys() {
            report.push_str(&format!("{} | ", system));
        }
        report.push_str("\n");
        
        // Add separator row
        report.push_str("|-----------|");
        for _ in metrics.keys() {
            report.push_str("---------|");
        }
        report.push_str("\n");
        
        // Get all unique file types
        let mut file_types: Vec<String> = metrics.values()
            .flat_map(|m| m.files_by_type.keys().cloned())
            .collect();
        file_types.sort();
        file_types.dedup();
        
        // Add a row for each file type
        for file_type in file_types {
            report.push_str(&format!("| {} | ", file_type));
            
            for system in metrics.keys() {
                let count = metrics.get(system)
                    .and_then(|m| m.files_by_type.get(&file_type))
                    .copied()
                    .unwrap_or(0);
                    
                report.push_str(&format!("{} | ", count));
            }
            
            report.push_str("\n");
        }
        
        // Write the report to file
        fs::write(&report_path, report).context("Failed to write report file")?;
        
        info!("Source comparison report written to {:?}", report_path);
        
        Ok(report_path)
    }
}
