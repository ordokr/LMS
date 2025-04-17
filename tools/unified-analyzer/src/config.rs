use std::fs::File;
use std::io::Read;
use std::path::Path;
use serde::{Serialize, Deserialize};
use anyhow::{Result, Context};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub general: GeneralConfig,
    pub analysis: AnalysisConfig,
    pub documentation: DocumentationConfig,
    pub performance: PerformanceConfig,
    #[serde(default)]
    pub paths: PathsConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralConfig {
    pub project_name: String,
    pub output_dir: String,
    pub log_level: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisConfig {
    pub analyzers: Vec<String>,
    pub exclude_patterns: Vec<String>,
    pub include_patterns: Vec<String>,
    pub max_file_size_mb: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentationConfig {
    pub generate_high_priority: bool,
    pub generate_medium_priority: bool,
    pub generate_low_priority: bool,
    pub high_priority: HighPriorityDocConfig,
    pub medium_priority: MediumPriorityDocConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HighPriorityDocConfig {
    pub central_reference_hub: bool,
    pub api_documentation: bool,
    pub implementation_details: bool,
    pub testing_documentation: bool,
    pub tech_debt_report: bool,
    pub summary_report: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediumPriorityDocConfig {
    pub sync_architecture: bool,
    pub database_architecture: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    pub parallel_processing: bool,
    pub enable_caching: bool,
    pub incremental_analysis: bool,
    pub cache_dir: String,
    pub max_memory_mb: usize,
    pub timeout_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PathsConfig {
    pub canvas_path: Option<String>,
    pub discourse_path: Option<String>,
    pub lms_path: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            general: GeneralConfig {
                project_name: "Unified Analyzer".to_string(),
                output_dir: "output".to_string(),
                log_level: "info".to_string(),
            },
            analysis: AnalysisConfig {
                analyzers: vec![
                    "file_structure".to_string(),
                    "ruby_rails".to_string(),
                    "ember".to_string(),
                    "react".to_string(),
                    "template".to_string(),
                    "route".to_string(),
                    "api".to_string(),
                    "dependency".to_string(),
                    "auth_flow".to_string(),
                    "offline_first_readiness".to_string(),
                    "database_schema".to_string(),
                    "business_logic".to_string(),
                ],
                exclude_patterns: vec![
                    "node_modules".to_string(),
                    "vendor".to_string(),
                    "dist".to_string(),
                    "tmp".to_string(),
                    "log".to_string(),
                ],
                include_patterns: vec![],
                max_file_size_mb: 10,
            },
            documentation: DocumentationConfig {
                generate_high_priority: true,
                generate_medium_priority: false,
                generate_low_priority: false,
                high_priority: HighPriorityDocConfig {
                    central_reference_hub: true,
                    api_documentation: true,
                    implementation_details: true,
                    testing_documentation: true,
                    tech_debt_report: true,
                    summary_report: true,
                },
                medium_priority: MediumPriorityDocConfig {
                    sync_architecture: false,
                    database_architecture: false,
                },
            },
            performance: PerformanceConfig {
                parallel_processing: false,
                enable_caching: false,
                incremental_analysis: false,
                cache_dir: ".cache".to_string(),
                max_memory_mb: 1024,
                timeout_seconds: 3600,
            },
            paths: PathsConfig {
                canvas_path: Some("C:\\Users\\Tim\\Desktop\\port\\canvas".to_string()),
                discourse_path: Some("C:\\Users\\Tim\\Desktop\\port\\discourse".to_string()),
                lms_path: Some("C:\\Users\\Tim\\Desktop\\LMS".to_string()),
            },
        }
    }
}

impl Config {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let mut file = File::open(path).context("Failed to open config file")?;
        let mut contents = String::new();
        file.read_to_string(&mut contents).context("Failed to read config file")?;

        let config: Config = toml::from_str(&contents).context("Failed to parse config file")?;

        Ok(config)
    }

    pub fn get_path(&self, path_name: &str) -> Option<&str> {
        match path_name {
            "canvas_path" => self.paths.canvas_path.as_deref(),
            "discourse_path" => self.paths.discourse_path.as_deref(),
            "lms_path" => self.paths.lms_path.as_deref(),
            "moodle_path" => None,
            "wordpress_path" => None,
            _ => None,
        }
    }

    pub fn set_path(&mut self, path_name: &str, path_value: String) {
        match path_name {
            "canvas_path" => self.paths.canvas_path = Some(path_value),
            "discourse_path" => self.paths.discourse_path = Some(path_value),
            "lms_path" => self.paths.lms_path = Some(path_value),
            "moodle_path" => {}, // Not implemented yet
            "wordpress_path" => {}, // Not implemented yet
            _ => {},
        }
    }
}
