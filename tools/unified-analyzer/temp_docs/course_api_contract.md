# Course API Contract

## Overview

API contract for Course-related endpoints

System: canvas

## GET /api/v1/courses/:id

### Parameters

| Name | Type | Description |
|------|------|-------------|
| id | integer | Course ID |

### Responses

#### 200 - Success

```json
{ "id": 1, "name": "Example Course" }
```

