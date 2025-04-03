use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};
use web_sys::FormData;

use crate::utils::auth::{save_auth_token, get_auth_token, is_authenticated};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserData {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub display_name: String,
    pub avatar_url: Option<String>,
    pub is_admin: bool,
    pub trust_level: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthData {
    pub token: String,
    pub user: UserData,
}

// Signal resource for current user
#[component]
pub fn AuthProvider(cx: Scope, children: Children) -> impl IntoView {
    // Create a signal for authentication state
    let (auth_data, set_auth_data) = create_signal::<Option<AuthData>>(cx, None);
    
    // Check local storage for existing token on component mount
    create_effect(cx, move |_| {
        let window = web_sys::window().unwrap();
        let storage = window.local_storage().unwrap().unwrap();
        
        if let Ok(Some(token)) = storage.get_item("auth_token") {
            if let Ok(Some(user_data)) = storage.get_item("user_data") {
                if let Ok(user) = serde_json::from_str::<UserData>(&user_data) {
                    set_auth_data.set(Some(AuthData {
                        token,
                        user,
                    }));
                }
            }
        }
    });
    
    // Function to set authentication data and save to local storage
    let login = create_callback(cx, move |auth: AuthData| {
        let window = web_sys::window().unwrap();
        let storage = window.local_storage().unwrap().unwrap();
        
        // Save token and user data to local storage
        let _ = storage.set_item("auth_token", &auth.token);
        let _ = storage.set_item("user_data", &serde_json::to_string(&auth.user).unwrap());
        
        // Update auth signal
        set_auth_data.set(Some(auth));
    });
    
    // Function to logout
    let logout = create_callback(cx, move |_| {
        let window = web_sys::window().unwrap();
        let storage = window.local_storage().unwrap().unwrap();
        
        // Clear auth data from storage
        let _ = storage.remove_item("auth_token");
        let _ = storage.remove_item("user_data");
        
        // Clear auth signal
        set_auth_data.set(None);
    });
    
    // Provide the auth context to all child components
    provide_context(cx, auth_data);
    provide_context(cx, login);
    provide_context(cx, logout);
    
    // Render children
    view! { cx,
        {children(cx)}
    }
}

// Registration form component
#[component]
pub fn Register(cx: Scope) -> impl IntoView {
    let login = use_context::<Callback<AuthData>>(cx).expect("Login callback not provided");
    
    let (username, set_username) = create_signal(cx, String::new());
    let (email, set_email) = create_signal(cx, String::new());
    let (password, set_password) = create_signal(cx, String::new());
    let (display_name, set_display_name) = create_signal(cx, String::new());
    let (error, set_error) = create_signal(cx, None::<String>);
    let (is_submitting, set_is_submitting) = create_signal(cx, false);
    
    let handle_register = create_action(cx, move |_: &()| {
        let username_value = username.get();
        let email_value = email.get();
        let password_value = password.get();
        let display_name_value = display_name.get();
        let login_callback = login;
        
        async move {
            set_is_submitting.set(true);
            set_error.set(None);
            
            // Basic validation
            if username_value.trim().is_empty() || email_value.trim().is_empty() || password_value.trim().is_empty() {
                set_error.set(Some("All fields are required".to_string()));
                set_is_submitting.set(false);
                return;
            }
            
            // Create payload
            let payload = serde_json::json!({
                "username": username_value,
                "email": email_value,
                "password": password_value,
                "display_name": if display_name_value.trim().is_empty() { 
                    null 
                } else { 
                    Some(display_name_value) 
                }
            });
            
            // Send registration request
            let client = reqwest::Client::new();
            let response = client.post("http://localhost:3030/auth/register")
                .json(&payload)
                .send()
                .await;
            
            match response {
                Ok(resp) => {
                    if resp.status().is_success() {
                        match resp.json::<AuthData>().await {
                            Ok(auth_data) => {
                                // Login with the received token and user data
                                login_callback.call(auth_data);
                                
                                // Navigate to home page
                                let window = web_sys::window().unwrap();
                                let _ = window.location().set_href("/");
                            },
                            Err(e) => {
                                set_error.set(Some(format!("Failed to parse response: {}", e)));
                            }
                        }
                    } else {
                        let error_message = resp.text().await
                            .unwrap_or_else(|_| "Registration failed".to_string());
                        set_error.set(Some(error_message));
                    }
                },
                Err(e) => {
                    set_error.set(Some(format!("Request failed: {}", e)));
                }
            }
            
            set_is_submitting.set(false);
        }
    });
    
    let on_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        handle_register.dispatch(());
    };
    
    view! { cx,
        <div class="auth-form">
            <h1>"Register an Account"</h1>
            
            {move || error.get().map(|err| view! { cx, <div class="error-message">{err}</div> })}
            
            <form on:submit=on_submit>
                <div class="form-group">
                    <label for="username">"Username"</label>
                    <input
                        id="username"
                        type="text"
                        value=move || username.get()
                        on:input=move |ev| set_username.set(event_target_value(&ev))
                        required
                    />
                </div>
                
                <div class="form-group">
                    <label for="email">"Email"</label>
                    <input
                        id="email"
                        type="email"
                        value=move || email.get()
                        on:input=move |ev| set_email.set(event_target_value(&ev))
                        required
                    />
                </div>
                
                <div class="form-group">
                    <label for="display_name">"Display Name"</label>
                    <input
                        id="display_name"
                        type="text"
                        value=move || display_name.get()
                        on:input=move |ev| set_display_name.set(event_target_value(&ev))
                    />
                    <small>"Optional - defaults to username"</small>
                </div>
                
                <div class="form-group">
                    <label for="password">"Password"</label>
                    <input
                        id="password"
                        type="password"
                        value=move || password.get()
                        on:input=move |ev| set_password.set(event_target_value(&ev))
                        required
                    />
                </div>
                
                <div class="form-actions">
                    <button type="submit" disabled=move || is_submitting.get() class="button primary">
                        {move || if is_submitting.get() { "Registering..." } else { "Register" }}
                    </button>
                    <a href="/login" class="form-link">"Already have an account? Login"</a>
                </div>
            </form>
        </div>
    }
}

