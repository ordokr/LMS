use leptos::*;
use web_sys::SubmitEvent;
use crate::models::admin::SiteCustomization;
use crate::services::admin::AdminService;

#[component]
pub fn SiteCustomization() -> impl IntoView {
    // Admin permission check
    let auth_state = use_context::<AuthState>();
    let is_admin = move || auth_state.map(|s| s.is_admin()).unwrap_or(false);
    
    // State signals
    let (customization, set_customization) = create_signal(None::<SiteCustomization>);
    let (loading, set_loading) = create_signal(true);
    let (saving, set_saving) = create_signal(false);
    let (error, set_error) = create_signal(None::<String>);
    let (success, set_success) = create_signal(None::<String>);
    
    // Form signals for site header
    let (site_name, set_site_name) = create_signal(String::new());
    let (site_tagline, set_site_tagline) = create_signal(String::new());
    let (site_description, set_site_description) = create_signal(String::new());
    let (site_logo_url, set_site_logo_url) = create_signal(String::new());
    let (site_favicon_url, set_site_favicon_url) = create_signal(String::new());
    
    // Form signals for colors
    let (primary_color, set_primary_color) = create_signal("#0d6efd".to_string());
    let (secondary_color, set_secondary_color) = create_signal("#6c757d".to_string());
    let (success_color, set_success_color) = create_signal("#198754".to_string());
    let (info_color, set_info_color) = create_signal("#0dcaf0".to_string());
    let (warning_color, set_warning_color) = create_signal("#ffc107".to_string());
    let (danger_color, set_danger_color) = create_signal("#dc3545".to_string());
    let (background_color, set_background_color) = create_signal("#f8f9fa".to_string());
    let (text_color, set_text_color) = create_signal("#212529".to_string());
    
    // Form signals for typography
    let (heading_font, set_heading_font) = create_signal("system-ui, -apple-system, sans-serif".to_string());
    let (body_font, set_body_font) = create_signal("system-ui, -apple-system, sans-serif".to_string());
    let (code_font, set_code_font) = create_signal("SFMono-Regular, Menlo, Monaco, Consolas, monospace".to_string());
    let (base_font_size, set_base_font_size) = create_signal(16);
    
    // Form signals for components
    let (border_radius, set_border_radius) = create_signal(0.25);
    let (button_style, set_button_style) = create_signal("default".to_string());
    let (card_style, set_card_style) = create_signal("default".to_string());
    
    // Form signals for custom code
    let (custom_css, set_custom_css) = create_signal(String::new());
    let (custom_header_html, set_custom_header_html) = create_signal(String::new());
    let (custom_footer_html, set_custom_footer_html) = create_signal(String::new());
    
    // Load current customizations
    create_effect(move |_| {
        if !is_admin() {
            set_loading.set(false);
            return;
        }
        
        spawn_local(async move {
            match AdminService::get_site_customization().await {
                Ok(loaded_customization) => {
                    // Populate form fields
                    set_site_name.set(loaded_customization.site_name.clone());
                    set_site_tagline.set(loaded_customization.site_tagline.clone().unwrap_or_default());
                    set_site_description.set(loaded_customization.site_description.clone().unwrap_or_default());
                    set_site_logo_url.set(loaded_customization.site_logo_url.clone().unwrap_or_default());
                    set_site_favicon_url.set(loaded_customization.site_favicon_url.clone().unwrap_or_default());
                    
                    set_primary_color.set(loaded_customization.primary_color.clone());
                    set_secondary_color.set(loaded_customization.secondary_color.clone());
                    set_success_color.set(loaded_customization.success_color.clone());
                    set_info_color.set(loaded_customization.info_color.clone());
                    set_warning_color.set(loaded_customization.warning_color.clone());
                    set_danger_color.set(loaded_customization.danger_color.clone());
                    set_background_color.set(loaded_customization.background_color.clone());
                    set_text_color.set(loaded_customization.text_color.clone());
                    
                    set_heading_font.set(loaded_customization.heading_font.clone());
                    set_body_font.set(loaded_customization.body_font.clone());
                    set_code_font.set(loaded_customization.code_font.clone());
                    set_base_font_size.set(loaded_customization.base_font_size);
                    
                    set_border_radius.set(loaded_customization.border_radius);
                    set_button_style.set(loaded_customization.button_style.clone());
                    set_card_style.set(loaded_customization.card_style.clone());
                    
                    set_custom_css.set(loaded_customization.custom_css.clone().unwrap_or_default());
                    set_custom_header_html.set(loaded_customization.custom_header_html.clone().unwrap_or_default());
                    set_custom_footer_html.set(loaded_customization.custom_footer_html.clone().unwrap_or_default());
                    
                    set_customization.set(Some(loaded_customization));
                    set_loading.set(false);
                },
                Err(e) => {
                    set_error.set(Some(format!("Failed to load site customization: {}", e)));
                    set_loading.set(false);
                }
            }
        });
    });
    
    // Save customizations
    let save_customization = move |ev: SubmitEvent| {
        ev.prevent_default();
        
        set_saving.set(true);
        set_error.set(None);
        set_success.set(None);
        
        let updated_customization = SiteCustomization {
            site_name: site_name(),
            site_tagline: if site_tagline().is_empty() { None } else { Some(site_tagline()) },
            site_description: if site_description().is_empty() { None } else { Some(site_description()) },
            site_logo_url: if site_logo_url().is_empty() { None } else { Some(site_logo_url()) },
            site_favicon_url: if site_favicon_url().is_empty() { None } else { Some(site_favicon_url()) },
            
            primary_color: primary_color(),
            secondary_color: secondary_color(),
            success_color: success_color(),
            info_color: info_color(),
            warning_color: warning_color(),
            danger_color: danger_color(),
            background_color: background_color(),
            text_color: text_color(),
            
            heading_font: heading_font(),
            body_font: body_font(),
            code_font: code_font(),
            base_font_size: base_font_size(),
            
            border_radius: border_radius(),
            button_style: button_style(),
            card_style: card_style(),
            
            custom_css: if custom_css().is_empty() { None } else { Some(custom_css()) },
            custom_header_html: if custom_header_html().is_empty() { None } else { Some(custom_header_html()) },
            custom_footer_html: if custom_footer_html().is_empty() { None } else { Some(custom_footer_html()) },
        };
        
        spawn_local(async move {
            match AdminService::update_site_customization(updated_customization).await {
                Ok(new_customization) => {
                    set_customization.set(Some(new_customization));
                    set_success.set(Some("Site customization updated successfully".to_string()));
                },
                Err(e) => {
                    set_error.set(Some(format!("Failed to update site customization: {}", e)));
                }
            }
            set_saving.set(false);
        });
    };
    
    // Reset to defaults
    let reset_to_defaults = move |_| {
        if !window().confirm_with_message("Are you sure you want to reset all customizations to default values?").unwrap_or(false) {
            return;
        }
        
        set_saving.set(true);
        set_error.set(None);
        set_success.set(None);
        
        spawn_local(async move {
            match AdminService::reset_site_customization().await {
                Ok(default_customization) => {
                    // Update all form fields with default values
                    set_site_name.set(default_customization.site_name.clone());
                    set_site_tagline.set(default_customization.site_tagline.clone().unwrap_or_default());
                    set_site_description.set(default_customization.site_description.clone().unwrap_or_default());
                    set_site_logo_url.set(default_customization.site_logo_url.clone().unwrap_or_default());
                    set_site_favicon_url.set(default_customization.site_favicon_url.clone().unwrap_or_default());
                    
                    set_primary_color.set(default_customization.primary_color.clone());
                    set_secondary_color.set(default_customization.secondary_color.clone());
                    set_success_color.set(default_customization.success_color.clone());
                    set_info_color.set(default_customization.info_color.clone());
                    set_warning_color.set(default_customization.warning_color.clone());
                    set_danger_color.set(default_customization.danger_color.clone());
                    set_background_color.set(default_customization.background_color.clone());
                    set_text_color.set(default_customization.text_color.clone());
                    
                    set_heading_font.set(default_customization.heading_font.clone());
                    set_body_font.set(default_customization.body_font.clone());
                    set_code_font.set(default_customization.code_font.clone());
                    set_base_font_size.set(default_customization.base_font_size);
                    
                    set_border_radius.set(default_customization.border_radius);
                    set_button_style.set(default_customization.button_style.clone());
                    set_card_style.set(default_customization.card_style.clone());
                    
                    set_custom_css.set(default_customization.custom_css.clone().unwrap_or_default());
                    set_custom_header_html.set(default_customization.custom_header_html.clone().unwrap_or_default());
                    set_custom_footer_html.set(default_customization.custom_footer_html.clone().unwrap_or_default());
                    
                    set_customization.set(Some(default_customization));
                    set_success.set(Some("Site customization reset to defaults successfully".to_string()));
                },
                Err(e) => {
                    set_error.set(Some(format!("Failed to reset site customization: {}", e)));
                }
            }
            set_saving.set(false);
        });
    };

    view! {
        <div class="site-customization">
            {move || if !is_admin() {
                view! {
                    <div class="alert alert-danger">
                        "You don't have permission to access this page."
                    </div>
                }
            } else {
                view! {
                    <div>
                        <h1 class="mb-4">"Site Customization"</h1>
                        
                        {move || error().map(|err| view! { <div class="alert alert-danger mb-4">{err}</div> })}
                        {move || success().map(|msg| view! { <div class="alert alert-success mb-4">{msg}</div> })}
                        
                        {move || if loading() {
                            view! { <div class="d-flex justify-content-center p-5"><div class="spinner-border" role="status"></div></div> }
                        } else {
                            view! {
                                <ul class="nav nav-tabs mb-4" role="tablist">
                                    <li class="nav-item" role="presentation">
                                        <button
                                            class="nav-link active"
                                            id="branding-tab"
                                            data-bs-toggle="tab"
                                            data-bs-target="#branding"
                                            type="button"
                                            role="tab"
                                            aria-controls="branding"
                                            aria-selected="true"
                                        >
                                            "Branding"
                                        </button>
                                    </li>
                                    <li class="nav-item" role="presentation">
                                        <button
                                            class="nav-link"
                                            id="colors-tab"
                                            data-bs-toggle="tab"
                                            data-bs-target="#colors"
                                            type="button"
                                            role="tab"
                                            aria-controls="colors"
                                            aria-selected="false"
                                        >
                                            "Colors"
                                        </button>
                                    </li>
                                    <li class="nav-item" role="presentation">
                                        <button
                                            class="nav-link"
                                            id="typography-tab"
                                            data-bs-toggle="tab"
                                            data-bs-target="#typography"
                                            type="button"
                                            role="tab"
                                            aria-controls="typography"
                                            aria-selected="false"
                                        >
                                            "Typography"
                                        </button>
                                    </li>
                                    <li class="nav-item" role="presentation">
                                        <button
                                            class="nav-link"
                                            id="components-tab"
                                            data-bs-toggle="tab"
                                            data-bs-target="#components"
                                            type="button"
                                            role="tab"
                                            aria-controls="components"
                                            aria-selected="false"
                                        >
                                            "Components"
                                        </button>
                                    </li>
                                    <li class="nav-item" role="presentation">
                                        <button
                                            class="nav-link"
                                            id="advanced-tab"
                                            data-bs-toggle="tab"
                                            data-bs-target="#advanced"
                                            type="button"
                                            role="tab"
                                            aria-controls="advanced"
                                            aria-selected="false"
                                        >
                                            "Advanced"
                                        </button>
                                    </li>
                                </ul>
                                
                                <form on:submit=save_customization>
                                    <div class="tab-content">
                                        // Branding Tab
                                        <div class="tab-pane fade show active" id="branding" role="tabpanel" aria-labelledby="branding-tab">
                                            <div class="card mb-4">
                                                <div class="card-header">
                                                    <h5 class="mb-0">"Site Branding"</h5>
                                                </div>
                                                <div class="card-body">
                                                    <div class="mb-3">
                                                        <label for="siteName" class="form-label">"Site Name"</label>
                                                        <input
                                                            id="siteName"
                                                            type="text"
                                                            class="form-control"
                                                            prop:value=move || site_name()
                                                            on:input=move |ev| set_site_name.set(event_target_value(&ev))
                                                            required
                                                        />
                                                    </div>
                                                    
                                                    <div class="mb-3">
                                                        <label for="siteTagline" class="form-label">"Site Tagline"</label>
                                                        <input
                                                            id="siteTagline"
                                                            type="text"
                                                            class="form-control"
                                                            prop:value=move || site_tagline()
                                                            on:input=move |ev| set_site_tagline.set(event_target_value(&ev))
                                                            placeholder="Optional short description"
                                                        />
                                                    </div>
                                                    
                                                    <div class="mb-3">
                                                        <label for="siteDescription" class="form-label">"Site Description"</label>
                                                        <textarea
                                                            id="siteDescription"
                                                            class="form-control"
                                                            prop:value=move || site_description()
                                                            on:input=move |ev| set_site_description.set(event_target_value(&ev))
                                                            placeholder="Used for SEO meta description"
                                                            rows="3"
                                                        ></textarea>
                                                    </div>
                                                    
                                                    <div class="row mb-3">
                                                        <div class="col-md-6">
                                                            <label for="siteLogo" class="form-label">"Site Logo URL"</label>
                                                            <input
                                                                id="siteLogo"
                                                                type="text"
                                                                class="form-control"
                                                                prop:value=move || site_logo_url()
                                                                on:input=move |ev| set_site_logo_url.set(event_target_value(&ev))
                                                                placeholder="https://example.com/logo.png"
                                                            />
                                                            <div class="form-text">
                                                                "Recommended size: 200x50 pixels"
                                                            </div>
                                                        </div>
                                                        <div class="col-md-6">
                                                            <label for="siteFavicon" class="form-label">"Site Favicon URL"</label>
                                                            <input
                                                                id="siteFavicon"
                                                                type="text"
                                                                class="form-control"
                                                                prop:value=move || site_favicon_url()
                                                                on:input=move |ev| set_site_favicon_url.set(event_target_value(&ev))
                                                                placeholder="https://example.com/favicon.ico"
                                                            />
                                                            <div class="form-text">
                                                                "Recommended size: 32x32 pixels"
                                                            </div>
                                                        </div>
                                                    </div>
                                                </div>
                                            </div>
                                        </div>
                                        
                                        // Colors Tab
                                        <div class="tab-pane fade" id="colors" role="tabpanel" aria-labelledby="colors-tab">
                                            <div class="card mb-4">
                                                <div class="card-header">
                                                    <h5 class="mb-0">"Color Scheme"</h5>
                                                </div>
                                                <div class="card-body">
                                                    <div class="row mb-4">
                                                        <div class="col-md-6">
                                                            <label for="primaryColor" class="form-label">"Primary Color"</label>
                                                            <div class="input-group">
                                                                <input
                                                                    id="primaryColor"
                                                                    type="color"
                                                                    class="form-control form-control-color"
                                                                    prop:value=move || primary_color()
                                                                    on:input=move |ev| set_primary_color.set(event_target_value(&ev))
                                                                />
                                                                <input
                                                                    type="text"
                                                                    class="form-control"
                                                                    prop:value=move || primary_color()
                                                                    on:input=move |ev| set_primary_color.set(event_target_value(&ev))
                                                                />
                                                            </div>
                                                        </div>
                                                        <div class="col-md-6">
                                                            <label for="secondaryColor" class="form-label">"Secondary Color"</label>
                                                            <div class="input-group">
                                                                <input
                                                                    id="secondaryColor"
                                                                    type="color"
                                                                    class="form-control form-control-color"
                                                                    prop:value=move || secondary_color()
                                                                    on:input=move |ev| set_secondary_color.set(event_target_value(&ev))
                                                                />
                                                                <input
                                                                    type="text"
                                                                    class="form-control"
                                                                    prop:value=move || secondary_color()
                                                                    on:input=move |ev| set_secondary_color.set(event_target_value(&ev))
                                                                />
                                                            </div>
                                                        </div>
                                                    </div>
                                                    
                                                    <div class="row mb-4">
                                                        <div class="col-md-3">
                                                            <label for="successColor" class="form-label">"Success Color"</label>
                                                            <div class="input-group">
                                                                <input
                                                                    id="successColor"
                                                                    type="color"
                                                                    class="form-control form-control-color"
                                                                    prop:value=move || success_color()
                                                                    on:input=move |ev| set_success_color.set(event_target_value(&ev))
                                                                />
                                                                <input
                                                                    type="text"
                                                                    class="form-control"
                                                                    prop:value=move || success_color()
                                                                    on:input=move |ev| set_success_color.set(event_target_value(&ev))
                                                                />
                                                            </div>
                                                        </div>
                                                        <div class="col-md-3">
                                                            <label for="infoColor" class="form-label">"Info Color"</label>
                                                            <div class="input-group">
                                                                <input
                                                                    id="infoColor"
                                                                    type="color"
                                                                    class="form-control form-control-color"
                                                                    prop:value=move || info_color()
                                                                    on:input=move |ev| set_info_color.set(event_target_value(&ev))
                                                                />
                                                                <input
                                                                    type="text"
                                                                    class="form-control"
                                                                    prop:value=move || info_color()
                                                                    on:input=move |ev| set_info_color.set(event_target_value(&ev))
                                                                />
                                                            </div>
                                                        </div>
                                                        <div class="col-md-3">
                                                            <label for="warningColor" class="form-label">"Warning Color"</label>
                                                            <div class="input-group">
                                                                <input
                                                                    id="warningColor"
                                                                    type="color"
                                                                    class="form-control form-control-color"
                                                                    prop:value=move || warning_color()
                                                                    on:input=move |ev| set_warning_color.set(event_target_value(&ev))
                                                                />
                                                                <input
                                                                    type="text"
                                                                    class="form-control"
                                                                    prop:value=move || warning_color()
                                                                    on:input=move |ev| set_warning_color.set(event_target_value(&ev))
                                                                />
                                                            </div>
                                                        </div>
                                                        <div class="col-md-3">
                                                            <label for="dangerColor" class="form-label">"Danger Color"</label>
                                                            <div class="input-group">
                                                                <input
                                                                    id="dangerColor"
                                                                    type="color"
                                                                    class="form-control form-control-color"
                                                                    prop:value=move || danger_color()
                                                                    on:input=move |ev| set_danger_color.set(event_target_value(&ev))
                                                                />
                                                                <input
                                                                    type="text"
                                                                    class="form-control"
                                                                    prop:value=move || danger_color()
                                                                    on:input=move |ev| set_danger_color.set(event_target_value(&ev))
                                                                />
                                                            </div>
                                                        </div>
                                                    </div>
                                                    
                                                    <div class="row mb-4">
                                                        <div class="col-md-6">
                                                            <label for="backgroundColor" class="form-label">"Background Color"</label>
                                                            <div class="input-group">
                                                                <input
                                                                    id="backgroundColor"
                                                                    type="color"
                                                                    class="form-control form-control-color"
                                                                    prop:value=move || background_color()
                                                                    on:input=move |ev| set_background_color.set(event_target_value(&ev))
                                                                />
                                                                <input
                                                                    type="text"
                                                                    class="form-control"
                                                                    prop:value=move || background_color()
                                                                    on:input=move |ev| set_background_color.set(event_target_value(&ev))
                                                                />
                                                            </div>
                                                        </div>
                                                        <div class="col-md-6">
                                                            <label for="textColor" class="form-label">"Text Color"</label>
                                                            <div class="input-group">
                                                                <input
                                                                    id="textColor"
                                                                    type="color"
                                                                    class="form-control form-control-color"
                                                                    prop:value=move || text_color()
                                                                    on:input=move |ev| set_text_color.set(event_target_value(&ev))
                                                                />
                                                                <input
                                                                    type="text"
                                                                    class="form-control"
                                                                    prop:value=move || text_color()
                                                                    on:input=move |ev| set_text_color.set(event_target_value(&ev))
                                                                />
                                                            </div>
                                                        </div>
                                                    </div>
                                                    
                                                    <div class="mt-4">
                                                        <h6>"Color Preview"</h6>
                                                        <div class="row">
                                                            <div class="col-md-6">
                                                                <div class="color-preview">
                                                                    <div class="p-3 mb-2" style=move || format!("background-color: {}; color: white;", primary_color())>
                                                                        "Primary"
                                                                    </div>
                                                                    <div class="p-3 mb-2" style=move || format!("background-color: {}; color: white;", secondary_color())>
                                                                        "Secondary"
                                                                    </div>
                                                                    <div class="p-3 mb-2" style=move || format!("background-color: {}; color: white;", success_color())>
                                                                        "Success"
                                                                    </div>
                                                                    <div class="p-3 mb-2" style=move || format!("background-color: {}; color: white;", danger_color())>
                                                                        "Danger"
                                                                    </div>
                                                                </div>
                                                            </div>
                                                            <div class="col-md-6">
                                                                <div class="p-3 mb-2" style=move || format!("background-color: {}; color: white;", info_color())>
                                                                    "Info"
                                                                </div>
                                                                <div class="p-3 mb-2" style=move || format!("background-color: {}; color: black;", warning_color())>
                                                                    "Warning"
                                                                </div>
                                                                <div class="p-3 mb-2" style=move || format!("background-color: {}; color: {};", background_color(), text_color())>
                                                                    "Background & Text"
                                                                </div>
                                                            </div>
                                                        </div>
                                                    </div>
                                                </div>
                                            </div>
                                        </div>
                                        
                                        // Typography Tab
                                        <div class="tab-pane fade" id="typography" role="tabpanel" aria-labelledby="typography-tab">
                                            <div class="card mb-4">
                                                <div class="card-header">
                                                    <h5 class="mb-0">"Typography Settings"</h5>
                                                </div>
                                                <div class="card-body">
                                                    <div class="mb-3">
                                                        <label for="headingFont" class="form-label">"Heading Font"</label>
                                                        <input
                                                            id="headingFont"
                                                            type="text"
                                                            class="form-control"
                                                            prop:value=move || heading_font()
                                                            on:input=move |ev| set_heading_font.set(event_target_value(&ev))
                                                        />
                                                        <div class="form-text">
                                                            "CSS font-family value for headings"
                                                        </div>
                                                    </div>
                                                    
                                                    <div class="mb-3">
                                                        <label for="bodyFont" class="form-label">"Body Font"</label>
                                                        <input
                                                            id="bodyFont"
                                                            type="text"
                                                            class="form-control"
                                                            prop:value=move || body_font()
                                                            on:input=move |ev| set_body_font.set(event_target_value(&ev))
                                                        />
                                                        <div class="form-text">
                                                            "CSS font-family value for body text"
                                                        </div>
                                                    </div>
                                                    
                                                    <div class="mb-3">
                                                        <label for="codeFont" class="form-label">"Code Font"</label>
                                                        <input
                                                            id="codeFont"
                                                            type="text"
                                                            class="form-control"
                                                            prop:value=move || code_font()
                                                            on:input=move |ev| set_code_font.set(event_target_value(&ev))
                                                        />
                                                        <div class="form-text">
                                                            "CSS font-family value for code blocks"
                                                        </div>
                                                    </div>
                                                    
                                                    <div class="mb-3">
                                                        <label for="baseFontSize" class="form-label">"Base Font Size"</label>
                                                        <div class="row align-items-center">
                                                            <div class="col-md-6">
                                                                <input
                                                                    id="baseFontSize"
                                                                    type="range"
                                                                    class="form-range"
                                                                    min="12"
                                                                    max="20"
                                                                    step="1"
                                                                    prop:value=move || base_font_size()
                                                                    on:input=move |ev| {
                                                                        if let Ok(size) = event_target_value(&ev).parse::<i32>() {
                                                                            set_base_font_size.set(size);
                                                                        }
                                                                    }
                                                                />
                                                            </div>
                                                            <div class="col-md-6">
                                                                <div class="input-group">
                                                                    <input
                                                                        type="number"
                                                                        class="form-control"
                                                                        prop:value=move || base_font_size()
                                                                        on:input=move |ev| {
                                                                            if let Ok(size) = event_target_value(&ev).parse::<i32>() {
                                                                                if size >= 12 && size <= 20 {
                                                                                    set_base_font_size.set(size);
                                                                                }
                                                                            }
                                                                        }
                                                                        min="12"
                                                                        max="20"
                                                                    />
                                                                    <span class="input-group-text">px</span>
                                                                </div>
                                                            </div>
                                                        </div>
                                                    </div>
                                                    
                                                    <div class="mt-4">
                                                        <h6>"Typography Preview"</h6>
                                                        <div class="typography-preview p-3 border rounded" style=move || {
                                                            format!("font-family: {}; font-size: {}px;", body_font(), base_font_size())
                                                        }>
                                                            <h1 style=move || format!("font-family: {};", heading_font())>"Heading 1"</h1>
                                                            <h2 style=move || format!("font-family: {};", heading_font())>"Heading 2"</h2>
                                                            <h3 style=move || format!("font-family: {};", heading_font())>"Heading 3"</h3>
                                                            <p>"This is a paragraph with <strong>bold</strong> and <em>italic</em> text to preview your typography settings."</p>
                                                            <p><code style=move || format!("font-family: {};", code_font())>"This is a code sample"</code></p>
                                                        </div>
                                                    </div>
                                                </div>
                                            </div>
                                        </div>
                                        
                                        // Components Tab
                                        <div class="tab-pane fade" id="components" role="tabpanel" aria-labelledby="components-tab">
                                            <div class="card mb-4">
                                                <div class="card-header">
                                                    <h5 class="mb-0">"Component Styling"</h5>
                                                </div>
                                                <div class="card-body">
                                                    <div class="mb-3">
                                                        <label for="borderRadius" class="form-label">"Border Radius"</label>
                                                        <div class="row align-items-center">
                                                            <div class="col-md-6">
                                                                <input
                                                                    id="borderRadius"
                                                                    type="range"
                                                                    class="form-range"
                                                                    min="0"
                                                                    max="1"
                                                                    step="0.05"
                                                                    prop:value=move || border_radius()
                                                                    on:input=move |ev| {
                                                                        if let Ok(radius) = event_target_value(&ev).parse::<f64>() {
                                                                            set_border_radius.set(radius);
                                                                        }
                                                                    }
                                                                />
                                                            </div>
                                                            <div class="col-md-6">
                                                                <div class="input-group">
                                                                    <input
                                                                        type="number"
                                                                        class="form-control"
                                                                        prop:value=move || format!("{:.2}", border_radius())
                                                                        on:input=move |ev| {
                                                                            if let Ok(radius) = event_target_value(&ev).parse::<f64>() {
                                                                                if radius >= 0.0 && radius <= 1.0 {
                                                                                    set_border_radius.set(radius);
                                                                                }
                                                                            }
                                                                        }
                                                                        min="0"
                                                                        max="1"
                                                                        step="0.05"
                                                                    />
                                                                    <span class="input-group-text">rem</span>
                                                                </div>
                                                            </div>
                                                        </div>
                                                    </div>
                                                    
                                                    <div class="mb-3">
                                                        <label for="buttonStyle" class="form-label">"Button Style"</label>
                                                        <select
                                                            id="buttonStyle"
                                                            class="form-select"
                                                            prop:value=move || button_style()
                                                            on:change=move |ev| set_button_style.set(event_target_value(&ev))
                                                        >
                                                            <option value="default">"Default"</option>
                                                            <option value="rounded">"Rounded"</option>
                                                            <option value="pill">"Pill"</option>
                                                            <option value="square">"Square"</option>
                                                        </select>
                                                    </div>
                                                    
                                                    <div class="mb-4">
                                                        <label for="cardStyle" class="form-label">"Card Style"</label>
                                                        <select
                                                            id="cardStyle"
                                                            class="form-select"
                                                            prop:value=move || card_style()
                                                            on:change=move |ev| set_card_style.set(event_target_value(&ev))
                                                        >
                                                            <option value="default">"Default"</option>
                                                            <option value="flat">"Flat"</option>
                                                            <option value="bordered">"Bordered"</option>
                                                            <option value="shadowed">"Shadowed"</option>
                                                        </select>
                                                    </div>
                                                    
                                                    <div class="mt-4">
                                                        <h6>"Component Preview"</h6>
                                                        <div class="row">
                                                            <div class="col-md-6">
                                                                <div class="mb-3">
                                                                    <label class="form-label">"Buttons"</label>
                                                                    <div class="d-flex gap-2 flex-wrap">
                                                                        <button 
                                                                            class="btn btn-primary" 
                                                                            style=move || {
                                                                                let radius = match button_style().as_str() {
                                                                                    "rounded" => "1rem",
                                                                                    "pill" => "2rem",
                                                                                    "square" => "0",
                                                                                    _ => format!("{}rem", border_radius())
                                                                                };
                                                                                format!("border-radius: {}", radius)
                                                                            }
                                                                        >
                                                                            "Primary"
                                                                        </button>
                                                                        <button 
                                                                            class="btn btn-secondary"
                                                                            style=move || {
                                                                                let radius = match button_style().as_str() {
                                                                                    "rounded" => "1rem",
                                                                                    "pill" => "2rem",
                                                                                    "square" => "0",
                                                                                    _ => format!("{}rem", border_radius())
                                                                                };
                                                                                format!("border-radius: {}", radius)
                                                                            }
                                                                        >
                                                                            "Secondary"
                                                                        </button>
                                                                        <button 
                                                                            class="btn btn-success"
                                                                            style=move || {
                                                                                let radius = match button_style().as_str() {
                                                                                    "rounded" => "1rem",
                                                                                    "pill" => "2rem",
                                                                                    "square" => "0",
                                                                                    _ => format!("{}rem", border_radius())
                                                                                };
                                                                                format!("border-radius: {}", radius)
                                                                            }
                                                                        >
                                                                            "Success"
                                                                        </button>
                                                                    </div>
                                                                </div>
                                                            </div>
                                                            <div class="col-md-6">
                                                                <div
                                                                    class="card"
                                                                    style=move || {
                                                                        let radius = format!("{}rem", border_radius());
                                                                        let shadow = match card_style().as_str() {
                                                                            "flat" => "none",
                                                                            "shadowed" => "0 .5rem 1rem rgba(0,0,0,.15)",
                                                                            _ => "0 .125rem .25rem rgba(0,0,0,.075)"
                                                                        };
                                                                        let border = match card_style().as_str() {
                                                                            "bordered" => "1px solid rgba(0,0,0,.2)",
                                                                            "flat" => "none",
                                                                            _ => "1px solid rgba(0,0,0,.125)"
                                                                        };
                                                                        format!("border-radius: {}; box-shadow: {}; border: {}", radius, shadow, border)
                                                                    }
                                                                >
                                                                    <div class="card-body">
                                                                        <h5 class="card-title">"Card Preview"</h5>
                                                                        <p class="card-text">"This card shows your selected styling options."</p>
                                                                    </div>
                                                                </div>
                                                            </div>
                                                        </div>
                                                        <div class="row mt-3">
                                                            <div class="col-md-6">
                                                                <div
                                                                    class="form-control"
                                                                    style=move || format!("border-radius: {}rem", border_radius())
                                                                >
                                                                    "Input field preview"
                                                                </div>
                                                            </div>
                                                        </div>
                                                    </div>
                                                </div>
                                            </div>
                                        </div>
                                        
                                        // Advanced Tab
                                        <div class="tab-pane fade" id="advanced" role="tabpanel" aria-labelledby="advanced-tab">
                                            <div class="card mb-4">
                                                <div class="card-header">
                                                    <h5 class="mb-0">"Advanced Customization"</h5>
                                                </div>
                                                <div class="card-body">
                                                    <div class="mb-3">
                                                        <label for="customCSS" class="form-label">"Custom CSS"</label>
                                                        <textarea
                                                            id="customCSS"
                                                            class="form-control font-monospace"
                                                            rows="8"
                                                            prop:value=move || custom_css()
                                                            on:input=move |ev| set_custom_css.set(event_target_value(&ev))
                                                            placeholder="/* Add your custom CSS here */"
                                                            style="font-size: 14px;"
                                                        ></textarea>
                                                        <div class="form-text">
                                                            "Add custom CSS styles that will be applied site-wide"
                                                        </div>
                                                    </div>
                                                    
                                                    <div class="mb-3">
                                                        <label for="customHeaderHTML" class="form-label">"Custom Header HTML"</label>
                                                        <textarea
                                                            id="customHeaderHTML"
                                                            class="form-control font-monospace"
                                                            rows="5"
                                                            prop:value=move || custom_header_html()
                                                            on:input=move |ev| set_custom_header_html.set(event_target_value(&ev))
                                                            placeholder="<!-- HTML to include in the <head> section -->"
                                                            style="font-size: 14px;"
                                                        ></textarea>
                                                        <div class="form-text">
                                                            "HTML that will be included in the <head> section of every page"
                                                        </div>
                                                    </div>
                                                    
                                                    <div class="mb-3">
                                                        <label for="customFooterHTML" class="form-label">"Custom Footer HTML"</label>
                                                        <textarea
                                                            id="customFooterHTML"
                                                            class="form-control font-monospace"
                                                            rows="5"
                                                            prop:value=move || custom_footer_html()
                                                            on:input=move |ev| set_custom_footer_html.set(event_target_value(&ev))
                                                            placeholder="<!-- HTML to include at the end of the body -->"
                                                            style="font-size: 14px;"
                                                        ></textarea>
                                                        <div class="form-text">
                                                            "HTML that will be included at the end of the <body> section of every page"
                                                        </div>
                                                    </div>
                                                </div>
                                            </div>
                                        </div>
                                    </div>
                                    
                                    <div class="d-flex justify-content-between mt-4">
                                        <button 
                                            type="button" 
                                            class="btn btn-outline-danger"
                                            on:click=reset_to_defaults
                                        >
                                            "Reset to Defaults"
                                        </button>
                                        
                                        <button 
                                            type="submit" 
                                            class="btn btn-primary"
                                            disabled=move || saving()
                                        >
                                            {move || if saving() {
                                                view! { <span class="spinner-border spinner-border-sm me-2" role="status"></span> "Saving..." }
                                            } else {
                                                view! { "Save Customization" }
                                            }}
                                        </button>
                                    </div>
                                </form>
                            }
                        }}
                    </div>
                }
            }}
        </div>
    }
}