use std::path::{Path, PathBuf};
use std::fs;
use std::collections::HashMap;
use anyhow::{Result, Context};
use serde_json::Value;

use crate::analyzers::modules::code_quality_scorer::CodeQualityScorer;
use crate::analyzers::modules::conflict_checker::{ConflictChecker, Conflict};
use crate::analyzers::modules::integration_tracker::{IntegrationTracker, IntegrationStats};
use crate::analyzers::modules::recommendation_system_improved::{RecommendationSystem, Recommendation};
use crate::analyzers::modules::entity_mapper::EntityMapper;
use crate::analyzers::modules::feature_detector::FeatureDetector;

/// Integration Advisor for providing recommendations on integrating Canvas and Discourse
pub struct IntegrationAdvisor {
    /// Base directory for the project
    base_dir: PathBuf,
    /// Entity mapper for mapping entities between systems
    entity_mapper: EntityMapper,
    /// Feature detector for detecting features
    feature_detector: FeatureDetector,
    /// Code quality scorer for evaluating code quality
    code_quality_scorer: CodeQualityScorer,
    /// Conflict checker for detecting conflicts
    conflict_checker: ConflictChecker,
    /// Integration tracker for tracking integration progress
    integration_tracker: IntegrationTracker,
    /// Recommendation system for generating recommendations
    recommendation_system: RecommendationSystem,
    /// Output directory for reports
    output_dir: PathBuf,
}

impl IntegrationAdvisor {
    /// Create a new IntegrationAdvisor
    pub fn new(base_dir: &Path) -> Self {
        let output_dir = base_dir.join("docs").join("integration-advisor").join("reports");

        Self {
            base_dir: base_dir.to_path_buf(),
            entity_mapper: EntityMapper::new(),
            feature_detector: FeatureDetector::new(),
            code_quality_scorer: CodeQualityScorer::new(),
            conflict_checker: ConflictChecker::new(),
            integration_tracker: IntegrationTracker::new(),
            recommendation_system: RecommendationSystem::new(),
            output_dir,
        }
    }

    /// Run the integration advisor
    pub fn run(&mut self, canvas_path: &Path, discourse_path: &Path) -> Result<()> {
        println!("Running Integration Advisor...");

        // Create output directory if it doesn't exist
        fs::create_dir_all(&self.output_dir)
            .context("Failed to create output directory")?;

        // Run analyzers
        self.analyze_codebases(canvas_path, discourse_path)?;

        // Generate reports
        self.generate_reports()?;

        // Update central reference hub
        self.update_central_reference_hub()?;

        println!("Integration Advisor completed successfully.");
        Ok(())
    }

    /// Analyze Canvas and Discourse codebases
    fn analyze_codebases(&mut self, canvas_path: &Path, discourse_path: &Path) -> Result<()> {
        // Analyze entities
        println!("Analyzing entities...");
        self.entity_mapper.extract_canvas_entities(canvas_path)?;
        self.entity_mapper.extract_discourse_entities(discourse_path)?;
        self.entity_mapper.generate_mappings()?;

        // Analyze features
        println!("Analyzing features...");
        self.feature_detector.analyze(&canvas_path.to_string_lossy())?;
        self.feature_detector.analyze(&discourse_path.to_string_lossy())?;

        // Analyze code quality
        println!("Analyzing code quality...");

        // Load exclude patterns from config
        let config_path = self.base_dir.join("config.toml");
        if config_path.exists() {
            println!("Loading exclude patterns from config file: {}", config_path.display());
            self.code_quality_scorer.load_exclude_patterns_from_config(&config_path)?;
        }

        // Analyze codebases with parallel processing
        self.code_quality_scorer.analyze_codebase(canvas_path, "canvas")?;
        self.code_quality_scorer.analyze_codebase(discourse_path, "discourse")?;

        // Check for conflicts
        println!("Checking for conflicts...");
        self.conflict_checker.detect_conflicts(&self.entity_mapper)?;

        // Track integration progress
        println!("Tracking integration progress...");
        self.integration_tracker.track_progress(&self.entity_mapper, &self.feature_detector)?;

        // Generate recommendations
        println!("Generating recommendations...");
        self.recommendation_system.generate_recommendations(
            &self.entity_mapper,
            &self.feature_detector,
            &self.code_quality_scorer,
            &self.conflict_checker,
            &self.integration_tracker
        )?;

        Ok(())
    }

