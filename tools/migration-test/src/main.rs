use std::path::{Path, PathBuf};
use std::fs;
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

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

/// Factors that influence component prioritization
#[derive(Debug, Clone)]
pub struct PrioritizationFactors {
    /// Weight for component complexity (higher means complexity is more important)
    pub complexity_weight: f32,
    /// Weight for number of dependents (higher means more dependents is more important)
    pub dependents_weight: f32,
    /// Weight for number of dependencies (higher means more dependencies is more important)
    pub dependencies_weight: f32,
    /// Weight for leaf components (components with no dependencies)
    pub leaf_component_bonus: f32,
    /// Weight for root components (components with no dependents)
    pub root_component_bonus: f32,
}

impl Default for PrioritizationFactors {
    fn default() -> Self {
        Self {
            complexity_weight: 0.3,
            dependents_weight: 0.4,
            dependencies_weight: 0.2,
            leaf_component_bonus: 10.0,
            root_component_bonus: 5.0,
        }
    }
}

/// Component with priority score
#[derive(Debug, Clone)]
pub struct PrioritizedComponent {
    /// Component metadata
    pub component: ComponentMetadata,
    /// Priority score (higher means higher priority)
    pub priority_score: f32,
    /// Factors that contributed to the priority score
    pub score_factors: HashMap<String, f32>,
}

/// Component prioritizer that determines the order of component migration
pub struct ComponentPrioritizer {
    /// Factors that influence prioritization
    pub factors: PrioritizationFactors,
}

impl Default for ComponentPrioritizer {
    fn default() -> Self {
        Self {
            factors: PrioritizationFactors::default(),
        }
    }
}

impl ComponentPrioritizer {
    /// Create a new component prioritizer
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new component prioritizer with custom factors
    pub fn with_factors(factors: PrioritizationFactors) -> Self {
        Self { factors }
    }

    /// Prioritize components for migration
    pub fn prioritize(&self, tracker: &MigrationTracker) -> Vec<PrioritizedComponent> {
        let mut prioritized = Vec::new();
        
        // Only prioritize components that haven't been migrated yet
        let components_to_prioritize: Vec<_> = tracker.components.values()
            .filter(|c| matches!(c.status, MigrationStatus::NotStarted))
            .cloned()
            .collect();
        
        if components_to_prioritize.is_empty() {
            return prioritized;
        }

        // Calculate max values for normalization
        let max_complexity = components_to_prioritize.iter()
            .map(|c| c.complexity)
            .max()
            .unwrap_or(1) as f32;
        
        let max_dependents = components_to_prioritize.iter()
            .map(|c| c.dependents.len())
            .max()
            .unwrap_or(1) as f32;
        
        let max_dependencies = components_to_prioritize.iter()
            .map(|c| c.dependencies.len())
            .max()
            .unwrap_or(1) as f32;

        // Calculate priority scores
        for component in components_to_prioritize {
            let mut score_factors = HashMap::new();
            
            // Normalize complexity (0-1)
            let normalized_complexity = component.complexity as f32 / max_complexity;
            let complexity_score = normalized_complexity * self.factors.complexity_weight;
            score_factors.insert("complexity".to_string(), complexity_score);
            
            // Normalize dependents (0-1)
            let normalized_dependents = component.dependents.len() as f32 / max_dependents;
            let dependents_score = normalized_dependents * self.factors.dependents_weight;
            score_factors.insert("dependents".to_string(), dependents_score);
            
            // Normalize dependencies (0-1)
            let normalized_dependencies = component.dependencies.len() as f32 / max_dependencies;
            let dependencies_score = normalized_dependencies * self.factors.dependencies_weight;
            score_factors.insert("dependencies".to_string(), dependencies_score);
            
            // Add bonus for leaf components (no dependencies)
            let leaf_bonus = if component.dependencies.is_empty() {
                self.factors.leaf_component_bonus
            } else {
                0.0
            };
            score_factors.insert("leaf_bonus".to_string(), leaf_bonus);
            
            // Add bonus for root components (no dependents)
            let root_bonus = if component.dependents.is_empty() {
                self.factors.root_component_bonus
            } else {
                0.0
            };
            score_factors.insert("root_bonus".to_string(), root_bonus);
            
            // Calculate total score
            let priority_score = score_factors.values().sum();
            
            prioritized.push(PrioritizedComponent {
                component,
                priority_score,
                score_factors,
            });
        }
        
        // Sort by priority score (highest first)
        prioritized.sort_by(|a, b| b.priority_score.partial_cmp(&a.priority_score).unwrap());
        
        prioritized
    }

