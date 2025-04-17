use std::collections::{HashMap, HashSet};
use std::path::Path;
use serde::{Deserialize, Serialize};
use anyhow::{Result, anyhow};

// We'll need to import the EntityMapper module
use crate::analyzers::modules::entity_mapper::{EntityMapper, EntityMapping};

/// Represents a naming or semantic conflict between entities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NamingConflict {
    /// First entity
    pub entity1: String,
    /// Second entity
    pub entity2: String,
    /// Conflict type (name, field, semantic)
    pub conflict_type: String,
    /// Conflict description
    pub description: String,
    /// Suggested resolution
    pub suggested_resolution: String,
}

/// Conflict Checker for detecting naming and semantic conflicts
pub struct ConflictChecker {
    /// Detected conflicts
    conflicts: Vec<NamingConflict>,
    /// Naming conflict threshold
    naming_conflict_threshold: f32,
    /// Semantic conflict threshold
    semantic_conflict_threshold: f32,
}

impl ConflictChecker {
    /// Create a new ConflictChecker with default settings
    pub fn new() -> Self {
        Self {
            conflicts: Vec::new(),
            naming_conflict_threshold: 0.8,
            semantic_conflict_threshold: 0.6,
        }
    }

    /// Create a new ConflictChecker with custom settings
    pub fn with_config(
        naming_conflict_threshold: f32,
        semantic_conflict_threshold: f32,
    ) -> Self {
        Self {
            conflicts: Vec::new(),
            naming_conflict_threshold,
            semantic_conflict_threshold,
        }
    }

    /// Get all detected conflicts
    pub fn get_conflicts(&self) -> &Vec<NamingConflict> {
        &self.conflicts
    }

    /// Detect conflicts between entities
    pub fn detect_conflicts(&mut self, entity_mapper: &EntityMapper) -> Result<()> {
        println!("Detecting conflicts between entities...");

        // Get all entities
        let canvas_entities = entity_mapper.get_entities_by_source("canvas");
        let discourse_entities = entity_mapper.get_entities_by_source("discourse");
        let ordo_entities = entity_mapper.get_entities_by_source("ordo");

        // Check for naming conflicts
        self.detect_naming_conflicts(canvas_entities, discourse_entities, ordo_entities)?;

        // Check for semantic conflicts
        self.detect_semantic_conflicts(entity_mapper)?;

        println!("Detected {} conflicts", self.conflicts.len());

        Ok(())
    }

    /// Detect naming conflicts between entities
    fn detect_naming_conflicts(
        &mut self,
        canvas_entities: Vec<String>,
        discourse_entities: Vec<String>,
        ordo_entities: Vec<String>,
    ) -> Result<()> {
        println!("Detecting naming conflicts...");

        // Check for naming conflicts between Canvas and Discourse entities
        for canvas_entity in &canvas_entities {
            for discourse_entity in &discourse_entities {
                let canvas_name = self.extract_entity_name(canvas_entity);
                let discourse_name = self.extract_entity_name(discourse_entity);

                if canvas_name == discourse_name {
                    // Same name but different sources - potential conflict
                    self.conflicts.push(NamingConflict {
                        entity1: canvas_entity.clone(),
                        entity2: discourse_entity.clone(),
                        conflict_type: "name".to_string(),
                        description: format!("Canvas and Discourse both have an entity named '{}'", canvas_name),
                        suggested_resolution: format!("Rename one of the entities or merge them if they represent the same concept"),
                    });
                }
            }
        }

        // Check for naming conflicts with Ordo entities
        for ordo_entity in &ordo_entities {
            let ordo_name = self.extract_entity_name(ordo_entity);

            // Check against Canvas entities
            for canvas_entity in &canvas_entities {
                let canvas_name = self.extract_entity_name(canvas_entity);

                if ordo_name == canvas_name {
                    // Check if this is a proper mapping or a conflict
                    if !self.is_mapped_entity(canvas_entity, ordo_entity) {
                        self.conflicts.push(NamingConflict {
                            entity1: canvas_entity.clone(),
                            entity2: ordo_entity.clone(),
                            conflict_type: "name".to_string(),
                            description: format!("Canvas and Ordo both have an entity named '{}' but they may represent different concepts", canvas_name),
                            suggested_resolution: format!("Verify if these entities represent the same concept and adjust naming if needed"),
                        });
                    }
                }
            }

            // Check against Discourse entities
            for discourse_entity in &discourse_entities {
                let discourse_name = self.extract_entity_name(discourse_entity);

                if ordo_name == discourse_name {
                    // Check if this is a proper mapping or a conflict
                    if !self.is_mapped_entity(discourse_entity, ordo_entity) {
                        self.conflicts.push(NamingConflict {
                            entity1: discourse_entity.clone(),
                            entity2: ordo_entity.clone(),
                            conflict_type: "name".to_string(),
                            description: format!("Discourse and Ordo both have an entity named '{}' but they may represent different concepts", discourse_name),
                            suggested_resolution: format!("Verify if these entities represent the same concept and adjust naming if needed"),
                        });
                    }
                }
            }
        }

        Ok(())
    }

