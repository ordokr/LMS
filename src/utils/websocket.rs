use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use gloo_timers::callback::Interval;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{MessageEvent, WebSocket, CloseEvent, ErrorEvent};
use serde::{Serialize, Deserialize};

pub type MessageCallback = Box<dyn Fn(String) + 'static>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum WebSocketMessage {
    TopicUpdate { id: i64, title: String },
    NewPost { topic_id: i64, post_id: i64, author: String },
    UserOnline { user_id: i64, username: String },
    UserOffline { user_id: i64 },
    Notification { id: String, message: String, type_: String },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionStatus {
    Connecting,
    Open,
    Closed,
    Error,
}

pub struct WebSocketClient {
    url: String,
    socket: Option<WebSocket>,
    status: ConnectionStatus,
    message_callbacks: HashMap<String, Vec<MessageCallback>>,
    status_callbacks: Vec<Box<dyn Fn(ConnectionStatus) + 'static>>,
    auto_reconnect: bool,
    reconnect_interval: Duration,
    _reconnect_timer: Option<Interval>,
    max_reconnect_attempts: usize,
    current_reconnect_attempts: usize,
    message_queue: Vec<String>,
}

impl WebSocketClient {
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
            socket: None,
            status: ConnectionStatus::Closed,
            message_callbacks: HashMap::new(),
            status_callbacks: Vec::new(),
            auto_reconnect: true,
            reconnect_interval: Duration::from_secs(5),
            _reconnect_timer: None,
            max_reconnect_attempts: 5,
            current_reconnect_attempts: 0,
            message_queue: Vec::new(),
        }
    }
    
    pub fn connect(&mut self) -> Result<(), String> {
        if self.socket.is_some() && self.status == ConnectionStatus::Open {
            return Ok(());
        }
        
        let ws = match WebSocket::new(&self.url) {
            Ok(ws) => ws,
            Err(err) => return Err(format!("Failed to create WebSocket: {:?}", err)),
        };
        
        self.setup_callbacks(&ws);
        self.socket = Some(ws);
        self.status = ConnectionStatus::Connecting;
        self.notify_status_change();
        
        Ok(())
    }
    
    fn setup_callbacks(&mut self, ws: &WebSocket) {
        let ws_clone = ws.clone();
        let on_open = Closure::wrap(Box::new(move |_| {
            log::info!("WebSocket connection opened");
        }) as Box<dyn FnMut(JsValue)>);
        ws.set_onopen(Some(on_open.as_ref().unchecked_ref()));
        on_open.forget();
        
        let clients_clone = Arc::new(Mutex::new(self.clone()));
        
        // Handle incoming messages
        let on_message = {
            let clients = clients_clone.clone();
            Closure::wrap(Box::new(move |e: MessageEvent| {
                if let Ok(text) = e.data().dyn_into::<js_sys::JsString>() {
                    let message = String::from(text);
                    let mut clients = clients.lock().unwrap();
                    clients.handle_message(&message);
                }
            }) as Box<dyn FnMut(MessageEvent)>)
        };
        ws.set_onmessage(Some(on_message.as_ref().unchecked_ref()));
        on_message.forget();
        
        // Handle connection close
        let on_close = {
            let clients = clients_clone.clone();
            Closure::wrap(Box::new(move |e: CloseEvent| {
                log::info!("WebSocket closed: {} - {}", e.code(), e.reason());
                let mut clients = clients.lock().unwrap();
                clients.status = ConnectionStatus::Closed;
                clients.notify_status_change();
                clients.attempt_reconnect();
            }) as Box<dyn FnMut(CloseEvent)>)
        };
        ws.set_onclose(Some(on_close.as_ref().unchecked_ref()));
        on_close.forget();
        
        // Handle errors
        let on_error = {
            let clients = clients_clone.clone();
            Closure::wrap(Box::new(move |_: ErrorEvent| {
                log::error!("WebSocket error");
                let mut clients = clients.lock().unwrap();
                clients.status = ConnectionStatus::Error;
                clients.notify_status_change();
                clients.attempt_reconnect();
            }) as Box<dyn FnMut(ErrorEvent)>)
        };
        ws.set_onerror(Some(on_error.as_ref().unchecked_ref()));
        on_error.forget();
    }
    
    fn handle_message(&mut self, message: &str) {
        // Dispatch message to all callbacks
        for (topic, callbacks) in &self.message_callbacks {
            // Check if message is for this topic
            if message.contains(topic) {
                for callback in callbacks {
                    callback(message.to_string());
                }
            }
        }
    }
    
    fn attempt_reconnect(&mut self) {
        if !self.auto_reconnect || self.current_reconnect_attempts >= self.max_reconnect_attempts {
            return;
        }
        
        self.current_reconnect_attempts += 1;
        log::info!(
            "Attempting to reconnect ({}/{}) in {} ms",
            self.current_reconnect_attempts,
            self.max_reconnect_attempts,
            self.reconnect_interval.as_millis()
        );
        
        let url = self.url.clone();
        let clients_clone = Arc::new(Mutex::new(self.clone()));
        
        let interval = {
            let clients = clients_clone.clone();
            Interval::new(self.reconnect_interval.as_millis() as u32, move || {
                let mut clients = clients.lock().unwrap();
                if let Err(err) = clients.connect() {
                    log::error!("Failed to reconnect: {}", err);
                } else {
                    log::info!("Reconnection initiated");
                }
            })
        };
        
        self._reconnect_timer = Some(interval);
    }
    
    pub fn send(&mut self, message: &WebSocketMessage) -> Result<(), String> {
        match serde_json::to_string(message) {
            Ok(json) => self.send_raw(&json),
            Err(err) => Err(format!("Failed to serialize message: {}", err)),
        }
    }
    
    pub fn send_raw(&mut self, message: &str) -> Result<(), String> {
        if let Some(ws) = &self.socket {
            if self.status != ConnectionStatus::Open {
                // Queue message for later sending
                self.message_queue.push(message.to_string());
                return Ok(());
            }
            
            ws.send_with_str(message)
                .map_err(|err| format!("Failed to send message: {:?}", err))
        } else {
            // Queue message for when we connect
            self.message_queue.push(message.to_string());
            self.connect()?;
            Ok(())
        }
    }
    
    pub fn subscribe(&mut self, topic: &str, callback: MessageCallback) {
        let entry = self.message_callbacks
            .entry(topic.to_string())
            .or_insert_with(Vec::new);
        
        entry.push(callback);
    }
    
    pub fn on_status_change<F>(&mut self, callback: F)
    where
        F: Fn(ConnectionStatus) + 'static,
    {
        self.status_callbacks.push(Box::new(callback));
    }
    
    fn notify_status_change(&self) {
        for callback in &self.status_callbacks {
            callback(self.status);
        }
    }
    
    pub fn close(&mut self) {
        if let Some(ws) = &self.socket {
            let _ = ws.close();
        }
        self.socket = None;
        self.status = ConnectionStatus::Closed;
        self._reconnect_timer = None;
    }
    
    pub fn status(&self) -> ConnectionStatus {
        self.status
    }
    
    fn process_queued_messages(&mut self) {
        if self.status != ConnectionStatus::Open || self.message_queue.is_empty() {
            return;
        }
        
        let messages = std::mem::take(&mut self.message_queue);
        for message in messages {
            if let Err(err) = self.send_raw(&message) {
                log::error!("Failed to send queued message: {}", err);
                self.message_queue.push(message);
                break;
            }
        }
    }
}

