use super::migration_tracker::{ComponentMetadata, ComponentType, MigrationStatus, MigrationTracker};
use super::component_prioritizer::{ComponentPrioritizer, PrioritizationFactors};
use crate::code_generators::{
    ReactToLeptosGenerator, EmberToLeptosGenerator, VueToLeptosGenerator, AngularToLeptosGenerator
};
use crate::analyzers::modules::{
    enhanced_react_analyzer::EnhancedReactAnalyzer,
    enhanced_ember_analyzer::EnhancedEmberAnalyzer,
    enhanced_vue_analyzer::EnhancedVueAnalyzer,
    enhanced_angular_analyzer::EnhancedAngularAnalyzer
};

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use chrono::Utc;

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

        // Scan for components in source directories
        for source_dir in &self.config.source_dirs {
            self.scan_components(source_dir)?;
        }

        // Detect dependencies if enabled
        if self.config.auto_detect_dependencies {
            self.detect_dependencies()?;
        }

        // Save the tracker
        self.tracker.save()?;

        println!("Migration initialized. Found {} components.", self.tracker.components.len());
        println!("{}", self.tracker.get_progress_string());

        Ok(())
    }

    /// Scan for components in a directory
    fn scan_components(&mut self, dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
        println!("Scanning for components in {:?}...", dir);

        // Create maps to store component names to IDs for dependency detection
        let mut react_component_map = HashMap::new();
        let mut ember_component_map = HashMap::new();
        let mut vue_component_map = HashMap::new();
        let mut angular_component_map = HashMap::new();

        // Scan for React components
        let mut react_analyzer = EnhancedReactAnalyzer::new();
        if let Ok(()) = react_analyzer.analyze_directory(dir) {
            for (path, component) in &react_analyzer.components {
                let id = MigrationTracker::generate_component_id(&component.name, path, &ComponentType::React);

                // Store component name to ID mapping for dependency detection
                react_component_map.insert(component.name.clone(), id.clone());

                // Skip if component already exists
                if self.tracker.components.contains_key(&id) {
                    continue;
                }

                let complexity = self.calculate_complexity(component.name.as_str(), ComponentType::React, path);

                let metadata = ComponentMetadata {
                    id: id.clone(),
                    name: component.name.clone(),
                    file_path: path.clone(),
                    component_type: ComponentType::React,
                    status: MigrationStatus::NotStarted,
                    complexity,
                    dependencies: Vec::new(), // Will be filled later
                    dependents: Vec::new(),   // Will be filled later
                    last_updated: Utc::now(),
                    migrated_path: None,
                    notes: None,
                };

                self.tracker.add_component(metadata);
            }
        }

        // Scan for Ember components
        let mut ember_analyzer = EnhancedEmberAnalyzer::new();
        if let Ok(()) = ember_analyzer.analyze_directory(dir) {
            for (path, component) in &ember_analyzer.components {
                let id = MigrationTracker::generate_component_id(&component.name, path, &ComponentType::Ember);

                // Store component name to ID mapping for dependency detection
                ember_component_map.insert(component.name.clone(), id.clone());

                // Skip if component already exists
                if self.tracker.components.contains_key(&id) {
                    continue;
                }

                let complexity = self.calculate_complexity(component.name.as_str(), ComponentType::Ember, path);

                let metadata = ComponentMetadata {
                    id: id.clone(),
                    name: component.name.clone(),
                    file_path: path.clone(),
                    component_type: ComponentType::Ember,
                    status: MigrationStatus::NotStarted,
                    complexity,
                    dependencies: Vec::new(),
                    dependents: Vec::new(),
                    last_updated: Utc::now(),
                    migrated_path: None,
                    notes: None,
                };

                self.tracker.add_component(metadata);
            }
        }

        // Scan for Vue components
        let mut vue_analyzer = EnhancedVueAnalyzer::new();
        if let Ok(()) = vue_analyzer.analyze_directory(dir) {
            for (path, component) in &vue_analyzer.components {
                let id = MigrationTracker::generate_component_id(&component.name, path, &ComponentType::Vue);

                // Store component name to ID mapping for dependency detection
                vue_component_map.insert(component.name.clone(), id.clone());

                // Skip if component already exists
                if self.tracker.components.contains_key(&id) {
                    continue;
                }

                let complexity = self.calculate_complexity(component.name.as_str(), ComponentType::Vue, path);

                let metadata = ComponentMetadata {
                    id: id.clone(),
                    name: component.name.clone(),
                    file_path: path.clone(),
                    component_type: ComponentType::Vue,
                    status: MigrationStatus::NotStarted,
                    complexity,
                    dependencies: Vec::new(),
                    dependents: Vec::new(),
                    last_updated: Utc::now(),
                    migrated_path: None,
                    notes: None,
                };

                self.tracker.add_component(metadata);
            }
        }

        // Scan for Angular components
        let mut angular_analyzer = EnhancedAngularAnalyzer::new();
        if let Ok(()) = angular_analyzer.analyze_directory(dir) {
            for (path, component) in &angular_analyzer.components {
                let id = MigrationTracker::generate_component_id(&component.name, path, &ComponentType::Angular);

                // Store component name to ID mapping for dependency detection
                angular_component_map.insert(component.name.clone(), id.clone());

                // Skip if component already exists
                if self.tracker.components.contains_key(&id) {
                    continue;
                }

                let complexity = self.calculate_complexity(component.name.as_str(), ComponentType::Angular, path);

                let metadata = ComponentMetadata {
                    id: id.clone(),
                    name: component.name.clone(),
                    file_path: path.clone(),
                    component_type: ComponentType::Angular,
                    status: MigrationStatus::NotStarted,
                    complexity,
                    dependencies: Vec::new(),
                    dependents: Vec::new(),
                    last_updated: Utc::now(),
                    migrated_path: None,
                    notes: None,
                };

                self.tracker.add_component(metadata);
            }
        }

        // Detect dependencies for React components
        for (path, component) in &react_analyzer.components {
            let id = MigrationTracker::generate_component_id(&component.name, path, &ComponentType::React);

            // Get the component's imports/dependencies
            let mut dependencies = Vec::new();

            // Check for imports in the component's code
            if let Ok(content) = fs::read_to_string(path) {
                // Check for React component imports
                for (other_name, other_id) in &react_component_map {
                    if other_name != &component.name && (
                        content.contains(&format!("import {} from", other_name)) ||
                        content.contains(&format!("import {{ {} }} from", other_name)) ||
                        content.contains(&format!("<{}", other_name)) ||
                        content.contains(&format!("<{} ", other_name))
                    ) {
                        dependencies.push(other_id.clone());
                    }
                }
            }

            // Update component dependencies
            if let Some(comp) = self.tracker.components.get_mut(&id) {
                comp.dependencies = dependencies.clone();
            }

            // Update dependents for each dependency
            for dep_id in dependencies {
                if let Some(dep) = self.tracker.components.get_mut(&dep_id) {
                    if !dep.dependents.contains(&id) {
                        dep.dependents.push(id.clone());
                    }
                }
            }
        }

        // Detect dependencies for Ember components
        for (path, component) in &ember_analyzer.components {
            let id = MigrationTracker::generate_component_id(&component.name, path, &ComponentType::Ember);

            // Get the component's imports/dependencies
            let mut dependencies = Vec::new();

            // Check for imports in the component's code
            if let Ok(content) = fs::read_to_string(path) {
                // Check for Ember component imports
                for (other_name, other_id) in &ember_component_map {
                    if other_name != &component.name && (
                        content.contains(&format!("import {} from", other_name)) ||
                        content.contains(&format!("import {{ {} }} from", other_name)) ||
                        content.contains(&format!("{{#{}", other_name)) ||
                        content.contains(&format!("{{component '{}'", other_name.to_lowercase()))
                    ) {
                        dependencies.push(other_id.clone());
                    }
                }
            }

            // Update component dependencies
            if let Some(comp) = self.tracker.components.get_mut(&id) {
                comp.dependencies = dependencies.clone();
            }

            // Update dependents for each dependency
            for dep_id in dependencies {
                if let Some(dep) = self.tracker.components.get_mut(&dep_id) {
                    if !dep.dependents.contains(&id) {
                        dep.dependents.push(id.clone());
                    }
                }
            }
        }

        // Detect dependencies for Vue components
        for (path, component) in &vue_analyzer.components {
            let id = MigrationTracker::generate_component_id(&component.name, path, &ComponentType::Vue);

            // Get the component's imports/dependencies
            let mut dependencies = Vec::new();

            // Check for imports in the component's code
            if let Ok(content) = fs::read_to_string(path) {
                // Check for Vue component imports
                for (other_name, other_id) in &vue_component_map {
                    if other_name != &component.name && (
                        content.contains(&format!("import {} from", other_name)) ||
                        content.contains(&format!("import {{ {} }} from", other_name)) ||
                        content.contains(&format!("<{}", other_name)) ||
                        content.contains(&format!("<{} ", other_name)) ||
                        content.contains(&format!("components: {{ {}", other_name))
                    ) {
                        dependencies.push(other_id.clone());
                    }
                }
            }

            // Update component dependencies
            if let Some(comp) = self.tracker.components.get_mut(&id) {
                comp.dependencies = dependencies.clone();
            }

            // Update dependents for each dependency
            for dep_id in dependencies {
                if let Some(dep) = self.tracker.components.get_mut(&dep_id) {
                    if !dep.dependents.contains(&id) {
                        dep.dependents.push(id.clone());
                    }
                }
            }
        }

        // Detect dependencies for Angular components
        for (path, component) in &angular_analyzer.components {
            let id = MigrationTracker::generate_component_id(&component.name, path, &ComponentType::Angular);

            // Get the component's imports/dependencies
            let mut dependencies = Vec::new();

            // Check for imports in the component's code
            if let Ok(content) = fs::read_to_string(path) {
                // Check for Angular component imports
                for (other_name, other_id) in &angular_component_map {
                    if other_name != &component.name && (
                        content.contains(&format!("import {{ {} }} from", other_name)) ||
                        content.contains(&format!("<{}", other_name.to_lowercase())) ||
                        content.contains(&format!("<{} ", other_name.to_lowercase()))
                    ) {
                        dependencies.push(other_id.clone());
                    }
                }
            }

            // Update component dependencies
            if let Some(comp) = self.tracker.components.get_mut(&id) {
                comp.dependencies = dependencies.clone();
            }

            // Update dependents for each dependency
            for dep_id in dependencies {
                if let Some(dep) = self.tracker.components.get_mut(&dep_id) {
                    if !dep.dependents.contains(&id) {
                        dep.dependents.push(id.clone());
                    }
                }
            }
        }

        Ok(())
    }

    /// Calculate component complexity based on various factors
    fn calculate_complexity(&self, name: &str, component_type: ComponentType, file_path: &str) -> u32 {
        // Read the file content
        let content = match fs::read_to_string(file_path) {
            Ok(content) => content,
            Err(_) => return 1, // Default complexity if file can't be read
        };

        // Base complexity
        let mut complexity = 1;

        // Add complexity based on file size
        complexity += (content.len() / 1000) as u32;

        // Add complexity based on number of lines
        let line_count = content.lines().count();
        complexity += (line_count / 50) as u32;

        // Add complexity based on number of methods/functions
        let method_count = match component_type {
            ComponentType::React => content.matches("function").count() + content.matches("=>").count(),
            ComponentType::Ember => content.matches("actions:").count() * 3 + content.matches("function").count(),
            ComponentType::Vue => content.matches("methods:").count() * 3 + content.matches("function").count(),
            ComponentType::Angular => content.matches("ngOn").count() * 2 + content.matches("function").count(),
            _ => content.matches("function").count(),
        };
        complexity += method_count as u32;

        // Add complexity based on state management
        let state_complexity = match component_type {
            ComponentType::React => content.matches("useState").count() * 2 + content.matches("useReducer").count() * 3,
            ComponentType::Vue => content.matches("data:").count() * 2 + content.matches("computed:").count() * 2,
            ComponentType::Angular => content.matches("@Input").count() + content.matches("@Output").count(),
            _ => 0,
        };
        complexity += state_complexity as u32;

        // Add complexity for conditional rendering
        let conditional_complexity = content.matches("if").count() + content.matches("? :").count() * 2;
        complexity += (conditional_complexity / 5) as u32;

        // Cap complexity at 100
        complexity.min(100)
    }

    /// Detect dependencies between components
    fn detect_dependencies(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Detecting component dependencies...");

        // Create a map of component names to IDs for quick lookup
        let mut name_to_id = HashMap::new();
        for (id, component) in &self.tracker.components {
            name_to_id.insert(component.name.clone(), id.clone());
        }

        // Clear existing dependencies
        for component in self.tracker.components.values_mut() {
            component.dependencies.clear();
            component.dependents.clear();
        }

        // Detect dependencies by scanning file content
        for (id, component) in self.tracker.components.clone().iter() {
            let content = match fs::read_to_string(&component.file_path) {
                Ok(content) => content,
                Err(_) => continue,
            };

            // Check for imports/references to other components
            for (other_name, other_id) in &name_to_id {
                // Skip self-references
                if other_id == id {
                    continue;
                }

                // Check if this component imports or uses the other component
                if content.contains(&format!("import {} from", other_name)) ||
                   content.contains(&format!("import {{ {} }} from", other_name)) ||
                   content.contains(&format!("<{}", other_name)) ||
                   content.contains(&format!("components: {{ {}", other_name)) {

                    // Add dependency
                    if let Some(comp) = self.tracker.components.get_mut(id) {
                        if !comp.dependencies.contains(other_id) {
                            comp.dependencies.push(other_id.clone());
                        }
                    }

                    // Add dependent
                    if let Some(other_comp) = self.tracker.components.get_mut(other_id) {
                        if !other_comp.dependents.contains(id) {
                            other_comp.dependents.push(id.clone());
                        }
                    }
                }
            }
        }

        println!("Dependency detection complete.");
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

    /// Migrate a single component
    pub fn migrate_component(&mut self, component_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Get the component
        let component = match self.tracker.components.get(component_id) {
            Some(component) => component.clone(),
            None => return Err(format!("Component with ID {} not found", component_id).into()),
        };

        println!("Migrating component: {} ({})", component.name, component.file_path);

        // Update status to in progress
        self.tracker.update_status(component_id, MigrationStatus::InProgress)?;
        self.tracker.save()?;

        // Create output directory based on component type
        let component_type_dir = match component.component_type {
            ComponentType::React => "react",
            ComponentType::Ember => "ember",
            ComponentType::Vue => "vue",
            ComponentType::Angular => "angular",
            ComponentType::Ruby => "ruby",
            ComponentType::Other(ref s) => s,
        };

        let output_dir = self.config.output_dir.join("components").join(component_type_dir);
        fs::create_dir_all(&output_dir)?;

        // Migrate the component based on its type
        let result = match component.component_type {
            ComponentType::React => {
                // Initialize analyzer and generator
                let mut react_analyzer = EnhancedReactAnalyzer::new();
                let generator = ReactToLeptosGenerator::new(&output_dir);

                // Analyze the component file
                let component_path = Path::new(&component.file_path);
                if component_path.exists() {
                    // If it's a directory, analyze the directory
                    if component_path.is_dir() {
                        react_analyzer.analyze_directory(component_path)?;
                    } else {
                        // If it's a file, analyze the file
                        react_analyzer.analyze_file(component_path)?;
                    }

                    // Find the component in the analyzer
                    let mut found = false;
                    for (path, react_component) in &react_analyzer.components {
                        if path == &component.file_path {
                            // Generate the component
                            generator.generate_component(react_component)?;
                            found = true;
                            break;
                        }
                    }

                    if !found {
                        return Err(format!("React component not found in analyzer: {}", component.file_path).into());
                    }

                    Ok(())
                } else {
                    Err(format!("Component file not found: {}", component.file_path).into())
                }
            },
            ComponentType::Ember => {
                // Initialize analyzer and generator
                let mut ember_analyzer = EnhancedEmberAnalyzer::new();
                let generator = EmberToLeptosGenerator::new(&output_dir);

                // Analyze the component file
                let component_path = Path::new(&component.file_path);
                if component_path.exists() {
                    // If it's a directory, analyze the directory
                    if component_path.is_dir() {
                        ember_analyzer.analyze_directory(component_path)?;
                    } else {
                        // If it's a file, analyze the file
                        ember_analyzer.analyze_file(component_path)?;
                    }

                    // Find the component in the analyzer
                    let mut found = false;
                    for (path, ember_component) in &ember_analyzer.components {
                        if path == &component.file_path {
                            // Generate the component
                            generator.generate_component(ember_component)?;
                            found = true;
                            break;
                        }
                    }

                    if !found {
                        return Err(format!("Ember component not found in analyzer: {}", component.file_path).into());
                    }

                    Ok(())
                } else {
                    Err(format!("Component file not found: {}", component.file_path).into())
                }
            },
            ComponentType::Vue => {
                // Initialize analyzer and generator
                let mut vue_analyzer = EnhancedVueAnalyzer::new();
                let generator = VueToLeptosGenerator::new(&output_dir);

                // Analyze the component file
                let component_path = Path::new(&component.file_path);
                if component_path.exists() {
                    // If it's a directory, analyze the directory
                    if component_path.is_dir() {
                        vue_analyzer.analyze_directory(component_path)?;
                    } else {
                        // If it's a file, analyze the file
                        vue_analyzer.analyze_file(component_path)?;
                    }

                    // Find the component in the analyzer
                    let mut found = false;
                    for (path, vue_component) in &vue_analyzer.components {
                        if path == &component.file_path {
                            // Generate the component
                            generator.generate_component(vue_component)?;
                            found = true;
                            break;
                        }
                    }

                    if !found {
                        return Err(format!("Vue component not found in analyzer: {}", component.file_path).into());
                    }

                    Ok(())
                } else {
                    Err(format!("Component file not found: {}", component.file_path).into())
                }
            },
            ComponentType::Angular => {
                // Initialize analyzer and generator
                let mut angular_analyzer = EnhancedAngularAnalyzer::new();
                let generator = AngularToLeptosGenerator::new(&output_dir);

                // Analyze the component file
                let component_path = Path::new(&component.file_path);
                if component_path.exists() {
                    // If it's a directory, analyze the directory
                    if component_path.is_dir() {
                        angular_analyzer.analyze_directory(component_path)?;
                    } else {
                        // If it's a file, analyze the file
                        angular_analyzer.analyze_file(component_path)?;
                    }

                    // Find the component in the analyzer
                    let mut found = false;
                    for (path, angular_component) in &angular_analyzer.components {
                        if path == &component.file_path {
                            // Generate the component
                            generator.generate_component(angular_component)?;
                            found = true;
                            break;
                        }
                    }

                    if !found {
                        return Err(format!("Angular component not found in analyzer: {}", component.file_path).into());
                    }

                    Ok(())
                } else {
                    Err(format!("Component file not found: {}", component.file_path).into())
                }
            },
            _ => Err(format!("Unsupported component type: {:?}", component.component_type).into()),
        };

        // Update status based on result
        match result {
            Ok(()) => {
                // Update migrated path
                let file_name = Path::new(&component.file_path).file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("unknown");

                let migrated_path = output_dir.join(format!("{}.rs", to_snake_case(file_name)));
                self.tracker.update_migrated_path(component_id, &migrated_path.to_string_lossy())?;

                // Update status to completed
                self.tracker.update_status(component_id, MigrationStatus::Completed)?;
                println!("Successfully migrated component: {}", component.name);
            },
            Err(e) => {
                if self.config.skip_on_error {
                    // Update status to skipped
                    self.tracker.update_status(component_id, MigrationStatus::Skipped(e.to_string()))?;
                    println!("Skipped component due to error: {}", e);
                } else {
                    // Update status to failed
                    self.tracker.update_status(component_id, MigrationStatus::Failed(e.to_string()))?;
                    println!("Failed to migrate component: {}", e);
                    return Err(e);
                }
            }
        }

        // Save tracker
        self.tracker.save()?;

        Ok(())
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

        // Migrate each component
        for component in batch {
            if let Err(e) = self.migrate_component(&component.id) {
                println!("Error migrating component {}: {}", component.name, e);
                if !self.config.skip_on_error {
                    return Err(e);
                }
            }
        }

        println!("Batch migration complete.");
        println!("{}", self.tracker.get_progress_string());

        Ok(())
    }

    /// Run the migration process until completion
    pub fn run_migration(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Starting migration process...");

        // Initialize if not already done
        if self.tracker.components.is_empty() {
            self.initialize()?;
        }

        // Keep migrating batches until done
        loop {
            let not_started = self.tracker.get_components_by_status(&MigrationStatus::NotStarted);

            if not_started.is_empty() {
                break;
            }

            self.migrate_batch()?;
        }

        println!("Migration process complete!");
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
            report.push_str("|-----------|------|--------------|--------------|\n");

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

        // Add dependency graph
        report.push_str("## Dependency Graph\n\n");
        report.push_str("```mermaid\ngraph TD;\n");

        for (id, component) in &self.tracker.components {
            for dep_id in &component.dependencies {
                if let Some(dep) = self.tracker.components.get(dep_id) {
                    report.push_str(&format!(
                        "    {}[{}] --> {}[{}];\n",
                        id, component.name,
                        dep_id, dep.name
                    ));
                }
            }
        }

        report.push_str("```\n\n");

        report
    }
}

/// Helper function to convert PascalCase to snake_case
fn to_snake_case(pascal_case: &str) -> String {
    let mut snake_case = String::new();

    for (i, c) in pascal_case.chars().enumerate() {
        if i > 0 && c.is_uppercase() {
            snake_case.push('_');
        }
        snake_case.push(c.to_lowercase().next().unwrap());
    }

    snake_case
}
