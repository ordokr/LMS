use leptos::*;
use web_sys::HtmlImageElement;

#[component]
pub fn ProgressiveImage(
    #[prop(into)] src: String,
    #[prop(into)] alt: String,
    #[prop(optional)] thumbnail: Option<String>,
    #[prop(optional)] width: Option<u32>,
    #[prop(optional)] height: Option<u32>,
    #[prop(optional)] class: Option<String>,
) -> impl IntoView {
    let (loaded, set_loaded) = create_signal(false);
    let image_ref = create_node_ref::<html::Img>();
    
    // Handle image load
    let on_load = move |_| {
        set_loaded.set(true);
    };
    
    // Handle image error
    let on_error = move |_| {
        log::error!("Failed to load image: {}", src);
        // Optionally set a default image
    };
    
    // Initial thumbnail or blur placeholder
    let placeholder = thumbnail.unwrap_or_else(|| {
        // If no thumbnail, generate a 1x1 pixel placeholder with the correct aspect ratio
        if let (Some(w), Some(h)) = (width, height) {
            format!("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 {w} {h}'%3E%3C/svg%3E")
        } else {
            // Default placeholder
            "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 1 1'%3E%3C/svg%3E".to_string()
        }
    });

    // Calculate style
    let style = move || {
        let mut styles = vec![
            "transition: opacity 0.3s ease".to_string(),
            format!("opacity: {}", if loaded.get() { "1" } else { "0.5" }),
        ];
        
        // Add width/height if provided
        if let Some(w) = width {
            styles.push(format!("width: {}px", w));
        }
        if let Some(h) = height {
            styles.push(format!("height: {}px", h));
        }
        
        styles.join("; ")
    };

    view! {
        <div class="progressive-image-container">
            // Show thumbnail/placeholder while loading
            {move || {
                if !loaded.get() {
                    view! {
                        <img 
                            src=&placeholder
                            alt=&alt
                            class=class.clone().unwrap_or_default() + " placeholder"
                            style="position: absolute; top: 0; left: 0; width: 100%; height: 100%; filter: blur(10px); object-fit: cover;"
                        />
                    }
                } else {
                    view! { <></> }
                }
            }}
            
            // Main image (hidden until loaded)
            <img
                ref=image_ref
                src=&src
                alt=&alt
                style=style
                class=class
                width=width
                height=height
                on:load=on_load
                on:error=on_error
                loading="lazy"
            />
        </div>
    }
}