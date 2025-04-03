use leptos::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct LoginResponse {
    token: String,
    // Add any other fields returned by your API
}

#[component]
pub fn Login(cx: Scope) -> impl IntoView {
    let (email, set_email) = create_signal(cx, String::new());
    let (password, set_password) = create_signal(cx, String::new());
    let (error, set_error) = create_signal(cx, String::new());
    let (is_loading, set_is_loading) = create_signal(cx, false);

    let handle_submit = create_action(cx, move |_: &()| {
        let email_value = email.get();
        let password_value = password.get();
        
        async move {
            set_is_loading.set(true);
            set_error.set(String::new());

            // Basic validation
            if email_value.is_empty() || password_value.is_empty() {
                set_error.set("Email and password are required".to_string());
                set_is_loading.set(false);
                return Err("Validation failed");
            }

            // Prepare the request
            let request = LoginRequest {
                email: email_value,
                password: password_value,
            };

            // Make API call
            match reqwasm::http::Request::post("/api/auth/login")
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&request).unwrap())
                .send()
                .await
            {
                Ok(response) => {
                    if response.status() == 200 {
                        match response.json::<LoginResponse>().await {
                            Ok(data) => {
                                // Save token to local storage
                                let window = web_sys::window().unwrap();
                                let local_storage = window.local_storage().unwrap().unwrap();
                                local_storage.set_item("auth_token", &data.token).unwrap();
                                
                                // Redirect to dashboard
                                let history = use_navigate(cx);
                                history("/dashboard", Default::default());
                                
                                Ok(())
                            }
                            Err(_) => {
                                set_error.set("Failed to parse server response".to_string());
                                Err("Parse error")
                            }
                        }
                    } else {
                        let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
                        set_error.set(format!("Login failed: {}", error_text));
                        Err("Login failed")
                    }
                }
                Err(_) => {
                    set_error.set("Network error. Please try again.".to_string());
                    Err("Network error")
                }
            }
            .map_err(|e| {
                logging::log!("Login error: {}", e);
                e
            })?;

            set_is_loading.set(false);
            Ok(())
        }
    });

    view! { cx,
        <div class="auth-container">
            <div class="auth-card">
                <h2 class="auth-title">"Log In"</h2>
                
                {move || {
                    if !error.get().is_empty() {
                        view! { cx,
                            <div class="error-message">
                                {error.get()}
                            </div>
                        }
                    } else {
                        view! { cx, <></> }
                    }
                }}
                
                <form class="auth-form" on:submit=|ev| {
                    ev.prevent_default();
                    handle_submit.dispatch(());
                }>
                    <div class="form-group">
                        <label for="email">"Email"</label>
                        <input
                            type="email"
                            id="email"
                            placeholder="Enter your email"
                            value=move || email.get()
                            on:input=move |ev| {
                                set_email.set(event_target_value(&ev));
                            }
                            required
                        />
                    </div>
                    
                    <div class="form-group">
                        <label for="password">"Password"</label>
                        <input
                            type="password"
                            id="password"
                            placeholder="Enter your password"
                            value=move || password.get()
                            on:input=move |ev| {
                                set_password.set(event_target_value(&ev));
                            }
                            required
                        />
                    </div>
                    
                    <button
                        type="submit"
                        class="btn-primary"
                        disabled=move || is_loading.get()
                    >
                        {move || {
                            if is_loading.get() {
                                "Logging in..."
                            } else {
                                "Log In"
                            }
                        }}
                    </button>
                    
                    <div class="auth-links">
                        <a href="/register">"Don't have an account? Register"</a>
                        <a href="/forgot-password">"Forgot password?"</a>
                    </div>
                </form>
            </div>
        </div>
    }
}