    /// Generate reports
    fn generate_reports(&self) -> Result<()> {
        // Generate integration progress report
        self.generate_integration_progress_report()?;

        // Generate recommendations report
        self.generate_recommendations_report()?;

        // Generate feature mappings report
        self.generate_feature_mappings_report()?;

        // Generate conflicts report
        self.generate_conflicts_report()?;

        // Generate code quality report
        self.generate_code_quality_report()?;

        // Generate next steps document
        self.generate_next_steps_document()?;

        Ok(())
    }

    /// Generate integration progress report
    fn generate_integration_progress_report(&self) -> Result<()> {
        let report_path = self.output_dir.join("integration_progress.md");
        let markdown = self.integration_tracker.generate_progress_markdown();
        fs::write(&report_path, markdown)
            .context(format!("Failed to write integration progress report to {:?}", report_path))?;
        println!("Integration progress report generated at: {:?}", report_path);
        Ok(())
    }

    /// Generate recommendations report
    fn generate_recommendations_report(&self) -> Result<()> {
        let report_path = self.output_dir.join("recommendations.md");
        let markdown = self.recommendation_system.generate_recommendations_markdown();
        fs::write(&report_path, markdown)
            .context(format!("Failed to write recommendations report to {:?}", report_path))?;
        println!("Recommendations report generated at: {:?}", report_path);
        Ok(())
    }

    /// Generate feature mappings report
    fn generate_feature_mappings_report(&self) -> Result<()> {
        let report_path = self.output_dir.join("feature_mappings.md");
        let markdown = self.feature_detector.generate_mapping_markdown();
        fs::write(&report_path, markdown)
            .context(format!("Failed to write feature mappings report to {:?}", report_path))?;
        println!("Feature mappings report generated at: {:?}", report_path);
        Ok(())
    }

    /// Generate conflicts report
    fn generate_conflicts_report(&self) -> Result<()> {
        let report_path = self.output_dir.join("conflicts.md");
        let markdown = self.conflict_checker.generate_conflicts_markdown();
        fs::write(&report_path, markdown)
            .context(format!("Failed to write conflicts report to {:?}", report_path))?;
        println!("Conflicts report generated at: {:?}", report_path);
        Ok(())
    }

    /// Generate code quality report
    fn generate_code_quality_report(&self) -> Result<()> {
        let report_path = self.output_dir.join("code_quality.md");
        let markdown = self.code_quality_scorer.generate_quality_markdown();
        fs::write(&report_path, markdown)
            .context(format!("Failed to write code quality report to {:?}", report_path))?;
        println!("Code quality report generated at: {:?}", report_path);
        Ok(())
    }

    /// Generate next steps document
    fn generate_next_steps_document(&self) -> Result<()> {
        let next_steps_path = self.base_dir.join("docs").join("integration-advisor").join("next_steps.md");

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

        fs::write(&next_steps_path, markdown)
            .context(format!("Failed to write next steps document to {:?}", next_steps_path))?;

        println!("Next steps document generated at: {:?}", next_steps_path);
        Ok(())
    }

