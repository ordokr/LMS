use leptos::*;
use wasm_bindgen::JsCast;
use web_sys::{Event, File, FileList, HtmlInputElement};
use std::rc::Rc;
use gloo_file::Blob;

#[derive(Debug, Clone)]
pub struct UploadedFile {
    pub name: String,
    pub size: u32,
    pub file: File,
    pub content_type: String,
    pub preview_url: Option<String>,
}

impl PartialEq for UploadedFile {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.size == other.size && self.content_type == other.content_type
    }
}

#[component]
pub fn FileUpload(
    #[prop(into)] on_files_selected: Callback<Vec<UploadedFile>>,
    #[prop(default = false)] multiple: bool,
    #[prop(default = vec!["*".to_string()])] accept: Vec<String>,
    #[prop(default = "Upload Files")] button_text: &'static str,
    #[prop(default = "Upload files (or drag and drop)")] placeholder_text: &'static str,
    #[prop(default = 10 * 1024 * 1024)] max_size: u32, // 10MB by default
    #[prop(default = None)] class: Option<&'static str>,
) -> impl IntoView {
    let (is_dragging, set_is_dragging) = create_signal(false);
    let (error_message, set_error_message) = create_signal(None::<String>);
    
    let handle_file_select = move |event: Event| {
        let input: HtmlInputElement = event.target().unwrap().dyn_into().unwrap();
        
        if let Some(files) = input.files() {
            process_files(files, set_error_message, on_files_selected, max_size);
        }
    };
    
    let handle_drag_over = move |event: DragEvent| {
        event.prevent_default();
        set_is_dragging(true);
    };
    
    let handle_drag_leave = move |event: DragEvent| {
        event.prevent_default();
        set_is_dragging(false);
    };
    
    let handle_drop = move |event: DragEvent| {
        event.prevent_default();
        set_is_dragging(false);
        
        if let Some(data_transfer) = event.data_transfer() {
            if let Some(files) = data_transfer.files() {
                process_files(files, set_error_message, on_files_selected, max_size);
            }
        }
    };
    
    view! {
        <div 
            class=format!("file-upload-container {}", class.unwrap_or(""))
            class=("dragging", move || is_dragging())
            on:dragover=handle_drag_over
            on:dragleave=handle_drag_leave
            on:drop=handle_drop
        >
            <label class="file-upload-label">
                <div class="upload-icon">
                    <i class="bi bi-cloud-arrow-up"></i>
                </div>
                <span class="upload-text">{placeholder_text}</span>
                <span class="btn btn-primary upload-button">{button_text}</span>
                <input 
                    type="file" 
                    on:change=handle_file_select 
                    multiple=multiple
                    accept=accept.join(",")
                    class="file-input" 
                />
            </label>
            
            {move || error_message().map(|msg| view! {
                <div class="alert alert-danger mt-2" role="alert">
                    {msg}
                </div>
            })}
        </div>
    }
}

fn process_files(
    files: FileList,
    set_error_message: WriteSignal<Option<String>>,
    on_files_selected: Callback<Vec<UploadedFile>>,
    max_size: u32,
) {
    let mut uploaded_files = Vec::new();
    let files_count = files.length();
    
    for i in 0..files_count {
        if let Some(file) = files.get(i) {
            // Check file size
            if file.size() as u32 > max_size {
                set_error_message(Some(format!(
                    "File '{}' is too large. Maximum allowed size is {}MB.",
                    file.name(),
                    max_size / (1024 * 1024)
                )));
                return;
            }
            
            // Create URL for image preview if it's an image
            let preview_url = if file.type_().starts_with("image/") {
                match create_object_url(&file) {
                    Ok(url) => Some(url),
                    Err(_) => None,
                }
            } else {
                None
            };
            
            uploaded_files.push(UploadedFile {
                name: file.name(),
                size: file.size() as u32,
                content_type: file.type_(),
                file,
                preview_url,
            });
        }
    }
    
    if !uploaded_files.is_empty() {
        set_error_message(None);
        on_files_selected.call(uploaded_files);
    }
}

fn create_object_url(file: &File) -> Result<String, String> {
    let blob = Blob::from(file.clone());
    let url = web_sys::Url::create_object_url_with_blob(&blob.as_ref())
        .map_err(|_| "Failed to create object URL".to_string())?;
    Ok(url)
}

#[component]
pub fn FilePreview(
    #[prop(into)] files: MaybeSignal<Vec<UploadedFile>>,
    #[prop(into)] on_remove: Callback<usize>,
) -> impl IntoView {
    let format_size = |size: u32| -> String {
        if size < 1024 {
            format!("{} B", size)
        } else if size < 1024 * 1024 {
            format!("{:.1} KB", size as f64 / 1024.0)
        } else {
            format!("{:.1} MB", size as f64 / (1024.0 * 1024.0))
        }
    };
    
    let get_file_icon = |file: &UploadedFile| -> String {
        match file.content_type.as_str() {
            type_str if type_str.starts_with("image/") => "bi-file-image".to_string(),
            "application/pdf" => "bi-file-pdf".to_string(),
            type_str if type_str.starts_with("text/") => "bi-file-text".to_string(),
            type_str if type_str.starts_with("video/") => "bi-file-play".to_string(),
            type_str if type_str.starts_with("audio/") => "bi-file-music".to_string(),
            "application/zip" | "application/x-zip-compressed" => "bi-file-zip".to_string(),
            "application/msword" | "application/vnd.openxmlformats-officedocument.wordprocessingml.document" => "bi-file-word".to_string(),
            "application/vnd.ms-excel" | "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet" => "bi-file-excel".to_string(),
            "application/vnd.ms-powerpoint" | "application/vnd.openxmlformats-officedocument.presentationml.presentation" => "bi-file-ppt".to_string(),
            _ => "bi-file-earmark".to_string(),
        }
    };
    
    let handle_remove = move |index: usize| {
        on_remove.call(index);
    };
    
    view! {
        <div class="file-preview-container">
            {move || {
                let files = files();
                if files.is_empty() {
                    view! { <div></div> }.into_view()
                } else {
                    files.into_iter().enumerate().map(|(index, file)| {
                        let icon_class = get_file_icon(&file);
                        
                        view! {
                            <div class="file-preview-item">
                                {move || {
                                    if let Some(url) = &file.preview_url {
                                        view! {
                                            <div class="file-preview-image">
                                                <img src={url.clone()} alt={file.name.clone()} />
                                            </div>
                                        }.into_view()
                                    } else {
                                        view! {
                                            <div class="file-preview-icon">
                                                <i class=format!("bi {}", icon_class)></i>
                                            </div>
                                        }.into_view()
                                    }
                                }}
                                
                                <div class="file-preview-details">
                                    <div class="file-preview-name">{file.name.clone()}</div>
                                    <div class="file-preview-size">{format_size(file.size)}</div>
                                </div>
                                
                                <button 
                                    type="button" 
                                    class="btn btn-sm btn-outline-danger file-preview-remove"
                                    on:click=move |_| handle_remove(index)
                                >
                                    <i class="bi bi-x"></i>
                                </button>
                            </div>
                        }
                    }).collect_view()
                }
            }}
        </div>
    }
}
