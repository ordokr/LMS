use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use anyhow::{Result, Context};
use regex::Regex;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Feature {
    pub source: String,
    pub name: String,
    pub category: String,
    pub source_files: Vec<String>,
    pub related_entities: Vec<String>,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureMapping {
    pub source_feature: String,
    pub target_feature: String,
    pub confidence: f32,
    pub status: String,
    pub priority: u8,
}

/// Cache entry for a file
#[derive(Debug, Clone, Serialize, Deserialize)]
struct FileCache {
    /// Last modified time
    last_modified: u64,
    /// Extracted features
    features: Vec<Feature>,
}

#[derive(Debug)]
pub struct FeatureDetector {
    pub features: HashMap<String, Vec<Feature>>,
    pub mappings: Vec<FeatureMapping>,
    pub categories: Vec<String>,
    /// File cache to avoid re-analyzing unchanged files
    file_cache: HashMap<String, FileCache>,
    /// Whether to use caching
    use_cache: bool,
}

impl FeatureDetector {
    pub fn new() -> Self {
        let categories = vec![
            "course_mgmt".to_string(),
            "assignment_mgmt".to_string(),
            "grading".to_string(),
            "discussions".to_string(),
            "auth".to_string(),
            "roles".to_string(),
            "moderation".to_string(),
            "tagging".to_string(),
            "other".to_string(),
        ];

        FeatureDetector {
            features: HashMap::new(),
            mappings: Vec::new(),
            categories,
            file_cache: HashMap::new(),
            use_cache: true,
        }
    }

    /// Create a new FeatureDetector with caching disabled
    pub fn new_without_cache() -> Self {
        let mut detector = Self::new();
        detector.use_cache = false;
        detector
    }

    pub fn analyze(&mut self, path: &str) -> Result<()> {
        println!("Analyzing features in: {}", path);

        // Detect features in the current project
        self.detect_features_in_project(path, "ordo")?;

        // Detect features in Canvas
        let canvas_path = "C:\\Users\\Tim\\Desktop\\port\\canvas";
        if Path::new(canvas_path).exists() {
            self.detect_features_in_project(canvas_path, "canvas")?;
        } else {
            println!("Canvas path not found: {}", canvas_path);
        }

        // Detect features in Discourse
        let discourse_path = "C:\\Users\\Tim\\Desktop\\port\\discourse";
        if Path::new(discourse_path).exists() {
            self.detect_features_in_project(discourse_path, "discourse")?;
        } else {
            println!("Discourse path not found: {}", discourse_path);
        }

        // Generate mappings between features
        self.generate_mappings()?;

        Ok(())
    }

    fn detect_features_in_project(&mut self, project_path: &str, source: &str) -> Result<()> {
        println!("Detecting features in {} project: {}", source, project_path);

        let mut features = Vec::new();
        let project_dir = PathBuf::from(project_path);

        // Check for Ruby on Rails project
        let routes_file = project_dir.join("config").join("routes.rb");
        if routes_file.exists() {
            println!("Detected Ruby on Rails project");
            self.extract_ruby_routes(&routes_file, source, &mut features)?;

            // Extract features from views
            let views_dir = project_dir.join("app").join("views");
            if views_dir.exists() {
                self.extract_ruby_views(&views_dir, source, &mut features)?;
            }
        }

        // Check for Rust project
        let src_dir = project_dir.join("src");
        if src_dir.exists() {
            println!("Detected Rust project");
            self.extract_rust_features(&src_dir, source, &mut features)?;
        }

        // Store features for this source
        self.features.insert(source.to_string(), features);

        Ok(())
    }

    fn walk_directory<F>(&self, dir: &Path, mut callback: F) -> Result<()>
    where
        F: FnMut(&Path),
    {
        if dir.is_dir() {
            // First collect all files to avoid borrow issues
            let mut all_files = Vec::new();
            let mut dirs_to_process = vec![dir.to_path_buf()];

            while let Some(current_dir) = dirs_to_process.pop() {
                for entry in fs::read_dir(&current_dir)? {
                    let entry = entry?;
                    let path = entry.path();

                    if path.is_dir() {
                        dirs_to_process.push(path);
                    } else {
                        all_files.push(path);
                    }
                }
            }

            // Now process all files
            for file in all_files {
                callback(&file);
            }
        }

        Ok(())
    }

    /// Extract features from Ruby routes
    fn extract_ruby_routes(&mut self, routes_file: &Path, source: &str, features: &mut Vec<Feature>) -> Result<()> {
        println!("Extracting features from Ruby routes in: {}", routes_file.display());

        // Check if file is in cache and hasn't been modified
        let file_path_str = routes_file.to_string_lossy().to_string();
        let should_parse = if self.use_cache {
            match fs::metadata(routes_file) {
                Ok(metadata) => {
                    if let Ok(modified_time) = metadata.modified() {
                        if let Ok(modified_secs) = modified_time.duration_since(SystemTime::UNIX_EPOCH) {
                            let modified_secs = modified_secs.as_secs();
                            if let Some(cache) = self.file_cache.get(&file_path_str) {
                                if cache.last_modified >= modified_secs {
                                    // File hasn't been modified, use cached features
                                    features.extend(cache.features.clone());
                                    println!("Using cached features for: {}", file_path_str);
                                    return Ok(());
                                }
                            }
                        }
                    }
                    true
                },
                Err(_) => true,
            }
        } else {
            true
        };

        if !should_parse {
            return Ok(());
        }

        let mut extracted_features = Vec::new();
        if let Ok(content) = fs::read_to_string(routes_file) {
            // Extract routes - using a very simple regex
            let route_regex = Regex::new(r"resources\s+:(\w+)").ok();

            if let Some(regex) = route_regex {
                for captures in regex.captures_iter(&content) {
                    if let Some(resource) = captures.get(1) {
                        let resource_name = resource.as_str();
                        println!("Found RESTful resource: {}", resource_name);

                        // Create features for standard RESTful actions
                        let restful_actions = vec!["index", "show", "new", "create", "edit", "update", "destroy"];
                        let category = self.determine_category(resource_name);

                        for action in restful_actions {
                            let feature_name = format!("{}_{}_{}", resource_name, action, "route");
                            println!("  Creating feature: {}", feature_name);

                            let mut metadata = HashMap::new();
                            metadata.insert("resource".to_string(), resource_name.to_string());
                            metadata.insert("action".to_string(), action.to_string());
                            metadata.insert("type".to_string(), "restful".to_string());

                            extracted_features.push(Feature {
                                source: source.to_string(),
                                name: feature_name,
                                category: category.clone(),
                                source_files: vec![routes_file.to_string_lossy().to_string()],
                                related_entities: Vec::new(),
                                metadata,
                            });
                        }
                    }
                }
            }
        }

        // Update cache
        if self.use_cache {
            if let Ok(metadata) = fs::metadata(routes_file) {
                if let Ok(modified_time) = metadata.modified() {
                    if let Ok(modified_secs) = modified_time.duration_since(SystemTime::UNIX_EPOCH) {
                        let modified_secs = modified_secs.as_secs();
                        self.file_cache.insert(file_path_str, FileCache {
                            last_modified: modified_secs,
                            features: extracted_features.clone(),
                        });
                    }
                }
            }
        }

        // Add extracted features to the result
        features.extend(extracted_features);

        Ok(())
    }

    /// Extract features from Ruby views
    fn extract_ruby_views(&mut self, views_dir: &Path, source: &str, features: &mut Vec<Feature>) -> Result<()> {
        println!("Extracting features from Ruby views in: {}", views_dir.display());

        // First collect all view files
        let mut view_files = Vec::new();
        self.walk_directory(views_dir, |file_path| {
            if let Some(ext) = file_path.extension() {
                if ext == "erb" || ext == "html" || ext == "haml" || ext == "slim" {
                    view_files.push(file_path.to_path_buf());
                }
            }
        })?;

        let mut all_extracted_features = Vec::new();

        // Now process each file
        for file_path in view_files {
            // Extract view path relative to views directory
            if let Ok(relative_path) = file_path.strip_prefix(views_dir) {
                let path_str = relative_path.to_string_lossy().to_string();
                println!("Found view: {}", path_str);

                // Parse path to determine controller and action
                let parts: Vec<&str> = path_str.split('/').collect();
                if parts.len() >= 2 {
                    let controller = parts[0];
                    let action = parts[1].split('.').next().unwrap_or("");

                    if !controller.is_empty() && !action.is_empty() {
                        let feature_name = format!("{}_{}_{}", controller, action, "view");
                        let category = self.determine_category(controller);

                        println!("  Creating feature: {}", feature_name);

                        let mut metadata = HashMap::new();
                        metadata.insert("controller".to_string(), controller.to_string());
                        metadata.insert("action".to_string(), action.to_string());
                        metadata.insert("view_path".to_string(), path_str);

                        // Check if file is in cache and hasn't been modified
                        let file_path_str = file_path.to_string_lossy().to_string();
                        let should_parse = if self.use_cache {
                            match fs::metadata(&file_path) {
                                Ok(metadata) => {
                                    if let Ok(modified_time) = metadata.modified() {
                                        if let Ok(modified_secs) = modified_time.duration_since(SystemTime::UNIX_EPOCH) {
                                            let modified_secs = modified_secs.as_secs();
                                            if let Some(cache) = self.file_cache.get(&file_path_str) {
                                                if cache.last_modified >= modified_secs {
                                                    // File hasn't been modified, use cached features
                                                    all_extracted_features.extend(cache.features.clone());
                                                    println!("Using cached features for: {}", file_path_str);
                                                    continue;
                                                }
                                            }
                                        }
                                    }
                                    true
                                },
                                Err(_) => true,
                            }
                        } else {
                            true
                        };

                        if !should_parse {
                            continue;
                        }

                        let mut extracted_features = Vec::new();
                        extracted_features.push(Feature {
                            source: source.to_string(),
                            name: feature_name,
                            category,
                            source_files: vec![file_path.to_string_lossy().to_string()],
                            related_entities: Vec::new(),
                            metadata,
                        });

                        // Update cache
                        if self.use_cache {
                            if let Ok(metadata) = fs::metadata(&file_path) {
                                if let Ok(modified_time) = metadata.modified() {
                                    if let Ok(modified_secs) = modified_time.duration_since(SystemTime::UNIX_EPOCH) {
                                        let modified_secs = modified_secs.as_secs();
                                        self.file_cache.insert(file_path_str, FileCache {
                                            last_modified: modified_secs,
                                            features: extracted_features.clone(),
                                        });
                                    }
                                }
                            }
                        }

                        // Add extracted features to the result
                        all_extracted_features.extend(extracted_features);
                    }
                }
            }
        }

        // Add all extracted features to the result
        features.extend(all_extracted_features);

        Ok(())
    }

    /// Extract features from Rust code
    fn extract_rust_features(&mut self, src_dir: &Path, source: &str, features: &mut Vec<Feature>) -> Result<()> {
        println!("Extracting features from Rust code in: {}", src_dir.display());

        // First collect all Rust files
        let mut rust_files = Vec::new();
        self.walk_directory(src_dir, |file_path| {
            if let Some(ext) = file_path.extension() {
                if ext == "rs" {
                    rust_files.push(file_path.to_path_buf());
                }
            }
        })?;

        let mut all_extracted_features = Vec::new();

        // Now process each file
        for file_path in rust_files {
            // Check if file is in cache and hasn't been modified
            let file_path_str = file_path.to_string_lossy().to_string();
            let should_parse = if self.use_cache {
                match fs::metadata(&file_path) {
                    Ok(metadata) => {
                        if let Ok(modified_time) = metadata.modified() {
                            if let Ok(modified_secs) = modified_time.duration_since(SystemTime::UNIX_EPOCH) {
                                let modified_secs = modified_secs.as_secs();
                                if let Some(cache) = self.file_cache.get(&file_path_str) {
                                    if cache.last_modified >= modified_secs {
                                        // File hasn't been modified, use cached features
                                        all_extracted_features.extend(cache.features.clone());
                                        println!("Using cached features for: {}", file_path_str);
                                        continue;
                                    }
                                }
                            }
                        }
                        true
                    },
                    Err(_) => true,
                }
            } else {
                true
            };

            if !should_parse {
                continue;
            }

            let mut extracted_features = Vec::new();
            if let Ok(content) = fs::read_to_string(&file_path) {
                // Extract module name
                let module_regex = Regex::new(r"(?:pub\s+)?mod\s+(\w+)").ok();

                if let Some(regex) = module_regex {
                    for captures in regex.captures_iter(&content) {
                        let module_name = captures.get(1).map_or("", |m| m.as_str());
                        println!("Found module: {}", module_name);

                        // Extract functions
                        let fn_regex = Regex::new(r"(?:pub\s+)?fn\s+(\w+)").ok();

                        if let Some(fn_re) = fn_regex {
                            let functions: Vec<String> = fn_re.captures_iter(&content)
                                .filter_map(|cap| cap.get(1).map(|m| m.as_str().to_string()))
                                .collect();

                            println!("  Found {} functions", functions.len());

                            // Determine category based on module name
                            let category = self.determine_category(module_name);

                            // Create a feature for each function
                            for function in functions {
                                let feature_name = format!("{}_{}_{}", module_name.to_lowercase(), function, "function");
                                println!("  Creating feature: {}", feature_name);

                                let mut metadata = HashMap::new();
                                metadata.insert("module".to_string(), module_name.to_string());
                                metadata.insert("function".to_string(), function.clone());

                                extracted_features.push(Feature {
                                    source: source.to_string(),
                                    name: feature_name,
                                    category: category.clone(),
                                    source_files: vec![file_path.to_string_lossy().to_string()],
                                    related_entities: Vec::new(),
                                    metadata,
                                });
                            }
                        }

                        // Extract structs
                        let struct_regex = Regex::new(r"(?:pub\s+)?struct\s+(\w+)").ok();

                        if let Some(struct_re) = struct_regex {
                            let structs: Vec<String> = struct_re.captures_iter(&content)
                                .filter_map(|cap| cap.get(1).map(|m| m.as_str().to_string()))
                                .collect();

                            println!("  Found {} structs", structs.len());

                            // Create a feature for each struct
                            for struct_name in structs {
                                let feature_name = format!("{}_{}_{}", module_name.to_lowercase(), struct_name.to_lowercase(), "struct");
                                println!("  Creating feature: {}", feature_name);

                                let mut metadata = HashMap::new();
                                metadata.insert("module".to_string(), module_name.to_string());
                                metadata.insert("struct".to_string(), struct_name.clone());

                                extracted_features.push(Feature {
                                    source: source.to_string(),
                                    name: feature_name,
                                    category: self.determine_category(&struct_name),
                                    source_files: vec![file_path.to_string_lossy().to_string()],
                                    related_entities: Vec::new(),
                                    metadata,
                                });
                            }
                        }
                    }
                }
            }

            // Update cache
            if self.use_cache {
                if let Ok(metadata) = fs::metadata(&file_path) {
                    if let Ok(modified_time) = metadata.modified() {
                        if let Ok(modified_secs) = modified_time.duration_since(SystemTime::UNIX_EPOCH) {
                            let modified_secs = modified_secs.as_secs();
                            self.file_cache.insert(file_path_str, FileCache {
                                last_modified: modified_secs,
                                features: extracted_features.clone(),
                            });
                        }
                    }
                }
            }

            // Add extracted features to the result
            all_extracted_features.extend(extracted_features);
        }

        // Add all extracted features to the result
        features.extend(all_extracted_features);

        Ok(())
    }

    /// Determine the category of a feature based on its name
    fn determine_category(&self, name: &str) -> String {
        let name_lower = name.to_lowercase();

        if name_lower.contains("course") {
            return "course_mgmt".to_string();
        } else if name_lower.contains("assignment") {
            return "assignment_mgmt".to_string();
        } else if name_lower.contains("grade") || name_lower.contains("score") || name_lower.contains("submission") {
            return "grading".to_string();
        } else if name_lower.contains("discussion") || name_lower.contains("topic") || name_lower.contains("post") || name_lower.contains("comment") {
            return "discussions".to_string();
        } else if name_lower.contains("auth") || name_lower.contains("login") || name_lower.contains("user") || name_lower.contains("account") {
            return "auth".to_string();
        } else if name_lower.contains("role") || name_lower.contains("permission") || name_lower.contains("admin") {
            return "roles".to_string();
        } else if name_lower.contains("moderat") || name_lower.contains("flag") || name_lower.contains("report") {
            return "moderation".to_string();
        } else if name_lower.contains("tag") || name_lower.contains("categor") || name_lower.contains("label") {
            return "tagging".to_string();
        }

        // Default category
        "other".to_string()
    }

    /// Generate mappings between features from different systems
    pub fn generate_mappings(&mut self) -> Result<()> {
        println!("Generating feature mappings...");

        let mut mappings = Vec::new();

        // Get features from each source
        let canvas_features = self.features.get("canvas").cloned().unwrap_or_default();
        let discourse_features = self.features.get("discourse").cloned().unwrap_or_default();
        let ordo_features = self.features.get("ordo").cloned().unwrap_or_default();

        // Map Canvas features to Ordo
        for canvas_feature in &canvas_features {
            if let Some(mapping) = self.map_feature(canvas_feature, &ordo_features) {
                mappings.push(mapping);
            }
        }

        // Map Discourse features to Ordo
        for discourse_feature in &discourse_features {
            if let Some(mapping) = self.map_feature(discourse_feature, &ordo_features) {
                mappings.push(mapping);
            }
        }

        self.mappings = mappings;

        println!("Generated {} feature mappings", self.mappings.len());

        Ok(())
    }

    /// Map a source feature to a target feature
    fn map_feature(&self, source_feature: &Feature, target_features: &[Feature]) -> Option<FeatureMapping> {
        // First try exact name match
        let exact_match = target_features.iter().find(|f| f.name == source_feature.name);

        if let Some(target) = exact_match {
            return Some(FeatureMapping {
                source_feature: format!("{}.{}", source_feature.source, source_feature.name),
                target_feature: format!("{}.{}", target.source, target.name),
                confidence: 1.0,
                status: "implemented".to_string(),
                priority: 1,
            });
        }

        // Try category match
        let category_matches: Vec<&Feature> = target_features.iter()
            .filter(|f| f.category == source_feature.category)
            .collect();

        if !category_matches.is_empty() {
            // Find the best match based on name similarity
            let mut best_match = None;
            let mut best_score = 0.3; // Minimum threshold for a match

            for target in &category_matches {
                let score = self.calculate_name_similarity(&source_feature.name, &target.name);

                if score > best_score {
                    best_score = score;
                    best_match = Some(*target);
                }
            }

            if let Some(target) = best_match {
                return Some(FeatureMapping {
                    source_feature: format!("{}.{}", source_feature.source, source_feature.name),
                    target_feature: format!("{}.{}", target.source, target.name),
                    confidence: best_score,
                    status: "partial".to_string(),
                    priority: 2,
                });
            }
        }

        // No match found, mark as missing
        Some(FeatureMapping {
            source_feature: format!("{}.{}", source_feature.source, source_feature.name),
            target_feature: "".to_string(),
            confidence: 0.0,
            status: "missing".to_string(),
            priority: self.calculate_priority(source_feature),
        })
    }

    /// Calculate the similarity between two feature names
    fn calculate_name_similarity(&self, name1: &str, name2: &str) -> f32 {
        // Simple similarity check - could be improved with more sophisticated algorithms
        let name1_lower = name1.to_lowercase();
        let name2_lower = name2.to_lowercase();

        // Exact match
        if name1_lower == name2_lower {
            return 1.0;
        }

        // Split into parts and check for common words
        let parts1: HashSet<&str> = name1_lower.split('_').collect();
        let parts2: HashSet<&str> = name2_lower.split('_').collect();

        let common_parts = parts1.intersection(&parts2).count();
        let total_parts = parts1.len() + parts2.len();

        if total_parts > 0 {
            (common_parts * 2) as f32 / total_parts as f32
        } else {
            0.0
        }
    }

    /// Calculate the priority of a missing feature
    fn calculate_priority(&self, feature: &Feature) -> u8 {
        // Assign priority based on category
        match feature.category.as_str() {
            "course_mgmt" => 5,     // Highest priority
            "assignment_mgmt" => 5,
            "grading" => 4,
            "discussions" => 4,
            "auth" => 5,
            "roles" => 3,
            "moderation" => 2,
            "tagging" => 2,
            _ => 1,                // Lowest priority
        }
    }

    /// Generate a JSON report of feature mappings
    pub fn generate_mapping_report(&self) -> Result<String> {
        let report = serde_json::to_string_pretty(&self.mappings)?;
        Ok(report)
    }

    /// Generate a Markdown report of feature mappings
    /// Enable or disable caching
    pub fn set_use_cache(&mut self, use_cache: bool) {
        self.use_cache = use_cache;
    }

    /// Clear the file cache
    pub fn clear_cache(&mut self) {
        self.file_cache.clear();
    }

    pub fn generate_mapping_markdown(&self) -> String {
        let mut markdown = String::new();

        markdown.push_str("# Feature Mapping Report\n\n");

        // Summary statistics
        let canvas_count = self.features.get("canvas").map(|e| e.len()).unwrap_or(0);
        let discourse_count = self.features.get("discourse").map(|e| e.len()).unwrap_or(0);
        let ordo_count = self.features.get("ordo").map(|e| e.len()).unwrap_or(0);

        markdown.push_str("## Summary\n\n");
        markdown.push_str(&format!("- Canvas Features: {}\n", canvas_count));
        markdown.push_str(&format!("- Discourse Features: {}\n", discourse_count));
        markdown.push_str(&format!("- Ordo Features: {}\n", ordo_count));
        markdown.push_str(&format!("- Total Mappings: {}\n\n", self.mappings.len()));

        // Implementation status
        let implemented = self.mappings.iter().filter(|m| m.status == "implemented").count();
        let partial = self.mappings.iter().filter(|m| m.status == "partial").count();
        let missing = self.mappings.iter().filter(|m| m.status == "missing").count();

        markdown.push_str("## Implementation Status\n\n");
        markdown.push_str(&format!("- Implemented: {} ({:.1}%)\n",
            implemented,
            if self.mappings.len() > 0 { implemented as f32 / self.mappings.len() as f32 * 100.0 } else { 0.0 }));
        markdown.push_str(&format!("- Partial: {} ({:.1}%)\n",
            partial,
            if self.mappings.len() > 0 { partial as f32 / self.mappings.len() as f32 * 100.0 } else { 0.0 }));
        markdown.push_str(&format!("- Missing: {} ({:.1}%)\n\n",
            missing,
            if self.mappings.len() > 0 { missing as f32 / self.mappings.len() as f32 * 100.0 } else { 0.0 }));

        // Feature mappings by category
        markdown.push_str("## Feature Mappings by Category\n\n");

        for category in &self.categories {
            let category_mappings: Vec<&FeatureMapping> = self.mappings.iter()
                .filter(|m| {
                    let source_parts: Vec<&str> = m.source_feature.split(".").collect();
                    if source_parts.len() >= 2 {
                        let source_name = source_parts[1];
                        let source_feature = self.find_feature_by_name(source_name);
                        source_feature.map_or(false, |f| f.category == *category)
                    } else {
                        false
                    }
                })
                .collect();

            if !category_mappings.is_empty() {
                markdown.push_str(&format!("### {}\n\n", category));
                markdown.push_str("| Source Feature | Target Feature | Status | Confidence | Priority |\n");
                markdown.push_str("|---------------|----------------|--------|------------|----------|\n");

                for mapping in category_mappings {
                    let source_name = mapping.source_feature.split(".").nth(1).unwrap_or("");
                    let target_name = if mapping.target_feature.is_empty() {
                        "Not implemented"
                    } else {
                        mapping.target_feature.split(".").nth(1).unwrap_or("")
                    };

                    markdown.push_str(&format!(
                        "| {} | {} | {} | {:.2} | {} |\n",
                        source_name,
                        target_name,
                        mapping.status,
                        mapping.confidence,
                        mapping.priority
                    ));
                }

                markdown.push_str("\n");
            }
        }

        // Missing features by priority
        markdown.push_str("## Missing Features by Priority\n\n");

        for priority in (1..=5).rev() {
            let priority_mappings: Vec<&FeatureMapping> = self.mappings.iter()
                .filter(|m| m.status == "missing" && m.priority == priority)
                .collect();

            if !priority_mappings.is_empty() {
                markdown.push_str(&format!("### Priority {}\n\n", priority));
                markdown.push_str("| Feature | Category |\n");
                markdown.push_str("|---------|----------|\n");

                for mapping in priority_mappings {
                    let source_name = mapping.source_feature.split(".").nth(1).unwrap_or("");
                    let source_feature = self.find_feature_by_name(source_name);
                    let category = source_feature.map_or("unknown".to_string(), |f| f.category.clone());

                    markdown.push_str(&format!("| {} | {} |\n", source_name, category));
                }

                markdown.push_str("\n");
            }
        }

        markdown
    }

    /// Find a feature by name
    fn find_feature_by_name(&self, name: &str) -> Option<&Feature> {
        for features in self.features.values() {
            for feature in features {
                if feature.name == name {
                    return Some(feature);
                }
            }
        }
        None
    }
}
