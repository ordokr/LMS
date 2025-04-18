use super::migration_tracker::{ComponentMetadata, MigrationStatus, MigrationTracker};
use std::collections::{HashMap, HashSet, VecDeque};

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
        let mut queue: VecDeque<String> = VecDeque::new();
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
