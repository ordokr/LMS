use std::path::{Path, PathBuf};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::fs;
use walkdir::WalkDir;
use regex::Regex;
use anyhow::{Result, Context};

use crate::utils::incremental_analyzer::{IncrementalAnalyzer, AnalysisCache};
use crate::analyzers::modules::enhanced_api_analyzer::{ApiEndpoint, ApiClient};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ApiAnalysisResult {
    pub endpoints: HashMap<String, ApiEndpoint>,
    pub clients: Vec<ApiClient>,
    pub route_patterns: HashMap<String, Vec<String>>,
    pub auth_protected_routes: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IncrementalApiAnalyzer {
    pub base_dir: PathBuf,
    pub use_incremental: bool,
    pub cache_path: Option<PathBuf>,
    pub exclude_dirs: Vec<String>,
    pub include_extensions: Vec<String>,
}

impl Default for IncrementalApiAnalyzer {
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
                "rb".to_string(),
                "js".to_string(),
                "ts".to_string(),
                "jsx".to_string(),
                "tsx".to_string(),
            ],
        }
    }
}

impl IncrementalApiAnalyzer {
    pub fn new(base_dir: PathBuf) -> Self {
        let mut analyzer = Self::default();
        analyzer.base_dir = base_dir.clone();
        analyzer.cache_path = Some(base_dir.join(".api_analyzer_cache.json"));
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

    pub fn analyze(&self) -> Result<ApiAnalysisResult> {
        // Collect all files to analyze
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
        let mut combined_result = ApiAnalysisResult::default();

        for result in file_results {
            // Add endpoints
            for (key, endpoint) in result.endpoints {
                combined_result.endpoints.insert(key, endpoint);
            }

            // Add clients
            combined_result.clients.extend(result.clients);
        }

        // Post-process the combined results
        self.post_process_results(&mut combined_result);

        Ok(combined_result)
    }

    fn post_process_results(&self, result: &mut ApiAnalysisResult) {
        // Deduplicate clients
        let mut unique_clients = Vec::new();
        let mut seen = HashMap::new();

        for client in &result.clients {
            let key = format!("{}:{}:{}", client.method, client.endpoint, client.client_type);
            if !seen.contains_key(&key) {
                seen.insert(key, true);
                unique_clients.push(client.clone());
            }
        }

        result.clients = unique_clients;

        // Identify route patterns
        self.identify_route_patterns(result);

        // Identify authentication-protected routes
        self.identify_auth_protected_routes(result);
    }

    fn identify_route_patterns(&self, result: &mut ApiAnalysisResult) {
        let mut patterns: HashMap<String, Vec<String>> = HashMap::new();

        // REST pattern
        let mut rest_pattern = Vec::new();
        for (_, endpoint) in &result.endpoints {
            if let Some(action) = &endpoint.action {
                if action == "index" || action == "show" || action == "create" ||
                   action == "update" || action == "destroy" {
                    rest_pattern.push(endpoint.path.clone());
                }
            }
        }
        if !rest_pattern.is_empty() {
            patterns.insert("REST API".to_string(), rest_pattern);
        }

        // Versioned API pattern
        let mut versioned_pattern = Vec::new();
        for (_, endpoint) in &result.endpoints {
            if endpoint.path.contains("/api/v") || endpoint.path.contains("/api/version") {
                versioned_pattern.push(endpoint.path.clone());
            }
        }
        if !versioned_pattern.is_empty() {
            patterns.insert("Versioned API".to_string(), versioned_pattern);
        }

        // Resource-based pattern
        let mut resource_pattern = Vec::new();
        for (_, endpoint) in &result.endpoints {
            let path_segments: Vec<&str> = endpoint.path.split('/').filter(|s| !s.is_empty()).collect();
            if path_segments.len() >= 3 && path_segments[0] == "api" && !path_segments[2].starts_with(':') {
                resource_pattern.push(endpoint.path.clone());
            }
        }
        if !resource_pattern.is_empty() {
            patterns.insert("Resource-based API".to_string(), resource_pattern);
        }

        // GraphQL pattern
        let mut graphql_pattern = Vec::new();
        for client in &result.clients {
            if client.client_type == "graphql" {
                graphql_pattern.push(client.endpoint.clone());
            }
        }
        if !graphql_pattern.is_empty() {
            patterns.insert("GraphQL API".to_string(), graphql_pattern);
        }

        result.route_patterns = patterns;
    }

    fn identify_auth_protected_routes(&self, result: &mut ApiAnalysisResult) {
        let mut auth_routes = Vec::new();

        // Identify routes that require authentication
        for (_, endpoint) in &result.endpoints {
            if endpoint.authentication_required {
                auth_routes.push(endpoint.path.clone());
            }
        }

        result.auth_protected_routes = auth_routes;
    }

    fn analyze_rails_routes(&self, file_path: &Path) -> Result<ApiAnalysisResult> {
        let mut result = ApiAnalysisResult::default();

        // Only process routes.rb files
        if !file_path.file_name().map_or(false, |n| n == "routes.rb") {
            return Ok(result);
        }

        let content = fs::read_to_string(file_path)
            .context(format!("Failed to read file {}", file_path.display()))?;

        // Check if this file contains API routes
        if content.contains("namespace :api") || content.contains("scope :api") {
            // Extract standard Rails API routes
            let route_regex = Regex::new(r#"(get|post|put|patch|delete)\s+['"]([^'"]+)['"](?:,\s+to:\s*['"]([^#]+)#([^'"]+)['"])?(?:,\s+as:\s*:([^,\s]+))?"#).unwrap();

            for cap in route_regex.captures_iter(&content) {
                let method_str = cap.get(1).map_or("GET", |m| m.as_str()).to_uppercase();
                let path = cap.get(2).map_or("", |m| m.as_str());

                // Only process API routes
                if path.starts_with("/api") || content.contains("namespace :api") {
                    let controller = cap.get(3).map(|m| m.as_str().to_string());
                    let action = cap.get(4).map(|m| m.as_str().to_string());

                    // Extract parameters from path
                    let param_regex = Regex::new(r#":([a-zA-Z_]+)"#).unwrap();
                    let mut parameters = Vec::new();
                    for param_cap in param_regex.captures_iter(path) {
                        if let Some(param_name) = param_cap.get(1) {
                            parameters.push(param_name.as_str().to_string());
                        }
                    }

                    // Determine if authentication is required
                    let auth_required = content.contains("authenticate_user!") ||
                                       content.contains("before_action :authenticate_user") ||
                                       content.contains("before_filter :authenticate_user");

                    // Determine response format
                    let response_format = if content.contains("respond_to do |format|") {
                        if content.contains("format.json") {
                            Some("JSON".to_string())
                        } else if content.contains("format.xml") {
                            Some("XML".to_string())
                        } else {
                            None
                        }
                    } else if path.ends_with(".json") {
                        Some("JSON".to_string())
                    } else if path.ends_with(".xml") {
                        Some("XML".to_string())
                    } else {
                        Some("JSON".to_string()) // Default for API routes
                    };

                    // Extract API description from comments
                    let description_regex = Regex::new(r#"#\s*([^\n]+)\s*\n\s*(get|post|put|patch|delete)\s+['"]([^'"]+)['"]\s*"#).unwrap();
                    let description = description_regex
                        .captures_iter(&content)
                        .filter_map(|desc_cap| {
                            let desc = desc_cap.get(1)?.as_str();
                            let method = desc_cap.get(2)?.as_str().to_uppercase();
                            let route = desc_cap.get(3)?.as_str();

                            if method == method_str && route == path {
                                Some(desc.to_string())
                            } else {
                                None
                            }
                        })
                        .next();

                    // Check for rate limiting
                    let rate_limited = content.contains("throttle") ||
                                     content.contains("rate_limit") ||
                                     content.contains("RateLimit");

                    // Extract required permissions
                    let mut required_permissions = Vec::new();
                    let permission_regex = Regex::new(r#"(authorize|can\?|authorize!|permission|role)\s*[\(:]?\s*['"]([\w_]+)['"]\s*"#).unwrap();

                    for perm_cap in permission_regex.captures_iter(&content) {
                        if let Some(permission) = perm_cap.get(2) {
                            required_permissions.push(permission.as_str().to_string());
                        }
                    }

                    let path_str = path.to_string();
                    let endpoint = ApiEndpoint {
                        path: path_str.clone(),
                        method: method_str.clone(),
                        controller,
                        action,
                        authentication_required: auth_required,
                        parameters,
                        response_format,
                        source_file: file_path.to_string_lossy().to_string(),
                        description,
                        rate_limited,
                        required_permissions,
                        request_body_params: Vec::new(),
                        response_fields: Vec::new(),
                    };

                    let key = format!("{}:{}", method_str, path);
                    result.endpoints.insert(key, endpoint);
                }
            }
        }

        Ok(result)
    }

    fn analyze_js_clients(&self, file_path: &Path) -> Result<ApiAnalysisResult> {
        let mut result = ApiAnalysisResult::default();

        // Only process JavaScript/TypeScript files
        if let Some(extension) = file_path.extension() {
            if let Some(ext) = extension.to_str() {
                if ext != "js" && ext != "ts" && ext != "jsx" && ext != "tsx" {
                    return Ok(result);
                }
            } else {
                return Ok(result);
            }
        } else {
            return Ok(result);
        }

        let content = fs::read_to_string(file_path)
            .context(format!("Failed to read file {}", file_path.display()))?;

        // Look for fetch API calls
        let fetch_regex = Regex::new(r#"fetch\(['"]([^'"]+)['"](?:,\s*\{\s*method:\s*['"]([^'"]+)['"])?\)"#).unwrap();

        for cap in fetch_regex.captures_iter(&content) {
            if let Some(endpoint) = cap.get(1) {
                let method = cap.get(2).map_or("GET", |m| m.as_str()).to_uppercase();

                result.clients.push(ApiClient {
                    endpoint: endpoint.as_str().to_string(),
                    method,
                    client_type: "fetch".to_string(),
                    source_file: file_path.to_string_lossy().to_string(),
                });
            }
        }

        // Look for axios API calls
        let axios_regex = Regex::new(r#"axios\.(get|post|put|patch|delete)\(['"]([^'"]+)['"]"#).unwrap();

        for cap in axios_regex.captures_iter(&content) {
            if let (Some(method), Some(endpoint)) = (cap.get(1), cap.get(2)) {
                result.clients.push(ApiClient {
                    endpoint: endpoint.as_str().to_string(),
                    method: method.as_str().to_uppercase(),
                    client_type: "axios".to_string(),
                    source_file: file_path.to_string_lossy().to_string(),
                });
            }
        }

        // Look for jQuery AJAX calls
        let jquery_regex = Regex::new(r#"\$\.ajax\(\{\s*url:\s*['"]([^'"]+)['"](?:,\s*type:\s*['"]([^'"]+)['"])?"#).unwrap();

        for cap in jquery_regex.captures_iter(&content) {
            if let Some(endpoint) = cap.get(1) {
                let method = cap.get(2).map_or("GET", |m| m.as_str()).to_uppercase();

                result.clients.push(ApiClient {
                    endpoint: endpoint.as_str().to_string(),
                    method,
                    client_type: "jquery".to_string(),
                    source_file: file_path.to_string_lossy().to_string(),
                });
            }
        }

        // Look for GraphQL queries and mutations
        let graphql_query_regex = Regex::new(r#"(query|mutation)\s+(\w+)\s*\{([^}]+)\}"#).unwrap();

        for cap in graphql_query_regex.captures_iter(&content) {
            if let (Some(operation_type), Some(operation_name)) = (cap.get(1), cap.get(2)) {
                let method = if operation_type.as_str() == "query" { "GET" } else { "POST" };
                let endpoint = format!("/graphql/{}", operation_name.as_str());

                result.clients.push(ApiClient {
                    endpoint,
                    method: method.to_string(),
                    client_type: "graphql".to_string(),
                    source_file: file_path.to_string_lossy().to_string(),
                });
            }
        }

        Ok(result)
    }

    pub fn generate_report(&self, result: &ApiAnalysisResult) -> Result<String> {
        // Generate a markdown report
        let mut report = String::new();

        // Header
        report.push_str("# API Analysis Report\n\n");
        report.push_str(&format!("_Generated on: {}_\n\n", chrono::Local::now().format("%Y-%m-%d")));

        // Summary
        report.push_str("## Summary\n\n");
        report.push_str(&format!("- **Total API Endpoints**: {}\n", result.endpoints.len()));
        report.push_str(&format!("- **Total API Clients**: {}\n", result.clients.len()));

        // Count HTTP methods
        let mut method_counts = HashMap::new();
        for (_, endpoint) in &result.endpoints {
            *method_counts.entry(endpoint.method.clone()).or_insert(0) += 1;
        }

        report.push_str("\n### HTTP Methods\n\n");
        report.push_str("| Method | Count |\n");
        report.push_str("|--------|-------|\n");

        for (method, count) in &method_counts {
            report.push_str(&format!("| {} | {} |\n", method, count));
        }

        // API Endpoints
        report.push_str("\n## API Endpoints\n\n");
        report.push_str("| Method | Path | Controller | Action | Auth Required | Parameters |\n");
        report.push_str("|--------|------|------------|--------|---------------|------------|\n");

        let mut sorted_endpoints: Vec<_> = result.endpoints.values().collect();
        sorted_endpoints.sort_by(|a, b| a.path.cmp(&b.path));

        for endpoint in sorted_endpoints {
            report.push_str(&format!("| {} | {} | {} | {} | {} | {} |\n",
                endpoint.method,
                endpoint.path,
                endpoint.controller.as_deref().unwrap_or("N/A"),
                endpoint.action.as_deref().unwrap_or("N/A"),
                if endpoint.authentication_required { "Yes" } else { "No" },
                endpoint.parameters.join(", ")
            ));
        }

        // API Clients
        report.push_str("\n## API Clients\n\n");
        report.push_str("| Method | Endpoint | Client Type | Source File |\n");
        report.push_str("|--------|----------|-------------|-------------|\n");

        let mut sorted_clients = result.clients.clone();
        sorted_clients.sort_by(|a, b| a.endpoint.cmp(&b.endpoint));

        for client in &sorted_clients {
            report.push_str(&format!("| {} | {} | {} | {} |\n",
                client.method,
                client.endpoint,
                client.client_type,
                client.source_file
            ));
        }

        // Route Patterns
        report.push_str("\n## API Patterns\n\n");

        for (pattern_name, routes) in &result.route_patterns {
            report.push_str(&format!("### {}\n\n", pattern_name));
            report.push_str("| Route |\n");
            report.push_str("|-------|\n");

            let mut sorted_routes = routes.clone();
            sorted_routes.sort();

            for route in sorted_routes {
                report.push_str(&format!("| {} |\n", route));
            }

            report.push_str("\n");
        }

        // Authentication Protected Routes
        report.push_str("## Authentication Protected Routes\n\n");

        if result.auth_protected_routes.is_empty() {
            report.push_str("No authentication protected routes found.\n\n");
        } else {
            report.push_str("| Route |\n");
            report.push_str("|-------|\n");

            let mut sorted_auth_routes = result.auth_protected_routes.clone();
            sorted_auth_routes.sort();

            for route in sorted_auth_routes {
                report.push_str(&format!("| {} |\n", route));
            }
        }

        Ok(report)
    }

    pub fn export_to_json(&self, result: &ApiAnalysisResult) -> Result<String> {
        let json = serde_json::to_string_pretty(result)
            .context("Failed to serialize API analysis result to JSON")?;

        Ok(json)
    }
}

impl IncrementalAnalyzer<ApiAnalysisResult> for IncrementalApiAnalyzer {
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

    fn analyze_file(&self, file_path: &Path) -> Result<ApiAnalysisResult> {
        // Check file extension
        if let Some(ext) = file_path.extension() {
            if let Some(ext_str) = ext.to_str() {
                if ext_str == "rb" {
                    return self.analyze_rails_routes(file_path);
                } else if ext_str == "js" || ext_str == "ts" || ext_str == "jsx" || ext_str == "tsx" {
                    return self.analyze_js_clients(file_path);
                }
            }
        }

        // Default empty result
        Ok(ApiAnalysisResult::default())
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

    fn setup_test_directory() -> (TempDir, IncrementalApiAnalyzer) {
        let dir = tempdir().unwrap();
        let analyzer = IncrementalApiAnalyzer::new(dir.path().to_path_buf());
        (dir, analyzer)
    }

    #[test]
    fn test_analyze_rails_routes() -> Result<()> {
        let (dir, analyzer) = setup_test_directory();

        // Create a routes.rb file
        let routes_content = r#"
        Rails.application.routes.draw do
          namespace :api do
            namespace :v1 do
              resources :users, only: [:index, :show, :create, :update, :destroy]
              get '/profile', to: 'users#profile'
              post '/login', to: 'sessions#create'
            end
          end
        end
        "#;

        let file_path = create_test_file(dir.path(), "routes.rb", routes_content);

        let result = analyzer.analyze_file(&file_path)?;

        assert!(result.endpoints.len() > 0);
        assert!(result.endpoints.values().any(|e| e.path.contains("/profile")));
        assert!(result.endpoints.values().any(|e| e.path.contains("/login")));

        Ok(())
    }

    #[test]
    fn test_analyze_js_clients() -> Result<()> {
        let (dir, analyzer) = setup_test_directory();

        // Create a JavaScript file with API clients
        let js_content = r#"
        // Fetch API
        fetch('/api/v1/users', { method: 'GET' })
          .then(response => response.json())
          .then(data => console.log(data));

        // Axios
        axios.get('/api/v1/profile')
          .then(response => console.log(response.data));

        // jQuery
        $.ajax({
          url: '/api/v1/login',
          type: 'POST',
          data: { username, password }
        });
        "#;

        let file_path = create_test_file(dir.path(), "api_client.js", js_content);

        let result = analyzer.analyze_file(&file_path)?;

        assert_eq!(result.clients.len(), 3);
        assert!(result.clients.iter().any(|c| c.endpoint == "/api/v1/users" && c.method == "GET"));
        assert!(result.clients.iter().any(|c| c.endpoint == "/api/v1/profile" && c.method == "GET"));
        assert!(result.clients.iter().any(|c| c.endpoint == "/api/v1/login" && c.method == "POST"));

        Ok(())
    }

    #[test]
    fn test_incremental_analysis() -> Result<()> {
        let (dir, analyzer) = setup_test_directory();

        // Create a routes.rb file
        let routes_content = r#"
        Rails.application.routes.draw do
          namespace :api do
            namespace :v1 do
              resources :users, only: [:index, :show]
            end
          end
        end
        "#;

        let routes_path = create_test_file(dir.path(), "routes.rb", routes_content);

        // Create a JavaScript file
        let js_content = r#"
        fetch('/api/v1/users', { method: 'GET' })
          .then(response => response.json());
        "#;

        let js_path = create_test_file(dir.path(), "api_client.js", js_content);

        // First analysis
        let result1 = analyzer.analyze()?;

        // Check that endpoints and clients were detected
        assert!(result1.endpoints.len() > 0);
        assert_eq!(result1.clients.len(), 1);

        // Check that the cache file was created
        let cache_path = dir.path().join(".api_analyzer_cache.json");
        assert!(cache_path.exists());

        // Create a new analyzer with the same cache path
        let analyzer2 = IncrementalApiAnalyzer::new(dir.path().to_path_buf());

        // Second analysis - should use the cache
        let result2 = analyzer2.analyze()?;

        // Results should be the same
        assert_eq!(result1.endpoints.len(), result2.endpoints.len());
        assert_eq!(result1.clients.len(), result2.clients.len());

        // Modify the JavaScript file
        let new_js_content = r#"
        fetch('/api/v1/users', { method: 'GET' })
          .then(response => response.json());

        axios.get('/api/v1/profile')
          .then(response => console.log(response.data));
        "#;

        let _ = create_test_file(dir.path(), "api_client.js", new_js_content);

        // Third analysis - should detect the new client
        let result3 = analyzer2.analyze()?;

        // Should have one more client
        assert_eq!(result3.clients.len(), 2);
        assert!(result3.clients.iter().any(|c| c.endpoint == "/api/v1/profile"));

        Ok(())
    }
}
