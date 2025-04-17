use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::fs;
use serde::{Deserialize, Serialize};
use anyhow::{Result, anyhow};

use crate::analyzers::modules::entity_mapper::EntityMapper;
use crate::analyzers::modules::feature_detector::FeatureDetector;
use crate::analyzers::modules::code_quality_scorer::CodeQualityScorer;
use crate::analyzers::modules::conflict_checker::ConflictChecker;
use crate::analyzers::modules::integration_tracker::IntegrationTracker;

/// Represents a development recommendation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recommendation {
    /// Recommendation ID
    pub id: String,
    /// Recommendation title
    pub title: String,
    /// Recommendation description
    pub description: String,
    /// Priority (1-5, with 5 being highest)
    pub priority: u8,
    /// Estimated effort (days)
    pub effort: f32,
    /// Related entities
    pub related_entities: Vec<String>,
    /// Related features
    pub related_features: Vec<String>,
    /// Implementation steps
    pub steps: Vec<String>,
}

/// Recommendation System for generating development recommendations
pub struct RecommendationSystem {
    /// Generated recommendations
    recommendations: Vec<Recommendation>,
    /// Maximum number of recommendations
    max_recommendations: usize,
    /// Priority thresholds
    high_priority_threshold: u8,
    medium_priority_threshold: u8,
}

impl RecommendationSystem {
    /// Create a new RecommendationSystem with default settings
    pub fn new() -> Self {
        Self {
            recommendations: Vec::new(),
            max_recommendations: 20,
            high_priority_threshold: 4,
            medium_priority_threshold: 2,
        }
    }

    /// Create a new RecommendationSystem with custom settings
    pub fn with_config(
        max_recommendations: usize,
        high_priority_threshold: u8,
        medium_priority_threshold: u8,
    ) -> Self {
        Self {
            recommendations: Vec::new(),
            max_recommendations,
            high_priority_threshold,
            medium_priority_threshold,
        }
    }

    /// Get all recommendations
    pub fn get_recommendations(&self) -> &Vec<Recommendation> {
        &self.recommendations
    }

    /// Generate recommendations
    pub fn generate_recommendations(
        &mut self,
        entity_mapper: &EntityMapper,
        feature_detector: &FeatureDetector,
        code_quality_scorer: &CodeQualityScorer,
        conflict_checker: &ConflictChecker,
        integration_tracker: &IntegrationTracker
    ) -> Result<()> {
        println!("Generating recommendations...");

        // Generate entity-based recommendations
        self.generate_entity_recommendations(entity_mapper)?;

        // Generate feature-based recommendations
        self.generate_feature_recommendations(feature_detector)?;

        // Generate code quality recommendations
        self.generate_code_quality_recommendations(code_quality_scorer)?;

        // Generate conflict resolution recommendations
        self.generate_conflict_recommendations(conflict_checker)?;

        // Generate integration recommendations
        self.generate_integration_recommendations(integration_tracker)?;

        // Sort recommendations by priority
        self.recommendations.sort_by(|a, b| b.priority.cmp(&a.priority));

        // Limit to max recommendations
        if self.recommendations.len() > self.max_recommendations {
            self.recommendations.truncate(self.max_recommendations);
        }

        println!("Generated {} recommendations", self.recommendations.len());

        Ok(())
    }

