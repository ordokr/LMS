# Category Model

## Overview

Discourse category for organizing topics

System: discourse

## Properties

| Name | Type | Description |
|------|------|-------------|
| id | integer | Unique identifier |
| name | string | Category name |
| slug | string | URL-friendly name |

## Relationships

| Name | Type | Target | Description |
|------|------|--------|-------------|
| topics | has_many | Topic | Topics in this category |
