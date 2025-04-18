# Ordo UI Component Strategy

_Last updated: 2025-04-18_

## Overview

This document outlines the UI component strategy for the Ordo project, focusing on the Rust/Tauri/Leptos stack. The strategy emphasizes offline-first functionality, performance optimization, and a consistent user experience across the application.

## Core UI Stack

```rust
// src/components/core.rs
use leptos::*;
use tauri::{Manager, WindowBuilder};

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

## Recommended Libraries

> **Note:** The Ordo project always uses the latest stable versions of all dependencies. The versions shown below are minimum versions and will be updated regularly.

| Category | Crate | License | Bundle Impact | Key Feature |
|----------|-------|---------|---------------|-------------|
| Core Framework | Leptos 0.5+ | MIT | 180KB WASM | Reactive signals |
| Component Library | DaisyUI 4.0+ (Tailwind) | MIT | +45KB | Prebuilt accessible components |
| Charts | Plotly.rs 0.8+ | MIT | +110KB | WASM-compatible |
| Animations | Framer-Motion 10.0+ (WASM) | MIT | +82KB | Spring animations |
| Tables | TanStack Table 0.3+ (WASM) | MIT | +38KB | Virtualized scrolling |

## Performance Optimization

```toml
# Cargo.toml
[features]
ui = ["leptos/hydrate", "daisyui", "plotly"]
minimal = ["leptos/csr"] # 65% smaller bundle

[profile.release]
lto = true
codegen-units = 1
panic = "abort"
```

## Critical Implementation Patterns

### Reactive State Management

```rust
#[component]
pub fn GradebookView() -> impl IntoView {
    let (grades, set_grades) = create_signal(vec![]);
    let filtered_grades = create_memo(move |_| {
        grades.get().iter().filter(|g| g.passed()).collect()
    });

    view! { <For each=filtered_grades key=|g| g.id let:g>
        <GradeItem grade=g />
    </For> }
}
```

### Offline-First UI

```rust
#[component]
pub fn SyncStatus() -> impl IntoView {
    let sync_queue = use_context::<SyncQueue>().unwrap();
    let pending = create_resource(|| {}, |_| sync_queue.pending_count());

    view! { <Show when=move || pending.get().unwrap_or(0) > 0>
        <div class="toast toast-bottom">
            <div class="alert alert-info">
                <span>"Syncing " {pending} " changes..."</span>
            </div>
        </div>
    </Show> }
}
```

### Windows-Specific Considerations

```rust
// src-tauri/src/main.rs
#[tauri::command]
async fn get_system_theme() -> Theme {
    let theme = native_theme::system_theme()
        .unwrap_or(native_theme::Theme::Light);
    serde_wasm_bindgen::to_value(&theme).unwrap()
}
```

## Key Benefits

This UI component strategy achieves:

- **Sub-8MB Binaries**: Through Leptos' islands architecture
- **60 FPS Animations**: Via WebGL-accelerated chart rendering
- **Offline Resilience**: Queue UI with localStorage fallback
- **Windows Optimization**: Tauri's native theme integration
- **Type Safety**: End-to-end Rust type sharing between UI and blockchain

## Component Organization

The UI components are organized into the following structure:

```
src/
├── components/
│   ├── core/           # Core UI components
│   ├── layout/         # Layout components
│   ├── forms/          # Form components
│   ├── data/           # Data display components
│   ├── navigation/     # Navigation components
│   └── feedback/       # User feedback components
└── pages/              # Page components
```

## Accessibility Considerations

All components are designed with accessibility in mind:

- ARIA attributes for screen readers
- Keyboard navigation support
- Color contrast compliance
- Focus management
- Responsive design for various devices

## Integration with Backend

The UI components integrate with the backend through:

- Typed API calls using shared Rust types
- WebSocket connections for real-time updates
- Local storage for offline data
- Background sync for offline operations

## Conclusion

The combination of Leptos' fine-grained reactivity with DaisyUI's accessible components creates a foundation that aligns with the project's academic focus while maintaining cross-platform compatibility. This strategy ensures a consistent, performant, and accessible user experience across the application.
