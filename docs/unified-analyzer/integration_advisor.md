# Integration Advisor User Guide

## Overview

The Integration Advisor is a comprehensive tool designed to help with the integration of Canvas and Discourse functionality into the Ordo LMS system. It provides detailed analysis, mapping, and recommendations to guide the development process.

## Components

The Integration Advisor consists of several modules:

1. **Entity Mapper**: Maps data models between Canvas, Discourse, and Ordo
2. **Feature Detector**: Identifies and maps features between systems
3. **Code Quality Scorer**: Analyzes code quality and identifies areas for improvement
4. **Conflict Checker**: Detects naming conflicts and semantic inconsistencies
5. **Integration Tracker**: Tracks progress of the integration effort
6. **Recommendation System**: Generates prioritized development recommendations

## Commands

The Integration Advisor provides several commands:

- `integration-advisor`: Run the full Integration Advisor
- `entity-mapping`: Run only the Entity Mapper
- `feature-detection`: Run only the Feature Detector
- `code-quality`: Run only the Code Quality Scorer
- `conflict-detection`: Run only the Conflict Checker
- `integration-tracking`: Run only the Integration Tracker
- `recommendations`: Run only the Recommendation System

## Usage

To run the full Integration Advisor:

```
cargo run --bin unified-analyzer -- integration-advisor
```

To run individual components:

```
cargo run --bin unified-analyzer -- entity-mapping
cargo run --bin unified-analyzer -- feature-detection
cargo run --bin unified-analyzer -- code-quality
cargo run --bin unified-analyzer -- conflict-detection
cargo run --bin unified-analyzer -- integration-tracking
cargo run --bin unified-analyzer -- recommendations
```

## Configuration

The Integration Advisor can be configured in the `config.toml` file:

```toml
[analysis.integration_advisor]
enabled = true
entity_mapping = true
feature_detection = true
code_quality = true
conflict_detection = true
integration_tracking = true
recommendations = true

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

## Reports

The Integration Advisor generates both JSON and Markdown reports in the `reports` directory:

- `entity_mappings.json` and `entity_mappings.md`: Entity mapping reports
- `feature_mappings.json` and `feature_mappings.md`: Feature mapping reports
- `code_quality.json` and `code_quality.md`: Code quality reports
- `conflicts.json` and `conflicts.md`: Conflict detection reports
- `integration_progress.json` and `integration_progress.md`: Integration progress reports
- `recommendations.json` and `recommendations.md`: Development recommendations

Additionally, a `next_steps.md` file is generated in the root directory with prioritized recommendations.

## Workflow

1. Run the full Integration Advisor to get a comprehensive analysis
2. Review the generated reports
3. Implement the highest priority recommendations
4. Run the Integration Advisor again to track progress
5. Repeat until the integration is complete

## HelixDB Integration Plan (Analyzer-First Approach)

### Overview

This plan outlines the phased integration of HelixDB into the LMS project, with an initial focus on the analyzer components. The goal is to leverage HelixDB’s high-performance vector and hybrid search capabilities to enhance semantic analysis, code search, and recommendation features. App integration will only proceed after successful analyzer implementation and validation.

### Phase 1: Preparation

- Evaluate current analyzer data flows and identify where vector/hybrid search can provide value (e.g., entity mapping, feature detection, code similarity).
- Review HelixDB’s API, data model, and performance characteristics.
- Ensure Rust toolchain compatibility and readiness for adding HelixDB as a dependency.

### Phase 2: Analyzer Integration

1. **Dependency Setup**
   - Add HelixDB as a dependency in the analyzer’s Cargo.toml.
   - Build and test for compatibility.

2. **Embedding Generation**
   - Define a workflow for generating embeddings (e.g., using a local LLM via LM Studio).
   - Implement a Rust interface to communicate with the local LLM for embedding extraction.

3. **Data Ingestion**
   - Identify analyzer outputs (entities, features, code snippets) to be indexed.
   - Store embeddings and relevant metadata in HelixDB.

4. **Query Integration**
   - Update analyzer modules (e.g., Entity Mapper, Feature Detector) to use HelixDB for similarity and hybrid search.
   - Optimize queries for performance (batching, caching, etc.).

5. **Performance Tuning**
   - Benchmark indexing and query times.
   - Tune HelixDB configuration (e.g., index type, memory usage) for optimal analyzer performance.

6. **Validation**
   - Run full analyzer workflows and validate output accuracy and speed.
   - Compare results with previous implementation to ensure quality.

### Phase 3: Documentation & Review

- Document integration steps, configuration, and usage in the analyzer context.
- Gather feedback from analyzer users and stakeholders.
- Address any issues or bottlenecks.

## Troubleshooting

If you encounter issues:

1. Check the configuration in `config.toml`
2. Ensure the paths to Canvas and Discourse codebases are correct
3. Try running individual components to isolate the issue
4. Check the logs for error messages
