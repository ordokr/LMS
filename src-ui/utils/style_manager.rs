use leptos::*;
use wasm_bindgen::JsCast;
use web_sys::{Document, HtmlLinkElement};
use std::collections::HashSet;

// Track which stylesheets have been loaded to avoid duplicates
static mut LOADED_STYLESHEETS: Option<HashSet<String>> = None;

/// Initialize the stylesheet tracking system
pub fn init_style_manager() {
    unsafe {
        if LOADED_STYLESHEETS.is_none() {
            LOADED_STYLESHEETS = Some(HashSet::new());
        }
    }
}

/// Load a CSS stylesheet
pub fn load_stylesheet(href: &str) {
    // Initialize the style manager if needed
    unsafe {
        if LOADED_STYLESHEETS.is_none() {
            init_style_manager();
        }
        
        // Check if stylesheet is already loaded
        let loaded_stylesheets = LOADED_STYLESHEETS.as_mut().unwrap();
        if loaded_stylesheets.contains(href) {
            return;
        }
        
        // Mark as loaded
        loaded_stylesheets.insert(href.to_string());
    }
    
    // Get the document and head
    if let Some(window) = web_sys::window() {
        if let Some(document) = window.document() {
            if let Some(head) = document.head() {
                // Create link element
                if let Ok(link) = document.create_element("link") {
                    let link = link.dyn_into::<HtmlLinkElement>().unwrap();
                    
                    // Set attributes
                    link.set_rel("stylesheet");
                    link.set_href(href);
                    
                    // Append to head
                    if let Err(e) = head.append_child(&link) {
                        log::error!("Failed to add stylesheet: {}", e);
                    }
                }
            }
        }
    }
}

/// Component hook to load a CSS stylesheet
#[hook]
pub fn use_stylesheet(href: &str) {
    let href = href.to_string();
    
    create_effect(move |_| {
        load_stylesheet(&href);
    });
}