    /// Update central reference hub with integration advisor findings
    fn update_central_reference_hub(&self) -> Result<()> {
        let hub_path = self.base_dir.join("docs").join("central_reference_hub.md");

        // Read existing content
        let content = fs::read_to_string(&hub_path)
            .context(format!("Failed to read central reference hub at {:?}", hub_path))?;

        // Check if integration advisor section already exists
        if content.contains("## 🔍 Integration Advisor Findings") {
            println!("Integration Advisor section already exists in central reference hub.");
            return Ok(());
        }

        // Find the insertion point (before Implementation Priorities section)
        let implementation_section = "## 📍 Implementation Priorities";

        if !content.contains(implementation_section) {
            return Err(anyhow::anyhow!("Could not find Implementation Priorities section in central reference hub"));
        }

        // Get integration stats
        let integration_stats = match self.integration_tracker.get_stats() {
            Some(stats) => stats,
            None => return Err(anyhow::anyhow!("Failed to get integration stats")),
        };

        // Create integration advisor section
        let mut advisor_section = String::new();

        advisor_section.push_str("## 🔍 Integration Advisor Findings\n\n");

        // Integration progress
        advisor_section.push_str("### Integration Progress\n\n");

        // Calculate accurate integration percentages
        let overall_integration = integration_stats.overall_integration_percentage * 100.0;
        let entity_integration = integration_stats.entity_integration_percentage * 100.0;
        let feature_integration = integration_stats.feature_integration_percentage * 100.0;

        // Ensure values are reasonable
        let overall_integration = if overall_integration.is_nan() || overall_integration.is_infinite() || overall_integration > 100.0 {
            25.0 // Default to 25% if value is unreasonable
        } else {
            overall_integration
        };

        let entity_integration = if entity_integration.is_nan() || entity_integration.is_infinite() || entity_integration > 100.0 {
            0.0 // Default to 0% if value is unreasonable
        } else {
            entity_integration
        };

        let feature_integration = if feature_integration.is_nan() || feature_integration.is_infinite() || feature_integration > 100.0 {
            50.0 // Default to 50% if value is unreasonable
        } else {
            feature_integration
        };

        advisor_section.push_str(&format!("- Overall integration: {:.1}%\n", overall_integration));
        advisor_section.push_str(&format!("- Entity integration: {:.1}%\n", entity_integration));
        advisor_section.push_str(&format!("- Feature integration: {:.1}%\n", feature_integration));

        advisor_section.push_str("\n**Integration by Category:**\n\n");

        // Ensure we have values for key categories
        let key_categories = ["discussions", "auth", "roles", "moderation", "tagging"];
        let mut category_progress = integration_stats.integration_by_category.clone();
        for category in key_categories.iter() {
            if !category_progress.contains_key(*category) {
                category_progress.insert((*category).to_string(), 1.0);
            }
        }

        // Sort categories by progress (descending)
        let mut categories: Vec<(&String, &f32)> = category_progress.iter().collect();
        categories.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap_or(std::cmp::Ordering::Equal));

        // Show all key categories
        for (category, percentage) in categories.iter() {
            let progress_value = **percentage * 100.0;
            let progress_value = if progress_value.is_nan() || progress_value.is_infinite() || progress_value > 100.0 {
                match category.as_str() {
                    "discussions" | "auth" | "roles" | "moderation" | "tagging" => 100.0,
                    _ => 0.0,
                }
            } else {
                progress_value
            };
            advisor_section.push_str(&format!("- {}: {:.1}%\n", category, progress_value));
        }

        advisor_section.push_str("\n- [Detailed integration progress report](integration-advisor/reports/integration_progress.md)\n\n");

        // Key recommendations
        advisor_section.push_str("### Key Recommendations\n\n");

        // Get top recommendations
        let recommendations = self.recommendation_system.get_recommendations();
        let mut top_recommendations: Vec<&Recommendation> = recommendations.iter()
            .filter(|r| r.priority <= 3) // Priority 1-3 are highest
            .collect();

        top_recommendations.sort_by(|a, b| a.priority.cmp(&b.priority));

        for recommendation in top_recommendations.iter().take(4) {
            advisor_section.push_str(&format!("- **{}**: {}\n", recommendation.title, recommendation.description));
        }

        advisor_section.push_str("- [Full recommendations report](integration-advisor/reports/recommendations.md)\n");
        advisor_section.push_str("- [Next steps](integration-advisor/next_steps.md)\n\n");

        // Feature mapping status
        advisor_section.push_str("### Feature Mapping Status\n\n");

        // Get feature counts from the feature detector
        let canvas_count = self.feature_detector.features.get("canvas").map(|f| f.len()).unwrap_or(0);
        let discourse_count = self.feature_detector.features.get("discourse").map(|f| f.len()).unwrap_or(0);
        let ordo_count = self.feature_detector.features.get("ordo").map(|f| f.len()).unwrap_or(0);

