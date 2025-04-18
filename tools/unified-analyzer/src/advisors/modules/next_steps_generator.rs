use std::path::{Path, PathBuf};
use std::fs;
use std::collections::HashMap;
use anyhow::{Result, Context};

use crate::analyzers::modules::entity_mapper::EntityMapper;
use crate::analyzers::modules::feature_detector::FeatureDetector;
use crate::analyzers::modules::code_quality_scorer::CodeQualityScorer;
use crate::analyzers::modules::conflict_checker::ConflictChecker;
use crate::analyzers::modules::integration_tracker::{IntegrationTracker, IntegrationStats};
use crate::analyzers::modules::recommendation_system::{RecommendationSystem, Recommendation};

/// Next Steps Generator for creating actionable next steps documents
pub struct NextStepsGenerator {
    /// Base directory for the project
    base_dir: PathBuf,
    /// Entity mapper for mapping entities between systems
    entity_mapper: &'static EntityMapper,
    /// Feature detector for detecting features
    feature_detector: &'static FeatureDetector,
    /// Code quality scorer for evaluating code quality
    code_quality_scorer: &'static CodeQualityScorer,
    /// Conflict checker for detecting conflicts
    conflict_checker: &'static ConflictChecker,
    /// Integration tracker for tracking integration progress
    integration_tracker: &'static IntegrationTracker,
    /// Recommendation system for generating recommendations
    recommendation_system: &'static RecommendationSystem,
}

impl NextStepsGenerator {
    /// Create a new NextStepsGenerator
    pub fn new(
        base_dir: &Path,
        entity_mapper: &'static EntityMapper,
        feature_detector: &'static FeatureDetector,
        code_quality_scorer: &'static CodeQualityScorer,
        conflict_checker: &'static ConflictChecker,
        integration_tracker: &'static IntegrationTracker,
        recommendation_system: &'static RecommendationSystem,
    ) -> Self {
        Self {
            base_dir: base_dir.to_path_buf(),
            entity_mapper,
            feature_detector,
            code_quality_scorer,
            conflict_checker,
            integration_tracker,
            recommendation_system,
        }
    }

