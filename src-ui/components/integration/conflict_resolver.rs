use leptos::*;
use crate::models::integration::{SyncConflict, ConflictResolutionStrategy};

#[component]
pub fn ConflictResolver(
    #[prop(into)] conflict: Option<SyncConflict>,
    #[prop(into)] on_close: Callback<()>,
    #[prop(into)] on_resolve: Callback<(String, ConflictResolutionStrategy)>,
) -> impl IntoView {
    // State
    let (selected_strategy, set_selected_strategy) = create_signal(ConflictResolutionStrategy::PreferMostRecent);
    
    // Handle resolve button click
    let handle_resolve = move |_| {
        if let Some(conflict) = conflict.clone() {
            on_resolve.call((conflict.id, selected_strategy.get()));
        }
    };
    
    view! {
        <div class="modal-overlay">
            <div class="modal-container">
                <div class="modal-header">
                    <h3>"Resolve Sync Conflict"</h3>
                    <button class="close-button" on:click=move |_| on_close.call(())>
                        <i class="icon-close"></i>
                    </button>
                </div>
                
                <div class="modal-body">
                    {move || match &conflict {
                        Some(c) => view! {
                            <div class="conflict-details">
                                <div class="conflict-item">
                                    <span class="label">"Entity Type:"</span>
                                    <span class="value">{&c.entity_type}</span>
                                </div>
                                
                                <div class="conflict-item">
                                    <span class="label">"Title:"</span>
                                    <span class="value">{&c.title}</span>
                                </div>
                                
                                <div class="conflict-comparison">
                                    <div class="canvas-version">
                                        <h4>"Canvas Version"</h4>
                                        <div class="version-details">
                                            <p>"Last Updated: "{&c.canvas_updated_at}</p>
                                            <div class="content-preview">
                                                <pre>{&c.canvas_content}</pre>
                                            </div>
                                        </div>
                                    </div>
                                    
                                    <div class="discourse-version">
                                        <h4>"Discourse Version"</h4>
                                        <div class="version-details">
                                            <p>"Last Updated: "{&c.discourse_updated_at}</p>
                                            <div class="content-preview">
                                                <pre>{&c.discourse_content}</pre>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                                
                                <div class="resolution-options">
                                    <h4>"Resolution Strategy"</h4>
                                    
                                    <div class="radio-group">
                                        <label>
                                            <input 
                                                type="radio" 
                                                name="resolution-strategy" 
                                                value="prefer-canvas"
                                                checked={selected_strategy.get() == ConflictResolutionStrategy::PreferCanvas}
                                                on:change=move |_| set_selected_strategy.set(ConflictResolutionStrategy::PreferCanvas)
                                            />
                                            <span>"Use Canvas Version"</span>
                                        </label>
                                        
                                        <label>
                                            <input 
                                                type="radio" 
                                                name="resolution-strategy" 
                                                value="prefer-discourse"
                                                checked={selected_strategy.get() == ConflictResolutionStrategy::PreferDiscourse}
                                                on:change=move |_| set_selected_strategy.set(ConflictResolutionStrategy::PreferDiscourse)
                                            />
                                            <span>"Use Discourse Version"</span>
                                        </label>
                                        
                                        <label>
                                            <input 
                                                type="radio" 
                                                name="resolution-strategy" 
                                                value="prefer-most-recent"
                                                checked={selected_strategy.get() == ConflictResolutionStrategy::PreferMostRecent}
                                                on:change=move |_| set_selected_strategy.set(ConflictResolutionStrategy::PreferMostRecent)
                                            />
                                            <span>"Use Most Recent Version"</span>
                                        </label>
                                        
                                        <label>
                                            <input 
                                                type="radio" 
                                                name="resolution-strategy" 
                                                value="merge-prefer-canvas"
                                                checked={selected_strategy.get() == ConflictResolutionStrategy::MergePreferCanvas}
                                                on:change=move |_| set_selected_strategy.set(ConflictResolutionStrategy::MergePreferCanvas)
                                            />
                                            <span>"Merge (Prefer Canvas for Conflicts)"</span>
                                        </label>
                                        
                                        <label>
                                            <input 
                                                type="radio" 
                                                name="resolution-strategy" 
                                                value="merge-prefer-discourse"
                                                checked={selected_strategy.get() == ConflictResolutionStrategy::MergePreferDiscourse}
                                                on:change=move |_| set_selected_strategy.set(ConflictResolutionStrategy::MergePreferDiscourse)
                                            />
                                            <span>"Merge (Prefer Discourse for Conflicts)"</span>
                                        </label>
                                    </div>
                                </div>
                            </div>
                        },
                        None => view! { <p>"No conflict data available."</p> }
                    }}
                </div>
                
                <div class="modal-footer">
                    <button class="btn btn-secondary" on:click=move |_| on_close.call(())>
                        "Cancel"
                    </button>
                    
                    <button 
                        class="btn btn-primary" 
                        on:click=handle_resolve
                        disabled={conflict.is_none()}
                    >
                        "Resolve Conflict"
                    </button>
                </div>
            </div>
        </div>
    }
}
