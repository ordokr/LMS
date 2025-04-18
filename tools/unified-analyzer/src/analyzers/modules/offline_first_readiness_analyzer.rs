use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use std::fs;
use walkdir::WalkDir;
use regex::Regex;
use lazy_static::lazy_static;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DataAccessPattern {
    pub pattern_type: String,
    pub description: String,
    pub files: Vec<String>,
    pub sync_feasibility: SyncFeasibility,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum SyncFeasibility {
    High,
    Medium,
    Low,
    NotFeasible,
}

impl Default for SyncFeasibility {
    fn default() -> Self {
        SyncFeasibility::Medium
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DataUpdatePattern {
    pub pattern_type: String,
    pub description: String,
    pub files: Vec<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ConflictResolutionStrategy {
    pub name: String,
    pub description: String,
    pub files: Vec<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct RealTimeUpdateRequirement {
    pub feature: String,
    pub description: String,
    pub criticality: String,
    pub files: Vec<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct OfflineFirstReadinessAnalyzer {
    pub data_access_patterns: Vec<DataAccessPattern>,
    pub data_update_patterns: Vec<DataUpdatePattern>,
    pub conflict_resolution_strategies: Vec<ConflictResolutionStrategy>,
    pub real_time_update_requirements: Vec<RealTimeUpdateRequirement>,
    pub offline_readiness_score: u8,
    pub recommendations: Vec<String>,
}

impl OfflineFirstReadinessAnalyzer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn analyze(&self, base_dir: &PathBuf) -> Result<String, String> {
        let mut analyzer = OfflineFirstReadinessAnalyzer::default();

        // Detect remote data access patterns
        analyzer.detect_remote_data_access(base_dir);

        // Map data update patterns
        analyzer.map_data_update_patterns(base_dir);

        // Identify conflict resolution strategies
        analyzer.identify_conflict_resolution_strategies(base_dir);

        // Document real-time update requirements
        analyzer.document_real_time_update_requirements(base_dir);

        // Calculate offline readiness score
        analyzer.calculate_offline_readiness_score();

        // Generate recommendations
        analyzer.generate_recommendations();

        match serde_json::to_string_pretty(&analyzer) {
            Ok(json) => Ok(json),
            Err(e) => Err(format!("Failed to serialize OfflineFirstReadinessAnalyzer: {}", e)),
        }
    }

    fn detect_remote_data_access(&mut self, base_dir: &PathBuf) {
        // Define regex patterns for different remote data access methods
        lazy_static! {
            // jQuery AJAX patterns
            static ref JQUERY_AJAX_REGEX: Regex = Regex::new(r#"\$\.(ajax|get|post|put|delete)\s*\(\s*\{?[^}]*url\s*:\s*['"]([^'"]+)['"]"#).unwrap();
            static ref JQUERY_SHORTHAND_REGEX: Regex = Regex::new(r#"\$\.(get|post|put|delete)\s*\(\s*['"]([^'"]+)['"]"#).unwrap();

            // Fetch API patterns
            static ref FETCH_REGEX: Regex = Regex::new(r#"fetch\s*\(\s*['"]([^'"]+)['"]"#).unwrap();
            static ref FETCH_OPTIONS_REGEX: Regex = Regex::new(r#"fetch\s*\(\s*['"]([^'"]+)['"]\s*,\s*\{[^}]*method\s*:\s*['"]([^'"]+)['"]"#).unwrap();

            // Axios patterns
            static ref AXIOS_REGEX: Regex = Regex::new(r#"axios\s*\.\s*(get|post|put|delete|patch)\s*\(\s*['"]([^'"]+)['"]"#).unwrap();
            static ref AXIOS_INSTANCE_REGEX: Regex = Regex::new(r#"axios\s*\(\s*\{[^}]*url\s*:\s*['"]([^'"]+)['"]"#).unwrap();

            // Rails HTTP patterns
            static ref RAILS_HTTP_REGEX: Regex = Regex::new(r#"(HTTParty|Net::HTTP)\.(get|post|put|delete|patch)\s*\(\s*['"]([^'"]+)['"]"#).unwrap();
            static ref RAILS_REQUEST_REGEX: Regex = Regex::new(r#"(request|response)\.(get|post|put|delete|patch)\s*['"]([^'"]+)['"]"#).unwrap();

            // WebSocket patterns
            static ref WEBSOCKET_REGEX: Regex = Regex::new(r#"(new\s+WebSocket\s*\(\s*['"]([^'"]+)['"]|ActionCable\.createConsumer\s*\(\s*['"]([^'"]+)['"]|socket\.io|io\.connect)"#).unwrap();

            // GraphQL patterns
            static ref GRAPHQL_REGEX: Regex = Regex::new(r#"(graphql|GraphQL|ApolloClient|gql|useQuery|useMutation)"#).unwrap();

            // REST API patterns
            static ref REST_API_REGEX: Regex = Regex::new(r#"(api|API|endpoint|resource)\s*['"]([^'"]+)['"]"#).unwrap();
        }

        // Look for remote data access patterns
        for entry in WalkDir::new(base_dir)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file() {
                if let Some(extension) = path.extension() {
                    if let Some(ext) = extension.to_str() {
                        if ext == "js" || ext == "ts" || ext == "jsx" || ext == "tsx" || ext == "rb" || ext == "erb" || ext == "html" {
                            if let Ok(content) = fs::read_to_string(path) {
                                if let Some(rel_path) = path.strip_prefix(base_dir).ok() {
                                    let file_path = rel_path.to_string_lossy().to_string();

                                    // Check for jQuery AJAX calls
                                    if JQUERY_AJAX_REGEX.is_match(&content) || JQUERY_SHORTHAND_REGEX.is_match(&content) {
                                        // Extract endpoints from jQuery AJAX calls
                                        let mut endpoints = Vec::new();

                                        for cap in JQUERY_AJAX_REGEX.captures_iter(&content) {
                                            if let Some(endpoint) = cap.get(2) {
                                                endpoints.push(endpoint.as_str().to_string());
                                            }
                                        }

                                        for cap in JQUERY_SHORTHAND_REGEX.captures_iter(&content) {
                                            if let Some(endpoint) = cap.get(2) {
                                                endpoints.push(endpoint.as_str().to_string());
                                            }
                                        }

                                        let description = if !endpoints.is_empty() {
                                            format!("jQuery AJAX calls to: {}", endpoints.join(", "))
                                        } else {
                                            "jQuery AJAX call".to_string()
                                        };

                                        self.data_access_patterns.push(DataAccessPattern {
                                            pattern_type: "AJAX".to_string(),
                                            description,
                                            files: vec![file_path.clone()],
                                            sync_feasibility: SyncFeasibility::Medium,
                                        });
                                    }

                                    // Check for Fetch API
                                    if FETCH_REGEX.is_match(&content) {
                                        // Extract endpoints from Fetch API calls
                                        let mut endpoints = Vec::new();
                                        let mut methods = Vec::new();

                                        for cap in FETCH_REGEX.captures_iter(&content) {
                                            if let Some(endpoint) = cap.get(1) {
                                                endpoints.push(endpoint.as_str().to_string());
                                            }
                                        }

                                        for cap in FETCH_OPTIONS_REGEX.captures_iter(&content) {
                                            if let Some(method) = cap.get(2) {
                                                methods.push(method.as_str().to_string());
                                            }
                                        }

                                        let description = if !endpoints.is_empty() {
                                            if !methods.is_empty() {
                                                format!("Fetch API calls to: {} using methods: {}", endpoints.join(", "), methods.join(", "))
                                            } else {
                                                format!("Fetch API calls to: {}", endpoints.join(", "))
                                            }
                                        } else {
                                            "Modern Fetch API call".to_string()
                                        };

                                        self.data_access_patterns.push(DataAccessPattern {
                                            pattern_type: "Fetch API".to_string(),
                                            description,
                                            files: vec![file_path.clone()],
                                            sync_feasibility: SyncFeasibility::High,
                                        });
                                    }

                                    // Check for Axios
                                    if AXIOS_REGEX.is_match(&content) || AXIOS_INSTANCE_REGEX.is_match(&content) {
                                        // Extract endpoints from Axios calls
                                        let mut endpoints = Vec::new();
                                        let mut methods = Vec::new();

                                        for cap in AXIOS_REGEX.captures_iter(&content) {
                                            if let Some(method) = cap.get(1) {
                                                methods.push(method.as_str().to_string());
                                            }
                                            if let Some(endpoint) = cap.get(2) {
                                                endpoints.push(endpoint.as_str().to_string());
                                            }
                                        }

                                        for cap in AXIOS_INSTANCE_REGEX.captures_iter(&content) {
                                            if let Some(endpoint) = cap.get(1) {
                                                endpoints.push(endpoint.as_str().to_string());
                                            }
                                        }

                                        let description = if !endpoints.is_empty() {
                                            if !methods.is_empty() {
                                                format!("Axios HTTP client calls to: {} using methods: {}", endpoints.join(", "), methods.join(", "))
                                            } else {
                                                format!("Axios HTTP client calls to: {}", endpoints.join(", "))
                                            }
                                        } else {
                                            "Axios HTTP client".to_string()
                                        };

                                        self.data_access_patterns.push(DataAccessPattern {
                                            pattern_type: "Axios".to_string(),
                                            description,
                                            files: vec![file_path.clone()],
                                            sync_feasibility: SyncFeasibility::High,
                                        });
                                    }

                                    // Check for Rails HTTP requests
                                    if RAILS_HTTP_REGEX.is_match(&content) || RAILS_REQUEST_REGEX.is_match(&content) {
                                        // Extract endpoints from Rails HTTP requests
                                        let mut endpoints = Vec::new();
                                        let mut methods = Vec::new();

                                        for cap in RAILS_HTTP_REGEX.captures_iter(&content) {
                                            if let Some(method) = cap.get(2) {
                                                methods.push(method.as_str().to_string());
                                            }
                                            if let Some(endpoint) = cap.get(3) {
                                                endpoints.push(endpoint.as_str().to_string());
                                            }
                                        }

                                        for cap in RAILS_REQUEST_REGEX.captures_iter(&content) {
                                            if let Some(method) = cap.get(2) {
                                                methods.push(method.as_str().to_string());
                                            }
                                            if let Some(endpoint) = cap.get(3) {
                                                endpoints.push(endpoint.as_str().to_string());
                                            }
                                        }

                                        let description = if !endpoints.is_empty() {
                                            if !methods.is_empty() {
                                                format!("Ruby HTTP client calls to: {} using methods: {}", endpoints.join(", "), methods.join(", "))
                                            } else {
                                                format!("Ruby HTTP client calls to: {}", endpoints.join(", "))
                                            }
                                        } else {
                                            "Ruby HTTP client".to_string()
                                        };

                                        self.data_access_patterns.push(DataAccessPattern {
                                            pattern_type: "Rails HTTP".to_string(),
                                            description,
                                            files: vec![file_path.clone()],
                                            sync_feasibility: SyncFeasibility::Medium,
                                        });
                                    }

                                    // Check for WebSockets
                                    if WEBSOCKET_REGEX.is_match(&content) {
                                        // Extract WebSocket endpoints
                                        let mut endpoints = Vec::new();

                                        for cap in WEBSOCKET_REGEX.captures_iter(&content) {
                                            if let Some(endpoint) = cap.get(2).or_else(|| cap.get(3)) {
                                                endpoints.push(endpoint.as_str().to_string());
                                            }
                                        }

                                        let description = if !endpoints.is_empty() {
                                            format!("WebSocket connections to: {}", endpoints.join(", "))
                                        } else {
                                            "Real-time WebSocket communication".to_string()
                                        };

                                        self.data_access_patterns.push(DataAccessPattern {
                                            pattern_type: "WebSockets".to_string(),
                                            description,
                                            files: vec![file_path.clone()],
                                            sync_feasibility: SyncFeasibility::Low,
                                        });
                                    }

                                    // Check for GraphQL
                                    if GRAPHQL_REGEX.is_match(&content) {
                                        self.data_access_patterns.push(DataAccessPattern {
                                            pattern_type: "GraphQL".to_string(),
                                            description: "GraphQL API queries and mutations".to_string(),
                                            files: vec![file_path.clone()],
                                            sync_feasibility: SyncFeasibility::Medium,
                                        });
                                    }

                                    // Check for REST API patterns
                                    if REST_API_REGEX.is_match(&content) {
                                        // Extract API endpoints
                                        let mut endpoints = Vec::new();

                                        for cap in REST_API_REGEX.captures_iter(&content) {
                                            if let Some(endpoint) = cap.get(2) {
                                                endpoints.push(endpoint.as_str().to_string());
                                            }
                                        }

                                        let description = if !endpoints.is_empty() {
                                            format!("REST API endpoints: {}", endpoints.join(", "))
                                        } else {
                                            "REST API endpoints".to_string()
                                        };

                                        self.data_access_patterns.push(DataAccessPattern {
                                            pattern_type: "REST API".to_string(),
                                            description,
                                            files: vec![file_path.clone()],
                                            sync_feasibility: SyncFeasibility::High,
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

    fn map_data_update_patterns(&mut self, base_dir: &PathBuf) {
        // Define regex patterns for different data update methods
        lazy_static! {
            // Form submission patterns
            static ref FORM_SUBMIT_REGEX: Regex = Regex::new(r#"<form[^>]*>|\bform\b.*\b(submit|onSubmit)\b|\bhandleSubmit\b"#).unwrap();
            static ref FORM_ACTION_REGEX: Regex = Regex::new(r#"<form[^>]*action=['"]([^'"]+)['"]"#).unwrap();

            // AJAX update patterns
            static ref AJAX_UPDATE_REGEX: Regex = Regex::new(r#"\$\.(ajax|post|put|patch)\s*\(\s*\{[^}]*url\s*:\s*['"]([^'"]+)['"]"#).unwrap();
            static ref AJAX_DATA_REGEX: Regex = Regex::new(r#"\$\.(ajax|post|put|patch)[^}]*data\s*:\s*\{([^}]+)\}"#).unwrap();
        }

        // Look for patterns related to data updates
        for entry in WalkDir::new(base_dir)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file() {
                if let Some(extension) = path.extension() {
                    if let Some(ext) = extension.to_str() {
                        if ext == "js" || ext == "ts" || ext == "jsx" || ext == "tsx" || ext == "rb" || ext == "erb" || ext == "html" {
                            if let Ok(content) = fs::read_to_string(path) {
                                if let Some(rel_path) = path.strip_prefix(base_dir).ok() {
                                    let file_path = rel_path.to_string_lossy().to_string();

                                    // Check for form submissions
                                    if FORM_SUBMIT_REGEX.is_match(&content) {
                                        // Extract form actions
                                        let mut actions = Vec::new();

                                        for cap in FORM_ACTION_REGEX.captures_iter(&content) {
                                            if let Some(action) = cap.get(1) {
                                                actions.push(action.as_str().to_string());
                                            }
                                        }

                                        let description = if !actions.is_empty() {
                                            format!("Form submissions to: {}", actions.join(", "))
                                        } else {
                                            "Traditional form submission".to_string()
                                        };

                                        self.data_update_patterns.push(DataUpdatePattern {
                                            pattern_type: "Form Submission".to_string(),
                                            description,
                                            files: vec![file_path.clone()],
                                        });
                                    }

                                    // Check for AJAX updates
                                    if AJAX_UPDATE_REGEX.is_match(&content) {
                                        // Extract AJAX update endpoints and data
                                        let mut endpoints = Vec::new();
                                        let mut data_fields = Vec::new();

                                        for cap in AJAX_UPDATE_REGEX.captures_iter(&content) {
                                            if let Some(endpoint) = cap.get(2) {
                                                endpoints.push(endpoint.as_str().to_string());
                                            }
                                        }

                                        for cap in AJAX_DATA_REGEX.captures_iter(&content) {
                                            if let Some(data) = cap.get(2) {
                                                // Extract field names from data object
                                                let data_str = data.as_str();
                                                let field_regex = Regex::new(r#"\b([a-zA-Z0-9_]+)\s*:\s*"#).unwrap();

                                                for field_cap in field_regex.captures_iter(data_str) {
                                                    if let Some(field) = field_cap.get(1) {
                                                        data_fields.push(field.as_str().to_string());
                                                    }
                                                }
                                            }
                                        }

                                        let description = if !endpoints.is_empty() {
                                            if !data_fields.is_empty() {
                                                format!("AJAX updates to: {} with fields: {}", endpoints.join(", "), data_fields.join(", "))
                                            } else {
                                                format!("AJAX updates to: {}", endpoints.join(", "))
                                            }
                                        } else {
                                            "Asynchronous data update via AJAX".to_string()
                                        };

                                        self.data_update_patterns.push(DataUpdatePattern {
                                            pattern_type: "AJAX Update".to_string(),
                                            description,
                                            files: vec![file_path.clone()],
                                        });
                                    }

                                    // Define more regex patterns for REST API and real-time updates
                                    lazy_static! {
                                        // REST API update patterns
                                        static ref REST_UPDATE_REGEX: Regex = Regex::new(r#"(api|API|endpoint|route).*?['"](POST|PUT|PATCH|DELETE)['"]|['"](POST|PUT|PATCH|DELETE)['"].*?(api|API|endpoint|route)"#).unwrap();

                                        // Real-time update patterns
                                        static ref REALTIME_UPDATE_REGEX: Regex = Regex::new(r#"(real-time|realtime|live\s+update|socket\.emit|ActionCable\.createConsumer)"#).unwrap();
                                    }

                                    // Check for REST API updates
                                    if REST_UPDATE_REGEX.is_match(&content) {
                                        // Extract HTTP methods and endpoints
                                        let method_regex = Regex::new(r#"['"](POST|PUT|PATCH|DELETE)['"]"#).unwrap();
                                        let endpoint_regex = Regex::new(r#"(api|API|endpoint|route).*?['"]([^'"]+)['"]"#).unwrap();

                                        let mut methods = Vec::new();
                                        let mut endpoints = Vec::new();

                                        for cap in method_regex.captures_iter(&content) {
                                            if let Some(method) = cap.get(1) {
                                                methods.push(method.as_str().to_string());
                                            }
                                        }

                                        for cap in endpoint_regex.captures_iter(&content) {
                                            if let Some(endpoint) = cap.get(2) {
                                                endpoints.push(endpoint.as_str().to_string());
                                            }
                                        }

                                        let description = if !endpoints.is_empty() {
                                            if !methods.is_empty() {
                                                format!("RESTful API updates to: {} using methods: {}", endpoints.join(", "), methods.join(", "))
                                            } else {
                                                format!("RESTful API updates to: {}", endpoints.join(", "))
                                            }
                                        } else if !methods.is_empty() {
                                            format!("RESTful API updates using methods: {}", methods.join(", "))
                                        } else {
                                            "RESTful API data update".to_string()
                                        };

                                        self.data_update_patterns.push(DataUpdatePattern {
                                            pattern_type: "REST API".to_string(),
                                            description,
                                            files: vec![file_path.clone()],
                                        });
                                    }

                                    // Check for real-time updates
                                    if REALTIME_UPDATE_REGEX.is_match(&content) {
                                        // Extract real-time update details
                                        let socket_regex = Regex::new(r#"socket\.emit\s*\(\s*['"]([^'"]+)['"]"#).unwrap();
                                        let action_cable_regex = Regex::new(r#"ActionCable\.createConsumer\s*\(\s*['"]([^'"]+)['"]"#).unwrap();

                                        let mut events = Vec::new();
                                        let mut channels = Vec::new();

                                        for cap in socket_regex.captures_iter(&content) {
                                            if let Some(event) = cap.get(1) {
                                                events.push(event.as_str().to_string());
                                            }
                                        }

                                        for cap in action_cable_regex.captures_iter(&content) {
                                            if let Some(channel) = cap.get(1) {
                                                channels.push(channel.as_str().to_string());
                                            }
                                        }

                                        let description = if !events.is_empty() {
                                            format!("Socket.io real-time events: {}", events.join(", "))
                                        } else if !channels.is_empty() {
                                            format!("ActionCable channels: {}", channels.join(", "))
                                        } else {
                                            "Live data updates".to_string()
                                        };

                                        self.data_update_patterns.push(DataUpdatePattern {
                                            pattern_type: "Real-time Update".to_string(),
                                            description,
                                            files: vec![file_path.clone()],
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

    fn identify_conflict_resolution_strategies(&mut self, base_dir: &PathBuf) {
        // Define regex patterns for conflict resolution strategies
        lazy_static! {
            // Timestamp-based conflict resolution
            static ref TIMESTAMP_CONFLICT_REGEX: Regex = Regex::new(r#"(timestamp|updated_at|last_modified).*?(conflict|resolution|sync|merge)|resolve.*?(conflict|sync).*?(timestamp|updated_at|last_modified)"#).unwrap();

            // Version-based conflict resolution
            static ref VERSION_CONFLICT_REGEX: Regex = Regex::new(r#"(version|revision|etag).*?(conflict|resolution|sync|merge)|resolve.*?(conflict|sync).*?(version|revision|etag)"#).unwrap();

            // Merge-based conflict resolution
            static ref MERGE_CONFLICT_REGEX: Regex = Regex::new(r#"(merge|combine|reconcile).*?(conflict|resolution|sync)|resolve.*?(conflict|sync).*?(merge|combine|reconcile)"#).unwrap();

            // Custom conflict resolution
            static ref CUSTOM_CONFLICT_REGEX: Regex = Regex::new(r#"(resolve|handle|manage).*?(conflict|sync|collision).*?(function|method|strategy|algorithm|custom)|(function|method|strategy|algorithm|custom).*?(resolve|handle|manage).*?(conflict|sync|collision)"#).unwrap();

            // Last-write-wins strategy
            static ref LWW_CONFLICT_REGEX: Regex = Regex::new(r#"(last.write.wins|lww|overwrite|latest).*?(conflict|resolution|sync|strategy)"#).unwrap();

            // Operational transformation
            static ref OT_CONFLICT_REGEX: Regex = Regex::new(r#"(operational.transformation|OT|transform.operation)"#).unwrap();

            // Conflict-free replicated data types (CRDTs)
            static ref CRDT_CONFLICT_REGEX: Regex = Regex::new(r#"(CRDT|conflict.free|replicated.data.type)"#).unwrap();
        }

        // Look for conflict resolution strategies
        for entry in WalkDir::new(base_dir)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file() {
                if let Some(extension) = path.extension() {
                    if let Some(ext) = extension.to_str() {
                        if ext == "js" || ext == "ts" || ext == "jsx" || ext == "tsx" || ext == "rb" || ext == "md" || ext == "txt" {
                            if let Ok(content) = fs::read_to_string(path) {
                                if let Some(rel_path) = path.strip_prefix(base_dir).ok() {
                                    let file_path = rel_path.to_string_lossy().to_string();

                                    // Check for timestamp-based conflict resolution
                                    if TIMESTAMP_CONFLICT_REGEX.is_match(&content) {
                                        // Extract specific timestamp fields or methods
                                        let timestamp_field_regex = Regex::new(r#"(timestamp|updated_at|last_modified|modified_date)\s*[=:]"#).unwrap();
                                        let mut timestamp_fields = Vec::new();

                                        for cap in timestamp_field_regex.captures_iter(&content) {
                                            if let Some(field) = cap.get(1) {
                                                timestamp_fields.push(field.as_str().to_string());
                                            }
                                        }

                                        let description = if !timestamp_fields.is_empty() {
                                            format!("Uses timestamp fields ({}) to resolve conflicts", timestamp_fields.join(", "))
                                        } else {
                                            "Uses timestamps to resolve conflicts".to_string()
                                        };

                                        self.conflict_resolution_strategies.push(ConflictResolutionStrategy {
                                            name: "Timestamp-based".to_string(),
                                            description,
                                            files: vec![file_path.clone()],
                                        });
                                    }

                                    // Check for version-based conflict resolution
                                    if VERSION_CONFLICT_REGEX.is_match(&content) {
                                        // Extract specific version fields or methods
                                        let version_field_regex = Regex::new(r#"(version|revision|etag|version_number)\s*[=:]"#).unwrap();
                                        let mut version_fields = Vec::new();

                                        for cap in version_field_regex.captures_iter(&content) {
                                            if let Some(field) = cap.get(1) {
                                                version_fields.push(field.as_str().to_string());
                                            }
                                        }

                                        let description = if !version_fields.is_empty() {
                                            format!("Uses version fields ({}) to resolve conflicts", version_fields.join(", "))
                                        } else {
                                            "Uses version numbers to resolve conflicts".to_string()
                                        };

                                        self.conflict_resolution_strategies.push(ConflictResolutionStrategy {
                                            name: "Version-based".to_string(),
                                            description,
                                            files: vec![file_path.clone()],
                                        });
                                    }

                                    // Check for merge-based conflict resolution
                                    if MERGE_CONFLICT_REGEX.is_match(&content) {
                                        // Extract merge strategy details
                                        let merge_strategy_regex = Regex::new(r#"(merge|combine|reconcile)\s*[^\w]+(\w+)"#).unwrap();
                                        let mut merge_strategies = Vec::new();

                                        for cap in merge_strategy_regex.captures_iter(&content) {
                                            if let Some(strategy) = cap.get(2) {
                                                merge_strategies.push(strategy.as_str().to_string());
                                            }
                                        }

                                        let description = if !merge_strategies.is_empty() {
                                            format!("Merges conflicting changes using strategies: {}", merge_strategies.join(", "))
                                        } else {
                                            "Merges conflicting changes".to_string()
                                        };

                                        self.conflict_resolution_strategies.push(ConflictResolutionStrategy {
                                            name: "Merge-based".to_string(),
                                            description,
                                            files: vec![file_path.clone()],
                                        });
                                    }

                                    // Check for last-write-wins strategy
                                    if LWW_CONFLICT_REGEX.is_match(&content) {
                                        self.conflict_resolution_strategies.push(ConflictResolutionStrategy {
                                            name: "Last-Write-Wins".to_string(),
                                            description: "Resolves conflicts by keeping the most recent write".to_string(),
                                            files: vec![file_path.clone()],
                                        });
                                    }

                                    // Check for operational transformation
                                    if OT_CONFLICT_REGEX.is_match(&content) {
                                        self.conflict_resolution_strategies.push(ConflictResolutionStrategy {
                                            name: "Operational Transformation".to_string(),
                                            description: "Uses OT algorithms to transform concurrent operations".to_string(),
                                            files: vec![file_path.clone()],
                                        });
                                    }

                                    // Check for CRDTs
                                    if CRDT_CONFLICT_REGEX.is_match(&content) {
                                        self.conflict_resolution_strategies.push(ConflictResolutionStrategy {
                                            name: "CRDT".to_string(),
                                            description: "Uses Conflict-free Replicated Data Types for automatic conflict resolution".to_string(),
                                            files: vec![file_path.clone()],
                                        });
                                    }

                                    // Check for custom conflict resolution
                                    if CUSTOM_CONFLICT_REGEX.is_match(&content) &&
                                       !TIMESTAMP_CONFLICT_REGEX.is_match(&content) &&
                                       !VERSION_CONFLICT_REGEX.is_match(&content) &&
                                       !MERGE_CONFLICT_REGEX.is_match(&content) &&
                                       !LWW_CONFLICT_REGEX.is_match(&content) &&
                                       !OT_CONFLICT_REGEX.is_match(&content) &&
                                       !CRDT_CONFLICT_REGEX.is_match(&content) {

                                        // Extract function names related to conflict resolution
                                        let function_regex = Regex::new(r#"function\s+(resolve\w*|handle\w*Conflict|merge\w*|reconcile\w*)"#).unwrap();
                                        let mut functions = Vec::new();

                                        for cap in function_regex.captures_iter(&content) {
                                            if let Some(func) = cap.get(1) {
                                                functions.push(func.as_str().to_string());
                                            }
                                        }

                                        let description = if !functions.is_empty() {
                                            format!("Custom conflict resolution logic using functions: {}", functions.join(", "))
                                        } else {
                                            "Custom conflict resolution logic".to_string()
                                        };

                                        self.conflict_resolution_strategies.push(ConflictResolutionStrategy {
                                            name: "Custom Resolution".to_string(),
                                            description,
                                            files: vec![file_path.clone()],
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

    fn document_real_time_update_requirements(&mut self, base_dir: &PathBuf) {
        // Define regex patterns for real-time update requirements
        lazy_static! {
            // Chat/messaging patterns
            static ref CHAT_REGEX: Regex = Regex::new(r#"(chat|message|messaging|conversation).*?(real.?time|live|instant|socket|websocket)|(real.?time|live|instant|socket|websocket).*?(chat|message|messaging|conversation)"#).unwrap();

            // Notification patterns
            static ref NOTIFICATION_REGEX: Regex = Regex::new(r#"(notification|alert|notify).*?(real.?time|live|push|instant)|(real.?time|live|push|instant).*?(notification|alert|notify)"#).unwrap();

            // Collaborative editing patterns
            static ref COLLAB_EDIT_REGEX: Regex = Regex::new(r#"(collaborative|real.?time.edit|simultaneous.edit|concurrent.edit|shared.edit)"#).unwrap();

            // Live updates patterns
            static ref LIVE_UPDATE_REGEX: Regex = Regex::new(r#"(live.update|auto.refresh|auto.update|real.?time.update|polling|stream)"#).unwrap();

            // Real-time dashboard patterns
            static ref DASHBOARD_REGEX: Regex = Regex::new(r#"(dashboard|analytics|metrics|chart|graph).*?(real.?time|live|auto.update)|(real.?time|live|auto.update).*?(dashboard|analytics|metrics|chart|graph)"#).unwrap();

            // Multiplayer/gaming patterns
            static ref MULTIPLAYER_REGEX: Regex = Regex::new(r#"(multiplayer|game|player).*?(real.?time|live|sync|websocket)|(real.?time|live|sync|websocket).*?(multiplayer|game|player)"#).unwrap();

            // Auction/bidding patterns
            static ref AUCTION_REGEX: Regex = Regex::new(r#"(auction|bid|bidding).*?(real.?time|live|instant)|(real.?time|live|instant).*?(auction|bid|bidding)"#).unwrap();
        }

        // Look for real-time update requirements
        for entry in WalkDir::new(base_dir)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file() {
                if let Some(extension) = path.extension() {
                    if let Some(ext) = extension.to_str() {
                        if ext == "js" || ext == "ts" || ext == "jsx" || ext == "tsx" || ext == "rb" || ext == "erb" || ext == "html" || ext == "md" {
                            if let Ok(content) = fs::read_to_string(path) {
                                if let Some(rel_path) = path.strip_prefix(base_dir).ok() {
                                    let file_path = rel_path.to_string_lossy().to_string();

                                    // Check for chat or messaging features
                                    if CHAT_REGEX.is_match(&content) {
                                        // Extract specific chat features
                                        let chat_feature_regex = Regex::new(r#"(chat|message)\s*(component|service|controller|module|system)"#).unwrap();
                                        let mut chat_features = Vec::new();

                                        for cap in chat_feature_regex.captures_iter(&content) {
                                            if let (Some(feature), Some(component)) = (cap.get(1), cap.get(2)) {
                                                chat_features.push(format!("{} {}", feature.as_str(), component.as_str()));
                                            }
                                        }

                                        let description = if !chat_features.is_empty() {
                                            format!("Real-time chat or messaging functionality: {}", chat_features.join(", "))
                                        } else {
                                            "Real-time chat or messaging functionality".to_string()
                                        };

                                        self.real_time_update_requirements.push(RealTimeUpdateRequirement {
                                            feature: "Chat/Messaging".to_string(),
                                            description,
                                            criticality: "High".to_string(),
                                            files: vec![file_path.clone()],
                                        });
                                    }

                                    // Check for notifications
                                    if NOTIFICATION_REGEX.is_match(&content) {
                                        // Extract notification types
                                        let notification_type_regex = Regex::new(r#"(notification|alert)\s*(type|kind|category)\s*[=:]\s*['"]([^'"]+)['"]"#).unwrap();
                                        let mut notification_types = Vec::new();

                                        for cap in notification_type_regex.captures_iter(&content) {
                                            if let Some(notif_type) = cap.get(3) {
                                                notification_types.push(notif_type.as_str().to_string());
                                            }
                                        }

                                        let description = if !notification_types.is_empty() {
                                            format!("Real-time notification system with types: {}", notification_types.join(", "))
                                        } else {
                                            "Real-time notification system".to_string()
                                        };

                                        self.real_time_update_requirements.push(RealTimeUpdateRequirement {
                                            feature: "Notifications".to_string(),
                                            description,
                                            criticality: "Medium".to_string(),
                                            files: vec![file_path.clone()],
                                        });
                                    }

                                    // Check for collaborative editing
                                    if COLLAB_EDIT_REGEX.is_match(&content) {
                                        // Extract collaborative editing details
                                        let collab_tech_regex = Regex::new(r#"(OT|operational transformation|CRDT|differential synchronization|ShareDB|yjs|automerge)"#).unwrap();
                                        let mut collab_techs = Vec::new();

                                        for cap in collab_tech_regex.captures_iter(&content) {
                                            if let Some(tech) = cap.get(1) {
                                                collab_techs.push(tech.as_str().to_string());
                                            }
                                        }

                                        let description = if !collab_techs.is_empty() {
                                            format!("Real-time collaborative document editing using: {}", collab_techs.join(", "))
                                        } else {
                                            "Real-time collaborative document editing".to_string()
                                        };

                                        self.real_time_update_requirements.push(RealTimeUpdateRequirement {
                                            feature: "Collaborative Editing".to_string(),
                                            description,
                                            criticality: "High".to_string(),
                                            files: vec![file_path.clone()],
                                        });
                                    }

                                    // Check for live updates
                                    if LIVE_UPDATE_REGEX.is_match(&content) {
                                        // Extract update interval or mechanism
                                        let interval_regex = Regex::new(r#"(interval|timeout|poll)\s*[=:]\s*([0-9]+)"#).unwrap();
                                        let mut intervals = Vec::new();

                                        for cap in interval_regex.captures_iter(&content) {
                                            if let Some(interval) = cap.get(2) {
                                                intervals.push(interval.as_str().to_string());
                                            }
                                        }

                                        let description = if !intervals.is_empty() {
                                            format!("Automatic content refreshing with intervals: {} ms", intervals.join(", "))
                                        } else {
                                            "Automatic content refreshing".to_string()
                                        };

                                        self.real_time_update_requirements.push(RealTimeUpdateRequirement {
                                            feature: "Live Updates".to_string(),
                                            description,
                                            criticality: "Medium".to_string(),
                                            files: vec![file_path.clone()],
                                        });
                                    }

                                    // Check for real-time dashboards
                                    if DASHBOARD_REGEX.is_match(&content) {
                                        self.real_time_update_requirements.push(RealTimeUpdateRequirement {
                                            feature: "Real-time Dashboard".to_string(),
                                            description: "Live updating analytics dashboard".to_string(),
                                            criticality: "Medium".to_string(),
                                            files: vec![file_path.clone()],
                                        });
                                    }

                                    // Check for multiplayer/gaming features
                                    if MULTIPLAYER_REGEX.is_match(&content) {
                                        self.real_time_update_requirements.push(RealTimeUpdateRequirement {
                                            feature: "Multiplayer/Gaming".to_string(),
                                            description: "Real-time multiplayer or gaming functionality".to_string(),
                                            criticality: "High".to_string(),
                                            files: vec![file_path.clone()],
                                        });
                                    }

                                    // Check for auction/bidding features
                                    if AUCTION_REGEX.is_match(&content) {
                                        self.real_time_update_requirements.push(RealTimeUpdateRequirement {
                                            feature: "Auction/Bidding".to_string(),
                                            description: "Real-time auction or bidding system".to_string(),
                                            criticality: "High".to_string(),
                                            files: vec![file_path.clone()],
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

    fn calculate_offline_readiness_score(&mut self) {
        // Calculate a score based on the analysis results
        let mut score = 50; // Start with a neutral score

        // Adjust score based on data access patterns
        for pattern in &self.data_access_patterns {
            match pattern.sync_feasibility {
                SyncFeasibility::High => score += 5,
                SyncFeasibility::Medium => score += 2,
                SyncFeasibility::Low => score -= 2,
                SyncFeasibility::NotFeasible => score -= 5,
            }
        }

        // Adjust score based on conflict resolution strategies
        score += self.conflict_resolution_strategies.len() as i32 * 5;

        // Adjust score based on real-time update requirements
        for requirement in &self.real_time_update_requirements {
            if requirement.criticality == "High" {
                score -= 5;
            } else if requirement.criticality == "Medium" {
                score -= 2;
            }
        }

        // Ensure score is within 0-100 range
        if score < 0 {
            score = 0;
        } else if score > 100 {
            score = 100;
        }

        self.offline_readiness_score = score as u8;
    }

    fn generate_recommendations(&mut self) {
        // Generate recommendations based on the analysis

        // Add general recommendation based on score
        if self.offline_readiness_score < 30 {
            self.recommendations.push("The application has significant challenges for offline-first implementation. Consider redesigning the architecture.".to_string());
        } else if self.offline_readiness_score < 60 {
            self.recommendations.push("The application requires moderate changes to support offline-first functionality.".to_string());
        } else {
            self.recommendations.push("The application is well-suited for offline-first implementation with minimal changes.".to_string());
        }

        // Add specific recommendations based on findings
        if self.data_access_patterns.iter().any(|p| p.pattern_type == "WebSockets") {
            self.recommendations.push("Replace WebSocket communication with a store-and-forward pattern for offline support.".to_string());
        }

        if self.conflict_resolution_strategies.is_empty() {
            self.recommendations.push("Implement conflict resolution strategies (e.g., timestamp-based or version-based) for offline data synchronization.".to_string());
        }

        if self.real_time_update_requirements.iter().any(|r| r.criticality == "High") {
            self.recommendations.push("Provide fallback mechanisms for high-criticality real-time features during offline operation.".to_string());
        }

        // Add technology-specific recommendations
        self.recommendations.push("Consider using IndexedDB or SQLite for client-side data storage.".to_string());
        self.recommendations.push("Implement a background sync mechanism using Service Workers.".to_string());
        self.recommendations.push("Add a queue system for operations performed while offline.".to_string());
    }
}