# AI Agent Guide: Upgrading Unified-Analyzer to Full Integration Advisor

## Overview
This guide provides step-by-step instructions for upgrading the existing unified-analyzer to a Full Integration Advisor that can analyze Canvas and Discourse codebases, map them to Ordo, and provide recommendations.

## Prerequisites
- Access to Canvas codebase at `C:\Users\Tim\Desktop\port\canvas`
- Access to Discourse codebase at `C:\Users\Tim\Desktop\port\discourse`
- Access to Ordo codebase at `C:\Users\Tim\Desktop\LMS`
- Existing unified-analyzer codebase

## Implementation Steps

### 1. Enhance Entity Mapper
The existing `entity_mapper.rs` already provides basic entity extraction. Enhance it with:

```rust
// Add to entity_mapper.rs
impl EntityMapper {
    // Add method to compare field similarity between entities
    pub fn calculate_field_similarity(&self, entity1: &NormalizedEntity, entity2: &NormalizedEntity) -> f32 {
        let mut matching_fields = 0;
        let total_fields = entity1.fields.len() + entity2.fields.len();
        
        for (field_name, _) in &entity1.fields {
            if entity2.fields.contains_key(field_name) {
                matching_fields += 1;
            }
        }
        
        if total_fields > 0 {
            (matching_fields * 2) as f32 / total_fields as f32
        } else {
            0.0
        }
    }
    
    // Add method to generate comprehensive mapping report
    pub fn generate_comprehensive_mapping_report(&self) -> String {
        let mut report = String::new();
        report.push_str("# Comprehensive Entity Mapping Report\n\n");
        
        // Add mapping statistics
        let mapped_count = self.mappings.len();
        let total_canvas = self.entities.get("canvas").map_or(0, |e| e.len());
        let total_discourse = self.entities.get("discourse").map_or(0, |e| e.len());
        let total_ordo = self.entities.get("ordo").map_or(0, |e| e.len());
        
        report.push_str("## Mapping Statistics\n\n");
        report.push_str(&format!("- Total Canvas Entities: {}\n", total_canvas));
        report.push_str(&format!("- Total Discourse Entities: {}\n", total_discourse));
        report.push_str(&format!("- Total Ordo Entities: {}\n", total_ordo));
        report.push_str(&format!("- Mapped Entities: {}\n", mapped_count));
        report.push_str(&format!("- Canvas Coverage: {:.1}%\n", 
            if total_canvas > 0 { 
                self.mappings.iter().filter(|m| m.source_entity.starts_with("canvas")).count() as f32 / total_canvas as f32 * 100.0 
            } else { 
                0.0 
            }));
        report.push_str(&format!("- Discourse Coverage: {:.1}%\n\n", 
            if total_discourse > 0 { 
                self.mappings.iter().filter(|m| m.source_entity.starts_with("discourse")).count() as f32 / total_discourse as f32 * 100.0 
            } else { 
                0.0 
            }));
        
        // Add detailed mappings
        report
    }
}
```

### 2. Create Feature & Module Detector

Create a new file `feature_detector.rs` in the `analyzers/modules` directory:

