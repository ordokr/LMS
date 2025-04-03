use leptos::*;
use crate::models::user::TopicSubscription;
use crate::services::user::UserService;
use crate::utils::auth::AuthState;

#[component]
pub fn SubscriptionButton(
    #[prop(into)] topic_id: i64,
) -> impl IntoView {
    // Get auth state
    let auth_state = use_context::<AuthState>();
    let is_logged_in = move || auth_state.map(|s| s.is_authenticated()).unwrap_or(false);
    let current_user_id = move || auth_state.map(|s| s.user_id()).unwrap_or(0);
    
    // State signals
    let (subscription_level, set_subscription_level) = create_signal(String::from("normal"));
    let (loading, set_loading) = create_signal(true);
    let (updating, set_updating) = create_signal(false);
    let (dropdown_open, set_dropdown_open) = create_signal(false);
    
    // Check subscription status
    create_effect(move |_| {
        if !is_logged_in() {
            set_loading.set(false);
            return;
        }
        
        let user_id = current_user_id();
        let topic = topic_id;
        
        spawn_local(async move {
            match UserService::get_topic_subscriptions(user_id).await {
                Ok(subscriptions) => {
                    if let Some(subscription) = subscriptions.iter().find(|s| s.topic_id == topic) {
                        set_subscription_level.set(subscription.notification_level.clone());
                    } else {
                        set_subscription_level.set(String::from("normal"));
                    }
                },
                Err(_) => {
                    set_subscription_level.set(String::from("normal"));
                }
            }
            set_loading.set(false);
        });
    });
    
    // Update subscription level
    let update_subscription = move |level: String| {
        if !is_logged_in() {
            // Redirect to login or show login modal
            return;
        }
        
        let user_id = current_user_id();
        let topic = topic_id;
        
        set_updating.set(true);
        set_dropdown_open.set(false);
        
        spawn_local(async move {
            match UserService::update_topic_subscription(user_id, topic, level.clone()).await {
                Ok(_) => {
                    set_subscription_level.set(level);
                },
                Err(_) => {}
            }
            set_updating.set(false);
        });
    };
    
    // Toggle dropdown
    let toggle_dropdown = move |_| {
        set_dropdown_open.update(|open| *open = !*open);
    };

    view! {
        <div class="dropdown">
            <button 
                class="btn btn-sm btn-outline-secondary dropdown-toggle" 
                type="button" 
                on:click=toggle_dropdown
                disabled=move || loading() || updating()
                aria-expanded=move || dropdown_open()
            >
                {move || if loading() || updating() {
                    view! { <span class="spinner-border spinner-border-sm" role="status"></span> }
                } else {
                    match subscription_level().as_str() {
                        "watching" => view! { <><i class="bi bi-eye-fill me-1"></i> "Watching"</> },
                        "tracking" => view! { <><i class="bi bi-bell-fill me-1"></i> "Tracking"</> },
                        "muted" => view! { <><i class="bi bi-bell-slash-fill me-1"></i> "Muted"</> },
                        _ => view! { <><i class="bi bi-bell me-1"></i> "Normal"</> }
                    }
                }}
            </button>
            <ul class="dropdown-menu" class:show=move || dropdown_open()>
                <li>
                    <button 
                        class="dropdown-item" 
                        class:active=move || subscription_level() == "watching"
                        on:click=move |_| update_subscription("watching".to_string())
                    >
                        <i class="bi bi-eye-fill me-2"></i>
                        <span>"Watching"</span>
                        <small class="d-block text-muted ms-4">
                            "Get notified of all new posts"
                        </small>
                    </button>
                </li>
                <li>
                    <button 
                        class="dropdown-item" 
                        class:active=move || subscription_level() == "tracking"
                        on:click=move |_| update_subscription("tracking".to_string())
                    >
                        <i class="bi bi-bell-fill me-2"></i>
                        <span>"Tracking"</span>
                        <small class="d-block text-muted ms-4">
                            "Track unread posts, no notifications"
                        </small>
                    </button>
                </li>
                <li>
                    <button 
                        class="dropdown-item" 
                        class:active=move || subscription_level() == "normal"
                        on:click=move |_| update_subscription("normal".to_string())
                    >
                        <i class="bi bi-bell me-2"></i>
                        <span>"Normal"</span>
                        <small class="d-block text-muted ms-4">
                            "Notified if someone mentions you"
                        </small>
                    </button>
                </li>
                <li>
                    <button 
                        class="dropdown-item" 
                        class:active=move || subscription_level() == "muted"
                        on:click=move |_| update_subscription("muted".to_string())
                    >
                        <i class="bi bi-bell-slash-fill me-2"></i>
                        <span>"Muted"</span>
                        <small class="d-block text-muted ms-4">
                            "Never notified, hide from latest"
                        </small>
                    </button>
                </li>
            </ul>
        </div>
    }
}