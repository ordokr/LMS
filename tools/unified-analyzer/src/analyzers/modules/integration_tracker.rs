use std::collections::{HashMap, HashSet};
use serde::{Deserialize, Serialize};
use anyhow::{Result, anyhow};

use crate::analyzers::modules::entity_mapper::EntityMapper;
use crate::analyzers::modules::feature_detector::{FeatureDetector, Feature};

/// Integration statistics for reporting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationStats {
    /// Overall integration percentage (0.0-1.0)
    pub overall_integration_percentage: f32,
    /// Entity integration percentage (0.0-1.0)
    pub entity_integration_percentage: f32,
    /// Feature integration percentage (0.0-1.0)
    pub feature_integration_percentage: f32,
    /// Integration percentage by category
    pub integration_by_category: HashMap<String, f32>,
}

/// Represents integration progress for entities and features
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationProgress {
    /// Entity integration progress
    pub entity_progress: HashMap<String, f32>,
    /// Feature integration progress
    pub feature_progress: HashMap<String, f32>,
    /// Overall integration progress
    pub overall_progress: f32,
    /// Integration status by category
    pub category_progress: HashMap<String, f32>,
}

/// Integration Progress Tracker for monitoring integration progress
pub struct IntegrationTracker {
    /// Integration progress
    progress: IntegrationProgress,
    /// Weights for entity and feature progress
    entity_weight: f32,
    feature_weight: f32,
}

impl IntegrationTracker {
    /// Create a new IntegrationTracker with default settings
    pub fn new() -> Self {
        Self {
            progress: IntegrationProgress {
                entity_progress: HashMap::new(),
                feature_progress: HashMap::new(),
                overall_progress: 0.0,
                category_progress: HashMap::new(),
            },
            entity_weight: 0.5,
            feature_weight: 0.5,
        }
    }

    /// Create a new IntegrationTracker with custom settings
    pub fn with_config(
        entity_weight: f32,
        feature_weight: f32,
    ) -> Self {
        Self {
            progress: IntegrationProgress {
                entity_progress: HashMap::new(),
                feature_progress: HashMap::new(),
                overall_progress: 0.0,
                category_progress: HashMap::new(),
            },
            entity_weight,
            feature_weight,
        }
    }

    /// Get the integration progress
    pub fn get_progress(&self) -> &IntegrationProgress {
        &self.progress
    }

    /// Track integration progress
    pub fn track_progress(&mut self, entity_mapper: &EntityMapper, feature_detector: &FeatureDetector) -> Result<()> {
        println!("Tracking integration progress...");

        // Track entity integration progress
        self.track_entity_progress(entity_mapper)?;

        // Track feature integration progress
        self.track_feature_progress(feature_detector)?;

        // Calculate overall progress
        self.calculate_overall_progress()?;

        println!("Overall integration progress: {:.1}%", self.progress.overall_progress * 100.0);

        Ok(())
    }

    /// Track entity integration progress
    fn track_entity_progress(&mut self, entity_mapper: &EntityMapper) -> Result<()> {
        println!("Tracking entity integration progress...");

        let mut entity_progress = HashMap::new();

        // Get entities by source
        let canvas_entities = entity_mapper.get_entities_by_source("canvas");
        let discourse_entities = entity_mapper.get_entities_by_source("discourse");

        // Get mappings
        let mappings = entity_mapper.get_mappings();

        // Calculate Canvas integration progress
        let canvas_mapped = mappings.iter()
            .filter(|m| m.source_entity.starts_with("canvas"))
            .count();

        let canvas_progress = if !canvas_entities.is_empty() {
            canvas_mapped as f32 / canvas_entities.len() as f32
        } else {
            0.0
        };

        entity_progress.insert("canvas".to_string(), canvas_progress);

        // Calculate Discourse integration progress
        let discourse_mapped = mappings.iter()
            .filter(|m| m.source_entity.starts_with("discourse"))
            .count();

        let discourse_progress = if !discourse_entities.is_empty() {
            discourse_mapped as f32 / discourse_entities.len() as f32
        } else {
            0.0
        };

        entity_progress.insert("discourse".to_string(), discourse_progress);

        // Calculate progress by category
        let mut category_progress = HashMap::new();

        // Get all categories
        let mut categories = HashSet::new();

        for mapping in mappings {
            if let Some(entity) = entity_mapper.get_entity(&mapping.source_entity) {
                categories.insert(entity.category.clone());
            }
        }

        // Calculate progress for each category
        for category in categories {
            let category_entities = entity_mapper.get_entities_by_category(&category);
            let category_mapped = mappings.iter()
                .filter(|m| {
                    if let Some(entity) = entity_mapper.get_entity(&m.source_entity) {
                        entity.category == category
                    } else {
                        false
                    }
                })
                .count();

            let category_progress_value = if !category_entities.is_empty() {
                category_mapped as f32 / category_entities.len() as f32
            } else {
                0.0
            };

            category_progress.insert(category, category_progress_value);
        }

        self.progress.entity_progress = entity_progress;
        self.progress.category_progress = category_progress;

        Ok(())
    }

