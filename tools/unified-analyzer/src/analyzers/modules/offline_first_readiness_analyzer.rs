use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::fs;
use walkdir::WalkDir;
use regex::Regex;

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
        // Look for AJAX calls, fetch, axios, etc.
        for entry in WalkDir::new(base_dir)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file() {
                if let Some(extension) = path.extension() {
                    if let Some(ext) = extension.to_str() {
                        if ext == "js" || ext == "ts" || ext == "jsx" || ext == "tsx" || ext == "rb" {
                            if let Ok(content) = fs::read_to_string(path) {
                                // Check for AJAX calls
                                if content.contains("$.ajax") || content.contains("$.get") || content.contains("$.post") {
                                    if let Some(rel_path) = path.strip_prefix(base_dir).ok() {
                                        let file_path = rel_path.to_string_lossy().to_string();

                                        self.data_access_patterns.push(DataAccessPattern {
                                            pattern_type: "AJAX".to_string(),
                                            description: "jQuery AJAX call".to_string(),
                                            files: vec![file_path],
                                            sync_feasibility: SyncFeasibility::Medium,
                                        });
                                    }
                                }

                                // Check for fetch API
                                if content.contains("fetch(") {
                                    if let Some(rel_path) = path.strip_prefix(base_dir).ok() {
                                        let file_path = rel_path.to_string_lossy().to_string();

                                        self.data_access_patterns.push(DataAccessPattern {
                                            pattern_type: "Fetch API".to_string(),
                                            description: "Modern Fetch API call".to_string(),
                                            files: vec![file_path],
                                            sync_feasibility: SyncFeasibility::High,
                                        });
                                    }
                                }

                                // Check for axios
                                if content.contains("axios.") || content.contains("axios(") {
                                    if let Some(rel_path) = path.strip_prefix(base_dir).ok() {
                                        let file_path = rel_path.to_string_lossy().to_string();

                                        self.data_access_patterns.push(DataAccessPattern {
                                            pattern_type: "Axios".to_string(),
                                            description: "Axios HTTP client".to_string(),
                                            files: vec![file_path],
                                            sync_feasibility: SyncFeasibility::High,
                                        });
                                    }
                                }

                                // Check for Rails HTTP requests
                                if content.contains("HTTParty") || content.contains("Net::HTTP") {
                                    if let Some(rel_path) = path.strip_prefix(base_dir).ok() {
                                        let file_path = rel_path.to_string_lossy().to_string();

                                        self.data_access_patterns.push(DataAccessPattern {
                                            pattern_type: "Rails HTTP".to_string(),
                                            description: "Ruby HTTP client".to_string(),
                                            files: vec![file_path],
                                            sync_feasibility: SyncFeasibility::Medium,
                                        });
                                    }
                                }

                                // Check for WebSockets
                                if content.contains("WebSocket") || content.contains("ActionCable") || content.contains("socket.io") {
                                    if let Some(rel_path) = path.strip_prefix(base_dir).ok() {
                                        let file_path = rel_path.to_string_lossy().to_string();

                                        self.data_access_patterns.push(DataAccessPattern {
                                            pattern_type: "WebSockets".to_string(),
                                            description: "Real-time WebSocket communication".to_string(),
                                            files: vec![file_path],
                                            sync_feasibility: SyncFeasibility::Low,
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
        // Look for patterns related to data updates
        for entry in WalkDir::new(base_dir)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file() {
                if let Some(extension) = path.extension() {
                    if let Some(ext) = extension.to_str() {
                        if ext == "js" || ext == "ts" || ext == "jsx" || ext == "tsx" || ext == "rb" {
                            if let Ok(content) = fs::read_to_string(path) {
                                // Check for form submissions
                                if content.contains("form") && (content.contains("submit") || content.contains("onSubmit")) {
                                    if let Some(rel_path) = path.strip_prefix(base_dir).ok() {
                                        let file_path = rel_path.to_string_lossy().to_string();

                                        self.data_update_patterns.push(DataUpdatePattern {
                                            pattern_type: "Form Submission".to_string(),
                                            description: "Traditional form submission".to_string(),
                                            files: vec![file_path],
                                        });
                                    }
                                }

                                // Check for AJAX updates
                                if (content.contains("$.ajax") || content.contains("$.post") || content.contains("$.put")) &&
                                   (content.contains("update") || content.contains("save") || content.contains("create")) {
                                    if let Some(rel_path) = path.strip_prefix(base_dir).ok() {
                                        let file_path = rel_path.to_string_lossy().to_string();

                                        self.data_update_patterns.push(DataUpdatePattern {
                                            pattern_type: "AJAX Update".to_string(),
                                            description: "Asynchronous data update via AJAX".to_string(),
                                            files: vec![file_path],
                                        });
                                    }
                                }

                                // Check for REST API updates
                                if (content.contains("PUT") || content.contains("POST") || content.contains("PATCH") || content.contains("DELETE")) &&
                                   (content.contains("api") || content.contains("endpoint") || content.contains("route")) {
                                    if let Some(rel_path) = path.strip_prefix(base_dir).ok() {
                                        let file_path = rel_path.to_string_lossy().to_string();

                                        self.data_update_patterns.push(DataUpdatePattern {
                                            pattern_type: "REST API".to_string(),
                                            description: "RESTful API data update".to_string(),
                                            files: vec![file_path],
                                        });
                                    }
                                }

                                // Check for real-time updates
                                if content.contains("real-time") || content.contains("realtime") || content.contains("live update") {
                                    if let Some(rel_path) = path.strip_prefix(base_dir).ok() {
                                        let file_path = rel_path.to_string_lossy().to_string();

                                        self.data_update_patterns.push(DataUpdatePattern {
                                            pattern_type: "Real-time Update".to_string(),
                                            description: "Live data updates".to_string(),
                                            files: vec![file_path],
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
        // Look for conflict resolution strategies
        for entry in WalkDir::new(base_dir)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file() {
                if let Some(extension) = path.extension() {
                    if let Some(ext) = extension.to_str() {
                        if ext == "js" || ext == "ts" || ext == "jsx" || ext == "tsx" || ext == "rb" {
                            if let Ok(content) = fs::read_to_string(path) {
                                // Check for timestamp-based conflict resolution
                                if content.contains("timestamp") && (content.contains("conflict") || content.contains("resolution")) {
                                    if let Some(rel_path) = path.strip_prefix(base_dir).ok() {
                                        let file_path = rel_path.to_string_lossy().to_string();

                                        self.conflict_resolution_strategies.push(ConflictResolutionStrategy {
                                            name: "Timestamp-based".to_string(),
                                            description: "Uses timestamps to resolve conflicts".to_string(),
                                            files: vec![file_path],
                                        });
                                    }
                                }

                                // Check for version-based conflict resolution
                                if content.contains("version") && (content.contains("conflict") || content.contains("resolution")) {
                                    if let Some(rel_path) = path.strip_prefix(base_dir).ok() {
                                        let file_path = rel_path.to_string_lossy().to_string();

                                        self.conflict_resolution_strategies.push(ConflictResolutionStrategy {
                                            name: "Version-based".to_string(),
                                            description: "Uses version numbers to resolve conflicts".to_string(),
                                            files: vec![file_path],
                                        });
                                    }
                                }

                                // Check for merge-based conflict resolution
                                if content.contains("merge") && (content.contains("conflict") || content.contains("resolution")) {
                                    if let Some(rel_path) = path.strip_prefix(base_dir).ok() {
                                        let file_path = rel_path.to_string_lossy().to_string();

                                        self.conflict_resolution_strategies.push(ConflictResolutionStrategy {
                                            name: "Merge-based".to_string(),
                                            description: "Merges conflicting changes".to_string(),
                                            files: vec![file_path],
                                        });
                                    }
                                }

                                // Check for custom conflict resolution
                                if content.contains("resolve") && content.contains("conflict") {
                                    if let Some(rel_path) = path.strip_prefix(base_dir).ok() {
                                        let file_path = rel_path.to_string_lossy().to_string();

                                        self.conflict_resolution_strategies.push(ConflictResolutionStrategy {
                                            name: "Custom Resolution".to_string(),
                                            description: "Custom conflict resolution logic".to_string(),
                                            files: vec![file_path],
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
        // Look for real-time update requirements
        for entry in WalkDir::new(base_dir)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file() {
                if let Some(extension) = path.extension() {
                    if let Some(ext) = extension.to_str() {
                        if ext == "js" || ext == "ts" || ext == "jsx" || ext == "tsx" || ext == "rb" {
                            if let Ok(content) = fs::read_to_string(path) {
                                // Check for chat or messaging features
                                if content.contains("chat") || content.contains("message") || content.contains("messaging") {
                                    if let Some(rel_path) = path.strip_prefix(base_dir).ok() {
                                        let file_path = rel_path.to_string_lossy().to_string();

                                        self.real_time_update_requirements.push(RealTimeUpdateRequirement {
                                            feature: "Chat/Messaging".to_string(),
                                            description: "Real-time chat or messaging functionality".to_string(),
                                            criticality: "High".to_string(),
                                            files: vec![file_path],
                                        });
                                    }
                                }

                                // Check for notifications
                                if content.contains("notification") || content.contains("alert") || content.contains("notify") {
                                    if let Some(rel_path) = path.strip_prefix(base_dir).ok() {
                                        let file_path = rel_path.to_string_lossy().to_string();

                                        self.real_time_update_requirements.push(RealTimeUpdateRequirement {
                                            feature: "Notifications".to_string(),
                                            description: "Real-time notification system".to_string(),
                                            criticality: "Medium".to_string(),
                                            files: vec![file_path],
                                        });
                                    }
                                }

                                // Check for collaborative editing
                                if content.contains("collaborative") || content.contains("real-time edit") || content.contains("simultaneous edit") {
                                    if let Some(rel_path) = path.strip_prefix(base_dir).ok() {
                                        let file_path = rel_path.to_string_lossy().to_string();

                                        self.real_time_update_requirements.push(RealTimeUpdateRequirement {
                                            feature: "Collaborative Editing".to_string(),
                                            description: "Real-time collaborative document editing".to_string(),
                                            criticality: "High".to_string(),
                                            files: vec![file_path],
                                        });
                                    }
                                }

                                // Check for live updates
                                if content.contains("live update") || content.contains("auto refresh") || content.contains("auto update") {
                                    if let Some(rel_path) = path.strip_prefix(base_dir).ok() {
                                        let file_path = rel_path.to_string_lossy().to_string();

                                        self.real_time_update_requirements.push(RealTimeUpdateRequirement {
                                            feature: "Live Updates".to_string(),
                                            description: "Automatic content refreshing".to_string(),
                                            criticality: "Medium".to_string(),
                                            files: vec![file_path],
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