use leptos::*;
use crate::components::user::NotificationsPanel;
use crate::providers::auth_provider::get_current_user_id;
use web_sys::{Event, MouseEvent};
use wasm_bindgen::JsCast;

#[component]
pub fn NotificationCenter() -> impl IntoView {
    let (open, set_open) = create_signal(false);
    let (unread_count, set_unread_count) = create_signal(0);
    
    // Get the current user ID
    let user_id = get_current_user_id();
    
    // Load unread count periodically
    create_effect(move |_| {
        if let Some(id) = &user_id {
            // Initial load
            load_unread_count(id.clone(), set_unread_count);
            
            // Set up timer to refresh every 60 seconds
            let id_clone = id.clone();
            let interval_id = set_interval(
                move || load_unread_count(id_clone.clone(), set_unread_count),
                60000
            );
            
            on_cleanup(move || {
                clear_interval(interval_id);
            });
        }
    });
    
    // Toggle the notification panel
    let toggle_panel = move |_| {
        set_open.update(|val| *val = !*val);
    };
    
    // Set up click away listener
    let panel_ref = create_node_ref::<html::Div>();
    let button_ref = create_node_ref::<html::Button>();
    
    // Close panel when clicking outside
    create_effect(move |_| {
        if open.get() {
            let panel = panel_ref.get();
            let button = button_ref.get();
            
            let handler = EventListener::new(&window(), "click", move |event| {
                let event = event.dyn_ref::<MouseEvent>().unwrap();
                let target = event.target().unwrap();
                
                let in_panel = panel
                    .as_ref()
                    .map(|p| p.contains(&target.dyn_into().ok()))
                    .unwrap_or(false);
                    
                let in_button = button
                    .as_ref()
                    .map(|b| b.contains(&target.dyn_into().ok()))
                    .unwrap_or(false);
                
                if !in_panel && !in_button {
                    set_open.set(false);
                }
            });
            
            on_cleanup(move || {
                handler.remove();
            });
        }
    });

    view! {
        <div class="notification-center-container">
            <button 
                ref={button_ref}
                class=move || format!("notification-button{}", if unread_count.get() > 0 { " has-unread" } else { "" })
                on:click=toggle_panel
            >
                <span class="notification-icon">🔔</span>
                {move || if unread_count.get() > 0 {
                    view! {
                        <span class="notification-badge">{unread_count.get()}</span>
                    }
                } else {
                    view! { <span></span> }
                }}
            </button>
            
            {move || if open.get() {
                view! {
                    <div ref={panel_ref} class="notification-dropdown">
                        {move || if let Some(id) = &user_id {
                            view! {
                                <NotificationsPanel user_id={id.clone()} />
                            }
                        } else {
                            view! {
                                <div class="notification-login-prompt">
                                    <p>"Please log in to view notifications"</p>
                                    <a href="/login" class="login-button">"Log In"</a>
                                </div>
                            }
                        }}
                    </div>
                }
            } else {
                view! { <span></span> }
            }}
        </div>
    }
}

// Helper function to load unread notification count
fn load_unread_count(user_id: String, set_count: WriteSignal<i32>) {
    spawn_local(async move {
        if let Ok(count) = invoke::<_, i64>("get_unread_notification_count", &user_id).await {
            set_count.set(count as i32);
        }
    });
}

// Helper function to invoke Tauri commands
async fn invoke<T, R>(cmd: &str, args: &T) -> Result<R, String>
where
    T: serde::Serialize + ?Sized,
    R: for<'de> serde::de::DeserializeOwned,
{
    tauri_sys::tauri::invoke(cmd, args)
        .await
        .map_err(|e| e.to_string())
}

// Helper function for JS interop - set interval
fn set_interval<F>(callback: F, timeout: i32) -> i32
where
    F: Fn() + 'static,
{
    use wasm_bindgen::{closure::Closure, JsCast};
    
    let window = web_sys::window().unwrap();
    
    let callback = Closure::wrap(Box::new(callback) as Box<dyn Fn()>);
    
    let id = window
        .set_interval_with_callback_and_timeout_and_arguments_0(
            callback.as_ref().unchecked_ref(),
            timeout,
        )
        .unwrap();
    
    callback.forget();
    
    id
}

// Helper function for JS interop - clear interval
fn clear_interval(id: i32) {
    if let Some(window) = web_sys::window() {
        window.clear_interval_with_handle(id);
    }
}

// Helper for DOM events
struct EventListener {
    target: web_sys::EventTarget,
    event_type: String,
    callback: Closure<dyn FnMut(Event)>,
}

impl EventListener {
    fn new<T, F>(target: &T, event_type: &str, callback: F) -> Self
    where
        T: AsRef<web_sys::EventTarget>,
        F: FnMut(Event) + 'static,
    {
        let callback = Closure::wrap(Box::new(callback) as Box<dyn FnMut(Event)>);
        
        target
            .as_ref()
            .add_event_listener_with_callback(
                event_type,
                callback.as_ref().unchecked_ref(),
            )
            .unwrap();
        
        Self {
            target: target.as_ref().clone(),
            event_type: event_type.to_string(),
            callback,
        }
    }
    
    fn remove(self) {
        self.target
            .remove_event_listener_with_callback(
                &self.event_type,
                self.callback.as_ref().unchecked_ref(),
            )
            .unwrap();
    }
}

// Helper to get window
fn window() -> web_sys::Window {
    web_sys::window().expect("No global `window` exists")
}