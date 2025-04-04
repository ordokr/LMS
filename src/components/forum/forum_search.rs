use leptos::*;
use leptos_router::{use_location, use_navigate};
use web_sys::SubmitEvent;
use crate::models::search::{SearchRequest, SearchResponse, SearchResult, SearchFilters};
use crate::models::forum::Category;
use crate::services::search::SearchService;
use crate::services::forum::ForumService;
use crate::components::common::pagination::Pagination;
use crate::utils::formatting::{format_datetime, format_relative_time};
use crate::components::forum::tag_cloud::TagCloud;
use crate::components::forum::tag_filter::TagFilter;

#[component]
pub fn ForumSearch() -> impl IntoView {
    let location = use_location();
    let navigate = use_navigate();
    
    // Query params
    let query = move || {
        location.query.get().get("q").cloned().unwrap_or_default()
    };
    
    let type_filter = move || {
        location.query.get().get("type").cloned()
    };
    
    let category_filter = move || {
        location.query.get().get("category").and_then(|c| c.parse::<i64>().ok())
    };
    
    let user_filter = move || {
        location.query.get().get("user").and_then(|u| u.parse::<i64>().ok())
    };
    
    let tags_filter = move || {
        location.query.get().get("tags").and_then(|t| {
            if t.is_empty() { None } else { Some(t.split(',').map(String::from).collect::<Vec<_>>()) }
        })
    };
    
    let date_from_filter = move || {
        location.query.get().get("date_from").and_then(|d| {
            chrono::DateTime::parse_from_rfc3339(d).ok().map(|dt| dt.with_timezone(&chrono::Utc))
        })
    };
    
    let date_to_filter = move || {
        location.query.get().get("date_to").and_then(|d| {
            chrono::DateTime::parse_from_rfc3339(d).ok().map(|dt| dt.with_timezone(&chrono::Utc))
        })
    };
    
    let sort_by = move || {
        location.query.get().get("sort").cloned().unwrap_or_else(|| "relevance".to_string())
    };
    
    let page = move || {
        location.query.get().get("page").and_then(|p| p.parse::<usize>().ok()).unwrap_or(1)
    };
    
    // State signals
    let (search_response, set_search_response) = create_signal(None::<SearchResponse>);
    let (loading, set_loading) = create_signal(false);
    let (error, set_error) = create_signal(None::<String>);
    
    // Categories for filter dropdown
    let (categories, set_categories) = create_signal(Vec::<Category>::new());
    
    // Form state for advanced search
    let (search_query, set_search_query) = create_signal(String::new());
    let (selected_type, set_selected_type) = create_signal(None::<String>);
    let (selected_category, set_selected_category) = create_signal(None::<i64>);
    let (selected_user, set_selected_user) = create_signal(None::<i64>);
    let (selected_tags, set_selected_tags) = create_signal(String::new());
    let (selected_date_from, set_selected_date_from) = create_signal(String::new());
    let (selected_date_to, set_selected_date_to) = create_signal(String::new());
    let (selected_sort, set_selected_sort) = create_signal("relevance".to_string());
    let (show_advanced_search, set_show_advanced_search) = create_signal(false);
    let (selected_tag_names, set_selected_tag_names) = create_signal(
        tags_filter().unwrap_or_default()
    );
    
    // Load categories
    create_effect(move |_| {
        spawn_local(async move {
            match ForumService::get_categories().await {
                Ok(loaded_categories) => {
                    set_categories.set(loaded_categories);
                },
                Err(_) => {
                    // Silently fail, categories aren't critical
                }
            }
        });
    });
    
    // Initialize form with URL params
    create_effect(move |_| {
        set_search_query.set(query());
        set_selected_type.set(type_filter());
        set_selected_category.set(category_filter());
        set_selected_user.set(user_filter());
        
        if let Some(tags) = tags_filter() {
            set_selected_tags.set(tags.join(", "));
        }
        
        if let Some(date_from) = date_from_filter() {
            set_selected_date_from.set(date_from.format("%Y-%m-%d").to_string());
        }
        
        if let Some(date_to) = date_to_filter() {
            set_selected_date_to.set(date_to.format("%Y-%m-%d").to_string());
        }
        
        set_selected_sort.set(sort_by());
    });
    
    // Perform search when URL parameters change
    create_effect(move |_| {
        let q = query();
        if q.trim().is_empty() {
            return;
        }
        
        let search_request = SearchRequest {
            query: q,
            filter_type: type_filter(),
            filter_categories: category_filter().map(|c| vec![c]),
            filter_tags: tags_filter(),
            filter_date_from: date_from_filter(),
            filter_date_to: date_to_filter(),
            filter_user_id: user_filter(),
            sort_by: Some(sort_by()),
            page: page(),
            limit: 20,
        };
        
        set_loading.set(true);
        set_error.set(None);
        
        spawn_local(async move {
            match SearchService::search(&search_request).await {
                Ok(response) => {
                    set_search_response.set(Some(response));
                },
                Err(e) => {
                    set_error.set(Some(format!("Search failed: {}", e)));
                }
            }
            set_loading.set(false);
        });
    });
    
    // Handle search form submission
    let handle_search = move |ev: SubmitEvent| {
        ev.prevent_default();
        
        if search_query().trim().is_empty() {
            return;
        }
        
        // Build query parameters
        let mut params = vec![("q", search_query())];
        
        if let Some(type_) = selected_type() {
            params.push(("type", type_));
        }
        
        if let Some(category) = selected_category() {
            params.push(("category", category.to_string()));
        }
        
        if let Some(user) = selected_user() {
            params.push(("user", user.to_string()));
        }
        
        if !selected_tags().trim().is_empty() {
            let formatted_tags = selected_tags()
                .split(',')
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .collect::<Vec<_>>()
                .join(",");
            
            if !formatted_tags.is_empty() {
                params.push(("tags", formatted_tags));
            }
        }
        
        if !selected_date_from().is_empty() {
            // Convert date string to RFC3339
            if let Ok(date) = chrono::NaiveDate::parse_from_str(&selected_date_from(), "%Y-%m-%d") {
                let datetime = chrono::DateTime::<chrono::Utc>::from_utc(
                    date.and_hms_opt(0, 0, 0).unwrap(),
                    chrono::Utc,
                );
                params.push(("date_from", datetime.to_rfc3339()));
            }
        }
        
        if !selected_date_to().is_empty() {
            // Convert date string to RFC3339
            if let Ok(date) = chrono::NaiveDate::parse_from_str(&selected_date_to(), "%Y-%m-%d") {
                let datetime = chrono::DateTime::<chrono::Utc>::from_utc(
                    date.and_hms_opt(23, 59, 59).unwrap(),
                    chrono::Utc,
                );
                params.push(("date_to", datetime.to_rfc3339()));
            }
        }
        
        params.push(("sort", selected_sort()));
        params.push(("page", "1".to_string()));
        
        // Build query string
        let query_string = params.iter()
            .map(|(k, v)| format!("{}={}", k, urlencoding::encode(v)))
            .collect::<Vec<_>>()
            .join("&");
        
        // Navigate to search page with new parameters
        navigate(&format!("/forum/search?{}", query_string), Default::default());
    };
    
    // Handle pagination
    let change_page = move |new_page: usize| {
        let mut current_params = location.query.get();
        current_params.insert("page".to_string(), new_page.to_string());
        
        // Build query string
        let query_string = current_params.iter()
            .map(|(k, v)| format!("{}={}", k, urlencoding::encode(v)))
            .collect::<Vec<_>>()
            .join("&");
        
        // Navigate to search page with new page
        navigate(&format!("/forum/search?{}", query_string), Default::default());
    };
    
    // Toggle advanced search
    let toggle_advanced_search = move |_| {
        set_show_advanced_search.update(|show| *show = !*show);
    };
    
    // Clear all filters
    let clear_filters = move |_| {
        set_selected_type.set(None);
        set_selected_category.set(None);
        set_selected_user.set(None);
        set_selected_tags.set(String::new());
        set_selected_date_from.set(String::new());
        set_selected_date_to.set(String::new());
        set_selected_sort.set("relevance".to_string());
    };
    
    // Format result highlight
    let format_highlight = |result: &SearchResult| -> String {
        result.highlight.clone().unwrap_or_else(|| {
            result.excerpt
                .clone()
                .unwrap_or_else(|| "No excerpt available".to_string())
        })
    };
    
    // Get icon for result type
    let get_result_icon = |result_type: &str| -> &'static str {
        match result_type {
            "topic" => "bi-chat-text",
            "post" => "bi-file-text",
            "user" => "bi-person",
            "category" => "bi-folder",
            _ => "bi-search",
        }
    };

    // Add a handler for tag selection from TagFilter
    let handle_tags_change = move |new_tags: Vec<String>| {
        set_selected_tags.set(new_tags.join(", "));
    };

    // Add a handler for tag click from TagCloud
    let handle_tag_cloud_click = move |tag: Tag| {
        let mut current_tags = selected_tag_names.get();
        if !current_tags.contains(&tag.name) {
            current_tags.push(tag.name);
            set_selected_tag_names.set(current_tags);
            set_selected_tags.set(selected_tag_names.get().join(", "));
        }
    };

    view! {
        <div class="search-page container py-4">
            <div class="mb-4">
                <h1>"Search Results"</h1>
            </div>
            
            <div class="card mb-4">
                <div class="card-body">
                    <form on:submit=handle_search>
                        <div class="input-group mb-3">
                            <input
                                type="text"
                                class="form-control form-control-lg"
                                placeholder="Search topics, posts, and users..."
                                prop:value=move || search_query()
                                on:input=move |ev| set_search_query.set(event_target_value(&ev))
                                required
                            />
                            <button class="btn btn-primary" type="submit" disabled=move || loading()>
                                {move || if loading() {
                                    view! { <span class="spinner-border spinner-border-sm" role="status"></span> }
                                } else {
                                    view! { <i class="bi bi-search"></i> }
                                }}
                            </button>
                        </div>
                        
                        <div class="d-flex justify-content-between">
                            <button
                                type="button"
                                class="btn btn-link p-0"
                                on:click=toggle_advanced_search
                            >
                                {move || if show_advanced_search() {
                                    view! { <><i class="bi bi-caret-up"></i> "Hide Advanced Search"</> }
                                } else {
                                    view! { <><i class="bi bi-caret-down"></i> "Advanced Search"</> }
                                }}
                            </button>
                            
                            <button
                                type="button"
                                class="btn btn-link p-0"
                                on:click=clear_filters
                                style:visibility=move || {
                                    if selected_type().is_some() || selected_category().is_some() || selected_user().is_some() ||
                                       !selected_tags().is_empty() || !selected_date_from().is_empty() || !selected_date_to().is_empty() ||
                                       selected_sort() != "relevance" {
                                        "visible"
                                    } else {
                                        "hidden"
                                    }
                                }
                            >
                                <i class="bi bi-x-circle"></i> "Clear Filters"
                            </button>
                        </div>
                        
                        <div class="advanced-search mt-4" style:display=move || if show_advanced_search() { "block" } else { "none" }>
                            <div class="row g-3">
                                <div class="col-md-6">
                                    <label for="filterType" class="form-label">"Content Type"</label>
                                    <select
                                        id="filterType"
                                        class="form-select"
                                        prop:value=move || selected_type().unwrap_or_default()
                                        on:change=move |ev| {
                                            let value = event_target_value(&ev);
                                            if value.is_empty() {
                                                set_selected_type.set(None);
                                            } else {
                                                set_selected_type.set(Some(value));
                                            }
                                        }
                                    >
                                        <option value="">"All Content"</option>
                                        <option value="topics">"Topics"</option>
                                        <option value="posts">"Posts"</option>
                                        <option value="users">"Users"</option>
                                        <option value="categories">"Categories"</option>
                                    </select>
                                </div>
                                
                                <div class="col-md-6">
                                    <label for="filterCategory" class="form-label">"Category"</label>
                                    <select
                                        id="filterCategory"
                                        class="form-select"
                                        prop:value=move || selected_category().map(|c| c.to_string()).unwrap_or_default()
                                        on:change=move |ev| {
                                            let value = event_target_value(&ev);
                                            if value.is_empty() {
                                                set_selected_category.set(None);
                                            } else if let Ok(cat_id) = value.parse::<i64>() {
                                                set_selected_category.set(Some(cat_id));
                                            }
                                        }
                                    >
                                        <option value="">"All Categories"</option>
                                        {move || categories().into_iter().map(|category| {
                                            view! {
                                                <option value={category.id.to_string()}>{&category.name}</option>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </select>
                                </div>
                                
                                <div class="col-md-6">
                                    <label for="filterTags" class="form-label">"Tags (comma separated)"</label>
                                    <input
                                        type="text"
                                        id="filterTags"
                                        class="form-control"
                                        placeholder="e.g. rust, programming, help"
                                        prop:value=move || selected_tags()
                                        on:input=move |ev| set_selected_tags.set(event_target_value(&ev))
                                    />
                                </div>
                                
                                <div class="col-md-6">
                                    <label for="filterUser" class="form-label">"Posted by User ID"</label>
                                    <input
                                        type="number"
                                        id="filterUser"
                                        class="form-control"
                                        placeholder="User ID"
                                        prop:value=move || selected_user().map(|u| u.to_string()).unwrap_or_default()
                                        on:input=move |ev| {
                                            let value = event_target_value(&ev);
                                            if value.is_empty() {
                                                set_selected_user.set(None);
                                            } else if let Ok(user_id) = value.parse::<i64>() {
                                                set_selected_user.set(Some(user_id));
                                            }
                                        }
                                    />
                                </div>
                                
                                <div class="col-md-6">
                                    <label for="filterDateFrom" class="form-label">"Date From"</label>
                                    <input
                                        type="date"
                                        id="filterDateFrom"
                                        class="form-control"
                                        prop:value=move || selected_date_from()
                                        on:input=move |ev| set_selected_date_from.set(event_target_value(&ev))
                                    />
                                </div>
                                
                                <div class="col-md-6">
                                    <label for="filterDateTo" class="form-label">"Date To"</label>
                                    <input
                                        type="date"
                                        id="filterDateTo"
                                        class="form-control"
                                        prop:value=move || selected_date_to()
                                        on:input=move |ev| set_selected_date_to.set(event_target_value(&ev))
                                    />
                                </div>
                                
                                <div class="col-md-6">
                                    <label for="sortBy" class="form-label">"Sort By"</label>
                                    <select
                                        id="sortBy"
                                        class="form-select"
                                        prop:value=move || selected_sort()
                                        on:change=move |ev| set_selected_sort.set(event_target_value(&ev))
                                    >
                                        <option value="relevance">"Relevance"</option>
                                        <option value="newest">"Newest First"</option>
                                        <option value="oldest">"Oldest First"</option>
                                        <option value="most_replies">"Most Replies"</option>
                                        <option value="most_views">"Most Views"</option>
                                    </select>
                                </div>
                                
                                <div class="col-12">
                                    <button type="submit" class="btn btn-primary">
                                        <i class="bi bi-search me-1"></i>
                                        "Search with Filters"
                                    </button>
                                </div>
                            </div>
                        </div>
                    </form>
                </div>
            </div>
            
            {move || error().map(|err| view! { <div class="alert alert-danger mb-4">{err}</div> })}
            
            <div class="row">
                <div class="col-md-8">
                    <div class="search-results">
                        {move || if loading() {
                            view! { <div class="d-flex justify-content-center p-5"><div class="spinner-border" role="status"></div></div> }
                        } else if let Some(response) = search_response() {
                            view! {
                                <>
                                    <div class="mb-4">
                                        <div class="d-flex justify-content-between align-items-center">
                                            <h2>
                                                {format!("{} Results", response.total)}
                                                <small class="text-muted ms-2">
                                                    {format!("for \"{}\"", response.query)}
                                                </small>
                                            </h2>
                                            <small class="text-muted">
                                                {format!("Found in {}ms", response.execution_time_ms)}
                                            </small>
                                        </div>
                                        
                                        {(!response.results.is_empty()).then(|| {
                                            let applied_filters = &response.filters_applied;
                                            let has_filters = applied_filters.filter_type.is_some() || 
                                                            applied_filters.filter_categories.is_some() ||
                                                            applied_filters.filter_tags.is_some() ||
                                                            applied_filters.filter_date_from.is_some() ||
                                                            applied_filters.filter_date_to.is_some() ||
                                                            applied_filters.filter_user_id.is_some() ||
                                                            (applied_filters.sort_by.is_some() && applied_filters.sort_by.as_deref() != Some("relevance"));
                                            
                                            if has_filters {
                                                view! {
                                                    <div class="mt-2 d-flex flex-wrap gap-1">
                                                        <span class="text-muted me-1">"Filters:"</span>
                                                        
                                                        {applied_filters.filter_type.as_ref().map(|t| view! {
                                                            <span class="badge bg-secondary">{format!("Type: {}", t)}</span>
                                                        })}
                                                        
                                                        {applied_filters.filter_categories.as_ref().map(|cats| {
                                                            if cats.is_empty() { view! {} } else {
                                                                let category_names = cats.iter()
                                                                    .map(|&id| categories().iter()
                                                                        .find(|&c| c.id == id)
                                                                        .map(|c| c.name.clone())
                                                                        .unwrap_or_else(|| id.to_string()))
                                                                    .collect::<Vec<_>>()
                                                                    .join(", ");
                                                                
                                                                view! {
                                                                    <span class="badge bg-secondary">{format!("Category: {}", category_names)}</span>
                                                                }
                                                            }
                                                        })}
                                                        
                                                        {applied_filters.filter_tags.as_ref().map(|tags| {
                                                            if tags.is_empty() { view! {} } else {
                                                                view! {
                                                                    <span class="badge bg-secondary">{format!("Tags: {}", tags.join(", "))}</span>
                                                                }
                                                            }
                                                        })}
                                                        
                                                        {applied_filters.filter_user_id.map(|uid| view! {
                                                            <span class="badge bg-secondary">{format!("User ID: {}", uid)}</span>
                                                        })}
                                                        
                                                        {applied_filters.filter_date_from.map(|d| view! {
                                                            <span class="badge bg-secondary">
                                                                {format!("From: {}", d.format("%Y-%m-%d"))}
                                                            </span>
                                                        })}
                                                        
                                                        {applied_filters.filter_date_to.map(|d| view! {
                                                            <span class="badge bg-secondary">
                                                                {format!("To: {}", d.format("%Y-%m-%d"))}
                                                            </span>
                                                        })}
                                                        
                                                        {applied_filters.sort_by.as_ref().filter(|&s| s != "relevance").map(|s| view! {
                                                            <span class="badge bg-secondary">{format!("Sort: {}", s)}</span>
                                                        })}
                                                    </div>
                                                }
                                            } else {
                                                view! {}
                                            }
                                        })}
                                    </div>
                                    
                                    {if response.results.is_empty() {
                                        view! {
                                            <div class="alert alert-info">
                                                <i class="bi bi-info-circle me-2"></i>
                                                "No results found. Try different keywords or adjust your filters."
                                            </div>
                                        }
                                    } else {
                                        view! {
                                            <div class="list-group mb-4">
                                                {response.results.into_iter().map(|result| {
                                                    let highlight = format_highlight(&result);
                                                    let result_icon = get_result_icon(&result.result_type);
                                                    
                                                    view! {
                                                        <a href={&result.url} class="list-group-item list-group-item-action">
                                                            <div class="d-flex">
                                                                <div class="me-3">
                                                                    <i class={format!("bi {} fs-3", result_icon)}></i>
                                                                </div>
                                                                <div class="flex-grow-1">
                                                                    <div class="d-flex justify-content-between align-items-center mb-1">
                                                                        <h5 class="mb-0">
                                                                            {result.title.unwrap_or_else(|| "Untitled".to_string())}
                                                                        </h5>
                                                                        <small class="text-muted">
                                                                            {format_relative_time(result.created_at)}
                                                                        </small>
                                                                    </div>
                                                                    
                                                                    <div class="mb-2 d-flex flex-wrap gap-2 align-items-center">
                                                                        <span class="badge"
                                                                            style:background-color=move || match result.result_type.as_str() {
                                                                                "topic" => "#0d6efd",
                                                                                "post" => "#198754",
                                                                                "user" => "#6f42c1",
                                                                                "category" => "#fd7e14",
                                                                                _ => "#6c757d"
                                                                            }
                                                                        >
                                                                            {result.result_type.to_uppercase()}
                                                                        </span>
                                                                        
                                                                        {result.category_name.map(|cat| {
                                                                            view! {
                                                                                <span 
                                                                                    class="badge" 
                                                                                    style=format!("background-color: {}", result.category_color.unwrap_or_else(|| "#6c757d".to_string()))
                                                                                >
                                                                                    {cat}
                                                                                </span>
                                                                            }
                                                                        })}
                                                                        
                                                                        {result.tags.map(|tags| {
                                                                            view! {
                                                                                <>
                                                                                    {tags.iter().map(|tag| {
                                                                                        view! {
                                                                                            <span class="badge bg-secondary">{tag}</span>
                                                                                        }
                                                                                    }).collect::<Vec<_>>()}
                                                                                </>
                                                                            }
                                                                        })}
                                                                        
                                                                        {result.author.as_ref().map(|author| {
                                                                            view! {
                                                                                <span class="text-muted">
                                                                                    <i class="bi bi-person"></i>
                                                                                    {&author.username}
                                                                                </span>
                                                                            }
                                                                        })}
                                                                        
                                                                        {(result.reply_count.is_some() || result.view_count.is_some()).then(|| {
                                                                            view! {
                                                                                <span class="text-muted">
                                                                                    {result.reply_count.map(|c| view! {
                                                                                        <span class="me-2"><i class="bi bi-chat"></i> {c}</span>
                                                                                    })}
                                                                                    
                                                                                    {result.view_count.map(|c| view! {
                                                                                        <span><i class="bi bi-eye"></i> {c}</span>
                                                                                    })}
                                                                                </span>
                                                                            }
                                                                        })}
                                                                    </div>
                                                                    
                                                                    <div class="search-result-excerpt" inner_html={&highlight}></div>
                                                                    
                                                                    {result.topic_title.filter(|_| result.result_type == "post").map(|title| {
                                                                        view! {
                                                                            <div class="mt-2 text-muted">
                                                                                <small>
                                                                                    <i class="bi bi-chat-text me-1"></i>
                                                                                    {"In topic: "}{title}
                                                                                </small>
                                                                            </div>
                                                                        }
                                                                    })}
                                                                </div>
                                                            </div>
                                                        </a>
                                                    }
                                                }).collect::<Vec<_>>()}
                                            </div>
                                        }
                                    }}
                                    
                                    {(response.total > response.limit).then(|| {
                                        let total_pages = (response.total + response.limit - 1) / response.limit;
                                        
                                        view! {
                                            <div class="mb-4">
                                                <Pagination
                                                    current_page=response.page
                                                    total_pages=total_pages
                                                    on_page_change=change_page
                                                />
                                            </div>
                                        }
                                    })}
                                </>
                            }
                        } else if !query().trim().is_empty() {
                            view! {
                                <div class="alert alert-info">
                                    <i class="bi bi-info-circle me-2"></i>
                                    "Enter search terms and press Enter to search."
                                </div>
                            }
                        } else {
                            view! {}
                        }}
                    </div>
                </div>
                <div class="col-md-4">
                    <div class="card mb-3">
                        <div class="card-body">
                            <TagFilter 
                                selected_tags=selected_tag_names
                                on_change=handle_tags_change
                            />
                        </div>
                    </div>
                    
                    <div class="card">
                        <div class="card-body">
                            <TagCloud 
                                max_tags=15
                                title="Popular Tags"
                                on_tag_click=handle_tag_cloud_click
                            />
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}