// Login form component
#[component]
pub fn Login(cx: Scope) -> impl IntoView {
    let login = use_context::<Callback<AuthData>>(cx).expect("Login callback not provided");
    
    let (username, set_username) = create_signal(cx, String::new());
    let (password, set_password) = create_signal(cx, String::new());
    let (error, set_error) = create_signal(cx, None::<String>);
    let (is_submitting, set_is_submitting) = create_signal(cx, false);
    
    let handle_login = create_action(cx, move |_: &()| {
        let username_value = username.get();
        let password_value = password.get();
        let login_callback = login;
        
        async move {
            set_is_submitting.set(true);
            set_error.set(None);
            
            // Basic validation
            if username_value.trim().is_empty() || password_value.trim().is_empty() {
                set_error.set(Some("Username and password are required".to_string()));
                set_is_submitting.set(false);
                return;
            }
            
            // Create payload
            let payload = serde_json::json!({
                "username": username_value,
                "password": password_value,
            });
            
            // Send login request
            let client = reqwest::Client::new();
            let response = client.post("http://localhost:3030/auth/login")
                .json(&payload)
                .send()
                .await;
            
            match response {
                Ok(resp) => {
                    if resp.status().is_success() {
                        match resp.json::<AuthData>().await {
                            Ok(auth_data) => {
                                // Login with the received token and user data
                                login_callback.call(auth_data);
                                
                                // Navigate to home page
                                let window = web_sys::window().unwrap();
                                let _ = window.location().set_href("/");
                            },
                            Err(e) => {
                                set_error.set(Some(format!("Failed to parse response: {}", e)));
                            }
                        }
                    } else {
                        let error_message = resp.text().await
                            .unwrap_or_else(|_| "Login failed".to_string());
                        set_error.set(Some(error_message));
                    }
                },
                Err(e) => {
                    set_error.set(Some(format!("Request failed: {}", e)));
                }
            }
            
            set_is_submitting.set(false);
        }
    });
    
    let on_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        handle_login.dispatch(());
    };
    
    view! { cx,
        <div class="auth-form">
            <h1>"Login to Your Account"</h1>
            
            {move || error.get().map(|err| view! { cx, <div class="error-message">{err}</div> })}
            
            <form on:submit=on_submit>
                <div class="form-group">
                    <label for="username">"Username"</label>
                    <input
                        id="username"
                        type="text"
                        value=move || username.get()
                        on:input=move |ev| set_username.set(event_target_value(&ev))
                        required
                    />
                </div>
                
                <div class="form-group">
                    <label for="password">"Password"</label>
                    <input
                        id="password"
                        type="password"
                        value=move || password.get()
                        on:input=move |ev| set_password.set(event_target_value(&ev))
                        required
                    />
                </div>
                
                <div class="form-actions">
                    <button type="submit" disabled=move || is_submitting.get() class="button primary">
                        {move || if is_submitting.get() { "Logging in..." } else { "Login" }}
                    </button>
                    <a href="/register" class="form-link">"Need an account? Register"</a>
                </div>
            </form>
        </div>
    }
}

