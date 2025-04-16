|
# Unified Data Models

This document outlines the proposed unified data models for the integrated platform, combining elements from Canvas and Discourse.

## User

| Field          | Type     | Description                               | Source(s)        |
|----------------|----------|-------------------------------------------|-------------------|
| id             | UUID     | Unique identifier                         | Canvas, Discourse |
| username       | String   | User's login name                         | Canvas, Discourse |
| email          | String   | User's email address                      | Canvas, Discourse |
| name           | String   | User's full name                            | Canvas, Discourse |
| avatar_url     | String   | URL of user's avatar image                | Canvas, Discourse |
| created_at     | DateTime | Timestamp of user creation                | Canvas, Discourse |
| updated_at     | DateTime | Timestamp of last user update             | Canvas, Discourse |
| canvas_id      | Integer  | User ID in Canvas (if applicable)         | Canvas            |
| discourse_id   | Integer  | User ID in Discourse (if applicable)      | Discourse         |

## Course/Topic

| Field             | Type     | Description                                  | Source(s)        |
|-------------------|----------|----------------------------------------------|-------------------|
| id                | UUID     | Unique identifier                            | Canvas, Discourse |
| title             | String   | Course/Topic title                             | Canvas, Discourse |
| description       | String   | Course/Topic description                        | Canvas, Discourse |
| created_at        | DateTime | Timestamp of Course/Topic creation             | Canvas, Discourse |
| updated_at        | DateTime | Timestamp of last Course/Topic update          | Canvas, Discourse |
| canvas_id         | Integer  | Course ID in Canvas (if applicable)          | Canvas            |
| discourse_id      | Integer  | Topic ID in Discourse (if applicable)       | Discourse         |
| instructor_id     | UUID     | User ID of the instructor/creator            | Canvas, Discourse |
| start_date        | DateTime | Course start date                              | Canvas            |
| end_date          | DateTime | Course end date                                | Canvas            |
| category          | String   | Category of the course/topic                   | Canvas, Discourse |

## Discussion/Post

| Field          | Type     | Description                               | Source(s)        |
|----------------|----------|-------------------------------------------|-------------------|
| id             | UUID     | Unique identifier                         | Canvas, Discourse |
| title          | String   | Discussion/Post title                       | Canvas, Discourse |
| content        | String   | Discussion/Post content                     | Canvas, Discourse |
| author_id      | UUID     | User ID of the author                     | Canvas, Discourse |
| created_at     | DateTime | Timestamp of Discussion/Post creation     | Canvas, Discourse |
| updated_at     | DateTime | Timestamp of last Discussion/Post update  | Canvas, Discourse |
| course_id      | UUID     | Course ID to which this Discussion belongs | Canvas, Discourse |
| topic_id       | UUID     | Topic ID to which this Post belongs        | Canvas, Discourse |
| canvas_id      | Integer  | Discussion ID in Canvas (if applicable)   | Canvas            |
| discourse_id   | Integer  | Post ID in Discourse (if applicable)        | Discourse         |