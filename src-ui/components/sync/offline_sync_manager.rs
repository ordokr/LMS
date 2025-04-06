use leptos::*;
use serde::{Deserialize, Serialize};
use crate::hooks::use_auth::use_auth;

#[derive(Clone, Debug, Deserialize)]
struct OfflineAction {
    id: String,
    action_type: String,
    entity_type: String,
    entity_id: String,
    description: String,
    created_at: String,
}

#[derive(Clone, Debug, Deserialize)]
struct SyncResult {
    success: bool,
    message: String,
    action_id: String,
}

#[component]
pub fn OfflineSyncManager() -> impl IntoView {
    let (actions, set_actions) = create_signal::<Vec<OfflineAction>>(vec![]);
    let (loading, set_loading) = create_signal(false);
    let (error, set_error) = create_signal::<Option<String>>(None);
    let (success, set_success) = create_signal::<Option<String>>(None);
    let (is_offline, set_is_offline) = create_signal(false);
    
    // Check if offline
    create_effect(move |_| {
        set_is_offline(!window_has_network());
    });
    
    // Load offline actions when component mounts
    create_effect(move |_| {
        spawn_local(async move {
            set_loading(true);
            
            match fetch_offline_actions().await {
                Ok(fetched_actions) => {
                    set_actions(fetched_actions);
                    set_error(None);
                },
                Err(err) => {
                    set_error(Some(format!("Error loading offline actions: {}", err)));
                }
            }
            
            set_loading(false);
        });
    });
    
    // Listen for online status changes
    let online_listener = window_online_listener(move |is_online| {
        set_is_offline(!is_online);
        
        if is_online {
            // When coming back online, refresh the list
            spawn_local(async move {
                set_loading(true);
                
                match fetch_offline_actions().await {
                    Ok(fetched_actions) => {
                        set_actions(fetched_actions);
                        set_error(None);
                    },
                    Err(err) => {
                        set_error(Some(format!("Error loading offline actions: {}", err)));
                    }
                }
                
                set_loading(false);
            });
        }
    });
    
    // Clean up the listener when component is destroyed
    on_cleanup(move || {
        online_listener.remove();
    });
    
    let handle_sync = move || {
        set_loading(true);
        set_error(None);
        set_success(None);
        
        spawn_local(async move {
            match sync_actions().await {
                Ok(result_count) => {
                    // Refresh actions list
                    match fetch_offline_actions().await {
                        Ok(fetched_actions) => {
                            set_actions(fetched_actions);
                            
                            if result_count > 0 {
                                set_success(Some(format!("Successfully synced {} actions", result_count)));
                            } else {
                                set_success(Some("All actions already synced".to_string()));
                            }
                        },
                        Err(err) => {
                            set_error(Some(format!("Error refreshing actions: {}", err)));
                        }
                    }
                },
                Err(err) => {
                    set_error(Some(format!("Sync failed: {}", err)));
                }
            }
            
            set_loading(false);
        });
    };

    view! {
        <div class="bg-white shadow rounded-lg p-6">
            <div class="flex items-center justify-between mb-4">
                <div>
                    <h2 class="text-lg font-medium text-gray-900">Offline Changes</h2>
                    <p class="text-sm text-gray-500">
                        "Manage changes made while offline"
                    </p>
                </div>
                
                {move || {
                    if is_offline.get() {
                        view! {
                            <div class="inline-flex items-center px-3 py-1 text-sm font-medium rounded-full bg-red-100 text-red-800">
                                <span class="mr-1 h-2 w-2 rounded-full bg-red-500"></span>
                                "Offline"
                            </div>
                        }.into_view()
                    } else {
                        view! {
                            <div class="inline-flex items-center px-3 py-1 text-sm font-medium rounded-full bg-green-100 text-green-800">
                                <span class="mr-1 h-2 w-2 rounded-full bg-green-500"></span>
                                "Online"
                            </div>
                        }.into_view()
                    }
                }}
            </div>
            
            {move || {
                if loading.get() {
                    view! {
                        <div class="flex justify-center py-4">
                            <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-500"></div>
                        </div>
                    }.into_view()
                } else {
                    view! { <></> }.into_view()
                }
            }}
            
            {move || {
                error.get().map(|err| {
                    view! {
                        <div class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-4">
                            {err}
                        </div>
                    }
                })
            }}
            
            {move || {
                success.get().map(|msg| {
                    view! {
                        <div class="bg-green-100 border border-green-400 text-green-700 px-4 py-3 rounded mb-4">
                            {msg}
                        </div>
                    }
                })
            }}
            
            {move || {
                if actions.get().is_empty() {
                    view! {
                        <div class="bg-gray-50 p-4 text-center rounded">
                            <p class="text-gray-700">
                                "No pending offline changes"
                            </p>
                        </div>
                    }.into_view()
                } else {
                    view! {
                        <div>
                            <div class="mb-4">
                                <button 
                                    class="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md shadow-sm text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
                                    disabled=move || loading.get() || is_offline.get()
                                    on:click=move |_| handle_sync()
                                >
                                    "Sync All Changes"
                                </button>
                            </div>
                            
                            <div class="border rounded-md overflow-hidden">
                                <table class="min-w-full divide-y divide-gray-200">
                                    <thead class="bg-gray-50">
                                        <tr>
                                            <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                                                "Type"
                                            </th>
                                            <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                                                "Description"
                                            </th>
                                            <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                                                "Created"
                                            </th>
                                        </tr>
                                    </thead>
                                    <tbody class="bg-white divide-y divide-gray-200">
                                        {actions.get().into_iter().map(|action| {
                                            view! {
                                                <tr>
                                                    <td class="px-6 py-4 whitespace-nowrap text-sm font-medium">
                                                        <span class=format!("inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-{action_color(&action.action_type)}-100 text-{action_color(&action.action_type)}-800")>
                                                            {action.action_type}
                                                        </span>
                                                        <span class="text-gray-500 ml-2">
                                                            {action.entity_type}
                                                        </span>
                                                    </td>
                                                    <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                                                        {action.description}
                                                    </td>
                                                    <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                                                        {format_date(&action.created_at)}
                                                    </td>
                                                </tr>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </tbody>
                                </table>
                            </div>
                        </div>
                    }.into_view()
                }
            }}
        </div>
    }
}

// Helper function to get color based on action type
fn action_color(action_type: &str) -> &'static str {
    match action_type {
        "Create" => "green",
        "Update" => "blue",
        "Delete" => "red",
        _ => "gray",
    }
}

// Helper function to format date
fn format_date(date_str: &str) -> String {
    // Simple formatting - in a real app you might use a date library
    date_str.split('T').next().unwrap_or(date_str).to_string()
}

// Helper function to check if window has network
fn window_has_network() -> bool {
    let window = web_sys::window();
    if let Some(window) = window {
        if let Some(navigator) = window.navigator().dyn_ref::<web_sys::Navigator>() {
            return navigator.online();
        }
    }
    false
}

// Helper function to listen for online status changes
fn window_online_listener<F>(callback: F) -> OnlineListener
where
    F: Fn(bool) + 'static,
{
    let window = web_sys::window().expect("Window not available");
    
    let online_callback = Closure::wrap(Box::new(move || {
        callback(true);
    }) as Box<dyn FnMut()>);
    
    let offline_callback = Closure::wrap(Box::new(move || {
        callback(false);
    }) as Box<dyn FnMut()>);
    
    window.add_event_listener_with_callback("online", online_callback.as_ref().unchecked_ref())
        .expect("Failed to add online listener");
    
    window.add_event_listener_with_callback("offline", offline_callback.as_ref().unchecked_ref())
        .expect("Failed to add offline listener");
    
    OnlineListener {
        window,
        online_callback,
        offline_callback,
    }
}

struct OnlineListener {
    window: web_sys::Window,
    online_callback: Closure<dyn FnMut()>,
    offline_callback: Closure<dyn FnMut()>,
}

impl OnlineListener {
    fn remove(self) {
        let _ = self.window.remove_event_listener_with_callback(
            "online",
            self.online_callback.as_ref().unchecked_ref(),
        );
        let _ = self.window.remove_event_listener_with_callback(
            "offline",
            self.offline_callback.as_ref().unchecked_ref(),
        );
    }
}

// Helper function to fetch offline actions
async fn fetch_offline_actions() -> Result<Vec<OfflineAction>, String> {
    let auth = use_auth();
    
    // Ensure we have a token
    let token = match auth.token.get() {
        Some(token) => token,
        None => return Err("Authentication required".into()),
    };
    
    let mut opts = web_sys::RequestInit::new();
    opts.method("GET");
    
    let request = web_sys::Request::new_with_str_and_init(
        "/api/sync/offline-actions", 
        &opts
    ).map_err(|_| "Failed to create request".to_string())?;
    
    request.headers().set("Authorization", &format!("Bearer {}", token))
        .map_err(|_| "Failed to set headers".to_string())?;
    
    let window = web_sys::window().ok_or("No window found".to_string())?;
    let resp_value = wasm_bindgen_futures::JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(|_| "Failed to fetch".to_string())?;
    
    let resp: web_sys::Response = resp_value.dyn_into()
        .map_err(|_| "Response is not a Response".to_string())?;
    
    if !resp.ok() {
        return Err(format!("HTTP error: {}", resp.status()));
    }
    
    let json = wasm_bindgen_futures::JsFuture::from(
        resp.json().map_err(|_| "Failed to parse response as JSON".to_string())?
    )
    .await
    .map_err(|_| "Failed to parse JSON".to_string())?;
    
    let actions: Vec<OfflineAction> = serde_wasm_bindgen::from_value(json)
        .map_err(|_| "Failed to deserialize response".to_string())?;
    
    Ok(actions)
}

// Helper function to sync offline actions
async fn sync_actions() -> Result<usize, String> {
    let auth = use_auth();
    
    // Ensure we have a token
    let token = match auth.token.get() {
        Some(token) => token,
        None => return Err("Authentication required".into()),
    };
    
    let mut opts = web_sys::RequestInit::new();
    opts.method("POST");
    
    let request = web_sys::Request::new_with_str_and_init(
        "/api/sync/sync-all", 
        &opts
    ).map_err(|_| "Failed to create request".to_string())?;
    
    request.headers().set("Authorization", &format!("Bearer {}", token))
        .map_err(|_| "Failed to set headers".to_string())?;
    
    let window = web_sys::window().ok_or("No window found".to_string())?;
    let resp_value = wasm_bindgen_futures::JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(|_| "Failed to fetch".to_string())?;
    
    let resp: web_sys::Response = resp_value.dyn_into()
        .map_err(|_| "Response is not a Response".to_string())?;
    
    if !resp.ok() {
        return Err(format!("HTTP error: {}", resp.status()));
    }
    
    let json = wasm_bindgen_futures::JsFuture::from(
        resp.json().map_err(|_| "Failed to parse response as JSON".to_string())?
    )
    .await
    .map_err(|_| "Failed to parse JSON".to_string())?;
    
    #[derive(Deserialize)]
    struct SyncResponse {
        synced_count: usize,
    }
    
    let response: SyncResponse = serde_wasm_bindgen::from_value(json)
        .map_err(|_| "Failed to deserialize response".to_string())?;
    
    Ok(response.synced_count)
}