// Create a React-like hook for WebSocket usage
#[hook]
pub fn use_websocket(url: &str) -> (
    ConnectionStatus,
    impl Fn(WebSocketMessage) -> Result<(), String>,
    impl Fn(&str, Box<dyn Fn(String) + 'static>) -> (),
) {
    let (status, set_status) = create_signal(ConnectionStatus::Closed);
    let ws_client = use_memo(move |_| {
        let client = WebSocketClient::new(url);
        std::rc::Rc::new(std::cell::RefCell::new(client))
    });
    
    // Connect when component mounts
    create_effect(move |_| {
        let mut client = ws_client.borrow_mut();
        if let Err(err) = client.connect() {
            log::error!("Failed to connect to WebSocket: {}", err);
        }
        
        // Setup status change callback
        client.on_status_change(move |new_status| {
            set_status.set(new_status);
        });
    });
    
    // Send function
    let send = {
        let ws_client = ws_client.clone();
        move |message: WebSocketMessage| {
            let mut client = ws_client.borrow_mut();
            client.send(&message)
        }
    };
    
    // Subscribe function
    let subscribe = {
        let ws_client = ws_client.clone();
        move |topic: &str, callback: Box<dyn Fn(String) + 'static>| {
            let mut client = ws_client.borrow_mut();
            client.subscribe(topic, callback);
        }
    };
    
    // Clean up on component unmount
    on_cleanup(move || {
        let mut client = ws_client.borrow_mut();
        client.close();
    });
    
    (status, send, subscribe)
}