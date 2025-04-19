use leptos::*;
use crate::models::network::{SyncItem, SyncOperation};
use std::rc::Rc;

/// Props for the SyncQueueManager component
#[derive(Props, Clone)]
pub struct SyncQueueManagerProps {
    /// List of pending sync items
    pub pending_items: Signal<Vec<SyncItem>>,
    
    /// Callback to sync all items
    #[prop(default = None)]
    pub on_sync_all: Option<Callback<()>>,
    
    /// Callback to sync a specific item
    #[prop(default = None)]
    pub on_sync_item: Option<Callback<String>>,
    
    /// Callback to remove an item from the queue
    #[prop(default = None)]
    pub on_remove_item: Option<Callback<String>>,
    
    /// Callback to clear the queue
    #[prop(default = None)]
    pub on_clear_queue: Option<Callback<()>>,
    
    /// Callback to prioritize an item
    #[prop(default = None)]
    pub on_prioritize: Option<Callback<(String, u32)>>,
    
    /// Whether the app is offline
    #[prop(default = Signal::derive(move || false))]
    pub is_offline: Signal<bool>,
    
    /// Whether sync is in progress
    #[prop(default = Signal::derive(move || false))]
    pub is_syncing: Signal<bool>,
    
    /// CSS class for the component
    #[prop(default = "".to_string())]
    pub class: String,
}

