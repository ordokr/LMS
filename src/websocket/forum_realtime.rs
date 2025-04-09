use leptos::*;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{WebSocket, MessageEvent, CloseEvent, ErrorEvent};
use serde::{Serialize, Deserialize};
use std::sync::atomic::{AtomicU32, Ordering};
use std::collections::HashMap;
use std::time::Duration;
use gloo_timers::future::TimeoutFuture;

// Websocket message types
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ForumUpdate {
    #[serde(rename = "new_topic")]
    NewTopic { 
        category_id: i64, 
        topic_id: i64,
        title: String,
    },
    
    #[serde(rename = "new_post")]
    NewPost { 
        topic_id: i64, 
        post_id: i64,
        author_id: i64,
        author_name: String,
    },
    
    #[serde(rename = "topic_edited")]
    TopicEdited { 
        topic_id: i64, 
        new_title: Option<String>,
        new_category_id: Option<i64>,
        is_pinned: Option<bool>,
    },
    
    #[serde(rename = "typing")]
    Typing {
        topic_id: i64,
        user_name: String,
    },
    
    #[serde(rename = "ping")]
    Ping { 
        id: u32,
    },
    
    #[serde(rename = "pong")]
    Pong { 
        id: u32,
    },
}

// WebSocket connection states
#[derive(Debug, Clone, PartialEq)]
pub enum ConnectionState {
    Connecting,
    Connected,
    Reconnecting { attempt: u32 },
    Disconnected,
}

// Persistent WebSocket connection with automatic reconnection
#[derive(Clone)]
pub struct ForumRealtimeClient {
    socket: StoredValue<Option<WebSocket>>,
    state: RwSignal<ConnectionState>,
    updates: RwSignal<Vec<ForumUpdate>>,
    subscribed_topics: StoredValue<HashMap<i64, ()>>,
    subscribed_categories: StoredValue<HashMap<i64, ()>>,
    ping_id: StoredValue<AtomicU32>,
    latest_ping_time: StoredValue<Option<f64>>,
    ping_interval_handle: StoredValue<Option<i32>>,
    auth_token: RwSignal<Option<String>>,
    base_url: String,
    max_reconnect_attempts: u32,
    reconnect_timeout_ms: u32,
}

impl ForumRealtimeClient {
    pub fn new() -> Self {
        let base_url = if window().location().protocol().unwrap_or_default() == "https:" {
            "wss://".to_string()
        } else {
            "ws://".to_string()
        };
        let base_url = format!("{}{}/ws/forum", base_url, window().location().host().unwrap_or_default());
        
        Self {
            socket: store_value(None),
            state: create_rw_signal(ConnectionState::Disconnected),
            updates: create_rw_signal(Vec::new()),
            subscribed_topics: store_value(HashMap::new()),
            subscribed_categories: store_value(HashMap::new()),
            ping_id: store_value(AtomicU32::new(1)),
            latest_ping_time: store_value(None),
            ping_interval_handle: store_value(None),
            auth_token: create_rw_signal(None),
            base_url,
            max_reconnect_attempts: 5,
            reconnect_timeout_ms: 2000, // Start with 2 seconds, will increase
        }
    }
    
    // Set authentication token
    pub fn set_auth_token(&self, token: Option<String>) {
        self.auth_token.set(token);
        
        // Reconnect if we're already connected
        if matches!(self.state.get(), ConnectionState::Connected) {
            // Disconnect and reconnect to apply new token
            let _ = self.disconnect();
            let _ = self.connect();
        }
    }
    
