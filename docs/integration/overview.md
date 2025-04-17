# Integration Overview

_Last updated: 2025-04-16_

This document provides an overview of the integration between Canvas LMS and Discourse forum functionality in Ordo.

## Integration Overview

Ordo integrates Canvas LMS and Discourse forum functionality into a unified application. This integration allows for a seamless experience where users can access both learning management and discussion forum features in a single application.

## Integration Architecture

The integration between Canvas and Discourse is built on the following principles:

1. **Event-Driven Architecture**: For data synchronization
2. **Conflict Resolution**: Source of truth policies based on entity type
3. **Offline-First Capabilities**: Local storage, change tracking, sync queue
4. **Unified Authentication**: Single sign-on across all components
5. **Consistent UI/UX**: Unified design language across all features

## Integration Status

The following table shows the current status of the integration:

| Integration | Source | Target | Status |
|-------------|--------|--------|--------|
| Canvas Course Management | Canvas | Ordo | In Progress |
| Canvas Assignments | Canvas | Ordo | In Progress |
| Canvas Discussions | Canvas | Ordo | Planned |
| Discourse Forums | Discourse | Ordo | Planned |
| Discourse User System | Discourse | Ordo | In Progress |
| Blockchain Certification | Native | Ordo | In Progress |

## Model Mapping

The following table shows how Ordo models map to Canvas and Discourse models:

| Canvas | Discourse | Ordo | Notes |
|--------|-----------|------------|-------|
| Course | Category | Course | One-to-one mapping |
| Course Sections | Sub-categories | CourseSection | Optional |
| Discussion | Topic | Discussion | One-to-one mapping |
| Discussion Entry | Post | DiscussionPost | One-to-one mapping |
| Assignment | - | Assignment | Canvas-only |
| User | User | User | Unified user model |
| - | Tags | Tags | Discourse-only |

## Integration Challenges

The integration between Canvas and Discourse presents several challenges:

1. **Data Synchronization**: Ensuring data is synchronized correctly between the local and remote databases
2. **Conflict Resolution**: Handling conflicts when the same data is modified both locally and remotely
3. **Offline Support**: Ensuring all functionality works offline with seamless synchronization when connectivity returns
4. **Performance**: Maintaining good performance with potentially large local databases
5. **Security**: Ensuring proper authentication and authorization across all components

## Related Documentation

- [Implementation Roadmap](roadmap.md)
