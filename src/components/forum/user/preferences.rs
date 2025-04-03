use leptos::*;
use web_sys::SubmitEvent;
use crate::models::user::{UserPreferences, UserPreferencesUpdate};
use crate::services::user::UserService;
use crate::utils::auth::AuthState;

#[component]
pub fn UserPreferences() -> impl IntoView {
    // Get auth state
    let auth_state = use_context::<AuthState>();
    let is_logged_in = move || auth_state.map(|s| s.is_authenticated()).unwrap_or(false);
    let current_user_id = move || auth_state.map(|s| s.user_id()).unwrap_or(0);
    
    // State signals
    let (preferences, set_preferences) = create_signal(None::<UserPreferences>);
    let (loading, set_loading) = create_signal(true);
    let (saving, set_saving) = create_signal(false);
    let (error, set_error) = create_signal(None::<String>);
    let (success, set_success) = create_signal(None::<String>);
    
    // Interface preferences
    let (theme_preference, set_theme_preference) = create_signal("system".to_string());
    let (homepage_view, set_homepage_view) = create_signal("latest".to_string());
    let (posts_per_page, set_posts_per_page) = create_signal(20);
    let (compact_view, set_compact_view) = create_signal(false);
    let (highlight_new_content, set_highlight_new_content) = create_signal(true);
    let (interface_language, set_interface_language) = create_signal("en".to_string());
    
    // Email preferences
    let (enable_email_notifications, set_enable_email_notifications) = create_signal(true);
    let (notify_on_reply, set_notify_on_reply) = create_signal(true);
    let (notify_on_mention, set_notify_on_mention) = create_signal(true);
    let (notify_on_message, set_notify_on_message) = create_signal(true);
    let (digest_emails, set_digest_emails) = create_signal("weekly".to_string());
    let (mailing_list_mode, set_mailing_list_mode) = create_signal(false);
    
    // Privacy preferences
    let (hide_profile, set_hide_profile) = create_signal(false);
    let (hide_online_status, set_hide_online_status) = create_signal(false);
    let (allow_private_messages, set_allow_private_messages) = create_signal(true);
    let (hide_activity, set_hide_activity) = create_signal(false);
    
    // Content preferences
    let (auto_track_topics, set_auto_track_topics) = create_signal(true);
    let (auto_watch_replied, set_auto_watch_replied) = create_signal(true);
    let (include_toc, set_include_toc) = create_signal(true);
    let (default_code_lang, set_default_code_lang) = create_signal("rust".to_string());
    let (link_previews, set_link_previews) = create_signal(true);
    let (embedded_media, set_embedded_media) = create_signal(true);
    
    // Load user preferences
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
            match UserService::get_preferences(user_id).await {
                Ok(prefs) => {
                    // Populate form fields
                    set_theme_preference.set(prefs.theme_preference.clone());
                    set_homepage_view.set(prefs.homepage_view.clone());
                    set_posts_per_page.set(prefs.posts_per_page);
                    set_compact_view.set(prefs.compact_view);
                    set_highlight_new_content.set(prefs.highlight_new_content);
                    set_interface_language.set(prefs.interface_language.clone());
                    
                    set_enable_email_notifications.set(prefs.enable_email_notifications);
                    set_notify_on_reply.set(prefs.notify_on_reply);
                    set_notify_on_mention.set(prefs.notify_on_mention);
                    set_notify_on_message.set(prefs.notify_on_message);
                    set_digest_emails.set(prefs.digest_emails.clone());
                    set_mailing_list_mode.set(prefs.mailing_list_mode);
                    
                    set_hide_profile.set(prefs.hide_profile);
                    set_hide_online_status.set(prefs.hide_online_status);
                    set_allow_private_messages.set(prefs.allow_private_messages);
                    set_hide_activity.set(prefs.hide_activity);
                    
                    set_auto_track_topics.set(prefs.auto_track_topics);
                    set_auto_watch_replied.set(prefs.auto_watch_replied);
                    set_include_toc.set(prefs.include_toc);
                    set_default_code_lang.set(prefs.default_code_lang.clone());
                    set_link_previews.set(prefs.link_previews);
                    set_embedded_media.set(prefs.embedded_media);
                    
                    set_preferences.set(Some(prefs));
                    set_loading.set(false);
                },
                Err(e) => {
                    set_error.set(Some(format!("Failed to load user preferences: {}", e)));
                    set_loading.set(false);
                }
            }
        });
    });
    
    // Save user preferences
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
        let updated_preferences = UserPreferencesUpdate {
            theme_preference: theme_preference(),
            homepage_view: homepage_view(),
            posts_per_page: posts_per_page(),
            compact_view: compact_view(),
            highlight_new_content: highlight_new_content(),
            interface_language: interface_language(),
            
            enable_email_notifications: enable_email_notifications(),
            notify_on_reply: notify_on_reply(),
            notify_on_mention: notify_on_mention(),
            notify_on_message: notify_on_message(),
            digest_emails: digest_emails(),
            mailing_list_mode: mailing_list_mode(),
            
            hide_profile: hide_profile(),
            hide_online_status: hide_online_status(),
            allow_private_messages: allow_private_messages(),
            hide_activity: hide_activity(),
            
            auto_track_topics: auto_track_topics(),
            auto_watch_replied: auto_watch_replied(),
            include_toc: include_toc(),
            default_code_lang: default_code_lang(),
            link_previews: link_previews(),
            embedded_media: embedded_media(),
        };
        
        spawn_local(async move {
            match UserService::update_preferences(user_id, updated_preferences).await {
                Ok(prefs) => {
                    set_preferences.set(Some(prefs));
                    set_success.set(Some("Preferences updated successfully".to_string()));
                    // Refresh auth context if needed for theme changes
                    if let Some(state) = auth_state {
                        state.refresh().await;
                    }
                },
                Err(e) => {
                    set_error.set(Some(format!("Failed to update preferences: {}", e)));
                }
            }
            set_saving.set(false);
        });
    };
    
    // Reset to defaults
    let reset_to_defaults = move |_| {
        if !window().confirm_with_message("Are you sure you want to reset all preferences to default values?").unwrap_or(false) {
            return;
        }
        
        set_saving.set(true);
        set_error.set(None);
        set_success.set(None);
        
        let user_id = current_user_id();
        
        spawn_local(async move {
            match UserService::reset_preferences(user_id).await {
                Ok(default_preferences) => {
                    // Update all form fields with default values
                    set_theme_preference.set(default_preferences.theme_preference.clone());
                    set_homepage_view.set(default_preferences.homepage_view.clone());
                    set_posts_per_page.set(default_preferences.posts_per_page);
                    set_compact_view.set(default_preferences.compact_view);
                    set_highlight_new_content.set(default_preferences.highlight_new_content);
                    set_interface_language.set(default_preferences.interface_language.clone());
                    
                    set_enable_email_notifications.set(default_preferences.enable_email_notifications);
                    set_notify_on_reply.set(default_preferences.notify_on_reply);
                    set_notify_on_mention.set(default_preferences.notify_on_mention);
                    set_notify_on_message.set(default_preferences.notify_on_message);
                    set_digest_emails.set(default_preferences.digest_emails.clone());
                    set_mailing_list_mode.set(default_preferences.mailing_list_mode);
                    
                    set_hide_profile.set(default_preferences.hide_profile);
                    set_hide_online_status.set(default_preferences.hide_online_status);
                    set_allow_private_messages.set(default_preferences.allow_private_messages);
                    set_hide_activity.set(default_preferences.hide_activity);
                    
                    set_auto_track_topics.set(default_preferences.auto_track_topics);
                    set_auto_watch_replied.set(default_preferences.auto_watch_replied);
                    set_include_toc.set(default_preferences.include_toc);
                    set_default_code_lang.set(default_preferences.default_code_lang.clone());
                    set_link_previews.set(default_preferences.link_previews);
                    set_embedded_media.set(default_preferences.embedded_media);
                    
                    set_preferences.set(Some(default_preferences));
                    set_success.set(Some("Preferences reset to defaults successfully".to_string()));
                    
                    // Refresh auth context if needed for theme changes
                    if let Some(state) = auth_state {
                        state.refresh().await;
                    }
                },
                Err(e) => {
                    set_error.set(Some(format!("Failed to reset preferences: {}", e)));
                }
            }
            set_saving.set(false);
        });
    };

    view! {
        <div class="user-preferences">
            {move || if !is_logged_in() {
                view! {
                    <div class="alert alert-warning">
                        "You must be logged in to view your preferences"
                    </div>
                }
            } else {
                view! {
                    <div>
                        <h1 class="mb-4">"User Preferences"</h1>
                        
                        {move || error().map(|err| view! { <div class="alert alert-danger mb-4">{err}</div> })}
                        {move || success().map(|msg| view! { <div class="alert alert-success mb-4">{msg}</div> })}
                        
                        {move || if loading() {
                            view! { <div class="d-flex justify-content-center p-5"><div class="spinner-border" role="status"></div></div> }
                        } else {
                            view! {
                                <form on:submit=save_preferences>
                                    <ul class="nav nav-tabs mb-4" role="tablist">
                                        <li class="nav-item" role="presentation">
                                            <button
                                                class="nav-link active"
                                                id="interface-tab"
                                                data-bs-toggle="tab"
                                                data-bs-target="#interface"
                                                type="button"
                                                role="tab"
                                                aria-controls="interface"
                                                aria-selected="true"
                                            >
                                                <i class="bi bi-display me-1"></i>
                                                "Interface"
                                            </button>
                                        </li>
                                        <li class="nav-item" role="presentation">
                                            <button
                                                class="nav-link"
                                                id="notifications-tab"
                                                data-bs-toggle="tab"
                                                data-bs-target="#notifications"
                                                type="button"
                                                role="tab"
                                                aria-controls="notifications"
                                                aria-selected="false"
                                            >
                                                <i class="bi bi-bell me-1"></i>
                                                "Notifications"
                                            </button>
                                        </li>
                                        <li class="nav-item" role="presentation">
                                            <button
                                                class="nav-link"
                                                id="privacy-tab"
                                                data-bs-toggle="tab"
                                                data-bs-target="#privacy"
                                                type="button"
                                                role="tab"
                                                aria-controls="privacy"
                                                aria-selected="false"
                                            >
                                                <i class="bi bi-shield-lock me-1"></i>
                                                "Privacy"
                                            </button>
                                        </li>
                                        <li class="nav-item" role="presentation">
                                            <button
                                                class="nav-link"
                                                id="content-tab"
                                                data-bs-toggle="tab"
                                                data-bs-target="#content"
                                                type="button"
                                                role="tab"
                                                aria-controls="content"
                                                aria-selected="false"
                                            >
                                                <i class="bi bi-layout-text-window me-1"></i>
                                                "Content"
                                            </button>
                                        </li>
                                    </ul>
                                    
                                    <div class="tab-content">
                                        // Interface Tab
                                        <div class="tab-pane fade show active" id="interface" role="tabpanel" aria-labelledby="interface-tab">
                                            <div class="card mb-4">
                                                <div class="card-header">
                                                    <h5 class="mb-0">"Interface Preferences"</h5>
                                                </div>
                                                <div class="card-body">
                                                    <div class="mb-3">
                                                        <label for="themePreference" class="form-label">"Theme"</label>
                                                        <select
                                                            id="themePreference"
                                                            class="form-select"
                                                            prop:value=move || theme_preference()
                                                            on:change=move |ev| set_theme_preference.set(event_target_value(&ev))
                                                        >
                                                            <option value="system">"Use System Setting"</option>
                                                            <option value="light">"Light"</option>
                                                            <option value="dark">"Dark"</option>
                                                        </select>
                                                    </div>
                                                    
                                                    <div class="mb-3">
                                                        <label for="homepageView" class="form-label">"Default Homepage View"</label>
                                                        <select
                                                            id="homepageView"
                                                            class="form-select"
                                                            prop:value=move || homepage_view()
                                                            on:change=move |ev| set_homepage_view.set(event_target_value(&ev))
                                                        >
                                                            <option value="latest">"Latest Topics"</option>
                                                            <option value="top">"Top Topics"</option>
                                                            <option value="unread">"Unread Topics"</option>
                                                            <option value="categories">"Categories"</option>
                                                        </select>
                                                    </div>
                                                    
                                                    <div class="mb-3">
                                                        <label for="postsPerPage" class="form-label">"Posts Per Page"</label>
                                                        <select
                                                            id="postsPerPage"
                                                            class="form-select"
                                                            prop:value=move || posts_per_page().to_string()
                                                            on:change=move |ev| {
                                                                if let Ok(val) = event_target_value(&ev).parse::<i32>() {
                                                                    set_posts_per_page.set(val);
                                                                }
                                                            }
                                                        >
                                                            <option value="10">"10"</option>
                                                            <option value="20">"20"</option>
                                                            <option value="30">"30"</option>
                                                            <option value="50">"50"</option>
                                                            <option value="100">"100"</option>
                                                        </select>
                                                    </div>
                                                    
                                                    <div class="mb-3">
                                                        <label for="interfaceLanguage" class="form-label">"Interface Language"</label>
                                                        <select
                                                            id="interfaceLanguage"
                                                            class="form-select"
                                                            prop:value=move || interface_language()
                                                            on:change=move |ev| set_interface_language.set(event_target_value(&ev))
                                                        >
                                                            <option value="en">"English"</option>
                                                            <option value="es">"Español (Spanish)"</option>
                                                            <option value="fr">"Français (French)"</option>
                                                            <option value="de">"Deutsch (German)"</option>
                                                            <option value="zh">"中文 (Chinese)"</option>
                                                            <option value="ja">"日本語 (Japanese)"</option>
                                                            <option value="ru">"Русский (Russian)"</option>
                                                        </select>
                                                    </div>
                                                    
                                                    <div class="mb-3">
                                                        <div class="form-check form-switch">
                                                            <input
                                                                class="form-check-input"
                                                                type="checkbox"
                                                                id="compactView"
                                                                prop:checked=move || compact_view()
                                                                on:change=move |ev| set_compact_view.set(event_target_checked(&ev))
                                                            />
                                                            <label class="form-check-label" for="compactView">
                                                                "Compact View Mode"
                                                            </label>
                                                            <div class="form-text">
                                                                "Shows more content with less whitespace"
                                                            </div>
                                                        </div>
                                                    </div>
                                                    
                                                    <div class="mb-3">
                                                        <div class="form-check form-switch">
                                                            <input
                                                                class="form-check-input"
                                                                type="checkbox"
                                                                id="highlightNewContent"
                                                                prop:checked=move || highlight_new_content()
                                                                on:change=move |ev| set_highlight_new_content.set(event_target_checked(&ev))
                                                            />
                                                            <label class="form-check-label" for="highlightNewContent">
                                                                "Highlight New Content"
                                                            </label>
                                                            <div class="form-text">
                                                                "Visually highlight content that's been posted since your last visit"
                                                            </div>
                                                        </div>
                                                    </div>
                                                </div>
                                            </div>
                                        </div>
                                        
                                        // Notifications Tab
                                        <div class="tab-pane fade" id="notifications" role="tabpanel" aria-labelledby="notifications-tab">
                                            <div class="card mb-4">
                                                <div class="card-header">
                                                    <h5 class="mb-0">"Email & Notifications"</h5>
                                                </div>
                                                <div class="card-body">
                                                    <div class="mb-3">
                                                        <div class="form-check form-switch">
                                                            <input
                                                                class="form-check-input"
                                                                type="checkbox"
                                                                id="enableEmailNotifications"
                                                                prop:checked=move || enable_email_notifications()
                                                                on:change=move |ev| set_enable_email_notifications.set(event_target_checked(&ev))
                                                            />
                                                            <label class="form-check-label" for="enableEmailNotifications">
                                                                "Enable Email Notifications"
                                                            </label>
                                                        </div>
                                                    </div>
                                                    
                                                    <fieldset disabled=move || !enable_email_notifications()>
                                                        <div class="mb-3">
                                                            <div class="form-check mb-2">
                                                                <input
                                                                    class="form-check-input"
                                                                    type="checkbox"
                                                                    id="notifyOnReply"
                                                                    prop:checked=move || notify_on_reply()
                                                                    on:change=move |ev| set_notify_on_reply.set(event_target_checked(&ev))
                                                                />
                                                                <label class="form-check-label" for="notifyOnReply">
                                                                    "Email me when someone replies to my post"
                                                                </label>
                                                            </div>
                                                            <div class="form-check mb-2">
                                                                <input
                                                                    class="form-check-input"
                                                                    type="checkbox"
                                                                    id="notifyOnMention"
                                                                    prop:checked=move || notify_on_mention()
                                                                    on:change=move |ev| set_notify_on_mention.set(event_target_checked(&ev))
                                                                />
                                                                <label class="form-check-label" for="notifyOnMention">
                                                                    "Email me when someone mentions my @username"
                                                                </label>
                                                            </div>
                                                            <div class="form-check mb-2">
                                                                <input
                                                                    class="form-check-input"
                                                                    type="checkbox"
                                                                    id="notifyOnMessage"
                                                                    prop:checked=move || notify_on_message()
                                                                    on:change=move |ev| set_notify_on_message.set(event_target_checked(&ev))
                                                                />
                                                                <label class="form-check-label" for="notifyOnMessage">
                                                                    "Email me when someone sends me a private message"
                                                                </label>
                                                            </div>
                                                        </div>
                                                        
                                                        <div class="mb-3">
                                                            <label for="digestEmails" class="form-label">"Email Digests"</label>
                                                            <select
                                                                id="digestEmails"
                                                                class="form-select"
                                                                prop:value=move || digest_emails()
                                                                on:change=move |ev| set_digest_emails.set(event_target_value(&ev))
                                                            >
                                                                <option value="none">"Never send me digest emails"</option>
                                                                <option value="daily">"Daily digest of new content"</option>
                                                                <option value="weekly">"Weekly digest of new content"</option>
                                                            </select>
                                                            <div class="form-text">
                                                                "Digest emails contain a summary of activity in topics you follow"
                                                            </div>
                                                        </div>
                                                        
                                                        <div class="mb-3">
                                                            <div class="form-check form-switch">
                                                                <input
                                                                    class="form-check-input"
                                                                    type="checkbox"
                                                                    id="mailingListMode"
                                                                    prop:checked=move || mailing_list_mode()
                                                                    on:change=move |ev| set_mailing_list_mode.set(event_target_checked(&ev))
                                                                />
                                                                <label class="form-check-label" for="mailingListMode">
                                                                    "Mailing List Mode"
                                                                </label>
                                                                <div class="form-text">
                                                                    "When enabled, you'll receive an email for every new post in topics you follow, similar to a mailing list"
                                                                </div>
                                                            </div>
                                                        </div>
                                                    </fieldset>
                                                </div>
                                            </div>
                                        </div>
                                        
                                        // Privacy Tab
                                        <div class="tab-pane fade" id="privacy" role="tabpanel" aria-labelledby="privacy-tab">
                                            <div class="card mb-4">
                                                <div class="card-header">
                                                    <h5 class="mb-0">"Privacy Settings"</h5>
                                                </div>
                                                <div class="card-body">
                                                    <div class="mb-3">
                                                        <div class="form-check form-switch">
                                                            <input
                                                                class="form-check-input"
                                                                type="checkbox"
                                                                id="hideProfile"
                                                                prop:checked=move || hide_profile()
                                                                on:change=move |ev| set_hide_profile.set(event_target_checked(&ev))
                                                            />
                                                            <label class="form-check-label" for="hideProfile">
                                                                "Hide my profile from public view"
                                                            </label>
                                                            <div class="form-text">
                                                                "Only logged-in users can view your profile"
                                                            </div>
                                                        </div>
                                                    </div>
                                                    
                                                    <div class="mb-3">
                                                        <div class="form-check form-switch">
                                                            <input
                                                                class="form-check-input"
                                                                type="checkbox"
                                                                id="hideOnlineStatus"
                                                                prop:checked=move || hide_online_status()
                                                                on:change=move |ev| set_hide_online_status.set(event_target_checked(&ev))
                                                            />
                                                            <label class="form-check-label" for="hideOnlineStatus">
                                                                "Hide my online status"
                                                            </label>
                                                        </div>
                                                    </div>
                                                    
                                                    <div class="mb-3">
                                                        <div class="form-check form-switch">
                                                            <input
                                                                class="form-check-input"
                                                                type="checkbox"
                                                                id="allowPrivateMessages"
                                                                prop:checked=move || allow_private_messages()
                                                                on:change=move |ev| set_allow_private_messages.set(event_target_checked(&ev))
                                                            />
                                                            <label class="form-check-label" for="allowPrivateMessages">
                                                                "Allow others to send me private messages"
                                                            </label>
                                                        </div>
                                                    </div>
                                                    
                                                    <div class="mb-3">
                                                        <div class="form-check form-switch">
                                                            <input
                                                                class="form-check-input"
                                                                type="checkbox"
                                                                id="hideActivity"
                                                                prop:checked=move || hide_activity()
                                                                on:change=move |ev| set_hide_activity.set(event_target_checked(&ev))
                                                            />
                                                            <label class="form-check-label" for="hideActivity">
                                                                "Hide my activity feed"
                                                            </label>
                                                            <div class="form-text">
                                                                "Others won't see your recent posts and topics in your profile"
                                                            </div>
                                                        </div>
                                                    </div>
                                                </div>
                                            </div>
                                        </div>
                                        
                                        // Content Tab
                                        <div class="tab-pane fade" id="content" role="tabpanel" aria-labelledby="content-tab">
                                            <div class="card mb-4">
                                                <div class="card-header">
                                                    <h5 class="mb-0">"Content & Reading Preferences"</h5>
                                                </div>
                                                <div class="card-body">
                                                    <div class="mb-3">
                                                        <div class="form-check form-switch">
                                                            <input
                                                                class="form-check-input"
                                                                type="checkbox"
                                                                id="autoTrackTopics"
                                                                prop:checked=move || auto_track_topics()
                                                                on:change=move |ev| set_auto_track_topics.set(event_target_checked(&ev))
                                                            />
                                                            <label class="form-check-label" for="autoTrackTopics">
                                                                "Automatically track topics I visit"
                                                            </label>
                                                            <div class="form-text">
                                                                "Topics you view will be added to your watched list"
                                                            </div>
                                                        </div>
                                                    </div>
                                                    
                                                    <div class="mb-3">
                                                        <div class="form-check form-switch">
                                                            <input
                                                                class="form-check-input"
                                                                type="checkbox"
                                                                id="autoWatchReplied"
                                                                prop:checked=move || auto_watch_replied()
                                                                on:change=move |ev| set_auto_watch_replied.set(event_target_checked(&ev))
                                                            />
                                                            <label class="form-check-label" for="autoWatchReplied">
                                                                "Automatically watch topics I reply to"
                                                            </label>
                                                        </div>
                                                    </div>
                                                    
                                                    <div class="mb-3">
                                                        <div class="form-check form-switch">
                                                            <input
                                                                class="form-check-input"
                                                                type="checkbox"
                                                                id="includeToc"
                                                                prop:checked=move || include_toc()
                                                                on:change=move |ev| set_include_toc.set(event_target_checked(&ev))
                                                            />
                                                            <label class="form-check-label" for="includeToc">
                                                                "Show table of contents in long posts"
                                                            </label>
                                                        </div>
                                                    </div>
                                                    
                                                    <div class="mb-3">
                                                        <label for="defaultCodeLang" class="form-label">"Default Code Block Language"</label>
                                                        <select
                                                            id="defaultCodeLang"
                                                            class="form-select"
                                                            prop:value=move || default_code_lang()
                                                            on:change=move |ev| set_default_code_lang.set(event_target_value(&ev))
                                                        >
                                                            <option value="">"Plain text (no highlighting)"</option>
                                                            <option value="rust">"Rust"</option>
                                                            <option value="javascript">"JavaScript"</option>
                                                            <option value="python">"Python"</option>
                                                            <option value="html">"HTML"</option>
                                                            <option value="css">"CSS"</option>
                                                            <option value="java">"Java"</option>
                                                            <option value="csharp">"C#"</option>
                                                            <option value="cpp">"C++"</option>
                                                            <option value="go">"Go"</option>
                                                            <option value="sql">"SQL"</option>
                                                        </select>
                                                    </div>
                                                    
                                                    <div class="mb-3">
                                                        <div class="form-check form-switch">
                                                            <input
                                                                class="form-check-input"
                                                                type="checkbox"
                                                                id="linkPreviews"
                                                                prop:checked=move || link_previews()
                                                                on:change=move |ev| set_link_previews.set(event_target_checked(&ev))
                                                            />
                                                            <label class="form-check-label" for="linkPreviews">
                                                                "Show link previews"
                                                            </label>
                                                            <div class="form-text">
                                                                "Display rich previews for links in posts"
                                                            </div>
                                                        </div>
                                                    </div>
                                                    
                                                    <div class="mb-3">
                                                        <div class="form-check form-switch">
                                                            <input
                                                                class="form-check-input"
                                                                type="checkbox"
                                                                id="embeddedMedia"
                                                                prop:checked=move || embedded_media()
                                                                on:change=move |ev| set_embedded_media.set(event_target_checked(&ev))
                                                            />
                                                            <label class="form-check-label" for="embeddedMedia">
                                                                "Show embedded media in posts"
                                                            </label>
                                                            <div class="form-text">
                                                                "Automatically embed videos, tweets, and other content"
                                                            </div>
                                                        </div>
                                                    </div>
                                                </div>
                                            </div>
                                        </div>
                                    </div>
                                    
                                    <div class="d-flex justify-content-between mt-4">
                                        <button 
                                            type="button" 
                                            class="btn btn-outline-danger"
                                            on:click=reset_to_defaults
                                        >
                                            "Reset to Defaults"
                                        </button>
                                        
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