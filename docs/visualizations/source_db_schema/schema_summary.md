# Database Schema Summary

## Overview

- Total tables: 9
- Total relationships: 9
- Canvas tables: 4
- Discourse tables: 5

## Tables

### Canvas Tables

#### assignments

| Column | Type |
|--------|------|
| id | integer |
| title | string |
| description | text |
| course_id | integer |
| points_possible | float |
| due_at | datetime |
| created_at | datetime |
| updated_at | datetime |

#### courses

| Column | Type |
|--------|------|
| id | integer |
| name | string |
| account_id | integer |
| root_account_id | integer |
| enrollment_term_id | integer |
| created_at | datetime |
| updated_at | datetime |

#### enrollments

| Column | Type |
|--------|------|
| id | integer |
| user_id | integer |
| course_id | integer |
| type | string |
| created_at | datetime |
| updated_at | datetime |

#### submissions

| Column | Type |
|--------|------|
| id | integer |
| assignment_id | integer |
| user_id | integer |
| grade | string |
| score | float |
| submitted_at | datetime |
| created_at | datetime |
| updated_at | datetime |

### Discourse Tables

#### posts

| Column | Type |
|--------|------|
| id | integer |
| topic_id | integer |
| user_id | integer |
| raw | text |
| cooked | text |
| created_at | datetime |
| updated_at | datetime |

#### tags

| Column | Type |
|--------|------|
| id | integer |
| name | string |
| created_at | datetime |
| updated_at | datetime |

#### users

| Column | Type |
|--------|------|
| id | integer |
| username | string |
| name | string |
| email | string |
| created_at | datetime |
| updated_at | datetime |

#### topics

| Column | Type |
|--------|------|
| id | integer |
| title | string |
| user_id | integer |
| category_id | integer |
| created_at | datetime |
| updated_at | datetime |

#### categories

| Column | Type |
|--------|------|
| id | integer |
| name | string |
| slug | string |
| description | text |
| created_at | datetime |
| updated_at | datetime |


## Relationships

| From Table | To Table | Cardinality | Description |
|------------|----------|-------------|-------------|
| courses | assignments | 1-n | has |
| assignments | submissions | 1-n | has |
| users | submissions | 1-n | makes |
| users | enrollments | 1-n | has |
| courses | enrollments | 1-n | has |
| users | topics | 1-n | creates |
| users | posts | 1-n | creates |
| topics | posts | 1-n | has |
| categories | topics | 1-n | contains |