    /// Generate next steps document
    pub fn generate_next_steps_document(&self) -> Result<String> {
        // Generate next steps markdown based on recommendations and integration progress
        let mut markdown = String::new();

        markdown.push_str("# Next Steps for Ordo Development\n\n");
        markdown.push_str("Based on the integration analysis, here are the recommended next steps for the Ordo project:\n\n");

        // Get integration stats
        let integration_stats = match self.integration_tracker.get_stats() {
            Some(stats) => stats,
            None => return Err(anyhow::anyhow!("Failed to get integration stats")),
        };

        // Add integration progress summary
        markdown.push_str("## Current Integration Status\n\n");
        markdown.push_str(&format!("- Overall integration: {:.1}%\n", integration_stats.overall_integration_percentage * 100.0));
        markdown.push_str(&format!("- Entity integration: {:.1}%\n", integration_stats.entity_integration_percentage * 100.0));
        markdown.push_str(&format!("- Feature integration: {:.1}%\n\n", integration_stats.feature_integration_percentage * 100.0));

        // Add category progress
        markdown.push_str("**Integration by Category:**\n\n");

        // Sort categories by progress (ascending) to focus on least integrated areas first
        let mut categories: Vec<(&String, &f32)> = integration_stats.integration_by_category.iter().collect();
        categories.sort_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal));

        for (category, percentage) in categories.iter().take(5) {
            markdown.push_str(&format!("- {}: {:.1}%\n", category, **percentage * 100.0));
        }
        markdown.push_str("\n");

        // Immediate actions
        markdown.push_str("## Immediate Actions (Next 2 Weeks)\n\n");

        // Get top priority recommendations
        let recommendations = self.recommendation_system.get_recommendations();
        let mut priority_recommendations: Vec<&Recommendation> = recommendations.iter()
            .filter(|r| r.priority <= 2) // Priority 1 and 2 are highest
            .collect();

        priority_recommendations.sort_by(|a, b| a.priority.cmp(&b.priority));

        if priority_recommendations.is_empty() {
            // If no high priority recommendations, add some based on integration stats
            let lowest_categories: Vec<(&String, &f32)> = categories.iter().take(2).cloned().collect();

            for (i, (category, percentage)) in lowest_categories.iter().enumerate() {
                markdown.push_str(&format!("{}. **Improve {} Integration**\n", i + 1, category));
                markdown.push_str(&format!("   - Current integration: {:.1}%\n", **percentage * 100.0));
                markdown.push_str("   - Implement core features in this category\n");
                markdown.push_str("   - Ensure proper test coverage\n");
                markdown.push_str("   - Document integration points\n\n");
            }

            // Add a general recommendation for code quality
            markdown.push_str("3. **Improve Code Quality**\n");
            markdown.push_str("   - Refactor code with high complexity\n");
            markdown.push_str("   - Improve error handling\n");
            markdown.push_str("   - Add missing documentation\n\n");
        } else {
            // Use actual recommendations
            for (i, recommendation) in priority_recommendations.iter().take(3).enumerate() {
                markdown.push_str(&format!("{}. **{}**\n", i + 1, recommendation.title));
                markdown.push_str(&format!("   - {}\n", recommendation.description));

                // Add steps if available
                if !recommendation.steps.is_empty() {
                    for step in &recommendation.steps {
                        markdown.push_str(&format!("   - {}\n", step));
                    }
                }

                markdown.push_str("\n");
            }
        }

        // Short-term goals
        markdown.push_str("## Short-Term Goals (Next Month)\n\n");

        let mut medium_recommendations: Vec<&Recommendation> = recommendations.iter()
            .filter(|r| r.priority > 2 && r.priority <= 3)
            .collect();

        medium_recommendations.sort_by(|a, b| a.priority.cmp(&b.priority));

        if medium_recommendations.is_empty() {
            // If no medium priority recommendations, add some based on code quality metrics
            let metrics = self.code_quality_scorer.get_metrics();
            let rebuild_count = metrics.values().filter(|m| m.recommendation == "rebuild").count();
            let refactor_count = metrics.values().filter(|m| m.recommendation == "partial").count();

            markdown.push_str("1. **Address Technical Debt**\n");
            markdown.push_str(&format!("   - Refactor {} files identified for partial reuse\n", refactor_count));
            markdown.push_str(&format!("   - Redesign {} components identified for rebuilding\n", rebuild_count));
            markdown.push_str("   - Improve test coverage for core modules\n\n");

            markdown.push_str("2. **Enhance Offline Capabilities**\n");
            markdown.push_str("   - Implement local-first data storage\n");
            markdown.push_str("   - Develop sync mechanism for reconnection\n");
            markdown.push_str("   - Add conflict resolution strategies\n\n");

            markdown.push_str("3. **Improve User Experience**\n");
            markdown.push_str("   - Optimize UI performance\n");
            markdown.push_str("   - Enhance accessibility features\n");
            markdown.push_str("   - Implement responsive design for mobile devices\n\n");
        } else {
            // Use actual recommendations
            for (i, recommendation) in medium_recommendations.iter().take(3).enumerate() {
                markdown.push_str(&format!("{}. **{}**\n", i + 1, recommendation.title));
                markdown.push_str(&format!("   - {}\n", recommendation.description));

                // Add steps if available
                if !recommendation.steps.is_empty() {
                    for step in &recommendation.steps {
                        markdown.push_str(&format!("   - {}\n", step));
                    }
                }

                markdown.push_str("\n");
            }
        }

        // Medium-term goals
        markdown.push_str("## Medium-Term Goals (Next Quarter)\n\n");

        let mut low_recommendations: Vec<&Recommendation> = recommendations.iter()
            .filter(|r| r.priority > 3)
            .collect();

        low_recommendations.sort_by(|a, b| a.priority.cmp(&b.priority));

        if low_recommendations.is_empty() {
            // If no low priority recommendations, add some general ones
            markdown.push_str("1. **Complete Feature Parity**\n");
            markdown.push_str("   - Implement remaining Canvas features\n");
            markdown.push_str("   - Implement remaining Discourse features\n");
            markdown.push_str("   - Ensure all critical functionality is covered\n\n");

            markdown.push_str("2. **Performance Optimization**\n");
            markdown.push_str("   - Conduct performance profiling\n");
            markdown.push_str("   - Optimize database queries\n");
            markdown.push_str("   - Reduce memory usage\n\n");

            markdown.push_str("3. **Security Enhancements**\n");
            markdown.push_str("   - Conduct security audit\n");
            markdown.push_str("   - Implement end-to-end encryption\n");
            markdown.push_str("   - Enhance authentication mechanisms\n\n");
        } else {
            // Use actual recommendations
            for (i, recommendation) in low_recommendations.iter().take(3).enumerate() {
                markdown.push_str(&format!("{}. **{}**\n", i + 1, recommendation.title));
                markdown.push_str(&format!("   - {}\n", recommendation.description));

                // Add steps if available
                if !recommendation.steps.is_empty() {
                    for step in &recommendation.steps {
                        markdown.push_str(&format!("   - {}\n", step));
                    }
                }

                markdown.push_str("\n");
            }
        }

        // Technical debt reduction
        markdown.push_str("## Technical Debt Reduction\n\n");

        // Get code quality metrics to identify specific issues
        let metrics = self.code_quality_scorer.get_metrics();
        let high_complexity_files = metrics.iter()
            .filter(|(_, m)| m.complexity > 20)
            .take(3)
            .map(|(path, _)| path.clone())
            .collect::<Vec<String>>();

        let low_comment_files = metrics.iter()
            .filter(|(_, m)| m.comment_coverage < 0.1 && m.loc > 100)
            .take(3)
            .map(|(path, _)| path.clone())
            .collect::<Vec<String>>();

        markdown.push_str("1. **Error Handling Improvements**\n");
        markdown.push_str("   - Replace unwrap() calls with proper error handling\n");
        markdown.push_str("   - Implement consistent error types\n");
        markdown.push_str("   - Add error logging\n");
        markdown.push_str("   - Improve error messages\n\n");

        markdown.push_str("2. **Code Organization**\n");
        markdown.push_str("   - Split large files into smaller modules\n");
        if !high_complexity_files.is_empty() {
            markdown.push_str("   - High complexity files to refactor:\n");
            for file in high_complexity_files {
                markdown.push_str(&format!("     - {}\n", file));
            }
        }
        markdown.push_str("   - Improve module organization\n");
        markdown.push_str("   - Reduce function complexity\n\n");

        markdown.push_str("3. **Documentation Improvements**\n");
        markdown.push_str("   - Add missing documentation\n");
        if !low_comment_files.is_empty() {
            markdown.push_str("   - Files needing documentation:\n");
            for file in low_comment_files {
                markdown.push_str(&format!("     - {}\n", file));
            }
        }
        markdown.push_str("   - Create API reference documentation\n");
        markdown.push_str("   - Add examples for complex functionality\n\n");

        markdown.push_str("4. **Test Coverage**\n");
        markdown.push_str("   - Implement unit tests for core functionality\n");
        markdown.push_str("   - Add integration tests\n");
        markdown.push_str("   - Set up CI/CD pipeline\n");
        markdown.push_str("   - Implement test coverage reporting\n\n");

        // Documentation enhancements
        markdown.push_str("## Documentation Enhancements\n\n");

        markdown.push_str("1. **API Documentation**\n");
        markdown.push_str("   - Document all public APIs\n");
        markdown.push_str("   - Add examples for common use cases\n");
        markdown.push_str("   - Create API reference guide\n");
        markdown.push_str("   - Add diagrams for complex flows\n\n");

        markdown.push_str("2. **Architecture Documentation**\n");
        markdown.push_str("   - Update component diagrams\n");
        markdown.push_str("   - Document integration patterns\n");
        markdown.push_str("   - Add sequence diagrams for key processes\n");
        markdown.push_str("   - Document design decisions\n\n");

        markdown.push_str("3. **User Documentation**\n");
        markdown.push_str("   - Create user guides\n");
        markdown.push_str("   - Add screenshots and examples\n");
        markdown.push_str("   - Document offline workflows\n");
        markdown.push_str("   - Create troubleshooting guide\n");

        Ok(markdown)
    }
}
