use leptos::*;
use wasm_bindgen::prelude::*;
use web_sys::{IntersectionObserver, IntersectionObserverEntry, IntersectionObserverInit};
use std::rc::Rc;

/// Props for the LazyMedia component
#[derive(Props, Clone)]
pub struct LazyMediaProps {
    /// The URL of the media to load
    pub src: String,
    
    /// The type of media (image, audio, video)
    #[prop(default = "image".to_string())]
    pub media_type: String,
    
    /// Alternative text for images
    #[prop(default = "".to_string())]
    pub alt: String,
    
    /// CSS classes to apply to the media element
    #[prop(default = "".to_string())]
    pub class: String,
    
    /// Width of the media element
    #[prop(default = None)]
    pub width: Option<String>,
    
    /// Height of the media element
    #[prop(default = None)]
    pub height: Option<String>,
    
    /// Placeholder to show while loading
    #[prop(default = None)]
    pub placeholder: Option<String>,
    
    /// Whether to load the media immediately
    #[prop(default = false)]
    pub immediate: bool,
    
    /// Callback when media is loaded
    #[prop(default = None)]
    pub on_load: Option<Callback<()>>,
    
    /// Callback when media fails to load
    #[prop(default = None)]
    pub on_error: Option<Callback<String>>,
}

/// A component that lazily loads media when it enters the viewport
#[component]
pub fn LazyMedia(props: LazyMediaProps) -> impl IntoView {
    let LazyMediaProps {
        src,
        media_type,
        alt,
        class,
        width,
        height,
        placeholder,
        immediate,
        on_load,
        on_error,
    } = props;
    
    // State for tracking loading status
    let (loaded, set_loaded) = create_signal(false);
    let (loading, set_loading) = create_signal(false);
    let (error, set_error) = create_signal(false);
    let (visible, set_visible) = create_signal(immediate);
    
    // Reference to the container element
    let container_ref = create_node_ref::<html::Div>();
    
    // Create an intersection observer to detect when the element is visible
    create_effect(move |_| {
        // Skip if we're already loading or loaded, or if immediate loading is requested
        if loading.get() || loaded.get() || immediate {
            return;
        }
        
        // Get the container element
        let container = container_ref.get();
        if container.is_none() {
            return;
        }
        
        let container = container.unwrap();
        
        // Create the intersection observer
        let callback = Closure::wrap(Box::new(move |entries: js_sys::Array, _observer: IntersectionObserver| {
            let entry = entries.get(0);
            let entry: IntersectionObserverEntry = entry.dyn_into().unwrap();
            
            if entry.is_intersecting() {
                set_visible.set(true);
            }
        }) as Box<dyn FnMut(js_sys::Array, IntersectionObserver)>);
        
        let options = IntersectionObserverInit::new();
        options.root_margin("100px"); // Start loading a bit before it's visible
        options.threshold(0.01); // Trigger when 1% of the element is visible
        
        let observer = IntersectionObserver::new_with_options(
            callback.as_ref().unchecked_ref(),
            &options,
        ).unwrap();
        
        observer.observe(&container);
        
        // Keep the callback alive
        callback.forget();
    });
    
    // Load the media when it becomes visible
    create_effect(move |_| {
        if visible.get() && !loading.get() && !loaded.get() {
            set_loading.set(true);
            
            // Different loading strategies based on media type
            match media_type.as_str() {
                "image" => {
                    let image = web_sys::HtmlImageElement::new().unwrap();
                    let src_clone = src.clone();
                    let on_load_clone = on_load.clone();
                    let on_error_clone = on_error.clone();
                    
                    let load_callback = Closure::wrap(Box::new(move || {
                        set_loaded.set(true);
                        set_loading.set(false);
                        if let Some(callback) = on_load_clone.clone() {
                            callback.call(());
                        }
                    }) as Box<dyn FnMut()>);
                    
                    let error_callback = Closure::wrap(Box::new(move || {
                        set_error.set(true);
                        set_loading.set(false);
                        if let Some(callback) = on_error_clone.clone() {
                            callback.call(format!("Failed to load image: {}", src_clone));
                        }
                    }) as Box<dyn FnMut()>);
                    
                    image.set_onload(Some(load_callback.as_ref().unchecked_ref()));
                    image.set_onerror(Some(error_callback.as_ref().unchecked_ref()));
                    image.set_src(&src);
                    
                    // Keep the callbacks alive
                    load_callback.forget();
                    error_callback.forget();
                },
                "audio" | "video" => {
                    // For audio/video, we'll just set loaded to true and let the browser handle loading
                    set_loaded.set(true);
                    set_loading.set(false);
                    if let Some(callback) = on_load {
                        callback.call(());
                    }
                },
                _ => {
                    // Unknown media type
                    set_error.set(true);
                    set_loading.set(false);
                    if let Some(callback) = on_error {
                        callback.call(format!("Unknown media type: {}", media_type));
                    }
                }
            }
        }
    });
    
    // Render the appropriate element based on media type and loading state
    view! {
        <div
            class=format!("lazy-media-container {}", class)
            style=move || {
                if let (Some(w), Some(h)) = (width.clone(), height.clone()) {
                    format!("width: {}; height: {};", w, h)
                } else if let Some(w) = width.clone() {
                    format!("width: {};", w)
                } else if let Some(h) = height.clone() {
                    format!("height: {};", h)
                } else {
                    "".to_string()
                }
            }
            _ref=container_ref
        >
            {move || {
                if error.get() {
                    view! {
                        <div class="lazy-media-error">
                            <span class="error-icon">⚠️</span>
                            <span class="error-text">Failed to load media</span>
                        </div>
                    }.into_view()
                } else if !loaded.get() {
                    view! {
                        <div class="lazy-media-placeholder">
                            {placeholder.clone().unwrap_or_else(|| "".to_string())}
                            {move || {
                                if loading.get() {
                                    view! {
                                        <div class="lazy-media-loading-spinner">
                                            <div class="spinner"></div>
                                        </div>
                                    }.into_view()
                                } else {
                                    view! { <div></div> }.into_view()
                                }
                            }}
                        </div>
                    }.into_view()
                } else {
                    match media_type.as_str() {
                        "image" => {
                            view! {
                                <img
                                    src=src.clone()
                                    alt=alt.clone()
                                    class="lazy-media-image"
                                />
                            }.into_view()
                        },
                        "audio" => {
                            view! {
                                <audio
                                    src=src.clone()
                                    controls=true
                                    class="lazy-media-audio"
                                />
                            }.into_view()
                        },
                        "video" => {
                            view! {
                                <video
                                    src=src.clone()
                                    controls=true
                                    class="lazy-media-video"
                                />
                            }.into_view()
                        },
                        _ => {
                            view! {
                                <div class="lazy-media-unknown">
                                    <span>Unsupported media type</span>
                                </div>
                            }.into_view()
                        }
                    }
                }
            }}
        </div>
    }
}

