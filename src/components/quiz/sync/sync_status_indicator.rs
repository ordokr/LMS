use leptos::*;
use crate::models::network::{ConnectionStatus, SyncStatus};
use std::rc::Rc;

/// Props for the SyncStatusIndicator component
#[derive(Props, Clone)]
pub struct SyncStatusIndicatorProps {
    /// Current connection status
    pub connection_status: Signal<ConnectionStatus>,
    
    /// Current sync status
    pub sync_status: Signal<SyncStatus>,
    
    /// Number of pending sync items
    pub pending_count: Signal<usize>,
    
    /// Callback to trigger a manual sync
    #[prop(default = None)]
    pub on_sync_now: Option<Callback<()>>,
    
    /// Whether to show detailed information
    #[prop(default = false)]
    pub detailed: bool,
    
    /// CSS class for the indicator
    #[prop(default = "".to_string())]
    pub class: String,
}

/// A component that displays the current sync status
#[component]
pub fn SyncStatusIndicator(props: SyncStatusIndicatorProps) -> impl IntoView {
    let SyncStatusIndicatorProps {
        connection_status,
        sync_status,
        pending_count,
        on_sync_now,
        detailed,
        class,
    } = props;
    
    // Derived signals
    let is_offline = create_memo(move |_| connection_status.get() == ConnectionStatus::Offline);
    let is_syncing = create_memo(move |_| sync_status.get() == SyncStatus::Syncing);
    let has_pending = create_memo(move |_| pending_count.get() > 0);
    let sync_failed = create_memo(move |_| sync_status.get() == SyncStatus::Failed);
    
    // Handle sync now button click
    let handle_sync_now = move |_| {
        if let Some(callback) = on_sync_now.clone() {
            callback.call(());
        }
    };
    
    view! {
        <div class=format!("sync-status-indicator {}", class)>
            <div class="sync-status-icon">
                {move || {
                    if is_offline.get() {
                        view! {
                            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="offline-icon">
                                <line x1="1" y1="1" x2="23" y2="23"></line>
                                <path d="M16.72 11.06A10.94 10.94 0 0 1 19 12.55"></path>
                                <path d="M5 12.55a10.94 10.94 0 0 1 5.17-2.39"></path>
                                <path d="M10.71 5.05A16 16 0 0 1 22.58 9"></path>
                                <path d="M1.42 9a15.91 15.91 0 0 1 4.7-2.88"></path>
                                <path d="M8.53 16.11a6 6 0 0 1 6.95 0"></path>
                                <line x1="12" y1="20" x2="12.01" y2="20"></line>
                            </svg>
                        }.into_view()
                    } else if is_syncing.get() {
                        view! {
                            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="syncing-icon">
                                <polyline points="23 4 23 10 17 10"></polyline>
                                <polyline points="1 20 1 14 7 14"></polyline>
                                <path d="M3.51 9a9 9 0 0 1 14.85-3.36L23 10M1 14l4.64 4.36A9 9 0 0 0 20.49 15"></path>
                            </svg>
                        }.into_view()
                    } else if sync_failed.get() {
                        view! {
                            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="sync-failed-icon">
                                <circle cx="12" cy="12" r="10"></circle>
                                <line x1="12" y1="8" x2="12" y2="12"></line>
                                <line x1="12" y1="16" x2="12.01" y2="16"></line>
                            </svg>
                        }.into_view()
                    } else if has_pending.get() {
                        view! {
                            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="pending-icon">
                                <circle cx="12" cy="12" r="10"></circle>
                                <polyline points="12 6 12 12 16 14"></polyline>
                            </svg>
                        }.into_view()
                    } else {
                        view! {
                            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="synced-icon">
                                <path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"></path>
                                <polyline points="22 4 12 14.01 9 11.01"></polyline>
                            </svg>
                        }.into_view()
                    }
                }}
            </div>
            
            <div class="sync-status-text">
                {move || {
                    if is_offline.get() {
                        view! {
                            <span class="status-text offline">
                                "Offline"
                                {has_pending.get().then(|| view! {
                                    <span class="pending-count">
                                        " ("
                                        {pending_count.get()}
                                        " pending)"
                                    </span>
                                })}
                            </span>
                        }.into_view()
                    } else if is_syncing.get() {
                        view! {
                            <span class="status-text syncing">"Syncing..."</span>
                        }.into_view()
                    } else if sync_failed.get() {
                        view! {
                            <span class="status-text failed">"Sync Failed"</span>
                        }.into_view()
                    } else if has_pending.get() {
                        view! {
                            <span class="status-text pending">
                                "Pending Sync"
                                <span class="pending-count">
                                    " ("
                                    {pending_count.get()}
                                    " items)"
                                </span>
                            </span>
                        }.into_view()
                    } else {
                        view! {
                            <span class="status-text synced">"Synced"</span>
                        }.into_view()
                    }
                }}
            </div>
            
            {move || {
                if detailed && (has_pending.get() || sync_failed.get()) && !is_offline.get() && on_sync_now.is_some() {
                    view! {
                        <button
                            class="sync-now-button"
                            on:click=handle_sync_now
                            disabled=is_syncing.get()
                        >
                            "Sync Now"
                        </button>
                    }.into_view()
                } else {
                    view! {}.into_view()
                }
            }}
        </div>
    }
}

