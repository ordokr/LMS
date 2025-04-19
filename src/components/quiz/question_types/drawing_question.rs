use leptos::*;
use crate::models::quiz::{Question, Answer};
use crate::models::quiz_question_types::{DrawingContent, DrawingTool};
use std::rc::Rc;

/// Props for the DrawingQuestion component
#[derive(Props, Clone)]
pub struct DrawingQuestionProps {
    /// The question to display
    pub question: Question,
    
    /// Callback when an answer is submitted
    #[prop(default = None)]
    pub on_answer: Option<Callback<Answer>>,
    
    /// Whether the question is in review mode
    #[prop(default = false)]
    pub review_mode: bool,
    
    /// The user's current answer
    #[prop(default = None)]
    pub current_answer: Option<Answer>,
}

/// A component for drawing/sketch questions
#[component]
pub fn DrawingQuestion(props: DrawingQuestionProps) -> impl IntoView {
    let DrawingQuestionProps {
        question,
        on_answer,
        review_mode,
        current_answer,
    } = props;
    
    // Get the drawing content
    let drawing_content = question.content.drawing_content.clone().unwrap_or_else(|| {
        DrawingContent {
            background_image_url: None,
            canvas_width: 800,
            canvas_height: 600,
            tools: vec![
                DrawingTool::Pen,
                DrawingTool::Eraser,
                DrawingTool::Line,
                DrawingTool::Rectangle,
                DrawingTool::Circle,
            ],
            reference_drawing: None,
        }
    });
    
    // Canvas element reference
    let canvas_ref = create_node_ref::<html::Canvas>();
    
    // Current drawing state
    let (drawing_data, set_drawing_data) = create_signal(String::new());
    let (selected_tool, set_selected_tool) = create_signal(DrawingTool::Pen);
    let (is_drawing, set_is_drawing) = create_signal(false);
    let (stroke_color, set_stroke_color) = create_signal("#000000".to_string());
    let (stroke_width, set_stroke_width) = create_signal(2);
    
    // Initialize canvas with current answer if available
    create_effect(move |_| {
        if let Some(Answer::Drawing(data)) = current_answer.clone() {
            set_drawing_data.set(data);
            
            // Load the drawing data into the canvas
            if let Some(canvas) = canvas_ref.get() {
                let ctx = canvas
                    .get_context("2d")
                    .expect("Failed to get canvas context")
                    .unwrap();
                
                let ctx = ctx.unchecked_into::<web_sys::CanvasRenderingContext2d>();
                
                // Clear the canvas
                ctx.clear_rect(0.0, 0.0, drawing_content.canvas_width as f64, drawing_content.canvas_height as f64);
                
                // Create a new image
                let img = web_sys::HtmlImageElement::new().unwrap();
                
                // Set up the onload handler
                let ctx_clone = ctx.clone();
                let onload = Closure::wrap(Box::new(move || {
                    ctx_clone.draw_image_with_html_image_element(&img, 0.0, 0.0)
                        .expect("Failed to draw image");
                }) as Box<dyn FnMut()>);
                
                img.set_onload(Some(onload.as_ref().unchecked_ref()));
                img.set_src(&drawing_data.get());
                
                // Keep the closure alive
                onload.forget();
            }
        }
    });
    
    // Set up canvas event handlers
    create_effect(move |_| {
        if let Some(canvas) = canvas_ref.get() {
            // Mouse down event
            let mouse_down_handler = move |e: web_sys::MouseEvent| {
                if review_mode {
                    return;
                }
                
                set_is_drawing.set(true);
                
                let ctx = canvas
                    .get_context("2d")
                    .expect("Failed to get canvas context")
                    .unwrap()
                    .unchecked_into::<web_sys::CanvasRenderingContext2d>();
                
                let rect = canvas.get_bounding_client_rect();
                let x = e.client_x() as f64 - rect.left();
                let y = e.client_y() as f64 - rect.top();
                
                ctx.begin_path();
                ctx.set_stroke_style(&wasm_bindgen::JsValue::from_str(&stroke_color.get()));
                ctx.set_line_width(stroke_width.get() as f64);
                ctx.set_line_cap("round");
                ctx.move_to(x, y);
            };
            
            // Mouse move event
            let mouse_move_handler = move |e: web_sys::MouseEvent| {
                if !is_drawing.get() || review_mode {
                    return;
                }
                
                let ctx = canvas
                    .get_context("2d")
                    .expect("Failed to get canvas context")
                    .unwrap()
                    .unchecked_into::<web_sys::CanvasRenderingContext2d>();
                
                let rect = canvas.get_bounding_client_rect();
                let x = e.client_x() as f64 - rect.left();
                let y = e.client_y() as f64 - rect.top();
                
                match selected_tool.get() {
                    DrawingTool::Pen | DrawingTool::Brush => {
                        ctx.line_to(x, y);
                        ctx.stroke();
                    },
                    DrawingTool::Eraser => {
                        ctx.save();
                        ctx.set_global_composite_operation("destination-out");
                        ctx.set_line_width(stroke_width.get() as f64 * 2.0);
                        ctx.line_to(x, y);
                        ctx.stroke();
                        ctx.restore();
                    },
                    _ => {
                        // Other tools handled in mouse up
                    }
                }
            };
            
            // Mouse up event
            let mouse_up_handler = move |e: web_sys::MouseEvent| {
                if !is_drawing.get() || review_mode {
                    return;
                }
                
                let ctx = canvas
                    .get_context("2d")
                    .expect("Failed to get canvas context")
                    .unwrap()
                    .unchecked_into::<web_sys::CanvasRenderingContext2d>();
                
                let rect = canvas.get_bounding_client_rect();
                let x = e.client_x() as f64 - rect.left();
                let y = e.client_y() as f64 - rect.top();
                
                match selected_tool.get() {
                    DrawingTool::Line => {
                        ctx.line_to(x, y);
                        ctx.stroke();
                    },
                    DrawingTool::Rectangle => {
                        let start_x = ctx.get_line_dash()[0];
                        let start_y = ctx.get_line_dash()[1];
                        let width = x - start_x;
                        let height = y - start_y;
                        
                        ctx.stroke_rect(start_x, start_y, width, height);
                    },
                    DrawingTool::Circle => {
                        let start_x = ctx.get_line_dash()[0];
                        let start_y = ctx.get_line_dash()[1];
                        let radius = ((x - start_x).powi(2) + (y - start_y).powi(2)).sqrt();
                        
                        ctx.begin_path();
                        ctx.arc(start_x, start_y, radius, 0.0, 2.0 * std::f64::consts::PI)
                            .expect("Failed to draw arc");
                        ctx.stroke();
                    },
                    _ => {
                        // Other tools handled in mouse move
                    }
                }
                
                set_is_drawing.set(false);
                
                // Save the drawing data
                let data_url = canvas.to_data_url().expect("Failed to get data URL");
                set_drawing_data.set(data_url.clone());
                
                // Submit the answer
                if let Some(callback) = on_answer.clone() {
                    callback.call(Answer::Drawing(data_url));
                }
            };
            
            // Attach event listeners
            let canvas_element = canvas.clone();
            let mouse_down_closure = Closure::wrap(Box::new(mouse_down_handler) as Box<dyn FnMut(_)>);
            let mouse_move_closure = Closure::wrap(Box::new(mouse_move_handler) as Box<dyn FnMut(_)>);
            let mouse_up_closure = Closure::wrap(Box::new(mouse_up_handler) as Box<dyn FnMut(_)>);
            
            canvas_element.add_event_listener_with_callback(
                "mousedown",
                mouse_down_closure.as_ref().unchecked_ref(),
            ).expect("Failed to add mousedown event listener");
            
            canvas_element.add_event_listener_with_callback(
                "mousemove",
                mouse_move_closure.as_ref().unchecked_ref(),
            ).expect("Failed to add mousemove event listener");
            
            canvas_element.add_event_listener_with_callback(
                "mouseup",
                mouse_up_closure.as_ref().unchecked_ref(),
            ).expect("Failed to add mouseup event listener");
            
            // Keep the closures alive
            mouse_down_closure.forget();
            mouse_move_closure.forget();
            mouse_up_closure.forget();
        }
    });
    
    // Handle tool selection
    let handle_tool_select = move |tool: DrawingTool| {
        set_selected_tool.set(tool);
    };
    
    // Handle color change
    let handle_color_change = move |e: web_sys::Event| {
        let input = event_target_value(&e);
        set_stroke_color.set(input);
    };
    
    // Handle stroke width change
    let handle_width_change = move |e: web_sys::Event| {
        let input = event_target_value(&e);
        if let Ok(width) = input.parse::<i32>() {
            set_stroke_width.set(width);
        }
    };
    
    // Handle clear canvas
    let handle_clear = move |_| {
        if let Some(canvas) = canvas_ref.get() {
            let ctx = canvas
                .get_context("2d")
                .expect("Failed to get canvas context")
                .unwrap()
                .unchecked_into::<web_sys::CanvasRenderingContext2d>();
            
            ctx.clear_rect(0.0, 0.0, drawing_content.canvas_width as f64, drawing_content.canvas_height as f64);
            
            // Save the empty drawing data
            let data_url = canvas.to_data_url().expect("Failed to get data URL");
            set_drawing_data.set(data_url.clone());
            
            // Submit the answer
            if let Some(callback) = on_answer.clone() {
                callback.call(Answer::Drawing(data_url));
            }
        }
    };
    
    view! {
        <div class="drawing-question">
            <div class="question-content" inner_html=question.content.render()></div>
            
            <div class="drawing-canvas-container">
                {drawing_content.background_image_url.clone().map(|url| view! {
                    <img src=url alt="Background" class="drawing-background" />
                })}
                
                <canvas
                    _ref=canvas_ref
                    width=drawing_content.canvas_width.to_string()
                    height=drawing_content.canvas_height.to_string()
                    class="drawing-canvas"
                ></canvas>
                
                {drawing_content.reference_drawing.clone().map(|drawing| view! {
                    <div class="reference-drawing">
                        <h4>"Reference Drawing"</h4>
                        <img src=drawing alt="Reference" />
                    </div>
                })}
            </div>
            
            {move || {
                if !review_mode {
                    view! {
                        <div class="drawing-tools">
                            <div class="tool-buttons">
                                {drawing_content.tools.iter().map(|tool| {
                                    let tool = tool.clone();
                                    let is_selected = create_memo(move |_| selected_tool.get() == tool);
                                    
                                    view! {
                                        <button
                                            class=move || format!("tool-button {}", if is_selected.get() { "selected" } else { "" })
                                            on:click=move |_| handle_tool_select(tool.clone())
                                        >
                                            {match tool {
                                                DrawingTool::Pen => "✏️",
                                                DrawingTool::Brush => "🖌️",
                                                DrawingTool::Eraser => "🧽",
                                                DrawingTool::Line => "📏",
                                                DrawingTool::Rectangle => "⬜",
                                                DrawingTool::Circle => "⭕",
                                                DrawingTool::Text => "T",
                                            }}
                                        </button>
                                    }
                                }).collect_view()}
                            </div>
                            
                            <div class="tool-options">
                                <div class="color-picker">
                                    <label for="stroke-color">"Color: "</label>
                                    <input
                                        type="color"
                                        id="stroke-color"
                                        value=stroke_color.get()
                                        on:change=handle_color_change
                                    />
                                </div>
                                
                                <div class="stroke-width">
                                    <label for="stroke-width">"Width: "</label>
                                    <input
                                        type="range"
                                        id="stroke-width"
                                        min="1"
                                        max="20"
                                        value=stroke_width.get().to_string()
                                        on:change=handle_width_change
                                    />
                                    <span>{stroke_width.get()}</span>
                                </div>
                                
                                <button class="clear-button" on:click=handle_clear>
                                    "Clear Canvas"
                                </button>
                            </div>
                        </div>
                    }.into_view()
                } else {
                    view! {}.into_view()
                }
            }}
            
            {move || {
                if review_mode {
                    if let Some(Answer::Drawing(correct_drawing)) = question.correct_answer.clone() {
                        view! {
                            <div class="correct-answer">
                                <h4>"Correct Answer"</h4>
                                <img src=correct_drawing alt="Correct drawing" class="correct-drawing" />
                            </div>
                        }.into_view()
                    } else {
                        view! {}.into_view()
                    }
                } else {
                    view! {}.into_view()
                }
            }}
        </div>
    }
}