    /// Detect semantic conflicts between entities
    fn detect_semantic_conflicts(&mut self, entity_mapper: &EntityMapper) -> Result<()> {
        println!("Detecting semantic conflicts...");

        // Get all mappings
        let mappings = entity_mapper.get_mappings();

        // Check for semantic conflicts in mappings
        for mapping in mappings {
            // Check if the mapping has a low confidence score
            if mapping.confidence < self.semantic_conflict_threshold {
                self.conflicts.push(NamingConflict {
                    entity1: mapping.source_entity.clone(),
                    entity2: mapping.target_entity.clone(),
                    conflict_type: "semantic".to_string(),
                    description: format!("Mapping between '{}' and '{}' has low confidence ({})",
                        mapping.source_entity, mapping.target_entity, mapping.confidence),
                    suggested_resolution: format!("Review the mapping and adjust entity definitions if needed"),
                });
            }

            // Check for field conflicts
            if let Some(field_conflicts) = self.detect_field_conflicts(entity_mapper, mapping) {
                for conflict in field_conflicts {
                    self.conflicts.push(conflict);
                }
            }
        }

        Ok(())
    }

    /// Detect field conflicts between mapped entities
    fn detect_field_conflicts(&self, entity_mapper: &EntityMapper, mapping: &EntityMapping) -> Option<Vec<NamingConflict>> {
        let mut conflicts = Vec::new();

        // Get the source and target entities
        let source_entity = entity_mapper.get_entity(&mapping.source_entity)?;
        let target_entity = entity_mapper.get_entity(&mapping.target_entity)?;

        // Check for fields with the same name but different types
        for (field_name, field_type) in &source_entity.fields {
            if let Some(target_field_type) = target_entity.fields.get(field_name) {
                if field_type != target_field_type {
                    conflicts.push(NamingConflict {
                        entity1: mapping.source_entity.clone(),
                        entity2: mapping.target_entity.clone(),
                        conflict_type: "field".to_string(),
                        description: format!("Field '{}' has type '{}' in '{}' but type '{}' in '{}'",
                            field_name, field_type, mapping.source_entity, target_field_type, mapping.target_entity),
                        suggested_resolution: format!("Harmonize field types or rename one of the fields"),
                    });
                }
            }
        }

        if conflicts.is_empty() {
            None
        } else {
            Some(conflicts)
        }
    }

    /// Extract the entity name from a fully qualified entity name
    fn extract_entity_name(&self, entity: &str) -> String {
        let parts: Vec<&str> = entity.split('.').collect();
        if parts.len() >= 2 {
            parts[1].to_string()
        } else {
            entity.to_string()
        }
    }

    /// Check if two entities are properly mapped
    fn is_mapped_entity(&self, _entity1: &str, _entity2: &str) -> bool {
        // This is a simplified check - in a real implementation, we would check against the actual mappings
        false
    }

    /// Generate a JSON report of conflicts
    pub fn generate_conflicts_report(&self) -> Result<String> {
        let report = serde_json::to_string_pretty(&self.conflicts)?;
        Ok(report)
    }

    /// Generate a Markdown report of conflicts
    pub fn generate_conflicts_markdown(&self) -> String {
        let mut markdown = String::new();

        markdown.push_str("# Conflict Analysis Report\n\n");

        // Summary statistics
        let total_conflicts = self.conflicts.len();
        let name_conflicts = self.conflicts.iter().filter(|c| c.conflict_type == "name").count();
        let field_conflicts = self.conflicts.iter().filter(|c| c.conflict_type == "field").count();
        let semantic_conflicts = self.conflicts.iter().filter(|c| c.conflict_type == "semantic").count();

        markdown.push_str("## Summary\n\n");
        markdown.push_str(&format!("- Total Conflicts: {}\n", total_conflicts));
        markdown.push_str(&format!("- Naming Conflicts: {}\n", name_conflicts));
        markdown.push_str(&format!("- Field Conflicts: {}\n", field_conflicts));
        markdown.push_str(&format!("- Semantic Conflicts: {}\n\n", semantic_conflicts));

        // Naming conflicts
        if name_conflicts > 0 {
            markdown.push_str("## Naming Conflicts\n\n");
            markdown.push_str("| Entity 1 | Entity 2 | Description | Suggested Resolution |\n");
            markdown.push_str("|---------|----------|-------------|----------------------|\n");

            for conflict in self.conflicts.iter().filter(|c| c.conflict_type == "name") {
                markdown.push_str(&format!(
                    "| {} | {} | {} | {} |\n",
                    conflict.entity1,
                    conflict.entity2,
                    conflict.description,
                    conflict.suggested_resolution
                ));
            }

            markdown.push_str("\n");
        }

        // Field conflicts
        if field_conflicts > 0 {
            markdown.push_str("## Field Conflicts\n\n");
            markdown.push_str("| Entity 1 | Entity 2 | Description | Suggested Resolution |\n");
            markdown.push_str("|---------|----------|-------------|----------------------|\n");

            for conflict in self.conflicts.iter().filter(|c| c.conflict_type == "field") {
                markdown.push_str(&format!(
                    "| {} | {} | {} | {} |\n",
                    conflict.entity1,
                    conflict.entity2,
                    conflict.description,
                    conflict.suggested_resolution
                ));
            }

            markdown.push_str("\n");
        }

        // Semantic conflicts
        if semantic_conflicts > 0 {
            markdown.push_str("## Semantic Conflicts\n\n");
            markdown.push_str("| Entity 1 | Entity 2 | Description | Suggested Resolution |\n");
            markdown.push_str("|---------|----------|-------------|----------------------|\n");

            for conflict in self.conflicts.iter().filter(|c| c.conflict_type == "semantic") {
                markdown.push_str(&format!(
                    "| {} | {} | {} | {} |\n",
                    conflict.entity1,
                    conflict.entity2,
                    conflict.description,
                    conflict.suggested_resolution
                ));
            }

            markdown.push_str("\n");
        }

        markdown
    }
}
