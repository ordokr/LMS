# Canvas LMS and Discourse Forum Integration Plan

## Executive Summary

This project combines a Canvas LMS port with a Discourse-like forum system,
creating an **offline-first educational platform** using Rust, Tauri, and Leptos.
The system will support rich learning experiences even when internet access is limited.

---

### Source Code References:
- Canvas: C:\Users\Tim\Desktop\port\canvas
- Discourse: C:\Users\Tim\Desktop\port\port

## Current Implementation Status

### 1. Core Framework - 70% Complete
- ✅ Tauri desktop application container setup
- ✅ Leptos frontend structure established
- ✅ SQLite database integration
- 🔄 Offline-first engine implementation in progress
- 🔄 Background sync service partially implemented

### 2. LMS Component - 45% Complete
- ✅ Course management base functionality
- ✅ Assignment models and services
- 🔄 Gradebook implementation
- ❌ Learning outcomes and rubrics pending

### 3. Forum Component - 60% Complete
- ✅ Category and topic structure
- ✅ Thread conversation models
- 🔄 User trust system
- 🔄 Moderation tools in progress

### 4. Integration Points - 35% Complete
- ✅ Basic LMS ↔ Forum cross-linking
- 🔄 Shared notification system
- ❌ Activity stream integration pending

---

## Core Architecture Components

### 1. Core Framework
- **Tauri**: Desktop application container
- **Axum**: API server
- **Leptos**: WebAssembly-based reactive frontend
- **SQLite**: Local database for persistence
- **Offline-first engine**: Vector clock + CRDT sync

### 2. LMS Component (Canvas Port)
- Course management
- Assignments & submissions
- Gradebook and assessments
- Content modules & pages
- User roles and permissions
- Learning outcomes and rubrics

### 3. Forum Component (Discourse Port)
- Categories and topics
- Threaded conversations
- User trust levels
- Reactions, voting, moderation
- Notification system

---

## Integration Strategy (Phased Plan)

### Phase 1: Core Framework Enhancement (2–3 weeks)
- Enhance offline-first data layer
- Improve auth system (login, roles, tokens)
- Add unified error handling
- Implement background sync workers

### Phase 2: Canvas Model Adaptation (4–6 weeks)
- Port Canvas data models to Rust
- Repository layer for data access
- LMS business logic (course, grading, etc.)
- Define LMS API endpoints

### Phase 3: Forum Refinement (3–4 weeks)
- Complete forum models & business logic
- Forum API endpoints
- Trust system and moderation tools

### Phase 4: Integration Points (3–4 weeks)
- LMS ↔ Forum cross-linking
- Shared notification system
- Activity stream & embedding support
- Unified permission rules

### Phase 5: UI Development (4–6 weeks)
- LMS UI (courses, assignments)
- Forum UI (categories, threads)
- Integrated workflows
- Offline status indicators

---

## Current Directory Structure

```text
LMS/
├── INTEGRATION_PLAN.md          # Project integration plan
├── mapping.md                   # Feature mapping from Canvas/Discourse
├── src/
│   ├── app.rs                   # Main app with routing
│   ├── main.rs                  # Entry point
│   ├── components/
│   │   ├── mod.rs               # Component exports
│   │   ├── home.rs              # Home page
│   │   ├── layout.rs            # Layout component
│   │   ├── auth/                # Authentication components
│   │   │   ├── mod.rs
│   │   │   ├── login.rs
│   │   │   ├── register.rs
│   │   │   └── profile.rs
│   │   ├── lms/                 # LMS components
│   │   │   ├── mod.rs
│   │   │   ├── courses.rs       # Course management
│   │   │   ├── assignments.rs   # Assignment components
│   │   │   └── modules.rs       # Module components
│   │   ├── forum/               # Forum components
│   │   │   ├── mod.rs
│   │   │   ├── categories.rs
│   │   │   ├── threads.rs
│   │   │   ├── posts.rs
│   │   │   └── admin/
│   │   │       ├── mod.rs
│   │   │       ├── dashboard.rs
│   │   │       └── settings.rs
│   │   └── shared/              # Shared UI components
│   │       ├── mod.rs
│   │       ├── offline_indicator.rs
│   │       └── error_display.rs
│   ├── models/                  # Data models
│   │   ├── mod.rs
│   │   ├── auth.rs              # Auth models
│   │   ├── lms.rs               # LMS models
│   │   ├── forum.rs             # Forum models
│   │   └── sync.rs              # Sync models
│   ├── services/                # Services for API interaction
│   │   ├── mod.rs
│   │   ├── auth_service.rs
│   │   ├── lms_service.rs
│   │   ├── forum_service.rs
│   │   ├── integration_service.rs  # Integration between LMS and forum
│   │   └── websocket.rs         # Real-time communication
│   ├── sync/                    # Offline sync functionality
│   │   ├── mod.rs
│   │   ├── sync_manager.rs      # Core sync logic
│   │   ├── vector_clock.rs      # Vector clock implementation
│   │   └── conflict_resolution.rs
│   └── utils/                   # Utility functions
│       ├── mod.rs
│       ├── auth.rs              # Auth utilities
│       ├── errors.rs            # Error handling
│       ├── offline.rs           # Offline detection
│       └── sync.rs              # Sync utilities
└── src-tauri/                   # Tauri backend
    └── src/
        ├── main.rs
        ├── api/                 # API handlers
        └── db/                  # Database layer
```

