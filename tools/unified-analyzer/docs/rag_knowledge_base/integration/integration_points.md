# Canvas-Discourse Integration Points

## Overview

This document describes the key integration points between Canvas LMS and Discourse forum systems.

## Integration Mapping

### Course to Category Mapping

Canvas courses can be mapped to Discourse categories:

| Canvas | Discourse | Notes |
|--------|-----------|-------|
| Course | Category | One-to-one mapping |
| Course Sections | Sub-categories | Optional |

### Discussion Topic Mapping

Canvas discussion topics can be synchronized with Discourse topics:

| Canvas | Discourse | Notes |
|--------|-----------|-------|
| Discussion Topic | Topic | One-to-one mapping |
| Discussion Entry | Post | One-to-one mapping |
| Discussion Reply | Reply | One-to-one mapping |

## Integration Strategies

1. **API-based integration**: Use REST APIs on both systems
2. **Event-driven integration**: Use webhooks and event subscribers
3. **Database-level integration**: Direct database integration (not recommended)

## Authentication Flow

For SSO between Canvas and Discourse:

1. Canvas authenticates the user
2. Canvas generates a signed payload with user information
3. User is redirected to Discourse with the payload
4. Discourse verifies the payload and creates/logs in the user