// User profile component
#[component]
pub fn UserProfile(cx: Scope) -> impl IntoView {
    let auth_data = use_context::<Signal<Option<AuthData>>>(cx)
        .expect("Auth data not provided");
    
    let logout = use_context::<Callback<()>>(cx)
        .expect("Logout callback not provided");
    
    // Redirect to login if not authenticated
    create_effect(cx, move |_| {
        if auth_data.get().is_none() {
            let window = web_sys::window().unwrap();
            let _ = window.location().set_href("/login");
        }
    });
    
    view! { cx,
        <div class="user-profile">
            {move || match auth_data.get() {
                Some(data) => {
                    view! { cx,
                        <div class="profile-container">
                            <h1>"User Profile"</h1>
                            
                            <div class="profile-info">
                                <div class="profile-avatar">
                                    {if let Some(avatar_url) = &data.user.avatar_url {
                                        view! { cx, <img src={avatar_url.clone()} alt="User avatar" /> }.into_view(cx)
                                    } else {
                                        view! { cx, <div class="avatar-placeholder">
                                            {&data.user.display_name.chars().next().unwrap_or('U')}
                                        </div> }.into_view(cx)
                                    }}
                                </div>
                                
                                <div class="profile-details">
                                    <h2>{&data.user.display_name}</h2>
                                    <p class="username">{"@"}{&data.user.username}</p>
                                    <p class="email">{&data.user.email}</p>
                                    
                                    <div class="trust-level">
                                        {"Trust Level: "}{data.user.trust_level}
                                        {if data.user.is_admin {
                                            view! { cx, <span class="admin-badge">"Admin"</span> }.into_view(cx)
                                        } else { view! {}.into_view(cx) }}
                                    </div>
                                </div>
                            </div>
                            
                            <div class="profile-actions">
                                <a href="/profile/edit" class="button secondary">
                                    "Edit Profile"
                                </a>
                                <button on:click=move |_| logout.call(()) class="button danger">
                                    "Logout"
                                </button>
                            </div>
                        </div>
                    }.into_view(cx)
                },
                None => view! { cx, <p>"Loading..."</p> }.into_view(cx)
            }}
        </div>
    }
}

