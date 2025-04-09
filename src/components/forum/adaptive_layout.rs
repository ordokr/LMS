use leptos::*;
use web_sys::window;

// Viewport sizes for responsive design
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Viewport {
    Mobile,  // < 640px
    Tablet,  // 640px - 1024px
    Desktop, // > 1024px
}

// Responsive hook to track viewport size
#[hook]
pub fn use_viewport() -> Signal<Viewport> {
    let (viewport, set_viewport) = create_signal(get_current_viewport());
    
    // Update on resize
    let listener = window_event_listener(ev::resize, move |_| {
        set_viewport.set(get_current_viewport());
    });
    
    on_cleanup(move || {
        drop(listener); // Clean up event listener
    });
    
    viewport
}

// Helper to get current viewport
fn get_current_viewport() -> Viewport {
    let width = window()
        .and_then(|w| Some(w.inner_width().ok()?.as_f64()?))
        .unwrap_or(1024.0);
    
    match width as u32 {
        w if w < 640 => Viewport::Mobile,
        w if w < 1024 => Viewport::Tablet,
        _ => Viewport::Desktop,
    }
}

// Adaptive quality component for images/content
#[component]
pub fn AdaptiveForumContent(
    #[prop(into)] topic_id: i64,
) -> impl IntoView {
    let viewport = use_viewport();
    
    // Adapt content based on screen size
    let num_posts = create_memo(move |_| {
        match viewport.get() {
            Viewport::Mobile => 10,
            Viewport::Tablet => 20,
            Viewport::Desktop => 30,
        }
    });
    
    let load_images = create_memo(move |_| {
        viewport.get() != Viewport::Mobile
    });
    
    let render_sidebar = create_memo(move |_| {
        viewport.get() == Viewport::Desktop
    });
    
    // Layout class based on viewport
    let layout_class = create_memo(move |_| {
        match viewport.get() {
            Viewport::Mobile => "forum-layout-mobile",
            Viewport::Tablet => "forum-layout-tablet",
            Viewport::Desktop => "forum-layout-desktop",
        }
    });
    
    view! {
        <div class=move || format!("forum-adaptive-layout {}", layout_class.get())>
            <div class="forum-main-content">
                <TopicHeader topic_id=topic_id />
                <PostList 
                    topic_id=topic_id
                    load_count=num_posts
                    load_images=load_images
                />
            </div>
            
            {move || {
                if render_sidebar.get() {
                    view! {
                        <aside class="forum-sidebar">
                            <TopicSidebar topic_id=topic_id />
                        </aside>
                    }
                } else {
                    view! { <></> }
                }
            }}
        </div>
    }
}

// Adaptive image loading component
#[component]
pub fn AdaptiveImage(
    #[prop(into)] src: String,
    #[prop(into)] alt: String,
    #[prop(optional)] width: Option<u32>,
    #[prop(optional)] height: Option<u32>,
    #[prop(default = false)] lazy: bool,
) -> impl IntoView {
    let viewport = use_viewport();
    
    // Determine image quality based on viewport
    let image_src = create_memo(move |_| {
        let base_url = src.clone();
        match viewport.get() {
            Viewport::Mobile => {
                if base_url.contains('?') {
                    format!("{}&quality=60&width=480", base_url)
                } else {
                    format!("{}?quality=60&width=480", base_url)
                }
            },
            Viewport::Tablet => {
                if base_url.contains('?') {
                    format!("{}&quality=80&width=800", base_url)
                } else {
                    format!("{}?quality=80&width=800", base_url)
                }
            },
            Viewport::Desktop => base_url,
        }
    });
    
    // Loading attribute based on lazy flag
    let loading_attr = if lazy { "lazy" } else { "eager" };
    
    view! {
        <img 
            src=move || image_src.get()
            alt=alt
            width=width
            height=height
            loading=loading_attr
            class="adaptive-image"
        />
    }
}