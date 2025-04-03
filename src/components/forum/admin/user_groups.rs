use leptos::*;
use crate::models::admin::{UserGroup, UserGroupCreate, UserGroupUpdate, GroupMember};
use crate::services::admin::AdminService;
use web_sys::SubmitEvent;

#[component]
pub fn UserGroups() -> impl IntoView {
    // Admin permission check
    let auth_state = use_context::<AuthState>();
    let is_admin = move || auth_state.map(|s| s.is_admin()).unwrap_or(false);
    
    // State signals
    let (groups, set_groups) = create_signal(Vec::<UserGroup>::new());
    let (loading, set_loading) = create_signal(true);
    let (error, set_error) = create_signal(None::<String>);
    let (success, set_success) = create_signal(None::<String>);
    
    // Selected group for editing/viewing
    let (selected_group, set_selected_group) = create_signal(None::<UserGroup>);
    let (is_new_group, set_is_new_group) = create_signal(false);
    let (members, set_members) = create_signal(Vec::<GroupMember>::new());
    let (members_loading, set_members_loading) = create_signal(false);
    
    // Form signals
    let (form_name, set_form_name) = create_signal(String::new());
    let (form_description, set_form_description) = create_signal(String::new());
    let (form_color, set_form_color) = create_signal("#6c757d".to_string());
    let (form_icon, set_form_icon) = create_signal("people".to_string());
    let (form_is_visible, set_form_is_visible) = create_signal(true);
    let (form_is_public, set_form_is_public) = create_signal(false);
    let (form_can_self_assign, set_form_can_self_assign) = create_signal(false);
    
    // Search user for adding to group
    let (search_user, set_search_user) = create_signal(String::new());
    let (search_results, set_search_results) = create_signal(Vec::<crate::models::user::User>::new());
    let (searching, set_searching) = create_signal(false);
    
    // Load all groups
    let load_groups = move || {
        set_loading.set(true);
        set_error.set(None);
        
        spawn_local(async move {
            match AdminService::get_user_groups().await {
                Ok(loaded_groups) => {
                    set_groups.set(loaded_groups);
                    set_loading.set(false);
                },
                Err(e) => {
                    set_error.set(Some(format!("Failed to load user groups: {}", e)));
                    set_loading.set(false);
                }
            }
        });
    };
    
    // Initial load
    create_effect(move |_| {
        if is_admin() {
            load_groups();
        } else {
            set_loading.set(false);
        }
    });
    
    // Load group members when a group is selected
    let load_group_members = move |group_id: i64| {
        set_members_loading.set(true);
        
        spawn_local(async move {
            match AdminService::get_group_members(group_id).await {
                Ok(loaded_members) => {
                    set_members.set(loaded_members);
                    set_members_loading.set(false);
                },
                Err(e) => {
                    set_error.set(Some(format!("Failed to load group members: {}", e)));
                    set_members_loading.set(false);
                }
            }
        });
    };
    
    // Select group for editing
    let select_group = move |group: UserGroup| {
        set_form_name.set(group.name.clone());
        set_form_description.set(group.description.clone().unwrap_or_default());
        set_form_color.set(group.color.clone().unwrap_or_else(|| "#6c757d".to_string()));
        set_form_icon.set(group.icon.clone().unwrap_or_else(|| "people".to_string()));
        set_form_is_visible.set(group.is_visible);
        set_form_is_public.set(group.is_public);
        set_form_can_self_assign.set(group.can_self_assign);
        
        set_selected_group.set(Some(group.clone()));
        set_is_new_group.set(false);
        
        // Load group members
        load_group_members(group.id);
    };
    
    // Create new group
    let new_group = move |_| {
        set_form_name.set(String::new());
        set_form_description.set(String::new());
        set_form_color.set("#6c757d".to_string());
        set_form_icon.set("people".to_string());
        set_form_is_visible.set(true);
        set_form_is_public.set(false);
        set_form_can_self_assign.set(false);
        
        set_selected_group.set(None);
        set_is_new_group.set(true);
        set_members.set(Vec::new());
    };
    
    // Cancel editing
    let cancel_edit = move |_| {
        set_selected_group.set(None);
        set_is_new_group.set(false);
    };
    
    // Save group
    let save_group = move |ev: SubmitEvent| {
        ev.prevent_default();
        
        let saving_indicator = create_signal(true);
        set_error.set(None);
        set_success.set(None);
        
        if is_new_group() {
            // Create new group
            let new_group = UserGroupCreate {
                name: form_name(),
                description: if form_description().is_empty() { None } else { Some(form_description()) },
                color: if form_color() == "#6c757d" { None } else { Some(form_color()) },
                icon: if form_icon() == "people" { None } else { Some(form_icon()) },
                is_visible: form_is_visible(),
                is_public: form_is_public(),
                can_self_assign: form_can_self_assign(),
            };
            
            spawn_local(async move {
                match AdminService::create_user_group(new_group).await {
                    Ok(group) => {
                        set_success.set(Some("Group created successfully".to_string()));
                        set_selected_group.set(Some(group));
                        set_is_new_group.set(false);
                        load_groups();
                    },
                    Err(e) => {
                        set_error.set(Some(format!("Failed to create group: {}", e)));
                    }
                }
                saving_indicator.1.set(false);
            });
        } else if let Some(group) = selected_group() {
            // Update existing group
            let update_group = UserGroupUpdate {
                name: form_name(),
                description: if form_description().is_empty() { None } else { Some(form_description()) },
                color: if form_color() == "#6c757d" { None } else { Some(form_color()) },
                icon: if form_icon() == "people" { None } else { Some(form_icon()) },
                is_visible: form_is_visible(),
                is_public: form_is_public(),
                can_self_assign: form_can_self_assign(),
            };
            
            let group_id = group.id;
            
            spawn_local(async move {
                match AdminService::update_user_group(group_id, update_group).await {
                    Ok(updated_group) => {
                        set_success.set(Some("Group updated successfully".to_string()));
                        set_selected_group.set(Some(updated_group));
                        load_groups();
                    },
                    Err(e) => {
                        set_error.set(Some(format!("Failed to update group: {}", e)));
                    }
                }
                saving_indicator.1.set(false);
            });
        }
    };
    
    // Delete group
    let delete_group = move |group: UserGroup| {
        if !window().confirm_with_message(&format!("Are you sure you want to delete the group \"{}\"? This action cannot be undone.", group.name))
            .unwrap_or(false) {
            return;
        }
        
        let group_id = group.id;
        
        spawn_local(async move {
            match AdminService::delete_user_group(group_id).await {
                Ok(_) => {
                    set_success.set(Some("Group deleted successfully".to_string()));
                    set_selected_group.set(None);
                    load_groups();
                },
                Err(e) => {
                    set_error.set(Some(format!("Failed to delete group: {}", e)));
                }
            }
        });
    };
    
    // Search users to add to group
    let search_users = move |_| {
        if search_user().trim().is_empty() {
            return;
        }
        
        set_searching.set(true);
        set_search_results.set(Vec::new());
        
        spawn_local(async move {
            match AdminService::search_users(search_user()).await {
                Ok(users) => {
                    set_search_results.set(users);
                    set_searching.set(false);
                },
                Err(e) => {
                    set_error.set(Some(format!("Failed to search users: {}", e)));
                    set_searching.set(false);
                }
            }
        });
    };
    
    // Add user to group
    let add_user_to_group = move |user_id: i64| {
        if let Some(group) = selected_group() {
            spawn_local(async move {
                match AdminService::add_user_to_group(group.id, user_id).await {
                    Ok(_) => {
                        set_success.set(Some("User added to group".to_string()));
                        set_search_user.set(String::new());
                        set_search_results.set(Vec::new());
                        load_group_members(group.id);
                    },
                    Err(e) => {
                        set_error.set(Some(format!("Failed to add user to group: {}", e)));
                    }
                }
            });
        }
    };
    
    // Remove user from group
    let remove_user_from_group = move |member: GroupMember| {
        if let Some(group) = selected_group() {
            let group_id = group.id;
            let user_id = member.user_id;
            
            spawn_local(async move {
                match AdminService::remove_user_from_group(group_id, user_id).await {
                    Ok(_) => {
                        set_success.set(Some("User removed from group".to_string()));
                        load_group_members(group_id);
                    },
                    Err(e) => {
                        set_error.set(Some(format!("Failed to remove user from group: {}", e)));
                    }
                }
            });
        }
    };

    view! {
        <div class="user-groups">
            {move || if !is_admin() {
                view! {
                    <div class="alert alert-danger">
                        "You don't have permission to access this page."
                    </div>
                }
            } else {
                view! {
                    <div>
                        <div class="d-flex justify-content-between align-items-center mb-4">
                            <h1 class="mb-0">"User Groups"</h1>
                            <button class="btn btn-primary" on:click=new_group>
                                <i class="bi bi-plus-circle me-1"></i>
                                "New Group"
                            </button>
                        </div>
                        
                        {move || error().map(|err| view! {
                            <div class="alert alert-danger mb-4">{err}</div>
                        })}
                        
                        {move || success().map(|msg| view! {
                            <div class="alert alert-success mb-4">{msg}</div>
                        })}
                        
                        <div class="row">
                            <div class="col-md-4">
                                {move || if loading() {
                                    view! { <div class="d-flex justify-content-center p-5"><div class="spinner-border" role="status"></div></div> }
                                } else if groups().is_empty() && !is_new_group() {
                                    view! {
                                        <div class="card">
                                            <div class="card-body text-center p-5">
                                                <i class="bi bi-people mb-3 d-block" style="font-size: 3rem;"></i>
                                                <h4>"No User Groups Found"</h4>
                                                <p class="text-muted">
                                                    "Create your first user group to organize your community."
                                                </p>
                                                <button class="btn btn-primary mt-3" on:click=new_group>
                                                    <i class="bi bi-plus-circle me-1"></i>
                                                    "New Group"
                                                </button>
                                            </div>
                                        </div>
                                    }
                                } else {
                                    view! {
                                        <div class="card mb-4">
                                            <div class="card-header">
                                                <h5 class="mb-0">"User Groups"</h5>
                                            </div>
                                            <div class="list-group list-group-flush">
                                                {groups().into_iter().map(|group| {
                                                    let group_for_select = group.clone();
                                                    let group_for_delete = group.clone();
                                                    
                                                    view! {
                                                        <div
                                                            class="list-group-item list-group-item-action d-flex justify-content-between align-items-center"
                                                            class:active=move || selected_group().map(|g| g.id == group.id).unwrap_or(false)
                                                            on:click=move |_| select_group(group_for_select.clone())
                                                        >
                                                            <div class="d-flex align-items-center">
                                                                <span 
                                                                    class="group-icon me-2" 
                                                                    style=format!(
                                                                        "background-color: {}; color: white; padding: 6px; border-radius: 50%; display: inline-flex; align-items: center; justify-content: center; width: 32px; height: 32px;", 
                                                                        group.color.clone().unwrap_or_else(|| "#6c757d".to_string())
                                                                    )
                                                                >
                                                                    <i class=format!("bi bi-{}", group.icon.unwrap_or_else(|| "people".to_string()))></i>
                                                                </span>
                                                                <div>
                                                                    <strong>{&group.name}</strong>
                                                                    {group.description.filter(|d| !d.is_empty()).map(|desc| {
                                                                        view! { <div class="small text-muted">{desc}</div> }
                                                                    })}
                                                                </div>
                                                            </div>
                                                            <button 
                                                                class="btn btn-sm btn-outline-danger"
                                                                on:click=move |ev| {
                                                                    ev.stop_propagation();
                                                                    delete_group(group_for_delete.clone());
                                                                }
                                                            >
                                                                <i class="bi bi-trash"></i>
                                                            </button>
                                                        </div>
                                                    }
                                                }).collect::<Vec<_>>()}
                                            </div>
                                        </div>
                                    }
                                }}
                            </div>
                            
                            <div class="col-md-8">
                                {move || if is_new_group() || selected_group().is_some() {
                                    view! {
                                        <div class="card mb-4">
                                            <div class="card-header">
                                                <h5 class="mb-0">
                                                    {if is_new_group() {
                                                        "New User Group" 
                                                    } else {
                                                        "Edit User Group"
                                                    }}
                                                </h5>
                                            </div>
                                            <div class="card-body">
                                                <form on:submit=save_group>
                                                    <div class="mb-3">
                                                        <label for="groupName" class="form-label">"Group Name"</label>
                                                        <input
                                                            id="groupName"
                                                            type="text"
                                                            class="form-control"
                                                            prop:value=move || form_name()
                                                            on:input=move |ev| set_form_name.set(event_target_value(&ev))
                                                            required
                                                        />
                                                    </div>
                                                    
                                                    <div class="mb-3">
                                                        <label for="groupDescription" class="form-label">"Description"</label>
                                                        <textarea
                                                            id="groupDescription"
                                                            class="form-control"
                                                            rows="3"
                                                            prop:value=move || form_description()
                                                            on:input=move |ev| set_form_description.set(event_target_value(&ev))
                                                            placeholder="Optional group description"
                                                        ></textarea>
                                                    </div>
                                                    
                                                    <div class="row mb-3">
                                                        <div class="col-md-6">
                                                            <label for="groupColor" class="form-label">"Group Color"</label>
                                                            <div class="input-group">
                                                                <input
                                                                    id="groupColor"
                                                                    type="color"
                                                                    class="form-control form-control-color"
                                                                    prop:value=move || form_color()
                                                                    on:input=move |ev| set_form_color.set(event_target_value(&ev))
                                                                />
                                                                <input
                                                                    type="text"
                                                                    class="form-control"
                                                                    prop:value=move || form_color()
                                                                    on:input=move |ev| set_form_color.set(event_target_value(&ev))
                                                                />
                                                            </div>
                                                        </div>
                                                        <div class="col-md-6">
                                                            <label for="groupIcon" class="form-label">"Group Icon"</label>
                                                            <input
                                                                id="groupIcon"
                                                                type="text"
                                                                class="form-control"
                                                                prop:value=move || form_icon()
                                                                on:input=move |ev| set_form_icon.set(event_target_value(&ev))
                                                                placeholder="Bootstrap icon name (e.g., people)"
                                                            />
                                                            <div class="form-text">
                                                                "Uses Bootstrap Icons (e.g., people, shield, star)"
                                                            </div>
                                                        </div>
                                                    </div>
                                                    
                                                    <div class="mb-3">
                                                        <div class="form-check form-switch mb-2">
                                                            <input
                                                                class="form-check-input"
                                                                type="checkbox"
                                                                id="groupVisible"
                                                                prop:checked=move || form_is_visible()
                                                                on:change=move |ev| set_form_is_visible.set(event_target_checked(&ev))
                                                            />
                                                            <label class="form-check-label" for="groupVisible">
                                                                "Visible Group"
                                                            </label>
                                                            <div class="form-text">
                                                                "If enabled, this group will be visible in member profiles and group listings"
                                                            </div>
                                                        </div>
                                                        
                                                        <div class="form-check form-switch mb-2">
                                                            <input
                                                                class="form-check-input"
                                                                type="checkbox"
                                                                id="groupPublic"
                                                                prop:checked=move || form_is_public()
                                                                on:change=move |ev| set_form_is_public.set(event_target_checked(&ev))
                                                            />
                                                            <label class="form-check-label" for="groupPublic">
                                                                "Public Group"
                                                            </label>
                                                            <div class="form-text">
                                                                "If enabled, anyone can view the members of this group"
                                                            </div>
                                                        </div>
                                                        
                                                        <div class="form-check form-switch mb-2">
                                                            <input
                                                                class="form-check-input"
                                                                type="checkbox"
                                                                id="groupSelfAssign"
                                                                prop:checked=move || form_can_self_assign()
                                                                on:change=move |ev| set_form_can_self_assign.set(event_target_checked(&ev))
                                                            />
                                                            <label class="form-check-label" for="groupSelfAssign">
                                                                "Allow Self-Assign"
                                                            </label>
                                                            <div class="form-text">
                                                                "If enabled, users can join this group without admin approval"
                                                            </div>
                                                        </div>
                                                    </div>
                                                    
                                                    <div class="d-flex justify-content-end gap-2 mt-4">
                                                        <button 
                                                            type="button" 
                                                            class="btn btn-outline-secondary" 
                                                            on:click=cancel_edit
                                                        >
                                                            "Cancel"
                                                        </button>
                                                        <button type="submit" class="btn btn-primary">
                                                            {if is_new_group() { "Create Group" } else { "Update Group" }}
                                                        </button>
                                                    </div>
                                                </form>
                                            </div>
                                        </div>
                                        
                                        {move || if !is_new_group() && selected_group().is_some() {
                                            view! {
                                                <div class="card">
                                                    <div class="card-header d-flex justify-content-between align-items-center">
                                                        <h5 class="mb-0">"Group Members"</h5>
                                                        <span class="badge bg-secondary">
                                                            {format!("{} members", members().len())}
                                                        </span>
                                                    </div>
                                                    <div class="card-body">
                                                        <div class="mb-4">
                                                            <label class="form-label">"Add Members"</label>
                                                            <div class="input-group">
                                                                <input
                                                                    type="text"
                                                                    class="form-control"
                                                                    placeholder="Search by username or email"
                                                                    prop:value=move || search_user()
                                                                    on:input=move |ev| set_search_user.set(event_target_value(&ev))
                                                                />
                                                                <button
                                                                    class="btn btn-outline-secondary"
                                                                    type="button"
                                                                    on:click=search_users
                                                                    disabled=move || searching()
                                                                >
                                                                    {move || if searching() {
                                                                        view! { <span class="spinner-border spinner-border-sm" role="status"></span> }
                                                                    } else {
                                                                        view! { "Search" }
                                                                    }}
                                                                </button>
                                                            </div>
                                                            
                                                            {move || if !search_results().is_empty() {
                                                                view! {
                                                                    <div class="mt-2">
                                                                        <div class="list-group">
                                                                            {search_results().into_iter().map(|user| {
                                                                                let already_member = members().iter().any(|m| m.user_id == user.id);
                                                                                let user_id = user.id;
                                                                                
                                                                                view! {
                                                                                    <div class="list-group-item d-flex justify-content-between align-items-center">
                                                                                        <div>
                                                                                            <div>{user.username}</div>
                                                                                            <small class="text-muted">{user.email}</small>
                                                                                        </div>
                                                                                        <button
                                                                                            class="btn btn-sm btn-primary"
                                                                                            on:click=move |_| add_user_to_group(user_id)
                                                                                            disabled=already_member
                                                                                        >
                                                                                            {if already_member { "Already Member" } else { "Add to Group" }}
                                                                                        </button>
                                                                                    </div>
                                                                                }
                                                                            }).collect::<Vec<_>>()}
                                                                        </div>
                                                                    </div>
                                                                }
                                                            } else {
                                                                view! {}
                                                            }}
                                                        </div>
                                                        
                                                        <hr class="my-4"/>
                                                        
                                                        <div>
                                                            <h6 class="mb-3">"Current Members"</h6>
                                                            
                                                            {move || if members_loading() {
                                                                view! { <div class="d-flex justify-content-center p-3"><div class="spinner-border spinner-border-sm" role="status"></div></div> }
                                                            } else if members().is_empty() {
                                                                view! { <div class="text-muted text-center">"No members in this group yet"</div> }
                                                            } else {
                                                                view! {
                                                                    <div class="list-group">
                                                                        {members().into_iter().map(|member| {
                                                                            let member_for_remove = member.clone();
                                                                            
                                                                            view! {
                                                                                <div class="list-group-item d-flex justify-content-between align-items-center">
                                                                                    <div>
                                                                                        <div class="d-flex align-items-center">
                                                                                            <div class="avatar me-2">
                                                                                                {if let Some(avatar) = &member.avatar_url {
                                                                                                    view! { <img src={avatar} alt={&member.username} class="rounded-circle" width="32" height="32"/> }
                                                                                                } else {
                                                                                                    view! { <div class="avatar-placeholder">{member.username.chars().next().unwrap_or('?')}</div> }
                                                                                                }}
                                                                                            </div>
                                                                                            <div>
                                                                                                <div>{member.username}</div>
                                                                                                <small class="text-muted">
                                                                                                    {"Joined "}{format_date(member.joined_at)}
                                                                                                </small>
                                                                                            </div>
                                                                                        </div>
                                                                                    </div>
                                                                                    <button
                                                                                        class="btn btn-sm btn-outline-danger"
                                                                                        on:click=move |_| remove_user_from_group(member_for_remove.clone())
                                                                                    >
                                                                                        <i class="bi bi-person-dash"></i>
                                                                                        " Remove"
                                                                                    </button>
                                                                                </div>
                                                                            }
                                                                        }).collect::<Vec<_>>()}
                                                                    </div>
                                                                }
                                                            }}
                                                        </div>
                                                    </div>
                                                </div>
                                            }
                                        } else {
                                            view! {}
                                        }}
                                    }
                                } else {
                                    view! {
                                        <div class="card">
                                            <div class="card-body text-center p-5">
                                                <i class="bi bi-people-fill mb-3 d-block" style="font-size: 3rem;"></i>
                                                <h4>"User Group Management"</h4>
                                                <p class="text-muted">
                                                    "Create and manage user groups to organize your community members."
                                                </p>
                                                <p class="text-muted">
                                                    "Select a group from the list to edit it, or create a new group."
                                                </p>
                                                <button class="btn btn-primary mt-3" on:click=new_group>
                                                    <i class="bi bi-plus-circle me-1"></i>
                                                    "New Group"
                                                </button>
                                            </div>
                                        </div>
                                    }
                                }}
                            </div>
                        </div>
                    </div>
                }
            }}
        </div>
    }
}

fn format_date(date: chrono::DateTime<chrono::Utc>) -> String {
    // Format date as "Jan 1, 2023"
    date.format("%b %e, %Y").to_string()
}