    /// Generate a topological sort of components based on dependencies
    pub fn topological_sort(&self, tracker: &MigrationTracker) -> Result<Vec<ComponentMetadata>, String> {
        let components = &tracker.components;
        
        // Build adjacency list
        let mut graph: HashMap<String, Vec<String>> = HashMap::new();
        for (id, component) in components {
            graph.insert(id.clone(), component.dependencies.clone());
        }
        
        // Find components with no dependencies
        let mut in_degree: HashMap<String, usize> = HashMap::new();
        for (id, _) in components {
            in_degree.insert(id.clone(), 0);
        }
        
        for (_, deps) in &graph {
            for dep in deps {
                if let Some(count) = in_degree.get_mut(dep) {
                    *count += 1;
                }
            }
        }
        
        // Queue for BFS
        let mut queue = std::collections::VecDeque::new();
        for (id, count) in &in_degree {
            if *count == 0 {
                queue.push_back(id.clone());
            }
        }
        
        // Perform topological sort
        let mut sorted = Vec::new();
        while let Some(id) = queue.pop_front() {
            if let Some(component) = components.get(&id) {
                sorted.push(component.clone());
            }
            
            if let Some(deps) = graph.get(&id) {
                for dep in deps {
                    if let Some(count) = in_degree.get_mut(dep) {
                        *count -= 1;
                        if *count == 0 {
                            queue.push_back(dep.clone());
                        }
                    }
                }
            }
        }
        
        // Check for cycles
        if sorted.len() != components.len() {
            return Err("Dependency cycle detected in components".to_string());
        }
        
        Ok(sorted)
    }

    /// Generate a migration plan based on component priorities and dependencies
    pub fn generate_migration_plan(&self, tracker: &MigrationTracker) -> Result<Vec<ComponentMetadata>, String> {
        // Try topological sort first
        match self.topological_sort(tracker) {
            Ok(sorted) => {
                // Filter out components that are already migrated
                let plan: Vec<_> = sorted.into_iter()
                    .filter(|c| matches!(c.status, MigrationStatus::NotStarted))
                    .collect();
                
                Ok(plan)
            },
            Err(_) => {
                // Fall back to priority-based sorting if there are cycles
                let prioritized = self.prioritize(tracker);
                let plan: Vec<_> = prioritized.into_iter()
                    .map(|p| p.component)
                    .collect();
                
                Ok(plan)
            }
        }
    }
}

/// Configuration for the migration manager
#[derive(Debug, Clone)]
pub struct MigrationConfig {
    /// Path to the migration tracker file
    pub tracker_file_path: PathBuf,
    /// Path to the output directory for migrated components
    pub output_dir: PathBuf,
    /// Path to the source directory for components to migrate
    pub source_dirs: Vec<PathBuf>,
    /// Whether to automatically detect dependencies
    pub auto_detect_dependencies: bool,
    /// Whether to skip components with errors
    pub skip_on_error: bool,
    /// Maximum number of components to migrate in one batch
    pub batch_size: usize,
    /// Prioritization factors
    pub prioritization_factors: PrioritizationFactors,
}

impl Default for MigrationConfig {
    fn default() -> Self {
        Self {
            tracker_file_path: PathBuf::from("migration_tracker.json"),
            output_dir: PathBuf::from("generated/leptos"),
            source_dirs: vec![],
            auto_detect_dependencies: true,
            skip_on_error: true,
            batch_size: 10,
            prioritization_factors: PrioritizationFactors::default(),
        }
    }
}

/// Migration manager that handles the incremental migration process
pub struct MigrationManager {
    /// Migration tracker
    pub tracker: MigrationTracker,
    /// Migration configuration
    pub config: MigrationConfig,
    /// Component prioritizer
    prioritizer: ComponentPrioritizer,
}

