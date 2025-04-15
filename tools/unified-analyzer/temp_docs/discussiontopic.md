# DiscussionTopic Model

## Overview

Canvas discussion topic model

System: canvas

## Properties

| Name | Type | Description |
|------|------|-------------|
| id | integer | Unique identifier |
| title | string | Topic title |
| message | text | Topic content |

## Relationships

| Name | Type | Target | Description |
|------|------|--------|-------------|
| course | belongs_to | Course | Associated course |
| entries | has_many | DiscussionEntry | Discussion replies |
