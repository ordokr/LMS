use leptos::*;
use crate::models::quiz::{Question, Answer};
use crate::models::quiz_question_types::{CodeExecutionContent, CodeExecutionAnswer, CodeExecutionResult};
use std::rc::Rc;

/// Props for the CodeExecutionQuestion component
#[derive(Props, Clone)]
pub struct CodeExecutionQuestionProps {
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

/// A component for code execution questions
#[component]
pub fn CodeExecutionQuestion(props: CodeExecutionQuestionProps) -> impl IntoView {
    let CodeExecutionQuestionProps {
        question,
        on_answer,
        review_mode,
        current_answer,
    } = props;
    
    // Get the code execution content
    let code_content = question.content.code_execution_content.clone().unwrap_or_else(|| {
        CodeExecutionContent {
            language: "python".to_string(),
            initial_code: "# Write your code here\n".to_string(),
            test_cases: vec![],
            allowed_imports: None,
            time_limit_ms: Some(1000),
            memory_limit_kb: Some(64000),
        }
    });
    
    // Code editor state
    let (code, set_code) = create_signal(code_content.initial_code.clone());
    let (execution_results, set_execution_results) = create_signal(Vec::<CodeExecutionResult>::new());
    let (is_executing, set_is_executing) = create_signal(false);
    let (selected_language, set_selected_language) = create_signal(code_content.language.clone());
    
    // Initialize with current answer if available
    create_effect(move |_| {
        if let Some(Answer::CodeExecution(answer)) = current_answer.clone() {
            set_code.set(answer.code);
            set_selected_language.set(answer.language);
            set_execution_results.set(answer.execution_results);
        }
    });
    
    // Handle code change
    let handle_code_change = move |e: web_sys::Event| {
        let input = event_target_value(&e);
        set_code.set(input);
    };
    
    // Handle language change
    let handle_language_change = move |e: web_sys::Event| {
        let input = event_target_value(&e);
        set_selected_language.set(input);
    };
    
    // Handle code execution
    let handle_execute = move |_| {
        set_is_executing.set(true);
        
        // In a real implementation, this would call a backend service to execute the code
        // For now, we'll simulate execution with a timeout
        let code_value = code.get();
        let language_value = selected_language.get();
        
        // Create simulated results
        let mut results = Vec::new();
        
        for test_case in &code_content.test_cases {
            // Simple simulation - just check if the code contains the expected output
            let passed = code_value.contains(&test_case.expected_output);
            
            results.push(CodeExecutionResult {
                test_case_id: test_case.id.clone(),
                output: if passed {
                    test_case.expected_output.clone()
                } else {
                    "Incorrect output".to_string()
                },
                passed,
                execution_time_ms: 100,
                memory_used_kb: Some(1000),
                error: if passed { None } else { Some("Output does not match expected result".to_string()) },
            });
        }
        
        // Simulate network delay
        set_timeout(
            move || {
                set_execution_results.set(results.clone());
                set_is_executing.set(false);
                
                // Submit the answer
                if let Some(callback) = on_answer.clone() {
                    let answer = CodeExecutionAnswer {
                        code: code_value,
                        language: language_value,
                        execution_results: results,
                    };
                    callback.call(Answer::CodeExecution(answer));
                }
            },
            std::time::Duration::from_millis(1000),
        );
    };
    
    view! {
        <div class="code-execution-question">
            <div class="question-content" inner_html=question.content.render()></div>
            
            <div class="code-editor-container">
                <div class="editor-header">
                    <select
                        class="language-selector"
                        value=selected_language.get()
                        on:change=handle_language_change
                        disabled=review_mode
                    >
                        <option value="python">"Python"</option>
                        <option value="javascript">"JavaScript"</option>
                        <option value="java">"Java"</option>
                        <option value="cpp">"C++"</option>
                        <option value="csharp">"C#"</option>
                        <option value="rust">"Rust"</option>
                    </select>
                    
                    {move || {
                        if !review_mode {
                            view! {
                                <button
                                    class="execute-button"
                                    on:click=handle_execute
                                    disabled=is_executing.get()
                                >
                                    {if is_executing.get() { "Executing..." } else { "Run Code" }}
                                </button>
                            }.into_view()
                        } else {
                            view! {}.into_view()
                        }
                    }}
                </div>
                
                <textarea
                    class="code-editor"
                    value=code.get()
                    on:input=handle_code_change
                    readonly=review_mode
                ></textarea>
                
                <div class="test-cases">
                    <h4>"Test Cases"</h4>
                    {code_content.test_cases.iter().filter(|test| !test.is_hidden || review_mode).map(|test| {
                        let test_id = test.id.clone();
                        let result = create_memo(move |_| {
                            execution_results.get().iter().find(|r| r.test_case_id == test_id).cloned()
                        });
                        
                        view! {
                            <div class="test-case">
                                <div class="test-case-header">
                                    {test.description.clone().unwrap_or_else(|| format!("Test Case {}", test.id))}
                                    {move || {
                                        if let Some(res) = result.get() {
                                            view! {
                                                <span class=format!("test-result {}", if res.passed { "passed" } else { "failed" })>
                                                    {if res.passed { "✓ Passed" } else { "✗ Failed" }}
                                                </span>
                                            }.into_view()
                                        } else {
                                            view! {
                                                <span class="test-result pending">"Not run"</span>
                                            }.into_view()
                                        }
                                    }}
                                </div>
                                
                                <div class="test-case-details">
                                    <div class="test-input">
                                        <strong>"Input: "</strong>
                                        <pre>{test.input.clone()}</pre>
                                    </div>
                                    
                                    <div class="test-expected">
                                        <strong>"Expected Output: "</strong>
                                        <pre>{test.expected_output.clone()}</pre>
                                    </div>
                                    
                                    {move || {
                                        if let Some(res) = result.get() {
                                            view! {
                                                <div class="test-actual">
                                                    <strong>"Actual Output: "</strong>
                                                    <pre>{res.output.clone()}</pre>
                                                    
                                                    {res.error.clone().map(|err| view! {
                                                        <div class="test-error">
                                                            <strong>"Error: "</strong>
                                                            <pre>{err}</pre>
                                                        </div>
                                                    })}
                                                    
                                                    <div class="test-stats">
                                                        <span>"Execution Time: "{res.execution_time_ms}" ms"</span>
                                                        {res.memory_used_kb.map(|mem| view! {
                                                            <span>"Memory Used: "{mem}" KB"</span>
                                                        })}
                                                    </div>
                                                </div>
                                            }.into_view()
                                        } else {
                                            view! {}.into_view()
                                        }
                                    }}
                                </div>
                            </div>
                        }
                    }).collect_view()}
                </div>
            </div>
            
            {move || {
                if review_mode {
                    if let Some(Answer::CodeExecution(correct_answer)) = question.correct_answer.clone() {
                        view! {
                            <div class="correct-answer">
                                <h4>"Correct Solution"</h4>
                                <pre class="correct-code">{correct_answer.code}</pre>
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