/// Props for the SyncNotificationList component
#[derive(Props, Clone)]
pub struct SyncNotificationListProps {
    /// List of notifications
    pub notifications: Signal<Vec<SyncNotification>>,
    
    /// Callback when a notification is clicked
    #[prop(default = None)]
    pub on_notification_click: Option<Callback<SyncNotification>>,
    
    /// Callback when a notification is marked as read
    #[prop(default = None)]
    pub on_mark_read: Option<Callback<String>>,
    
    /// Callback when all notifications are marked as read
    #[prop(default = None)]
    pub on_mark_all_read: Option<Callback<()>>,
    
    /// Callback when a notification is dismissed
    #[prop(default = None)]
    pub on_dismiss: Option<Callback<String>>,
    
    /// CSS class for the list
    #[prop(default = "".to_string())]
    pub class: String,
}

/// Sync notification data
#[derive(Clone, Debug)]
pub struct SyncNotification {
    /// Notification ID
    pub id: String,
    
    /// Notification title
    pub title: String,
    
    /// Notification message
    pub message: String,
    
    /// Notification type
    pub notification_type: SyncNotificationType,
    
    /// Timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    
    /// Whether the notification has been read
    pub read: bool,
}

/// Sync notification type
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SyncNotificationType {
    /// Sync completed successfully
    SyncComplete,
    
    /// Sync failed
    SyncFailed,
    
    /// Conflict detected
    Conflict,
    
    /// New data available
    NewData,
}