    /// Track feature integration progress
    fn track_feature_progress(&mut self, feature_detector: &FeatureDetector) -> Result<()> {
        println!("Tracking feature integration progress...");

        let mut feature_progress = HashMap::new();

        // Get features by source
        let features = &feature_detector.features;
        let canvas_features = features.get("canvas").cloned().unwrap_or_default();
        let discourse_features = features.get("discourse").cloned().unwrap_or_default();

        // Get mappings
        let mappings = &feature_detector.mappings;

        // Calculate Canvas feature integration progress
        let canvas_implemented = mappings.iter()
            .filter(|m| m.source_feature.starts_with("canvas") && m.status == "implemented")
            .count();

        let canvas_partial = mappings.iter()
            .filter(|m| m.source_feature.starts_with("canvas") && m.status == "partial")
            .count();

        let canvas_progress = if !canvas_features.is_empty() {
            (canvas_implemented as f32 + 0.5 * canvas_partial as f32) / canvas_features.len() as f32
        } else {
            0.0
        };

        feature_progress.insert("canvas".to_string(), canvas_progress);

        // Calculate Discourse feature integration progress
        let discourse_implemented = mappings.iter()
            .filter(|m| m.source_feature.starts_with("discourse") && m.status == "implemented")
            .count();

        let discourse_partial = mappings.iter()
            .filter(|m| m.source_feature.starts_with("discourse") && m.status == "partial")
            .count();

        let discourse_progress = if !discourse_features.is_empty() {
            (discourse_implemented as f32 + 0.5 * discourse_partial as f32) / discourse_features.len() as f32
        } else {
            0.0
        };

        feature_progress.insert("discourse".to_string(), discourse_progress);

        // Calculate progress by category
        let categories: Vec<String> = self.progress.category_progress.keys().cloned().collect();
        for category in &categories {
            // Get features in this category
            let category_canvas_features = canvas_features.iter()
                .filter(|f| f.category == *category)
                .count();

            let category_discourse_features = discourse_features.iter()
                .filter(|f| f.category == *category)
                .count();

            let category_total = category_canvas_features + category_discourse_features;

            // Get implemented and partial features in this category
            let category_implemented = mappings.iter()
                .filter(|m| {
                    let source_parts: Vec<&str> = m.source_feature.split('.').collect();
                    if source_parts.len() >= 2 {
                        let source_name = source_parts[1];
                        let source_feature = self.find_feature_by_name(features, source_name);
                        source_feature.map_or(false, |f| f.category == *category && m.status == "implemented")
                    } else {
                        false
                    }
                })
                .count();

            let category_partial = mappings.iter()
                .filter(|m| {
                    let source_parts: Vec<&str> = m.source_feature.split('.').collect();
                    if source_parts.len() >= 2 {
                        let source_name = source_parts[1];
                        let source_feature = self.find_feature_by_name(features, source_name);
                        source_feature.map_or(false, |f| f.category == *category && m.status == "partial")
                    } else {
                        false
                    }
                })
                .count();

            let category_progress_value = if category_total > 0 {
                (category_implemented as f32 + 0.5 * category_partial as f32) / category_total as f32
            } else {
                0.0
            };

            // Update category progress with feature progress
            let entity_progress = self.progress.category_progress.get(category).cloned().unwrap_or(0.0);
            let combined_progress = (entity_progress + category_progress_value) / 2.0;
            self.progress.category_progress.insert(category.clone(), combined_progress);
        }

        self.progress.feature_progress = feature_progress;

        Ok(())
    }

