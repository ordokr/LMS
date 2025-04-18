use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::fs;
use walkdir::WalkDir;
use regex::Regex;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ApiEndpoint {
    pub path: String,
    pub method: String,
    pub controller: Option<String>,
    pub action: Option<String>,
    pub authentication_required: bool,
    pub parameters: Vec<String>,
    pub response_format: Option<String>,
    pub source_file: String,
    pub description: Option<String>,
    pub rate_limited: bool,
    pub required_permissions: Vec<String>,
    pub request_body_params: Vec<String>,
    pub response_fields: Vec<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ApiClient {
    pub endpoint: String,
    pub method: String,
    pub client_type: String,
    pub source_file: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ApiAnalyzer {
    pub endpoints: HashMap<String, ApiEndpoint>,
    pub clients: Vec<ApiClient>,
    pub route_patterns: HashMap<String, Vec<String>>,
    pub auth_protected_routes: Vec<String>,
}

impl ApiAnalyzer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn analyze(&self, base_dir: &PathBuf) -> Result<String, String> {
        let mut analyzer = ApiAnalyzer::default();

        // Analyze Rails routes for API endpoints
        analyzer.analyze_rails_routes(base_dir);

        // Analyze JavaScript/TypeScript files for API clients
        analyzer.analyze_js_clients(base_dir);

        // Identify route patterns
        analyzer.identify_route_patterns();

        // Identify authentication-protected routes
        analyzer.identify_auth_protected_routes();

        match serde_json::to_string_pretty(&analyzer) {
            Ok(json) => Ok(json),
            Err(e) => Err(format!("Failed to serialize ApiAnalyzer: {}", e)),
        }
    }

    fn analyze_rails_routes(&mut self, base_dir: &PathBuf) {
        // Look for routes.rb files
        for entry in WalkDir::new(base_dir)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file() && path.file_name().map_or(false, |n| n == "routes.rb") {
                if let Ok(content) = fs::read_to_string(path) {
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

                                // Look for controller file to extract more information
                                let mut request_body_params = Vec::new();
                                let mut response_fields = Vec::new();

                                if let Some(controller_name) = &controller {
                                    let controller_path = self.find_controller_file(base_dir, controller_name);

                                    if let Some(controller_path) = controller_path {
                                        if let Ok(controller_content) = fs::read_to_string(&controller_path) {
                                            // Extract request body parameters
                                            let params_regex = Regex::new(r#"params\.(?:require|permit)\(:\w+\)\.permit\(([^\)]+)\)"#).unwrap();

                                            for params_cap in params_regex.captures_iter(&controller_content) {
                                                if let Some(params_str) = params_cap.get(1) {
                                                    let params = params_str.as_str();
                                                    let param_names_regex = Regex::new(r#":([a-zA-Z0-9_]+)"#).unwrap();

                                                    for param_name_cap in param_names_regex.captures_iter(params) {
                                                        if let Some(param_name) = param_name_cap.get(1) {
                                                            request_body_params.push(param_name.as_str().to_string());
                                                        }
                                                    }
                                                }
                                            }

                                            // Extract response fields
                                            let render_regex = Regex::new(r#"render\s+(?:json|xml):\s*\{([^\}]+)\}"#).unwrap();

                                            for render_cap in render_regex.captures_iter(&controller_content) {
                                                if let Some(fields_str) = render_cap.get(1) {
                                                    let fields = fields_str.as_str();
                                                    let field_names_regex = Regex::new(r#"(\w+)\s*[=:]"#).unwrap();

                                                    for field_name_cap in field_names_regex.captures_iter(fields) {
                                                        if let Some(field_name) = field_name_cap.get(1) {
                                                            response_fields.push(field_name.as_str().to_string());
                                                        }
                                                    }
                                                }
                                            }
                                        }
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
                                    source_file: path.to_string(),
                                    description,
                                    rate_limited,
                                    required_permissions,
                                    request_body_params,
                                    response_fields,
                                };

                                let key = format!("{}:{}", method_str, path);
                                self.endpoints.insert(key, endpoint);
                            }
                        }
                    }
                }
            }
        }
    }

    fn analyze_js_clients(&mut self, base_dir: &PathBuf) {
        // Look for JavaScript/TypeScript files that might contain API clients
        for entry in WalkDir::new(base_dir)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file() {
                if let Some(extension) = path.extension() {
                    if let Some(ext) = extension.to_str() {
                        if ext == "js" || ext == "ts" || ext == "jsx" || ext == "tsx" {
                            if let Ok(content) = fs::read_to_string(path) {
                                // Look for fetch API calls
                                let fetch_regex = Regex::new(r#"fetch\(['"]([^'"]+)['"](?:,\s*\{\s*method:\s*['"]([^'"]+)['"])?\)"#).unwrap();

                                for cap in fetch_regex.captures_iter(&content) {
                                    if let Some(endpoint) = cap.get(1) {
                                        let method = cap.get(2).map_or("GET", |m| m.as_str()).to_uppercase();

                                        self.clients.push(ApiClient {
                                            endpoint: endpoint.as_str().to_string(),
                                            method,
                                            client_type: "fetch".to_string(),
                                            source_file: path.to_string_lossy().to_string(),
                                        });
                                    }
                                }

                                // Look for axios API calls
                                let axios_regex = Regex::new(r#"axios\.(get|post|put|patch|delete)\(['"]([^'"]+)['"]"#).unwrap();

                                for cap in axios_regex.captures_iter(&content) {
                                    if let (Some(method), Some(endpoint)) = (cap.get(1), cap.get(2)) {
                                        self.clients.push(ApiClient {
                                            endpoint: endpoint.as_str().to_string(),
                                            method: method.as_str().to_uppercase(),
                                            client_type: "axios".to_string(),
                                            source_file: path.to_string_lossy().to_string(),
                                        });
                                    }
                                }

                                // Look for jQuery AJAX calls
                                let jquery_regex = Regex::new(r#"\$\.ajax\(\{\s*url:\s*['"]([^'"]+)['"](?:,\s*type:\s*['"]([^'"]+)['"])?"#).unwrap();

                                for cap in jquery_regex.captures_iter(&content) {
                                    if let Some(endpoint) = cap.get(1) {
                                        let method = cap.get(2).map_or("GET", |m| m.as_str()).to_uppercase();

                                        self.clients.push(ApiClient {
                                            endpoint: endpoint.as_str().to_string(),
                                            method,
                                            client_type: "jquery".to_string(),
                                            source_file: path.to_string_lossy().to_string(),
                                        });
                                    }
                                }

                                // Look for GraphQL queries and mutations
                                let graphql_query_regex = Regex::new(r#"(query|mutation)\s+(\w+)\s*\{([^}]+)\}"#).unwrap();

                                for cap in graphql_query_regex.captures_iter(&content) {
                                    if let (Some(operation_type), Some(operation_name)) = (cap.get(1), cap.get(2)) {
                                        let method = if operation_type.as_str() == "query" { "GET" } else { "POST" };
                                        let endpoint = format!("/graphql/{}", operation_name.as_str());

                                        self.clients.push(ApiClient {
                                            endpoint,
                                            method: method.to_string(),
                                            client_type: "graphql".to_string(),
                                            source_file: path.to_string_lossy().to_string(),
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn identify_route_patterns(&mut self) {
        let mut patterns: HashMap<String, Vec<String>> = HashMap::new();

        // REST pattern
        let mut rest_pattern = Vec::new();
        for (_, endpoint) in &self.endpoints {
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
        for (_, endpoint) in &self.endpoints {
            if endpoint.path.contains("/api/v") || endpoint.path.contains("/api/version") {
                versioned_pattern.push(endpoint.path.clone());
            }
        }
        if !versioned_pattern.is_empty() {
            patterns.insert("Versioned API".to_string(), versioned_pattern);
        }

        // Resource-based pattern
        let mut resource_pattern = Vec::new();
        for (_, endpoint) in &self.endpoints {
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
        for client in &self.clients {
            if client.client_type == "graphql" {
                graphql_pattern.push(client.endpoint.clone());
            }
        }
        if !graphql_pattern.is_empty() {
            patterns.insert("GraphQL API".to_string(), graphql_pattern);
        }

        // JSON:API pattern
        let mut jsonapi_pattern = Vec::new();
        for (_, endpoint) in &self.endpoints {
            if !endpoint.response_fields.is_empty() &&
               (endpoint.response_fields.contains(&"data".to_string()) ||
                endpoint.response_fields.contains(&"attributes".to_string()) ||
                endpoint.response_fields.contains(&"relationships".to_string())) {
                jsonapi_pattern.push(endpoint.path.clone());
            }
        }
        if !jsonapi_pattern.is_empty() {
            patterns.insert("JSON:API".to_string(), jsonapi_pattern);
        }

        self.route_patterns = patterns;
    }

    fn identify_auth_protected_routes(&mut self) {
        let mut auth_routes = Vec::new();

        // Identify routes that require authentication
        for (_, endpoint) in &self.endpoints {
            if endpoint.authentication_required {
                auth_routes.push(endpoint.path.clone());
            }
        }

        self.auth_protected_routes = auth_routes;
    }

    fn find_controller_file(&self, base_dir: &PathBuf, controller_name: &str) -> Option<PathBuf> {
        // Convert controller name to file path (e.g., "api/v1/users" -> "app/controllers/api/v1/users_controller.rb")
        let controller_path = format!("{}_controller.rb", controller_name.replace("/", "\\"));
        let full_path = base_dir.join("app").join("controllers").join(controller_path);

        if full_path.exists() {
            return Some(full_path);
        }

        // Try alternative path formats
        let alt_controller_path = format!("{}_controller.rb", controller_name);
        let alt_full_path = base_dir.join("app").join("controllers").join(alt_controller_path);

        if alt_full_path.exists() {
            return Some(alt_full_path);
        }

        None
    }
}
