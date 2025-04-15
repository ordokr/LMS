# Unified Models Documentation

This document describes the unified model system used to bridge Canvas LMS and Discourse.

## Overview

The Canvas-Discourse integration uses a unified model approach to solve duplication issues and ensure consistency between both systems. Each entity in the system (User, Course, Discussion, Assignment) has a unified model that can represent it in both Canvas and Discourse formats.

## Model Structure

### User

Maps between Canvas User and Discourse User.

| Unified Field | Canvas Field | Discourse Field | Description |
|---------------|--------------|-----------------|-------------|
| id | id | id | Internal ID |
| email | email | email | User email |
| name | name | name | Full name |
| canvasId | id | - | Canvas-specific ID |
| discourseId | - | id | Discourse-specific ID |
| username | - | username | Username (generated from email if not set) |
| roles | enrollments | trust_level | User roles/privileges |

### Course

Maps between Canvas Course and Discourse Category.

| Unified Field | Canvas Field | Discourse Field | Description |
|---------------|--------------|-----------------|-------------|
| id | id | id | Internal ID |
| title | name | name | Course/category title |
| description | description | description | Course details |
| canvasId | id | - | Canvas-specific ID |
| discourseId | - | id | Discourse-specific ID |
| startDate | start_at | - | Course start date |
| endDate | end_at | - | Course end date |
| slug | - | slug | URL-friendly name |

### Discussion

Maps between Canvas Discussion and Discourse Topic.

| Unified Field | Canvas Field | Discourse Field | Description |
|---------------|--------------|-----------------|-------------|
| id | id | id | Internal ID |
| title | title | title | Discussion title |
| message | message | raw (first post) | Discussion content |
| canvasId | id | - | Canvas-specific ID |
| discourseId | - | id | Discourse-specific ID |
| courseId | course_id | - | Related Canvas course |
| categoryId | - | category_id | Related Discourse category |
| locked | locked | closed | Whether discussion is closed |

### Assignment

Maps between Canvas Assignment and Discourse Topic with custom fields.

| Unified Field | Canvas Field | Discourse Field | Description |
|---------------|--------------|-----------------|-------------|
| id | id | custom_fields.assignment_id | Internal ID |
| title | name | title | Assignment title |
| description | description | raw (first post) | Instructions |
| canvasId | id | - | Canvas-specific ID |
| discourseId | - | topic_id | Related Discourse topic |
| dueAt | due_at | custom_fields.due_at | Due date |
| pointsPossible | points_possible | custom_fields.points_possible | Maximum points |

## Using the Models

### Creating Models

Use the ModelFactory to create unified models from source system data:

```javascript
import { ModelFactory } from '../models';

// Create from Canvas data
const user = ModelFactory.create('user', canvasUserData, 'canvas');

// Create from Discourse data
const course = ModelFactory.create('course', discourseCategoryData, 'discourse');

Converting to Source System Format

// Convert to Canvas format
const canvasFormat = ModelFactory.convertToSource(user, 'canvas');

// Convert to Discourse format
const discourseFormat = ModelFactory.convertToSource(course, 'discourse');

Direct Model Usage
You can also use the model classes directly:

import { User, Course } from '../models';

// Create a user from Canvas data
const user = User.fromCanvasUser(canvasUserData);

// Convert to Discourse format
const discourseUser = user.toDiscourseUser();

Implementation Details
Each model provides:

Constructor to create a unified model from any data
Static methods to create from specific source (fromCanvasX, fromDiscourseX)
Methods to convert to specific source format (toCanvasX, toDiscourseX)
Auto-generated IDs, slugs, and usernames when needed

