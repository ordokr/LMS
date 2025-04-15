# Authentication API Reference

This document describes the Tauri command API for authentication in the LMS Integration Project.

## Commands Overview

| Command | Function | Description | Status |
|---------|----------|-------------|--------|
| `login_user` | `login_user(login_request: LoginRequest)` | Authenticates a user with email and password | Implemented |
| `register_user` | `register_user(register_request: RegisterRequest)` | Registers a new user | Implemented |
| `get_current_user` | `get_current_user(token: string)` | Gets current authenticated user information | Implemented |

## Data Types

### LoginRequest

```rust
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}
```

### RegisterRequest

```rust
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
}
```

### AuthResponse

```rust
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AuthResponse {
    pub user: User,
    pub token: String,
}
```

### User

```rust
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
    pub id: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub role: String,
    pub created_at: String,
    pub updated_at: String,
}
```

## Example Usage

```typescript
// Login
const authResponse = await invoke<AuthResponse>("login_user", { 
  login_request: {
    email: "user@example.com",
    password: "securepassword"
  }
});

// Store token
localStorage.setItem("auth_token", authResponse.token);

// Get current user
const token = localStorage.getItem("auth_token");
const currentUser = await invoke<User>("get_current_user", { token });
```

## Tauri Command Handler

```rust
.invoke_handler(tauri::generate_handler![
    // ...other commands
    api::auth::login_user,
    api::auth::register_user,
    api::auth::get_current_user,
])
```

## Leptos Component Example

```rust
// In your Leptos component
use crate::models::user::{LoginRequest, AuthResponse, User};
use leptos::*;
use tauri_sys::tauri::invoke;

#[component]
pub fn LoginForm() -> impl IntoView {
    // Form state
    let (email, set_email) = create_signal(String::new());
    let (password, set_password) = create_signal(String::new());
    let (user, set_user) = create_signal::<Option<User>>(None);
    
    // Login action
    let login = create_action(move |_| async move {
        let login_request = LoginRequest {
            email: email.get(),
            password: password.get(),
        };
        
        match invoke::<_, AuthResponse>("login_user", &login_request).await {
            Ok(auth_response) => {
                // Store token in session storage
                web_sys::window()
                    .unwrap()
                    .local_storage()
                    .unwrap()
                    .unwrap()
                    .set_item("auth_token", &auth_response.token)
                    .unwrap();
                
                // Update user state
                set_user.set(Some(auth_response.user));
                
                Ok(())
            }
            Err(e) => Err(e.to_string()),
        }
    });
    
    // Get current user function
    let get_current_user = create_action(move |_| async move {
        // Get token from storage
        let token = web_sys::window()
            .unwrap()
            .local_storage()
            .unwrap()
            .unwrap()
            .get_item("auth_token")
            .unwrap()
            .unwrap_or_default();
        
        match invoke::<_, User>("get_current_user", &token).await {
            Ok(user) => {
                set_user.set(Some(user));
                Ok(())
            }
            Err(e) => Err(e.to_string()),
        }
    });
    
    view! {
        <form on:submit=move |ev| {
            ev.prevent_default();
            login.dispatch(());
        }>
            <div>
                <label>"Email"</label>
                <input
                    type="email"
                    on:input=move |ev| {
                        set_email.set(event_target_value(&ev));
                    }
                    value=email
                />
            </div>
            <div>
                <label>"Password"</label>
                <input
                    type="password"
                    on:input=move |ev| {
                        set_password.set(event_target_value(&ev));
                    }
                    value=password
                />
            </div>
            <button type="submit">"Login"</button>
        </form>
        
        {move || match user.get() {
            Some(u) => view! { <div>"Logged in as: " {u.email}</div> },
            None => view! { <div>"Not logged in"</div> }
        }}
    }
}
