use std::path::{Path, PathBuf};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::fs;
use walkdir::WalkDir;
use regex::Regex;
use anyhow::{Result, Context};

use crate::utils::incremental_analyzer::{IncrementalAnalyzer, AnalysisCache};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateBinding {
    pub name: String,
    pub binding_type: String,
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplatePartial {
    pub name: String,
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateLoop {
    pub iterator: String,
    pub collection: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateConditional {
    pub condition: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateAnalysis {
    pub path: String,
    pub template_type: String,
    pub bindings: Vec<TemplateBinding>,
    pub partials: Vec<TemplatePartial>,
    pub loops: Vec<TemplateLoop>,
    pub conditionals: Vec<TemplateConditional>,
}

impl Default for TemplateAnalysis {
    fn default() -> Self {
        Self {
            path: String::new(),
            template_type: String::new(),
            bindings: Vec::new(),
            partials: Vec::new(),
            loops: Vec::new(),
            conditionals: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TemplateAnalysisResult {
    pub templates: HashMap<String, TemplateAnalysis>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IncrementalTemplateAnalyzer {
    pub base_dir: PathBuf,
    pub use_incremental: bool,
    pub cache_path: Option<PathBuf>,
    pub exclude_dirs: Vec<String>,
    pub include_extensions: Vec<String>,
}

impl Default for IncrementalTemplateAnalyzer {
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
                "erb".to_string(),
                "hbs".to_string(),
                "html".to_string(),
                "haml".to_string(),
                "slim".to_string(),
                "vue".to_string(),
                "jsx".to_string(),
                "tsx".to_string(),
            ],
        }
    }
}

impl IncrementalTemplateAnalyzer {
    pub fn new(base_dir: PathBuf) -> Self {
        let mut analyzer = Self::default();
        analyzer.base_dir = base_dir.clone();
        analyzer.cache_path = Some(base_dir.join(".template_analyzer_cache.json"));
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

    pub fn analyze(&self) -> Result<TemplateAnalysisResult> {
        // Collect all template files
        let mut files_to_analyze = Vec::new();
        
        // Walk the directory tree to collect files
        for entry in WalkDir::new(&self.base_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
        {
            let file_path = entry.path();
            
            // Skip excluded files
            if !self.should_exclude_file(file_path) {
                files_to_analyze.push(file_path.to_path_buf());
            }
        }
        
        // Analyze files incrementally
        let file_results = self.analyze_files_incrementally(&files_to_analyze)?;
        
        // Combine results
        let mut combined_result = TemplateAnalysisResult::default();
        
        for result in file_results {
            // Combine templates
            for (key, template) in result.templates {
                combined_result.templates.insert(key, template);
            }
        }
        
        Ok(combined_result)
    }
    
    fn analyze_template(&self, content: &str, template_type: &str) -> TemplateAnalysis {
        let mut analysis = TemplateAnalysis {
            template_type: template_type.to_string(),
            ..Default::default()
        };

        match template_type {
            "erb" => self.analyze_erb(&mut analysis, content),
            "hbs" => self.analyze_hbs(&mut analysis, content),
            "html" => self.analyze_html(&mut analysis, content),
            "vue" => {
                // Vue files can contain both HTML and JavaScript
                self.analyze_html(&mut analysis, content);
                // Look for Vue-specific patterns
                self.analyze_vue(&mut analysis, content);
            },
            "jsx" | "tsx" => self.analyze_jsx(&mut analysis, content),
            _ => {}
        }

        analysis
    }

    fn analyze_erb(&self, analysis: &mut TemplateAnalysis, content: &str) {
        // ERB binding pattern: <%= variable %> or <%= method(args) %>
        let binding_regex = Regex::new(r#"<%=\s*([^%]+?)\s*%>"#).unwrap();
        for cap in binding_regex.captures_iter(content) {
            if let Some(binding) = cap.get(1) {
                analysis.bindings.push(TemplateBinding {
                    name: binding.as_str().trim().to_string(),
                    binding_type: "erb".to_string(),
                    source: "rails".to_string(),
                });
            }
        }

        // ERB partial pattern: <%= render partial: 'path/to/partial' %>
        let partial_regex = Regex::new(r#"<%=\s*render\s+partial:\s*['"](.*?)['"](\s*%>)"#).unwrap();
        for cap in partial_regex.captures_iter(content) {
            if let Some(partial) = cap.get(1) {
                analysis.partials.push(TemplatePartial {
                    name: partial.as_str().split('/').last().unwrap_or(partial.as_str()).to_string(),
                    path: partial.as_str().to_string(),
                });
            }
        }

        // ERB loop pattern: <% collection.each do |item| %> ... <% end %>
        let loop_regex = Regex::new(r#"<%\s*([^%]+?)\.each\s+do\s+\|([^|]+?)\|\s*%>"#).unwrap();
        for cap in loop_regex.captures_iter(content) {
            if let (Some(collection), Some(iterator)) = (cap.get(1), cap.get(2)) {
                analysis.loops.push(TemplateLoop {
                    iterator: iterator.as_str().trim().to_string(),
                    collection: collection.as_str().trim().to_string(),
                });
            }
        }

        // ERB conditional pattern: <% if condition %> ... <% end %>
        let conditional_regex = Regex::new(r#"<%\s*if\s+([^%]+?)\s*%>"#).unwrap();
        for cap in conditional_regex.captures_iter(content) {
            if let Some(condition) = cap.get(1) {
                analysis.conditionals.push(TemplateConditional {
                    condition: condition.as_str().trim().to_string(),
                });
            }
        }
    }

    fn analyze_hbs(&self, analysis: &mut TemplateAnalysis, content: &str) {
        // Handlebars binding pattern: {{variable}} or {{{variable}}}
        let binding_regex = Regex::new(r#"\{\{\{?([^}]+?)\}?\}\}"#).unwrap();
        for cap in binding_regex.captures_iter(content) {
            if let Some(binding) = cap.get(1) {
                let binding_str = binding.as_str().trim();
                if !binding_str.starts_with("#") && !binding_str.starts_with("/") {
                    analysis.bindings.push(TemplateBinding {
                        name: binding_str.to_string(),
                        binding_type: "hbs".to_string(),
                        source: "ember".to_string(),
                    });
                }
            }
        }

        // Handlebars partial pattern: {{> partial}}
        let partial_regex = Regex::new(r#"\{\{>\s*([^}]+?)\s*\}\}"#).unwrap();
        for cap in partial_regex.captures_iter(content) {
            if let Some(partial) = cap.get(1) {
                analysis.partials.push(TemplatePartial {
                    name: partial.as_str().trim().to_string(),
                    path: partial.as_str().trim().to_string(),
                });
            }
        }

        // Handlebars loop pattern: {{#each items as |item|}}
        let loop_regex = Regex::new(r#"\{\{#each\s+([^\s]+)\s+as\s+\|([^|]+?)\|\}\}"#).unwrap();
        for cap in loop_regex.captures_iter(content) {
            if let (Some(collection), Some(iterator)) = (cap.get(1), cap.get(2)) {
                analysis.loops.push(TemplateLoop {
                    iterator: iterator.as_str().trim().to_string(),
                    collection: collection.as_str().trim().to_string(),
                });
            }
        }

        // Handlebars conditional pattern: {{#if condition}}
        let conditional_regex = Regex::new(r#"\{\{#if\s+([^}]+?)\s*\}\}"#).unwrap();
        for cap in conditional_regex.captures_iter(content) {
            if let Some(condition) = cap.get(1) {
                analysis.conditionals.push(TemplateConditional {
                    condition: condition.as_str().trim().to_string(),
                });
            }
        }
    }

    fn analyze_html(&self, analysis: &mut TemplateAnalysis, content: &str) {
        // Look for data-* attributes which might indicate bindings
        let data_attr_regex = Regex::new(r#"data-([\w-]+)=['"](.*?)['"](\s*)"#).unwrap();
        for cap in data_attr_regex.captures_iter(content) {
            if let (Some(attr), Some(value)) = (cap.get(1), cap.get(2)) {
                analysis.bindings.push(TemplateBinding {
                    name: format!("{}: {}", attr.as_str(), value.as_str()),
                    binding_type: "data-attribute".to_string(),
                    source: "html".to_string(),
                });
            }
        }

        // Look for ng-* attributes (Angular) or v-* attributes (Vue)
        let framework_attr_regex = Regex::new(r#"(ng-|v-)([\w-]+)=['"](.*?)['"](\s*)"#).unwrap();
        for cap in framework_attr_regex.captures_iter(content) {
            if let (Some(prefix), Some(attr), Some(value)) = (cap.get(1), cap.get(2), cap.get(3)) {
                let framework = if prefix.as_str() == "ng-" { "angular" } else { "vue" };
                analysis.bindings.push(TemplateBinding {
                    name: format!("{}{}: {}", prefix.as_str(), attr.as_str(), value.as_str()),
                    binding_type: "framework-binding".to_string(),
                    source: framework.to_string(),
                });
            }
        }

        // Look for includes or imports
        let include_regex = Regex::new(r#"<include\s+src=['"](.*?)['"](\s*>)"#).unwrap();
        for cap in include_regex.captures_iter(content) {
            if let Some(src) = cap.get(1) {
                analysis.partials.push(TemplatePartial {
                    name: src.as_str().split('/').last().unwrap_or(src.as_str()).to_string(),
                    path: src.as_str().to_string(),
                });
            }
        }
    }
    
    fn analyze_vue(&self, analysis: &mut TemplateAnalysis, content: &str) {
        // Look for Vue component imports
        let component_regex = Regex::new(r#"import\s+(\w+)\s+from\s+['"]([^'"]+)['"]\s*"#).unwrap();
        for cap in component_regex.captures_iter(content) {
            if let (Some(name), Some(path)) = (cap.get(1), cap.get(2)) {
                analysis.partials.push(TemplatePartial {
                    name: name.as_str().to_string(),
                    path: path.as_str().to_string(),
                });
            }
        }
        
        // Look for Vue component registrations
        let components_regex = Regex::new(r#"components:\s*\{([^}]+)\}"#).unwrap();
        if let Some(cap) = components_regex.captures(content) {
            if let Some(components_str) = cap.get(1) {
                let components = components_str.as_str().split(',');
                for component in components {
                    let component = component.trim();
                    if !component.is_empty() {
                        analysis.partials.push(TemplatePartial {
                            name: component.to_string(),
                            path: "local-component".to_string(),
                        });
                    }
                }
            }
        }
        
        // Look for Vue data properties
        let data_regex = Regex::new(r#"data\s*\(\s*\)\s*\{\s*return\s*\{([^}]+)\}"#).unwrap();
        if let Some(cap) = data_regex.captures(content) {
            if let Some(data_str) = cap.get(1) {
                let data_props = data_str.as_str().split(',');
                for prop in data_props {
                    let prop = prop.trim();
                    if !prop.is_empty() {
                        if let Some(name) = prop.split(':').next() {
                            analysis.bindings.push(TemplateBinding {
                                name: name.trim().to_string(),
                                binding_type: "vue-data".to_string(),
                                source: "vue".to_string(),
                            });
                        }
                    }
                }
            }
        }
    }
    
    fn analyze_jsx(&self, analysis: &mut TemplateAnalysis, content: &str) {
        // Look for React component imports
        let import_regex = Regex::new(r#"import\s+(\w+)\s+from\s+['"]([^'"]+)['"]\s*"#).unwrap();
        for cap in import_regex.captures_iter(content) {
            if let (Some(name), Some(path)) = (cap.get(1), cap.get(2)) {
                analysis.partials.push(TemplatePartial {
                    name: name.as_str().to_string(),
                    path: path.as_str().to_string(),
                });
            }
        }
        
        // Look for JSX component usage
        let component_regex = Regex::new(r#"<([A-Z]\w+)([^>]*?)(?:/>|>)"#).unwrap();
        for cap in component_regex.captures_iter(content) {
            if let Some(name) = cap.get(1) {
                analysis.partials.push(TemplatePartial {
                    name: name.as_str().to_string(),
                    path: "jsx-component".to_string(),
                });
            }
        }
        
        // Look for props and state usage
        let props_regex = Regex::new(r#"\{(?:props|this\.props)\.(\w+)\}"#).unwrap();
        for cap in props_regex.captures_iter(content) {
            if let Some(name) = cap.get(1) {
                analysis.bindings.push(TemplateBinding {
                    name: name.as_str().to_string(),
                    binding_type: "react-props".to_string(),
                    source: "react".to_string(),
                });
            }
        }
        
        let state_regex = Regex::new(r#"\{(?:state|this\.state)\.(\w+)\}"#).unwrap();
        for cap in state_regex.captures_iter(content) {
            if let Some(name) = cap.get(1) {
                analysis.bindings.push(TemplateBinding {
                    name: name.as_str().to_string(),
                    binding_type: "react-state".to_string(),
                    source: "react".to_string(),
                });
            }
        }
    }
    
    pub fn generate_report(&self, result: &TemplateAnalysisResult) -> Result<String> {
        // Generate a markdown report
        let mut report = String::new();
        
        // Header
        report.push_str("# Template Analysis Report\n\n");
        report.push_str(&format!("_Generated on: {}_\n\n", chrono::Local::now().format("%Y-%m-%d")));
        
        // Summary
        report.push_str("## Summary\n\n");
        report.push_str(&format!("- **Total Templates**: {}\n", result.templates.len()));
        
        // Count by template type
        let mut template_types = HashMap::new();
        for (_, template) in &result.templates {
            *template_types.entry(template.template_type.clone()).or_insert(0) += 1;
        }
        
        report.push_str("\n### Template Types\n\n");
        report.push_str("| Type | Count |\n");
        report.push_str("|------|-------|\n");
        
        for (template_type, count) in &template_types {
            report.push_str(&format!("| {} | {} |\n", template_type, count));
        }
        
        // Templates with most bindings
        report.push_str("\n## Templates with Most Bindings\n\n");
        report.push_str("| Template | Type | Bindings | Partials | Loops | Conditionals |\n");
        report.push_str("|----------|------|----------|----------|-------|-------------|\n");
        
        let mut templates_by_bindings: Vec<_> = result.templates.values().collect();
        templates_by_bindings.sort_by(|a, b| b.bindings.len().cmp(&a.bindings.len()));
        
        for template in templates_by_bindings.iter().take(20) {
            report.push_str(&format!("| `{}` | {} | {} | {} | {} | {} |\n",
                template.path,
                template.template_type,
                template.bindings.len(),
                template.partials.len(),
                template.loops.len(),
                template.conditionals.len()
            ));
        }
        
        // Most common partials
        report.push_str("\n## Most Common Partials\n\n");
        
        let mut partial_counts = HashMap::new();
        for (_, template) in &result.templates {
            for partial in &template.partials {
                *partial_counts.entry(partial.path.clone()).or_insert(0) += 1;
            }
        }
        
        if !partial_counts.is_empty() {
            report.push_str("| Partial | Usage Count |\n");
            report.push_str("|---------|-------------|\n");
            
            let mut partials_by_count: Vec<_> = partial_counts.iter().collect();
            partials_by_count.sort_by(|a, b| b.1.cmp(a.1));
            
            for (partial, count) in partials_by_count.iter().take(20) {
                report.push_str(&format!("| `{}` | {} |\n", partial, count));
            }
        } else {
            report.push_str("No partials found in the templates.\n");
        }
        
        Ok(report)
    }
    
    pub fn export_to_json(&self, result: &TemplateAnalysisResult) -> Result<String> {
        let json = serde_json::to_string_pretty(result)
            .context("Failed to serialize template analysis result to JSON")?;
        
        Ok(json)
    }
}

impl IncrementalAnalyzer<TemplateAnalysisResult> for IncrementalTemplateAnalyzer {
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
        
        true // Exclude by default if no extension
    }
    
    fn analyze_file(&self, file_path: &Path) -> Result<TemplateAnalysisResult> {
        let mut result = TemplateAnalysisResult::default();
        
        // Get file extension
        let ext = file_path.extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_string();
        
        // Read file content
        let content = fs::read_to_string(file_path)
            .context(format!("Failed to read file {}", file_path.display()))?;
        
        // Get relative path
        let relative_path = file_path.strip_prefix(&self.base_dir)
            .unwrap_or(file_path)
            .to_string_lossy()
            .to_string();
        
        // Analyze template
        let mut template_analysis = self.analyze_template(&content, &ext);
        template_analysis.path = relative_path.clone();
        
        // Add to result
        result.templates.insert(relative_path, template_analysis);
        
        Ok(result)
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
    
    fn setup_test_directory() -> (TempDir, IncrementalTemplateAnalyzer) {
        let dir = tempdir().unwrap();
        let analyzer = IncrementalTemplateAnalyzer::new(dir.path().to_path_buf());
        (dir, analyzer)
    }
    
    #[test]
    fn test_analyze_erb_template() -> Result<()> {
        let (dir, analyzer) = setup_test_directory();
        
        // Create an ERB template
        let erb_content = r#"
        <div class="user-profile">
          <h1><%= @user.name %></h1>
          <p><%= @user.bio %></p>
          
          <% if @user.admin? %>
            <span class="admin-badge">Admin</span>
          <% end %>
          
          <h2>Posts</h2>
          <% @user.posts.each do |post| %>
            <div class="post">
              <h3><%= post.title %></h3>
              <p><%= post.content %></p>
            </div>
          <% end %>
          
          <%= render partial: 'shared/footer' %>
        </div>
        "#;
        
        let file_path = create_test_file(dir.path(), "user_profile.erb", erb_content);
        
        let result = analyzer.analyze_file(&file_path)?;
        
        assert_eq!(result.templates.len(), 1);
        
        let template = result.templates.values().next().unwrap();
        assert_eq!(template.template_type, "erb");
        assert!(template.bindings.len() >= 4); // @user.name, @user.bio, post.title, post.content
        assert_eq!(template.partials.len(), 1); // shared/footer
        assert_eq!(template.loops.len(), 1); // @user.posts.each
        assert_eq!(template.conditionals.len(), 1); // if @user.admin?
        
        Ok(())
    }
    
    #[test]
    fn test_analyze_hbs_template() -> Result<()> {
        let (dir, analyzer) = setup_test_directory();
        
        // Create a Handlebars template
        let hbs_content = r#"
        <div class="user-profile">
          <h1>{{user.name}}</h1>
          <p>{{user.bio}}</p>
          
          {{#if user.isAdmin}}
            <span class="admin-badge">Admin</span>
          {{/if}}
          
          <h2>Posts</h2>
          {{#each user.posts as |post|}}
            <div class="post">
              <h3>{{post.title}}</h3>
              <p>{{post.content}}</p>
            </div>
          {{/each}}
          
          {{> shared/footer}}
        </div>
        "#;
        
        let file_path = create_test_file(dir.path(), "user_profile.hbs", hbs_content);
        
        let result = analyzer.analyze_file(&file_path)?;
        
        assert_eq!(result.templates.len(), 1);
        
        let template = result.templates.values().next().unwrap();
        assert_eq!(template.template_type, "hbs");
        assert!(template.bindings.len() >= 4); // user.name, user.bio, post.title, post.content
        assert_eq!(template.partials.len(), 1); // shared/footer
        assert_eq!(template.loops.len(), 1); // each user.posts
        assert_eq!(template.conditionals.len(), 1); // if user.isAdmin
        
        Ok(())
    }
    
    #[test]
    fn test_incremental_analysis() -> Result<()> {
        let (dir, analyzer) = setup_test_directory();
        
        // Create an ERB template
        let erb_content = r#"
        <div class="user-profile">
          <h1><%= @user.name %></h1>
          <p><%= @user.bio %></p>
        </div>
        "#;
        
        let file_path = create_test_file(dir.path(), "user_profile.erb", erb_content);
        
        // First analysis
        let result1 = analyzer.analyze()?;
        
        // Check that template was analyzed
        assert_eq!(result1.templates.len(), 1);
        
        // Check that the cache file was created
        let cache_path = dir.path().join(".template_analyzer_cache.json");
        assert!(cache_path.exists());
        
        // Create a new analyzer with the same cache path
        let analyzer2 = IncrementalTemplateAnalyzer::new(dir.path().to_path_buf());
        
        // Second analysis - should use the cache
        let result2 = analyzer2.analyze()?;
        
        // Results should be the same
        assert_eq!(result1.templates.len(), result2.templates.len());
        
        // Modify the template
        let new_erb_content = r#"
        <div class="user-profile">
          <h1><%= @user.name %></h1>
          <p><%= @user.bio %></p>
          
          <% if @user.admin? %>
            <span class="admin-badge">Admin</span>
          <% end %>
        </div>
        "#;
        
        let _ = create_test_file(dir.path(), "user_profile.erb", new_erb_content);
        
        // Third analysis - should detect the new conditional
        let result3 = analyzer2.analyze()?;
        
        // Template should be updated
        let template = result3.templates.get("user_profile.erb").unwrap();
        assert_eq!(template.conditionals.len(), 1); // if @user.admin?
        
        Ok(())
    }
}
