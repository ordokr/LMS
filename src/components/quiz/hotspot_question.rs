use leptos::*;
use web_sys::{MouseEvent, HtmlImageElement};
use std::rc::Rc;

use crate::models::quiz::{Question, Answer, Hotspot, Point, HotspotShape};

#[component]
pub fn HotspotQuestion(
    #[prop(into)] question: Signal<Question>,
    #[prop(into)] on_answer: Callback<Answer>,
    #[prop(into)] is_submitted: Signal<bool>,
    #[prop(into)] is_correct: Signal<Option<bool>>,
) -> impl IntoView {
    // State for selected hotspots
    let (selected_hotspots, set_selected_hotspots) = create_signal(Vec::<String>::new());
    let (image_dimensions, set_image_dimensions) = create_signal((0, 0));
    
    // Reference to the image element
    let image_ref = create_node_ref::<HtmlImageElement>();
    
    // Initialize when image loads
    let on_image_load = move |_| {
        if let Some(img) = image_ref.get() {
            let width = img.natural_width();
            let height = img.natural_height();
            set_image_dimensions((width as i32, height as i32));
        }
    };
    
    // Handle click on image
    let on_image_click = move |ev: MouseEvent| {
        if is_submitted.get() {
            return;
        }
        
        if let Some(img) = image_ref.get() {
            let rect = img.get_bounding_client_rect();
            let scale_x = image_dimensions.get().0 as f64 / rect.width();
            let scale_y = image_dimensions.get().1 as f64 / rect.height();
            
            let x = (ev.client_x() as f64 - rect.left()) * scale_x;
            let y = (ev.client_y() as f64 - rect.top()) * scale_y;
            
            let click_point = Point { x: x as f32, y: y as f32 };
            
            // Check if click is within any hotspot
            if let Some(content) = &question.get().content.hotspot_content {
                for hotspot in &content.hotspots {
                    if is_point_in_hotspot(&click_point, hotspot) {
                        toggle_hotspot(hotspot.id.clone(), set_selected_hotspots);
                        break;
                    }
                }
            }
            
            // Submit answer
            let answer = Answer::Hotspot(selected_hotspots.get());
            on_answer.call(answer);
        }
    };
    
    // Toggle a hotspot selection
    let toggle_hotspot = move |hotspot_id: String, set_fn: WriteSignal<Vec<String>>| {
        set_fn.update(|selected| {
            if let Some(index) = selected.iter().position(|id| id == &hotspot_id) {
                selected.remove(index);
            } else {
                selected.push(hotspot_id);
            }
        });
    };
    
    // Check if a point is within a hotspot
    let is_point_in_hotspot = move |point: &Point, hotspot: &Hotspot| -> bool {
        match &hotspot.shape {
            HotspotShape::Rectangle { x, y, width, height } => {
                point.x >= *x && point.x <= x + width && 
                point.y >= *y && point.y <= y + height
            },
            HotspotShape::Circle { center_x, center_y, radius } => {
                let dx = point.x - center_x;
                let dy = point.y - center_y;
                (dx * dx + dy * dy) <= radius * radius
            },
            HotspotShape::Polygon { points } => {
                is_point_in_polygon(point, points)
            }
        }
    };
    
    // Check if a point is within a polygon using ray casting algorithm
    let is_point_in_polygon = move |point: &Point, polygon: &Vec<Point>| -> bool {
        if polygon.len() < 3 {
            return false;
        }
        
        let mut inside = false;
        let mut j = polygon.len() - 1;
        
        for i in 0..polygon.len() {
            let pi = &polygon[i];
            let pj = &polygon[j];
            
            if ((pi.y > point.y) != (pj.y > point.y)) &&
               (point.x < (pj.x - pi.x) * (point.y - pi.y) / (pj.y - pi.y) + pi.x) {
                inside = !inside;
            }
            
            j = i;
        }
        
        inside
    };
    
    // Reset selections
    let reset = move |_| {
        set_selected_hotspots.set(Vec::new());
    };
    
    // Get hotspot content
    let hotspot_content = move || question.get().content.hotspot_content.clone();
    
    // Check if a hotspot is selected
    let is_hotspot_selected = move |hotspot_id: &str| -> bool {
        selected_hotspots.get().contains(&hotspot_id.to_string())
    };
    
    // Show correct hotspots after submission
    let show_correct_hotspots = move || {
        is_submitted.get() && is_correct.get().is_some()
    };
    
    view! {
        <div class="hotspot-question">
            <div class="question-text">
                {move || question.get().content.text.clone()}
            </div>
            
            <div class="hotspot-container">
                <div class="image-container" style="position: relative;">
                    {move || {
                        if let Some(content) = hotspot_content() {
                            view! {
                                <img 
                                    _ref=image_ref
                                    src={content.image_url}
                                    alt="Hotspot question image"
                                    on:load=on_image_load
                                    on:click=on_image_click
                                    style="cursor: pointer; max-width: 100%;"
                                />
                                
                                // Render selected hotspots
                                <For
                                    each=move || {
                                        if let Some(content) = hotspot_content() {
                                            content.hotspots
                                        } else {
                                            Vec::new()
                                        }
                                    }
                                    key=|hotspot| hotspot.id.clone()
                                    children=move |hotspot| {
                                        let hotspot_id = hotspot.id.clone();
                                        let is_selected = move || is_hotspot_selected(&hotspot_id);
                                        let is_correct_hotspot = move || {
                                            if let Some(correct) = &question.get().correct_answer {
                                                if let Answer::Hotspot(ids) = correct {
                                                    ids.contains(&hotspot_id)
                                                } else {
                                                    false
                                                }
                                            } else {
                                                false
                                            }
                                        };
                                        
                                        let hotspot_style = move || {
                                            let base_style = match &hotspot.shape {
                                                HotspotShape::Rectangle { x, y, width, height } => {
                                                    format!(
                                                        "position: absolute; left: {}px; top: {}px; width: {}px; height: {}px;",
                                                        x, y, width, height
                                                    )
                                                },
                                                HotspotShape::Circle { center_x, center_y, radius } => {
                                                    format!(
                                                        "position: absolute; left: {}px; top: {}px; width: {}px; height: {}px; border-radius: 50%;",
                                                        center_x - radius, center_y - radius, radius * 2.0, radius * 2.0
                                                    )
                                                },
                                                HotspotShape::Polygon { points } => {
                                                    // For polygon, we'll use a simplified rectangle for now
                                                    // In a real implementation, you'd use SVG or canvas
                                                    let mut min_x = f32::MAX;
                                                    let mut min_y = f32::MAX;
                                                    let mut max_x = f32::MIN;
                                                    let mut max_y = f32::MIN;
                                                    
                                                    for point in points {
                                                        min_x = min_x.min(point.x);
                                                        min_y = min_y.min(point.y);
                                                        max_x = max_x.max(point.x);
                                                        max_y = max_y.max(point.y);
                                                    }
                                                    
                                                    format!(
                                                        "position: absolute; left: {}px; top: {}px; width: {}px; height: {}px;",
                                                        min_x, min_y, max_x - min_x, max_y - min_y
                                                    )
                                                }
                                            };
                                            
                                            base_style
                                        };
                                        
                                        let hotspot_class = move || {
                                            let mut class = "hotspot".to_string();
                                            
                                            if is_selected() {
                                                class.push_str(" selected");
                                            }
                                            
                                            if show_correct_hotspots() {
                                                if is_correct_hotspot() {
                                                    class.push_str(" correct");
                                                } else if is_selected() {
                                                    class.push_str(" incorrect");
                                                }
                                            }
                                            
                                            class
                                        };
                                        
                                        view! {
                                            <div 
                                                class=hotspot_class
                                                style=hotspot_style
                                            ></div>
                                        }
                                    }
                                />
                            }
                        } else {
                            view! { <div>"No image available"</div> }
                        }
                    }}
                </div>
            </div>
            
            <div class="question-actions">
                <button 
                    class="reset-btn"
                    on:click=reset
                    disabled=move || is_submitted.get()
                >
                    "Reset"
                </button>
            </div>
            
            // Feedback section
            {move || {
                if is_submitted.get() {
                    let feedback_class = match is_correct.get() {
                        Some(true) => "feedback correct",
                        Some(false) => "feedback incorrect",
                        None => "feedback",
                    };
                    
                    let feedback_text = match is_correct.get() {
                        Some(true) => "Correct!",
                        Some(false) => "Incorrect. Try again.",
                        None => "",
                    };
                    
                    view! {
                        <div class={feedback_class}>
                            {feedback_text}
                            
                            // Show explanation if available and answer is submitted
                            {move || {
                                if let Some(explanation) = &question.get().explanation {
                                    view! {
                                        <div class="explanation">
                                            <h4>"Explanation"</h4>
                                            <p>{explanation.clone()}</p>
                                        </div>
                                    }
                                } else {
                                    view! { <div></div> }
                                }
                            }}
                        </div>
                    }
                } else {
                    view! { <div></div> }
                }
            }}
        </div>
    }
}
