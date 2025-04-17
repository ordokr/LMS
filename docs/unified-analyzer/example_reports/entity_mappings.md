# Entity Mapping Report

## Summary

- Canvas Entities: 42
- Discourse Entities: 35
- Ordo Entities: 28
- Total Mappings: 53

## Mapped Entities

| Source Entity | Target Entity | Confidence | Mapped Fields | Unmapped Source Fields | Unmapped Target Fields |
|--------------|--------------|------------|--------------|------------------------|------------------------|
| canvas.Course | ordo.Course | 0.85 | 12 | 3 | 2 |
| canvas.Assignment | ordo.Assignment | 0.92 | 15 | 1 | 0 |
| canvas.User | ordo.User | 0.78 | 8 | 4 | 2 |
| canvas.Submission | ordo.Submission | 0.90 | 10 | 2 | 1 |
| canvas.Enrollment | ordo.Enrollment | 0.95 | 6 | 0 | 1 |
| discourse.Topic | ordo.Discussion | 0.72 | 7 | 5 | 2 |
| discourse.Post | ordo.Post | 0.88 | 9 | 2 | 1 |
| discourse.User | ordo.User | 0.65 | 6 | 8 | 2 |
| discourse.Category | ordo.Category | 0.80 | 5 | 3 | 1 |
| discourse.Tag | ordo.Tag | 0.95 | 3 | 0 | 0 |

## Unmapped Canvas Entities

| Entity | Fields |
|--------|--------|
| QuizQuestion | id, quiz_id, question_type, question_text, points_possible |
| GradingPeriod | id, title, start_date, end_date, weight |
| ContentMigration | id, migration_type, workflow_state, started_at, finished_at |
| CalendarEvent | id, title, description, start_at, end_at, location_name |

## Unmapped Discourse Entities

| Entity | Fields |
|--------|--------|
| Badge | id, name, description, badge_type_id, granted_count |
| Notification | id, notification_type, user_id, data, read, created_at |
| UserAction | id, action_type, user_id, target_user_id, acting_user_id |
| TopicTimer | id, execute_at, status_type, user_id, topic_id |
