use leptos::*;
use leptos_router::*;
use crate::models::integration::{IntegrationStatus, SyncConflict, ConflictResolutionStrategy};
use crate::services::integration_service::IntegrationService;
use crate::components::shared::ErrorAlert;
use crate::utils::style_manager::use_stylesheet;
use crate::components::integration::conflict_resolver::ConflictResolver;

#[component]
pub fn DiscourseIntegration() -> impl IntoView {
    // Load the integration styles
    use_stylesheet("/styles/integration.css");

    // State
    let (loading, set_loading) = create_signal(false);
    let (active_tab, set_active_tab) = create_signal("topics".to_string());
    let (topics, set_topics) = create_signal(Vec::new());
    let (categories, set_categories) = create_signal(Vec::new());
    let (sync_history, set_sync_history) = create_signal(Vec::new());
    let (integration_status, set_integration_status) = create_signal(IntegrationStatus::default());
    let (error, set_error) = create_signal(None::<String>);
    let (conflicts, set_conflicts) = create_signal(Vec::<SyncConflict>::new());
    let (show_conflict_modal, set_show_conflict_modal) = create_signal(false);
    let (selected_conflict, set_selected_conflict) = create_signal(None::<SyncConflict>);

    // Load data on component mount
    create_effect(move |_| {
        load_data();
    });

    // Function to load data from backend
    let load_data = move || {
        set_loading.set(true);
        set_error.set(None);

        spawn_local(async move {
            match IntegrationService::get_discourse_integration_status().await {
                Ok(status) => {
                    set_integration_status.set(status);

                    if status.connected {
                        // Load topics
                        if let Ok(topics_data) = IntegrationService::get_discourse_topics().await {
                            set_topics.set(topics_data);
                        }

                        // Load categories
                        if let Ok(categories_data) = IntegrationService::get_discourse_categories().await {
                            set_categories.set(categories_data);
                        }

                        // Load sync history
                        if let Ok(history_data) = IntegrationService::get_discourse_sync_history().await {
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

    // Sync all topics
    let sync_all = move |_| {
        set_loading.set(true);

        spawn_local(async move {
            match IntegrationService::sync_all_discourse_topics().await {
                Ok(result) => {
                    load_data();
                },
                Err(err) => {
                    set_error.set(Some(format!("Synchronization failed: {}", err)));
                    set_loading.set(false);
                }
            }
        });
    };

    // Sync a specific topic
    let sync_topic = move |topic_id: String| {
        spawn_local(async move {
            match IntegrationService::sync_discourse_topic(&topic_id).await {
                Ok(_) => {
                    load_data();
                },
                Err(err) => {
                    set_error.set(Some(format!("Topic sync failed: {}", err)));
                }
            }
        });
    };

    // Connect to Discourse
    let connect_discourse = move |_| {
        spawn_local(async move {
            match IntegrationService::setup_discourse_integration().await {
                Ok(_) => {
                    load_data();
                },
                Err(err) => {
                    set_error.set(Some(format!("Failed to connect to Discourse: {}", err)));
                }
            }
        });
    };

    view! {
        <div class="integration-page">
            <div class="page-header">
                <h1 class="page-title">
                    <i class="icon-forum"></i>
                    "Discourse Integration"
                </h1>

                <div class="page-actions">
                    {move || if integration_status.get().connected {
                        view! {
                            <button
                                class="btn btn-primary"
                                on:click=sync_all
                                disabled=loading.get()
                            >
                                <i class="icon-sync"></i>
                                {if loading.get() { "Syncing..." } else { "Sync All" }}
                            </button>
                        }
                    } else {
                        view! {
                            <button
                                class="btn btn-primary"
                                on:click=connect_discourse
                            >
                                "Connect to Discourse"
                            </button>
                        }
                    }}
                </div>
            </div>

            {move || error.get().map(|err| view! { <ErrorAlert message=err /> })}

            <div class="integration-status-card">
                <h2 class="card-title">"Integration Status"</h2>
                <div class="status-overview">
                    <div class="status-item">
                        <span class="status-label">"Status:"</span>
                        <span class="status-value">
                            {move || if integration_status.get().connected {
                                view! { <span class="status-connected">"Connected"</span> }
                            } else {
                                view! { <span class="status-disconnected">"Not Connected"</span> }
                            }}
                        </span>
                    </div>

                    <div class="status-item">
                        <span class="status-label">"Last Sync:"</span>
                        <span class="status-value">
                            {move || match integration_status.get().last_sync {
                                Some(time) => view! { <span>{format_datetime(&time)}</span> },
                                None => view! { <span class="text-muted">"Never"</span> }
                            }}
                        </span>
                    </div>

                    {move || if integration_status.get().connected {
                        view! {
                            <>
                                <div class="status-item">
                                    <span class="status-label">"Topics:"</span>
                                    <span class="status-value">{topics.get().len()}</span>
                                </div>

                                <div class="status-item">
                                    <span class="status-label">"Categories:"</span>
                                    <span class="status-value">{categories.get().len()}</span>
                                </div>
                            </>
                        }
                    } else {
                        view! { <></> }
                    }}
                </div>
            </div>

            {move || if integration_status.get().connected {
                view! {
                    <div class="integration-tabs">
                        <div class="tabs-header">
                            <button
                                class={if active_tab.get() == "topics" { "tab-active" } else { "" }}
                                on:click=move |_| set_active_tab.set("topics".to_string())
                            >
                                "Topics"
                            </button>
                            <button
                                class={if active_tab.get() == "categories" { "tab-active" } else { "" }}
                                on:click=move |_| set_active_tab.set("categories".to_string())
                            >
                                "Categories"
                            </button>
                            <button
                                class={if active_tab.get() == "history" { "tab-active" } else { "" }}
                                on:click=move |_| set_active_tab.set("history".to_string())
                            >
                                "Sync History"
                            </button>
                        </div>

                        <div class="tabs-content">
                            <div class="tab-panel" style:display={if active_tab.get() == "topics" { "block" } else { "none" }}>
                                <TopicsList topics=topics sync_topic=sync_topic loading=loading />
                            </div>

                            <div class="tab-panel" style:display={if active_tab.get() == "categories" { "block" } else { "none" }}>
                                <CategoriesList categories=categories />
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
                view! {
                    <div class="empty-state">
                        <div class="empty-state-icon">🔄</div>
                        <h3>"Connect to Discourse"</h3>
                        <p>"Set up the integration with Discourse to synchronize forum content."</p>
                    </div>
                }
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

// Helper function to format datetime strings
fn format_datetime(datetime: &str) -> String {
    // In a real implementation, this would parse the datetime and format it nicely
    // For this example, we'll just return the raw string
    datetime.to_string()
}
