use wasm_bindgen::prelude::*;
use web_sys::Navigator;

// Check if the browser is online
pub fn is_online() -> bool {
    if let Some(window) = web_sys::window() {
        if let Some(navigator) = window.navigator().dyn_into::<Navigator>().ok() {
            return navigator.on_line();
        }
    }
    // Default to assuming online if we can't check
    true
}

// Register online/offline event handlers
pub fn register_online_status_listener<F>(mut callback: F) 
where 
    F: FnMut(bool) + 'static,
{
    if let Some(window) = web_sys::window() {
        // Closure for online event
        let online_callback = Closure::wrap(Box::new(move || {
            callback(true);
        }) as Box<dyn FnMut()>);
        
        // Closure for offline event
        let offline_callback = Closure::wrap(Box::new(move || {
            callback(false);
        }) as Box<dyn FnMut()>);
        
        // Register event listeners
        let _ = window.add_event_listener_with_callback("online", online_callback.as_ref().unchecked_ref());
        let _ = window.add_event_listener_with_callback("offline", offline_callback.as_ref().unchecked_ref());
        
        // Leaking the closures so they live for the lifetime of the page
        online_callback.forget();
        offline_callback.forget();
    }
}