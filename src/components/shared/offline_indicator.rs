use leptos::*;
use crate::utils::offline;

#[component]
pub fn OfflineIndicator(cx: Scope) -> impl IntoView {
    let (is_online, set_is_online) = create_signal(cx, offline::is_online());
    
    // Register for online/offline events
    create_effect(cx, move |_| {
        offline::register_online_status_listener(move |status| {
            set_is_online.set(status);
        });
    });
    
    view! { cx,
        <div class="offline-indicator" class:offline={move || !is_online.get()}>
            {move || if is_online.get() {
                view! { cx, <span class="online-status">"Online"</span> }
            } else {
                view! { cx, <span class="offline-status">"Offline (Changes will sync when online)"</span> }
            }}
        </div>
    }
}