    /// Generate entity-based recommendations
    fn generate_entity_recommendations(&mut self, entity_mapper: &EntityMapper) -> Result<()> {
        println!("Generating entity-based recommendations...");

        // Get unmapped entities
        let canvas_entities = entity_mapper.get_entities_by_source("canvas");
        let discourse_entities = entity_mapper.get_entities_by_source("discourse");
        let mappings = entity_mapper.get_mappings();

        let mapped_canvas_entities: HashSet<String> = mappings.iter()
            .filter(|m| m.source_entity.starts_with("canvas"))
            .map(|m| m.source_entity.clone())
            .collect();

        let mapped_discourse_entities: HashSet<String> = mappings.iter()
            .filter(|m| m.source_entity.starts_with("discourse"))
            .map(|m| m.source_entity.clone())
            .collect();

        let unmapped_canvas_entities: Vec<String> = canvas_entities.into_iter()
            .filter(|e| !mapped_canvas_entities.contains(e))
            .collect();

        let unmapped_discourse_entities: Vec<String> = discourse_entities.into_iter()
            .filter(|e| !mapped_discourse_entities.contains(e))
            .collect();

        // Generate recommendations for unmapped entities
        for entity_name in unmapped_canvas_entities {
            if let Some(entity) = entity_mapper.get_entity(&entity_name) {
                // Determine priority based on category
                let priority = match entity.category.as_str() {
                    "course" => 5,
                    "assignment" => 5,
                    "user" => 4,
                    "discussion" => 4,
                    _ => 3,
                };

                // Determine effort based on field count
                let effort = (entity.fields.len() as f32 / 5.0).max(0.5).min(5.0);

                // Create recommendation
                let recommendation = Recommendation {
                    id: format!("entity_{}", entity.entity),
                    title: format!("Implement Canvas Entity: {}", entity.entity),
                    description: format!("Canvas entity '{}' is not yet mapped to Ordo. This entity belongs to the '{}' category and has {} fields.",
                        entity.entity, entity.category, entity.fields.len()),
                    priority,
                    effort,
                    related_entities: vec![entity_name.clone()],
                    related_features: Vec::new(),
                    steps: vec![
                        format!("Create a new Rust struct for '{}'", entity.entity),
                        format!("Implement fields and relationships"),
                        format!("Add database schema and migrations"),
                        format!("Implement CRUD operations"),
                        format!("Add synchronization support"),
                    ],
                };

                self.recommendations.push(recommendation);
            }
        }

        for entity_name in unmapped_discourse_entities {
            if let Some(entity) = entity_mapper.get_entity(&entity_name) {
                // Determine priority based on category
                let priority = match entity.category.as_str() {
                    "topic" => 5,
                    "post" => 5,
                    "user" => 4,
                    "category" => 4,
                    _ => 3,
                };

                // Determine effort based on field count
                let effort = (entity.fields.len() as f32 / 5.0).max(0.5).min(5.0);

                // Create recommendation
                let recommendation = Recommendation {
                    id: format!("entity_{}", entity.entity),
                    title: format!("Implement Discourse Entity: {}", entity.entity),
                    description: format!("Discourse entity '{}' is not yet mapped to Ordo. This entity belongs to the '{}' category and has {} fields.",
                        entity.entity, entity.category, entity.fields.len()),
                    priority,
                    effort,
                    related_entities: vec![entity_name.clone()],
                    related_features: Vec::new(),
                    steps: vec![
                        format!("Create a new Rust struct for '{}'", entity.entity),
                        format!("Implement fields and relationships"),
                        format!("Add database schema and migrations"),
                        format!("Implement CRUD operations"),
                        format!("Add synchronization support"),
                    ],
                };

                self.recommendations.push(recommendation);
            }
        }

        Ok(())
    }

    /// Generate feature-based recommendations
    fn generate_feature_recommendations(&mut self, feature_detector: &FeatureDetector) -> Result<()> {
        println!("Generating feature-based recommendations...");

        // Get missing features
        let mappings = &feature_detector.mappings;

        let missing_features: Vec<_> = mappings.iter()
            .filter(|m| m.status == "missing")
            .collect();

        // Generate recommendations for missing features
        for mapping in missing_features {
            let source_parts: Vec<&str> = mapping.source_feature.split('.').collect();
            if source_parts.len() < 2 {
                continue;
            }

            let source = source_parts[0];
            let feature_name = source_parts[1];

            // Create recommendation
            let recommendation = Recommendation {
                id: format!("feature_{}", feature_name),
                title: format!("Implement {} Feature: {}", source, feature_name),
                description: format!("{} feature '{}' is not yet implemented in Ordo. This feature has priority {}.",
                    source, feature_name, mapping.priority),
                priority: mapping.priority,
                effort: match mapping.priority {
                    5 => 5.0, // High priority often means complex features
                    4 => 4.0,
                    3 => 3.0,
                    2 => 2.0,
                    _ => 1.0,
                },
                related_entities: Vec::new(),
                related_features: vec![mapping.source_feature.clone()],
                steps: vec![
                    format!("Analyze {} implementation of '{}'", source, feature_name),
                    format!("Design Rust implementation"),
                    format!("Implement backend logic"),
                    format!("Implement frontend components"),
                    format!("Add tests"),
                ],
            };

            self.recommendations.push(recommendation);
        }

        Ok(())
    }

