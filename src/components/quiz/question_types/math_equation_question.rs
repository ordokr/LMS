use leptos::*;
use crate::models::quiz::{Question, Answer};
use crate::models::quiz_question_types::{MathEquationContent, MathEquationType};
use std::rc::Rc;

/// Props for the MathEquationQuestion component
#[derive(Props, Clone)]
pub struct MathEquationQuestionProps {
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

/// A component for math equation questions
#[component]
pub fn MathEquationQuestion(props: MathEquationQuestionProps) -> impl IntoView {
    let MathEquationQuestionProps {
        question,
        on_answer,
        review_mode,
        current_answer,
    } = props;
    
    // Get the math equation content
    let math_content = question.content.math_equation_content.clone().unwrap_or_else(|| {
        MathEquationContent {
            equation_type: MathEquationType::Algebraic,
            variables: None,
            precision: Some(2),
            display_mode: true,
        }
    });
    
    // Math editor state
    let (equation, set_equation) = create_signal(String::new());
    let (is_valid, set_is_valid) = create_signal(true);
    let (error_message, set_error_message) = create_signal(String::new());
    let editor_ref = create_node_ref::<html::Div>();
    
    // Initialize with current answer if available
    create_effect(move |_| {
        if let Some(Answer::MathEquation(answer)) = current_answer.clone() {
            set_equation.set(answer);
        }
    });
    
    // Initialize MathQuill when the component mounts
    create_effect(move |_| {
        if let Some(editor) = editor_ref.get() {
            // In a real implementation, this would initialize MathQuill
            // For now, we'll just set up a basic textarea
            
            // Load MathJax
            let window = web_sys::window().expect("no global `window` exists");
            let document = window.document().expect("should have a document on window");
            let head = document.head().expect("document should have a head");
            
            // Check if MathJax is already loaded
            if document.get_element_by_id("mathjax-script").is_none() {
                let script = document.create_element("script").expect("failed to create script element");
                script.set_id("mathjax-script");
                script.set_attribute("type", "text/javascript").expect("failed to set attribute");
                script.set_attribute("src", "https://cdn.jsdelivr.net/npm/mathjax@3/es5/tex-mml-chtml.js").expect("failed to set attribute");
                script.set_attribute("async", "true").expect("failed to set attribute");
                head.append_child(&script).expect("failed to append script to head");
            }
        }
    });
    
    // Handle equation change
    let handle_equation_change = move |e: web_sys::Event| {
        let input = event_target_value(&e);
        set_equation.set(input.clone());
        
        // Simple validation - check if the equation has balanced parentheses
        let mut stack = Vec::new();
        let mut valid = true;
        
        for c in input.chars() {
            match c {
                '(' | '[' | '{' => stack.push(c),
                ')' => {
                    if stack.pop() != Some('(') {
                        valid = false;
                        break;
                    }
                },
                ']' => {
                    if stack.pop() != Some('[') {
                        valid = false;
                        break;
                    }
                },
                '}' => {
                    if stack.pop() != Some('{') {
                        valid = false;
                        break;
                    }
                },
                _ => {}
            }
        }
        
        if !stack.is_empty() {
            valid = false;
        }
        
        set_is_valid.set(valid);
        
        if !valid {
            set_error_message.set("Invalid equation: unbalanced parentheses".to_string());
        } else {
            set_error_message.set(String::new());
            
            // Submit the answer
            if let Some(callback) = on_answer.clone() {
                callback.call(Answer::MathEquation(input));
            }
        }
    };
    
    // Handle submit
    let handle_submit = move |_| {
        if is_valid.get() {
            // Submit the answer
            if let Some(callback) = on_answer.clone() {
                callback.call(Answer::MathEquation(equation.get()));
            }
        }
    };
    
    view! {
        <div class="math-equation-question">
            <div class="question-content" inner_html=question.content.render()></div>
            
            <div class="math-editor-container">
                <div class="math-editor-header">
                    <h4>
                        {match math_content.equation_type {
                            MathEquationType::Algebraic => "Algebraic Equation",
                            MathEquationType::Calculus => "Calculus Expression",
                            MathEquationType::Geometric => "Geometric Expression",
                            MathEquationType::Statistical => "Statistical Expression",
                            MathEquationType::Custom => "Math Expression",
                        }}
                    </h4>
                    
                    {math_content.variables.clone().map(|vars| view! {
                        <div class="variables-info">
                            <h5>"Variables:"</h5>
                            <ul class="variables-list">
                                {vars.iter().map(|(name, values)| view! {
                                    <li>
                                        <strong>{name}</strong>
                                        {if values.len() == 1 {
                                            format!(" = {}", values[0])
                                        } else {
                                            format!(" ∈ {:?}", values)
                                        }}
                                    </li>
                                }).collect_view()}
                            </ul>
                        </div>
                    })}
                </div>
                
                <div class="math-editor" _ref=editor_ref>
                    {if !review_mode {
                        view! {
                            <textarea
                                class=format!("equation-input {}", if is_valid.get() { "" } else { "invalid" })
                                placeholder="Enter your equation (e.g., x^2 + 3x - 4 = 0)"
                                value=equation.get()
                                on:input=handle_equation_change
                                disabled=review_mode
                            ></textarea>
                        }
                    } else {
                        view! {}
                    }}
                    
                    <div class="equation-preview">
                        <h5>"Preview:"</h5>
                        <div class="math-preview">
                            {move || {
                                if !equation.get().is_empty() {
                                    view! {
                                        <div class="math-display" inner_html=format!("\\[{}\\]", equation.get())></div>
                                    }.into_view()
                                } else {
                                    view! {
                                        <div class="math-display empty">"Enter an equation to see the preview"</div>
                                    }.into_view()
                                }
                            }}
                        </div>
                    </div>
                    
                    {move || {
                        if !error_message.get().is_empty() {
                            view! {
                                <div class="error-message">{error_message.get()}</div>
                            }.into_view()
                        } else {
                            view! {}.into_view()
                        }
                    }}
                </div>
                
                {move || {
                    if !review_mode {
                        view! {
                            <div class="math-controls">
                                <div class="symbol-buttons">
                                    <button class="symbol-button" on:click=move |_| {
                                        set_equation.update(|eq| *eq += "^");
                                    }>"^"</button>
                                    <button class="symbol-button" on:click=move |_| {
                                        set_equation.update(|eq| *eq += "\\frac{}{}");
                                    }>"\\frac{}{}"</button>
                                    <button class="symbol-button" on:click=move |_| {
                                        set_equation.update(|eq| *eq += "\\sqrt{}");
                                    }>"\\sqrt{}"</button>
                                    <button class="symbol-button" on:click=move |_| {
                                        set_equation.update(|eq| *eq += "\\int_{}^{}");
                                    }>"\\int"</button>
                                    <button class="symbol-button" on:click=move |_| {
                                        set_equation.update(|eq| *eq += "\\sum_{}^{}");
                                    }>"\\sum"</button>
                                    <button class="symbol-button" on:click=move |_| {
                                        set_equation.update(|eq| *eq += "\\pi");
                                    }>"π"</button>
                                    <button class="symbol-button" on:click=move |_| {
                                        set_equation.update(|eq| *eq += "\\theta");
                                    }>"θ"</button>
                                </div>
                                
                                <button
                                    class="submit-button"
                                    on:click=handle_submit
                                    disabled=!is_valid.get() || equation.get().is_empty()
                                >
                                    "Submit Equation"
                                </button>
                            </div>
                        }.into_view()
                    } else {
                        view! {}.into_view()
                    }
                }}
            </div>
            
            {move || {
                if review_mode {
                    if let Some(Answer::MathEquation(correct_equation)) = question.correct_answer.clone() {
                        view! {
                            <div class="correct-answer">
                                <h4>"Correct Equation"</h4>
                                <div class="math-display" inner_html=format!("\\[{}\\]", correct_equation)></div>
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
