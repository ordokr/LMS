# Discourse Forum Port to Rust/Tauri/Leptos - Implementation Plan

## Project Overview

This project aims to port the Ruby-based Discourse forum system to a modern Rust stack using Tauri for desktop integration, Axum for backend API, and Leptos for frontend UI rendering. The goal is to create a standalone, performant forum system that maintains feature parity with core Discourse functionality while embracing Rust's performance and safety benefits.  It is as close to a 1:1 clone of it as possible, while being flexible on what should be modified in order to achieve integration with the LMS.

## Architecture Components

| Discourse Component | Rust Replacement | Implementation Details |
|---------------------|------------------|------------------------|
| Ruby on Rails API | Axum + sqlx | REST/GraphQL endpoints with type-safe handlers |
| Ember.js Frontend | Leptos (SSR + WASM) | Fine-grained reactivity with server-side rendering |
| PostgreSQL | SQLite + Redb | Embedded database with key-value store for metadata |
| Redis Cache | Moka + Tokio | In-memory async caching with TTL support |
| Sidekiq Jobs | Tokio-workers | Asynchronous background processing |
| Discourse Plugins | WebAssembly Plugins | Plugin system using WASM modules |

## Core Features to Implement

### 1. Authentication & User Management
- [x] Basic user registration/login
- [ ] OAuth providers (Google, GitHub, etc.)
- [ ] User profiles with preferences
- [ ] Trust levels and permissions
- [ ] Admin dashboard

### 2. Forum Structure
- [x] Categories and subcategories
- [x] Topics/threads with metadata
- [x] Posts with rich text support
- [ ] Tags and topic classification
- [ ] Topic pinning and featuring

### 3. Content Features
- [ ] Markdown rendering with syntax highlighting
- [ ] Post editing and version history
- [ ] Quoting and @mentions
- [ ] File uploads and image handling
- [ ] Emoji reactions

### 4. Interaction Features
- [ ] Voting and like system
- [ ] Bookmarks and personal collections
- [ ] User notifications system
- [ ] Private messaging

### 5. Moderation Tools
- [ ] Post flagging system
- [ ] Moderation queue
- [ ] User silencing/suspension
- [ ] Content approval workflows
- [ ] Spam prevention

### 6. Real-time Features
- [ ] Live updates via WebSockets
- [ ] Presence indicators ("X users viewing")
- [ ] Real-time notifications
- [ ] Typing indicators

## Database Schema

### Core Tables
1. `users` - User accounts and profile data
2. `categories` - Forum organization structure
3. `topics` - Thread containers
4. `posts` - Individual messages
5. `tags` - Topic classification
6. `uploads` - File storage metadata
7. `user_actions` - Activity tracking
8. `notifications` - User alerts
9. `badges` - Achievement system
10. `groups` - User collections for permissions

### Relationships
- Users create many Topics and Posts
- Categories contain many Topics
- Topics contain many Posts
- Topics can have many Tags
- Posts can have many Uploads
- Users can belong to multiple Groups

## API Endpoints

### Authentication
- POST `/auth/register` - Create new account
- POST `/auth/login` - Authenticate user
- POST `/auth/logout` - End session
- GET `/auth/me` - Current user info

### Categories
- GET `/categories` - List all categories
- GET `/categories/:id` - Single category details
- POST `/categories` - Create category (admin)
- PUT `/categories/:id` - Update category
- DELETE `/categories/:id` - Remove category

### Topics
- GET `/topics` - List topics (paginated)
- GET `/topics/:id` - Single topic with posts
- POST `/topics` - Create new topic
- PUT `/topics/:id` - Update topic details
- DELETE `/topics/:id` - Delete topic

### Posts
- GET `/posts/:id` - Get single post
- POST `/topics/:id/posts` - Create reply
- PUT `/posts/:id` - Edit post
- DELETE `/posts/:id` - Remove post

### Users
- GET `/users/:id` - User profile
- PUT `/users/:id` - Update profile
- GET `/users/:id/activity` - User activity

### Admin
- GET `/admin/dashboard` - Stats overview
- GET `/admin/users` - User management
- GET `/admin/flags` - Moderation queue

## Frontend Components

### Layouts
- `MainLayout` - Primary site structure
- `ForumLayout` - Category and topic view structure
- `AdminLayout` - Dashboard structure

### Pages
- `HomePage` - Forum entry point
- `CategoryPage` - Topics in category
- `TopicPage` - Posts in topic
- `UserPage` - Profile view
- `AdminPage` - Administration interface