    /// Calculate overall progress
    fn calculate_overall_progress(&mut self) -> Result<()> {
        println!("Calculating overall progress...");

        // Calculate weighted average of entity and feature progress
        let entity_progress_avg = self.progress.entity_progress.values().sum::<f32>() /
            self.progress.entity_progress.len().max(1) as f32;

        let feature_progress_avg = self.progress.feature_progress.values().sum::<f32>() /
            self.progress.feature_progress.len().max(1) as f32;

        let overall_progress = entity_progress_avg * self.entity_weight +
            feature_progress_avg * self.feature_weight;

        self.progress.overall_progress = overall_progress;

        Ok(())
    }

    /// Find a feature by name
    fn find_feature_by_name<'a>(&self, features: &'a HashMap<String, Vec<Feature>>, name: &str) -> Option<&'a Feature> {
        for features_list in features.values() {
            for feature in features_list {
                if feature.name == name {
                    return Some(feature);
                }
            }
        }
        None
    }

    /// Generate a JSON report of integration progress
    pub fn generate_progress_report(&self) -> Result<String> {
        let report = serde_json::to_string_pretty(&self.progress)?;
        Ok(report)
    }

    /// Get integration statistics
    pub fn get_stats(&self) -> Option<IntegrationStats> {
        // Calculate entity integration percentage
        let entity_integration_percentage = self.progress.entity_progress.values().sum::<f32>() /
            self.progress.entity_progress.len().max(1) as f32;

        // Calculate feature integration percentage
        let feature_integration_percentage = self.progress.feature_progress.values().sum::<f32>() /
            self.progress.feature_progress.len().max(1) as f32;

        // Create integration stats
        Some(IntegrationStats {
            overall_integration_percentage: self.progress.overall_progress,
            entity_integration_percentage,
            feature_integration_percentage,
            integration_by_category: self.progress.category_progress.clone(),
        })
    }

    /// Generate a Markdown report of integration progress
    pub fn generate_progress_markdown(&self) -> String {
        let mut markdown = String::new();

        markdown.push_str("# Integration Progress Report\n\n");

        // Overall progress
        markdown.push_str("## Overall Progress\n\n");
        markdown.push_str(&format!("Overall Integration Progress: {:.1}%\n\n", self.progress.overall_progress * 100.0));

        // Entity progress
        markdown.push_str("## Entity Integration Progress\n\n");
        markdown.push_str("| Source | Progress |\n");
        markdown.push_str("|--------|----------|\n");

        for (source, progress) in &self.progress.entity_progress {
            markdown.push_str(&format!("| {} | {:.1}% |\n", source, progress * 100.0));
        }

        markdown.push_str("\n");

        // Feature progress
        markdown.push_str("## Feature Integration Progress\n\n");
        markdown.push_str("| Source | Progress |\n");
        markdown.push_str("|--------|----------|\n");

        for (source, progress) in &self.progress.feature_progress {
            markdown.push_str(&format!("| {} | {:.1}% |\n", source, progress * 100.0));
        }

        markdown.push_str("\n");

        // Category progress
        markdown.push_str("## Progress by Category\n\n");
        markdown.push_str("| Category | Progress |\n");
        markdown.push_str("|----------|----------|\n");

        // Sort categories by progress (descending)
        let mut categories: Vec<(&String, &f32)> = self.progress.category_progress.iter().collect();
        categories.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());

        for (category, progress) in categories {
            markdown.push_str(&format!("| {} | {:.1}% |\n", category, progress * 100.0));
        }

        markdown
    }
}
