use leptos::*;
use crate::models::user::BookmarkedTopic;
use crate::services::user::UserService;
use crate::utils::auth::AuthState;
use web_sys::SubmitEvent;

#[component]
pub fn BookmarkButton(
    #[prop(into)] topic_id: i64,
    #[prop(optional, into)] post_id: Option<i64>,
) -> impl IntoView {
    // Get auth state
    let auth_state = use_context::<AuthState>();
    let is_logged_in = move || auth_state.map(|s| s.is_authenticated()).unwrap_or(false);
    let current_user_id = move || auth_state.map(|s| s.user_id()).unwrap_or(0);
    
    // State signals
    let (is_bookmarked, set_is_bookmarked) = create_signal(false);
    let (loading, set_loading) = create_signal(true);
    let (bookmark, set_bookmark) = create_signal(None::<BookmarkedTopic>);
    
    // Modal state
    let (show_modal, set_show_modal) = create_signal(false);
    let (bookmark_note, set_bookmark_note) = create_signal(String::new());
    let (saving, set_saving) = create_signal(false);
    
    // Check if topic is bookmarked
    create_effect(move |_| {
        if !is_logged_in() {
            set_loading.set(false);
            return;
        }
        
        let user_id = current_user_id();
        let topic = topic_id;
        
        spawn_local(async move {
            match UserService::get_bookmarks(user_id).await {
                Ok(bookmarks) => {
                    let found = bookmarks.iter()
                        .find(|b| b.topic_id == topic && b.post_id == post_id)
                        .cloned();
                    
                    if let Some(found_bookmark) = found {
                        set_is_bookmarked.set(true);
                        set_bookmark.set(Some(found_bookmark));
                        if let Some(note) = &found_bookmark.note {
                            set_bookmark_note.set(note.clone());
                        }
                    } else {
                        set_is_bookmarked.set(false);
                    }
                    set_loading.set(false);
                },
                Err(_) => {
                    set_is_bookmarked.set(false);
                    set_loading.set(false);
                }
            }
        });
    });
    
    // Toggle bookmark
    let toggle_bookmark = move |_| {
        if !is_logged_in() {
            // Redirect to login or show login modal
            return;
        }
        
        if is_bookmarked() {
            // Remove bookmark
            if let Some(bm) = bookmark() {
                let user_id = current_user_id();
                let bookmark_id = bm.id;
                
                set_loading.set(true);
                
                spawn_local(async move {
                    match UserService::remove_bookmark(user_id, bookmark_id).await {
                        Ok(_) => {
                            set_is_bookmarked.set(false);
                            set_bookmark.set(None);
                            set_bookmark_note.set(String::new());
                        },
                        Err(_) => {}
                    }
                    set_loading.set(false);
                });
            }
        } else {
            // Show bookmark modal
            set_show_modal.set(true);
        }
    };
    
    // Add bookmark
    let add_bookmark = move |ev: SubmitEvent| {
        ev.prevent_default();
        
        let user_id = current_user_id();
        let topic = topic_id;
        let post = post_id;
        let note = if bookmark_note().is_empty() { None } else { Some(bookmark_note()) };
        
        set_saving.set(true);
        
        spawn_local(async move {
            match UserService::add_bookmark(user_id, topic, post, note).await {
                Ok(new_bookmark) => {
                    set_is_bookmarked.set(true);
                    set_bookmark.set(Some(new_bookmark));
                    set_show_modal.set(false);
                },
                Err(_) => {}
            }
            set_saving.set(false);
        });
    };
    
    // Close modal
    let close_modal = move |_| {
        set_show_modal.set(false);
    };

    view! {
        <>
            <button
                class="btn btn-sm"
                class:btn-outline-primary=move || !is_bookmarked()
                class:btn-primary=move || is_bookmarked()
                disabled=move || loading()
                on:click=toggle_bookmark
                title=move || if is_bookmarked() { "Remove bookmark" } else { "Add bookmark" }
            >
                {move || if loading() {
                    view! { <span class="spinner-border spinner-border-sm" role="status"></span> }
                } else if is_bookmarked() {
                    view! { <><i class="bi bi-bookmark-fill me-1"></i> "Bookmarked"</> }
                } else {
                    view! { <><i class="bi bi-bookmark me-1"></i> "Bookmark"</> }
                }}
            </button>

            // Bookmark Modal
            <div class="modal fade" class:show=move || show_modal() tabindex="-1" aria-hidden="true"
                 style:display=move || if show_modal() { "block" } else { "none" }>
                <div class="modal-dialog">
                    <div class="modal-content">
                        <form on:submit=add_bookmark>
                            <div class="modal-header">
                                <h5 class="modal-title">"Add Bookmark"</h5>
                                <button type="button" class="btn-close" on:click=close_modal></button>
                            </div>
                            <div class="modal-body">
                                <div class="mb-3">
                                    <label for="bookmarkNote" class="form-label">"Note (optional)"</label>
                                    <textarea
                                        id="bookmarkNote"
                                        class="form-control"
                                        rows="3"
                                        placeholder="Add a note to your bookmark..."
                                        prop:value=move || bookmark_note()
                                        on:input=move |ev| set_bookmark_note.set(event_target_value(&ev))
                                    ></textarea>
                                </div>
                            </div>
                            <div class="modal-footer">
                                <button type="button" class="btn btn-secondary" on:click=close_modal>
                                    "Cancel"
                                </button>
                                <button 
                                    type="submit" 
                                    class="btn btn-primary"
                                    disabled=move || saving()
                                >
                                    {move || if saving() {
                                        view! { <span class="spinner-border spinner-border-sm me-2" role="status"></span> "Saving..." }
                                    } else {
                                        view! { "Save Bookmark" }
                                    }}
                                </button>
                            </div>
                        </form>
                    </div>
                </div>
            </div>
            {move || if show_modal() {
                view! { <div class="modal-backdrop fade show"></div> }
            } else {
                view! {}
            }}
        </>
    }
}