```rust
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::fs;
use serde::{Deserialize, Serialize};
use regex::Regex;
use anyhow::{Result, anyhow};
use walkdir::WalkDir;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Feature {
    /// Source system (canvas, discourse, ordo)
    pub source: String,
    /// Feature name
    pub name: String,
    /// Feature category
    pub category: String,
    /// Source files implementing this feature
    pub source_files: Vec<String>,
    /// Related entities
    pub related_entities: Vec<String>,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureMapping {
    /// Source feature (e.g., "canvas.course_creation")
    pub source_feature: String,
    /// Target feature (e.g., "ordo.course_creation")
    pub target_feature: String,
    /// Mapping confidence (0.0 to 1.0)
    pub confidence: f32,
    /// Implementation status (implemented, partial, missing)
    pub status: String,
    /// Priority (1-5, with 5 being highest)
    pub priority: u8,
}

/// Feature Detector for extracting and mapping features between systems
pub struct FeatureDetector {
    /// Extracted features by source system
    features: HashMap<String, Vec<Feature>>,
    /// Feature mappings between systems
    mappings: Vec<FeatureMapping>,
}

impl FeatureDetector {
    /// Create a new FeatureDetector
    pub fn new() -> Self {
        Self {
            features: HashMap::new(),
            mappings: Vec::new(),
        }
    }
    
    /// Extract features from Canvas codebase
    pub fn extract_canvas_features(&mut self, canvas_path: &Path) -> Result<()> {
        println!("Extracting features from Canvas codebase at: {}", canvas_path.display());
        
        // Look for controllers, routes, and views
        let controllers_dir = canvas_path.join("app").join("controllers");
        let routes_file = canvas_path.join("config").join("routes.rb");
        let views_dir = canvas_path.join("app").join("views");
        
        let mut features = Vec::new();
        
        // Extract features from controllers
        if controllers_dir.exists() {
            self.extract_ruby_controllers(&controllers_dir, "canvas", &mut features)?;
        }
        
        // Extract features from routes
        if routes_file.exists() {
            self.extract_ruby_routes(&routes_file, "canvas", &mut features)?;
        }
        
        // Extract features from views
        if views_dir.exists() {
            self.extract_ruby_views(&views_dir, "canvas", &mut features)?;
        }
        
        self.features.insert("canvas".to_string(), features);
        
        println!("Extracted {} Canvas features", 
            self.features.get("canvas").map(|f| f.len()).unwrap_or(0));
        
        Ok(())
    }
    
    // Additional methods for feature extraction and mapping
    // ...
}
```

### 3. Create Code Quality & Usefulness Scorer

Create a new file `code_quality_scorer.rs` in the `analyzers/modules` directory:

```rust
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;
use serde::{Deserialize, Serialize};
use anyhow::{Result, anyhow};
use walkdir::WalkDir;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeMetrics {
    /// File path
    pub file_path: String,
    /// Lines of code
    pub loc: usize,
    /// Cyclomatic complexity
    pub complexity: u32,
    /// Comment coverage (percentage)
    pub comment_coverage: f32,
    /// Cohesion score (0.0 to 1.0)
    pub cohesion: f32,
    /// Overall usefulness score (0 to 100)
    pub usefulness_score: u8,
    /// Recommendation (reuse, partial, rebuild)
    pub recommendation: String,
}

/// Code Quality Scorer for analyzing code quality and usefulness
pub struct CodeQualityScorer {
    /// Code metrics by file path
    metrics: HashMap<String, CodeMetrics>,
}

impl CodeQualityScorer {
    /// Create a new CodeQualityScorer
    pub fn new() -> Self {
        Self {
            metrics: HashMap::new(),
        }
    }
    
    /// Analyze code quality for a codebase
    pub fn analyze_codebase(&mut self, path: &Path, source: &str) -> Result<()> {
        println!("Analyzing code quality for {} codebase at: {}", source, path.display());
        
        self.walk_directory(path, |file_path| {
            if let Some(ext) = file_path.extension() {
                let ext_str = ext.to_string_lossy().to_lowercase();
                if ["rb", "js", "ts", "jsx", "tsx", "rs", "hs"].contains(&ext_str.as_str()) {
                    if let Ok(content) = fs::read_to_string(file_path) {
                        let metrics = self.calculate_metrics(file_path, &content);
                        self.metrics.insert(file_path.to_string_lossy().to_string(), metrics);
                    }
                }
            }
        })?;
        
        println!("Analyzed {} files for code quality", self.metrics.len());
        
        Ok(())
    }
    
    // Additional methods for code quality analysis
    // ...
}
```

### 4. Create Conflict & Overlap Checker

Create a new file `conflict_checker.rs` in the `analyzers/modules` directory:

```rust
use std::collections::{HashMap, HashSet};
use std::path::Path;
use serde::{Deserialize, Serialize};
use anyhow::{Result, anyhow};

use crate::analyzers::modules::entity_mapper::{EntityMapper, NormalizedEntity};

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
}

impl ConflictChecker {
    /// Create a new ConflictChecker
    pub fn new() -> Self {
        Self {
            conflicts: Vec::new(),
        }
    }
    
    /// Detect conflicts between entities
    pub fn detect_conflicts(&mut self, entity_mapper: &EntityMapper) -> Result<()> {
        println!("Detecting conflicts between entities...");
        
        // Get all entities
        let all_entities = entity_mapper.get_entities();
        
        // Check for naming conflicts
        self.detect_naming_conflicts(all_entities)?;
        
        // Check for semantic conflicts
        self.detect_semantic_conflicts(all_entities, entity_mapper.get_mappings())?;
        
        println!("Detected {} conflicts", self.conflicts.len());
        
        Ok(())
    }
    
    // Additional methods for conflict detection
    // ...
}
```

