# Reports Guide

The Unified Analyzer generates various reports to provide insights into the codebase and guide the development process.

## Report Types

The following report types are generated:

- **Summary Report**: A high-level summary of the analysis results
- **Entity Mapping Report**: Mapping of data models between Canvas, Discourse, and Ordo
- **Feature Mapping Report**: Mapping of features between Canvas, Discourse, and Ordo
- **Code Quality Report**: Analysis of code quality and areas for improvement
- **Conflict Report**: Detection of naming conflicts and semantic inconsistencies
- **Integration Progress Report**: Tracking of integration progress
- **Recommendation Report**: Prioritized development recommendations

## Report Formats

Reports are generated in the following formats:

- **Markdown**: Human-readable reports in Markdown format
- **JSON**: Machine-readable reports in JSON format

## Report Location

Reports are generated in the `reports` directory by default. This can be configured in the `config.toml` file.

## Summary Report

The Summary Report provides a high-level overview of the analysis results, including:

- Number of entities and features analyzed
- Integration progress
- Top recommendations
- Key conflicts
- Code quality summary

Example:

```markdown
# Summary Report

## Overview

- Canvas Entities: 42
- Discourse Entities: 35
- Ordo Entities: 28
- Canvas Features: 128
- Discourse Features: 95
- Ordo Features: 86
- Overall Integration Progress: 52.5%

## Top Recommendations

1. Implement Canvas Entity: QuizQuestion
2. Implement Canvas Feature: quiz_create_route
3. Implement Canvas Feature: quiz_take_route
4. Improve Integration for Category: assignment_mgmt
5. Resolve Name Conflict: canvas.User and discourse.User

## Key Conflicts

- Name Conflict: canvas.User and discourse.User
- Name Conflict: canvas.Group and discourse.Group
- Field Conflict: canvas.Assignment.due_date and ordo.Assignment.deadline

## Code Quality

- High Quality Files: 142 (58.0%)
- Medium Quality Files: 78 (31.8%)
- Low Quality Files: 25 (10.2%)
- Average Usefulness Score: 72.5
```

## Entity Mapping Report

The Entity Mapping Report provides details on the mapping of data models between Canvas, Discourse, and Ordo, including:

- Summary statistics
- Mapped entities with confidence scores
- Unmapped entities

See [Entity Mappings Example](example_reports/entity_mappings.md) for a complete example.

## Feature Mapping Report

The Feature Mapping Report provides details on the mapping of features between Canvas, Discourse, and Ordo, including:

- Summary statistics
- Implementation status
- Feature mappings by category
- Missing features by priority

See [Feature Mappings Example](example_reports/feature_mappings.md) for a complete example.

## Code Quality Report

The Code Quality Report provides details on the code quality of the analyzed codebases, including:

- Summary statistics
- Quality distribution by source
- Files needing improvement
- Complexity analysis
- Recommendations

See [Code Quality Example](example_reports/code_quality.md) for a complete example.

## Conflict Report

The Conflict Report provides details on naming conflicts and semantic inconsistencies between Canvas, Discourse, and Ordo, including:

- Summary statistics
- Name conflicts
- Field conflicts
- Semantic conflicts

See [Conflicts Example](example_reports/conflicts.md) for a complete example.

## Integration Progress Report

The Integration Progress Report provides details on the progress of the integration effort, including:

- Overall progress
- Entity integration progress
- Feature integration progress
- Progress by category

See [Integration Progress Example](example_reports/integration_progress.md) for a complete example.

## Recommendation Report

The Recommendation Report provides prioritized development recommendations, including:

- Summary statistics
- High priority recommendations
- Medium priority recommendations
- Low priority recommendations

See [Recommendations Example](example_reports/recommendations.md) for a complete example.

## Next Steps

The `next_steps.md` file is generated in the root directory with prioritized recommendations. This file is a copy of the Recommendation Report and is intended to be a quick reference for the next steps in the development process.