    /// Generate code quality recommendations
    fn generate_code_quality_recommendations(&mut self, code_quality_scorer: &CodeQualityScorer) -> Result<()> {
        println!("Generating code quality recommendations...");

        // Get files recommended for rebuild
        let metrics = code_quality_scorer.get_metrics();

        let rebuild_files: Vec<_> = metrics.values()
            .filter(|m| m.recommendation == "rebuild")
            .collect();

        // Generate recommendations for files that need to be rebuilt
        for metrics in rebuild_files.iter().take(5) { // Limit to 5 recommendations
            // Create recommendation
            let recommendation = Recommendation {
                id: format!("quality_{}", metrics.file_path.replace("/", "_").replace("\\", "_")),
                title: format!("Refactor Low-Quality File: {}", metrics.file_path),
                description: format!("File '{}' has low code quality (score: {}). It has {} lines of code, complexity of {}, and comment coverage of {:.1}%.",
                    metrics.file_path, metrics.usefulness_score, metrics.loc, metrics.complexity, metrics.comment_coverage * 100.0),
                priority: 2, // Medium-low priority
                effort: (metrics.loc as f32 / 100.0).max(0.5).min(5.0),
                related_entities: Vec::new(),
                related_features: Vec::new(),
                steps: vec![
                    format!("Review file structure and complexity"),
                    format!("Break down large functions/methods"),
                    format!("Improve naming and documentation"),
                    format!("Add tests"),
                    format!("Verify functionality"),
                ],
            };

            self.recommendations.push(recommendation);
        }

        Ok(())
    }

    /// Generate conflict resolution recommendations
    fn generate_conflict_recommendations(&mut self, conflict_checker: &ConflictChecker) -> Result<()> {
        println!("Generating conflict resolution recommendations...");

        // Get conflicts
        let conflicts = conflict_checker.get_conflicts();

        // Generate recommendations for conflicts
        for conflict in conflicts {
            // Create recommendation
            let recommendation = Recommendation {
                id: format!("conflict_{}_{}",
                    conflict.entity1.replace(".", "_"),
                    conflict.entity2.replace(".", "_")),
                title: format!("Resolve {} Conflict: {} and {}",
                    conflict.conflict_type, conflict.entity1, conflict.entity2),
                description: format!("{}: {}", conflict.conflict_type, conflict.description),
                priority: match conflict.conflict_type.as_str() {
                    "name" => 4, // High priority
                    "field" => 3, // Medium priority
                    "semantic" => 2, // Lower priority
                    _ => 2,
                },
                effort: 1.0, // Conflicts are usually quick to resolve
                related_entities: vec![conflict.entity1.clone(), conflict.entity2.clone()],
                related_features: Vec::new(),
                steps: vec![
                    format!("Review conflict details"),
                    conflict.suggested_resolution.clone(),
                    format!("Update entity definitions"),
                    format!("Update related code"),
                    format!("Verify resolution"),
                ],
            };

            self.recommendations.push(recommendation);
        }

        Ok(())
    }

    /// Generate integration recommendations
    fn generate_integration_recommendations(&mut self, integration_tracker: &IntegrationTracker) -> Result<()> {
        println!("Generating integration recommendations...");

        // Get progress
        let progress = integration_tracker.get_progress();

        // Find categories with low progress
        let mut categories: Vec<(&String, &f32)> = progress.category_progress.iter().collect();
        categories.sort_by(|a, b| a.1.partial_cmp(b.1).unwrap());

        // Generate recommendations for low-progress categories
        for (category, progress_value) in categories.iter().take(3) { // Top 3 lowest progress
            if **progress_value < 0.5 { // Less than 50% progress
                // Create recommendation
                let recommendation = Recommendation {
                    id: format!("integration_{}", category),
                    title: format!("Improve Integration for Category: {}", category),
                    description: format!("Category '{}' has low integration progress ({:.1}%). Focus on implementing more entities and features in this category.",
                        category, **progress_value * 100.0),
                    priority: if **progress_value < 0.2 { 5 } else if **progress_value < 0.3 { 4 } else { 3 },
                    effort: 5.0, // Category integration is a significant effort
                    related_entities: Vec::new(),
                    related_features: Vec::new(),
                    steps: vec![
                        format!("Review missing entities in '{}' category", category),
                        format!("Review missing features in '{}' category", category),
                        format!("Prioritize implementation tasks"),
                        format!("Implement high-priority entities and features"),
                        format!("Add tests and documentation"),
                    ],
                };

                self.recommendations.push(recommendation);
            }
        }

        Ok(())
    }

