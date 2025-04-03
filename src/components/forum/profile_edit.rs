use leptos::*;
use web_sys::SubmitEvent;
use crate::models::user::{User, UserUpdateRequest};
use crate::services::user::UserService;
use crate::components::forum::rich_editor::RichEditor;

#[component]
pub fn ProfileEdit() -> impl IntoView {
    // Get the current user
    let auth_state = use_context::<AuthState>().expect("AuthState not found");
    let current_user_id = create_memo(move |_| auth_state.user_id());
    
    // State signals
    let (user, set_user) = create_signal(None::<User>);
    let (loading, set_loading) = create_signal(true);
    let (saving, set_saving) = create_signal(false);
    let (error, set_error) = create_signal(None::<String>);
    let (success, set_success) = create_signal(None::<String>);
    
    // Form fields
    let (display_name, set_display_name) = create_signal(String::new());
    let (bio, set_bio) = create_signal(String::new());
    let (avatar_url, set_avatar_url) = create_signal(String::new());
    let (website, set_website) = create_signal(String::new());
    let (location, set_location) = create_signal(String::new());
    
    // Load user data
    create_effect(move |_| {
        let id = current_user_id();
        if id <= 0 {
            set_error.set(Some("You must be logged in to edit your profile".to_string()));
            set_loading.set(false);
            return;
        }
        
        spawn_local(async move {
            match UserService::get_user(id).await {
                Ok(user_data) => {
                    set_display_name.set(user_data.name.clone());
                    set_bio.set(user_data.bio.clone().unwrap_or_default());
                    set_avatar_url.set(user_data.avatar_url.clone().unwrap_or_default());
                    set_website.set(user_data.website.clone().unwrap_or_default());
                    set_location.set(user_data.location.clone().unwrap_or_default());
                    set_user.set(Some(user_data));
                    set_loading.set(false);
                },
                Err(e) => {
                    set_error.set(Some(format!("Failed to load user profile: {}", e)));
                    set_loading.set(false);
                }
            }
        });
    });
    
    // Handle form submission
    let handle_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        set_saving.set(true);
        set_error.set(None);
        set_success.set(None);
        
        let user_id = current_user_id();
        let update_request = UserUpdateRequest {
            name: Some(display_name()),
            bio: Some(bio()),
            avatar_url: Some(avatar_url()),
            website: Some(if website().is_empty() { None } else { Some(website()) }),
            location: Some(if location().is_empty() { None } else { Some(location()) }),
        };
        
        spawn_local(async move {
            match UserService::update_user(user_id, update_request).await {
                Ok(_) => {
                    // Refresh user data
                    match UserService::get_user(user_id).await {
                        Ok(updated_user) => {
                            set_user.set(Some(updated_user));
                            set_success.set(Some("Profile updated successfully!".to_string()));
                        },
                        Err(e) => {
                            set_error.set(Some(format!("Profile updated but failed to refresh data: {}", e)));
                        }
                    }
                    set_saving.set(false);
                },
                Err(e) => {
                    set_error.set(Some(format!("Failed to update profile: {}", e)));
                    set_saving.set(false);
                }
            }
        });
    };
    
    view! {
        <div class="profile-edit">
            <h1 class="mb-4">"Edit Profile"</h1>
            
            {move || if loading() {
                view! { <div class="d-flex justify-content-center p-5"><div class="spinner-border" role="status"></div></div> }
            } else if current_user_id() <= 0 {
                view! {
                    <div class="alert alert-warning">
                        "You need to be logged in to edit your profile. "
                        <a href="/login" class="alert-link">"Log in"</a>
                        " or "
                        <a href="/register" class="alert-link">"register"</a>
                        " to continue."
                    </div>
                }
            } else {
                view! {
                    <form on:submit=handle_submit>
                        {move || error().map(|err| view! {
                            <div class="alert alert-danger mb-4">{err}</div>
                        })}
                        
                        {move || success().map(|msg| view! {
                            <div class="alert alert-success mb-4">{msg}</div>
                        })}
                        
                        <div class="row">
                            <div class="col-md-6">
                                <div class="mb-3">
                                    <label for="displayName" class="form-label">"Display Name"</label>
                                    <input
                                        id="displayName"
                                        type="text"
                                        class="form-control"
                                        prop:value=move || display_name()
                                        on:input=move |ev| set_display_name.set(event_target_value(&ev))
                                        required
                                    />
                                </div>
                                
                                <div class="mb-3">
                                    <label for="avatarUrl" class="form-label">"Avatar URL"</label>
                                    <input
                                        id="avatarUrl"
                                        type="url"
                                        class="form-control"
                                        prop:value=move || avatar_url()
                                        on:input=move |ev| set_avatar_url.set(event_target_value(&ev))
                                        placeholder="https://example.com/avatar.png"
                                    />
                                    <div class="form-text">
                                        "Leave empty to use the default avatar."
                                    </div>
                                </div>
                                
                                <div class="mb-3">
                                    <label for="website" class="form-label">"Website"</label>
                                    <input
                                        id="website"
                                        type="url"
                                        class="form-control"
                                        prop:value=move || website()
                                        on:input=move |ev| set_website.set(event_target_value(&ev))
                                        placeholder="https://example.com"
                                    />
                                </div>
                                
                                <div class="mb-3">
                                    <label for="location" class="form-label">"Location"</label>
                                    <input
                                        id="location"
                                        type="text"
                                        class="form-control"
                                        prop:value=move || location()
                                        on:input=move |ev| set_location.set(event_target_value(&ev))
                                        placeholder="City, Country"
                                    />
                                </div>
                            </div>
                            
                            <div class="col-md-6">
                                <div class="mb-3">
                                    <label for="bio" class="form-label">"About Me"</label>
                                    <RichEditor
                                        content=bio
                                        set_content=set_bio
                                        placeholder=Some("Tell the community about yourself...")
                                        rows=Some(8)
                                    />
                                </div>
                                
                                <div class="mb-3">
                                    <div class="d-flex align-items-center">
                                        <div class="me-3">
                                            <strong>"Avatar Preview:"</strong>
                                        </div>
                                        <div class="avatar-preview">
                                            <img 
                                                src={move || {
                                                    if avatar_url().is_empty() {
                                                        format!("https://ui-avatars.com/api/?name={}&background=random", 
                                                                urlencoding::encode(&display_name()))
                                                    } else {
                                                        avatar_url()
                                                    }
                                                }}
                                                alt="Avatar Preview"
                                                class="rounded-circle"
                                                style="width: 80px; height: 80px; object-fit: cover;"
                                            />
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                        
                        <div class="d-flex justify-content-between mt-4">
                            <a href={format!("/users/{}", current_user_id())} class="btn btn-outline-secondary">
                                "Cancel"
                            </a>
                            
                            <button type="submit" class="btn btn-primary" disabled=move || saving()>
                                {move || if saving() {
                                    view! { <span class="spinner-border spinner-border-sm me-2" role="status" aria-hidden="true"></span> "Saving..." }
                                } else {
                                    view! { "Save Changes" }
                                }}
                            </button>
                        </div>
                    </form>
                }
            }}
        </div>
    }
}