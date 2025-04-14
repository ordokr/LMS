use leptos::*;
use leptos_router::*;
use crate::services::integration_service::IntegrationService;
use crate::components::shared::ErrorAlert;
use crate::utils::style_manager::use_stylesheet;

#[component]
pub fn IntegrationSettingsPage() -> impl IntoView {
    // Load stylesheet
    use_stylesheet("integration.css");
    
    // State
    let (loading, set_loading) = create_signal(false);
    let (canvas_api_url, set_canvas_api_url) = create_signal(String::new());
    let (canvas_api_token, set_canvas_api_token) = create_signal(String::new());
    let (discourse_api_url, set_discourse_api_url) = create_signal(String::new());
    let (discourse_api_key, set_discourse_api_key) = create_signal(String::new());
    let (discourse_username, set_discourse_username) = create_signal(String::new());
    let (sync_interval, set_sync_interval) = create_signal(60);
    let (error, set_error) = create_signal(None::<String>);
    let (success_message, set_success_message) = create_signal(None::<String>);
    
    // Load settings
    let load_settings = move || {
        set_loading.set(true);
        set_error.set(None);
        
        spawn_local(async move {
            match IntegrationService::get_integration_settings().await {
                Ok(settings) => {
                    set_canvas_api_url.set(settings.canvas_api_url.unwrap_or_default());
                    set_canvas_api_token.set(settings.canvas_api_token.unwrap_or_default());
                    set_discourse_api_url.set(settings.discourse_api_url.unwrap_or_default());
                    set_discourse_api_key.set(settings.discourse_api_key.unwrap_or_default());
                    set_discourse_username.set(settings.discourse_username.unwrap_or_default());
                    set_sync_interval.set(settings.sync_interval.unwrap_or(60));
                },
                Err(err) => {
                    set_error.set(Some(format!("Failed to load settings: {}", err)));
                }
            }
            
            set_loading.set(false);
        });
    };
    
    // Save Canvas settings
    let save_canvas_settings = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        
        set_loading.set(true);
        set_error.set(None);
        set_success_message.set(None);
        
        let url = canvas_api_url.get();
        let token = canvas_api_token.get();
        
        spawn_local(async move {
            match IntegrationService::save_canvas_settings(&url, &token).await {
                Ok(_) => {
                    set_success_message.set(Some("Canvas settings saved successfully.".to_string()));
                },
                Err(err) => {
                    set_error.set(Some(format!("Failed to save Canvas settings: {}", err)));
                }
            }
            
            set_loading.set(false);
        });
    };
    
    // Save Discourse settings
    let save_discourse_settings = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        
        set_loading.set(true);
        set_error.set(None);
        set_success_message.set(None);
        
        let url = discourse_api_url.get();
        let key = discourse_api_key.get();
        let username = discourse_username.get();
        
        spawn_local(async move {
            match IntegrationService::save_discourse_settings(&url, &key, &username).await {
                Ok(_) => {
                    set_success_message.set(Some("Discourse settings saved successfully.".to_string()));
                },
                Err(err) => {
                    set_error.set(Some(format!("Failed to save Discourse settings: {}", err)));
                }
            }
            
            set_loading.set(false);
        });
    };
    
    // Save sync settings
    let save_sync_settings = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        
        set_loading.set(true);
        set_error.set(None);
        set_success_message.set(None);
        
        let interval = sync_interval.get();
        
        spawn_local(async move {
            match IntegrationService::save_sync_settings(interval).await {
                Ok(_) => {
                    set_success_message.set(Some("Sync settings saved successfully.".to_string()));
                },
                Err(err) => {
                    set_error.set(Some(format!("Failed to save sync settings: {}", err)));
                }
            }
            
            set_loading.set(false);
        });
    };
    
    // Test Canvas connection
    let test_canvas_connection = move |_| {
        set_loading.set(true);
        set_error.set(None);
        set_success_message.set(None);
        
        spawn_local(async move {
            match IntegrationService::test_canvas_connectivity().await {
                Ok(connected) => {
                    if connected {
                        set_success_message.set(Some("Canvas connection successful.".to_string()));
                    } else {
                        set_error.set(Some("Canvas connection failed. Please check your settings.".to_string()));
                    }
                },
                Err(err) => {
                    set_error.set(Some(format!("Failed to test Canvas connection: {}", err)));
                }
            }
            
            set_loading.set(false);
        });
    };
    
    // Test Discourse connection
    let test_discourse_connection = move |_| {
        set_loading.set(true);
        set_error.set(None);
        set_success_message.set(None);
        
        spawn_local(async move {
            match IntegrationService::test_discourse_connectivity().await {
                Ok(connected) => {
                    if connected {
                        set_success_message.set(Some("Discourse connection successful.".to_string()));
                    } else {
                        set_error.set(Some("Discourse connection failed. Please check your settings.".to_string()));
                    }
                },
                Err(err) => {
                    set_error.set(Some(format!("Failed to test Discourse connection: {}", err)));
                }
            }
            
            set_loading.set(false);
        });
    };
    
    // Load settings on component mount
    create_effect(move |_| {
        load_settings();
    });
    
    view! {
        <div class="page-container">
            <div class="page-header">
                <h1 class="page-title">
                    <i class="icon-settings"></i>
                    "Integration Settings"
                </h1>
                
                <div class="page-actions">
                    <A href="/integrations" class="btn btn-secondary">
                        "Back to Dashboard"
                    </A>
                </div>
            </div>
            
            {move || if let Some(err) = error.get() {
                view! { <ErrorAlert message=err /> }
            } else {
                view! { <></> }
            }}
            
            {move || if let Some(msg) = success_message.get() {
                view! {
                    <div class="alert alert-success">
                        <i class="icon-success"></i>
                        <span>{msg}</span>
                    </div>
                }
            } else {
                view! { <></> }
            }}
            
            <div class="settings-grid">
                <div class="settings-card">
                    <div class="card-header">
                        <h2 class="card-title">
                            <i class="icon-canvas"></i>
                            "Canvas Settings"
                        </h2>
                    </div>
                    
                    <div class="card-content">
                        <form on:submit=save_canvas_settings>
                            <div class="form-group">
                                <label for="canvas-api-url">"Canvas API URL"</label>
                                <input 
                                    type="text"
                                    id="canvas-api-url"
                                    class="form-control"
                                    placeholder="https://canvas.instructure.com/api/v1"
                                    prop:value=canvas_api_url
                                    on:input=move |ev| set_canvas_api_url.set(event_target_value(&ev))
                                    disabled=loading
                                />
                                <div class="form-text">"The base URL for your Canvas API."</div>
                            </div>
                            
                            <div class="form-group">
                                <label for="canvas-api-token">"Canvas API Token"</label>
                                <input 
                                    type="password"
                                    id="canvas-api-token"
                                    class="form-control"
                                    placeholder="Enter your Canvas API token"
                                    prop:value=canvas_api_token
                                    on:input=move |ev| set_canvas_api_token.set(event_target_value(&ev))
                                    disabled=loading
                                />
                                <div class="form-text">"Your Canvas API access token. Keep this secure."</div>
                            </div>
                            
                            <div class="form-actions">
                                <button 
                                    type="button"
                                    class="btn btn-secondary"
                                    on:click=test_canvas_connection
                                    disabled=loading
                                >
                                    "Test Connection"
                                </button>
                                
                                <button 
                                    type="submit"
                                    class="btn btn-primary"
                                    disabled=loading
                                >
                                    "Save Canvas Settings"
                                </button>
                            </div>
                        </form>
                    </div>
                </div>
                
                <div class="settings-card">
                    <div class="card-header">
                        <h2 class="card-title">
                            <i class="icon-discourse"></i>
                            "Discourse Settings"
                        </h2>
                    </div>
                    
                    <div class="card-content">
                        <form on:submit=save_discourse_settings>
                            <div class="form-group">
                                <label for="discourse-api-url">"Discourse API URL"</label>
                                <input 
                                    type="text"
                                    id="discourse-api-url"
                                    class="form-control"
                                    placeholder="https://discourse.example.com"
                                    prop:value=discourse_api_url
                                    on:input=move |ev| set_discourse_api_url.set(event_target_value(&ev))
                                    disabled=loading
                                />
                                <div class="form-text">"The base URL for your Discourse instance."</div>
                            </div>
                            
                            <div class="form-group">
                                <label for="discourse-api-key">"Discourse API Key"</label>
                                <input 
                                    type="password"
                                    id="discourse-api-key"
                                    class="form-control"
                                    placeholder="Enter your Discourse API key"
                                    prop:value=discourse_api_key
                                    on:input=move |ev| set_discourse_api_key.set(event_target_value(&ev))
                                    disabled=loading
                                />
                                <div class="form-text">"Your Discourse API key. Keep this secure."</div>
                            </div>
                            
                            <div class="form-group">
                                <label for="discourse-username">"Discourse Username"</label>
                                <input 
                                    type="text"
                                    id="discourse-username"
                                    class="form-control"
                                    placeholder="Enter your Discourse username"
                                    prop:value=discourse_username
                                    on:input=move |ev| set_discourse_username.set(event_target_value(&ev))
                                    disabled=loading
                                />
                                <div class="form-text">"The username associated with your Discourse API key."</div>
                            </div>
                            
                            <div class="form-actions">
                                <button 
                                    type="button"
                                    class="btn btn-secondary"
                                    on:click=test_discourse_connection
                                    disabled=loading
                                >
                                    "Test Connection"
                                </button>
                                
                                <button 
                                    type="submit"
                                    class="btn btn-primary"
                                    disabled=loading
                                >
                                    "Save Discourse Settings"
                                </button>
                            </div>
                        </form>
                    </div>
                </div>
                
                <div class="settings-card full-width">
                    <div class="card-header">
                        <h2 class="card-title">
                            <i class="icon-sync"></i>
                            "Sync Settings"
                        </h2>
                    </div>
                    
                    <div class="card-content">
                        <form on:submit=save_sync_settings>
                            <div class="form-group">
                                <label for="sync-interval">"Sync Interval (seconds)"</label>
                                <input 
                                    type="number"
                                    id="sync-interval"
                                    class="form-control"
                                    min="30"
                                    max="3600"
                                    prop:value=sync_interval
                                    on:input=move |ev| {
                                        if let Ok(val) = event_target_value(&ev).parse::<i32>() {
                                            set_sync_interval.set(val);
                                        }
                                    }
                                    disabled=loading
                                />
                                <div class="form-text">"How often the system should check for changes to sync (in seconds)."</div>
                            </div>
                            
                            <div class="form-actions">
                                <button 
                                    type="submit"
                                    class="btn btn-primary"
                                    disabled=loading
                                >
                                    "Save Sync Settings"
                                </button>
                            </div>
                        </form>
                    </div>
                </div>
            </div>
        </div>
    }
}

// Helper to get value from an event
fn event_target_value(ev: &web_sys::Event) -> String {
    let target: web_sys::EventTarget = ev.target().unwrap();
    let input: web_sys::HtmlInputElement = target.dyn_into().unwrap();
    input.value()
}
