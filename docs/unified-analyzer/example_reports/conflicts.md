# Entity Conflict Report

## Summary

- Total Conflicts: 12
- Name Conflicts: 3
- Field Conflicts: 7
- Semantic Conflicts: 2

## Name Conflicts

| Entity 1 | Entity 2 | Description | Suggested Resolution |
|----------|----------|-------------|----------------------|
| canvas.User | discourse.User | Both Canvas and Discourse have User entities with different field structures that map to the same Ordo entity | Create a unified User model that incorporates fields from both sources |
| canvas.Group | discourse.Group | Both Canvas and Discourse have Group entities with different purposes | Rename to CourseGroup and DiscussionGroup respectively |
| canvas.Tag | discourse.Tag | Both Canvas and Discourse have Tag entities with different structures | Create a unified Tag model with source-specific fields |

## Field Conflicts

| Entity 1 | Entity 2 | Description | Suggested Resolution |
|----------|----------|-------------|----------------------|
| canvas.Assignment.due_date | ordo.Assignment.deadline | Different field names for the same concept | Standardize on one field name or create aliases |
| canvas.Course.name | ordo.Course.title | Different field names for the same concept | Standardize on one field name or create aliases |
| canvas.User.sortable_name | ordo.User.display_name | Different field names for similar concepts | Map both to a common field or maintain both with clear documentation |
| discourse.Topic.title | ordo.Discussion.name | Different field names for the same concept | Standardize on one field name or create aliases |
| discourse.Post.raw | ordo.Post.content | Different field names for the same concept | Standardize on one field name or create aliases |
| canvas.Submission.submitted_at | ordo.Submission.created_at | Different field names for similar concepts | Map both to a common field or maintain both with clear documentation |
| discourse.Category.slug | ordo.Category.url_path | Different field names for the same concept | Standardize on one field name or create aliases |

## Semantic Conflicts

| Entity 1 | Entity 2 | Description | Suggested Resolution |
|----------|----------|-------------|----------------------|
| canvas.Discussion | discourse.Topic | Similar concepts with different structures and relationships | Create a unified Discussion model that can represent both sources |
| canvas.Role | discourse.TrustLevel | Different approaches to user permissions | Create a unified permission system that can represent both models |
