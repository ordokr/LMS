use leptos::*;
use web_sys::{HtmlTextAreaElement, MouseEvent};

#[component]
pub fn RichEditor(
    #[prop()] content: Signal<String>,
    #[prop()] set_content: SignalSetter<String>,
    #[prop(optional)] placeholder: Option<&'static str>,
    #[prop(optional)] rows: Option<u32>,
) -> impl IntoView {
    let rows = rows.unwrap_or(5);
    let placeholder = placeholder.unwrap_or("Write your reply here...");
    let editor_ref = create_node_ref::<HtmlTextAreaElement>();
    let (preview_active, set_preview_active) = create_signal(false);

    // Insert markdown formatting
    let insert_formatting = move |ev: MouseEvent, prefix: &str, suffix: &str, placeholder_text: &str| {
        ev.prevent_default();
        if let Some(textarea) = editor_ref.get() {
            let start = textarea.selection_start().unwrap_or(0);
            let end = textarea.selection_end().unwrap_or(0);
            let current_text = content();
            
            let selected_text = if start != end {
                current_text[start as usize..end as usize].to_string()
            } else {
                placeholder_text.to_string()
            };
            
            let new_text = format!(
                "{}{}{}{}{}",
                &current_text[0..start as usize],
                prefix,
                selected_text,
                suffix,
                &current_text[end as usize..]
            );
            
            set_content.set(new_text);
            
            // Request animation frame to ensure DOM update
            request_animation_frame(move || {
                if let Some(textarea) = editor_ref.get() {
                    let _ = textarea.focus();
                    let new_cursor_pos = start as u32 + prefix.len() as u32;
                    let end_pos = new_cursor_pos + selected_text.len() as u32;
                    textarea.set_selection_range(new_cursor_pos, end_pos).ok();
                }
            });
        }
    };

    let rendered_markdown = move || {
        // Use markdown crate for rendering
        match markdown::to_html_with_options(&content(), &markdown::Options::gfm()) {
            Ok(html) => html,
            Err(_) => format!("<p>{}</p>", content().replace("<", "&lt;").replace(">", "&gt;").replace("\n", "<br>"))
        }
    };

    view! {
        <div class="rich-editor">
            <div class="editor-toolbar d-flex flex-wrap gap-2 mb-2 bg-light p-2 rounded">
                <button type="button" class="btn btn-sm btn-outline-secondary" 
                        on:click=move |ev| insert_formatting(ev, "**", "**", "bold text")>
                    <i class="bi bi-type-bold"></i>
                </button>
                <button type="button" class="btn btn-sm btn-outline-secondary"
                        on:click=move |ev| insert_formatting(ev, "*", "*", "italic text")>
                    <i class="bi bi-type-italic"></i>
                </button>
                <button type="button" class="btn btn-sm btn-outline-secondary"
                        on:click=move |ev| insert_formatting(ev, "[", "](https://example.com)", "link text")>
                    <i class="bi bi-link"></i>
                </button>
                <button type="button" class="btn btn-sm btn-outline-secondary"
                        on:click=move |ev| insert_formatting(ev, "![alt text](", ")", "https://example.com/image.jpg")>
                    <i class="bi bi-image"></i>
                </button>
                <button type="button" class="btn btn-sm btn-outline-secondary"
                        on:click=move |ev| insert_formatting(ev, "`", "`", "code")>
                    <i class="bi bi-code"></i>
                </button>
                <button type="button" class="btn btn-sm btn-outline-secondary"
                        on:click=move |ev| insert_formatting(ev, "```\n", "\n```", "code block")>
                    <i class="bi bi-code-square"></i>
                </button>
                <button type="button" class="btn btn-sm btn-outline-secondary"
                        on:click=move |ev| insert_formatting(ev, "> ", "", "blockquote")>
                    <i class="bi bi-blockquote-left"></i>
                </button>
                <button type="button" class="btn btn-sm btn-outline-secondary"
                        on:click=move |ev| insert_formatting(ev, "* ", "", "list item")>
                    <i class="bi bi-list-ul"></i>
                </button>
                <button type="button" class="btn btn-sm btn-outline-secondary"
                        on:click=move |ev| insert_formatting(ev, "1. ", "", "numbered item")>
                    <i class="bi bi-list-ol"></i>
                </button>
                <div class="ms-auto">
                    <button type="button" 
                            class="btn btn-sm" 
                            class:btn-outline-primary=move || !preview_active()
                            class:btn-primary=move || preview_active()
                            on:click=move |_| set_preview_active.update(|p| *p = !*p)>
                        {move || if preview_active() { "Edit" } else { "Preview" }}
                    </button>
                </div>
            </div>
            
            <div class="editor-content">
                {move || {
                    if preview_active() {
                        view! {
                            <div class="markdown-preview border rounded p-3" inner_html=rendered_markdown()></div>
                        }
                    } else {
                        view! {
                            <textarea
                                ref=editor_ref
                                class="form-control"
                                rows=rows
                                placeholder=placeholder
                                prop:value=content
                                on:input=move |ev| set_content.set(event_target_value(&ev))
                            ></textarea>
                        }
                    }
                }}
            </div>
            
            <div class="small text-muted mt-2">
                "You can use Markdown formatting. " 
                <a href="https://commonmark.org/help/" target="_blank" rel="noopener">
                    "Formatting help"
                </a>
            </div>
        </div>
    }
}

// Helper function for animation frames
fn request_animation_frame<F>(callback: F)
where
    F: FnOnce() + 'static,
{
    use wasm_bindgen::prelude::*;
    
    let window = web_sys::window().unwrap();
    let closure = Closure::once(callback);
    window.request_animation_frame(closure.as_ref().unchecked_ref()).ok();
    closure.forget(); // Prevent memory leak
}