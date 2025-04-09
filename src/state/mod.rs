mod app_state;
mod persistence;
mod diagnostics;

pub use app_state::{AppState, AppStore, ForumState, StateSection, Theme, User, UserRole, Notification, NotificationType};
pub use persistence::StatePersistence;
pub use diagnostics::{StateMonitoring, AppStateMetricsReport};

use leptos::*;
use std::sync::Arc;

// Create and provide global app state
pub fn provide_app_state() -> AppStore {
    let app_store = AppStore::new();
    
    // Setup persistence
    let _persistence = StatePersistence::new(
        app_store.clone(),
        "edu_connect_state",
        true // auto-save
    );
    
    // Provide to context
    provide_context(app_store.clone());
    
    app_store
}

// Component using efficient state access
#[component]
pub fn ForumNavigation() -> impl IntoView {
    let app_store = use_context::<AppStore>().expect("AppStore should be provided");
    
    // Use fine-grained signals for better performance
    let forum_state = create_read_slice(
        app_store.get_state(),
        |state| state.forum.clone()
    );
    
    let last_visited = create_memo(move |_| {
        let state = forum_state.get();
        state.last_visited_topic_ids.iter().take(5).cloned().collect::<Vec<_>>()
    });
    
    view! {
        <div class="forum-navigation">
            <h3>"Recent Topics"</h3>
            <ul>
                {move || last_visited.get().into_iter().map(|topic_id| {
                    view! {
                        <li>
                            <a href={format!("/forum/topic/{}", topic_id)}>
                                {"Topic #"}{topic_id}
                            </a>
                        </li>
                    }
                }).collect::<Vec<_>>()}
            </ul>
        </div>
    }
}

// Batch update usage example
pub fn handle_user_login(app_store: &AppStore, user: User) {
    app_store.batch_update(|batch| {
        batch.set_user(Some(user));
        batch.set_online(true);
        batch.add_notification(Notification {
            id: uuid::Uuid::new_v4().to_string(),
            message: "You've successfully logged in".to_string(),
            read: false,
            type_: NotificationType::Success,
            timestamp: chrono::Utc::now().to_rfc3339(),
        });
    });
}