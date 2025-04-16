use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::fs;
use walkdir::WalkDir;
use regex::Regex;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Dependency {
    pub name: String,
    pub version: String,
    pub dependency_type: String,
    pub source_file: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DependencyGraph {
    pub nodes: Vec<String>,
    pub edges: Vec<(String, String)>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DependencyAnalyzer {
    pub ruby_dependencies: HashMap<String, Dependency>,
    pub js_dependencies: HashMap<String, Dependency>,
    pub python_dependencies: HashMap<String, Dependency>,
    pub system_dependencies: HashMap<String, Dependency>,
    pub dependency_graph: DependencyGraph,
}

impl DependencyAnalyzer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn analyze(&self, base_dir: &PathBuf) -> Result<String, String> {
        let mut analyzer = DependencyAnalyzer::default();
        
        // Analyze Ruby dependencies
        analyzer.analyze_ruby_dependencies(base_dir);
        
        // Analyze JavaScript dependencies
        analyzer.analyze_js_dependencies(base_dir);
        
        // Analyze Python dependencies
        analyzer.analyze_python_dependencies(base_dir);
        
        // Analyze system dependencies
        analyzer.analyze_system_dependencies(base_dir);
        
        // Build dependency graph
        analyzer.build_dependency_graph();
        
        match serde_json::to_string_pretty(&analyzer) {
            Ok(json) => Ok(json),
            Err(e) => Err(format!("Failed to serialize DependencyAnalyzer: {}", e)),
        }
    }
    
    fn analyze_ruby_dependencies(&mut self, base_dir: &PathBuf) {
        // Look for Gemfile
        let gemfile_path = base_dir.join("Gemfile");
        if gemfile_path.exists() {
            if let Ok(content) = fs::read_to_string(&gemfile_path) {
                self.analyze_gemfile(&content, gemfile_path.to_string_lossy().to_string());
            }
        }
        
        // Look for gemspec files
        for entry in WalkDir::new(base_dir)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |ext| ext == "gemspec") {
                if let Ok(content) = fs::read_to_string(path) {
                    self.analyze_gemspec(&content, path.to_string_lossy().to_string());
                }
            }
        }
    }
    
    fn analyze_gemfile(&mut self, content: &str, source_file: String) {
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
                
                self.ruby_dependencies.insert(name.as_str().to_string(), dependency);
            }
        }
    }
    
    fn analyze_gemspec(&mut self, content: &str, source_file: String) {
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
                
                self.ruby_dependencies.insert(name.as_str().to_string(), dependency);
            }
        }
    }
    
    fn analyze_js_dependencies(&mut self, base_dir: &PathBuf) {
        // Look for package.json
        let package_json_path = base_dir.join("package.json");
        if package_json_path.exists() {
            if let Ok(content) = fs::read_to_string(&package_json_path) {
                self.analyze_package_json(&content, package_json_path.to_string_lossy().to_string());
            }
        }
        
        // Look for yarn.lock
        let yarn_lock_path = base_dir.join("yarn.lock");
        if yarn_lock_path.exists() {
            if let Ok(content) = fs::read_to_string(&yarn_lock_path) {
                self.analyze_yarn_lock(&content);
            }
        }
        
        // Look for package-lock.json
        let package_lock_path = base_dir.join("package-lock.json");
        if package_lock_path.exists() {
            if let Ok(content) = fs::read_to_string(&package_lock_path) {
                self.analyze_package_lock(&content);
            }
        }
    }
    
    fn analyze_package_json(&mut self, content: &str, source_file: String) {
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
                        
                        self.js_dependencies.insert(name.clone(), dependency);
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
                        
                        self.js_dependencies.insert(name.clone(), dependency);
                    }
                }
            }
        }
    }
    
    fn analyze_yarn_lock(&mut self, content: &str) {
        // Extract package versions from yarn.lock
        let package_regex = Regex::new(r#"([^@\s"]+)@[^:]+:\s*\n\s+version\s+"([^"]+)"#).unwrap();
        
        for cap in package_regex.captures_iter(content) {
            if let (Some(name), Some(version)) = (cap.get(1), cap.get(2)) {
                // Only update if we already know about this dependency
                if self.js_dependencies.contains_key(name.as_str()) {
                    if let Some(dep) = self.js_dependencies.get_mut(name.as_str()) {
                        dep.version = version.as_str().to_string();
                    }
                }
            }
        }
    }
    
    fn analyze_package_lock(&mut self, content: &str) {
        // Try to parse as JSON
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(content) {
            // Extract dependencies
            if let Some(dependencies) = json.get("dependencies").and_then(|d| d.as_object()) {
                for (name, info) in dependencies {
                    if let Some(version) = info.get("version").and_then(|v| v.as_str()) {
                        // Only update if we already know about this dependency
                        if self.js_dependencies.contains_key(name) {
                            if let Some(dep) = self.js_dependencies.get_mut(name) {
                                dep.version = version.to_string();
                            }
                        }
                    }
                }
            }
        }
    }
    
    fn analyze_python_dependencies(&mut self, base_dir: &PathBuf) {
        // Look for requirements.txt
        let requirements_path = base_dir.join("requirements.txt");
        if requirements_path.exists() {
            if let Ok(content) = fs::read_to_string(&requirements_path) {
                self.analyze_requirements_txt(&content, requirements_path.to_string_lossy().to_string());
            }
        }
        
        // Look for setup.py
        let setup_py_path = base_dir.join("setup.py");
        if setup_py_path.exists() {
            if let Ok(content) = fs::read_to_string(&setup_py_path) {
                self.analyze_setup_py(&content, setup_py_path.to_string_lossy().to_string());
            }
        }
    }
    
    fn analyze_requirements_txt(&mut self, content: &str, source_file: String) {
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
                
                self.python_dependencies.insert(name.to_string(), dependency);
            }
        }
    }
    
    fn analyze_setup_py(&mut self, content: &str, source_file: String) {
        // Extract dependencies from setup.py
        let install_requires_regex = Regex::new(r#"install_requires\s*=\s*\[(.*?)\]"#).unwrap();
        
        if let Some(cap) = install_requires_regex.captures(content) {
            if let Some(requires) = cap.get(1) {
                let requires_str = requires.as_str();
                let package_regex = Regex::new(r#"['"]([^'"]+)['"](.*?)(?:,|$)"#).unwrap();
                
                for pkg_cap in package_regex.captures_iter(requires_str) {
                    if let Some(name) = pkg_cap.get(1) {
                        let version_constraint = pkg_cap.get(2).map_or("", |m| m.as_str().trim());
                        
                        let dependency = Dependency {
                            name: name.as_str().to_string(),
                            version: if version_constraint.is_empty() { "latest".to_string() } else { version_constraint.to_string() },
                            dependency_type: "python".to_string(),
                            source_file: source_file.clone(),
                        };
                        
                        self.python_dependencies.insert(name.as_str().to_string(), dependency);
                    }
                }
            }
        }
    }
    
    fn analyze_system_dependencies(&mut self, base_dir: &PathBuf) {
        // Look for Dockerfile
        let dockerfile_path = base_dir.join("Dockerfile");
        if dockerfile_path.exists() {
            if let Ok(content) = fs::read_to_string(&dockerfile_path) {
                self.analyze_dockerfile(&content, dockerfile_path.to_string_lossy().to_string());
            }
        }
        
        // Look for docker-compose.yml
        let docker_compose_path = base_dir.join("docker-compose.yml");
        if docker_compose_path.exists() {
            if let Ok(content) = fs::read_to_string(&docker_compose_path) {
                self.analyze_docker_compose(&content, docker_compose_path.to_string_lossy().to_string());
            }
        }
    }
    
    fn analyze_dockerfile(&mut self, content: &str, source_file: String) {
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
                        
                        self.system_dependencies.insert(pkg.to_string(), dependency);
                    }
                }
            }
        }
    }
    
    fn analyze_docker_compose(&mut self, content: &str, source_file: String) {
        // Extract system dependencies from docker-compose.yml
        // This is a simplified approach, as docker-compose.yml doesn't directly list system dependencies
        
        // Try to parse as YAML
        if let Ok(yaml) = serde_yaml::from_str::<serde_yaml::Value>(content) {
            // Extract services
            if let Some(services) = yaml.get("services").and_then(|s| s.as_mapping()) {
                for (service_name, service_config) in services {
                    if let Some(service_name_str) = service_name.as_str() {
                        // Extract image
                        if let Some(image) = service_config.get("image").and_then(|i| i.as_str()) {
                            let dependency = Dependency {
                                name: format!("docker-image:{}", image),
                                version: "latest".to_string(),
                                dependency_type: "system-docker".to_string(),
                                source_file: source_file.clone(),
                            };
                            
                            self.system_dependencies.insert(format!("docker-service:{}", service_name_str), dependency);
                        }
                    }
                }
            }
        }
    }
    
    fn build_dependency_graph(&mut self) {
        let mut nodes = Vec::new();
        let mut edges = Vec::new();
        
        // Add all dependencies as nodes
        for (name, _) in &self.ruby_dependencies {
            nodes.push(name.clone());
        }
        
        for (name, _) in &self.js_dependencies {
            nodes.push(name.clone());
        }
        
        for (name, _) in &self.python_dependencies {
            nodes.push(name.clone());
        }
        
        for (name, _) in &self.system_dependencies {
            nodes.push(name.clone());
        }
        
        // Add edges based on known relationships
        // This is a simplified approach, as we don't have full dependency tree information
        
        // For Ruby, we can infer some relationships
        for (name, dep) in &self.ruby_dependencies {
            if name.contains("rails") {
                // Rails has many dependencies
                for (other_name, _) in &self.ruby_dependencies {
                    if other_name.contains("active") || other_name.contains("action") {
                        edges.push((name.clone(), other_name.clone()));
                    }
                }
            }
        }
        
        // For JS, we can infer some relationships
        for (name, dep) in &self.js_dependencies {
            if name == "react" {
                // React has some common dependencies
                for (other_name, _) in &self.js_dependencies {
                    if other_name.contains("react-") {
                        edges.push((name.clone(), other_name.clone()));
                    }
                }
            }
        }
        
        self.dependency_graph = DependencyGraph { nodes, edges };
    }
}