impl MigrationManager {
    /// Create a new migration manager
    pub fn new(config: MigrationConfig) -> Result<Self, Box<dyn std::error::Error>> {
        let tracker = MigrationTracker::load(&config.tracker_file_path)?;
        let prioritizer = ComponentPrioritizer::with_factors(config.prioritization_factors.clone());
        
        Ok(Self {
            tracker,
            config,
            prioritizer,
        })
    }
    
    /// Initialize the migration process by scanning for components
    pub fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Initializing migration process...");
        
        // Add some test components
        let react_component = ComponentMetadata {
            id: MigrationTracker::generate_component_id("ReactComponent", "src/components/ReactComponent.jsx", &ComponentType::React),
            name: "ReactComponent".to_string(),
            file_path: "src/components/ReactComponent.jsx".to_string(),
            component_type: ComponentType::React,
            status: MigrationStatus::NotStarted,
            complexity: 5,
            dependencies: Vec::new(),
            dependents: Vec::new(),
            last_updated: Utc::now(),
            migrated_path: None,
            notes: None,
        };
        
        let ember_component = ComponentMetadata {
            id: MigrationTracker::generate_component_id("EmberComponent", "src/components/EmberComponent.js", &ComponentType::Ember),
            name: "EmberComponent".to_string(),
            file_path: "src/components/EmberComponent.js".to_string(),
            component_type: ComponentType::Ember,
            status: MigrationStatus::NotStarted,
            complexity: 8,
            dependencies: Vec::new(),
            dependents: Vec::new(),
            last_updated: Utc::now(),
            migrated_path: None,
            notes: None,
        };
        
        let vue_component = ComponentMetadata {
            id: MigrationTracker::generate_component_id("VueComponent", "src/components/VueComponent.vue", &ComponentType::Vue),
            name: "VueComponent".to_string(),
            file_path: "src/components/VueComponent.vue".to_string(),
            component_type: ComponentType::Vue,
            status: MigrationStatus::NotStarted,
            complexity: 3,
            dependencies: Vec::new(),
            dependents: Vec::new(),
            last_updated: Utc::now(),
            migrated_path: None,
            notes: None,
        };
        
        let angular_component = ComponentMetadata {
            id: MigrationTracker::generate_component_id("AngularComponent", "src/components/AngularComponent.ts", &ComponentType::Angular),
            name: "AngularComponent".to_string(),
            file_path: "src/components/AngularComponent.ts".to_string(),
            component_type: ComponentType::Angular,
            status: MigrationStatus::NotStarted,
            complexity: 7,
            dependencies: Vec::new(),
            dependents: Vec::new(),
            last_updated: Utc::now(),
            migrated_path: None,
            notes: None,
        };
        
        // Add components to tracker
        self.tracker.add_component(react_component);
        self.tracker.add_component(ember_component);
        self.tracker.add_component(vue_component);
        self.tracker.add_component(angular_component);
        
        // Save the tracker
        self.tracker.save()?;
        
        println!("Migration initialized. Found {} components.", self.tracker.components.len());
        println!("{}", self.tracker.get_progress_string());
        
