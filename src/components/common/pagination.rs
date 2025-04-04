use leptos::*;

#[component]
pub fn Pagination(
    #[prop(into)] current_page: usize,
    #[prop(into)] total_pages: usize,
    #[prop(into)] on_page_change: Callback<usize>,
) -> impl IntoView {
    // Logic for displaying a window of page numbers
    let display_page_window = move || {
        let mut pages = Vec::new();
        
        // Always show first page
        pages.push(1);
        
        // Calculate window around current page
        let window_start = (current_page - 2).max(2);
        let window_end = (current_page + 2).min(total_pages - 1);
        
        // Add ellipsis after first page if needed
        if window_start > 2 {
            pages.push(0); // 0 indicates ellipsis
        }
        
        // Add pages in the window
        for page in window_start..=window_end {
            pages.push(page);
        }
        
        // Add ellipsis before last page if needed
        if window_end < total_pages - 1 {
            pages.push(0); // 0 indicates ellipsis
        }
        
        // Always show last page if there's more than one page
        if total_pages > 1 {
            pages.push(total_pages);
        }
        
        pages
    };

    view! {
        <nav aria-label="Page navigation">
            <ul class="pagination justify-content-center">
                // Previous button
                <li class="page-item" class:disabled=move || current_page == 1>
                    <a 
                        class="page-link" 
                        href="#"
                        aria-label="Previous"
                        on:click=move |ev| {
                            ev.prevent_default();
                            if current_page > 1 {
                                on_page_change.call(current_page - 1);
                            }
                        }
                    >
                        <span aria-hidden="true">"«"</span>
                    </a>
                </li>
                
                // Page numbers
                {move || display_page_window().into_iter().map(|page| {
                    if page == 0 {
                        // Render ellipsis
                        view! {
                            <li class="page-item disabled">
                                <span class="page-link">"…"</span>
                            </li>
                        }
                    } else {
                        // Render page number
                        view! {
                            <li class="page-item" class:active=move || page == current_page>
                                <a 
                                    class="page-link" 
                                    href="#"
                                    on:click=move |ev| {
                                        ev.prevent_default();
                                        on_page_change.call(page);
                                    }
                                >
                                    {page}
                                </a>
                            </li>
                        }
                    }
                }).collect::<Vec<_>>()}
                
                // Next button
                <li class="page-item" class:disabled=move || current_page >= total_pages>
                    <a 
                        class="page-link" 
                        href="#"
                        aria-label="Next"
                        on:click=move |ev| {
                            ev.prevent_default();
                            if current_page < total_pages {
                                on_page_change.call(current_page + 1);
                            }
                        }
                    >
                        <span aria-hidden="true">"»"</span>
                    </a>
                </li>
            </ul>
        </nav>
    }
}