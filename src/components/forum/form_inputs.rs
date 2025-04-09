use leptos::*;
use gloo_timers::future::TimeoutFuture;
use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, HtmlTextAreaElement, KeyboardEvent};
use std::rc::Rc;

// Generic debounce hook
#[hook]
pub fn use_debounced<T, F>(value: T, delay_ms: u32, callback: F) -> Signal<T> 
where
    T: Clone + PartialEq + 'static,
    F: Fn(T) + 'static,
{
    let (debounced, set_debounced) = create_signal(value.clone());
    
    // Use Resource for managing the debouncing
    let _debouncer = create_resource(
        move || value.clone(),
        move |new_value| {
            let new_value_clone = new_value.clone();
            let callback = callback.clone();
            let set_debounced = set_debounced.clone();
            
            async move {
                TimeoutFuture::new(delay_ms).await;
                set_debounced.set(new_value_clone.clone());
                callback(new_value_clone);
            }
        }
    );
    
    debounced
}

// Memo-based validator to reduce unnecessary re-validations
#[hook]
pub fn use_validator<T, F>(value: Signal<T>, validator: F) -> Signal<bool> 
where
    T: PartialEq + 'static,
    F: Fn(&T) -> bool + 'static,
{
    create_memo(move |_| validator(&value.get()))
}

// Optimized text input with debouncing and validation
#[component]
pub fn TextField(
    #[prop(into)] label: String,
    #[prop(into)] value: Signal<String>,
    #[prop(into)] set_value: Callback<String>,
    #[prop(optional)] placeholder: Option<String>,
    #[prop(optional)] error: Option<Signal<Option<String>>>,
    #[prop(optional)] debounce_ms: Option<u32>,
    #[prop(optional)] max_length: Option<u32>,
    #[prop(optional)] on_enter: Option<Callback<()>>,
    #[prop(optional)] input_type: Option<String>,
) -> impl IntoView {
    let placeholder = placeholder.unwrap_or_default();
    let debounce_ms = debounce_ms.unwrap_or(300);
    let input_type = input_type.unwrap_or_else(|| "text".to_string());
    
    // Internal value for immediate updates
    let (internal_value, set_internal_value) = create_signal(value.get());
    
    // Use debounced signal for actual updates
    let _debounced = use_debounced(
        internal_value,
        debounce_ms,
        move |val| set_value.call(val),
    );
    
    // Handle keyboard events
    let handle_keydown = move |event: KeyboardEvent| {
        if let Some(callback) = on_enter.as_ref() {
            if event.key() == "Enter" {
                callback.call(());
            }
        }
    };
    
    // Track focus state for optimizing updates
    let (is_focused, set_is_focused) = create_signal(false);
    
    // Synchronize external value changes when not focused
    create_effect(move |_| {
        let external = value.get();
        if !is_focused.get() && external != internal_value.get() {
            set_internal_value.set(external);
        }
    });
    
    view! {
        <div class="form-field">
            <label>{label}</label>
            <input
                type={input_type}
                value={move || internal_value.get()}
                placeholder={placeholder}
                maxlength={max_length.map(|m| m.to_string())}
                on:input=move |ev| {
                    let value = event_target_value(&ev);
                    set_internal_value.set(value);
                }
                on:keydown=handle_keydown
                on:focus=move |_| set_is_focused.set(true)
                on:blur=move |_| set_is_focused.set(false)
            />
            {move || {
                error
                    .as_ref()
                    .and_then(|e| e.get())
                    .map(|err_message| view! { <div class="error-message">{err_message}</div> })
            }}
        </div>
    }
}

// Auto-resizing TextArea component
#[component]
pub fn TextArea(
    #[prop(into)] label: String,
    #[prop(into)] value: Signal<String>,
    #[prop(into)] set_value: Callback<String>,
    #[prop(optional)] placeholder: Option<String>,
    #[prop(optional)] error: Option<Signal<Option<String>>>,
    #[prop(optional)] debounce_ms: Option<u32>,
    #[prop(optional)] max_length: Option<u32>,
    #[prop(optional)] min_rows: Option<u32>,
    #[prop(optional)] max_rows: Option<u32>,
) -> impl IntoView {
    let placeholder = placeholder.unwrap_or_default();
    let debounce_ms = debounce_ms.unwrap_or(300);
    let min_rows = min_rows.unwrap_or(3);
    let max_rows = max_rows.unwrap_or(10);
    
    // Internal value for immediate updates
    let (internal_value, set_internal_value) = create_signal(value.get());
    
    // Track element ref for auto-resize
    let textarea_ref = create_node_ref::<HtmlTextAreaElement>();
    
    // Use debounced signal for actual updates
    let _debounced = use_debounced(
        internal_value,
        debounce_ms,
        move |val| set_value.call(val),
    );
    
    // Auto-resize function
    let auto_resize = move || {
        if let Some(textarea) = textarea_ref.get() {
            // Reset height to calculate scroll height
            textarea.style().set_property("height", "auto").unwrap();
            
            // Get the scroll height and set it as the new height
            let scroll_height = textarea.scroll_height();
            
            // Calculate rows based on line height (approx. 20px per line)
            let line_height = 20;
            let current_rows = scroll_height as u32 / line_height;
            let clamped_rows = current_rows.clamp(min_rows, max_rows);
            let new_height = clamped_rows * line_height;
            
            textarea.style()
                .set_property("height", &format!("{}px", new_height))
                .unwrap();
        }
    };
    
    // Auto-resize on value change
    create_effect(move |_| {
        let _ = internal_value.get();
        request_animation_frame(auto_resize);
    });
    
    // Track focus state for optimizing updates
    let (is_focused, set_is_focused) = create_signal(false);
    
    // Synchronize external value changes when not focused
    create_effect(move |_| {
        let external = value.get();
        if !is_focused.get() && external != internal_value.get() {
            set_internal_value.set(external);
        }
    });
    
    view! {
        <div class="form-field">
            <label>{label}</label>
            <textarea
                ref=textarea_ref
                value={move || internal_value.get()}
                placeholder={placeholder}
                maxlength={max_length.map(|m| m.to_string())}
                on:input=move |ev| {
                    let value = event_target_value(&ev);
                    set_internal_value.set(value);
                    auto_resize();
                }
                on:focus=move |_| set_is_focused.set(true)
                on:blur=move |_| set_is_focused.set(false)
                style:min-height={format!("{}px", min_rows * 20)}
            ></textarea>
            {move || {
                error
                    .as_ref()
                    .and_then(|e| e.get())
                    .map(|err_message| view! { <div class="error-message">{err_message}</div> })
            }}
        </div>
    }
}

// Request animation frame helper for performance
fn request_animation_frame<F>(callback: F)
where
    F: FnOnce() + 'static,
{
    let window = web_sys::window().unwrap();
    let closure = Closure::once(callback);
    
    window
        .request_animation_frame(closure.as_ref().unchecked_ref())
        .unwrap();
    
    closure.forget();
}