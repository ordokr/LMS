# Course Model

## Overview

Canvas Course model representing a course in the LMS

System: canvas

## Properties

| Name | Type | Description |
|------|------|-------------|
| id | integer | Unique identifier |
| name | string | Course name |
| code | string | Course code |
| workflow_state | string | Current state of the course |

## Relationships

| Name | Type | Target | Description |
|------|------|--------|-------------|
| enrollments | has_many | Enrollment | Student enrollments |
| discussion_topics | has_many | DiscussionTopic | Discussion topics |
