# Topic Model

## Overview

Discourse Topic model representing a discussion topic

System: discourse

## Properties

| Name | Type | Description |
|------|------|-------------|
| id | integer | Unique identifier |
| title | string | Topic title |
| category_id | integer | Category ID |

## Relationships

| Name | Type | Target | Description |
|------|------|--------|-------------|
| category | belongs_to | Category | Parent category |
| posts | has_many | Post | Posts in this topic |
