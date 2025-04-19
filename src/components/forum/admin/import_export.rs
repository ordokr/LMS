use leptos::*;
use web_sys::{File, FormData, HtmlInputElement};
use wasm_bindgen::JsCast;
use crate::services::admin::AdminService;
use crate::models::admin::{ExportOptions, ImportOptions};
use gloo_file::Blob;

// This component provides import/export features for forum data for test/demo purposes only.
// It is NOT intended for production data migration or live system import. Ordo is a source-to-source port and does not support or recommend live data migration or user import from existing systems.
// All references to 'import' or 'export' in this file refer to test/demo data workflows, not production data migration.

#[component]
pub fn ImportExport() -> impl IntoView {
    // Admin permission check
    let auth_state = use_context::<AuthState>();
    let is_admin = move || auth_state.map(|s| s.is_admin()).unwrap_or(false);
    
    // State signals
    let (loading, set_loading) = create_signal(false);
    let (error, set_error) = create_signal(None::<String>);
    let (success, set_success) = create_signal(None::<String>);
    
    // Export options
    let (include_users, set_include_users) = create_signal(true);
    let (include_categories, set_include_categories) = create_signal(true);
    let (include_tags, set_include_tags) = create_signal(true);
    let (include_topics, set_include_topics) = create_signal(true);
    let (include_posts, set_include_posts) = create_signal(true);
    let (include_uploads, set_include_uploads) = create_signal(false);
    let (include_settings, set_include_settings) = create_signal(true);
    let (export_format, set_export_format) = create_signal("json".to_string());
    let (exporting, set_exporting) = create_signal(false);
    
    // Import options
    let (merge_users, set_merge_users) = create_signal(false);
    let (replace_categories, set_replace_categories) = create_signal(false);
    let (replace_tags, set_replace_tags) = create_signal(false);
    let (importing, set_importing) = create_signal(false);
    let (selected_file_name, set_selected_file_name) = create_signal(None::<String>);
    
    // Backup history
    let (backups, set_backups) = create_signal(Vec::<crate::models::admin::BackupInfo>::new());
    let (backups_loading, set_backups_loading) = create_signal(true);
    
    // Load backup history
    create_effect(move |_| {
        if !is_admin() {
            return;
        }
        
        set_backups_loading.set(true);
        
        spawn_local(async move {
            match AdminService::get_backup_history().await {
                Ok(loaded_backups) => {
                    set_backups.set(loaded_backups);
                    set_backups_loading.set(false);
                },
                Err(e) => {
                    set_error.set(Some(format!("Failed to load backup history: {}", e)));
                    set_backups_loading.set(false);
                }
            }
        });
    });
    
    // Handle export action
    let handle_export = move |_| {
        set_error.set(None);
        set_success.set(None);
        set_exporting.set(true);
        
        let options = ExportOptions {
            include_users: include_users(),
            include_categories: include_categories(),
            include_tags: include_tags(),
            include_topics: include_topics(),
            include_posts: include_posts(),
            include_uploads: include_uploads(),
            include_settings: include_settings(),
            format: export_format(),
        };
        
        spawn_local(async move {
            match AdminService::export_data(options).await {
                Ok(download_url) => {
                    // Trigger download by creating a temporary link
                    let document = web_sys::window().unwrap().document().unwrap();
                    let anchor = document.create_element("a").unwrap();
                    let anchor_element = anchor.dyn_into::<web_sys::HtmlAnchorElement>().unwrap();
                    
                    anchor_element.set_href(&download_url);
                    anchor_element.set_download(&format!("forum-export-{}.{}", chrono::Utc::now().format("%Y%m%d-%H%M%S"), export_format()));
                    document.body().unwrap().append_child(&anchor_element).unwrap();
                    anchor_element.click();
                    document.body().unwrap().remove_child(&anchor_element).unwrap();
                    
                    set_success.set(Some("Export completed successfully. Your download should begin shortly.".to_string()));
                    
                    // Refresh backup history
                    match AdminService::get_backup_history().await {
                        Ok(loaded_backups) => {
                            set_backups.set(loaded_backups);
                        },
                        Err(_) => {} // Ignore error on refresh
                    }
                },
                Err(e) => {
                    set_error.set(Some(format!("Failed to export data: {}", e)));
                }
            }
            set_exporting.set(false);
        });
    };
    
    // Handle import file selection
    let handle_file_select = move |ev: web_sys::Event| {
        let input = event_target::<HtmlInputElement>(&ev);
        let files = input.files().expect("No files");
        
        if let Some(file) = files.get(0) {
            set_selected_file_name.set(Some(file.name()));
        } else {
            set_selected_file_name.set(None);
        }
    };
    
    // Handle import action
    let handle_import = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        
        // Get the file input element
        let form = ev.target().unwrap().dyn_into::<web_sys::HtmlFormElement>().unwrap();
        let file_input = form.elements().named_item("importFile").unwrap().dyn_into::<HtmlInputElement>().unwrap();
        
        if let Some(files) = file_input.files() {
            if let Some(file) = files.get(0) {
                set_error.set(None);
                set_success.set(None);
                set_importing.set(true);
                
                // Create FormData with file and options
                let form_data = FormData::new().unwrap();
                form_data.append_with_blob("file", &file.into()).unwrap();
                
                // Add options as form data fields
                form_data.append_with_str("mergeUsers", &merge_users().to_string()).unwrap();
                form_data.append_with_str("replaceCategories", &replace_categories().to_string()).unwrap();
                form_data.append_with_str("replaceTags", &replace_tags().to_string()).unwrap();
                
                spawn_local(async move {
                    match AdminService::import_data(form_data).await {
                        Ok(stats) => {
                            let message = format!(
                                "Import completed successfully. Imported: {} users, {} categories, {} tags, {} topics, {} posts.",
                                stats.users_imported, 
                                stats.categories_imported,
                                stats.tags_imported,
                                stats.topics_imported,
                                stats.posts_imported
                            );
                            set_success.set(Some(message));
                            set_selected_file_name.set(None);
                            file_input.set_value(""); // Clear the file input
                            
                            // Refresh backup history
                            match AdminService::get_backup_history().await {
                                Ok(loaded_backups) => {
                                    set_backups.set(loaded_backups);
                                },
                                Err(_) => {} // Ignore error on refresh
                            }
                        },
                        Err(e) => {
                            set_error.set(Some(format!("Failed to import data: {}", e)));
                        }
                    }
                    set_importing.set(false);
                });
            } else {
                set_error.set(Some("No file selected".to_string()));
            }
        } else {
            set_error.set(Some("No file selected".to_string()));
        }
    };
    
    // Download backup
    let download_backup = move |backup_id: String| {
        set_loading.set(true);
        set_error.set(None);
        
        spawn_local(async move {
            match AdminService::download_backup(backup_id).await {
                Ok(download_url) => {
                    // Trigger download by creating a temporary link
                    let document = web_sys::window().unwrap().document().unwrap();
                    let anchor = document.create_element("a").unwrap();
                    let anchor_element = anchor.dyn_into::<web_sys::HtmlAnchorElement>().unwrap();
                    
                    anchor_element.set_href(&download_url);
                    anchor_element.set_download("backup.zip");
                    document.body().unwrap().append_child(&anchor_element).unwrap();
                    anchor_element.click();
                    document.body().unwrap().remove_child(&anchor_element).unwrap();
                },
                Err(e) => {
                    set_error.set(Some(format!("Failed to download backup: {}", e)));
                }
            }
            set_loading.set(false);
        });
    };
    
    // Delete backup
    let delete_backup = move |backup_id: String| {
        if !window().confirm_with_message("Are you sure you want to delete this backup?").unwrap_or(false) {
            return;
        }
        
        set_loading.set(true);
        set_error.set(None);
        
        spawn_local(async move {
            match AdminService::delete_backup(backup_id).await {
                Ok(_) => {
                    set_success.set(Some("Backup deleted successfully".to_string()));
                    
                    // Refresh backup history
                    match AdminService::get_backup_history().await {
                        Ok(loaded_backups) => {
                            set_backups.set(loaded_backups);
                        },
                        Err(e) => {
                            set_error.set(Some(format!("Failed to refresh backup history: {}", e)));
                        }
                    }
                },
                Err(e) => {
                    set_error.set(Some(format!("Failed to delete backup: {}", e)));
                }
            }
            set_loading.set(false);
        });
    };
    
    // Format file size
    let format_file_size = |size: usize| -> String {
        if size < 1024 {
            return format!("{} B", size);
        } else if size < 1024 * 1024 {
            return format!("{:.1} KB", size as f64 / 1024.0);
        } else if size < 1024 * 1024 * 1024 {
            return format!("{:.1} MB", size as f64 / (1024.0 * 1024.0));
        } else {
            return format!("{:.1} GB", size as f64 / (1024.0 * 1024.0 * 1024.0));
        }
    };
    
    // Format date
    let format_date = |date: chrono::DateTime<chrono::Utc>| -> String {
        date.format("%Y-%m-%d %H:%M:%S").to_string()
    };

    view! {
        <div class="import-export">
            <div class="alert alert-warning mb-3">
                <i class="bi bi-exclamation-triangle-fill me-1"></i>
                <b>Important:</b> Import/Export features are for test/demo data only. Ordo does <u>not</u> support production data migration or live system import. All references to import/export refer to test/demo workflows, not live data.
            </div>
            {move || if !is_admin() {
                view! {
                    <div class="alert alert-danger">
                        "You don't have permission to access this page."
                    </div>
                }
            } else {
                view! {
                    <div>
                        <h1 class="mb-4">"Import & Export"</h1>
                        
                        {move || error().map(|err| view! { <div class="alert alert-danger mb-4">{err}</div> })}
                        {move || success().map(|msg| view! { <div class="alert alert-success mb-4">{msg}</div> })}
                        
                        <div class="row">
                            <div class="col-md-6 mb-4">
                                <div class="card">
                                    <div class="card-header">
                                        <h5 class="mb-0">"Export Data"</h5>
                                    </div>
                                    <div class="card-body">
                                        <p class="text-muted mb-4">
                                            "Export your forum data for backup or migration purposes. Select which data to include in the export."
                                        </p>
                                        
                                        <div class="mb-3">
                                            <div class="form-check mb-2">
                                                <input 
                                                    class="form-check-input" 
                                                    type="checkbox" 
                                                    id="includeUsers"
                                                    prop:checked=move || include_users()
                                                    on:change=move |ev| set_include_users.set(event_target_checked(&ev))
                                                />
                                                <label class="form-check-label" for="includeUsers">
                                                    "Users"
                                                </label>
                                            </div>
                                            <div class="form-check mb-2">
                                                <input 
                                                    class="form-check-input" 
                                                    type="checkbox" 
                                                    id="includeCategories"
                                                    prop:checked=move || include_categories()
                                                    on:change=move |ev| set_include_categories.set(event_target_checked(&ev))
                                                />
                                                <label class="form-check-label" for="includeCategories">
                                                    "Categories"
                                                </label>
                                            </div>
                                            <div class="form-check mb-2">
                                                <input 
                                                    class="form-check-input" 
                                                    type="checkbox" 
                                                    id="includeTags"
                                                    prop:checked=move || include_tags()
                                                    on:change=move |ev| set_include_tags.set(event_target_checked(&ev))
                                                />
                                                <label class="form-check-label" for="includeTags">
                                                    "Tags"
                                                </label>
                                            </div>
                                            <div class="form-check mb-2">
                                                <input 
                                                    class="form-check-input" 
                                                    type="checkbox" 
                                                    id="includeTopics"
                                                    prop:checked=move || include_topics()
                                                    on:change=move |ev| set_include_topics.set(event_target_checked(&ev))
                                                />
                                                <label class="form-check-label" for="includeTopics">
                                                    "Topics"
                                                </label>
                                            </div>
                                            <div class="form-check mb-2">
                                                <input 
                                                    class="form-check-input" 
                                                    type="checkbox" 
                                                    id="includePosts"
                                                    prop:checked=move || include_posts()
                                                    on:change=move |ev| set_include_posts.set(event_target_checked(&ev))
                                                />
                                                <label class="form-check-label" for="includePosts">
                                                    "Posts"
                                                </label>
                                            </div>
                                            <div class="form-check mb-2">
                                                <input 
                                                    class="form-check-input" 
                                                    type="checkbox" 
                                                    id="includeUploads"
                                                    prop:checked=move || include_uploads()
                                                    on:change=move |ev| set_include_uploads.set(event_target_checked(&ev))
                                                />
                                                <label class="form-check-label" for="includeUploads">
                                                    "Uploads & Attachments"
                                                </label>
                                                <div class="form-text small">
                                                    "Warning: This may create a very large export file"
                                                </div>
                                            </div>
                                            <div class="form-check mb-2">
                                                <input 
                                                    class="form-check-input" 
                                                    type="checkbox" 
                                                    id="includeSettings"
                                                    prop:checked=move || include_settings()
                                                    on:change=move |ev| set_include_settings.set(event_target_checked(&ev))
                                                />
                                                <label class="form-check-label" for="includeSettings">
                                                    "Settings & Customizations"
                                                </label>
                                            </div>
                                        </div>
                                        
                                        <div class="mb-3">
                                            <label class="form-label">"Export Format"</label>
                                            <div class="d-flex align-items-center">
                                                <div class="form-check me-3">
                                                    <input 
                                                        class="form-check-input" 
                                                        type="radio" 
                                                        name="exportFormat" 
                                                        id="formatJSON"
                                                        value="json"
                                                        prop:checked=move || export_format() == "json"
                                                        on:change=move |_| set_export_format.set("json".to_string())
                                                    />
                                                    <label class="form-check-label" for="formatJSON">
                                                        "JSON"
                                                    </label>
                                                </div>
                                                <div class="form-check me-3">
                                                    <input 
                                                        class="form-check-input" 
                                                        type="radio" 
                                                        name="exportFormat" 
                                                        id="formatCSV"
                                                        value="csv"
                                                        prop:checked=move || export_format() == "csv"
                                                        on:change=move |_| set_export_format.set("csv".to_string())
                                                    />
                                                    <label class="form-check-label" for="formatCSV">
                                                        "CSV"
                                                    </label>
                                                </div>
                                                <div class="form-check">
                                                    <input 
                                                        class="form-check-input" 
                                                        type="radio" 
                                                        name="exportFormat" 
                                                        id="formatSql"
                                                        value="sql"
                                                        prop:checked=move || export_format() == "sql"
                                                        on:change=move |_| set_export_format.set("sql".to_string())
                                                    />
                                                    <label class="form-check-label" for="formatSql">
                                                        "SQL"
                                                    </label>
                                                </div>
                                            </div>
                                        </div>
                                        
                                        <div class="mt-4">
                                            <button 
                                                class="btn btn-primary" 
                                                on:click=handle_export
                                                disabled=move || exporting()
                                            >
                                                {move || if exporting() {
                                                    view! { <span class="spinner-border spinner-border-sm me-2" role="status"></span> "Exporting..." }
                                                } else {
                                                    view! { <><i class="bi bi-download me-1"></i> "Export Data"</> }
                                                }}
                                            </button>
                                        </div>
                                    </div>
                                </div>
                            </div>
                            
                            <div class="col-md-6 mb-4">
                                <div class="card">
                                    <div class="card-header">
                                        <h5 class="mb-0">"Import Data"</h5>
                                    </div>
                                    <div class="card-body">
                                        <p class="text-muted mb-4">
                                            "Import data from a previous export or from another platform. The file format will be automatically detected."
                                        </p>
                                        
                                        <form on:submit=handle_import>
                                            <div class="mb-3">
                                                <label for="importFile" class="form-label">"Import File"</label>
                                                <input 
                                                    class="form-control" 
                                                    type="file" 
                                                    id="importFile"
                                                    name="importFile"
                                                    accept=".json,.csv,.sql,.zip"
                                                    on:change=handle_file_select
                                                    required
                                                />
                                                {move || selected_file_name().map(|name| {
                                                    view! { <div class="form-text">"Selected file: " {name}</div> }
                                                })}
                                            </div>
                                            
                                            <div class="mb-3">
                                                <label class="form-label">"Import Options"</label>
                                                <div class="form-check mb-2">
                                                    <input 
                                                        class="form-check-input" 
                                                        type="checkbox" 
                                                        id="mergeUsers"
                                                        prop:checked=move || merge_users()
                                                        on:change=move |ev| set_merge_users.set(event_target_checked(&ev))
                                                    />
                                                    <label class="form-check-label" for="mergeUsers">
                                                        "Merge duplicate users"
                                                    </label>
                                                    <div class="form-text small">
                                                        "When enabled, existing users will be updated based on username or email."
                                                    </div>
                                                </div>
                                                <div class="form-check mb-2">
                                                    <input 
                                                        class="form-check-input" 
                                                        type="checkbox" 
                                                        id="replaceCategories"
                                                        prop:checked=move || replace_categories()
                                                        on:change=move |ev| set_replace_categories.set(event_target_checked(&ev))
                                                    />
                                                    <label class="form-check-label" for="replaceCategories">
                                                        "Replace existing categories"
                                                    </label>
                                                    <div class="form-text small">
                                                        "When enabled, existing categories with the same slug will be replaced."
                                                    </div>
                                                </div>
                                                <div class="form-check mb-2">
                                                    <input 
                                                        class="form-check-input" 
                                                        type="checkbox" 
                                                        id="replaceTags"
                                                        prop:checked=move || replace_tags()
                                                        on:change=move |ev| set_replace_tags.set(event_target_checked(&ev))
                                                    />
                                                    <label class="form-check-label" for="replaceTags">
                                                        "Replace existing tags"
                                                    </label>
                                                    <div class="form-text small">
                                                        "When enabled, existing tags with the same name will be replaced."
                                                    </div>
                                                </div>
                                            </div>
                                            
                                            <div class="alert alert-warning mb-4 small">
                                                <i class="bi bi-exclamation-triangle-fill me-1"></i>
                                                "Warning: Importing data may overwrite existing content. It's recommended to back up your database before importing."
                                            </div>
                                            
                                            <div class="mt-4">
                                                <button 
                                                    class="btn btn-primary" 
                                                    type="submit"
                                                    disabled=move || importing() || selected_file_name().is_none()
                                                >
                                                    {move || if importing() {
                                                        view! { <span class="spinner-border spinner-border-sm me-2" role="status"></span> "Importing..." }
                                                    } else {
                                                        view! { <><i class="bi bi-upload me-1"></i> "Import Data"</> }
                                                    }}
                                                </button>
                                            </div>
                                        </form>
                                    </div>
                                </div>
                            </div>
                        </div>
                        
                        <div class="card">
                            <div class="card-header d-flex justify-content-between align-items-center">
                                <h5 class="mb-0">"Backup History"</h5>
                            </div>
                            <div class="card-body">
                                {move || if backups_loading() {
                                    view! { <div class="d-flex justify-content-center p-3"><div class="spinner-border" role="status"></div></div> }
                                } else if backups().is_empty() {
                                    view! {
                                        <div class="text-center p-4">
                                            <i class="bi bi-archive mb-3 d-block" style="font-size: 3rem;"></i>
                                            <h4>"No Backups Found"</h4>
                                            <p class="text-muted">
                                                "Create a backup by exporting your forum data above."
                                            </p>
                                        </div>
                                    }
                                } else {
                                    view! {
                                        <div class="table-responsive">
                                            <table class="table table-hover">
                                                <thead>
                                                    <tr>
                                                        <th>"Filename"</th>
                                                        <th>"Date Created"</th>
                                                        <th>"Size"</th>
                                                        <th>"Type"</th>
                                                        <th>"Actions"</th>
                                                    </tr>
                                                </thead>
                                                <tbody>
                                                    {backups().into_iter().map(|backup| {
                                                        let backup_id_download = backup.id.clone();
                                                        let backup_id_delete = backup.id.clone();
                                                        
                                                        view! {
                                                            <tr>
                                                                <td>{backup.filename}</td>
                                                                <td>{format_date(backup.created_at)}</td>
                                                                <td>{format_file_size(backup.size)}</td>
                                                                <td>
                                                                    <span class="badge bg-secondary">
                                                                        {backup.format.to_uppercase()}
                                                                    </span>
                                                                </td>
                                                                <td>
                                                                    <div class="d-flex gap-2">
                                                                        <button 
                                                                            class="btn btn-sm btn-outline-primary" 
                                                                            on:click=move |_| download_backup(backup_id_download.clone())
                                                                            disabled=move || loading()
                                                                        >
                                                                            <i class="bi bi-download"></i>
                                                                        </button>
                                                                        <button 
                                                                            class="btn btn-sm btn-outline-danger" 
                                                                            on:click=move |_| delete_backup(backup_id_delete.clone())
                                                                            disabled=move || loading()
                                                                        >
                                                                            <i class="bi bi-trash"></i>
                                                                        </button>
                                                                    </div>
                                                                </td>
                                                            </tr>
                                                        }
                                                    }).collect::<Vec<_>>()}
                                                </tbody>
                                            </table>
                                        </div>
                                    }
                                }}
                            </div>
                        </div>
                    </div>
                }
            }}
        </div>
    }
}