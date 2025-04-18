use std::path::{Path, PathBuf};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::fs;
use walkdir::WalkDir;
use regex::Regex;
use anyhow::{Result, Context};

use crate::utils::incremental_analyzer::{IncrementalAnalyzer, AnalysisCache};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dependency {
    pub name: String,
    pub version: String,
    pub dependency_type: String,
    pub source_file: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyGraph {
    pub nodes: Vec<String>,
    pub edges: Vec<(String, String)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyAnalysisResult {
    pub ruby_dependencies: HashMap<String, Dependency>,
    pub js_dependencies: HashMap<String, Dependency>,
    pub python_dependencies: HashMap<String, Dependency>,
    pub system_dependencies: HashMap<String, Dependency>,
    pub dependency_graph: DependencyGraph,
}

impl Default for DependencyAnalysisResult {
    fn default() -> Self {
        Self {
            ruby_dependencies: HashMap::new(),
            js_dependencies: HashMap::new(),
            python_dependencies: HashMap::new(),
            system_dependencies: HashMap::new(),
            dependency_graph: DependencyGraph {
                nodes: Vec::new(),
                edges: Vec::new(),
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IncrementalDependencyAnalyzer {
    pub base_dir: PathBuf,
    pub use_incremental: bool,
    pub cache_path: Option<PathBuf>,
    pub exclude_dirs: Vec<String>,
    pub include_extensions: Vec<String>,
}

impl Default for IncrementalDependencyAnalyzer {
    fn default() -> Self {
        Self {
            base_dir: PathBuf::new(),
            use_incremental: true, // Enable incremental analysis by default
            cache_path: None,
            exclude_dirs: vec![
                "node_modules".to_string(),
                "target".to_string(),
                "dist".to_string(),
                "build".to_string(),
                ".git".to_string(),
            ],
            include_extensions: vec![
                "json".to_string(),
                "gemspec".to_string(),
                "rb".to_string(),
                "py".to_string(),
                "txt".to_string(),
                "lock".to_string(),
                "yml".to_string(),
                "yaml".to_string(),
                "toml".to_string(),
            ],
        }
    }
}

impl IncrementalDependencyAnalyzer {
    pub fn new(base_dir: PathBuf) -> Self {
        let mut analyzer = Self::default();
        analyzer.base_dir = base_dir.clone();
        analyzer.cache_path = Some(base_dir.join(".dependency_analyzer_cache.json"));
        analyzer
    }

    pub fn with_incremental(mut self, use_incremental: bool) -> Self {
        self.use_incremental = use_incremental;
        self
    }

    pub fn with_cache_path(mut self, cache_path: PathBuf) -> Self {
        self.cache_path = Some(cache_path);
        self
    }

    pub fn analyze(&self) -> Result<DependencyAnalysisResult> {
        // Collect specific dependency files
        let mut dependency_files = Vec::new();
        
        // Add common dependency files
        let common_files = [
            "Gemfile", "Gemfile.lock", "package.json", "package-lock.json", 
            "yarn.lock", "requirements.txt", "setup.py", "Dockerfile", 
            "docker-compose.yml", "Cargo.toml", "Cargo.lock"
        ];
        
        for file in &common_files {
            let file_path = self.base_dir.join(file);
            if file_path.exists() {
                dependency_files.push(file_path);
            }
        }
        
        // Find all gemspec files
        for entry in WalkDir::new(&self.base_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
        {
            let path = entry.path();
            if path.extension().map_or(false, |ext| ext == "gemspec") {
                dependency_files.push(path.to_path_buf());
            }
        }
        
        // Analyze files incrementally
        let file_results = self.analyze_files_incrementally(&dependency_files)?;
        
        // Combine results
        let mut combined_result = DependencyAnalysisResult::default();
        
        for result in file_results {
            // Combine dependencies
            for (key, dep) in result.ruby_dependencies {
                combined_result.ruby_dependencies.insert(key, dep);
            }
            
            for (key, dep) in result.js_dependencies {
                combined_result.js_dependencies.insert(key, dep);
            }
            
            for (key, dep) in result.python_dependencies {
                combined_result.python_dependencies.insert(key, dep);
            }
            
            for (key, dep) in result.system_dependencies {
                combined_result.system_dependencies.insert(key, dep);
            }
        }
        
        // Build dependency graph
        self.build_dependency_graph(&mut combined_result);
        
        Ok(combined_result)
    }
    
    fn build_dependency_graph(&self, result: &mut DependencyAnalysisResult) {
        let mut nodes = Vec::new();
        let mut edges = Vec::new();

        // Add all dependencies as nodes
        for (name, _) in &result.ruby_dependencies {
            nodes.push(name.clone());
        }

        for (name, _) in &result.js_dependencies {
            nodes.push(name.clone());
        }

        for (name, _) in &result.python_dependencies {
            nodes.push(name.clone());
        }

        for (name, _) in &result.system_dependencies {
            nodes.push(name.clone());
        }

        // Add edges based on known relationships
        // This is a simplified approach, as we don't have full dependency tree information

        // For Ruby, we can infer some relationships
        for (name, _dep) in &result.ruby_dependencies {
            if name.contains("rails") {
                // Rails has many dependencies
                for (other_name, _) in &result.ruby_dependencies {
                    if other_name.contains("active") || other_name.contains("action") {
                        edges.push((name.clone(), other_name.clone()));
                    }
                }
            }
        }

        // For JS, we can infer some relationships
        for (name, _dep) in &result.js_dependencies {
            if name == "react" {
                // React has some common dependencies
                for (other_name, _) in &result.js_dependencies {
                    if other_name.contains("react-") {
                        edges.push((name.clone(), other_name.clone()));
                    }
                }
            }
        }

        result.dependency_graph = DependencyGraph { nodes, edges };
    }
    
    fn analyze_gemfile(&self, content: &str, source_file: String) -> DependencyAnalysisResult {
        let mut result = DependencyAnalysisResult::default();
        
        // Extract gem dependencies from Gemfile
        let gem_regex = Regex::new(r#"gem\s+['"]([^'"]+)['"](?:,\s*['"]([^'"]+)['"])?(?:,\s*([^\n]+))?"#).unwrap();

        for cap in gem_regex.captures_iter(content) {
            if let Some(name) = cap.get(1) {
                let version = cap.get(2).map_or("latest", |m| m.as_str());

                let dependency = Dependency {
                    name: name.as_str().to_string(),
                    version: version.to_string(),
                    dependency_type: "ruby".to_string(),
                    source_file: source_file.clone(),
                };

                result.ruby_dependencies.insert(name.as_str().to_string(), dependency);
            }
        }
        
        result
    }
    
    fn analyze_gemspec(&self, content: &str, source_file: String) -> DependencyAnalysisResult {
        let mut result = DependencyAnalysisResult::default();
        
        // Extract dependencies from gemspec
        let dependency_regex = Regex::new(r#"(?:add_dependency|add_runtime_dependency|add_development_dependency)\s+['"]([^'"]+)['"](?:,\s*['"]([^'"]+)['"])?"#).unwrap();

        for cap in dependency_regex.captures_iter(content) {
            if let Some(name) = cap.get(1) {
                let version = cap.get(2).map_or("latest", |m| m.as_str());
                let dependency_type = if content.contains("add_development_dependency") {
                    "development"
                } else {
                    "runtime"
                };

                let dependency = Dependency {
                    name: name.as_str().to_string(),
                    version: version.to_string(),
                    dependency_type: format!("ruby-{}", dependency_type),
                    source_file: source_file.clone(),
                };

                result.ruby_dependencies.insert(name.as_str().to_string(), dependency);
            }
        }
        
        result
    }
    
    fn analyze_package_json(&self, content: &str, source_file: String) -> DependencyAnalysisResult {
        let mut result = DependencyAnalysisResult::default();
        
        // Try to parse as JSON
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(content) {
            // Extract dependencies
            if let Some(dependencies) = json.get("dependencies").and_then(|d| d.as_object()) {
                for (name, version) in dependencies {
                    if let Some(version_str) = version.as_str() {
                        let dependency = Dependency {
                            name: name.clone(),
                            version: version_str.to_string(),
                            dependency_type: "js-runtime".to_string(),
                            source_file: source_file.clone(),
                        };

                        result.js_dependencies.insert(name.clone(), dependency);
                    }
                }
            }

            // Extract dev dependencies
            if let Some(dev_dependencies) = json.get("devDependencies").and_then(|d| d.as_object()) {
                for (name, version) in dev_dependencies {
                    if let Some(version_str) = version.as_str() {
                        let dependency = Dependency {
                            name: name.clone(),
                            version: version_str.to_string(),
                            dependency_type: "js-development".to_string(),
                            source_file: source_file.clone(),
                        };

                        result.js_dependencies.insert(name.clone(), dependency);
                    }
                }
            }
        }
        
        result
    }
    
    fn analyze_requirements_txt(&self, content: &str, source_file: String) -> DependencyAnalysisResult {
        let mut result = DependencyAnalysisResult::default();
        
        // Extract Python dependencies from requirements.txt
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            // Parse requirement line
            let parts: Vec<&str> = line.split(&['=', '>', '<', '~', '!'][..]).collect();
            if !parts.is_empty() {
                let name = parts[0].trim();
                let version = if parts.len() > 1 {
                    line.replace(name, "").trim().to_string()
                } else {
                    "latest".to_string()
                };

                let dependency = Dependency {
                    name: name.to_string(),
                    version,
                    dependency_type: "python".to_string(),
                    source_file: source_file.clone(),
                };

                result.python_dependencies.insert(name.to_string(), dependency);
            }
        }
        
        result
    }
    
    fn analyze_cargo_toml(&self, content: &str, source_file: String) -> DependencyAnalysisResult {
        let mut result = DependencyAnalysisResult::default();
        
        // Try to parse as TOML
        if let Ok(toml) = toml::from_str::<toml::Value>(content) {
            // Extract dependencies
            if let Some(dependencies) = toml.get("dependencies").and_then(|d| d.as_table()) {
                for (name, version_value) in dependencies {
                    let version = match version_value {
                        toml::Value::String(v) => v.clone(),
                        toml::Value::Table(t) => {
                            if let Some(v) = t.get("version").and_then(|v| v.as_str()) {
                                v.to_string()
                            } else {
                                "latest".to_string()
                            }
                        },
                        _ => "latest".to_string(),
                    };

                    let dependency = Dependency {
                        name: name.clone(),
                        version,
                        dependency_type: "rust".to_string(),
                        source_file: source_file.clone(),
                    };

                    // Store in system dependencies for now
                    result.system_dependencies.insert(name.clone(), dependency);
                }
            }
        }
        
        result
    }
    
    fn analyze_dockerfile(&self, content: &str, source_file: String) -> DependencyAnalysisResult {
        let mut result = DependencyAnalysisResult::default();
        
        // Extract system dependencies from Dockerfile
        let apt_get_regex = Regex::new(r#"apt-get\s+install\s+(.+?)(?:\s*\\|\s*$)"#).unwrap();

        for cap in apt_get_regex.captures_iter(content) {
            if let Some(packages) = cap.get(1) {
                let packages_str = packages.as_str();
                for pkg in packages_str.split_whitespace() {
                    if pkg != "-y" && !pkg.starts_with('-') {
                        let dependency = Dependency {
                            name: pkg.to_string(),
                            version: "latest".to_string(),
                            dependency_type: "system-apt".to_string(),
                            source_file: source_file.clone(),
                        };

                        result.system_dependencies.insert(pkg.to_string(), dependency);
                    }
                }
            }
        }
        
        result
    }
    
    pub fn generate_report(&self, result: &DependencyAnalysisResult) -> Result<String> {
        // Generate a markdown report
        let mut report = String::new();
        
        // Header
        report.push_str("# Dependency Analysis Report\n\n");
        report.push_str(&format!("_Generated on: {}_\n\n", chrono::Local::now().format("%Y-%m-%d")));
        
        // Summary
        report.push_str("## Summary\n\n");
        report.push_str("| Dependency Type | Count |\n");
        report.push_str("|----------------|-------|\n");
        report.push_str(&format!("| Ruby | {} |\n", result.ruby_dependencies.len()));
        report.push_str(&format!("| JavaScript | {} |\n", result.js_dependencies.len()));
        report.push_str(&format!("| Python | {} |\n", result.python_dependencies.len()));
        report.push_str(&format!("| System | {} |\n", result.system_dependencies.len()));
        report.push_str(&format!("| **Total** | **{}** |\n", 
            result.ruby_dependencies.len() + 
            result.js_dependencies.len() + 
            result.python_dependencies.len() + 
            result.system_dependencies.len()));
        
        // Ruby Dependencies
        if !result.ruby_dependencies.is_empty() {
            report.push_str("\n## Ruby Dependencies\n\n");
            report.push_str("| Name | Version | Type | Source |\n");
            report.push_str("|------|---------|------|--------|\n");
            
            let mut sorted_deps: Vec<_> = result.ruby_dependencies.values().collect();
            sorted_deps.sort_by(|a, b| a.name.cmp(&b.name));
            
            for dep in sorted_deps {
                report.push_str(&format!("| {} | {} | {} | {} |\n",
                    dep.name, dep.version, dep.dependency_type, dep.source_file));
            }
        }
        
        // JavaScript Dependencies
        if !result.js_dependencies.is_empty() {
            report.push_str("\n## JavaScript Dependencies\n\n");
            report.push_str("| Name | Version | Type | Source |\n");
            report.push_str("|------|---------|------|--------|\n");
            
            let mut sorted_deps: Vec<_> = result.js_dependencies.values().collect();
            sorted_deps.sort_by(|a, b| a.name.cmp(&b.name));
            
            for dep in sorted_deps {
                report.push_str(&format!("| {} | {} | {} | {} |\n",
                    dep.name, dep.version, dep.dependency_type, dep.source_file));
            }
        }
        
        // Python Dependencies
        if !result.python_dependencies.is_empty() {
            report.push_str("\n## Python Dependencies\n\n");
            report.push_str("| Name | Version | Type | Source |\n");
            report.push_str("|------|---------|------|--------|\n");
            
            let mut sorted_deps: Vec<_> = result.python_dependencies.values().collect();
            sorted_deps.sort_by(|a, b| a.name.cmp(&b.name));
            
            for dep in sorted_deps {
                report.push_str(&format!("| {} | {} | {} | {} |\n",
                    dep.name, dep.version, dep.dependency_type, dep.source_file));
            }
        }
        
        // System Dependencies
        if !result.system_dependencies.is_empty() {
            report.push_str("\n## System Dependencies\n\n");
            report.push_str("| Name | Version | Type | Source |\n");
            report.push_str("|------|---------|------|--------|\n");
            
            let mut sorted_deps: Vec<_> = result.system_dependencies.values().collect();
            sorted_deps.sort_by(|a, b| a.name.cmp(&b.name));
            
            for dep in sorted_deps {
                report.push_str(&format!("| {} | {} | {} | {} |\n",
                    dep.name, dep.version, dep.dependency_type, dep.source_file));
            }
        }
        
        Ok(report)
    }
    
    pub fn export_to_json(&self, result: &DependencyAnalysisResult) -> Result<String> {
        let json = serde_json::to_string_pretty(result)
            .context("Failed to serialize dependency analysis result to JSON")?;
        
        Ok(json)
    }
}

impl IncrementalAnalyzer<DependencyAnalysisResult> for IncrementalDependencyAnalyzer {
    fn base_dir(&self) -> &Path {
        &self.base_dir
    }
    
    fn cache_path(&self) -> Option<&Path> {
        self.cache_path.as_deref()
    }
    
    fn use_incremental(&self) -> bool {
        self.use_incremental
    }
    
    fn config_hash(&self) -> String {
        use crate::utils::incremental_analyzer::calculate_hash;
        
        // Create a simple configuration object for hashing
        let config = (
            &self.exclude_dirs,
            &self.include_extensions,
        );
        
        calculate_hash(&config)
    }
    
    fn should_exclude_file(&self, file_path: &Path) -> bool {
        // Check if the file is in an excluded directory
        for dir in &self.exclude_dirs {
            if file_path.to_string_lossy().contains(dir) {
                return true;
            }
        }
        
        // Check if the file has an included extension
        if let Some(ext) = file_path.extension() {
            if let Some(ext_str) = ext.to_str() {
                return !self.include_extensions.contains(&ext_str.to_string());
            }
        }
        
        // Special case for common dependency files without extensions
        let filename = file_path.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
        let common_files = ["Gemfile", "Dockerfile", "docker-compose.yml"];
        
        if common_files.contains(&filename.as_str()) {
            return false;
        }
        
        true // Exclude by default if no extension
    }
    
    fn analyze_file(&self, file_path: &Path) -> Result<DependencyAnalysisResult> {
        let filename = file_path.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
        let source_file = file_path.to_string_lossy().to_string();
        
        // Read file content
        let content = fs::read_to_string(file_path)
            .context(format!("Failed to read file {}", file_path.display()))?;
        
        // Analyze based on file type
        if filename == "Gemfile" {
            Ok(self.analyze_gemfile(&content, source_file))
        } else if filename.ends_with(".gemspec") {
            Ok(self.analyze_gemspec(&content, source_file))
        } else if filename == "package.json" {
            Ok(self.analyze_package_json(&content, source_file))
        } else if filename == "requirements.txt" {
            Ok(self.analyze_requirements_txt(&content, source_file))
        } else if filename == "Cargo.toml" {
            Ok(self.analyze_cargo_toml(&content, source_file))
        } else if filename == "Dockerfile" {
            Ok(self.analyze_dockerfile(&content, source_file))
        } else {
            // Default empty result
            Ok(DependencyAnalysisResult::default())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::{tempdir, TempDir};
    use std::io::Write;
    use std::fs::File;
    
    fn create_test_file(dir: &Path, filename: &str, content: &str) -> PathBuf {
        let file_path = dir.join(filename);
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "{}", content).unwrap();
        file_path
    }
    
    fn setup_test_directory() -> (TempDir, IncrementalDependencyAnalyzer) {
        let dir = tempdir().unwrap();
        let analyzer = IncrementalDependencyAnalyzer::new(dir.path().to_path_buf());
        (dir, analyzer)
    }
    
    #[test]
    fn test_analyze_gemfile() -> Result<()> {
        let (dir, analyzer) = setup_test_directory();
        
        // Create a Gemfile
        let gemfile_content = r#"
        source 'https://rubygems.org'
        
        gem 'rails', '~> 6.1.0'
        gem 'pg', '~> 1.2.3'
        gem 'puma', '~> 5.0'
        gem 'sass-rails', '>= 6'
        gem 'webpacker', '~> 5.0'
        "#;
        
        let file_path = create_test_file(dir.path(), "Gemfile", gemfile_content);
        
        let result = analyzer.analyze_file(&file_path)?;
        
        assert_eq!(result.ruby_dependencies.len(), 5);
        assert!(result.ruby_dependencies.contains_key("rails"));
        assert!(result.ruby_dependencies.contains_key("pg"));
        
        Ok(())
    }
    
    #[test]
    fn test_analyze_package_json() -> Result<()> {
        let (dir, analyzer) = setup_test_directory();
        
        // Create a package.json
        let package_json_content = r#"
        {
          "name": "test-app",
          "version": "1.0.0",
          "dependencies": {
            "react": "^17.0.2",
            "react-dom": "^17.0.2",
            "axios": "^0.21.1"
          },
          "devDependencies": {
            "jest": "^27.0.6",
            "eslint": "^7.32.0"
          }
        }
        "#;
        
        let file_path = create_test_file(dir.path(), "package.json", package_json_content);
        
        let result = analyzer.analyze_file(&file_path)?;
        
        assert_eq!(result.js_dependencies.len(), 5);
        assert!(result.js_dependencies.contains_key("react"));
        assert!(result.js_dependencies.contains_key("jest"));
        
        Ok(())
    }
    
    #[test]
    fn test_incremental_analysis() -> Result<()> {
        let (dir, analyzer) = setup_test_directory();
        
        // Create a Gemfile
        let gemfile_content = r#"
        source 'https://rubygems.org'
        
        gem 'rails', '~> 6.1.0'
        gem 'pg', '~> 1.2.3'
        "#;
        
        let gemfile_path = create_test_file(dir.path(), "Gemfile", gemfile_content);
        
        // Create a package.json
        let package_json_content = r#"
        {
          "name": "test-app",
          "version": "1.0.0",
          "dependencies": {
            "react": "^17.0.2"
          }
        }
        "#;
        
        let package_json_path = create_test_file(dir.path(), "package.json", package_json_content);
        
        // First analysis
        let result1 = analyzer.analyze()?;
        
        // Check that dependencies were detected
        assert_eq!(result1.ruby_dependencies.len(), 2);
        assert_eq!(result1.js_dependencies.len(), 1);
        
        // Check that the cache file was created
        let cache_path = dir.path().join(".dependency_analyzer_cache.json");
        assert!(cache_path.exists());
        
        // Create a new analyzer with the same cache path
        let analyzer2 = IncrementalDependencyAnalyzer::new(dir.path().to_path_buf());
        
        // Second analysis - should use the cache
        let result2 = analyzer2.analyze()?;
        
        // Results should be the same
        assert_eq!(result1.ruby_dependencies.len(), result2.ruby_dependencies.len());
        assert_eq!(result1.js_dependencies.len(), result2.js_dependencies.len());
        
        // Modify the package.json file
        let new_package_json_content = r#"
        {
          "name": "test-app",
          "version": "1.0.0",
          "dependencies": {
            "react": "^17.0.2",
            "react-dom": "^17.0.2"
          }
        }
        "#;
        
        let _ = create_test_file(dir.path(), "package.json", new_package_json_content);
        
        // Third analysis - should detect the new dependency
        let result3 = analyzer2.analyze()?;
        
        // Should have one more JS dependency
        assert_eq!(result3.js_dependencies.len(), 2);
        assert!(result3.js_dependencies.contains_key("react-dom"));
        
        Ok(())
    }
}