/// A component that displays a list of sync notifications
#[component]
pub fn SyncNotificationList(props: SyncNotificationListProps) -> impl IntoView {
    let SyncNotificationListProps {
        notifications,
        on_notification_click,
        on_mark_read,
        on_mark_all_read,
        on_dismiss,
        class,
    } = props;
    
    // Derived signals
    let unread_count = create_memo(move |_| {
        notifications.get().iter().filter(|n| !n.read).count()
    });
    
    // Handle notification click
    let handle_notification_click = move |notification: SyncNotification| {
        if let Some(callback) = on_notification_click.clone() {
            callback.call(notification);
        }
    };
    
    // Handle mark as read
    let handle_mark_read = move |id: String| {
        if let Some(callback) = on_mark_read.clone() {
            callback.call(id);
        }
    };
    
    // Handle mark all as read
    let handle_mark_all_read = move |_| {
        if let Some(callback) = on_mark_all_read.clone() {
            callback.call(());
        }
    };
    
    // Handle dismiss
    let handle_dismiss = move |id: String| {
        if let Some(callback) = on_dismiss.clone() {
            callback.call(id);
        }
    };
    
    view! {
        <div class=format!("sync-notification-list {}", class)>
            <div class="notification-header">
                <h3 class="notification-title">
                    "Sync Notifications"
                    {move || {
                        let count = unread_count.get();
                        if count > 0 {
                            view! {
                                <span class="unread-count">{count}</span>
                            }.into_view()
                        } else {
                            view! {}.into_view()
                        }
                    }}
                </h3>
                
                {move || {
                    if unread_count.get() > 0 && on_mark_all_read.is_some() {
                        view! {
                            <button
                                class="mark-all-read-button"
                                on:click=handle_mark_all_read
                            >
                                "Mark All Read"
                            </button>
                        }.into_view()
                    } else {
                        view! {}.into_view()
                    }
                }}
            </div>
            
            <div class="notification-list">
                {move || {
                    let notifs = notifications.get();
                    if notifs.is_empty() {
                        view! {
                            <div class="no-notifications">
                                "No notifications"
                            </div>
                        }.into_view()
                    } else {
                        notifs.into_iter().map(|notification| {
                            let notification_clone = notification.clone();
                            let id = notification.id.clone();
                            
                            view! {
                                <div 
                                    class=format!("notification-item {}", if notification.read { "read" } else { "unread" })
                                    on:click=move |_| handle_notification_click(notification_clone.clone())
                                >
                                    <div class="notification-icon">
                                        {match notification.notification_type {
                                            SyncNotificationType::SyncComplete => view! {
                                                <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="sync-complete-icon">
                                                    <path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"></path>
                                                    <polyline points="22 4 12 14.01 9 11.01"></polyline>
                                                </svg>
                                            }.into_view(),
                                            SyncNotificationType::SyncFailed => view! {
                                                <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="sync-failed-icon">
                                                    <circle cx="12" cy="12" r="10"></circle>
                                                    <line x1="12" y1="8" x2="12" y2="12"></line>
                                                    <line x1="12" y1="16" x2="12.01" y2="16"></line>
                                                </svg>
                                            }.into_view(),
                                            SyncNotificationType::Conflict => view! {
                                                <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="conflict-icon">
                                                    <polygon points="11 5 6 9 2 9 2 15 6 15 11 19 11 5"></polygon>
                                                    <path d="M15.54 8.46a5 5 0 0 1 0 7.07"></path>
                                                    <path d="M19.07 4.93a10 10 0 0 1 0 14.14"></path>
                                                </svg>
                                            }.into_view(),
                                            SyncNotificationType::NewData => view! {
                                                <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="new-data-icon">
                                                    <circle cx="12" cy="12" r="10"></circle>
                                                    <line x1="12" y1="8" x2="12" y2="16"></line>
                                                    <line x1="8" y1="12" x2="16" y2="12"></line>
                                                </svg>
                                            }.into_view(),
                                        }}
                                    </div>
                                    
                                    <div class="notification-content">
                                        <div class="notification-title">{notification.title}</div>
                                        <div class="notification-message">{notification.message}</div>
                                        <div class="notification-time">
                                            {notification.timestamp.format("%H:%M:%S %d/%m/%Y").to_string()}
                                        </div>
                                    </div>
                                    
                                    <div class="notification-actions">
                                        {!notification.read && on_mark_read.is_some() then || view! {
                                            <button
                                                class="mark-read-button"
                                                on:click=move |e| {
                                                    e.stop_propagation();
                                                    handle_mark_read(id.clone());
                                                }
                                            >
                                                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                                    <path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"></path>
                                                    <polyline points="22 4 12 14.01 9 11.01"></polyline>
                                                </svg>
                                            </button>
                                        }}
                                        
                                        {on_dismiss.is_some() then || view! {
                                            <button
                                                class="dismiss-button"
                                                on:click=move |e| {
                                                    e.stop_propagation();
                                                    handle_dismiss(id.clone());
                                                }
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