/// A component that lazily loads an image when it enters the viewport
#[component]
pub fn LazyImage(
    /// The URL of the image to load
    src: String,
    
    /// Alternative text for the image
    #[prop(default = "".to_string())]
    alt: String,
    
    /// CSS classes to apply to the image
    #[prop(default = "".to_string())]
    class: String,
    
    /// Width of the image
    #[prop(default = None)]
    width: Option<String>,
    
    /// Height of the image
    #[prop(default = None)]
    height: Option<String>,
    
    /// Placeholder to show while loading
    #[prop(default = None)]
    placeholder: Option<String>,
    
    /// Whether to load the image immediately
    #[prop(default = false)]
    immediate: bool,
    
    /// Callback when image is loaded
    #[prop(default = None)]
    on_load: Option<Callback<()>>,
    
    /// Callback when image fails to load
    #[prop(default = None)]
    on_error: Option<Callback<String>>,
) -> impl IntoView {
    view! {
        <LazyMedia
            src=src
            media_type="image".to_string()
            alt=alt
            class=class
            width=width
            height=height
            placeholder=placeholder
            immediate=immediate
            on_load=on_load
            on_error=on_error
        />
    }
}

/// A component that lazily loads audio when it enters the viewport
#[component]
pub fn LazyAudio(
    /// The URL of the audio to load
    src: String,
    
    /// CSS classes to apply to the audio element
    #[prop(default = "".to_string())]
    class: String,
    
    /// Placeholder to show while loading
    #[prop(default = None)]
    placeholder: Option<String>,
    
    /// Whether to load the audio immediately
    #[prop(default = false)]
    immediate: bool,
    
    /// Callback when audio is loaded
    #[prop(default = None)]
    on_load: Option<Callback<()>>,
    
    /// Callback when audio fails to load
    #[prop(default = None)]
    on_error: Option<Callback<String>>,
) -> impl IntoView {
    view! {
        <LazyMedia
            src=src
            media_type="audio".to_string()
            class=class
            placeholder=placeholder
            immediate=immediate
            on_load=on_load
            on_error=on_error
        />
    }
}

/// A component that lazily loads video when it enters the viewport
#[component]
pub fn LazyVideo(
    /// The URL of the video to load
    src: String,
    
    /// CSS classes to apply to the video element
    #[prop(default = "".to_string())]
    class: String,
    
    /// Width of the video
    #[prop(default = None)]
    width: Option<String>,
    
    /// Height of the video
    #[prop(default = None)]
    height: Option<String>,
    
    /// Placeholder to show while loading
    #[prop(default = None)]
    placeholder: Option<String>,
    
    /// Whether to load the video immediately
    #[prop(default = false)]
    immediate: bool,
    
    /// Callback when video is loaded
    #[prop(default = None)]
    on_load: Option<Callback<()>>,
    
    /// Callback when video fails to load
    #[prop(default = None)]
    on_error: Option<Callback<String>>,
) -> impl IntoView {
    view! {
        <LazyMedia
            src=src
            media_type="video".to_string()
            class=class
            width=width
            height=height
            placeholder=placeholder
            immediate=immediate
            on_load=on_load
            on_error=on_error
        />
    }
}