/// A component that displays and manages the sync queue
#[component]
pub fn SyncQueueManager(props: SyncQueueManagerProps) -> impl IntoView {
    let SyncQueueManagerProps {
        pending_items,
        on_sync_all,
        on_sync_item,
        on_remove_item,
        on_clear_queue,
        on_prioritize,
        is_offline,
        is_syncing,
        class,
    } = props;
    
    // Derived signals
    let has_items = create_memo(move |_| !pending_items.get().is_empty());
    let item_count = create_memo(move |_| pending_items.get().len());
    
    // Handle sync all
    let handle_sync_all = move |_| {
        if let Some(callback) = on_sync_all.clone() {
            callback.call(());
        }
    };
    
    // Handle sync item
    let handle_sync_item = move |id: String| {
        if let Some(callback) = on_sync_item.clone() {
            callback.call(id);
        }
    };
    
    // Handle remove item
    let handle_remove_item = move |id: String| {
        if let Some(callback) = on_remove_item.clone() {
            callback.call(id);
        }
    };
    
    // Handle clear queue
    let handle_clear_queue = move |_| {
        if let Some(callback) = on_clear_queue.clone() {
            callback.call(());
        }
    };
    
    // Handle prioritize
    let handle_prioritize = move |id: String, priority: u32| {
        if let Some(callback) = on_prioritize.clone() {
            callback.call((id, priority));
        }
    };
    
    view! {
        <div class=format!("sync-queue-manager {}", class)>
            <div class="queue-header">
                <h3 class="queue-title">
                    "Sync Queue"
                    {move || {
                        let count = item_count.get();
                        if count > 0 {
                            view! {
                                <span class="item-count">
                                    " ("
                                    {count}
                                    " items)"
                                </span>
                            }.into_view()
                        } else {
                            view! {}.into_view()
                        }
                    }}
                </h3>
                
                <div class="queue-actions">
                    {move || {
                        if has_items.get() && !is_offline.get() && on_sync_all.is_some() {
                            view! {
                                <button
                                    class="sync-all-button"
                                    on:click=handle_sync_all
                                    disabled=is_syncing.get()
                                >
                                    {if is_syncing.get() { "Syncing..." } else { "Sync All" }}
                                </button>
                            }.into_view()
                        } else {
                            view! {}.into_view()
                        }
                    }}
                    
                    {move || {
                        if has_items.get() && on_clear_queue.is_some() {
                            view! {
                                <button
                                    class="clear-queue-button"
                                    on:click=handle_clear_queue
                                    disabled=is_syncing.get()
                                >
                                    "Clear Queue"
                                </button>
                            }.into_view()
                        } else {
                            view! {}.into_view()
                        }
                    }}
                </div>
            </div>
            
            <div class="queue-content">
                {move || {
                    let items = pending_items.get();
                    if items.is_empty() {
                        view! {
                            <div class="no-items">
                                "No pending sync items"
                            </div>
                        }.into_view()
                    } else {
                        items.into_iter().map(|item| {
                            let item_id = item.id.clone();
                            let entity_id = item.entity_id.clone();
                            let entity_type = item.entity_type.clone();
                            let operation = item.operation;
                            let priority = item.priority;
                            let retry_count = item.retry_count;
                            
                            view! {
                                <div class="queue-item">
                                    <div class="item-info">
                                        <div class="item-operation">
                                            {match operation {
                                                SyncOperation::Create => view! {
                                                    <span class="operation create">
                                                        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                                            <circle cx="12" cy="12" r="10"></circle>
                                                            <line x1="12" y1="8" x2="12" y2="16"></line>
                                                            <line x1="8" y1="12" x2="16" y2="12"></line>
                                                        </svg>
                                                        "Create"
                                                    </span>
                                                }.into_view(),
                                                SyncOperation::Update => view! {
                                                    <span class="operation update">
                                                        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                                            <path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"></path>
                                                            <path d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z"></path>
                                                        </svg>
                                                        "Update"
                                                    </span>
                                                }.into_view(),
                                                SyncOperation::Delete => view! {
                                                    <span class="operation delete">
                                                        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                                            <polyline points="3 6 5 6 21 6"></polyline>
                                                            <path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"></path>
                                                            <line x1="10" y1="11" x2="10" y2="17"></line>
                                                            <line x1="14" y1="11" x2="14" y2="17"></line>
                                                        </svg>
                                                        "Delete"
                                                    </span>
                                                }.into_view(),
                                            }}
                                        </div>
                                        
                                        <div class="item-entity">
                                            <span class="entity-type">{entity_type}</span>
                                            <span class="entity-id">{entity_id}</span>
                                        </div>
                                        
                                        <div class="item-meta">
                                            <span class="priority">
                                                "Priority: "
                                                {priority}
                                            </span>
                                            
                                            {retry_count > 0 then || view! {
                                                <span class="retry-count">
                                                    "Retries: "
                                                    {retry_count}
                                                </span>
                                            }}
                                        </div>
                                    </div>
                                    
                                    <div class="item-actions">
                                        {!is_offline.get() && on_sync_item.is_some() then || view! {
                                            <button
                                                class="sync-item-button"
                                                on:click=move |_| handle_sync_item(item_id.clone())
                                                disabled=is_syncing.get()
                                            >
                                                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                                    <polyline points="23 4 23 10 17 10"></polyline>
                                                    <polyline points="1 20 1 14 7 14"></polyline>
                                                    <path d="M3.51 9a9 9 0 0 1 14.85-3.36L23 10M1 14l4.64 4.36A9 9 0 0 0 20.49 15"></path>
                                                </svg>
                                            </button>
                                        }}
                                        
                                        {on_prioritize.is_some() then || view! {
                                            <div class="priority-buttons">
                                                <button
                                                    class="priority-up-button"
                                                    on:click=move |_| handle_prioritize(item_id.clone(), priority.saturating_sub(10))
                                                    disabled=is_syncing.get() || priority == 0
                                                >
                                                    <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                                        <polyline points="18 15 12 9 6 15"></polyline>
                                                    </svg>
                                                </button>
                                                
                                                <button
                                                    class="priority-down-button"
                                                    on:click=move |_| handle_prioritize(item_id.clone(), priority + 10)
                                                    disabled=is_syncing.get()
                                                >
                                                    <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                                        <polyline points="6 9 12 15 18 9"></polyline>
                                                    </svg>
                                                </button>
                                            </div>
                                        }}
                                        
                                        {on_remove_item.is_some() then || view! {
                                            <button
                                                class="remove-item-button"
                                                on:click=move |_| handle_remove_item(item_id.clone())
                                                disabled=is_syncing.get()
                                            >
                                                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                                    <line x1="18" y1="6" x2="6" y2="18"></line>
                                                    <line x1="6" y1="6" x2="18" y2="18"></line>
                                                </svg>
                                            </button>
                                        }}
                                    </div>
                                </div>
                            }
                        }).collect_view()
                    }
                }}
            </div>
        </div>
    }
}