### Components
- `TopicList` - Displays topics in category
- `PostList` - Shows posts in topic
- `Editor` - Rich text input for posts
- `UserCard` - Compact user profile
- `NotificationCenter` - Alert management

## Implementation Phases

### Phase 1: Core Forum Structure
- [x] Set up Tauri and Axum backend
- [x] Implement database schema
- [x] Create basic CRUD for categories, topics, posts
- [ ] Develop main forum views in Leptos

### Phase 2: User System
- [ ] Implement authentication
- [ ] Add user profiles
- [ ] Create permission system
- [ ] Develop trust level mechanics

### Phase 3: Rich Content
- [ ] Add markdown rendering
- [ ] Implement file uploads
- [ ] Create rich editor component
- [ ] Support embeds and code blocks

### Phase 4: Interactions
- [ ] Add voting and reactions
- [ ] Implement notifications
- [ ] Create bookmarking system
- [ ] Add private messaging

### Phase 5: Real-time Features
- [ ] Set up WebSocket infrastructure
- [ ] Implement live updates
- [ ] Add presence indicators
- [ ] Create typing notifications

### Phase 6: Moderation & Admin
- [ ] Develop flag/report system
- [ ] Create moderation queue
- [ ] Implement admin dashboard
- [ ] Add analytics and reporting

### Phase 7: Plugin System
- [ ] Design WASM plugin API
- [ ] Create plugin loading mechanism
- [ ] Develop core plugins (emoji, code highlighting)
- [ ] Document plugin development

## Technical Decisions

### Database
- SQLite for primary storage (simplifies deployment)
- Redb for fast key-value access to cached data
- Migration system for schema versioning

### API Design
- REST API with JSON for standard endpoints
- WebSockets for real-time features
- JWT for authentication

### UI Framework
- Leptos for reactive components
- Server-side rendering for initial load
- WASM for client-side interactivity

### Performance Optimizations
- Aggressive caching of read-heavy data
- Background processing for expensive operations
- Pagination and virtualization for large datasets

## Testing Strategy

- Unit tests for core business logic
- API tests for endpoint validation
- UI component tests
- End-to-end tests for critical user flows

## Deployment Strategy

- Tauri for desktop application packaging
- Docker for server deployment
- CI/CD pipeline for automated builds
- Update mechanism for client applications

## Technical Analysis: SQLite + Redb for Offline-First Forum

### Database Strategy Assessment

#### SQLite for Primary Storage

**Strength:** Ideal for embedded desktop apps with ACID compliance and zero-config deployment. Supports complex queries through rusqlite/sqlx crates with type-safe Rust bindings.

**Consideration:** Requires careful schema design to handle Discourse's relationships (1M+ topic/post connections). Use `WITHOUT ROWID` tables for PK-heavy access patterns.

#### Redb for Metadata

**Advantage:** Pure-Rust embedded KV store with B-tree indexing and memory-mapped I/O outperforms Redis for local storage. Typed API (`Table<u64, &[u8]>`) aligns well with Rust's safety goals.

**Limitation:** Lacks built-in replication. For offline sync, implement a CRDT layer for metadata conflict resolution.

### Key Implementation Recommendations

#### Data Partitioning

```rust
// SQLite for relational data
CREATE TABLE posts (
    id INTEGER PRIMARY KEY,
    content TEXT NOT NULL,
    user_id INTEGER REFERENCES users(id) ON DELETE CASCADE
);

// Redb for metadata
let user_meta_table = db.open_table("user_meta")?;
user_meta_table.insert(123, &[1, 0, 1, 1])?; // [has_avatar, trusted, etc.]
```

#### Concurrency Pattern

- **SQLite:** Use WAL mode with `busy_timeout` for concurrent writes.
- **Redb:** Leverage read-only transactions with `ReadTransaction` and batched writes.

#### Migration Path

1. **Phase 1:** Use SQLite's `.dump` command for baseline migration.
2. **Phase 2:** Implement Redb shadow tables for hot metadata.

### Alternative Considerations

Consider adding Sled for high-write scenarios like real-time collaboration metadata.

### Critical Path Items

- Implement SQLite `VACUUM` hooks in Tauri's setup phase.
- Use Redb's native encryption for sensitive metadata.
- Benchmark with realistic dataset (50GB SQLite + 5GB Redb).
- Develop cross-database transaction layer using event sourcing.

This architecture balances Rust's safety guarantees with pragmatic offline capabilities, though teams should budget for additional complexity in cross-store queries.