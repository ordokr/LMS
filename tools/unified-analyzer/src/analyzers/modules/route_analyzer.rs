use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::fs;
use walkdir::WalkDir;
use regex::Regex;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct RouteParameter {
    pub name: String,
    pub constraint: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Route {
    pub path: String,
    pub http_method: Option<String>,
    pub controller: Option<String>,
    pub action: Option<String>,
    pub name: Option<String>,
    pub parameters: Vec<RouteParameter>,
    pub authentication_required: bool,
    pub source_file: String,
    pub framework: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct RouteAnalyzer {
    pub routes: Vec<Route>,
    pub route_patterns: HashMap<String, Vec<String>>,
    pub auth_protected_routes: Vec<String>,
}

impl RouteAnalyzer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn analyze(&self, base_dir: &PathBuf) -> Result<String, String> {
        let mut analyzer = RouteAnalyzer::default();

        // Analyze Rails routes
        analyzer.analyze_rails_routes(base_dir);

        // Analyze React Router routes
        analyzer.analyze_react_routes(base_dir);

        // Analyze Ember routes
        analyzer.analyze_ember_routes(base_dir);

        // Analyze Express routes
        analyzer.analyze_express_routes(base_dir);

        // Identify route patterns
        analyzer.identify_route_patterns();

        // Identify authentication-protected routes
        analyzer.identify_auth_protected_routes();

        match serde_json::to_string_pretty(&analyzer) {
            Ok(json) => Ok(json),
            Err(e) => Err(format!("Failed to serialize RouteAnalyzer: {}", e)),
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
                    // Extract standard Rails routes
                    let standard_route_regex = Regex::new(r#"(get|post|put|patch|delete)\s+['"]([^'"]+)['"](?:,\s+to:\s*['"]([^#]+)#([^'"]+)['"])?(?:,\s+as:\s*:([^,\s]+))?(?:,\s+constraints:\s*\{([^}]+)\})?"#).unwrap();

                    for cap in standard_route_regex.captures_iter(&content) {
                        let mut route = Route {
                            http_method: cap.get(1).map(|m| m.as_str().to_uppercase()),
                            path: cap.get(2).map_or("".to_string(), |m| m.as_str().to_string()),
                            controller: cap.get(3).map(|m| m.as_str().to_string()),
                            action: cap.get(4).map(|m| m.as_str().to_string()),
                            name: cap.get(5).map(|m| m.as_str().to_string()),
                            source_file: path.to_string_lossy().to_string(),
                            framework: "rails".to_string(),
                            ..Default::default()
                        };

                        // Extract route parameters
                        let param_regex = Regex::new(r#":([a-zA-Z_]+)"#).unwrap();
                        for param_cap in param_regex.captures_iter(&route.path) {
                            if let Some(param_name) = param_cap.get(1) {
                                route.parameters.push(RouteParameter {
                                    name: param_name.as_str().to_string(),
                                    constraint: None,
                                });
                            }
                        }

                        // Check for authentication requirements
                        if content.contains("authenticate_user!") ||
                           content.contains("before_action :authenticate_user") ||
                           content.contains("before_filter :authenticate_user") {
                            route.authentication_required = true;
                        }

                        self.routes.push(route);
                    }

                    // Extract resourceful routes
                    let resources_regex = Regex::new(r#"resources\s+:([a-zA-Z_]+)"#).unwrap();
                    for cap in resources_regex.captures_iter(&content) {
                        if let Some(resource) = cap.get(1) {
                            let resource_name = resource.as_str();
                            let controller = resource_name;

                            // Add standard RESTful routes
                            let restful_actions = vec![
                                ("GET", format!("/{}", resource_name), "index"),
                                ("GET", format!("/{}/new", resource_name), "new"),
                                ("POST", format!("/{}", resource_name), "create"),
                                ("GET", format!("/{}/:id", resource_name), "show"),
                                ("GET", format!("/{}/:id/edit", resource_name), "edit"),
                                ("PUT", format!("/{}/:id", resource_name), "update"),
                                ("PATCH", format!("/{}/:id", resource_name), "update"),
                                ("DELETE", format!("/{}/:id", resource_name), "destroy"),
                            ];

                            for (method, path, action) in restful_actions {
                                let path_str = path.to_string();
                                let mut route = Route {
                                    http_method: Some(method.to_string()),
                                    path,
                                    controller: Some(controller.to_string()),
                                    action: Some(action.to_string()),
                                    source_file: path_str.to_string(),
                                    framework: "rails".to_string(),
                                    ..Default::default()
                                };

                                // Add id parameter for routes that have it
                                if route.path.contains(":id") {
                                    route.parameters.push(RouteParameter {
                                        name: "id".to_string(),
                                        constraint: None,
                                    });
                                }

                                self.routes.push(route);
                            }
                        }
                    }
                }
            }
        }
    }

    fn analyze_ember_routes(&mut self, base_dir: &PathBuf) {
        // Look for Ember route files
        let discourse_dir = base_dir.join("discourse");
        if discourse_dir.exists() {
            for entry in WalkDir::new(&discourse_dir)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                let path = entry.path();
                if path.is_file() && path.file_name().map_or(false, |n| n.to_string_lossy().ends_with("route.js")) {
                    if let Ok(content) = fs::read_to_string(path) {
                        // Extract Ember routes
                        let route_regex = Regex::new(r#"this\.route\(['"]([^'"]+)['"](?:,\s*\{([^}]+)\})?"#).unwrap();

                        for cap in route_regex.captures_iter(&content) {
                            if let Some(route_name) = cap.get(1) {
                                let mut route = Route {
                                    path: format!("/{}", route_name.as_str()),
                                    http_method: Some("GET".to_string()),
                                    name: Some(route_name.as_str().to_string()),
                                    source_file: path.to_string_lossy().to_string(),
                                    framework: "ember".to_string(),
                                    ..Default::default()
                                };

                                // Check for path override
                                if let Some(options) = cap.get(2) {
                                    let options_str = options.as_str();
                                    let path_regex = Regex::new(r#"path:\s*['"]([^'"]+)['"]?"#).unwrap();

                                    if let Some(path_cap) = path_regex.captures(options_str) {
                                        if let Some(path) = path_cap.get(1) {
                                            route.path = path.as_str().to_string();
                                        }
                                    }
                                }

                                // Extract route parameters
                                let param_regex = Regex::new(r#":([a-zA-Z_]+)"#).unwrap();
                                for param_cap in param_regex.captures_iter(&route.path) {
                                    if let Some(param_name) = param_cap.get(1) {
                                        route.parameters.push(RouteParameter {
                                            name: param_name.as_str().to_string(),
                                            constraint: None,
                                        });
                                    }
                                }

                                self.routes.push(route);
                            }
                        }
                    }
                }
            }
        }
    }

    fn analyze_react_routes(&mut self, base_dir: &PathBuf) {
        // Look for React Router route definitions
        for entry in WalkDir::new(base_dir)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file() {
                if let Some(extension) = path.extension() {
                    if let Some(ext) = extension.to_str() {
                        if ext == "jsx" || ext == "tsx" || ext == "js" || ext == "ts" {
                            if let Ok(content) = fs::read_to_string(path) {
                                // Extract React Router routes
                                let route_regex = Regex::new(r#"<Route\s+(?:path|exact\s+path)=['"]([^'"]+)['"](?:\s+component=\{([^}]+)\}|\s+render=\{[^}]+\}|\s+element=\{<([^>]+)[^}]*\})"#).unwrap();

                                for cap in route_regex.captures_iter(&content) {
                                    if let Some(path_match) = cap.get(1) {
                                        let component = cap.get(2).or_else(|| cap.get(3)).map(|m| m.as_str().to_string());

                                        let mut route = Route {
                                            path: path_match.as_str().to_string(),
                                            http_method: Some("GET".to_string()),
                                            controller: component.clone(),
                                            action: component,
                                            source_file: path.to_string_lossy().to_string(),
                                            framework: "react-router".to_string(),
                                            ..Default::default()
                                        };

                                        // Extract route parameters
                                        let param_regex = Regex::new(r#":([a-zA-Z_]+)"#).unwrap();
                                        for param_cap in param_regex.captures_iter(&route.path) {
                                            if let Some(param_name) = param_cap.get(1) {
                                                route.parameters.push(RouteParameter {
                                                    name: param_name.as_str().to_string(),
                                                    constraint: None,
                                                });
                                            }
                                        }

                                        self.routes.push(route);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn analyze_express_routes(&mut self, base_dir: &PathBuf) {
        // Look for Express.js route definitions
        for entry in WalkDir::new(base_dir)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file() {
                if let Some(extension) = path.extension() {
                    if let Some(ext) = extension.to_str() {
                        if ext == "js" {
                            if let Ok(content) = fs::read_to_string(path) {
                                // Check if this is an Express file
                                if content.contains("express") && (content.contains("app.") || content.contains("router.")) {
                                    // Extract Express routes
                                    let route_regex = Regex::new(r#"\.(get|post|put|patch|delete)\(['"]([^'"]+)['"](?:,\s*([^,)]+))?\)"#).unwrap();

                                    for cap in route_regex.captures_iter(&content) {
                                        if let (Some(method), Some(path_match)) = (cap.get(1), cap.get(2)) {
                                            let handler = cap.get(3).map(|m| m.as_str().to_string());

                                            let mut route = Route {
                                                path: path_match.as_str().to_string(),
                                                http_method: Some(method.as_str().to_uppercase()),
                                                action: handler,
                                                source_file: path.to_string_lossy().to_string(),
                                                framework: "express".to_string(),
                                                ..Default::default()
                                            };

                                            // Extract route parameters
                                            let param_regex = Regex::new(r#":([a-zA-Z_]+)"#).unwrap();
                                            for param_cap in param_regex.captures_iter(&route.path) {
                                                if let Some(param_name) = param_cap.get(1) {
                                                    route.parameters.push(RouteParameter {
                                                        name: param_name.as_str().to_string(),
                                                        constraint: None,
                                                    });
                                                }
                                            }

                                            self.routes.push(route);
                                        }
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
        for route in &self.routes {
            if let Some(action) = &route.action {
                if action == "index" || action == "show" || action == "create" ||
                   action == "update" || action == "destroy" || action == "new" ||
                   action == "edit" {
                    rest_pattern.push(route.path.clone());
                }
            }
        }
        if !rest_pattern.is_empty() {
            patterns.insert("REST".to_string(), rest_pattern);
        }

        // Nested resources pattern
        let mut nested_pattern = Vec::new();
        for route in &self.routes {
            let path_segments: Vec<&str> = route.path.split('/').filter(|s| !s.is_empty()).collect();
            if path_segments.len() > 2 && !path_segments.iter().any(|s| s.starts_with(':')) {
                nested_pattern.push(route.path.clone());
            }
        }
        if !nested_pattern.is_empty() {
            patterns.insert("Nested Resources".to_string(), nested_pattern);
        }

        // API versioning pattern
        let mut api_version_pattern = Vec::new();
        for route in &self.routes {
            if route.path.contains("/api/v") || route.path.contains("/api/version") {
                api_version_pattern.push(route.path.clone());
            }
        }
        if !api_version_pattern.is_empty() {
            patterns.insert("API Versioning".to_string(), api_version_pattern);
        }

        self.route_patterns = patterns;
    }

    fn identify_auth_protected_routes(&mut self) {
        let mut auth_routes = Vec::new();

        // Identify routes that require authentication
        for route in &self.routes {
            if route.authentication_required {
                auth_routes.push(route.path.clone());
                continue;
            }

            // Check for common authentication patterns in React
            if route.framework == "react-router" {
                let source_file = &route.source_file;
                if let Ok(content) = fs::read_to_string(source_file) {
                    // Look for PrivateRoute or RequireAuth components
                    let protected_route_regex = Regex::new(r#"<(?:PrivateRoute|RequireAuth)[^>]*path=['"]([^'"]+)['"][^>]*>"#).unwrap();

                    for cap in protected_route_regex.captures_iter(&content) {
                        if let Some(path) = cap.get(1) {
                            if path.as_str() == route.path {
                                auth_routes.push(route.path.clone());
                                break;
                            }
                        }
                    }
                }
            }

            // Check for common authentication patterns in Express
            if route.framework == "express" {
                let source_file = &route.source_file;
                if let Ok(content) = fs::read_to_string(source_file) {
                    if content.contains("isAuthenticated") ||
                       content.contains("requireAuth") ||
                       content.contains("checkAuth") ||
                       content.contains("passport.authenticate") {
                        auth_routes.push(route.path.clone());
                    }
                }
            }
        }

        self.auth_protected_routes = auth_routes;
    }
}