    /// Generate a JSON report of recommendations
    pub fn generate_recommendations_report(&self) -> Result<String> {
        let report = serde_json::to_string_pretty(&self.recommendations)?;
        Ok(report)
    }

    /// Generate a Markdown report of recommendations
    pub fn generate_recommendations_markdown(&self) -> String {
        let mut markdown = String::new();

        markdown.push_str("# Development Recommendations\n\n");

        // Summary statistics
        let total_recommendations = self.recommendations.len();
        let high_priority = self.recommendations.iter()
            .filter(|r| r.priority >= self.high_priority_threshold)
            .count();
        let medium_priority = self.recommendations.iter()
            .filter(|r| r.priority >= self.medium_priority_threshold && r.priority < self.high_priority_threshold)
            .count();
        let low_priority = self.recommendations.iter()
            .filter(|r| r.priority < self.medium_priority_threshold)
            .count();

        markdown.push_str("## Summary\n\n");
        markdown.push_str(&format!("- Total Recommendations: {}\n", total_recommendations));
        markdown.push_str(&format!("- High Priority: {}\n", high_priority));
        markdown.push_str(&format!("- Medium Priority: {}\n", medium_priority));
        markdown.push_str(&format!("- Low Priority: {}\n\n", low_priority));

        // High priority recommendations
        if high_priority > 0 {
            markdown.push_str("## High Priority Recommendations\n\n");

            for recommendation in self.recommendations.iter()
                .filter(|r| r.priority >= self.high_priority_threshold) {
                markdown.push_str(&format!("### {}\n\n", recommendation.title));
                markdown.push_str(&format!("**Priority:** {}/5 | **Effort:** {:.1} days\n\n",
                    recommendation.priority, recommendation.effort));
                markdown.push_str(&format!("{}\n\n", recommendation.description));

                markdown.push_str("**Implementation Steps:**\n\n");
                for (i, step) in recommendation.steps.iter().enumerate() {
                    markdown.push_str(&format!("{}. {}\n", i + 1, step));
                }

                markdown.push_str("\n");

                if !recommendation.related_entities.is_empty() {
                    markdown.push_str("**Related Entities:** ");
                    markdown.push_str(&recommendation.related_entities.join(", "));
                    markdown.push_str("\n\n");
                }

                if !recommendation.related_features.is_empty() {
                    markdown.push_str("**Related Features:** ");
                    markdown.push_str(&recommendation.related_features.join(", "));
                    markdown.push_str("\n\n");
                }

                markdown.push_str("---\n\n");
            }
        }

        // Medium priority recommendations
        if medium_priority > 0 {
            markdown.push_str("## Medium Priority Recommendations\n\n");

            for recommendation in self.recommendations.iter()
                .filter(|r| r.priority >= self.medium_priority_threshold && r.priority < self.high_priority_threshold) {
                markdown.push_str(&format!("### {}\n\n", recommendation.title));
                markdown.push_str(&format!("**Priority:** {}/5 | **Effort:** {:.1} days\n\n",
                    recommendation.priority, recommendation.effort));
                markdown.push_str(&format!("{}\n\n", recommendation.description));

                markdown.push_str("**Implementation Steps:**\n\n");
                for (i, step) in recommendation.steps.iter().enumerate() {
                    markdown.push_str(&format!("{}. {}\n", i + 1, step));
                }

                markdown.push_str("\n");

                if !recommendation.related_entities.is_empty() {
                    markdown.push_str("**Related Entities:** ");
                    markdown.push_str(&recommendation.related_entities.join(", "));
                    markdown.push_str("\n\n");
                }

                if !recommendation.related_features.is_empty() {
                    markdown.push_str("**Related Features:** ");
                    markdown.push_str(&recommendation.related_features.join(", "));
                    markdown.push_str("\n\n");
                }

                markdown.push_str("---\n\n");
            }
        }

        // Low priority recommendations (just list them)
        if low_priority > 0 {
            markdown.push_str("## Low Priority Recommendations\n\n");

            for recommendation in self.recommendations.iter()
                .filter(|r| r.priority < self.medium_priority_threshold) {
                markdown.push_str(&format!("- **{}** (Priority: {}/5, Effort: {:.1} days)\n",
                    recommendation.title, recommendation.priority, recommendation.effort));
            }
        }

        markdown
    }
}
