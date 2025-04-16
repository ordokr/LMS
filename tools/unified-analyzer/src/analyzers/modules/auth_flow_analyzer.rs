use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use std::collections::{HashMap, HashSet};
use std::fs;
use walkdir::WalkDir;
use regex::Regex;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AuthMethod {
    pub name: String,
    pub method_type: String,
    pub source_file: String,
    pub description: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AuthRole {
    pub name: String,
    pub permissions: Vec<String>,
    pub source_file: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AuthFlow {
    pub name: String,
    pub steps: Vec<String>,
    pub source_file: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AuthFlowAnalyzer {
    pub auth_methods: HashMap<String, AuthMethod>,
    pub auth_roles: HashMap<String, AuthRole>,
    pub auth_flows: Vec<AuthFlow>,
    pub protected_routes: HashSet<String>,
}

impl AuthFlowAnalyzer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn analyze(&self, base_dir: &PathBuf) -> Result<String, String> {
        let mut analyzer = AuthFlowAnalyzer::default();

        // Analyze authentication methods
        analyzer.analyze_auth_methods(base_dir);

        // Analyze authorization roles
        analyzer.analyze_auth_roles(base_dir);

        // Analyze authentication flows
        analyzer.analyze_auth_flows(base_dir);

        // Identify protected routes
        analyzer.identify_protected_routes(base_dir);

        match serde_json::to_string_pretty(&analyzer) {
            Ok(json) => Ok(json),
            Err(e) => Err(format!("Failed to serialize AuthFlowAnalyzer: {}", e)),
        }
    }

    fn analyze_auth_methods(&mut self, base_dir: &PathBuf) {
        // Look for common authentication files and patterns

        // Check for Devise (Rails)
        let devise_initializer = base_dir.join("config").join("initializers").join("devise.rb");
        if devise_initializer.exists() {
            self.auth_methods.insert("devise".to_string(), AuthMethod {
                name: "Devise".to_string(),
                method_type: "gem".to_string(),
                source_file: devise_initializer.to_string_lossy().to_string(),
                description: "Rails authentication solution using Warden".to_string(),
            });
        }

        // Check for JWT
        for entry in WalkDir::new(base_dir)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file() {
                if let Ok(content) = fs::read_to_string(path) {
                    if content.contains("JWT") || content.contains("JsonWebToken") || content.contains("jwt") {
                        self.auth_methods.insert("jwt".to_string(), AuthMethod {
                            name: "JWT".to_string(),
                            method_type: "token".to_string(),
                            source_file: path.to_string_lossy().to_string(),
                            description: "JSON Web Token authentication".to_string(),
                        });
                        break;
                    }
                }
            }
        }

        // Check for OAuth
        for entry in WalkDir::new(base_dir)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file() {
                if let Ok(content) = fs::read_to_string(path) {
                    if content.contains("OAuth") || content.contains("omniauth") {
                        self.auth_methods.insert("oauth".to_string(), AuthMethod {
                            name: "OAuth".to_string(),
                            method_type: "third-party".to_string(),
                            source_file: path.to_string_lossy().to_string(),
                            description: "Third-party authentication via OAuth".to_string(),
                        });
                        break;
                    }
                }
            }
        }

        // Check for Passport (Node.js)
        for entry in WalkDir::new(base_dir)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file() {
                if let Ok(content) = fs::read_to_string(path) {
                    if content.contains("passport") || content.contains("Passport") {
                        self.auth_methods.insert("passport".to_string(), AuthMethod {
                            name: "Passport".to_string(),
                            method_type: "middleware".to_string(),
                            source_file: path.to_string_lossy().to_string(),
                            description: "Node.js authentication middleware".to_string(),
                        });
                        break;
                    }
                }
            }
        }

        // Check for custom authentication
        for entry in WalkDir::new(base_dir)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file() {
                if let Some(file_name) = path.file_name() {
                    if let Some(file_name_str) = file_name.to_str() {
                        if file_name_str.contains("auth") || file_name_str.contains("session") {
                            if let Ok(content) = fs::read_to_string(path) {
                                if content.contains("authenticate") || content.contains("login") || content.contains("sign_in") {
                                    self.auth_methods.insert("custom".to_string(), AuthMethod {
                                        name: "Custom Authentication".to_string(),
                                        method_type: "custom".to_string(),
                                        source_file: path.to_string_lossy().to_string(),
                                        description: "Custom authentication implementation".to_string(),
                                    });
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn analyze_auth_roles(&mut self, base_dir: &PathBuf) {
        // Look for role definitions

        // Check for CanCanCan (Rails)
        let ability_file = base_dir.join("app").join("models").join("ability.rb");
        if ability_file.exists() {
            if let Ok(content) = fs::read_to_string(&ability_file) {
                self.extract_cancancan_roles(&content, ability_file.to_string_lossy().to_string());
            }
        }

        // Check for Pundit (Rails)
        let policies_dir = base_dir.join("app").join("policies");
        if policies_dir.exists() {
            for entry in WalkDir::new(&policies_dir)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                let path = entry.path();
                if path.is_file() && path.extension().map_or(false, |ext| ext == "rb") {
                    if let Ok(content) = fs::read_to_string(path) {
                        self.extract_pundit_roles(&content, path.to_string_lossy().to_string());
                    }
                }
            }
        }

        // Check for role-based checks in controllers
        for entry in WalkDir::new(base_dir.join("app").join("controllers"))
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |ext| ext == "rb") {
                if let Ok(content) = fs::read_to_string(path) {
                    self.extract_controller_roles(&content, path.to_string_lossy().to_string());
                }
            }
        }
    }

    fn extract_cancancan_roles(&mut self, content: &str, source_file: String) {
        // Extract roles from CanCanCan ability.rb
        let role_regex = Regex::new(r#"if\s+user\.([a-z_]+)\?"#).unwrap();

        for cap in role_regex.captures_iter(content) {
            if let Some(role) = cap.get(1) {
                let role_name = role.as_str();

                // Extract permissions for this role
                let mut permissions = Vec::new();
                let permission_regex = Regex::new(r#"can\s+:([a-z_]+)"#).unwrap();

                for perm_cap in permission_regex.captures_iter(content) {
                    if let Some(permission) = perm_cap.get(1) {
                        permissions.push(permission.as_str().to_string());
                    }
                }

                self.auth_roles.insert(role_name.to_string(), AuthRole {
                    name: role_name.to_string(),
                    permissions,
                    source_file: source_file.clone(),
                });
            }
        }
    }

    fn extract_pundit_roles(&mut self, content: &str, source_file: String) {
        // Extract roles from Pundit policies
        if let Some(file_name) = PathBuf::from(&source_file).file_name() {
            if let Some(file_name_str) = file_name.to_str() {
                let role_name = file_name_str.replace("_policy.rb", "");

                // Extract permissions for this role
                let mut permissions = Vec::new();
                let permission_regex = Regex::new(r#"def\s+([a-z_]+)\?"#).unwrap();

                for perm_cap in permission_regex.captures_iter(content) {
                    if let Some(permission) = perm_cap.get(1) {
                        permissions.push(permission.as_str().to_string());
                    }
                }

                self.auth_roles.insert(role_name.to_string(), AuthRole {
                    name: role_name,
                    permissions,
                    source_file,
                });
            }
        }
    }

    fn extract_controller_roles(&mut self, content: &str, source_file: String) {
        // Extract roles from controller role checks
        let role_regex = Regex::new(r#"user\.(\w+)\s*==\s*['"]([^'"]+)['"]|user\.is_(\w+)\?"#).unwrap();

        for cap in role_regex.captures_iter(content) {
            let role_name = if let Some(role1) = cap.get(1) {
                format!("{}_{}", role1.as_str(), cap.get(2).map_or("", |m| m.as_str()))
            } else if let Some(role2) = cap.get(3) {
                role2.as_str().to_string()
            } else {
                continue;
            };

            if !self.auth_roles.contains_key(&role_name) {
                self.auth_roles.insert(role_name.clone(), AuthRole {
                    name: role_name,
                    permissions: Vec::new(), // We don't have explicit permissions from controller checks
                    source_file: source_file.clone(),
                });
            }
        }
    }

    fn analyze_auth_flows(&mut self, base_dir: &PathBuf) {
        // Look for authentication flows

        // Check for login flow
        self.identify_login_flow(base_dir);

        // Check for registration flow
        self.identify_registration_flow(base_dir);

        // Check for password reset flow
        self.identify_password_reset_flow(base_dir);

        // Check for OAuth flow
        self.identify_oauth_flow(base_dir);
    }

    fn identify_login_flow(&mut self, base_dir: &PathBuf) {
        // Look for login controller or related files
        for entry in WalkDir::new(base_dir)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file() {
                if let Some(file_name) = path.file_name() {
                    if let Some(file_name_str) = file_name.to_str() {
                        if file_name_str.contains("login") || file_name_str.contains("session") || file_name_str.contains("sign_in") {
                            if let Ok(content) = fs::read_to_string(path) {
                                let mut steps = Vec::new();

                                // Extract login steps
                                if content.contains("authenticate") || content.contains("login") || content.contains("sign_in") {
                                    steps.push("User submits credentials".to_string());
                                    steps.push("System authenticates user".to_string());

                                    if content.contains("session") || content.contains("cookie") || content.contains("token") {
                                        steps.push("System creates session/token".to_string());
                                    }

                                    if content.contains("redirect") {
                                        steps.push("User is redirected to dashboard/home".to_string());
                                    }

                                    self.auth_flows.push(AuthFlow {
                                        name: "Login Flow".to_string(),
                                        steps,
                                        source_file: path.to_string_lossy().to_string(),
                                    });

                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn identify_registration_flow(&mut self, base_dir: &PathBuf) {
        // Look for registration controller or related files
        for entry in WalkDir::new(base_dir)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file() {
                if let Some(file_name) = path.file_name() {
                    if let Some(file_name_str) = file_name.to_str() {
                        if file_name_str.contains("register") || file_name_str.contains("signup") || file_name_str.contains("sign_up") {
                            if let Ok(content) = fs::read_to_string(path) {
                                let mut steps = Vec::new();

                                // Extract registration steps
                                if content.contains("register") || content.contains("signup") || content.contains("sign_up") || content.contains("create") {
                                    steps.push("User submits registration information".to_string());
                                    steps.push("System validates user input".to_string());
                                    steps.push("System creates new user account".to_string());

                                    if content.contains("email") && content.contains("confirm") {
                                        steps.push("System sends confirmation email".to_string());
                                        steps.push("User confirms email address".to_string());
                                    }

                                    if content.contains("login") || content.contains("sign_in") || content.contains("session") {
                                        steps.push("User is automatically logged in".to_string());
                                    }

                                    if content.contains("redirect") {
                                        steps.push("User is redirected to dashboard/home".to_string());
                                    }

                                    self.auth_flows.push(AuthFlow {
                                        name: "Registration Flow".to_string(),
                                        steps,
                                        source_file: path.to_string_lossy().to_string(),
                                    });

                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn identify_password_reset_flow(&mut self, base_dir: &PathBuf) {
        // Look for password reset controller or related files
        for entry in WalkDir::new(base_dir)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file() {
                if let Some(file_name) = path.file_name() {
                    if let Some(file_name_str) = file_name.to_str() {
                        if file_name_str.contains("password") || file_name_str.contains("reset") {
                            if let Ok(content) = fs::read_to_string(path) {
                                let mut steps = Vec::new();

                                // Extract password reset steps
                                if content.contains("reset") || content.contains("forgot") || content.contains("recover") {
                                    steps.push("User requests password reset".to_string());

                                    if content.contains("email") || content.contains("send") {
                                        steps.push("System sends password reset email".to_string());
                                        steps.push("User clicks reset link in email".to_string());
                                    }

                                    if content.contains("token") || content.contains("verify") {
                                        steps.push("System verifies reset token".to_string());
                                    }

                                    steps.push("User enters new password".to_string());
                                    steps.push("System updates user password".to_string());

                                    if content.contains("login") || content.contains("sign_in") || content.contains("session") {
                                        steps.push("User is redirected to login".to_string());
                                    }

                                    self.auth_flows.push(AuthFlow {
                                        name: "Password Reset Flow".to_string(),
                                        steps,
                                        source_file: path.to_string_lossy().to_string(),
                                    });

                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn identify_oauth_flow(&mut self, base_dir: &PathBuf) {
        // Look for OAuth controller or related files
        for entry in WalkDir::new(base_dir)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file() {
                if let Some(file_name) = path.file_name() {
                    if let Some(file_name_str) = file_name.to_str() {
                        if file_name_str.contains("oauth") || file_name_str.contains("omniauth") || file_name_str.contains("provider") {
                            if let Ok(content) = fs::read_to_string(path) {
                                let mut steps = Vec::new();

                                // Extract OAuth steps
                                if content.contains("oauth") || content.contains("omniauth") || content.contains("provider") {
                                    steps.push("User initiates third-party login".to_string());
                                    steps.push("System redirects to provider".to_string());
                                    steps.push("User authenticates with provider".to_string());
                                    steps.push("Provider redirects back with auth code".to_string());
                                    steps.push("System exchanges code for token".to_string());
                                    steps.push("System creates or updates user account".to_string());
                                    steps.push("User is logged in".to_string());

                                    self.auth_flows.push(AuthFlow {
                                        name: "OAuth Flow".to_string(),
                                        steps,
                                        source_file: path.to_string_lossy().to_string(),
                                    });

                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn identify_protected_routes(&mut self, base_dir: &PathBuf) {
        // Look for routes that require authentication

        // Check Rails routes
        for entry in WalkDir::new(base_dir)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file() && path.file_name().map_or(false, |n| n == "routes.rb") {
                if let Ok(content) = fs::read_to_string(path) {
                    self.extract_protected_rails_routes(&content);
                }
            }
        }

        // Check controllers for before_action authenticate filters
        for entry in WalkDir::new(base_dir.join("app").join("controllers"))
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |ext| ext == "rb") {
                if let Ok(content) = fs::read_to_string(path) {
                    self.extract_controller_auth_filters(&content, path.to_path_buf());
                }
            }
        }

        // Check React Router for protected routes
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
                                self.extract_protected_react_routes(&content);
                            }
                        }
                    }
                }
            }
        }
    }

    fn extract_protected_rails_routes(&mut self, content: &str) {
        // Extract protected routes from routes.rb
        let route_regex = Regex::new(r#"(get|post|put|patch|delete)\s+['"]([^'"]+)['"]"#).unwrap();

        for cap in route_regex.captures_iter(content) {
            if let Some(path) = cap.get(2) {
                let path_str = path.as_str();

                // Check if this route is in a protected block
                if content.contains("authenticate_user!") ||
                   content.contains("before_action :authenticate_user") ||
                   content.contains("before_filter :authenticate_user") {
                    self.protected_routes.insert(path_str.to_string());
                }
            }
        }
    }

    fn extract_controller_auth_filters(&mut self, content: &str, controller_path: PathBuf) {
        // Extract controller actions that require authentication
        if content.contains("before_action :authenticate_user") ||
           content.contains("before_filter :authenticate_user") ||
           content.contains("before_action :require_login") {

            // Extract controller name
            if let Some(file_name) = controller_path.file_name() {
                if let Some(file_name_str) = file_name.to_str() {
                    let controller_name = file_name_str.replace("_controller.rb", "");

                    // Extract actions
                    let action_regex = Regex::new(r#"def\s+([a-z_]+)"#).unwrap();
                    for action_cap in action_regex.captures_iter(content) {
                        if let Some(action) = action_cap.get(1) {
                            let action_name = action.as_str();

                            // Skip if explicitly excluded from authentication
                            if content.contains(&format!("skip_before_action :authenticate_user, only: [:{}]", action_name)) ||
                               content.contains(&format!("skip_before_filter :authenticate_user, only: [:{}]", action_name)) {
                                continue;
                            }

                            // Add as protected route
                            self.protected_routes.insert(format!("/{}/{}", controller_name, action_name));
                        }
                    }
                }
            }
        }
    }

    fn extract_protected_react_routes(&mut self, content: &str) {
        // Extract protected routes from React Router
        if content.contains("PrivateRoute") || content.contains("AuthRoute") || content.contains("ProtectedRoute") {
            let route_regex = Regex::new(r#"<(?:PrivateRoute|AuthRoute|ProtectedRoute)[^>]*path=['"]([^'"]+)['"]"#).unwrap();

            for cap in route_regex.captures_iter(content) {
                if let Some(path) = cap.get(1) {
                    self.protected_routes.insert(path.as_str().to_string());
                }
            }
        }
    }
}
