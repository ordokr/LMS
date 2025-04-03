use leptos::*;
use serde::{Serialize, Deserialize};
use wasm_bindgen::prelude::*;
use crate::models::notification::Notification;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum WebSocketMessage {
    Notification(Notification),
    OnlineCount(i32),
    TopicUpdated { topic_id: i64 },
    NewPost { topic_id: i64, post_id: i64 },
    Error(String),
}

#[derive(Clone)]
pub struct WebSocketService {
    ws: Option<web_sys::WebSocket>,
    connected: RwSignal<bool>,
    error: RwSignal<Option<String>>,
}

impl WebSocketService {
    pub fn new() -> Self {
        Self {
            ws: None,
            connected: create_rw_signal(false),
            error: create_rw_signal(None),
        }
    }
    
    pub fn is_connected(&self) -> bool {
        self.connected.get()
    }
    
    pub fn error(&self) -> Option<String> {
        self.error.get()
    }
    
    pub fn connect(&mut self) {
        // Close existing connection if any
        self.close();
        
        // Get auth token (would come from your auth system)
        let auth_token = get_auth_token();
        
        // Create WebSocket connection
        let ws_url = format!("wss://your-api-server.com/ws?token={}", auth_token);
        
        match web_sys::WebSocket::new(&ws_url) {
            Ok(ws) => {
                // Set up event handlers
                let connected = self.connected;
                let error_signal = self.error;
                
                // onopen handler
                let onopen_callback = Closure::wrap(Box::new(move |_: web_sys::Event| {
                    connected.set(true);
                    error_signal.set(None);
                    log::info!("WebSocket connection established");
                }) as Box<dyn FnMut(web_sys::Event)>);
                ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
                onopen_callback.forget();
                
                // onclose handler
                let connected_close = self.connected;
                let onclose_callback = Closure::wrap(Box::new(move |e: web_sys::CloseEvent| {
                    connected_close.set(false);
                    log::info!("WebSocket connection closed: {}", e.reason());
                }) as Box<dyn FnMut(web_sys::CloseEvent)>);
                ws.set_onclose(Some(onclose_callback.as_ref().unchecked_ref()));
                onclose_callback.forget();
                
                // onerror handler
                let error_signal_err = self.error;
                let connected_err = self.connected;
                let onerror_callback = Closure::wrap(Box::new(move |_: web_sys::Event| {
                    error_signal_err.set(Some("WebSocket connection error".to_string()));
                    connected_err.set(false);
                    log::error!("WebSocket connection error");
                }) as Box<dyn FnMut(web_sys::Event)>);
                ws.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
                onerror_callback.forget();
                
                // onmessage handler
                let onmessage_callback = Closure::wrap(Box::new(move |e: web_sys::MessageEvent| {
                    if let Ok(text) = e.data().dyn_into::<js_sys::JsString>() {
                        let text_string = text.as_string().unwrap();
                        
                        // Parse the message
                        match serde_json::from_str::<WebSocketMessage>(&text_string) {
                            Ok(message) => {
                                Self::handle_message(message);
                            },
                            Err(e) => {
                                log::error!("Failed to parse WebSocket message: {}", e);
                            }
                        }
                    }
                }) as Box<dyn FnMut(web_sys::MessageEvent)>);
                ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
                onmessage_callback.forget();
                
                // Store WebSocket instance
                self.ws = Some(ws);
            },
            Err(e) => {
                self.error.set(Some(format!("Failed to create WebSocket connection: {:?}", e)));
                log::error!("Failed to create WebSocket connection");
            }
        }
    }
    
    pub fn close(&mut self) {
        if let Some(ws) = &self.ws {
            let _ = ws.close();
        }
        self.ws = None;
        self.connected.set(false);
    }
    
    // Function to handle incoming messages
    fn handle_message(message: WebSocketMessage) {
        match message {
            WebSocketMessage::Notification(notification) => {
                // Emit an event that your notification indicator can listen to
                let event = CustomEvent::new("new-notification").unwrap();
                let window = web_sys::window().unwrap();
                let _ = event.init_custom_event_with_can_bubble_and_cancelable("new-notification", true, true);
                
                // Set the notification data as a detail property (requires more complex setup)
                // For now, just broadcast the event
                let _ = window.dispatch_event(&event);
                
                // Show browser notification if supported
                Self::show_browser_notification(&notification);
            },
            WebSocketMessage::OnlineCount(count) => {
                log::info!("Online users: {}", count);
                // Update UI if needed
            },
            WebSocketMessage::TopicUpdated { topic_id } => {
                log::info!("Topic updated: {}", topic_id);
                // Refresh topic if it's currently being viewed
            },
            WebSocketMessage::NewPost { topic_id, post_id } => {
                log::info!("New post in topic {}: {}", topic_id, post_id);
                // Refresh posts if the topic is being viewed
            },
            WebSocketMessage::Error(error) => {
                log::error!("WebSocket error: {}", error);
            }
        }
    }
    
    // Show browser notification
    fn show_browser_notification(notification: &Notification) {
        // Check if browser supports notifications
        if let Some(window) = web_sys::window() {
            if let Some(notification_api) = window.notification() {
                // Check if we have permission
                let permission = notification_api.permission();
                
                if permission == "granted" {
                    let options = web_sys::NotificationOptions::new();
                    options.body(&notification.data.message);
                    options.icon("/favicon.ico");
                    
                    let _ = web_sys::Notification::new_with_options(
                        &notification.data.title,
                        &options
                    );
                } else if permission != "denied" {
                    // Request permission
                    let _ = notification_api.request_permission();
                }
            }
        }
    }
}

// Helper function to get auth token
fn get_auth_token() -> String {
    // In a real app, get this from your auth service or local storage
    "example_token".to_string()
}