use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
struct RuleParseResult {
    valid: bool,
    requirement: Option<serde_json::Value>,
    error: Option<String>,
}

#[component]
pub fn CourseRequirementEditor() -> impl IntoView {
    let (rule_text, set_rule_text) = create_signal(String::new());
    let (parse_result, set_parse_result) = create_signal(None::<RuleParseResult>);
    
    let parse_rule = move |_| {
        if rule_text.get().is_empty() {
            return;
        }
        
        spawn_local(async move {
            // Call the Tauri command that invokes the Haskell parser.
            match invoke::<_, String>("parse_completion_rule", &RuleRequest { 
                rule_text: rule_text.get()
            }).await {
                Ok(json) => {
                    match serde_json::from_str::<serde_json::Value>(&json) {
                        Ok(requirement) => {
                            set_parse_result.set(Some(RuleParseResult {
                                valid: true,
                                requirement: Some(requirement),
                                error: None,
                            }));
                        },
                        Err(e) => {
                            set_parse_result.set(Some(RuleParseResult {
                                valid: false,
                                requirement: None,
                                error: Some(format!("JSON parse error: {}", e)),
                            }));
                        }
                    }
                },
                Err(e) => {
                    set_parse_result.set(Some(RuleParseResult {
                        valid: false,
                        requirement: None,
                        error: Some(e.to_string()),
                    }));
                }
            }
        });
    };
    
    view! {
        <div class="course-requirement-editor">
            <div class="editor-header">
                <h2>"Course Requirement Editor"</h2>
                <p class="helper-text">
                    "Define completion requirements using the DSL. Examples:"
                </p>
                <pre>
                    "complete assignment-101\n"
                    "score above 70% final-exam\n"
                    "and {\n  complete assignment-101,\n  score above 60% midterm\n}"
                </pre>
            </div>
            
            <div class="editor-main">
                <textarea 
                    value=rule_text
                    on:input=move |ev| set_rule_text.set(event_target_value(&ev))
                    placeholder="Enter completion rule..."
                    class="rule-editor"
                />
                
                <button on:click=parse_rule class="parse-button">
                    "Parse Rule"
                </button>
                
                <div class="parse-result">
                    {move || {
                        parse_result.get().map(|result| {
                            if result.valid {
                                view! {
                                    <div class="valid-result">
                                        <h3>"Valid Rule"</h3>
                                        <pre class="json-result">
                                            {serde_json::to_string_pretty(&result.requirement.unwrap()).unwrap()}
                                        </pre>
                                    </div>
                                }
                            } else {
                                view! {
                                    <div class="error-result">
                                        <h3>"Parse Error"</h3>
                                        <p class="error-message">{result.error.unwrap()}</p>
                                    </div>
                                }
                            }
                        })
                    }}
                </div>
            </div>
        </div>
    }
}

#[derive(Serialize)]
struct RuleRequest {
    rule_text: String,
}