use leptos::*;
use leptos_router::*;
use crate::models::integration::{IntegrationStatus, SyncConflict, ConflictResolutionStrategy};
use crate::services::integration_service::IntegrationService;
use crate::components::shared::ErrorAlert;
use crate::utils::style_manager::use_stylesheet;
use crate::components::integration::conflict_resolver::ConflictResolver;

#[component]
pub fn CanvasIntegration() -> impl IntoView {
    // Load stylesheet
    use_stylesheet("integration.css");
    
    // State
    let (loading, set_loading) = create_signal(false);
    let (active_tab, set_active_tab) = create_signal("courses".to_string());
    let (courses, set_courses) = create_signal(Vec::new());
    let (assignments, set_assignments) = create_signal(Vec::new());
    let (sync_history, set_sync_history) = create_signal(Vec::new());
    let (integration_status, set_integration_status) = create_signal(IntegrationStatus::default());
    let (error, set_error) = create_signal(None::<String>);
    let (conflicts, set_conflicts) = create_signal(Vec::<SyncConflict>::new());
    let (show_conflict_modal, set_show_conflict_modal) = create_signal(false);
    let (selected_conflict, set_selected_conflict) = create_signal(None::<SyncConflict>);
    
    // Load data function
    let load_data = move || {
        set_loading.set(true);
        set_error.set(None);
        
        spawn_local(async move {
            // Load integration status
            match IntegrationService::get_canvas_integration_status().await {
                Ok(status) => {
                    set_integration_status.set(status);
                    
                    // Only load other data if connected
                    if integration_status.get().connected {
                        // Load courses
                        if let Ok(courses_data) = IntegrationService::get_canvas_courses().await {
                            set_courses.set(courses_data);
                        }
                        
                        // Load assignments
                        if let Ok(assignments_data) = IntegrationService::get_canvas_assignments().await {
                            set_assignments.set(assignments_data);
                        }
                        
                        // Load sync history
                        if let Ok(history_data) = IntegrationService::get_canvas_sync_history().await {
                            set_sync_history.set(history_data);
                        }
                        
                        // Load pending conflicts
                        if let Ok(conflicts_data) = IntegrationService::get_sync_conflicts().await {
                            set_conflicts.set(conflicts_data);
                        }
                    }
                },
                Err(err) => {
                    set_error.set(Some(format!("Failed to load integration status: {}", err)));
                }
            }
            
            set_loading.set(false);
        });
    };
    
    // Sync all courses
    let sync_all_courses = move |_| {
        set_loading.set(true);
        set_error.set(None);
        
        spawn_local(async move {
            match IntegrationService::sync_all_canvas_courses().await {
                Ok(_) => {
                    // Reload data after sync
                    load_data();
                },
                Err(err) => {
                    set_error.set(Some(format!("Failed to sync courses: {}", err)));
                    set_loading.set(false);
                }
            }
        });
    };
    
    // Sync a specific course
    let sync_course = move |course_id: String| {
        set_loading.set(true);
        set_error.set(None);
        
        spawn_local(async move {
            match IntegrationService::sync_canvas_course(&course_id).await {
                Ok(_) => {
                    // Reload data after sync
                    load_data();
                },
                Err(err) => {
                    set_error.set(Some(format!("Failed to sync course: {}", err)));
                    set_loading.set(false);
                }
            }
        });
    };
    
    // Setup Canvas integration
    let setup_integration = move |_| {
        set_loading.set(true);
        set_error.set(None);
        
        spawn_local(async move {
            match IntegrationService::setup_canvas_integration().await {
                Ok(_) => {
                    // Reload data after setup
                    load_data();
                },
                Err(err) => {
                    set_error.set(Some(format!("Failed to setup integration: {}", err)));
                    set_loading.set(false);
                }
            }
        });
    };
    
    // Load data on component mount
    create_effect(move |_| {
        load_data();
    });
    
    view! {
        <div class="integration-page">
            <div class="page-header">
                <h1 class="page-title">
                    <i class="icon-canvas"></i>
                    "Canvas Integration"
                </h1>
                
                <div class="page-actions">
                    <button 
                        class="btn btn-primary"
                        on:click=sync_all_courses
                        disabled=move || loading.get() || !integration_status.get().connected
                    >
                        <i class="icon-sync"></i>
                        "Sync All"
                    </button>
                </div>
            </div>
            
            {move || if let Some(err) = error.get() {
                view! { <ErrorAlert message=err /> }
            } else {
                view! { <></> }
            }}
            
            <div class="integration-status-card">
                <h2 class="card-title">"Integration Status"</h2>
                
                <div class="status-overview">
                    <div class="status-item">
                        <span class="status-label">"Connection Status"</span>
                        <span class=if integration_status.get().connected { "status-connected" } else { "status-disconnected" }>
                            {if integration_status.get().connected { "Connected" } else { "Disconnected" }}
                        </span>
                    </div>
                    
                    <div class="status-item">
                        <span class="status-label">"Last Sync"</span>
                        <span class="status-value">
                            {move || integration_status.get().last_sync.unwrap_or_else(|| "Never".to_string())}
                        </span>
                    </div>
                    
                    <div class="status-item">
                        <span class="status-label">"Pending Syncs"</span>
                        <span class="status-value">{move || integration_status.get().pending_syncs}</span>
                    </div>
                    
                    <div class="status-item">
                        <span class="status-label">"Sync Errors"</span>
                        <span class="status-value">{move || integration_status.get().sync_errors}</span>
                    </div>
                </div>
                
                {move || if !integration_status.get().connected {
                    view! {
                        <div class="setup-integration">
                            <p class="text-muted">"Canvas integration is not set up. Click the button below to configure it."</p>
                            <button 
                                class="btn btn-primary"
                                on:click=setup_integration
                                disabled=loading.get()
                            >
                                "Setup Canvas Integration"
                            </button>
                        </div>
                    }
                } else {
                    view! { <></> }
                }}
            </div>
            
            {move || if integration_status.get().connected {
                view! {
                    <div class="integration-tabs">
                        <div class="tabs-header">
                            <button 
                                class={if active_tab.get() == "courses" { "tab-active" } else { "" }}
                                on:click=move |_| set_active_tab.set("courses".to_string())
                            >
                                "Courses"
                            </button>
                            
                            <button 
                                class={if active_tab.get() == "assignments" { "tab-active" } else { "" }}
                                on:click=move |_| set_active_tab.set("assignments".to_string())
                            >
                                "Assignments"
                            </button>
                            
                            <button 
                                class={if active_tab.get() == "history" { "tab-active" } else { "" }}
                                on:click=move |_| set_active_tab.set("history".to_string())
                            >
                                "Sync History"
                            </button>
                            
                            <button 
                                class={if active_tab.get() == "conflicts" { "tab-active" } else { "" }}
                                on:click=move |_| set_active_tab.set("conflicts".to_string())
                            >
                                "Conflicts "
                                {move || {
                                    let count = conflicts.get().len();
                                    if count > 0 {
                                        view! { <span class="conflict-badge">{count}</span> }
                                    } else {
                                        view! { <></> }
                                    }
                                }}
                            </button>
                        </div>
                        
                        <div class="tabs-content">
                            <div class="tab-panel" style:display={if active_tab.get() == "courses" { "block" } else { "none" }}>
                                <CoursesList 
                                    courses=courses
                                    sync_course=sync_course
                                    loading=loading
                                />
                            </div>
                            
                            <div class="tab-panel" style:display={if active_tab.get() == "assignments" { "block" } else { "none" }}>
                                <AssignmentsList 
                                    assignments=assignments
                                    loading=loading
                                />
                            </div>
                            
                            <div class="tab-panel" style:display={if active_tab.get() == "history" { "block" } else { "none" }}>
                                <SyncHistory history=sync_history />
                            </div>
                            
                            <div class="tab-panel" style:display={if active_tab.get() == "conflicts" { "block" } else { "none" }}>
                                <div class="conflicts-panel">
                                    <h3>"Sync Conflicts"</h3>
                                    
                                    {move || {
                                        let conflict_list = conflicts.get();
                                        if conflict_list.is_empty() {
                                            view! {
                                                <div class="empty-conflicts">
                                                    <p>"No conflicts found. All systems are in sync."</p>
                                                </div>
                                            }
                                        } else {
                                            view! {
                                                <div class="conflicts-list">
                                                    <table class="conflicts-table">
                                                        <thead>
                                                            <tr>
                                                                <th>"Entity Type"</th>
                                                                <th>"Title"</th>
                                                                <th>"Canvas Last Updated"</th>
                                                                <th>"Discourse Last Updated"</th>
                                                                <th>"Actions"</th>
                                                            </tr>
                                                        </thead>
                                                        <tbody>
                                                            {conflict_list.into_iter().map(|conflict| {
                                                                let conflict_clone = conflict.clone();
                                                                view! {
                                                                    <tr>
                                                                        <td>{conflict.entity_type}</td>
                                                                        <td>{conflict.title}</td>
                                                                        <td>{format_datetime(&conflict.canvas_updated_at)}</td>
                                                                        <td>{format_datetime(&conflict.discourse_updated_at)}</td>
                                                                        <td>
                                                                            <button 
                                                                                class="btn btn-sm btn-primary"
                                                                                on:click=move |_| {
                                                                                    set_selected_conflict.set(Some(conflict_clone.clone()));
                                                                                    set_show_conflict_modal.set(true);
                                                                                }
                                                                            >
                                                                                "Resolve"
                                                                            </button>
                                                                        </td>
                                                                    </tr>
                                                                }
                                                            }).collect::<Vec<_>>()}
                                                        </tbody>
                                                    </table>
                                                </div>
                                            }
                                        }
                                    }}
                                </div>
                            </div>
                        </div>
                    </div>
                }
            } else {
                view! { <></> }
            }}
        </div>
        
        {move || if show_conflict_modal.get() {
            view! {
                <ConflictResolver 
                    conflict=selected_conflict.get()
                    on_close=move |_| set_show_conflict_modal.set(false)
                    on_resolve=move |conflict_id, strategy| {
                        set_show_conflict_modal.set(false);
                        resolve_conflict(conflict_id, strategy);
                    }
                />
            }
        } else {
            view! { <></> }
        }}
    }
    
    // Function to resolve a conflict
    fn resolve_conflict(conflict_id: String, strategy: ConflictResolutionStrategy) {
        spawn_local(async move {
            match IntegrationService::resolve_sync_conflict(&conflict_id, strategy).await {
                Ok(_) => {
                    // Reload data after conflict resolution
                    load_data();
                },
                Err(err) => {
                    // Handle error
                    set_error.set(Some(format!("Failed to resolve conflict: {}", err)));
                }
            }
        });
    }
}

// Helper function to format datetime
fn format_datetime(datetime_str: &str) -> String {
    // Simple formatting for now, could be enhanced with proper date parsing
    datetime_str.replace('T', " ").replace('Z', "")
}

// Import components
use crate::components::integration::{
    CoursesList,
    AssignmentsList,
    SyncHistory,
};
