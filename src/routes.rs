use leptos::*;
use leptos_router::*;
use crate::components::forum::{
    topics_list::TopicsList,
    topic_edit::TopicEdit,
    topic_view::TopicView,
};
// Import other components as needed

#[component]
pub fn AppRoutes(cx: Scope) -> impl IntoView {
    view! { cx,
        <Routes>
            // Other routes
            <Route path="/forum/topics" view=|cx| view! { cx, <TopicsList/> }/>
            <Route path="/forum/topics/new" view=|cx| view! { cx, <TopicEdit/> }/>
            <Route path="/forum/topics/:id" view=|cx| view! { cx, <TopicView/> }/>
            <Route path="/forum/topics/:id/edit" view=|cx| view! { cx, <TopicEdit/> }/>
            // Categories and other routes
        </Routes>
    }
}

// Add User Profile Route to your routes

// User Profile Route
Route::new()
    .path("/users/:username")
    .view(|cx| view! { cx, <UserProfilePage /> })
    .child(
        Route::new()
            .path("/activity")
            .view(|cx| view! { cx, <UserProfilePage active_tab="activity" /> })
    )
    .child(
        Route::new()
            .path("/topics")
            .view(|cx| view! { cx, <UserProfilePage active_tab="topics" /> })
    )
    .child(
        Route::new()
            .path("/replies")
            .view(|cx| view! { cx, <UserProfilePage active_tab="replies" /> })
    )