#[component]
pub fn LoginForm(cx: Scope) -> impl IntoView {
    let (error, set_error) = create_signal(cx, None::<String>);
    let navigate = use_navigate(cx);
    
    let handle_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        
        let form_data = FormData::new_with_form(&ev.target.unwrap().dyn_into::<web_sys::HTMLFormElement>().unwrap()).unwrap();
        let email = form_data.get("email").as_string().unwrap_or_default();
        let password = form_data.get("password").as_string().unwrap_or_default();
        
        // Basic validation
        if email.is_empty() || password.is_empty() {
            set_error.set(Some("Email and password are required".to_string()));
            return;
        }
        
        // Create login request
        let login_request = serde_json::json!({
            "email": email,
            "password": password,
        });
        
        // Send login request
        wasm_bindgen_futures::spawn_local(async move {
            match gloo_net::http::Request::post("/api/auth/login")
                .json(&login_request)
                .unwrap()
                .send()
                .await
            {
                Ok(response) => {
                    if response.ok() {
                        match response.json::<AuthResponse>().await {
                            Ok(auth_response) => {
                                // Save token and redirect
                                save_auth_token(&auth_response.token);
                                navigate("/", NavigateOptions::default());
                            },
                            Err(_) => set_error.set(Some("Failed to parse response".to_string())),
                        }
                    } else {
                        match response.text().await {
                            Ok(text) => set_error.set(Some(text)),
                            Err(_) => set_error.set(Some("Login failed".to_string())),
                        }
                    }
                },
                Err(_) => set_error.set(Some("Network error".to_string())),
            }
        });
    };

    view! { cx,
        <div class="auth-form">
            <h2>"Login"</h2>
            
            {move || error.get().map(|err| view! { cx,
                <div class="error-message">{err}</div>
            })}
            
            <form on:submit=handle_submit>
                <div class="form-group">
                    <label for="email">"Email"</label>
                    <input type="email" name="email" id="email" required/>
                </div>
                
                <div class="form-group">
                    <label for="password">"Password"</label>
                    <input type="password" name="password" id="password" required/>
                </div>
                
                <button type="submit">"Login"</button>
            </form>
            
            <div class="auth-links">
                <p>"Don't have an account? " <a href="/register">"Register"</a></p>
            </div>
        </div>
    }
}

#[component]
pub fn RegisterForm(cx: Scope) -> impl IntoView {
    let (error, set_error) = create_signal(cx, None::<String>);
    let navigate = use_navigate(cx);
    
    let handle_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        
        let form_data = FormData::new_with_form(&ev.target.unwrap().dyn_into::<web_sys::HTMLFormElement>().unwrap()).unwrap();
        let name = form_data.get("name").as_string().unwrap_or_default();
        let email = form_data.get("email").as_string().unwrap_or_default();
        let password = form_data.get("password").as_string().unwrap_or_default();
        
        // Basic validation
        if name.is_empty() || email.is_empty() || password.is_empty() {
            set_error.set(Some("All fields are required".to_string()));
            return;
        }
        
        if password.len() < 8 {
            set_error.set(Some("Password must be at least 8 characters".to_string()));
            return;
        }
        
        // Create registration request
        let register_request = serde_json::json!({
            "name": name,
            "email": email,
            "password": password,
        });
        
        // Send registration request
        wasm_bindgen_futures::spawn_local(async move {
            match gloo_net::http::Request::post("/api/auth/register")
                .json(&register_request)
                .unwrap()
                .send()
                .await
            {
                Ok(response) => {
                    if response.ok() {
                        // Registration successful, redirect to login
                        navigate("/login", NavigateOptions::default());
                    } else {
                        match response.text().await {
                            Ok(text) => set_error.set(Some(text)),
                            Err(_) => set_error.set(Some("Registration failed".to_string())),
                        }
                    }
                },
                Err(_) => set_error.set(Some("Network error".to_string())),
            }
        });
    };

    view! { cx,
        <div class="auth-form">
            <h2>"Register"</h2>
            
            {move || error.get().map(|err| view! { cx,
                <div class="error-message">{err}</div>
            })}
            
            <form on:submit=handle_submit>
                <div class="form-group">
                    <label for="name">"Name"</label>
                    <input type="text" name="name" id="name" required/>
                </div>
                
                <div class="form-group">
                    <label for="email">"Email"</label>
                    <input type="email" name="email" id="email" required/>
                </div>
                
                <div class="form-group">
                    <label for="password">"Password"</label>
                    <input type="password" name="password" id="password" required/>
                </div>
                
                <button type="submit">"Register"</button>
            </form>
            
            <div class="auth-links">
                <p>"Already have an account? " <a href="/login">"Login"</a></p>
            </div>
        </div>
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct AuthResponse {
    token: String,
    user: UserProfile,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct UserProfile {
    user: User,
    roles: Vec<UserRole>,
    forum_trust_level: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct User {
    id: Option<i64>,
    name: String,
    email: String,
    avatar_url: Option<String>,
    created_at: Option<String>,
    updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct UserRole {
    id: Option<i64>,
    user_id: i64,
    role: String,
    context_type: Option<String>,
    context_id: Option<i64>,
}