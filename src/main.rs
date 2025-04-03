mod app;
mod components;
mod lms;
mod forum;
mod utils;

use app::App;
use leptos::*;

pub fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|cx| view! { cx, <App/> })
}