---

## Integration Points

### LMS → Forum
- Course discussions link to forum categories
- Assignment discussions shown as forum topics
- Embedded forum content in LMS pages
- Forum activity contributes to participation grades

### Forum → LMS
- Forum categories tied to specific courses
- Trust levels mapped to LMS roles
- Forum notifications appear in course activity feed
- Forum posts can reference course materials

---

## Offline-First Strategy

1. All operations written to local SQLite
2. Vector clocks track conflict resolution
3. Sync happens when online (background process)
4. UI reflects sync status with visual indicators

```rust
// Example SyncOperation
enum SyncOperation {
  Create { entity_type: String, data: Value },
  Update { entity_type: String, id: String, fields: HashMap<String, Value> },
  Delete { entity_type: String, id: String },
  Reference { source_type: String, source_id: String, target_type: String, target_id: String },
}

struct SyncBatch {
  device_id: String,
  user_id: String,
  operations: Vec<SyncOperation>,
  timestamp: i64,
  vector_clock: HashMap<String, i64>,
}
```

---

## Data Model Integration

- Shared user model with unified identity
- Content reference system spans LMS & forum
- Integrated notification & activity streams
- Unified permission/role model across both systems

---

## Next Steps

1. Finalize core enhancements (auth, sync, error handling)
2. Port key Canvas LMS models and endpoints
3. Implement and test forum backend logic
4. Build the service layer that unites LMS and forum

---

## 5. Offline-First Sync

### 5.1 Strategy
- UI → CRDT Operation → Local DB
- Background sync with server
- Conflict resolution by type

### 5.2 Rust Structures
```rust
enum SyncOperation {
  Create { entity_type: String, data: Value },
  Update { entity_type: String, id: String, fields: HashMap<String, Value> },
  Delete { entity_type: String, id: String },
  Reference { source_type: String, source_id: String, target_type: String, target_id: String },
}

struct SyncBatch {
  device_id: String,
  user_id: String,
  operations: Vec<SyncOperation>,
  timestamp: i64,
  vector_clock: HashMap<String, i64>,
}
```

---

## 6. Authentication & Permissions

- JWT token with role info (stored securely via Tauri)
- Offline login using cached credentials
- Permission Model:
  - Roles: Admin, Teacher, Student
  - Contextual: Course, Forum Category
  - Inheritance between LMS + Forum

---

## 7. API Structure

### Endpoints
```http
POST /api/v1/auth/login
GET  /api/v1/courses/:id
GET  /api/v1/forum/categories
POST /api/v1/sync
```

- REST (CRUD)
- WebSocket (real-time)
- GraphQL (optional complex queries)

---

## 8. Implementation Phases

- **Phase 1:** Core Framework (4–6w)
  - Tauri + Axum + Leptos setup
  - Auth + Offline DB + UI skeleton

- **Phase 2:** Canvas LMS (8–10w)
  - Courses, Assignments, Grades, Modules

- **Phase 3:** Forum (6–8w)
  - Categories, Topics, Trust System

- **Phase 4:** Integration & Sync (4–6w)
  - Cross-referencing, Notifications, Sync Engine

- **Phase 5:** Polish & Deploy (4w)
  - UI/UX, Tests, Docs, Build Pipelines

---

## 9. Testing Strategy

- Unit: Core logic
- Integration: API layers
- E2E: Workflow tests
- Offline: Sync conflict scenarios

---

## 10. Deployment

- **Platforms:** Tauri desktop (Win/macOS/Linux), Mobile (future)
- **Options:** Self-hosted server (optional)
- **Migration:** Importers for Canvas/Discourse

---

## 11. Dev Tooling

- **Editor:** VS Code (Rust + Tauri extensions)
- **Testing:** Docker + local server setup
- **CI/CD:** PR testing, Build automation, Release workflows

---

## Development Workflow

### Daily Tasks
1. Run "Find Canvas Equivalent" for any new models being developed
2. Use "Compare Models" when implementing new fields or methods
3. Pre-commit hook will flag low-completion models

### Weekly Tasks
1. Run "Full Codebase Audit" to update mapping.md
2. Generate updated documentation
3. Review dashboard for improvement areas

### Monthly Tasks
1. Complete review of all <70% completion models
2. Update project timeline based on audit results
```
