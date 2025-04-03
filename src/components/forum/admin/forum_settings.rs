use leptos::*;
use web_sys::SubmitEvent;
use crate::services::admin::AdminService;
use crate::models::admin::{ForumSettings, ForumSettingsUpdate};

#[component]
pub fn ForumSettings() -> impl IntoView {
    // Admin permission check
    let auth_state = use_context::<AuthState>();
    let is_admin = move || auth_state.map(|s| s.is_admin()).unwrap_or(false);
    
    // State signals
    let (settings, set_settings) = create_signal(None::<ForumSettings>);
    let (loading, set_loading) = create_signal(true);
    let (saving, set_saving) = create_signal(false);
    let (error, set_error) = create_signal(None::<String>);
    let (success, set_success) = create_signal(None::<String>);
    
    // Form signals
    let (forum_name, set_forum_name) = create_signal(String::new());
    let (forum_description, set_forum_description) = create_signal(String::new());
    let (logo_url, set_logo_url) = create_signal(String::new());
    let (favicon_url, set_favicon_url) = create_signal(String::new());
    let (primary_color, set_primary_color) = create_signal("#0d6efd".to_string());
    let (allow_guest_viewing, set_allow_guest_viewing) = create_signal(true);
    let (allow_registration, set_allow_registration) = create_signal(true);
    let (require_email_verification, set_require_email_verification) = create_signal(true);
    let (posts_per_page, set_posts_per_page) = create_signal(20);
    let (topics_per_page, set_topics_per_page) = create_signal(20);
    let (max_topic_title_length, set_max_topic_title_length) = create_signal(100);
    let (min_post_length, set_min_post_length) = create_signal(20);
    let (max_post_length, set_max_post_length) = create_signal(50000);
    
    // Load forum settings
    create_effect(move |_| {
        if (!is_admin()) {
            set_loading.set(false);
            return;
        }
        
        spawn_local(async move {
            match AdminService::get_forum_settings().await {
                Ok(forum_settings) => {
                    // Populate form fields
                    set_forum_name.set(forum_settings.forum_name.clone());
                    set_forum_description.set(forum_settings.forum_description.clone());
                    set_logo_url.set(forum_settings.logo_url.clone().unwrap_or_default());
                    set_favicon_url.set(forum_settings.favicon_url.clone().unwrap_or_default());
                    set_primary_color.set(forum_settings.primary_color.clone());
                    set_allow_guest_viewing.set(forum_settings.allow_guest_viewing);
                    set_allow_registration.set(forum_settings.allow_registration);
                    set_require_email_verification.set(forum_settings.require_email_verification);
                    set_posts_per_page.set(forum_settings.posts_per_page);
                    set_topics_per_page.set(forum_settings.topics_per_page);
                    set_max_topic_title_length.set(forum_settings.max_topic_title_length);
                    set_min_post_length.set(forum_settings.min_post_length);
                    set_max_post_length.set(forum_settings.max_post_length);
                    
                    set_settings.set(Some(forum_settings));
                    set_loading.set(false);
                },
                Err(e) => {
                    set_error.set(Some(format!("Failed to load forum settings: {}", e)));
                    set_loading.set(false);
                }
            }
        });
    });
    
    // Save forum settings
    let handle_save = move |ev: SubmitEvent| {
        ev.prevent_default();
        
        set_saving.set(true);
        set_error.set(None);
        set_success.set(None);
        
        let settings_update = ForumSettingsUpdate {
            forum_name: forum_name(),
            forum_description: forum_description(),
            logo_url: if logo_url().is_empty() { None } else { Some(logo_url()) },
            favicon_url: if favicon_url().is_empty() { None } else { Some(favicon_url()) },
            primary_color: primary_color(),
            allow_guest_viewing: allow_guest_viewing(),
            allow_registration: allow_registration(),
            require_email_verification: require_email_verification(),
            posts_per_page: posts_per_page(),
            topics_per_page: topics_per_page(),
            max_topic_title_length: max_topic_title_length(),
            min_post_length: min_post_length(),
            max_post_length: max_post_length(),
        };
        
        spawn_local(async move {
            match AdminService::update_forum_settings(settings_update).await {
                Ok(updated_settings) => {
                    set_settings.set(Some(updated_settings));
                    set_success.set(Some("Forum settings updated successfully".to_string()));
                    set_saving.set(false);
                },
                Err(e) => {
                    set_error.set(Some(format!("Failed to update settings: {}", e)));
                    set_saving.set(false);
                }
            }
        });
    };

    view! {
        <div class="forum-settings">
            {move || if !is_admin() {
                view! {
                    <div class="alert alert-danger">
                        "You don't have permission to access this page."
                    </div>
                }
            } else if loading() {
                view! { <div class="d-flex justify-content-center p-5"><div class="spinner-border" role="status"></div></div> }
            } else {
                view! {
                    <div>
                        <h1 class="mb-4">"Forum Settings"</h1>
                        
                        {move || error().map(|err| view! {
                            <div class="alert alert-danger">{err}</div>
                        })}
                        
                        {move || success().map(|msg| view! {
                            <div class="alert alert-success">{msg}</div>
                        })}
                        
                        <form on:submit=handle_save>
                            <div class="card mb-4">
                                <div class="card-header">
                                    <h5 class="mb-0">"General Settings"</h5>
                                </div>
                                <div class="card-body">
                                    <div class="mb-3">
                                        <label for="forumName" class="form-label">"Forum Name"</label>
                                        <input 
                                            id="forumName"
                                            type="text"
                                            class="form-control"
                                            prop:value=move || forum_name()
                                            on:input=move |ev| set_forum_name.set(event_target_value(&ev))
                                            required
                                        />
                                    </div>
                                    
                                    <div class="mb-3">
                                        <label for="forumDescription" class="form-label">"Forum Description"</label>
                                        <textarea
                                            id="forumDescription"
                                            class="form-control"
                                            prop:value=move || forum_description()
                                            on:input=move |ev| set_forum_description.set(event_target_value(&ev))
                                            rows="3"
                                        ></textarea>
                                    </div>
                                    
                                    <div class="row">
                                        <div class="col-md-6 mb-3">
                                            <label for="logoUrl" class="form-label">"Logo URL"</label>
                                            <input 
                                                id="logoUrl"
                                                type="url"
                                                class="form-control"
                                                prop:value=move || logo_url()
                                                on:input=move |ev| set_logo_url.set(event_target_value(&ev))
                                                placeholder="https://example.com/logo.png"
                                            />
                                            <div class="form-text">
                                                "URL to your forum logo image"
                                            </div>
                                        </div>
                                        
                                        <div class="col-md-6 mb-3">
                                            <label for="faviconUrl" class="form-label">"Favicon URL"</label>
                                            <input 
                                                id="faviconUrl"
                                                type="url"
                                                class="form-control"
                                                prop:value=move || favicon_url()
                                                on:input=move |ev| set_favicon_url.set(event_target_value(&ev))
                                                placeholder="https://example.com/favicon.ico"
                                            />
                                            <div class="form-text">
                                                "URL to your forum favicon"
                                            </div>
                                        </div>
                                    </div>
                                    
                                    <div class="mb-3">
                                        <label for="primaryColor" class="form-label">"Primary Color"</label>
                                        <div class="input-group">
                                            <input
                                                id="primaryColor"
                                                type="color"
                                                class="form-control form-control-color"
                                                prop:value=move || primary_color()
                                                on:input=move |ev| set_primary_color.set(event_target_value(&ev))
                                            />
                                            <input
                                                type="text"
                                                class="form-control"
                                                prop:value=move || primary_color()
                                                on:input=move |ev| set_primary_color.set(event_target_value(&ev))
                                            />
                                        </div>
                                        <div class="form-text">
                                            "Primary brand color for the forum"
                                        </div>
                                    </div>
                                </div>
                            </div>
                            
                            <div class="card mb-4">
                                <div class="card-header">
                                    <h5 class="mb-0">"Access Settings"</h5>
                                </div>
                                <div class="card-body">
                                    <div class="form-check form-switch mb-3">
                                        <input
                                            class="form-check-input"
                                            type="checkbox"
                                            id="allowGuestViewing"
                                            prop:checked=move || allow_guest_viewing()
                                            on:change=move |ev| set_allow_guest_viewing.set(event_target_checked(&ev))
                                        />
                                        <label class="form-check-label" for="allowGuestViewing">
                                            "Allow Guest Viewing"
                                        </label>
                                        <div class="form-text">
                                            "If enabled, non-registered users can view forum content"
                                        </div>
                                    </div>
                                    
                                    <div class="form-check form-switch mb-3">
                                        <input
                                            class="form-check-input"
                                            type="checkbox"
                                            id="allowRegistration"
                                            prop:checked=move || allow_registration()
                                            on:change=move |ev| set_allow_registration.set(event_target_checked(&ev))
                                        />
                                        <label class="form-check-label" for="allowRegistration">
                                            "Allow New User Registration"
                                        </label>
                                        <div class="form-text">
                                            "If disabled, new users cannot register"
                                        </div>
                                    </div>
                                    
                                    <div class="form-check form-switch mb-3">
                                        <input
                                            class="form-check-input"
                                            type="checkbox"
                                            id="requireEmailVerification"
                                            prop:checked=move || require_email_verification()
                                            on:change=move |ev| set_require_email_verification.set(event_target_checked(&ev))
                                        />
                                        <label class="form-check-label" for="requireEmailVerification">
                                            "Require Email Verification"
                                        </label>
                                        <div class="form-text">
                                            "If enabled, new users must verify their email address"
                                        </div>
                                    </div>
                                </div>
                            </div>
                            
                            <div class="card mb-4">
                                <div class="card-header">
                                    <h5 class="mb-0">"Content Settings"</h5>
                                </div>
                                <div class="card-body">
                                    <div class="row mb-3">
                                        <div class="col-md-6">
                                            <label for="postsPerPage" class="form-label">"Posts Per Page"</label>
                                            <input
                                                id="postsPerPage"
                                                type="number"
                                                class="form-control"
                                                prop:value=move || posts_per_page()
                                                on:input=move |ev| {
                                                    if let Ok(val) = event_target_value(&ev).parse::<i32>() {
                                                        set_posts_per_page.set(val);
                                                    }
                                                }
                                                min="5"
                                                max="100"
                                                required
                                            />
                                        </div>
                                        
                                        <div class="col-md-6">
                                            <label for="topicsPerPage" class="form-label">"Topics Per Page"</label>
                                            <input
                                                id="topicsPerPage"
                                                type="number"
                                                class="form-control"
                                                prop:value=move || topics_per_page()
                                                on:input=move |ev| {
                                                    if let Ok(val) = event_target_value(&ev).parse::<i32>() {
                                                        set_topics_per_page.set(val);
                                                    }
                                                }
                                                min="5"
                                                max="100"
                                                required
                                            />
                                        </div>
                                    </div>
                                    
                                    <div class="mb-3">
                                        <label for="maxTopicTitleLength" class="form-label">"Max Topic Title Length"</label>
                                        <input
                                            id="maxTopicTitleLength"
                                            type="number"
                                            class="form-control"
                                            prop:value=move || max_topic_title_length()
                                            on:input=move |ev| {
                                                if let Ok(val) = event_target_value(&ev).parse::<i32>() {
                                                    set_max_topic_title_length.set(val);
                                                }
                                            }
                                            min="20"
                                            max="500"
                                            required
                                        />
                                    </div>
                                    
                                    <div class="row mb-3">
                                        <div class="col-md-6">
                                            <label for="minPostLength" class="form-label">"Min Post Length"</label>
                                            <input
                                                id="minPostLength"
                                                type="number"
                                                class="form-control"
                                                prop:value=move || min_post_length()
                                                on:input=move |ev| {
                                                    if let Ok(val) = event_target_value(&ev).parse::<i32>() {
                                                        set_min_post_length.set(val);
                                                    }
                                                }
                                                min="1"
                                                max="1000"
                                                required
                                            />
                                        </div>
                                        
                                        <div class="col-md-6">
                                            <label for="maxPostLength" class="form-label">"Max Post Length"</label>
                                            <input
                                                id="maxPostLength"
                                                type="number"
                                                class="form-control"
                                                prop:value=move || max_post_length()
                                                on:input=move |ev| {
                                                    if let Ok(val) = event_target_value(&ev).parse::<i32>() {
                                                        set_max_post_length.set(val);
                                                    }
                                                }
                                                min="1000"
                                                max="1000000"
                                                required
                                            />
                                        </div>
                                    </div>
                                </div>
                            </div>
                            
                            <div class="d-flex justify-content-end">
                                <button type="submit" class="btn btn-primary" disabled=move || saving()>
                                    {move || if saving() {
                                        view! { <span class="spinner-border spinner-border-sm me-2" role="status"></span> "Saving..." }
                                    } else {
                                        view! { "Save Settings" }
                                    }}
                                </button>
                            </div>
                        </form>
                    </div>
                }
            }}
        </div>
    }
}
