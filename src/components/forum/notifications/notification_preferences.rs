use leptos::*;
use web_sys::SubmitEvent;
use crate::models::notification::NotificationPreferences;
use crate::services::notification::NotificationService;
use crate::utils::auth::AuthState;

#[component]
pub fn NotificationPreferencesPage() -> impl IntoView {
    // Get auth state
    let auth_state = use_context::<AuthState>();
    let is_logged_in = move || auth_state.map(|s| s.is_authenticated()).unwrap_or(false);
    let current_user_id = move || auth_state.map(|s| s.user_id()).unwrap_or(0);
    
    // State signals
    let (preferences, set_preferences) = create_signal(None::<NotificationPreferences>);
    let (loading, set_loading) = create_signal(true);
    let (saving, set_saving) = create_signal(false);
    let (error, set_error) = create_signal(None::<String>);
    let (success, set_success) = create_signal(None::<String>);
    
    // Form state bindings
    let (enable_browser_notifications, set_enable_browser_notifications) = create_signal(true);
    let (enable_email_notifications, set_enable_email_notifications) = create_signal(true);
    let (mentions_notification, set_mentions_notification) = create_signal(true);
    let (replies_notification, set_replies_notification) = create_signal(true);
    let (quotes_notification, set_quotes_notification) = create_signal(true);
    let (likes_notification, set_likes_notification) = create_signal(true);
    let (messages_notification, set_messages_notification) = create_signal(true);
    let (follows_notification, set_follows_notification) = create_signal(true);
    let (group_mentions_notification, set_group_mentions_notification) = create_signal(true);
    let (group_messages_notification, set_group_messages_notification) = create_signal(true);
    let (digest_emails, set_digest_emails) = create_signal("weekly".to_string());
    
    // Load notification preferences
    create_effect(move |_| {
        if !is_logged_in() {
            set_loading.set(false);
            return;
        }
        
        let user_id = current_user_id();
        if user_id == 0 {
            set_loading.set(false);
            return;
        }
        
        spawn_local(async move {
            match NotificationService::get_notification_preferences(user_id).await {
                Ok(prefs) => {
                    // Set form state from preferences
                    set_enable_browser_notifications.set(prefs.enable_browser_notifications);
                    set_enable_email_notifications.set(prefs.enable_email_notifications);
                    set_mentions_notification.set(prefs.mentions_notification);
                    set_replies_notification.set(prefs.replies_notification);
                    set_quotes_notification.set(prefs.quotes_notification);
                    set_likes_notification.set(prefs.likes_notification);
                    set_messages_notification.set(prefs.messages_notification);
                    set_follows_notification.set(prefs.follows_notification);
                    set_group_mentions_notification.set(prefs.group_mentions_notification);
                    set_group_messages_notification.set(prefs.group_messages_notification);
                    set_digest_emails.set(prefs.digest_emails.clone());
                    
                    set_preferences.set(Some(prefs));
                    set_loading.set(false);
                },
                Err(e) => {
                    set_error.set(Some(format!("Failed to load notification preferences: {}", e)));
                    set_loading.set(false);
                }
            }
        });
    });
    
    // Save notification preferences
    let save_preferences = move |ev: SubmitEvent| {
        ev.prevent_default();
        
        if !is_logged_in() {
            set_error.set(Some("You must be logged in to save preferences".to_string()));
            return;
        }
        
        set_saving.set(true);
        set_error.set(None);
        set_success.set(None);
        
        let user_id = current_user_id();
        let updated_preferences = NotificationPreferences {
            user_id,
            enable_browser_notifications: enable_browser_notifications(),
            enable_email_notifications: enable_email_notifications(),
            mentions_notification: mentions_notification(),
            replies_notification: replies_notification(),
            quotes_notification: quotes_notification(),
            likes_notification: likes_notification(),
            messages_notification: messages_notification(),
            follows_notification: follows_notification(),
            group_mentions_notification: group_mentions_notification(),
            group_messages_notification: group_messages_notification(),
            digest_emails: digest_emails(),
        };
        
        spawn_local(async move {
            match NotificationService::update_notification_preferences(user_id, updated_preferences).await {
                Ok(prefs) => {
                    set_preferences.set(Some(prefs));
                    set_success.set(Some("Notification preferences updated successfully".to_string()));
                    
                    // Clear success message after 3 seconds
                    spawn_local(async {
                        leptos::timeout(3000, move || {
                            set_success.set(None);
                        }).await;
                    });
                },
                Err(e) => {
                    set_error.set(Some(format!("Failed to update preferences: {}", e)));
                }
            }
            set_saving.set(false);
        });
    };
    
    // Request browser notification permission
    let request_notification_permission = move |_| {
        spawn_local(async {
            let window = web_sys::window().unwrap();
            let notification = window.get("Notification").unwrap();
            let permission_status = js_sys::Reflect::get(&notification, &"permission".into()).unwrap();
            
            if permission_status.as_string().unwrap() == "default" {
                let result = wasm_bindgen_futures::JsFuture::from(
                    js_sys::Promise::from(
                        js_sys::Reflect::apply(
                            &js_sys::Reflect::get(&notification, &"requestPermission".into()).unwrap(),
                            &notification,
                            &js_sys::Array::new(),
                        ).unwrap()
                    )
                ).await.unwrap();
                
                if result.as_string().unwrap() == "granted" {
                    set_enable_browser_notifications.set(true);
                } else {
                    set_enable_browser_notifications.set(false);
                }
            }
        });
    };

    view! {
        <div class="notification-preferences">
            {move || if !is_logged_in() {
                view! {
                    <div class="alert alert-warning">
                        "You must be logged in to view your notification preferences"
                    </div>
                }
            } else {
                view! {
                    <div>
                        <div class="d-flex justify-content-between align-items-center mb-4">
                            <h1>"Notification Preferences"</h1>
                            
                            <a href="/notifications" class="btn btn-outline-secondary">
                                <i class="bi bi-bell me-1"></i>
                                "View Notifications"
                            </a>
                        </div>
                        
                        {move || error().map(|err| view! { <div class="alert alert-danger mb-4">{err}</div> })}
                        {move || success().map(|msg| view! { <div class="alert alert-success mb-4">{msg}</div> })}
                        
                        {move || if loading() {
                            view! { <div class="d-flex justify-content-center p-5"><div class="spinner-border" role="status"></div></div> }
                        } else {
                            view! {
                                <form on:submit=save_preferences>
                                    <div class="row">
                                        <div class="col-lg-6">
                                            <div class="card mb-4">
                                                <div class="card-header">
                                                    <h5 class="mb-0">"Notification Channels"</h5>
                                                </div>
                                                <div class="card-body">
                                                    <div class="mb-4">
                                                        <div class="form-check form-switch">
                                                            <input
                                                                class="form-check-input"
                                                                type="checkbox"
                                                                id="enableBrowserNotifications"
                                                                prop:checked=move || enable_browser_notifications()
                                                                on:change=move |ev| set_enable_browser_notifications.set(event_target_checked(&ev))
                                                            />
                                                            <label class="form-check-label" for="enableBrowserNotifications">
                                                                "Enable browser notifications"
                                                            </label>
                                                            <div class="form-text">
                                                                "Receive notifications in your browser when you're on the site"
                                                            </div>
                                                            <button 
                                                                type="button" 
                                                                class="btn btn-sm btn-outline-primary mt-2"
                                                                on:click=request_notification_permission
                                                            >
                                                                "Grant notification permission"
                                                            </button>
                                                        </div>
                                                    </div>
                                                    
                                                    <div class="mb-4">
                                                        <div class="form-check form-switch">
                                                            <input
                                                                class="form-check-input"
                                                                type="checkbox"
                                                                id="enableEmailNotifications"
                                                                prop:checked=move || enable_email_notifications()
                                                                on:change=move |ev| set_enable_email_notifications.set(event_target_checked(&ev))
                                                            />
                                                            <label class="form-check-label" for="enableEmailNotifications">
                                                                "Enable email notifications"
                                                            </label>
                                                            <div class="form-text">
                                                                "Receive email notifications when you're away from the site"
                                                            </div>
                                                        </div>
                                                    </div>
                                                    
                                                    <div class="mb-4">
                                                        <label for="digestEmails" class="form-label">"Email Digest Frequency"</label>
                                                        <select
                                                            id="digestEmails"
                                                            class="form-select"
                                                            prop:value=move || digest_emails()
                                                            on:change=move |ev| set_digest_emails.set(event_target_value(&ev))
                                                            disabled=move || !enable_email_notifications()
                                                        >
                                                            <option value="never">"Never send digest emails"</option>
                                                            <option value="daily">"Daily digest"</option>
                                                            <option value="weekly">"Weekly digest"</option>
                                                        </select>
                                                        <div class="form-text">
                                                            "Receive a summary of activity in your subscribed topics"
                                                        </div>
                                                    </div>
                                                </div>
                                            </div>
                                        </div>
                                        
                                        <div class="col-lg-6">
                                            <div class="card mb-4">
                                                <div class="card-header">
                                                    <h5 class="mb-0">"Notification Events"</h5>
                                                </div>
                                                <div class="card-body">
                                                    <p class="text-muted mb-3">
                                                        "Select which events you want to be notified about:"
                                                    </p>
                                                    
                                                    <div class="mb-3">
                                                        <div class="form-check form-switch">
                                                            <input
                                                                class="form-check-input"
                                                                type="checkbox"
                                                                id="mentionsNotification"
                                                                prop:checked=move || mentions_notification()
                                                                on:change=move |ev| set_mentions_notification.set(event_target_checked(&ev))
                                                            />
                                                            <label class="form-check-label" for="mentionsNotification">
                                                                "When someone mentions me"
                                                            </label>
                                                        </div>
                                                    </div>
                                                    
                                                    <div class="mb-3">
                                                        <div class="form-check form-switch">
                                                            <input
                                                                class="form-check-input"
                                                                type="checkbox"
                                                                id="repliesNotification"
                                                                prop:checked=move || replies_notification()
                                                                on:change=move |ev| set_replies_notification.set(event_target_checked(&ev))
                                                            />
                                                            <label class="form-check-label" for="repliesNotification">
                                                                "When someone replies to my post"
                                                            </label>
                                                        </div>
                                                    </div>
                                                    
                                                    <div class="mb-3">
                                                        <div class="form-check form-switch">
                                                            <input
                                                                class="form-check-input"
                                                                type="checkbox"
                                                                id="quotesNotification"
                                                                prop:checked=move || quotes_notification()
                                                                on:change=move |ev| set_quotes_notification.set(event_target_checked(&ev))
                                                            />
                                                            <label class="form-check-label" for="quotesNotification">
                                                                "When someone quotes me"
                                                            </label>
                                                        </div>
                                                    </div>
                                                    
                                                    <div class="mb-3">
                                                        <div class="form-check form-switch">
                                                            <input
                                                                class="form-check-input"
                                                                type="checkbox"
                                                                id="likesNotification"
                                                                prop:checked=move || likes_notification()
                                                                on:change=move |ev| set_likes_notification.set(event_target_checked(&ev))
                                                            />
                                                            <label class="form-check-label" for="likesNotification">
                                                                "When someone likes my post"
                                                            </label>
                                                        </div>
                                                    </div>
                                                    
                                                    <div class="mb-3">
                                                        <div class="form-check form-switch">
                                                            <input
                                                                class="form-check-input"
                                                                type="checkbox"
                                                                id="messagesNotification"
                                                                prop:checked=move || messages_notification()
                                                                on:change=move |ev| set_messages_notification.set(event_target_checked(&ev))
                                                            />
                                                            <label class="form-check-label" for="messagesNotification">
                                                                "When I receive a private message"
                                                            </label>
                                                        </div>
                                                    </div>
                                                    
                                                    <div class="mb-3">
                                                        <div class="form-check form-switch">
                                                            <input
                                                                class="form-check-input"
                                                                type="checkbox"
                                                                id="followsNotification"
                                                                prop:checked=move || follows_notification()
                                                                on:change=move |ev| set_follows_notification.set(event_target_checked(&ev))
                                                            />
                                                            <label class="form-check-label" for="followsNotification">
                                                                "When someone follows me"
                                                            </label>
                                                        </div>
                                                    </div>
                                                    
                                                    <div class="mb-3">
                                                        <div class="form-check form-switch">
                                                            <input
                                                                class="form-check-input"
                                                                type="checkbox"
                                                                id="groupMentionsNotification"
                                                                prop:checked=move || group_mentions_notification()
                                                                on:change=move |ev| set_group_mentions_notification.set(event_target_checked(&ev))
                                                            />
                                                            <label class="form-check-label" for="groupMentionsNotification">
                                                                "When someone mentions a group I'm in"
                                                            </label>
                                                        </div>
                                                    </div>
                                                    
                                                    <div class="mb-3">
                                                        <div class="form-check form-switch">
                                                            <input
                                                                class="form-check-input"
                                                                type="checkbox"
                                                                id="groupMessagesNotification"
                                                                prop:checked=move || group_messages_notification()
                                                                on:change=move |ev| set_group_messages_notification.set(event_target_checked(&ev))
                                                            />
                                                            <label class="form-check-label" for="groupMessagesNotification">
                                                                "When someone sends a message to a group I'm in"
                                                            </label>
                                                        </div>
                                                    </div>
                                                </div>
                                            </div>
                                        </div>
                                    </div>
                                    
                                    <div class="d-flex justify-content-end mt-4">
                                        <button 
                                            type="submit" 
                                            class="btn btn-primary"
                                            disabled=move || saving()
                                        >
                                            {move || if saving() {
                                                view! { <span class="spinner-border spinner-border-sm me-2" role="status"></span> "Saving..." }
                                            } else {
                                                view! { "Save Preferences" }
                                            }}
                                        </button>
                                    </div>
                                </form>
                            }
                        }}
                    </div>
                }
            }}
        </div>
    }
}