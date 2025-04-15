# Courses API Reference

This document describes the Tauri command API for course management in the LMS Integration Project.

## Commands Overview

| Command | Function | Description | Status |
|---------|----------|-------------|--------|
| `get_courses` | `get_courses(status?: CourseStatus)` | Retrieves a list of courses with optional status filtering | Implemented |
| `get_course` | `get_course(course_id: string)` | Retrieves a specific course by ID | Implemented |
| `create_course` | `create_course(course: CourseCreate)` | Creates a new course | Pending |
| `update_course` | `update_course(course: Course)` | Updates an existing course | Pending |
| `delete_course` | `delete_course(course_id: string)` | Deletes a course | Pending |

## Data Types

### Course

```typescript
interface Course {
  id: string;
  title: string;
  description: string;
  status: CourseStatus;
  created_at: string; // ISO date string
  updated_at: string; // ISO date string
  modules: string[]; // Array of module IDs
}

enum CourseStatus {
  Active = "active",
  Archived = "archived",
  Draft = "draft"
}
```

## Usage Examples

### Frontend (TypeScript)

```typescript
// Get all active courses
const activeCourses = await invoke<Course[]>("get_courses", { 
  status: "active" 
});

// Get a specific course
const course = await invoke<Course>("get_course", { 
  course_id: "course123" 
});
```

Backend Registration
These commands are registered in main.rs:

```rust
.invoke_handler(tauri::generate_handler![
    // ...other commands
    api::courses::get_courses,
    api::courses::get_course,
])
```