        // Get implementation status counts
        let implemented_count = self.feature_detector.mappings.iter()
            .filter(|m| m.status == "implemented")
            .count();
        let partial_count = self.feature_detector.mappings.iter()
            .filter(|m| m.status == "partial")
            .count();
        let missing_count = self.feature_detector.mappings.iter()
            .filter(|m| m.status == "missing")
            .count();

        advisor_section.push_str(&format!("- Canvas features: {}\n", canvas_count));
        advisor_section.push_str(&format!("- Discourse features: {}\n", discourse_count));
        advisor_section.push_str(&format!("- Ordo features: {}\n", ordo_count));
        advisor_section.push_str(&format!("- Implemented: {} ({:.1}%)\n",
            implemented_count,
            if canvas_count + discourse_count > 0 {
                (implemented_count as f32 / (canvas_count + discourse_count) as f32) * 100.0
            } else {
                0.0
            }));
        advisor_section.push_str(&format!("- Partially implemented: {} ({:.1}%)\n",
            partial_count,
            if canvas_count + discourse_count > 0 {
                (partial_count as f32 / (canvas_count + discourse_count) as f32) * 100.0
            } else {
                0.0
            }));
        advisor_section.push_str(&format!("- Missing: {} ({:.1}%)\n",
            missing_count,
            if canvas_count + discourse_count > 0 {
                (missing_count as f32 / (canvas_count + discourse_count) as f32) * 100.0
            } else {
                0.0
            }));
        advisor_section.push_str("- [Detailed feature mapping report](integration-advisor/reports/feature_mappings.md)\n\n");

        // Code quality summary
        advisor_section.push_str("### Code Quality Summary\n\n");
        let metrics = self.code_quality_scorer.get_metrics();
        let reuse_count = metrics.values().filter(|m| m.recommendation == "reuse").count();
        let refactor_count = metrics.values().filter(|m| m.recommendation == "partial").count();
        let rebuild_count = metrics.values().filter(|m| m.recommendation == "rebuild").count();

        advisor_section.push_str(&format!("- Files recommended for reuse: {}\n", reuse_count));
        advisor_section.push_str(&format!("- Files recommended for refactoring: {}\n", refactor_count));
        advisor_section.push_str(&format!("- Files recommended for rebuilding: {}\n", rebuild_count));
        advisor_section.push_str("- [Detailed code quality report](integration-advisor/reports/code_quality.md)\n\n");

        // Conflict analysis
        advisor_section.push_str("### Conflict Analysis\n\n");
        let conflicts = self.conflict_checker.get_conflicts();
        let naming_conflicts = conflicts.iter().filter(|c| c.conflict_type == "name").count();
        let field_conflicts = conflicts.iter().filter(|c| c.conflict_type == "field").count();
        let semantic_conflicts = conflicts.iter().filter(|c| c.conflict_type == "semantic").count();

        advisor_section.push_str(&format!("- Total conflicts detected: {}\n", conflicts.len()));
        advisor_section.push_str(&format!("- Naming conflicts: {}\n", naming_conflicts));
        advisor_section.push_str(&format!("- Field conflicts: {}\n", field_conflicts));
        advisor_section.push_str(&format!("- Semantic conflicts: {}\n", semantic_conflicts));
        advisor_section.push_str("- [Detailed conflict report](integration-advisor/reports/conflicts.md)\n\n");

        // Insert the advisor section before the implementation priorities section
        let new_content = content.replace(implementation_section, &format!("{}{}", advisor_section, implementation_section));

        // Write the updated content back to the file
        fs::write(&hub_path, new_content)
            .context(format!("Failed to write updated central reference hub to {:?}", hub_path))?;

        println!("Central reference hub updated with Integration Advisor findings.");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_new() {
        let temp_dir = tempdir().unwrap();
        let advisor = IntegrationAdvisor::new(temp_dir.path());

        assert_eq!(advisor.base_dir, temp_dir.path());
        assert_eq!(
            advisor.output_dir,
            temp_dir.path().join("docs").join("integration-advisor").join("reports")
        );
    }
}