        Ok(())
    }
    
    /// Get the next batch of components to migrate
    pub fn get_next_batch(&self) -> Result<Vec<ComponentMetadata>, Box<dyn std::error::Error>> {
        // Generate migration plan
        let plan = self.prioritizer.generate_migration_plan(&self.tracker)?;
        
        // Take the first batch_size components
        let batch = plan.into_iter()
            .take(self.config.batch_size)
            .collect();
        
        Ok(batch)
    }
    
    /// Migrate a batch of components
    pub fn migrate_batch(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Get next batch
        let batch = self.get_next_batch()?;
        
        if batch.is_empty() {
            println!("No components left to migrate.");
            return Ok(());
        }
        
        println!("Migrating batch of {} components...", batch.len());
        
        // Simulate migration of each component
        for component in batch {
            println!("Migrating component: {} ({})", component.name, component.file_path);
            
            // Update status to in progress
            self.tracker.update_status(&component.id, MigrationStatus::InProgress)?;
            
            // Simulate migration (50% chance of success)
            let success = rand::random::<bool>();
            
            if success {
                // Update migrated path
                let migrated_path = format!("generated/leptos/components/{}/{}.rs", 
                    match component.component_type {
                        ComponentType::React => "react",
                        ComponentType::Ember => "ember",
                        ComponentType::Vue => "vue",
                        ComponentType::Angular => "angular",
                        ComponentType::Ruby => "ruby",
                        ComponentType::Other(ref s) => s,
                    },
                    component.name.to_lowercase()
                );
                
                self.tracker.update_migrated_path(&component.id, &migrated_path)?;
                
                // Update status to completed
                self.tracker.update_status(&component.id, MigrationStatus::Completed)?;
                println!("Successfully migrated component: {}", component.name);
            } else {
                // Update status to failed
                self.tracker.update_status(&component.id, MigrationStatus::Failed("Simulated failure".to_string()))?;
                println!("Failed to migrate component: {}", component.name);
            }
        }
        
        // Save tracker
        self.tracker.save()?;
        
        println!("Batch migration complete.");
        println!("{}", self.tracker.get_progress_string());
        
        Ok(())
    }
    
    /// Generate a migration report
    pub fn generate_report(&self) -> String {
        let mut report = String::new();
        
        report.push_str("# Migration Report\n\n");
        report.push_str(&format!("Generated: {}\n\n", Utc::now()));
        
        // Add progress summary
        report.push_str("## Progress Summary\n\n");
        report.push_str(&self.tracker.get_progress_string());
        report.push_str("\n");
        
        // Add completed components
        report.push_str("## Completed Components\n\n");
        let completed = self.tracker.get_components_by_status(&MigrationStatus::Completed);
        if completed.is_empty() {
            report.push_str("No components have been completed yet.\n\n");
        } else {
            report.push_str("| Component | Type | Original Path | Migrated Path |\n");
            report.push_str("|-----------|------|--------------|---------------|\n");
            
            for component in completed {
                let component_type = match component.component_type {
                    ComponentType::React => "React",
                    ComponentType::Ember => "Ember",
                    ComponentType::Vue => "Vue",
                    ComponentType::Angular => "Angular",
                    ComponentType::Ruby => "Ruby",
                    ComponentType::Other(ref s) => s,
                };
                
                let migrated_path = component.migrated_path.as_deref().unwrap_or("N/A");
                
                report.push_str(&format!(
                    "| {} | {} | {} | {} |\n",
                    component.name,
                    component_type,
                    component.file_path,
                    migrated_path
                ));
            }
            
            report.push_str("\n");
        }
        
        // Add failed components
        report.push_str("## Failed Components\n\n");
        let failed = self.tracker.components.values()
            .filter(|c| matches!(c.status, MigrationStatus::Failed(_)))
            .collect::<Vec<_>>();
        
        if failed.is_empty() {
            report.push_str("No components have failed migration.\n\n");
        } else {
            report.push_str("| Component | Type | Error |\n");
            report.push_str("|-----------|------|-------|\n");
            
            for component in failed {
                let component_type = match component.component_type {
                    ComponentType::React => "React",
                    ComponentType::Ember => "Ember",
                    ComponentType::Vue => "Vue",
                    ComponentType::Angular => "Angular",
                    ComponentType::Ruby => "Ruby",
                    ComponentType::Other(ref s) => s,
                };
                
                let error = match &component.status {
                    MigrationStatus::Failed(e) => e,
                    _ => "Unknown error",
                };
                
                report.push_str(&format!(
                    "| {} | {} | {} |\n",
                    component.name,
                    component_type,
                    error
                ));
            }
            
            report.push_str("\n");
        }
        
        report
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing Migration Functionality");
    
    // Create a test migration config
    let migration_config = MigrationConfig {
        tracker_file_path: PathBuf::from("migration_tracker.json"),
        output_dir: PathBuf::from("generated").join("leptos"),
        source_dirs: vec![],
        auto_detect_dependencies: false,
        skip_on_error: true,
        batch_size: 2,
        prioritization_factors: Default::default(),
    };
    
    // Initialize migration manager
    let mut migration_manager = MigrationManager::new(migration_config)?;
    
    // Initialize migration
    migration_manager.initialize()?;
    
    // Migrate first batch
    migration_manager.migrate_batch()?;
    
    // Generate report
    let report = migration_manager.generate_report();
    println!("\nMigration Report:\n{}", report);
    
    // Save report to file
    fs::create_dir_all("reports")?;
    fs::write("reports/migration_report.md", report)?;
    
    println!("Migration test completed successfully!");
    println!("Report saved to reports/migration_report.md");
    
    Ok(())
}
