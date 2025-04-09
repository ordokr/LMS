# Users API Reference

This document describes the Tauri command API for user profiles and settings in the LMS Integration Project.

## Commands Overview

| Command | Function | Description | Status |
|---------|----------|-------------|--------|
| `get_user_profile` | `get_user_profile(user_id: string)` | Retrieves a user's profile | Implemented |
| `update_user_profile` | `update_user_profile(user_id: string, profile_update: UserProfileUpdate)` | Updates a user's profile | Implemented |
| `get_user_preferences` | `get_user_preferences(user_id: string)` | Retrieves a user's preferences | Implemented |
| `update_user_preferences` | `update_user_preferences(user_id: string, preferences: object)` | Updates a user's preferences | Implemented |
| `get_user_integration_settings` | `get_user_integration_settings(user_id: string)` | Gets a user's integration settings | Implemented |
| `update_user_integration_settings` | `update_user_integration_settings(user_id: string, settings: object)` | Updates a user's integration settings | Implemented |

## Data Types

### UserProfile

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub user_id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub bio: String,
    pub avatar_url: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfileUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
}
```

## Leptos Component Example

```rust
// In your Leptos component
use crate::models::user::{UserProfile, UserProfileUpdate};
use leptos::*;
use tauri_sys::tauri::invoke;

#[component]
pub fn UserProfilePage(user_id: String) -> impl IntoView {
    // Fetch user profile
    let profile = create_resource(
        || user_id.clone(),
        |id| async move {
            invoke::<_, UserProfile>("get_user_profile", &serde_json::json!({
                "user_id": id
            })).await.ok()
        }
    );
    
    // State for edit mode
    let (editing, set_editing) = create_signal(false);
    let (bio, set_bio) = create_signal(String::new());
    let (avatar_url, set_avatar_url) = create_signal(String::new());
    
    // Effect to set initial form values when profile loads
    create_effect(move |_| {
        if let Some(Some(profile_data)) = profile.get() {
            set_bio.set(profile_data.bio);
            set_avatar_url.set(profile_data.avatar_url);
        }
    });
    
    // Update profile action
    let update_profile = create_action(move |_| {
        let profile_update = UserProfileUpdate {
            first_name: None,
            last_name: None,
            email: None,
            bio: Some(bio.get()),
            avatar_url: Some(avatar_url.get()),
        };
        
        let user_id = user_id.clone();
        
        async move {
            let result = invoke::<_, UserProfile>("update_user_profile", &serde_json::json!({
                "user_id": user_id,
                "profile_update": profile_update
            })).await;
            
            match result {
                Ok(updated) => {
                    // Exit edit mode
                    set_editing.set(false);
                    
                    // Refresh profile
                    profile.refetch();
                    
                    Ok(())
                },
                Err(e) => Err(e.to_string())
            }
        }
    });
    
    // Get user preferences
    let preferences = create_resource(
        || user_id.clone(),
        |id| async move {
            invoke::<_, serde_json::Value>("get_user_preferences", &serde_json::json!({
                "user_id": id
            })).await.ok()
        }
    );
    
    // Get integration settings
    let integration_settings = create_resource(
        || user_id.clone(),
        |id| async move {
            invoke::<_, serde_json::Value>("get_user_integration_settings", &serde_json::json!({
                "user_id": id
            })).await.ok()
        }
    );
    
    view! {
        <div class="profile-page">
            <h1>"User Profile"</h1>
            
            <Suspense fallback=move || view! { <p>"Loading profile..."</p> }>
                {move || profile.get().map(|maybe_profile| match maybe_profile {
                    Some(profile_data) => view! {
                        <div class="profile-card">
                            <div class="profile-header">
                                <div class="avatar">
                                    <img 
                                        src={if !profile_data.avatar_url.is_empty() { profile_data.avatar_url } else { "/default-avatar.png".to_string() }}
                                        alt="Profile avatar"
                                    />
                                </div>
                                <div class="user-info">
                                    <h2>{format!("{} {}", profile_data.first_name, profile_data.last_name)}</h2>
                                    <p class="email">{profile_data.email}</p>
                                </div>
                            </div>
                            
                            {move || {
                                if editing.get() {
                                    view! {
                                        <div class="profile-edit">
                                            <div class="form-group">
                                                <label>"Bio"</label>
                                                <textarea
                                                    value=bio
                                                    on:input=move |ev| set_bio.set(event_target_value(&ev))
                                                ></textarea>
                                            </div>
                                            
                                            <div class="form-group">
                                                <label>"Avatar URL"</label>
                                                <input
                                                    type="text"
                                                    value=avatar_url
                                                    on:input=move |ev| set_avatar_url.set(event_target_value(&ev))
                                                />
                                            </div>
                                            
                                            <div class="buttons">
                                                <button 
                                                    class="save-btn"
                                                    on:click=move |_| update_profile.dispatch(())
                                                >"Save Changes"</button>
                                                <button 
                                                    class="cancel-btn"
                                                    on:click=move |_| set_editing.set(false)
                                                >"Cancel"</button>
                                            </div>
                                            
                                            {move || update_profile.value().map(|result| match result {
                                                Ok(_) => view! { <p class="success">"Profile updated successfully"</p> },
                                                Err(e) => view! { <p class="error">{format!("Error: {}", e)}</p> }
                                            })}
                                        </div>
                                    }
                                } else {
                                    view! {
                                        <div class="profile-view">
                                            <div class="bio">
                                                <h3>"Bio"</h3>
                                                <p>{if profile_data.bio.is_empty() { "No bio added yet".to_string() } else { profile_data.bio.clone() }}</p>
                                            </div>
                                            
                                            <button
                                                class="edit-btn"
                                                on:click=move |_| set_editing.set(true)
                                            >"Edit Profile"</button>
                                        </div>
                                    }
                                }
                            }}
                        </div>
                    },
                    None => view! { <p>"User profile not found."</p> }
                })}
            </Suspense>
            
            <h2>"Preferences"</h2>
            <Suspense fallback=move || view! { <p>"Loading preferences..."</p> }>
                {move || preferences.get().map(|maybe_prefs| match maybe_prefs {
                    Some(prefs) => view! {
                        <div class="preferences-card">
                            <pre>{serde_json::to_string_pretty(&prefs).unwrap_or_default()}</pre>
                        </div>
                    },
                    None => view! { <p>"No preferences found."</p> }
                })}
            </Suspense>
            
            <h2>"Integration Settings"</h2>
            <Suspense fallback=move || view! { <p>"Loading integration settings..."</p> }>
                {move || integration_settings.get().map(|maybe_settings| match maybe_settings {
                    Some(settings) => view! {
                        <div class="settings-card">
                            <pre>{serde_json::to_string_pretty(&settings).unwrap_or_default()}</pre>
                        </div>
                    },
                    None => view! { <p>"No integration settings found."</p> }
                })}
            </Suspense>
        </div>
    }
}
```

```rust
.invoke_handler(tauri::generate_handler![
    // ...other commands
    api::users::get_user_profile,
    api::users::update_user_profile,
    api::users::get_user_preferences,
    api::users::update_user_preferences,
    api::users::get_user_integration_settings,
    api::users::update_user_integration_settings,
])
```