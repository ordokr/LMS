use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use regex::Regex;
use walkdir::WalkDir;
use semver::Version;

/// Analyzer for dependencies in the codebase
pub struct DependencyAnalyzer {
    /// Base directory for analysis
    base_dir: PathBuf,
    
    /// Directories to exclude from analysis
    exclude_dirs: Vec<String>,
}

/// Dependency information
#[derive(Debug, Clone)]
pub struct Dependency {
    /// Dependency name
    pub name: String,
    
    /// Dependency version
    pub version: String,
    
    /// Dependency type (direct or transitive)
    pub dependency_type: DependencyType,
    
    /// File where the dependency is declared
    pub file: String,
}

/// Dependency type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DependencyType {
    /// Direct dependency
    Direct,
    
    /// Transitive dependency
    Transitive,
}

/// Dependency metrics
#[derive(Debug, Clone)]
pub struct DependencyMetrics {
    /// Total number of dependencies
    pub total_dependencies: usize,
    
    /// Number of direct dependencies
    pub direct_dependencies: usize,
    
    /// Number of transitive dependencies
    pub transitive_dependencies: usize,
    
    /// Dependencies by file
    pub dependencies_by_file: HashMap<String, Vec<Dependency>>,
    
    /// Dependencies with outdated versions
    pub outdated_dependencies: Vec<(Dependency, String)>,
}

impl DependencyAnalyzer {
    /// Create a new dependency analyzer
    pub fn new(base_dir: PathBuf) -> Self {
        // Add directories to exclude
        let exclude_dirs = vec![
            "target".to_string(),
            "node_modules".to_string(),
            ".git".to_string(),
            "build-output".to_string(),
        ];
        
        Self {
            base_dir,
            exclude_dirs,
        }
    }
    