    // Connect to WebSocket server
    pub fn connect(&self) -> Result<(), JsValue> {
        // Don't reconnect if already connected or connecting
        if matches!(self.state.get(), ConnectionState::Connected | ConnectionState::Connecting) {
            return Ok(());
        }
        
        self.state.set(ConnectionState::Connecting);
        
        // Build connection URL
        let url = if let Some(token) = self.auth_token.get() {
            format!("{}?token={}", self.base_url, token)
        } else {
            self.base_url.clone()
        };
        
        // Create WebSocket
        let ws = WebSocket::new(&url)?;
        ws.set_binary_type(web_sys::BinaryType::Arraybuffer);
        
        // Set up listeners
        let client = self.clone();
        let onopen = Closure::wrap(Box::new(move |_| {
            client.state.set(ConnectionState::Connected);
            client.start_ping_interval();
            client.resubscribe();
        }) as Box<dyn FnMut(JsValue)>);
        
        let client = self.clone();
        let onclose = Closure::wrap(Box::new(move |e: CloseEvent| {
            // Clean up
            client.clear_ping_interval();
            
            // Try to reconnect if connection was established
            if matches!(client.state.get(), ConnectionState::Connected) {
                client.attempt_reconnect(1);
            } else {
                client.state.set(ConnectionState::Disconnected);
            }
        }) as Box<dyn FnMut(CloseEvent)>);
        
        let client = self.clone();
        let onmessage = Closure::wrap(Box::new(move |e: MessageEvent| {
            if let Ok(txt) = e.data().dyn_into::<js_sys::JsString>() {
                let txt_str = String::from(txt);
                if let Ok(update) = serde_json::from_str::<ForumUpdate>(&txt_str) {
                    client.handle_message(update);
                }
            }
        }) as Box<dyn FnMut(MessageEvent)>);
        
        let client = self.clone();
        let onerror = Closure::wrap(Box::new(move |e: ErrorEvent| {
            log::error!("WebSocket error: {:?}", e);
        }) as Box<dyn FnMut(ErrorEvent)>);
        
        ws.set_onopen(Some(onopen.as_ref().unchecked_ref()));
        ws.set_onclose(Some(onclose.as_ref().unchecked_ref()));
        ws.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));
        ws.set_onerror(Some(onerror.as_ref().unchecked_ref()));
        
        // Store WebSocket and closures
        *self.socket.get_value() = Some(ws);
        
        // We need these closures to live as long as the WebSocket
        onopen.forget();
        onclose.forget();
        onmessage.forget();
        onerror.forget();
        
        Ok(())
    }
    
    // Disconnect from server
    pub fn disconnect(&self) -> Result<(), JsValue> {
        self.clear_ping_interval();
        
        if let Some(ws) = &*self.socket.get_value() {
            if ws.ready_state() == WebSocket::OPEN || ws.ready_state() == WebSocket::CONNECTING {
                ws.close()?;
            }
        }
        
        *self.socket.get_value() = None;
        self.state.set(ConnectionState::Disconnected);
        
        Ok(())
    }
    
    // Subscribe to a topic's updates
    pub fn subscribe_to_topic(&self, topic_id: i64) -> Result<(), JsValue> {
        let mut topics = self.subscribed_topics.get_value();
        topics.insert(topic_id, ());
        self.subscribed_topics.set_value(topics);
        
        // If connected, send subscription
        if matches!(self.state.get(), ConnectionState::Connected) {
            if let Some(ws) = &*self.socket.get_value() {
                let msg = serde_json::json!({
                    "action": "subscribe",
                    "topic_id": topic_id
                });
                ws.send_with_str(&msg.to_string())?;
            }
        }
        
        Ok(())
    }
    
    // Subscribe to a category's updates
    pub fn subscribe_to_category(&self, category_id: i64) -> Result<(), JsValue> {
        let mut categories = self.subscribed_categories.get_value();
        categories.insert(category_id, ());
        self.subscribed_categories.set_value(categories);
        
        // If connected, send subscription
        if matches!(self.state.get(), ConnectionState::Connected) {
            if let Some(ws) = &*self.socket.get_value() {
                let msg = serde_json::json!({
                    "action": "subscribe_category",
                    "category_id": category_id
                });
                ws.send_with_str(&msg.to_string())?;
            }
        }
        
        Ok(())
    }
    
    // Handle incoming message
    fn handle_message(&self, update: ForumUpdate) {
        match &update {
            ForumUpdate::Pong { id } => {
                // Calculate latency for monitoring
                if let Some(ping_time) = *self.latest_ping_time.get_value() {
                    let now = js_sys::Date::now();
                    let latency = now - ping_time;
                    log::debug!("WebSocket latency: {}ms", latency);
                }
                return;
            },
            ForumUpdate::Ping { id } => {
                // Respond to server ping
                if let Some(ws) = &*self.socket.get_value() {
                    let pong = ForumUpdate::Pong { id: *id };
                    if let Ok(pong_str) = serde_json::to_string(&pong) {
                        let _ = ws.send_with_str(&pong_str);
                    }
                }
                return;
            },
            _ => {
                // Process other updates
                self.updates.update(|updates| {
                    updates.push(update.clone());
                    
                    // Keep last 100 updates
                    if updates.len() > 100 {
                        *updates = updates.split_off(updates.len() - 100);
                    }
                });
            }
        }
    }
    
    // Resubscribe to topics and categories after reconnect
    fn resubscribe(&self) {
        // Resubscribe to topics
        let topics = self.subscribed_topics.get_value();
        for topic_id in topics.keys() {
            let _ = self.subscribe_to_topic(*topic_id);
        }
        
        // Resubscribe to categories
        let categories = self.subscribed_categories.get_value();
        for category_id in categories.keys() {
            let _ = self.subscribe_to_category(*category_id);
        }
    }
    
    // Start ping interval to keep connection alive
    fn start_ping_interval(&self) {
        self.clear_ping_interval();
        
        let client = self.clone();
        let callback = Closure::wrap(Box::new(move || {
            if let Some(ws) = &*client.socket.get_value() {
                if ws.ready_state() == WebSocket::OPEN {
                    // Send ping message
                    let ping_id = client.ping_id.get_value().fetch_add(1, Ordering::SeqCst);
                    let ping = ForumUpdate::Ping { id: ping_id };
                    if let Ok(ping_str) = serde_json::to_string(&ping) {
                        let _ = ws.send_with_str(&ping_str);
                        *client.latest_ping_time.get_value() = Some(js_sys::Date::now());
                    }
                }
            }
        }) as Box<dyn FnMut()>);
        
        if let Some(window) = web_sys::window() {
            let handle = window
                .set_interval_with_callback_and_timeout_and_arguments_0(
                    callback.as_ref().unchecked_ref(),
                    30000, // 30 seconds ping interval
                )
                .unwrap_or(0);
                
            *self.ping_interval_handle.get_value() = Some(handle);
            callback.forget();
        }
    }
    
    // Clear ping interval
    fn clear_ping_interval(&self) {
        if let Some(handle) = *self.ping_interval_handle.get_value() {
            if let Some(window) = web_sys::window() {
                window.clear_interval_with_handle(handle);
            }
            *self.ping_interval_handle.get_value() = None;
        }
    }
    
    // Attempt to reconnect with exponential backoff
    fn attempt_reconnect(&self, attempt: u32) {
        if attempt > self.max_reconnect_attempts {
            self.state.set(ConnectionState::Disconnected);
            return;
        }
        
        self.state.set(ConnectionState::Reconnecting { attempt });
        
        // Calculate backoff time (exponential with jitter)
        let base_timeout = self.reconnect_timeout_ms as f64 * 1.5f64.powi(attempt as i32 - 1);
        let jitter = js_sys::Math::random() * base_timeout * 0.2;
        let timeout = (base_timeout + jitter) as u32;
        
        let client = self.clone();
        spawn_local(async move {
            // Wait for backoff time
            TimeoutFuture::new(timeout).await;
            
            // Try to reconnect
            match client.connect() {
                Ok(_) => {
                    // Connection started
                },
                Err(_) => {
                    // Failed to connect, try again
                    client.attempt_reconnect(attempt + 1);
                }
            }
        });
    }
}

// Leptos hook for using realtime updates
#[hook]
pub fn use_forum_realtime() -> (
    // Connection state
    Signal<ConnectionState>,
    // Connect function
    impl Fn() -> Result<(), JsValue>,
    // Subscribe to topic function
    impl Fn(i64) -> Result<(), JsValue>,
    // Updates signal
    Signal<Vec<ForumUpdate>>,
) {
    // Create or use context
    let client = use_context::<ForumRealtimeClient>().unwrap_or_else(|| {
        let client = ForumRealtimeClient::new();
        provide_context(client.clone());
        client
    });
    
    let state = client.state.read_only();
    let updates = client.updates.read_only();
    
    let connect = move || client.connect();
    let subscribe = move |topic_id| client.subscribe_to_topic(topic_id);
    
    (state, connect, subscribe, updates)
}