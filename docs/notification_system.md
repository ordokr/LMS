# Notification System

This document describes the notification unification system used in the Canvas-Discourse integration.

## Overview

The notification system creates a unified interface for notifications from both Canvas and Discourse, enabling users to see all their notifications in one place regardless of which system generated them.

## Components

1. **Unified Notification Model**: Represents notifications from both systems in a consistent format
2. **Notification Service**: Handles fetching, creating, and managing notifications
3. **Webhook Service**: Processes real-time events from both systems
4. **API Endpoints**: Provides access to unified notifications

## Notification Flow

1. **Event Occurs**: An event happens in Canvas or Discourse (e.g., new discussion post)
2. **Webhook Triggered**: The system that detected the event sends a webhook to our integration
3. **Webhook Processing**: Our system processes the webhook and creates a unified notification
4. **Cross-posting**: The notification is created in both systems when applicable
5. **User Notification**: The user sees the notification in either interface

## API Endpoints

### Get User Notifications

Query parameters:
- `read` (boolean): Filter by read status
- `type` (string): Filter by notification type
- `since` (ISO date): Only return notifications after this date
- `limit` (number): Maximum number of notifications to return

### Mark Notification as Read
POST /api/v1/notifications/:id/read
Request body:
```json
{
  "source": "canvas|discourse"
}

Webhook Endpoints
Canvas Webhook

POST /api/v1/webhooks/canvas

Discourse Webhook

POST /api/v1/webhooks/discourse

Supported Event Types
Canvas Events
submission_created
submission_updated
discussion_entry_created
course_created
user_created
Discourse Events
post_created
post_edited
topic_created
user_created
category_created
Implementation Notes
Notifications are synchronized between systems when possible
Read status is synced between systems
Each notification stores IDs from both systems when applicable
Webhooks require proper authentication to prevent abuse