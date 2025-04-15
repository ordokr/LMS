# Notification API

## Endpoints

### GET /api/v1/notifications

Retrieves notifications for the authenticated user.

**Authentication Required**: Yes (JWT token)

**Query Parameters**:
- `read` (optional): Filter notifications by read status (`true` or `false`)
- `source` (optional): Filter notifications by source system (`canvas` or `discourse`)

**Response**:
```json
{
  "notifications": [
    {
      "id": "notification-id",
      "title": "Notification title",
      "message": "Notification message",
      "createdAt": "2023-05-15T10:30:00Z",
      "read": false,
      "source": "canvas",
      "sourceId": "original-id-in-source-system",
      "link": "https://example.com/link-to-resource"
    }
  ]
}
```

### POST /api/v1/notifications/:id/read

Marks a notification as read.

**Authentication Required**: Yes (JWT token)

**URL Parameters**:
- `id`: The notification ID to mark as read

**Request Body**:
```json
{
  "source": "canvas"  // Required: The source system of the notification
}
```

**Response**:
```json
{
  "id": "notification-id",
  "title": "Notification title",
  "message": "Notification message",
  "createdAt": "2023-05-15T10:30:00Z",
  "read": true,
  "source": "canvas",
  "sourceId": "original-id-in-source-system",
  "link": "https://example.com/link-to-resource"
}
```

**Error Responses**:
- `400 Bad Request`: If source parameter is missing
- `401 Unauthorized`: If authentication token is missing or invalid
- `404 Not Found`: If notification is not found
- `500 Internal Server Error`: If server error occurs