### 5. Create Integration Progress Tracker

Create a new file `integration_tracker.rs` in the `analyzers/modules` directory:

```rust
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use anyhow::{Result, anyhow};

use crate::analyzers::modules::entity_mapper::{EntityMapper, EntityMapping};
use crate::analyzers::modules::feature_detector::{FeatureDetector, FeatureMapping};

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
}

impl IntegrationTracker {
    /// Create a new IntegrationTracker
    pub fn new() -> Self {
        Self {
            progress: IntegrationProgress {
                entity_progress: HashMap::new(),
                feature_progress: HashMap::new(),
                overall_progress: 0.0,
                category_progress: HashMap::new(),
            },
        }
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
    
    // Additional methods for tracking integration progress
    // ...
}
```

### 6. Create Recommendation System

Create a new file `recommendation_system.rs` in the `analyzers/modules` directory:

```rust
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
}

impl RecommendationSystem {
    /// Create a new RecommendationSystem
    pub fn new() -> Self {
        Self {
            recommendations: Vec::new(),
        }
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
        
        println!("Generated {} recommendations", self.recommendations.len());
        
        Ok(())
    }
    
    // Additional methods for generating recommendations
    // ...
}
```

### 7. Update Main Integration Function

Update the main integration function in `main.rs` to include the new components:

```rust
async fn run_full_integration_advisor(base_dir: &PathBuf, config: &Config) -> Result<()> {
    println!("---- Starting Full Integration Advisor ----");
    
    // Get paths to Canvas, Discourse, and Ordo codebases
    let canvas_path = match config.get_path("canvas_path") {
        Some(path) => PathBuf::from(path),
        None => PathBuf::from("C:\\Users\\Tim\\Desktop\\port\\canvas"),
    };
    
    let discourse_path = match config.get_path("discourse_path") {
        Some(path) => PathBuf::from(path),
        None => PathBuf::from("C:\\Users\\Tim\\Desktop\\port\\discourse"),
    };
    
    let ordo_path = match config.get_path("lms_path") {
        Some(path) => PathBuf::from(path),
        None => base_dir.clone(),
    };
    
    // Initialize components
    let mut entity_mapper = EntityMapper::new();
    let mut feature_detector = FeatureDetector::new();
    let mut code_quality_scorer = CodeQualityScorer::new();
    let mut conflict_checker = ConflictChecker::new();
    let mut integration_tracker = IntegrationTracker::new();
    let mut recommendation_system = RecommendationSystem::new();
    
    // Extract entities
    println!("Extracting entities...");
    entity_mapper.extract_canvas_entities(&canvas_path)?;
    entity_mapper.extract_discourse_entities(&discourse_path)?;
    entity_mapper.extract_ordo_entities(&ordo_path)?;
    entity_mapper.generate_mappings()?;
    
    // Extract features
    println!("Extracting features...");
    feature_detector.extract_canvas_features(&canvas_path)?;
    feature_detector.extract_discourse_features(&discourse_path)?;
    feature_detector.extract_ordo_features(&ordo_path)?;
    feature_detector.generate_mappings()?;
    
    // Analyze code quality
    println!("Analyzing code quality...");
    code_quality_scorer.analyze_codebase(&canvas_path, "canvas")?;
    code_quality_scorer.analyze_codebase(&discourse_path, "discourse")?;
    
    // Detect conflicts
    println!("Detecting conflicts...");
    conflict_checker.detect_conflicts(&entity_mapper)?;
    
    // Track integration progress
    println!("Tracking integration progress...");
    integration_tracker.track_progress(&entity_mapper, &feature_detector)?;
    
    // Generate recommendations
    println!("Generating recommendations...");
    recommendation_system.generate_recommendations(
        &entity_mapper,
        &feature_detector,
        &code_quality_scorer,
        &conflict_checker,
        &integration_tracker
    )?;
    
    // Generate reports
    println!("Generating reports...");
    generate_entity_mapping_report(&entity_mapper, &ordo_path)?;
    generate_feature_mapping_report(&feature_detector, &ordo_path)?;
    generate_code_quality_report(&code_quality_scorer, &ordo_path)?;
    generate_conflict_report(&conflict_checker, &ordo_path)?;
    generate_integration_progress_report(&integration_tracker, &ordo_path)?;
    generate_recommendation_report(&recommendation_system, &ordo_path)?;
    
    println!("---- Full Integration Advisor Completed ----");
    
    Ok(())
}
```

