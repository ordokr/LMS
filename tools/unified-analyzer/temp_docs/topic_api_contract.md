# Topic API Contract

## Overview

API contract for Topic-related endpoints

System: discourse

## GET /t/:slug/:topic_id.json

### Parameters

| Name | Type | Description |
|------|------|-------------|
| slug | string | Topic slug |
| topic_id | integer | Topic ID |

### Responses

#### 200 - Success

```json
{ "id": 1, "title": "Example Topic" }
```

