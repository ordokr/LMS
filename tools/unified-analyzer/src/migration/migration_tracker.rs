use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use chrono::{DateTime, Utc};

/// Status of a component's migration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum MigrationStatus {
    /// Component has not been migrated yet
    NotStarted,
    /// Component migration is in progress
    InProgress,
    /// Component has been migrated successfully
    Completed,
    /// Component migration failed
    Failed(String),
    /// Component is skipped (won't be migrated)
    Skipped(String),
}

/// Type of the component
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ComponentType {
    React,
    Ember,
    Vue,
    Angular,
    Ruby,
    Other(String),
}

/// Metadata about a component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentMetadata {
    /// Unique identifier for the component
    pub id: String,
    /// Name of the component
    pub name: String,
    /// Path to the component file
    pub file_path: String,
    /// Type of the component
    pub component_type: ComponentType,
    /// Current migration status
    pub status: MigrationStatus,
    /// Complexity score (higher means more complex)
    pub complexity: u32,
    /// Dependencies on other components
    pub dependencies: Vec<String>,
    /// Components that depend on this component
    pub dependents: Vec<String>,
    /// Last time this component was updated
    pub last_updated: DateTime<Utc>,
    /// Path to the migrated component (if any)
    pub migrated_path: Option<String>,
    /// Additional notes or metadata
    pub notes: Option<String>,
}

impl Default for ComponentMetadata {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            file_path: String::new(),
            component_type: ComponentType::Other("Unknown".to_string()),
            status: MigrationStatus::NotStarted,
            complexity: 0,
            dependencies: Vec::new(),
            dependents: Vec::new(),
            last_updated: Utc::now(),
            migrated_path: None,
            notes: None,
        }
    }
}

/// Migration tracker that keeps track of component migration status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationTracker {
    /// Map of component ID to component metadata
    pub components: HashMap<String, ComponentMetadata>,
    /// Path to the migration tracker file
    #[serde(skip)]
    file_path: Option<PathBuf>,
    /// Migration start time
    pub started_at: DateTime<Utc>,
    /// Last time the tracker was updated
    pub last_updated: DateTime<Utc>,
    /// Migration statistics
    pub stats: MigrationStats,
}

/// Migration statistics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MigrationStats {
    /// Total number of components
    pub total_components: usize,
    /// Number of components not started
    pub not_started: usize,
    /// Number of components in progress
    pub in_progress: usize,
    /// Number of components completed
    pub completed: usize,
    /// Number of components failed
    pub failed: usize,
    /// Number of components skipped
    pub skipped: usize,
    /// Percentage of migration completed
    pub completion_percentage: f32,
}

impl Default for MigrationTracker {
    fn default() -> Self {
        let now = Utc::now();
        Self {
            components: HashMap::new(),
            file_path: None,
            started_at: now,
            last_updated: now,
            stats: MigrationStats::default(),
        }
    }
}

impl MigrationTracker {
    /// Create a new migration tracker
    pub fn new() -> Self {
        Self::default()
    }

    /// Load a migration tracker from a file
    pub fn load(file_path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        if !file_path.exists() {
            let mut tracker = Self::default();
            tracker.file_path = Some(file_path.to_path_buf());
            return Ok(tracker);
        }

        let content = fs::read_to_string(file_path)?;
        let mut tracker: MigrationTracker = serde_json::from_str(&content)?;
        tracker.file_path = Some(file_path.to_path_buf());
        tracker.update_stats();
        Ok(tracker)
    }

    /// Save the migration tracker to a file
    pub fn save(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.last_updated = Utc::now();
        self.update_stats();

        if let Some(file_path) = &self.file_path {
            // Create parent directory if it doesn't exist
            if let Some(parent) = file_path.parent() {
                fs::create_dir_all(parent)?;
            }

            let content = serde_json::to_string_pretty(self)?;
            fs::write(file_path, content)?;
        }

        Ok(())
    }

    /// Set the file path for the migration tracker
    pub fn set_file_path(&mut self, file_path: &Path) {
        self.file_path = Some(file_path.to_path_buf());
    }

    /// Add a component to the migration tracker
    pub fn add_component(&mut self, component: ComponentMetadata) {
        self.components.insert(component.id.clone(), component);
        self.update_stats();
    }

    /// Update a component's status
    pub fn update_status(&mut self, component_id: &str, status: MigrationStatus) -> Result<(), String> {
        if let Some(component) = self.components.get_mut(component_id) {
            component.status = status;
            component.last_updated = Utc::now();
            self.update_stats();
            Ok(())
        } else {
            Err(format!("Component with ID {} not found", component_id))
        }
    }

    /// Update a component's migrated path
    pub fn update_migrated_path(&mut self, component_id: &str, migrated_path: &str) -> Result<(), String> {
        if let Some(component) = self.components.get_mut(component_id) {
            component.migrated_path = Some(migrated_path.to_string());
            component.last_updated = Utc::now();
            Ok(())
        } else {
            Err(format!("Component with ID {} not found", component_id))
        }
    }

    /// Get components by status
    pub fn get_components_by_status(&self, status: &MigrationStatus) -> Vec<&ComponentMetadata> {
        self.components
            .values()
            .filter(|c| &c.status == status)
            .collect()
    }

    /// Get components by type
    pub fn get_components_by_type(&self, component_type: &ComponentType) -> Vec<&ComponentMetadata> {
        self.components
            .values()
            .filter(|c| &c.component_type == component_type)
            .collect()
    }

    /// Update migration statistics
    pub fn update_stats(&mut self) {
        let total = self.components.len();
        let not_started = self.components.values().filter(|c| matches!(c.status, MigrationStatus::NotStarted)).count();
        let in_progress = self.components.values().filter(|c| matches!(c.status, MigrationStatus::InProgress)).count();
        let completed = self.components.values().filter(|c| matches!(c.status, MigrationStatus::Completed)).count();
        let failed = self.components.values().filter(|c| matches!(c.status, MigrationStatus::Failed(_))).count();
        let skipped = self.components.values().filter(|c| matches!(c.status, MigrationStatus::Skipped(_))).count();
        
        let completion_percentage = if total > 0 {
            (completed as f32 / total as f32) * 100.0
        } else {
            0.0
        };

        self.stats = MigrationStats {
            total_components: total,
            not_started,
            in_progress,
            completed,
            failed,
            skipped,
            completion_percentage,
        };
    }

    /// Get migration progress as a string
    pub fn get_progress_string(&self) -> String {
        format!(
            "Migration Progress: {:.1}% ({}/{} components)\n\
             - Not Started: {}\n\
             - In Progress: {}\n\
             - Completed: {}\n\
             - Failed: {}\n\
             - Skipped: {}\n",
            self.stats.completion_percentage,
            self.stats.completed,
            self.stats.total_components,
            self.stats.not_started,
            self.stats.in_progress,
            self.stats.completed,
            self.stats.failed,
            self.stats.skipped
        )
    }

    /// Generate a unique component ID
    pub fn generate_component_id(component_name: &str, file_path: &str, component_type: &ComponentType) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        component_name.hash(&mut hasher);
        file_path.hash(&mut hasher);
        format!("{:?}", component_type).hash(&mut hasher);
        
        format!("{:x}", hasher.finish())
    }
}
