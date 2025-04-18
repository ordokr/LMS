# Conflict Analysis Report

## Summary

- Total Conflicts: 12
- Naming Conflicts: 5
- Field Conflicts: 4
- Semantic Conflicts: 3

## Naming Conflicts

| Entity 1 | Entity 2 | Description | Suggested Resolution |
|---------|----------|-------------|----------------------|
| canvas.Discussion | discourse.Topic | Canvas and Discourse both have entities that represent discussion threads but with different names | Map both to a unified Discussion entity in Ordo with combined fields |
| canvas.DiscussionEntry | discourse.Post | Canvas and Discourse both have entities that represent discussion replies but with different names | Map both to a unified DiscussionPost entity in Ordo with combined fields |
| canvas.User | discourse.User | Canvas and Discourse both have User entities with different field structures | Create a unified User model that combines fields from both sources |
| canvas.Attachment | discourse.Upload | Both represent file uploads but with different naming | Create a unified Attachment model with combined fields |
| canvas.Announcement | discourse.Announcement | Same name but different structures and purposes | Keep separate as CourseAnnouncement and ForumAnnouncement |

## Field Conflicts

| Entity 1 | Entity 2 | Description | Suggested Resolution |
|---------|----------|-------------|----------------------|
| canvas.User.sortable_name | discourse.User.username | Different fields used for display sorting | Use username as primary and sortable_name as secondary property |
| canvas.Discussion.title | discourse.Topic.title | Same field name but different length constraints | Use the more restrictive constraint (discourse) |
| canvas.DiscussionEntry.message | discourse.Post.raw | Different field names for the same content | Standardize on 'content' for the unified model |
| canvas.Attachment.display_name | discourse.Upload.original_filename | Different field names for the same content | Standardize on 'filename' for the unified model |

## Semantic Conflicts

| Entity 1 | Entity 2 | Description | Suggested Resolution |
|---------|----------|-------------|----------------------|
| canvas.Group | discourse.Group | Both called 'Group' but serve different purposes | Rename to CourseGroup and ForumGroup in the domain model |
| canvas.Course | discourse.Category | Mapping between Course and Category has conceptual differences | Create a CourseCategory relationship with clear boundaries |
| canvas.Role | discourse.TrustLevel | Different approaches to user permissions | Implement a unified permission system with role and trust level components |