    /// Analyze a file for dependencies
    pub fn analyze_file(&self, file_path: &Path) -> Result<Vec<Dependency>, String> {
        // Skip files in excluded directories
        for exclude_dir in &self.exclude_dirs {
            if file_path.to_string_lossy().contains(exclude_dir) {
                return Ok(Vec::new());
            }
        }
        
        // Only analyze Cargo.toml files
        if file_path.file_name().and_then(|n| n.to_str()) != Some("Cargo.toml") {
            return Ok(Vec::new());
        }
        
        // Read file content
        let content = fs::read_to_string(file_path)
            .map_err(|e| format!("Failed to read file {}: {}", file_path.display(), e))?;
        
        // Create relative path
        let relative_path = file_path.strip_prefix(&self.base_dir)
            .map_err(|_| "Failed to create relative path".to_string())?
            .to_string_lossy()
            .to_string();
        
        let mut dependencies = Vec::new();
        
        // Parse dependencies section
        if let Some(deps_section) = content.find("[dependencies]") {
            let deps_content = &content[deps_section..];
            let deps_end = deps_content.find("\n[").unwrap_or(deps_content.len());
            let deps_content = &deps_content[..deps_end];
            
            // Regular expression to match dependencies
            let re = Regex::new(r#"(?m)^([a-zA-Z0-9_-]+)\s*=\s*(?:"([^"]+)"|(\{[^}]+\}))"#).unwrap();
            
            for captures in re.captures_iter(deps_content) {
                let name = captures.get(1).unwrap().as_str().to_string();
                
                // Check if the dependency is specified as a string or a table
                let version = if let Some(version_str) = captures.get(2) {
                    version_str.as_str().to_string()
                } else if let Some(table_str) = captures.get(3) {
                    // Extract version from table
                    let version_re = Regex::new(r#"version\s*=\s*"([^"]+)""#).unwrap();
                    if let Some(version_captures) = version_re.captures(table_str.as_str()) {
                        version_captures.get(1).unwrap().as_str().to_string()
                    } else {
                        "unknown".to_string()
                    }
                } else {
                    "unknown".to_string()
                };
                
                dependencies.push(Dependency {
                    name,
                    version,
                    dependency_type: DependencyType::Direct,
                    file: relative_path.clone(),
                });
            }
        }
        
        // Parse dev-dependencies section
        if let Some(deps_section) = content.find("[dev-dependencies]") {
            let deps_content = &content[deps_section..];
            let deps_end = deps_content.find("\n[").unwrap_or(deps_content.len());
            let deps_content = &deps_content[..deps_end];
            
            // Regular expression to match dependencies
            let re = Regex::new(r#"(?m)^([a-zA-Z0-9_-]+)\s*=\s*(?:"([^"]+)"|(\{[^}]+\}))"#).unwrap();
            
            for captures in re.captures_iter(deps_content) {
                let name = captures.get(1).unwrap().as_str().to_string();
                
                // Check if the dependency is specified as a string or a table
                let version = if let Some(version_str) = captures.get(2) {
                    version_str.as_str().to_string()
                } else if let Some(table_str) = captures.get(3) {
                    // Extract version from table
                    let version_re = Regex::new(r#"version\s*=\s*"([^"]+)""#).unwrap();
                    if let Some(version_captures) = version_re.captures(table_str.as_str()) {
                        version_captures.get(1).unwrap().as_str().to_string()
                    } else {
                        "unknown".to_string()
                    }
                } else {
                    "unknown".to_string()
                };
                
                dependencies.push(Dependency {
                    name,
                    version,
                    dependency_type: DependencyType::Direct,
                    file: relative_path.clone(),
                });
            }
        }
        
        Ok(dependencies)
    }
    
    /// Analyze a directory for dependencies
    pub fn analyze_directory(&self, dir_path: &Path, dependencies: &mut Vec<Dependency>) -> Result<(), String> {
        // Skip excluded directories
        for exclude_dir in &self.exclude_dirs {
            if dir_path.to_string_lossy().contains(exclude_dir) {
                return Ok(());
            }
        }
        
        // Walk through the directory
        for entry in WalkDir::new(dir_path)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| e.file_type().is_file())
        {
            let path = entry.path();
            
            // Analyze the file
            let mut file_deps = self.analyze_file(path)?;
            dependencies.append(&mut file_deps);
        }
        
        Ok(())
    }
    
    /// Analyze the entire codebase for dependencies
    pub fn analyze_codebase(&self) -> Result<DependencyMetrics, String> {
        let mut dependencies = Vec::new();
        
        // Analyze the base directory
        self.analyze_directory(&self.base_dir, &mut dependencies)?;
        
        // Count dependencies by type
        let mut direct_dependencies = 0;
        let mut transitive_dependencies = 0;
        
        for dependency in &dependencies {
            match dependency.dependency_type {
                DependencyType::Direct => direct_dependencies += 1,
                DependencyType::Transitive => transitive_dependencies += 1,
            }
        }
        
        // Group dependencies by file
        let mut dependencies_by_file = HashMap::new();
        
        for dependency in &dependencies {
            dependencies_by_file.entry(dependency.file.clone())
                .or_insert_with(Vec::new)
                .push(dependency.clone());
        }
        
        // Check for outdated dependencies
        let mut outdated_dependencies = Vec::new();
        
        // TODO: Implement check for outdated dependencies
        // This would involve querying crates.io for the latest versions
        
        Ok(DependencyMetrics {
            total_dependencies: dependencies.len(),
            direct_dependencies,
            transitive_dependencies,
            dependencies_by_file,
            outdated_dependencies,
        })
    }
    
    /// Generate a dependency report
    pub fn generate_report(&self) -> Result<String, String> {
        // Analyze the codebase
        let metrics = self.analyze_codebase()?;
        
        // Generate the report
        let mut report = String::new();
        
        // Header
        report.push_str("# Dependency Analysis Report\n\n");
        
        // Summary
        report.push_str("## Summary\n\n");
        report.push_str(&format!("**Total Dependencies: {}**\n\n", metrics.total_dependencies));
        report.push_str("| Type | Count |\n");
        report.push_str("|------|-------|\n");
        report.push_str(&format!("| Direct | {} |\n", metrics.direct_dependencies));
        report.push_str(&format!("| Transitive | {} |\n\n", metrics.transitive_dependencies));
        
        // Dependencies by File
        report.push_str("## Dependencies by File\n\n");
        
        for (file, deps) in &metrics.dependencies_by_file {
            report.push_str(&format!("### {}\n\n", file));
            
            if !deps.is_empty() {
                report.push_str("| Name | Version | Type |\n");
                report.push_str("|------|---------|------|\n");
                
                for dep in deps {
                    let dep_type = match dep.dependency_type {
                        DependencyType::Direct => "Direct",
                        DependencyType::Transitive => "Transitive",
                    };
                    
                    report.push_str(&format!("| {} | {} | {} |\n",
                        dep.name,
                        dep.version,
                        dep_type));
                }
            } else {
                report.push_str("No dependencies found.\n");
            }
            
            report.push_str("\n");
        }
        
        // Outdated Dependencies
        report.push_str("## Outdated Dependencies\n\n");
        
        if !metrics.outdated_dependencies.is_empty() {
            report.push_str("| Name | Current Version | Latest Version | File |\n");
            report.push_str("|------|----------------|----------------|------|\n");
            
            for (dep, latest_version) in &metrics.outdated_dependencies {
                report.push_str(&format!("| {} | {} | {} | {} |\n",
                    dep.name,
                    dep.version,
                    latest_version,
                    dep.file));
            }
        } else {
            report.push_str("No outdated dependencies found.\n");
        }
        
        Ok(report)
    }
}
