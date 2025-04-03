use leptos::*;
use web_sys::SubmitEvent;
use crate::services::admin::AdminService;
use crate::models::admin::NotificationSettings;

#[component]
pub fn NotificationSettings() -> impl IntoView {
    // Admin permission check
    let auth_state = use_context::<AuthState>();
    let is_admin = move || auth_state.map(|s| s.is_admin()).unwrap_or(false);
    
    // State signals
    let (settings, set_settings) = create_signal(None::<NotificationSettings>);
    let (loading, set_loading) = create_signal(true);
    let (saving, set_saving) = create_signal(false);
    let (error, set_error) = create_signal(None::<String>);
    let (success, set_success) = create_signal(None::<String>);
    
    // Form signals
    let (enable_email, set_enable_email) = create_signal(true);
    let (smtp_host, set_smtp_host) = create_signal(String::new());
    let (smtp_port, set_smtp_port) = create_signal(587);
    let (smtp_username, set_smtp_username) = create_signal(String::new());
    let (smtp_password, set_smtp_password) = create_signal(String::new());
    let (smtp_from_email, set_smtp_from_email) = create_signal(String::new());
    let (smtp_from_name, set_smtp_from_name) = create_signal(String::new());
    let (smtp_use_tls, set_smtp_use_tls) = create_signal(true);
    
    let (email_welcome_enabled, set_email_welcome_enabled) = create_signal(true);
    let (email_welcome_subject, set_email_welcome_subject) = create_signal(String::new());
    
    let (email_post_reply_enabled, set_email_post_reply_enabled) = create_signal(true);
    let (email_post_reply_subject, set_email_post_reply_subject) = create_signal(String::new());
    
    let (email_topic_reply_enabled, set_email_topic_reply_enabled) = create_signal(true);
    let (email_topic_reply_subject, set_email_topic_reply_subject) = create_signal(String::new());
    
    let (email_mention_enabled, set_email_mention_enabled) = create_signal(true);
    let (email_mention_subject, set_email_mention_subject) = create_signal(String::new());
    
    // Load notification settings
    create_effect(move |_| {
        if !is_admin() {
            set_loading.set(false);
            return;
        }
        
        spawn_local(async move {
            match AdminService::get_notification_settings().await {
                Ok(loaded_settings) => {
                    // Populate form fields
                    set_enable_email.set(loaded_settings.enable_email);
                    set_smtp_host.set(loaded_settings.smtp_host.clone());
                    set_smtp_port.set(loaded_settings.smtp_port);
                    set_smtp_username.set(loaded_settings.smtp_username.clone());
                    // Don't set password - it's typically not returned for security
                    set_smtp_from_email.set(loaded_settings.smtp_from_email.clone());
                    set_smtp_from_name.set(loaded_settings.smtp_from_name.clone());
                    set_smtp_use_tls.set(loaded_settings.smtp_use_tls);
                    
                    set_email_welcome_enabled.set(loaded_settings.email_welcome_enabled);
                    set_email_welcome_subject.set(loaded_settings.email_welcome_subject.clone());
                    
                    set_email_post_reply_enabled.set(loaded_settings.email_post_reply_enabled);
                    set_email_post_reply_subject.set(loaded_settings.email_post_reply_subject.clone());
                    
                    set_email_topic_reply_enabled.set(loaded_settings.email_topic_reply_enabled);
                    set_email_topic_reply_subject.set(loaded_settings.email_topic_reply_subject.clone());
                    
                    set_email_mention_enabled.set(loaded_settings.email_mention_enabled);
                    set_email_mention_subject.set(loaded_settings.email_mention_subject.clone());
                    
                    set_settings.set(Some(loaded_settings));
                    set_loading.set(false);
                },
                Err(e) => {
                    set_error.set(Some(format!("Failed to load notification settings: {}", e)));
                    set_loading.set(false);
                }
            }
        });
    });
    
    // Save notification settings
    let save_settings = move |ev: SubmitEvent| {
        ev.prevent_default();
        
        set_saving.set(true);
        set_error.set(None);
        set_success.set(None);
        
        let updated_settings = NotificationSettings {
            enable_email: enable_email(),
            smtp_host: smtp_host(),
            smtp_port: smtp_port(),
            smtp_username: smtp_username(),
            smtp_password: smtp_password(),
            smtp_from_email: smtp_from_email(),
            smtp_from_name: smtp_from_name(),
            smtp_use_tls: smtp_use_tls(),
            
            email_welcome_enabled: email_welcome_enabled(),
            email_welcome_subject: email_welcome_subject(),
            
            email_post_reply_enabled: email_post_reply_enabled(),
            email_post_reply_subject: email_post_reply_subject(),
            
            email_topic_reply_enabled: email_topic_reply_enabled(),
            email_topic_reply_subject: email_topic_reply_subject(),
            
            email_mention_enabled: email_mention_enabled(),
            email_mention_subject: email_mention_subject(),
        };
        
        spawn_local(async move {
            match AdminService::update_notification_settings(updated_settings).await {
                Ok(new_settings) => {
                    set_settings.set(Some(new_settings));
                    set_success.set(Some("Notification settings updated successfully".to_string()));
                },
                Err(e) => {
                    set_error.set(Some(format!("Failed to update notification settings: {}", e)));
                }
            }
            set_saving.set(false);
        });
    };
    
    // Send test email
    let send_test_email = move |_| {
        if !enable_email() {
            set_error.set(Some("Email notifications are disabled. Enable them first.".to_string()));
            return;
        }
        
        set_saving.set(true);
        set_error.set(None);
        set_success.set(None);
        
        spawn_local(async move {
            match AdminService::send_test_email().await {
                Ok(_) => {
                    set_success.set(Some("Test email sent successfully. Please check your inbox.".to_string()));
                },
                Err(e) => {
                    set_error.set(Some(format!("Failed to send test email: {}", e)));
                }
            }
            set_saving.set(false);
        });
    };

    view! {
        <div class="notification-settings">
            {move || if !is_admin() {
                view! {
                    <div class="alert alert-danger">
                        "You don't have permission to access this page."
                    </div>
                }
            } else {
                view! {
                    <h1 class="mb-4">"Email Notification Settings"</h1>
                    
                    {move || error().map(|err| view! { <div class="alert alert-danger mb-4">{err}</div> })}
                    {move || success().map(|msg| view! { <div class="alert alert-success mb-4">{msg}</div> })}
                    
                    {move || if loading() {
                        view! { <div class="d-flex justify-content-center p-5"><div class="spinner-border" role="status"></div></div> }
                    } else {
                        view! {
                            <form on:submit=save_settings>
                                <div class="card mb-4">
                                    <div class="card-header d-flex justify-content-between align-items-center">
                                        <h5 class="mb-0">"Email Configuration"</h5>
                                        <div class="form-check form-switch">
                                            <input
                                                class="form-check-input"
                                                type="checkbox"
                                                id="enableEmail"
                                                prop:checked=move || enable_email()
                                                on:change=move |ev| set_enable_email.set(event_target_checked(&ev))
                                            />
                                            <label class="form-check-label" for="enableEmail">
                                                "Enable Email Notifications"
                                            </label>
                                        </div>
                                    </div>
                                    <div class="card-body">
                                        <fieldset disabled=move || !enable_email()>
                                            <div class="row mb-3">
                                                <div class="col-md-8">
                                                    <label for="smtpHost" class="form-label">"SMTP Host"</label>
                                                    <input
                                                        id="smtpHost"
                                                        type="text"
                                                        class="form-control"
                                                        prop:value=move || smtp_host()
                                                        on:input=move |ev| set_smtp_host.set(event_target_value(&ev))
                                                        placeholder="smtp.example.com"
                                                        required
                                                    />
                                                </div>
                                                <div class="col-md-4">
                                                    <label for="smtpPort" class="form-label">"SMTP Port"</label>
                                                    <input
                                                        id="smtpPort"
                                                        type="number"
                                                        class="form-control"
                                                        prop:value=move || smtp_port()
                                                        on:input=move |ev| {
                                                            if let Ok(port) = event_target_value(&ev).parse::<i32>() {
                                                                set_smtp_port.set(port);
                                                            }
                                                        }
                                                        min="1"
                                                        max="65535"
                                                        required
                                                    />
                                                </div>
                                            </div>
                                            
                                            <div class="row mb-3">
                                                <div class="col-md-6">
                                                    <label for="smtpUsername" class="form-label">"SMTP Username"</label>
                                                    <input
                                                        id="smtpUsername"
                                                        type="text"
                                                        class="form-control"
                                                        prop:value=move || smtp_username()
                                                        on:input=move |ev| set_smtp_username.set(event_target_value(&ev))
                                                    />
                                                </div>
                                                <div class="col-md-6">
                                                    <label for="smtpPassword" class="form-label">"SMTP Password"</label>
                                                    <input
                                                        id="smtpPassword"
                                                        type="password"
                                                        class="form-control"
                                                        prop:value=move || smtp_password()
                                                        on:input=move |ev| set_smtp_password.set(event_target_value(&ev))
                                                        placeholder="Leave blank to keep current password"
                                                    />
                                                </div>
                                            </div>
                                            
                                            <div class="row mb-3">
                                                <div class="col-md-6">
                                                    <label for="smtpFromEmail" class="form-label">"From Email Address"</label>
                                                    <input
                                                        id="smtpFromEmail"
                                                        type="email"
                                                        class="form-control"
                                                        prop:value=move || smtp_from_email()
                                                        on:input=move |ev| set_smtp_from_email.set(event_target_value(&ev))
                                                        placeholder="noreply@example.com"
                                                        required
                                                    />
                                                </div>
                                                <div class="col-md-6">
                                                    <label for="smtpFromName" class="form-label">"From Name"</label>
                                                    <input
                                                        id="smtpFromName"
                                                        type="text"
                                                        class="form-control"
                                                        prop:value=move || smtp_from_name()
                                                        on:input=move |ev| set_smtp_from_name.set(event_target_value(&ev))
                                                        placeholder="LMS Forum"
                                                        required
                                                    />
                                                </div>
                                            </div>
                                            
                                            <div class="form-check mb-3">
                                                <input
                                                    class="form-check-input"
                                                    type="checkbox"
                                                    id="smtpUseTls"
                                                    prop:checked=move || smtp_use_tls()
                                                    on:change=move |ev| set_smtp_use_tls.set(event_target_checked(&ev))
                                                />
                                                <label class="form-check-label" for="smtpUseTls">
                                                    "Use TLS/SSL encryption"
                                                </label>
                                            </div>
                                            
                                            <button 
                                                type="button" 
                                                class="btn btn-outline-primary"
                                                on:click=send_test_email
                                                disabled=move || saving()
                                            >
                                                {move || if saving() {
                                                    view! { <span class="spinner-border spinner-border-sm me-2" role="status"></span> "Sending..." }
                                                } else {
                                                    view! { "Send Test Email" }
                                                }}
                                            </button>
                                        </fieldset>
                                    </div>
                                </div>
                                
                                <div class="card mb-4">
                                    <div class="card-header">
                                        <h5 class="mb-0">"Notification Templates"</h5>
                                    </div>
                                    <div class="card-body">
                                        <fieldset disabled=move || !enable_email()>
                                            <div class="mb-4">
                                                <div class="d-flex justify-content-between align-items-center mb-2">
                                                    <h6>"Welcome Email"</h6>
                                                    <div class="form-check form-switch">
                                                        <input
                                                            class="form-check-input"
                                                            type="checkbox"
                                                            id="emailWelcomeEnabled"
                                                            prop:checked=move || email_welcome_enabled()
                                                            on:change=move |ev| set_email_welcome_enabled.set(event_target_checked(&ev))
                                                        />
                                                        <label class="form-check-label" for="emailWelcomeEnabled">
                                                            "Enabled"
                                                        </label>
                                                    </div>
                                                </div>
                                                <input
                                                    type="text"
                                                    class="form-control mb-2"
                                                    placeholder="Email subject line"
                                                    prop:value=move || email_welcome_subject()
                                                    on:input=move |ev| set_email_welcome_subject.set(event_target_value(&ev))
                                                    disabled=move || !email_welcome_enabled()
                                                />
                                                <div class="form-text mb-2">
                                                    "Available variables: {username}, {forum_name}, {activation_link}"
                                                </div>
                                            </div>
                                            
                                            <hr class="my-4"/>
                                            
                                            <div class="mb-4">
                                                <div class="d-flex justify-content-between align-items-center mb-2">
                                                    <h6>"Post Reply Notification"</h6>
                                                    <div class="form-check form-switch">
                                                        <input
                                                            class="form-check-input"
                                                            type="checkbox"
                                                            id="emailPostReplyEnabled"
                                                            prop:checked=move || email_post_reply_enabled()
                                                            on:change=move |ev| set_email_post_reply_enabled.set(event_target_checked(&ev))
                                                        />
                                                        <label class="form-check-label" for="emailPostReplyEnabled">
                                                            "Enabled"
                                                        </label>
                                                    </div>
                                                </div>
                                                <input
                                                    type="text"
                                                    class="form-control mb-2"
                                                    placeholder="Email subject line"
                                                    prop:value=move || email_post_reply_subject()
                                                    on:input=move |ev| set_email_post_reply_subject.set(event_target_value(&ev))
                                                    disabled=move || !email_post_reply_enabled()
                                                />
                                                <div class="form-text mb-2">
                                                    "Available variables: {username}, {replier}, {topic_title}, {post_snippet}, {topic_link}"
                                                </div>
                                            </div>
                                            
                                            <hr class="my-4"/>
                                            
                                            <div class="mb-4">
                                                <div class="d-flex justify-content-between align-items-center mb-2">
                                                    <h6>"Topic Reply Notification"</h6>
                                                    <div class="form-check form-switch">
                                                        <input
                                                            class="form-check-input"
                                                            type="checkbox"
                                                            id="emailTopicReplyEnabled"
                                                            prop:checked=move || email_topic_reply_enabled()
                                                            on:change=move |ev| set_email_topic_reply_enabled.set(event_target_checked(&ev))
                                                        />
                                                        <label class="form-check-label" for="emailTopicReplyEnabled">
                                                            "Enabled"
                                                        </label>
                                                    </div>
                                                </div>
                                                <input
                                                    type="text"
                                                    class="form-control mb-2"
                                                    placeholder="Email subject line"
                                                    prop:value=move || email_topic_reply_subject()
                                                    on:input=move |ev| set_email_topic_reply_subject.set(event_target_value(&ev))
                                                    disabled=move || !email_topic_reply_enabled()
                                                />
                                                <div class="form-text mb-2">
                                                    "Available variables: {username}, {replier}, {topic_title}, {post_snippet}, {topic_link}"
                                                </div>
                                            </div>
                                            
                                            <hr class="my-4"/>
                                            
                                            <div class="mb-4">
                                                <div class="d-flex justify-content-between align-items-center mb-2">
                                                    <h6>"Mention Notification"</h6>
                                                    <div class="form-check form-switch">
                                                        <input
                                                            class="form-check-input"
                                                            type="checkbox"
                                                            id="emailMentionEnabled"
                                                            prop:checked=move || email_mention_enabled()
                                                            on:change=move |ev| set_email_mention_enabled.set(event_target_checked(&ev))
                                                        />
                                                        <label class="form-check-label" for="emailMentionEnabled">
                                                            "Enabled"
                                                        </label>
                                                    </div>
                                                </div>
                                                <input
                                                    type="text"
                                                    class="form-control mb-2"
                                                    placeholder="Email subject line"
                                                    prop:value=move || email_mention_subject()
                                                    on:input=move |ev| set_email_mention_subject.set(event_target_value(&ev))
                                                    disabled=move || !email_mention_enabled()
                                                />
                                                <div class="form-text mb-2">
                                                    "Available variables: {username}, {mentioner}, {topic_title}, {post_snippet}, {topic_link}"
                                                </div>
                                            </div>
                                        </fieldset>
                                    </div>
                                </div>
                                
                                <div class="d-flex justify-content-end">
                                    <button 
                                        type="submit" 
                                        class="btn btn-primary"
                                        disabled=move || saving() || !enable_email()
                                    >
                                        {move || if saving() {
                                            view! { <span class="spinner-border spinner-border-sm me-2" role="status"></span> "Saving..." }
                                        } else {
                                            view! { "Save Settings" }
                                        }}
                                    </button>
                                </div>
                            </form>
                        }
                    }}
                }
            }}
        </div>
    }
}