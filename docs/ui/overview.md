# UI Components Overview

_Last updated: 2025-04-18_

## Introduction

The Ordo project uses a modern UI architecture based on Leptos, a Rust-based reactive framework, combined with Tauri for desktop integration. This document provides an overview of the UI components and architecture.

## UI Architecture

The UI architecture follows these key principles:

1. **Component-Based**: UI is built from reusable, composable components
2. **Reactive**: Components react to state changes automatically
3. **Type-Safe**: End-to-end type safety from UI to database
4. **Offline-First**: All components work without an internet connection
5. **Accessible**: Components follow WCAG 2.1 guidelines

## Key UI Components

The UI is composed of the following key component categories:

- **Layout Components**: Page layouts, navigation, and structure
- **Form Components**: Input fields, buttons, and form validation
- **Data Display Components**: Tables, charts, and data visualization
- **Feedback Components**: Notifications, alerts, and progress indicators
- **Integration Components**: Components that integrate Canvas and Discourse functionality

## UI Component Strategy

For detailed information about the UI component implementation strategy, including recommended libraries, performance optimization, and implementation patterns, see the [UI Component Strategy](component_strategy.md) document.

## UI Examples

Here are some examples of key UI components:

### Course List Component

```rust
#[component]
pub fn CourseList() -> impl IntoView {
    let courses = create_resource(
        || (),
        |_| async move {
            CourseService::get_courses().await.unwrap_or_default()
        }
    );
    
    view! {
        <div class="course-list">
            <h1>"Courses"</h1>
            <Suspense fallback=move || view! { <p>"Loading courses..."</p> }>
                {move || {
                    courses.get().map(|courses| {
                        if courses.is_empty() {
                            view! { <p>"No courses found."</p> }.into_view()
                        } else {
                            view! {
                                <div class="course-grid">
                                    <For
                                        each=move || courses.clone()
                                        key=|course| course.id.clone()
                                        let:course
                                    >
                                        <CourseCard course=course />
                                    </For>
                                </div>
                            }.into_view()
                        }
                    })
                }}
            </Suspense>
        </div>
    }
}
```

### Offline-Ready Button Component

```rust
#[component]
pub fn OfflineReadyButton() -> impl IntoView {
    let (is_online, set_online) = create_signal(true);
    
    view! {
        <button 
            class="btn btn-primary rounded-lg"
            class:disabled=move || !is_online.get()
            on:click=move |_| {
                spawn_local(async {
                    WindowBuilder::new("offline-modal", "Offline Mode Active")
                        .build()
                        .unwrap();
                })
            }
        >
            <span class="i-ph-cloud-warning-bold"/> 
            "Submit Assignment"
        </button>
    }
}
```

## UI Development Guidelines

When developing UI components for the Ordo project, follow these guidelines:

1. **Use Leptos Signals**: Use Leptos signals for reactive state management
2. **Component Composition**: Compose complex components from simpler ones
3. **Consistent Styling**: Use DaisyUI and Tailwind for consistent styling
4. **Accessibility**: Ensure all components are accessible
5. **Performance**: Optimize components for performance
6. **Offline Support**: Design components to work offline
7. **Error Handling**: Handle errors gracefully in the UI

## UI Testing

UI components are tested using the following approaches:

1. **Unit Tests**: Test individual components in isolation
2. **Integration Tests**: Test component interactions
3. **End-to-End Tests**: Test complete user flows
4. **Accessibility Tests**: Test for accessibility compliance
5. **Performance Tests**: Test component performance

## Conclusion

The UI architecture of the Ordo project provides a modern, reactive, and type-safe approach to building user interfaces. By following the guidelines and using the recommended libraries, we can create a consistent, performant, and accessible user experience.
