use leptos::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsCast;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub role: String,
    pub canvas_id: String,
}

#[derive(Clone, Debug, Deserialize)]
struct LoginResponse {
    token: String,
    user: User,
}

#[derive(Clone, Serialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Clone, Serialize)]
pub struct RegisterData {
    username: String,
    email: String,
    password: String,
    canvas_id: String,
}

pub fn use_auth() -> AuthHook {
    static AUTH_CONTEXT: std::sync::OnceLock<RwSignal<Option<(String, User)>>> = std::sync::OnceLock::new();
    let auth_signal = AUTH_CONTEXT.get_or_init(|| {
        // Initialize from localStorage if available
        let token = local_storage_get("lms_token");
        let user_json = local_storage_get("lms_user");
        
        let initial_value = match (token, user_json) {
            (Some(token), Some(user_json)) => {
                match serde_json::from_str::<User>(&user_json) {
                    Ok(user) => Some((token, user)),
                    Err(_) => None,
                }
            },
            _ => None,
        };
        
        create_rw_signal(initial_value)
    });

    AuthHook {
        token: create_memo(move |_| auth_signal.get().map(|(token, _)| token)),
        user: create_memo(move |_| auth_signal.get().map(|(_, user)| user)),
        is_authenticated: create_memo(move |_| auth_signal.get().is_some()),
        login: create_action(move |(username, password): &(String, String)| {
            let username = username.clone();
            let password = password.clone();
            async move {
                login(&username, &password).await.map(|(token, user)| {
                    // Save to localStorage
                    local_storage_set("lms_token", &token);
                    if let Ok(user_json) = serde_json::to_string(&user) {
                        local_storage_set("lms_user", &user_json);
                    }
                    
                    auth_signal.set(Some((token, user)));
                })
            }
        }),
        logout: create_action(move |_: &()| {
            async move {
                // Clear from localStorage
                local_storage_remove("lms_token");
                local_storage_remove("lms_user");
                
                auth_signal.set(None);
                Ok(())
            }
        }),
        register: create_action(move |data: &RegisterData| {
            let data = data.clone();
            async move {
                register(data).await
            }
        }),
    }
}

#[derive(Clone)]
pub struct AuthHook {
    pub token: Memo<Option<String>>,
    pub user: Memo<Option<User>>,
    pub is_authenticated: Memo<bool>,
    pub login: Action<(String, String), Result<(), String>>,
    pub logout: Action<(), Result<(), String>>,
    pub register: Action<RegisterData, Result<(), String>>,
}

async fn login(username: &str, password: &str) -> Result<(String, User), String> {
    let request = LoginRequest {
        username: username.to_string(),
        password: password.to_string(),
    };
    
    let mut opts = web_sys::RequestInit::new();
    opts.method("POST");
    
    let body = serde_json::to_string(&request)
        .map_err(|_| "Failed to serialize request".to_string())?;
    
    opts.body(Some(&wasm_bindgen::JsValue::from_str(&body)));
    
    let request = web_sys::Request::new_with_str_and_init(
        "/api/auth/login", 
        &opts,
    ).map_err(|_| "Failed to create request".to_string())?;
    
    request.headers().set("Content-Type", "application/json")
        .map_err(|_| "Failed to set headers".to_string())?;
    
    let window = web_sys::window().ok_or("No window found".to_string())?;
    let resp_value = wasm_bindgen_futures::JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(|_| "Failed to fetch".to_string())?;
    
    let resp: web_sys::Response = resp_value.dyn_into()
        .map_err(|_| "Response is not a Response".to_string())?;
    
    if !resp.ok() {
        return Err(format!("Login failed with status: {}", resp.status()));
    }
    
    let json = wasm_bindgen_futures::JsFuture::from(
        resp.json().map_err(|_| "Failed to parse response as JSON".to_string())?
    )
    .await
    .map_err(|_| "Failed to parse JSON".to_string())?;
    
    let login_response: LoginResponse = serde_wasm_bindgen::from_value(json)
        .map_err(|_| "Failed to deserialize login response".to_string())?;
    
    Ok((login_response.token, login_response.user))
}

async fn register(data: RegisterData) -> Result<(), String> {
    let mut opts = web_sys::RequestInit::new();
    opts.method("POST");
    
    let body = serde_json::to_string(&data)
        .map_err(|_| "Failed to serialize request".to_string())?;
    
    opts.body(Some(&wasm_bindgen::JsValue::from_str(&body)));
    
    let request = web_sys::Request::new_with_str_and_init(
        "/api/auth/register", 
        &opts,
    ).map_err(|_| "Failed to create request".to_string())?;
    
    request.headers().set("Content-Type", "application/json")
        .map_err(|_| "Failed to set headers".to_string())?;
    
    let window = web_sys::window().ok_or("No window found".to_string())?;
    let resp_value = wasm_bindgen_futures::JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(|_| "Failed to fetch".to_string())?;
    
    let resp: web_sys::Response = resp_value.dyn_into()
        .map_err(|_| "Response is not a Response".to_string())?;
    
    if !resp.ok() {
        return Err(format!("Registration failed with status: {}", resp.status()));
    }
    
    Ok(())
}

// Helper functions for localStorage
fn local_storage_get(key: &str) -> Option<String> {
    let window = web_sys::window()?;
    let storage = window.local_storage().ok()??;
    storage.get_item(key).ok()?
}

fn local_storage_set(key: &str, value: &str) {
    if let Some(window) = web_sys::window() {
        if let Ok(Some(storage)) = window.local_storage() {
            let _ = storage.set_item(key, value);
        }
    }
}

fn local_storage_remove(key: &str) {
    if let Some(window) = web_sys::window() {
        if let Ok(Some(storage)) = window.local_storage() {
            let _ = storage.remove_item(key);
        }
    }
}