### 8. Add Command-Line Interface

Update the command-line interface in `main.rs` to include the new functionality:

```rust
// Add to the match statement in main()
match command {
    // ...existing commands...
    "integration-advisor" => {
        println!("Running Full Integration Advisor...");
        run_full_integration_advisor(&base_dir, &config).await?
    },
    "entity-mapping" => {
        println!("Running Entity Mapper...");
        run_entity_mapper(&base_dir, &config).await?
    },
    "feature-detection" => {
        println!("Running Feature Detector...");
        run_feature_detector(&base_dir, &config).await?
    },
    "code-quality" => {
        println!("Running Code Quality Scorer...");
        run_code_quality_scorer(&base_dir, &config).await?
    },
    "conflict-detection" => {
        println!("Running Conflict Checker...");
        run_conflict_checker(&base_dir, &config).await?
    },
    "integration-tracking" => {
        println!("Running Integration Tracker...");
        run_integration_tracker(&base_dir, &config).await?
    },
    "recommendations" => {
        println!("Running Recommendation System...");
        run_recommendation_system(&base_dir, &config).await?
    },
    // ...existing commands...
}
```

### 9. Update Configuration

Update `config.toml` to include the new components:

```toml
# Add to the [analysis] section
[analysis.integration_advisor]
enabled = true
entity_mapping = true
feature_detection = true
code_quality = true
conflict_detection = true
integration_tracking = true
recommendations = true

# Add new sections for each component
[analysis.entity_mapping]
similarity_threshold = 0.5
exact_match_bonus = 0.3
field_match_weight = 0.7
name_match_weight = 0.3

[analysis.feature_detection]
categories = ["course_mgmt", "assignment_mgmt", "grading", "discussions", "auth", "roles", "moderation", "tagging"]
controller_weight = 0.5
route_weight = 0.3
view_weight = 0.2

[analysis.code_quality]
usefulness_threshold_high = 80
usefulness_threshold_medium = 50
complexity_weight = 0.4
loc_weight = 0.2
comment_coverage_weight = 0.2
cohesion_weight = 0.2

[analysis.conflict_detection]
naming_conflict_threshold = 0.8
semantic_conflict_threshold = 0.6

[analysis.integration_tracking]
entity_weight = 0.5
feature_weight = 0.5

[analysis.recommendations]
max_recommendations = 20
high_priority_threshold = 4
medium_priority_threshold = 2
```

## Implementation Order

1. Start with enhancing the `entity_mapper.rs` file
2. Implement the `feature_detector.rs` module
3. Implement the `code_quality_scorer.rs` module
4. Implement the `conflict_checker.rs` module
5. Implement the `integration_tracker.rs` module
6. Implement the `recommendation_system.rs` module
7. Update the main integration function in `main.rs`
8. Update the command-line interface in `main.rs`
9. Update the configuration in `config.toml`

## Testing

After implementing each component, test it individually:

```bash
cargo run -- entity-mapping
cargo run -- feature-detection
cargo run -- code-quality
cargo run -- conflict-detection
cargo run -- integration-tracking
cargo run -- recommendations
cargo run -- integration-advisor
```

## Expected Outputs

The Full Integration Advisor should generate the following outputs:

1. `mapped_models.json` - Entity mappings between Canvas, Discourse, and Ordo
2. `unmapped_models.md` - Entities not yet mapped to Ordo
3. `feature_coverage_report.md` - Feature coverage analysis
4. `missing_features.json` - Features not yet implemented in Ordo
5. `reuse_or_rebuild_map.json` - Recommendations for reusing or rebuilding code
6. `integration_quality_summary.md` - Summary of code quality analysis
7. `conflict_analysis.md` - Analysis of naming and semantic conflicts
8. `integration_dashboard.json` - Integration progress tracking
9. `next_steps.md` - Prioritized recommendations for next steps
