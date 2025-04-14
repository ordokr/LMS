use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use chrono::{DateTime, Local, NaiveDate, NaiveDateTime};
use serde::{Serialize, Deserialize};

use crate::core::analysis_result::AnalysisResult;

/// Analyzer for trends in the codebase
pub struct TrendAnalyzer {
    /// Base directory for analysis
    base_dir: PathBuf,
    
    /// History file path
    history_file: PathBuf,
    
    /// Maximum number of history entries to keep
    max_history_entries: usize,
}

/// History entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    /// Timestamp of the analysis
    pub timestamp: DateTime<Local>,
    
    /// Project summary statistics
    pub total_files: usize,
    
    /// Lines of code
    pub lines_of_code: usize,
    
    /// Rust files
    pub rust_files: usize,
    
    /// Haskell files
    pub haskell_files: usize,
    
    /// Overall progress percentage
    pub overall_progress: f32,
    
    /// Models implementation percentage
    pub models_percentage: f32,
    
    /// API endpoints implementation percentage
    pub api_endpoints_percentage: f32,
    
    /// UI components implementation percentage
    pub ui_components_percentage: f32,
    
    /// Technical debt issues
    pub tech_debt_issues: usize,
}

/// Analysis history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisHistory {
    /// History entries
    pub entries: Vec<HistoryEntry>,
}

/// Trend metrics
#[derive(Debug, Clone)]
pub struct TrendMetrics {
    /// History entries
    pub history: AnalysisHistory,
    
    /// Changes since last analysis
    pub changes: HashMap<String, f32>,
    
    /// Weekly changes
    pub weekly_changes: HashMap<String, f32>,
    
    /// Monthly changes
    pub monthly_changes: HashMap<String, f32>,
}

impl TrendAnalyzer {
    /// Create a new trend analyzer
    pub fn new(base_dir: PathBuf) -> Self {
        let history_file = base_dir.join("docs").join("analysis_history.json");
        
        Self {
            base_dir,
            history_file,
            max_history_entries: 100,
        }
    }
    
    /// Load analysis history
    pub fn load_history(&self) -> Result<AnalysisHistory, String> {
        // Check if the history file exists
        if !self.history_file.exists() {
            return Ok(AnalysisHistory {
                entries: Vec::new(),
            });
        }
        
        // Read the history file
        let content = fs::read_to_string(&self.history_file)
            .map_err(|e| format!("Failed to read history file: {}", e))?;
        
        // Parse the history file
        let history: AnalysisHistory = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse history file: {}", e))?;
        
        Ok(history)
    }
    
    /// Save analysis history
    pub fn save_history(&self, history: &AnalysisHistory) -> Result<(), String> {
        // Ensure the directory exists
        if let Some(parent) = self.history_file.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create directory: {}", e))?;
        }
        
        // Serialize the history
        let content = serde_json::to_string_pretty(history)
            .map_err(|e| format!("Failed to serialize history: {}", e))?;
        
        // Write to file
        fs::write(&self.history_file, content)
            .map_err(|e| format!("Failed to write history file: {}", e))?;
        
