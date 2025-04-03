use leptos::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
struct RegisterRequest {
    name: String,
    email: String,
    password: String,
    password_confirmation: String,
    username: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct RegisterResponse {
    token: String,
    // Add any other fields returned by your API
}

#[component]
pub fn Register(cx: Scope) -> impl IntoView {
    let (name, set_name) = create_signal(cx, String::new());
    let (email, set_email) = create_signal(cx, String::new());
    let (username, set_username) = create_signal(cx, String::new());
    let (password, set_password) = create_signal(cx, String::new());
    let (password_confirmation, set_password_confirmation) = create_signal(cx, String::new());
    let (error, set_error) = create_signal(cx, String::new());
    let (is_loading, set_is_loading) = create_signal(cx, false);

    let handle_submit = create_action(cx, move |_: &()| {
        let name_value = name.get();
        let email_value = email.get();
        let username_value = username.get();
        let password_value = password.get();
        let password_confirmation_value = password_confirmation.get();
        
        async move {
            set_is_loading.set(true);
            set_error.set(String::new());

            // Basic validation
            if name_value.is_empty() || email_value.is_empty() || password_value.is_empty() {
                set_error.set("All fields are required".to_string());
                set_is_loading.set(false);
                return Err("Validation failed");
            }

            if password_value != password_confirmation_value {
                set_error.set("Passwords do not match".to_string());
                set_is_loading.set(false);
                return Err("Passwords don't match");
            }

            if password_value.len() < 8 {
                set_error.set("Password must be at least 8 characters long".to_string());
                set_is_loading.set(false);
                return Err("Password too short");
            }

            // Prepare the request
            let request = RegisterRequest {
                name: name_value,
                email: email_value,
                username: username_value,
                password: password_value,
                password_confirmation: password_confirmation_value,
            };

            // Make API call
            match reqwasm::http::Request::post("/api/auth/register")
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&request).unwrap())
                .send()
                .await
            {
                Ok(response) => {
                    if response.status() == 200 || response.status() == 201 {
                        match response.json::<RegisterResponse>().await {
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
                        set_error.set(format!("Registration failed: {}", error_text));
                        Err("Registration failed")
                    }
                }
                Err(_) => {
                    set_error.set("Network error. Please try again.".to_string());
                    Err("Network error")
                }
            }
            .map_err(|e| {
                logging::log!("Registration error: {}", e);
                e
            })?;

            set_is_loading.set(false);
            Ok(())
        }
    });

    view! { cx,
        <div class="auth-container">
            <div class="auth-card">
                <h2 class="auth-title">"Create an Account"</h2>
                
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
                        <label for="name">"Full Name"</label>
                        <input
                            type="text"
                            id="name"
                            placeholder="Enter your full name"
                            value=move || name.get()
                            on:input=move |ev| {
                                set_name.set(event_target_value(&ev));
                            }
                            required
                        />
                    </div>

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
                        <label for="username">"Username (optional)"</label>
                        <input
                            type="text"
                            id="username"
                            placeholder="Choose a username"
                            value=move || username.get()
                            on:input=move |ev| {
                                set_username.set(event_target_value(&ev));
                            }
                        />
                    </div>
                    
                    <div class="form-group">
                        <label for="password">"Password"</label>
                        <input
                            type="password"
                            id="password"
                            placeholder="Create a password"
                            value=move || password.get()
                            on:input=move |ev| {
                                set_password.set(event_target_value(&ev));
                            }
                            required
                        />
                    </div>

                    <div class="form-group">
                        <label for="password_confirmation">"Confirm Password"</label>
                        <input
                            type="password"
                            id="password_confirmation"
                            placeholder="Confirm your password"
                            value=move || password_confirmation.get()
                            on:input=move |ev| {
                                set_password_confirmation.set(event_target_value(&ev));
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
                                "Creating Account..."
                            } else {
                                "Register"
                            }
                        }}
                    </button>
                    
                    <div class="auth-links">
                        <a href="/login">"Already have an account? Log in"</a>
                    </div>
                </form>
            </div>
        </div>
    }
}