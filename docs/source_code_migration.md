# Source Code Porting Approach

> **Important Disclaimer:** This document describes the process of porting and transforming source code, models, and features from Canvas LMS and Discourse to Ordo. It does **not** cover data migration, user import, or live system integration. All references to “migration,” “integration,” or “import” refer solely to source code, schema, or feature porting, not to data or live system interoperability.

_Last updated: 2025-04-18_

This document outlines the approach for porting the Canvas LMS and Discourse forum source code to the Ordo platform. This approach focuses exclusively on source code and feature porting, not data migration from built applications or production systems.

## Key Principles

1. **Source Code Only**: This porting approach works exclusively with source code, not with built applications or databases.
2. **Static Analysis**: The analyzers extract information from source files through static analysis (no live data or database connection required).
3. **Code Transformation**: The porting process transforms Ruby/JavaScript code to Rust/Haskell.
4. **Incremental Porting**: Components are ported incrementally, with continuous testing.
5. **Preserve Functionality**: The ported code must maintain the same functionality as the original.

## Source Code Analysis

The unified analyzer includes several components for analyzing source code:

### Source Database Schema Analyzer

This analyzer extracts database schema information from Ruby migration files and model definitions. It does not require a connection to a built database.

```rust
/// Analyze the source code to extract database schema (no database connection required)
pub fn analyze(&mut self) -> Result<()> {
    println!("Analyzing Canvas source code for database schema...");
    self.analyze_canvas()?;

    println!("Analyzing Discourse source code for database schema...");
    self.analyze_discourse()?;

    Ok(())
}
```

### Model Analyzer

This analyzer extracts model definitions from Ruby files, including:

- Field definitions
- Relationships (belongs_to, has_many, etc.)
- Validations
- Callbacks
- Scopes

### Controller Analyzer

This analyzer extracts controller actions and business logic from Ruby files, including:

- Action definitions
- Parameter handling
- Authorization checks
- Response formatting

### View Analyzer

This analyzer extracts view templates from ERB (Canvas) and Handlebars (Discourse) files, including:

- HTML structure
- Template variables
- Conditional rendering
- Loops and iterations

## Code Transformation

The migration process transforms source code from Ruby/JavaScript to Rust/Haskell:

### Model Transformation

Ruby models are transformed to Rust structs with appropriate implementations:

```rust
// Original Ruby model:
// class User < ApplicationRecord
//   has_many :enrollments
//   validates :name, presence: true
// end

// Transformed Rust model:
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub fn new(name: String, email: String) -> Result<Self, ValidationError> {
        // Validation logic
        if name.is_empty() {
            return Err(ValidationError::new("name cannot be empty"));
        }
        
        // Create new user
        Ok(Self {
            id: Uuid::new_v4(),
            name,
            email,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }
    
    // Relationship methods
    pub async fn enrollments(&self, db: &SqlitePool) -> Result<Vec<Enrollment>, DbError> {
        sqlx::query_as!(Enrollment, 
            "SELECT * FROM enrollments WHERE user_id = ?", 
            self.id
        )
        .fetch_all(db)
        .await
        .map_err(|e| DbError::QueryError(e.to_string()))
    }
}
```

### Controller Transformation

Ruby controllers are transformed to Rust services and Tauri commands:

```rust
// Original Ruby controller:
// class CoursesController < ApplicationController
//   def index
//     @courses = Course.all
//   end
//
//   def show
//     @course = Course.find(params[:id])
//   end
// end

// Transformed Rust service:
pub struct CourseService {
    db: SqlitePool,
}

impl CourseService {
    pub fn new(db: SqlitePool) -> Self {
        Self { db }
    }
    
    pub async fn list_courses(&self) -> Result<Vec<Course>, ServiceError> {
        sqlx::query_as!(Course, "SELECT * FROM courses")
            .fetch_all(&self.db)
            .await
            .map_err(|e| ServiceError::DatabaseError(e.to_string()))
    }
    
    pub async fn get_course(&self, id: Uuid) -> Result<Course, ServiceError> {
        sqlx::query_as!(Course, "SELECT * FROM courses WHERE id = ?", id)
            .fetch_optional(&self.db)
            .await
            .map_err(|e| ServiceError::DatabaseError(e.to_string()))?
            .ok_or(ServiceError::NotFound("Course not found".to_string()))
    }
}

// Tauri command:
#[tauri::command]
pub async fn list_courses(state: State<'_, AppState>) -> Result<Vec<Course>, String> {
    let service = CourseService::new(state.db.clone());
    service.list_courses().await.map_err(|e| e.to_string())
}
```

### View Transformation

ERB/Handlebars templates are transformed to Leptos components:

```rust
// Original ERB template:
// <div class="course-header">
//   <h1><%= @course.name %></h1>
//   <p><%= @course.description %></p>
// </div>

// Transformed Leptos component:
#[component]
pub fn CourseHeader(cx: Scope, course: Course) -> impl IntoView {
    view! { cx,
        <div class="course-header">
            <h1>{&course.name}</h1>
            <p>{&course.description}</p>
        </div>
    }
}
```

## Migration Process

The migration process follows these steps:

1. **Analyze Source Code**: Run the unified analyzer to extract information from the source code.
2. **Generate Migration Plan**: Create a detailed plan for migrating each component.
3. **Transform Models**: Convert Ruby models to Rust structs.
4. **Transform Controllers**: Convert Ruby controllers to Rust services.
5. **Transform Views**: Convert ERB/Handlebars templates to Leptos components.
6. **Implement Business Logic**: Reimplement business logic in Rust/Haskell.
7. **Write Tests**: Create comprehensive tests for the migrated code.
8. **Integrate Components**: Ensure all components work together.
9. **Document**: Create technical documentation for the migrated codebase.

## Testing Strategy

The testing strategy ensures that the migrated code maintains the same functionality as the original:

1. **Unit Tests**: Test individual components in isolation.
2. **Integration Tests**: Test interactions between components.
3. **Behavior Tests**: Ensure the migrated code behaves the same as the original.
4. **Performance Tests**: Verify that the migrated code meets performance requirements.

## Conclusion

This source code migration approach allows for a systematic transformation of the Canvas LMS and Discourse forum source code to the Ordo platform. By focusing exclusively on source code analysis and transformation, we can create a clean implementation that fully leverages the strengths of Rust and Haskell while preserving the core functionality of the original systems.
