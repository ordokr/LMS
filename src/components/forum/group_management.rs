use leptos::*;
use crate::models::forum::{Group, GroupMembershipLevel};
use crate::services::forum_service::ForumService;

#[component]
pub fn GroupManagement() -> impl IntoView {
    let forum_service = expect_context::<ForumService>();
    
    // State for groups
    let groups = create_resource(
        || (),
        move |_| {
            let forum_service = forum_service.clone();
            async move { forum_service.get_groups().await }
        }
    );
    
    // Selected group state
    let (selected_group_id, set_selected_group_id) = create_signal(None::<i64>);
    
    // Get details for selected group
    let selected_group = create_resource(
        move || selected_group_id.get(),
        move |id| {
            let forum_service = forum_service.clone();
            async move {
                match id {
                    Some(group_id) => forum_service.get_group(group_id).await,
                    None => Ok(None)
                }
            }
        }
    );

    // Create a new group form state
    let (show_create_form, set_show_create_form) = create_signal(false);
    let (new_group_name, set_new_group_name) = create_signal(String::new());
    let (new_group_description, set_new_group_description) = create_signal(String::new());
    
    // Create group handler
    let create_group = move |ev: web_sys::Event| {
        ev.prevent_default();
        
        let forum_service = forum_service.clone();
        let name = new_group_name.get();
        let description = new_group_description.get();
        
        spawn_local(async move {
            // This is simplified - you'd need to build a complete Group object
            let new_group = Group {
                id: -1, // Will be set by server
                name: name.clone(),
                full_name: Some(name),
                description: Some(description),
                // Fill in other fields with defaults...
                // This is just an example and would need to be completed
                bio_raw: None,
                bio_cooked: None,
                user_count: 0,
                mentionable_level: 0,
                messageable_level: 0,
                visibility_level: 0,
                automatic: false,
                automatic_membership_email_domains: None,
                automatic_membership_retroactive: false,
                primary_group: false,
                title: None,
                grant_trust_level: None,
                members_visibility_level: 0,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
                custom_fields: None,
            };
            
            match forum_service.create_group(&new_group).await {
                Ok(_) => {
                    set_show_create_form.set(false);
                    set_new_group_name.set(String::new());
                    set_new_group_description.set(String::new());
                    // Refresh the groups list
                    groups.refetch();
                }
                Err(e) => {
                    // Handle error - you would display this to the user
                    log::error!("Failed to create group: {}", e);
                }
            }
        });
    };
    
    view! {
        <div class="group-management">
            <h1>"Forum Groups Management"</h1>
            
            <div class="group-actions">
                <button 
                    class="btn primary"
                    on:click=move |_| set_show_create_form.set(!show_create_form.get())
                >
                    {move || if show_create_form.get() { "Cancel" } else { "Create New Group" }}
                </button>
            </div>
            
            // Create group form
            {move || show_create_form.get().then(|| view! {
                <form class="create-group-form" on:submit=create_group>
                    <h3>"Create New Group"</h3>
                    
                    <div class="form-group">
                        <label for="group-name">"Name:"</label>
                        <input 
                            id="group-name"
                            type="text"
                            prop:value=move || new_group_name.get()
                            on:input=move |ev| {
                                let val = event_target_value(&ev);
                                set_new_group_name.set(val);
                            }
                            required
                        />
                    </div>
                    
                    <div class="form-group">
                        <label for="group-description">"Description:"</label>
                        <textarea 
                            id="group-description"
                            prop:value=move || new_group_description.get()
                            on:input=move |ev| {
                                let val = event_target_value(&ev);
                                set_new_group_description.set(val);
                            }
                            rows="3"
                        ></textarea>
                    </div>
                    
                    <div class="form-actions">
                        <button type="submit" class="btn primary">"Create Group"</button>
                        <button type="button" class="btn" on:click=move |_| set_show_create_form.set(false)>"Cancel"</button>
                    </div>
                </form>
            })}
            
            <div class="group-list">
                <h2>"Available Groups"</h2>
                {move || match groups.get() {
                    None => view! { <p>"Loading groups..."</p> },
                    Some(Ok(group_list)) => {
                        if group_list.is_empty() {
                            view! { <p>"No groups available. Create one to get started."</p> }
                        } else {
                            view! {
                                <div class="group-cards">
                                    {group_list.into_iter().map(|group| {
                                        let group_id = group.id;
                                        let name = group.name.clone();
                                        let description = group.description.clone();
                                        let member_count = group.user_count;
                                        
                                        view! {
                                            <div 
                                                class="group-card" 
                                                class:selected=move || selected_group_id.get() == Some(group_id)
                                                on:click=move |_| set_selected_group_id.set(Some(group_id))
                                            >
                                                <h3>{name}</h3>
                                                <p class="member-count">{member_count} " members"</p>
                                                {description.map(|desc| view! { <p class="description">{desc}</p> })
                                                    .unwrap_or_else(|| view! { <p class="no-description">"No description"</p> })}
                                            </div>
                                        }
                                    }).collect::<Vec<_>>()}
                                </div>
                            }
                        }
                    },
                    Some(Err(err)) => view! { <div class="error-message">"Error loading groups: " {err.to_string()}</div> }
                }}
            </div>
            
            <div class="group-details">
                {move || match (selected_group_id.get(), selected_group.get()) {
                    (Some(_), Some(Ok(Some(group)))) => {
                        let name = group.name.clone();
                        let full_name = group.full_name.clone();
                        let description = group.description.clone();
                        let user_count = group.user_count;
                        let created_at = group.created_at;
                        let automatic = group.automatic;
                        
                        view! {
                            <div class="group-detail-panel">
                                <h2>{name}</h2>
                                
                                <div class="group-info">
                                    <div class="info-row">
                                        <span class="label">"Full name:"</span>
                                        <span class="value">{full_name.unwrap_or_else(|| "N/A".to_string())}</span>
                                    </div>
                                    
                                    <div class="info-row">
                                        <span class="label">"Description:"</span>
                                        <span class="value">{description.unwrap_or_else(|| "No description".to_string())}</span>
                                    </div>
                                    
                                    <div class="info-row">
                                        <span class="label">"Members:"</span>
                                        <span class="value">{user_count}</span>
                                    </div>
                                    
                                    <div class="info-row">
                                        <span class="label">"Created:"</span>
                                        <span class="value">{created_at.format("%B %d, %Y %H:%M").to_string()}</span>
                                    </div>
                                    
                                    <div class="info-row">
                                        <span class="label">"Type:"</span>
                                        <span class="value">{if automatic { "Automatic" } else { "Manual" }}</span>
                                    </div>
                                </div>
                                
                                <div class="actions">
                                    <button class="btn primary">"Edit Group"</button>
                                    <button class="btn">"Manage Members"</button>
                                    <button class="btn danger">"Delete Group"</button>
                                </div>
                            </div>
                        }
                    },
                    (Some(_), Some(Ok(None))) => view! { <p>"Group not found"</p> },
                    (Some(_), Some(Err(err))) => view! { <div class="error-message">"Error: " {err.to_string()}</div> },
                    (Some(_), None) => view! { <p>"Loading group details..."</p> },
                    (None, _) => view! { <p class="select-prompt">"Select a group to view details"</p> },
                }}
            </div>
        </div>
    }
}