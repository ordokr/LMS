# Modules Overview

_Last updated: 2025-04-18_

## Introduction

The Ordo project implements a modular architecture that allows for extending the application with additional app-like modules that can be turned on and off. This document provides an overview of the available modules and their integration status.

## Module Types

The Ordo project supports several types of modules:

### 1. Core Modules

These are built into the application and cannot be disabled:

- User Management
- Course Management
- Synchronization Engine
- Storage Engine

### 2. Optional Native Modules

These are compiled into the application but can be enabled/disabled at runtime:

- Discussion Forums
- Assignment Management
- Gradebook
- Calendar

### 3. Plugin Modules

These are loaded dynamically at runtime:

- [Quiz System](quiz_integration.md) (Integration from Quenti)
- Attendance Tracking
- Peer Review
- Custom Assessment Tools

## Module Integration Status

| Module | Type | Status | Source | Documentation |
|--------|------|--------|--------|---------------|
| User Management | Core | Implemented | Native | [Link](../models/user.md) |
| Course Management | Core | Implemented | Native | [Link](../models/course.md) |
| Synchronization Engine | Core | Implemented | Native | [Link](../technical/sync_engine_implementation.md) |
| Storage Engine | Core | Implemented | Native | [Link](../architecture/database.md) |
| Discussion Forums | Optional | In Progress | Discourse | [Link](../integration/discourse.md) |
| Assignment Management | Optional | In Progress | Canvas | [Link](../integration/canvas.md) |
| Gradebook | Optional | Planned | Canvas | - |
| Calendar | Optional | Planned | Canvas | - |
| Content Creation & Management | Optional | In Progress | Native | [Link](content/overview.md) |
| Quiz System | Plugin | In Progress | Quenti | [Link](quiz_integration.md) |
| Attendance Tracking | Plugin | Planned | Native | - |
| Peer Review | Plugin | Planned | Native | - |
| Custom Assessment Tools | Plugin | Planned | Native | - |

## Module Development

For information on developing new modules for the Ordo project, see the [Modular Architecture](../architecture/modular_architecture.md) document.

## Module Configuration

Modules can be configured through the following mechanisms:

### 1. Compile-Time Configuration

```toml
# Cargo.toml
[features]
quiz-module = ["tauri-plugin-quiz", "leptos-quiz-components"]
forum-module = ["tauri-plugin-forum", "leptos-forum-components"]
```

### 2. Runtime Configuration

```rust
// src-tauri/src/config.rs
#[derive(Deserialize, Serialize)]
pub struct ModuleConfig {
    pub quiz_enabled: bool,
    pub forum_enabled: bool,
    pub gradebook_enabled: bool,
    pub calendar_enabled: bool,
}
```

### 3. User Preferences

```rust
// src-tauri/src/user/preferences.rs
#[derive(Deserialize, Serialize)]
pub struct UserModulePreferences {
    pub enabled_modules: Vec<String>,
    pub module_settings: HashMap<String, Value>,
}
```

## Module Interactions

Modules can interact with each other through the following mechanisms:

### 1. Event Bus

```rust
// src-tauri/src/events/mod.rs
pub struct EventBus {
    tx: flume::Sender<Arc<Event>>,
    rx: flume::Receiver<Arc<Event>>,
}

impl EventBus {
    pub fn post(&self, event: Event) {
        let event = Arc::new(event);
        self.tx.send(event).unwrap();
    }
}
```

### 2. Shared Services

```rust
// src-tauri/src/services/mod.rs
pub struct ServiceRegistry {
    services: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
}

impl ServiceRegistry {
    pub fn register<T: 'static + Send + Sync>(&mut self, service: T) {
        self.services.insert(TypeId::of::<T>(), Box::new(service));
    }

    pub fn get<T: 'static + Send + Sync>(&self) -> Option<&T> {
        self.services.get(&TypeId::of::<T>())
            .and_then(|boxed| boxed.downcast_ref::<T>())
    }
}
```

### 3. Extension Points

```rust
// src-tauri/src/extensions/mod.rs
pub trait ExtensionPoint<T> {
    fn register_extension(&mut self, extension: Box<dyn Fn(&T) -> Result<()> + Send + Sync>);
    fn execute_extensions(&self, context: &T) -> Result<()>;
}
```

## Conclusion

The modular architecture of the Ordo project provides a flexible and extensible foundation for building a comprehensive learning management system. By leveraging compile-time modularity, WASM-based extensions, and efficient inter-module communication, the project can grow and adapt to meet the needs of its users without sacrificing performance or maintainability.
