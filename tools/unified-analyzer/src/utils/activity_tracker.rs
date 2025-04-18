use std::fs;
use std::path::{Path, PathBuf};
use chrono::{DateTime, Local, Utc};
use serde::{Serialize, Deserialize};
use anyhow::{Result, Context};
use std::collections::VecDeque;

/// Represents a single activity entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityEntry {
    /// Date of the activity in ISO format
    pub date: String,
    /// Component that was modified
    pub component: String,
    /// Description of the activity
    pub description: String,
    /// Developer who made the change
    pub developer: String,
    /// Timestamp for sorting
    #[serde(with = "chrono::serde::ts_seconds")]
    pub timestamp: DateTime<Utc>,
}

/// Activity tracker for the project
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityTracker {
    /// Maximum number of activities to keep
    max_entries: usize,
    /// List of recent activities
    activities: VecDeque<ActivityEntry>,
    /// Path to the activity log file
    log_path: PathBuf,
}

impl ActivityTracker {
    /// Create a new activity tracker
    pub fn new(base_dir: &Path, max_entries: usize) -> Self {
        let log_path = base_dir.join("docs").join("activity_log.json");
        
        Self {
            max_entries,
            activities: VecDeque::new(),
            log_path,
        }
    }
    
    /// Load activities from the log file
    pub fn load(&mut self) -> Result<()> {
        if !self.log_path.exists() {
            return Ok(());
        }
        
        let content = fs::read_to_string(&self.log_path)
            .context(format!("Failed to read activity log from {:?}", self.log_path))?;
            
        let activities: Vec<ActivityEntry> = serde_json::from_str(&content)
            .context("Failed to parse activity log JSON")?;
            
        self.activities = VecDeque::from(activities);
        
        Ok(())
    }
    
    /// Save activities to the log file
    pub fn save(&self) -> Result<()> {
        // Ensure the directory exists
        if let Some(parent) = self.log_path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)
                    .context(format!("Failed to create directory {:?}", parent))?;
            }
        }
        
        let json = serde_json::to_string_pretty(&self.activities.iter().collect::<Vec<_>>())
            .context("Failed to serialize activity log")?;
            
        fs::write(&self.log_path, json)
            .context(format!("Failed to write activity log to {:?}", self.log_path))?;
            
        Ok(())
    }
    
    /// Add a new activity entry
    pub fn add_activity(&mut self, component: &str, description: &str, developer: &str) -> Result<()> {
        // Load existing activities
        self.load()?;
        
        // Create a new entry
        let entry = ActivityEntry {
            date: Local::now().format("%Y-%m-%d").to_string(),
            component: component.to_string(),
            description: description.to_string(),
            developer: developer.to_string(),
            timestamp: Utc::now(),
        };
        
        // Add to the front of the queue
        self.activities.push_front(entry);
        
        // Trim if needed
        while self.activities.len() > self.max_entries {
            self.activities.pop_back();
        }
        
        // Save the updated log
        self.save()?;
        
        Ok(())
    }
    
    /// Get all activities
    pub fn get_activities(&mut self) -> Result<Vec<ActivityEntry>> {
        self.load()?;
        Ok(self.activities.iter().cloned().collect())
    }
    
    /// Get the most recent activities, up to a limit
    pub fn get_recent_activities(&mut self, limit: usize) -> Result<Vec<ActivityEntry>> {
        self.load()?;
        Ok(self.activities.iter().take(limit).cloned().collect())
    }
    
    /// Format activities as a markdown table
    pub fn format_as_markdown(&mut self, limit: Option<usize>) -> Result<String> {
        let activities = match limit {
            Some(limit) => self.get_recent_activities(limit)?,
            None => self.get_activities()?,
        };
        
        if activities.is_empty() {
            return Ok("No recent activity recorded.".to_string());
        }
        
        let mut markdown = String::from("| Date | Component | Description | Developer |\n");
        markdown.push_str("|------|-----------|-------------|------------|\n");
        
        for activity in activities {
            markdown.push_str(&format!(
                "| {} | {} | {} | {} |\n",
                activity.date,
                activity.component,
                activity.description,
                activity.developer
            ));
        }
        
        Ok(markdown)
    }
}