        Ok(())
    }
    
    /// Add an entry to the history
    pub fn add_entry(&self, result: &AnalysisResult) -> Result<(), String> {
        // Load the history
        let mut history = self.load_history()?;
        
        // Create a new entry
        let entry = HistoryEntry {
            timestamp: Local::now(),
            total_files: result.summary.total_files,
            lines_of_code: result.summary.lines_of_code,
            rust_files: result.summary.rust_files,
            haskell_files: result.summary.haskell_files,
            overall_progress: result.overall_progress,
            models_percentage: result.models.implementation_percentage,
            api_endpoints_percentage: result.api_endpoints.implementation_percentage,
            ui_components_percentage: result.ui_components.implementation_percentage,
            tech_debt_issues: result.tech_debt_metrics.total_issues,
        };
        
        // Add the entry to the history
        history.entries.push(entry);
        
        // Limit the number of entries
        if history.entries.len() > self.max_history_entries {
            history.entries.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
            history.entries.truncate(self.max_history_entries);
        }
        
        // Save the history
        self.save_history(&history)?;
        
        Ok(())
    }
    
    /// Analyze trends
    pub fn analyze_trends(&self) -> Result<TrendMetrics, String> {
        // Load the history
        let history = self.load_history()?;
        
        // Sort entries by timestamp (newest first)
        let mut entries = history.entries.clone();
        entries.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        
        // Calculate changes since last analysis
        let mut changes = HashMap::new();
        
        if entries.len() >= 2 {
            let current = &entries[0];
            let previous = &entries[1];
            
            changes.insert("total_files".to_string(), (current.total_files as f32 - previous.total_files as f32) / previous.total_files as f32 * 100.0);
            changes.insert("lines_of_code".to_string(), (current.lines_of_code as f32 - previous.lines_of_code as f32) / previous.lines_of_code as f32 * 100.0);
            changes.insert("rust_files".to_string(), (current.rust_files as f32 - previous.rust_files as f32) / previous.rust_files as f32 * 100.0);
            changes.insert("haskell_files".to_string(), (current.haskell_files as f32 - previous.haskell_files as f32) / previous.haskell_files as f32 * 100.0);
            changes.insert("overall_progress".to_string(), current.overall_progress - previous.overall_progress);
            changes.insert("models_percentage".to_string(), current.models_percentage - previous.models_percentage);
            changes.insert("api_endpoints_percentage".to_string(), current.api_endpoints_percentage - previous.api_endpoints_percentage);
            changes.insert("ui_components_percentage".to_string(), current.ui_components_percentage - previous.ui_components_percentage);
            changes.insert("tech_debt_issues".to_string(), (current.tech_debt_issues as f32 - previous.tech_debt_issues as f32) / previous.tech_debt_issues as f32 * 100.0);
        }
        
        // Calculate weekly changes
        let mut weekly_changes = HashMap::new();
        
        if entries.len() >= 2 {
            let current = &entries[0];
            
            // Find the entry from a week ago
            let week_ago = current.timestamp.checked_sub_signed(chrono::Duration::days(7)).unwrap_or(current.timestamp);
            
            if let Some(week_ago_entry) = entries.iter().find(|e| e.timestamp <= week_ago) {
                weekly_changes.insert("total_files".to_string(), (current.total_files as f32 - week_ago_entry.total_files as f32) / week_ago_entry.total_files as f32 * 100.0);
                weekly_changes.insert("lines_of_code".to_string(), (current.lines_of_code as f32 - week_ago_entry.lines_of_code as f32) / week_ago_entry.lines_of_code as f32 * 100.0);
                weekly_changes.insert("rust_files".to_string(), (current.rust_files as f32 - week_ago_entry.rust_files as f32) / week_ago_entry.rust_files as f32 * 100.0);
                weekly_changes.insert("haskell_files".to_string(), (current.haskell_files as f32 - week_ago_entry.haskell_files as f32) / week_ago_entry.haskell_files as f32 * 100.0);
                weekly_changes.insert("overall_progress".to_string(), current.overall_progress - week_ago_entry.overall_progress);
                weekly_changes.insert("models_percentage".to_string(), current.models_percentage - week_ago_entry.models_percentage);
                weekly_changes.insert("api_endpoints_percentage".to_string(), current.api_endpoints_percentage - week_ago_entry.api_endpoints_percentage);
                weekly_changes.insert("ui_components_percentage".to_string(), current.ui_components_percentage - week_ago_entry.ui_components_percentage);
                weekly_changes.insert("tech_debt_issues".to_string(), (current.tech_debt_issues as f32 - week_ago_entry.tech_debt_issues as f32) / week_ago_entry.tech_debt_issues as f32 * 100.0);
            }
        }
        
        // Calculate monthly changes
        let mut monthly_changes = HashMap::new();
        
        if entries.len() >= 2 {
            let current = &entries[0];
            
            // Find the entry from a month ago
            let month_ago = current.timestamp.checked_sub_signed(chrono::Duration::days(30)).unwrap_or(current.timestamp);
            
            if let Some(month_ago_entry) = entries.iter().find(|e| e.timestamp <= month_ago) {
                monthly_changes.insert("total_files".to_string(), (current.total_files as f32 - month_ago_entry.total_files as f32) / month_ago_entry.total_files as f32 * 100.0);
                monthly_changes.insert("lines_of_code".to_string(), (current.lines_of_code as f32 - month_ago_entry.lines_of_code as f32) / month_ago_entry.lines_of_code as f32 * 100.0);
                monthly_changes.insert("rust_files".to_string(), (current.rust_files as f32 - month_ago_entry.rust_files as f32) / month_ago_entry.rust_files as f32 * 100.0);
                monthly_changes.insert("haskell_files".to_string(), (current.haskell_files as f32 - month_ago_entry.haskell_files as f32) / month_ago_entry.haskell_files as f32 * 100.0);
                monthly_changes.insert("overall_progress".to_string(), current.overall_progress - month_ago_entry.overall_progress);
                monthly_changes.insert("models_percentage".to_string(), current.models_percentage - month_ago_entry.models_percentage);
                monthly_changes.insert("api_endpoints_percentage".to_string(), current.api_endpoints_percentage - month_ago_entry.api_endpoints_percentage);
                monthly_changes.insert("ui_components_percentage".to_string(), current.ui_components_percentage - month_ago_entry.ui_components_percentage);
                monthly_changes.insert("tech_debt_issues".to_string(), (current.tech_debt_issues as f32 - month_ago_entry.tech_debt_issues as f32) / month_ago_entry.tech_debt_issues as f32 * 100.0);
            }
        }
        
        Ok(TrendMetrics {
            history: history.clone(),
            changes,
            weekly_changes,
            monthly_changes,
        })
    }
    
    /// Generate a trend report
    pub fn generate_report(&self) -> Result<String, String> {
        // Analyze trends
        let metrics = self.analyze_trends()?;
        
        // Generate the report
        let mut report = String::new();
        
        // Header
        report.push_str("# Trend Analysis Report\n\n");
        
        // Summary
        report.push_str("## Summary\n\n");
        report.push_str(&format!("**Total History Entries: {}**\n\n", metrics.history.entries.len()));
        
        if metrics.history.entries.is_empty() {
            report.push_str("No history entries found. Run the analyzer multiple times to generate trend data.\n\n");
            return Ok(report);
        }
        
        // Latest Analysis
        let latest = &metrics.history.entries[0];
        
        report.push_str("## Latest Analysis\n\n");
        report.push_str(&format!("**Date: {}**\n\n", latest.timestamp.format("%Y-%m-%d %H:%M:%S")));
        report.push_str(&format!("- Total Files: {}\n", latest.total_files));
        report.push_str(&format!("- Lines of Code: {}\n", latest.lines_of_code));
        report.push_str(&format!("- Rust Files: {}\n", latest.rust_files));
        report.push_str(&format!("- Haskell Files: {}\n", latest.haskell_files));
        report.push_str(&format!("- Overall Progress: {:.1}%\n", latest.overall_progress));
        report.push_str(&format!("- Models Implementation: {:.1}%\n", latest.models_percentage));
        report.push_str(&format!("- API Endpoints Implementation: {:.1}%\n", latest.api_endpoints_percentage));
        report.push_str(&format!("- UI Components Implementation: {:.1}%\n", latest.ui_components_percentage));
        report.push_str(&format!("- Technical Debt Issues: {}\n\n", latest.tech_debt_issues));
        
        // Changes Since Last Analysis
        report.push_str("## Changes Since Last Analysis\n\n");
        
        if !metrics.changes.is_empty() {
            report.push_str("| Metric | Change |\n");
            report.push_str("|--------|--------|\n");
            
            for (metric, change) in &metrics.changes {
                let formatted_metric = match metric.as_str() {
                    "total_files" => "Total Files",
                    "lines_of_code" => "Lines of Code",
                    "rust_files" => "Rust Files",
                    "haskell_files" => "Haskell Files",
                    "overall_progress" => "Overall Progress",
                    "models_percentage" => "Models Implementation",
                    "api_endpoints_percentage" => "API Endpoints Implementation",
                    "ui_components_percentage" => "UI Components Implementation",
                    "tech_debt_issues" => "Technical Debt Issues",
                    _ => metric,
                };
                
                let formatted_change = if metric.contains("percentage") || metric == "overall_progress" {
                    format!("{:+.1} percentage points", change)
                } else {
                    format!("{:+.1}%", change)
                };
                
                report.push_str(&format!("| {} | {} |\n", formatted_metric, formatted_change));
            }
        } else {
            report.push_str("No previous analysis found for comparison.\n");
        }
        
        report.push_str("\n");
        
        // Weekly Changes
        report.push_str("## Weekly Changes\n\n");
        
        if !metrics.weekly_changes.is_empty() {
            report.push_str("| Metric | Change |\n");
            report.push_str("|--------|--------|\n");
            
            for (metric, change) in &metrics.weekly_changes {
                let formatted_metric = match metric.as_str() {
                    "total_files" => "Total Files",
                    "lines_of_code" => "Lines of Code",
                    "rust_files" => "Rust Files",
                    "haskell_files" => "Haskell Files",
                    "overall_progress" => "Overall Progress",
                    "models_percentage" => "Models Implementation",
                    "api_endpoints_percentage" => "API Endpoints Implementation",
                    "ui_components_percentage" => "UI Components Implementation",
                    "tech_debt_issues" => "Technical Debt Issues",
                    _ => metric,
                };
                
                let formatted_change = if metric.contains("percentage") || metric == "overall_progress" {
                    format!("{:+.1} percentage points", change)
                } else {
                    format!("{:+.1}%", change)
                };
                
                report.push_str(&format!("| {} | {} |\n", formatted_metric, formatted_change));
            }
        } else {
            report.push_str("No analysis from a week ago found for comparison.\n");
        }
        
        report.push_str("\n");
        
        // Monthly Changes
        report.push_str("## Monthly Changes\n\n");
        
        if !metrics.monthly_changes.is_empty() {
            report.push_str("| Metric | Change |\n");
            report.push_str("|--------|--------|\n");
            
            for (metric, change) in &metrics.monthly_changes {
                let formatted_metric = match metric.as_str() {
                    "total_files" => "Total Files",
                    "lines_of_code" => "Lines of Code",
                    "rust_files" => "Rust Files",
                    "haskell_files" => "Haskell Files",
                    "overall_progress" => "Overall Progress",
                    "models_percentage" => "Models Implementation",
                    "api_endpoints_percentage" => "API Endpoints Implementation",
                    "ui_components_percentage" => "UI Components Implementation",
                    "tech_debt_issues" => "Technical Debt Issues",
                    _ => metric,
                };
                
                let formatted_change = if metric.contains("percentage") || metric == "overall_progress" {
                    format!("{:+.1} percentage points", change)
                } else {
                    format!("{:+.1}%", change)
                };
                
                report.push_str(&format!("| {} | {} |\n", formatted_metric, formatted_change));
            }
        } else {
            report.push_str("No analysis from a month ago found for comparison.\n");
        }
        
        report.push_str("\n");
        
        // History Chart
        report.push_str("## History Chart\n\n");
        report.push_str("```mermaid\ngantt\n");
        report.push_str("    title Project Progress Over Time\n");
        report.push_str("    dateFormat YYYY-MM-DD\n");
        report.push_str("    axisFormat %Y-%m-%d\n");
        
        // Sort entries by timestamp (oldest first)
        let mut entries = metrics.history.entries.clone();
        entries.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
        
        // Add milestones for each entry
        for entry in &entries {
            let date = entry.timestamp.format("%Y-%m-%d").to_string();
            report.push_str(&format!("    Progress {:.1}% : milestone, m{}, {}, 0d\n",
                entry.overall_progress,
                entry.timestamp.timestamp(),
                date));
        }
        
        report.push_str("```\n\n");
        
        // Progress Chart
        report.push_str("## Progress Chart\n\n");
        report.push_str("```mermaid\nxychart-beta\n");
        report.push_str("    title \"Implementation Progress Over Time\"\n");
        report.push_str("    x-axis [");
        
        for (i, entry) in entries.iter().enumerate() {
            let date = entry.timestamp.format("%m-%d").to_string();
            report.push_str(&format!("\"{}\"", date));
            
            if i < entries.len() - 1 {
                report.push_str(", ");
            }
        }
        
        report.push_str("]\n");
        
        // Overall Progress
        report.push_str("    y-axis \"Percentage (%)\"\n");
        report.push_str("    line [");
        
        for (i, entry) in entries.iter().enumerate() {
            report.push_str(&format!("{:.1}", entry.overall_progress));
            
            if i < entries.len() - 1 {
                report.push_str(", ");
            }
        }
        
        report.push_str("]\n");
        
        // Models Progress
        report.push_str("    line [");
        
        for (i, entry) in entries.iter().enumerate() {
            report.push_str(&format!("{:.1}", entry.models_percentage));
            
            if i < entries.len() - 1 {
                report.push_str(", ");
            }
        }
        
        report.push_str("]\n");
        
        // API Endpoints Progress
        report.push_str("    line [");
        
        for (i, entry) in entries.iter().enumerate() {
            report.push_str(&format!("{:.1}", entry.api_endpoints_percentage));
            
            if i < entries.len() - 1 {
                report.push_str(", ");
            }
        }
        
        report.push_str("]\n");
        
        // UI Components Progress
        report.push_str("    line [");
        
        for (i, entry) in entries.iter().enumerate() {
            report.push_str(&format!("{:.1}", entry.ui_components_percentage));
            
            if i < entries.len() - 1 {
                report.push_str(", ");
            }
        }
        
        report.push_str("]\n");
        
        report.push_str("```\n");
        
        Ok(report)
    }
}
