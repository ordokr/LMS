use leptos::*;
use crate::components::forum::category_list::CategoryList;

#[component]
pub fn ForumHomePage() -> impl IntoView {
    // Using OutOfOrder for fastest loading experience
    view! {
        <div class="forum-home">
            <h1>"Forum Home"</h1>
            <p>"Welcome to our community forum. Browse categories below."</p>

            <CategoryList />
        </div>
    }
}