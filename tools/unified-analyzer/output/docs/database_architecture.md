# Database Architecture

This document describes the database architecture for the LMS application.

## Overview

The LMS application uses a dual-database approach:

1. **Local Database**: SQLite database for offline storage
2. **Server Database**: PostgreSQL database for server-side storage

## Database Schema

The database schema consists of the following main tables:

- **users**: Stores user account information
- **courses**: Stores course information
- **enrollments**: Stores course enrollment information
- **assignments**: Stores assignment information
- **submissions**: Stores assignment submissions
- **discussions**: Stores discussion forum information
- **posts**: Stores discussion forum posts
- **comments**: Stores comments on posts
- **files**: Stores file metadata
- **notifications**: Stores user notifications

## Database Schema Visualization

For a detailed visualization of the database schema, see:

- [Database Schema (HTML)](visualizations/db_schema/db_schema.html)
- [Database Schema (Markdown)](visualizations/db_schema/db_schema.md)

## Relationships

The database schema includes the following key relationships:

- **users** to **courses**: One-to-many (a user can create multiple courses)
- **users** to **enrollments**: One-to-many (a user can be enrolled in multiple courses)
- **courses** to **enrollments**: One-to-many (a course can have multiple enrollments)
- **courses** to **assignments**: One-to-many (a course can have multiple assignments)
- **users** to **submissions**: One-to-many (a user can submit multiple assignments)
- **assignments** to **submissions**: One-to-many (an assignment can have multiple submissions)
- **users** to **discussions**: One-to-many (a user can create multiple discussions)
- **courses** to **discussions**: One-to-many (a course can have multiple discussions)
- **discussions** to **posts**: One-to-many (a discussion can have multiple posts)
- **users** to **posts**: One-to-many (a user can create multiple posts)
- **posts** to **comments**: One-to-many (a post can have multiple comments)
- **users** to **comments**: One-to-many (a user can create multiple comments)

## Synchronization

The local SQLite database and the server PostgreSQL database are synchronized using a custom sync mechanism. The sync mechanism handles:

- **Data Synchronization**: Syncing data between local and remote databases
- **Conflict Resolution**: Resolving conflicts when the same data is modified in both databases
- **Offline Queue**: Managing operations performed while offline
- **Sync Status**: Tracking the sync status of each record

## Migrations

Database migrations are managed using a custom migration system. The migration system handles:

- **Schema Changes**: Adding, modifying, or removing tables and columns
- **Data Migrations**: Transforming data from one format to another
- **Version Control**: Tracking the current database schema version
- **Rollbacks**: Reverting to a previous schema version if needed

## Performance Considerations

The database architecture is designed with the following performance considerations:

- **Indexing**: Appropriate indexes are created for frequently queried columns
- **Denormalization**: Some data is denormalized for performance reasons
- **Caching**: Frequently accessed data is cached in memory
- **Pagination**: Large result sets are paginated to improve performance
- **Lazy Loading**: Related data is loaded only when needed
