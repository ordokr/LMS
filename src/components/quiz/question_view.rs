use leptos::*;
use crate::quiz::models::*;
use crate::models::quiz::{Question, Answer, AnswerType};
use super::DragDropQuestion;
use super::HotspotQuestion;
use super::DrawingQuestion;
use super::CodeExecutionQuestion;
use super::MathEquationQuestion;

#[component]
pub fn QuestionView(
    question: Question,
    #[prop(into)] on_answer: Callback<Answer>,
) -> impl IntoView {
    let (selected_answer, set_selected_answer) = create_signal(None::<Answer>);
    let (show_explanation, set_show_explanation) = create_signal(false);

    view! {
        <div class="question-container">
            <div class="question-content">
                {question.content.render()}

                // Show image if available
                {move || {
                    if let Some(img_url) = &question.content.image_url {
                        view! {
                            <div class="question-image">
                                <img src={img_url.clone()} alt="Question image" />
                            </div>
                        }.into_view()
                    } else {
                        view! { <div></div> }.into_view()
                    }
                }}

                // Show audio if available
                {move || {
                    if let Some(audio_url) = &question.content.audio_url {
                        view! {
                            <div class="question-audio">
                                <audio controls>
                                    <source src={audio_url.clone()} type="audio/mpeg" />
                                    "Your browser does not support the audio element."
                                </audio>
                            </div>
                        }.into_view()
                    } else {
                        view! { <div></div> }.into_view()
                    }
                }}
            </div>

            {match question.answer_type {
                AnswerType::MultipleChoice => view! {
                    <div class="multiple-choice">
                        <For
                            each=move || question.choices.clone()
                            key=|choice| choice.id
                            children=move |choice| {
                                view! {
                                    <button
                                        class="choice-button"
                                        class:selected=move || {
                                            if let Some(Answer::Choice(id)) = selected_answer.get() {
                                                id == choice.id
                                            } else {
                                                false
                                            }
                                        }
                                        on:click=move |_| set_selected_answer.set(Some(Answer::Choice(choice.id)))
                                    >
                                        {choice.text}

                                        // Show choice image if available
                                        {move || {
                                            if let Some(img_url) = &choice.image_url {
                                                view! {
                                                    <img src={img_url.clone()} alt="Choice image" class="choice-image" />
                                                }.into_view()
                                            } else {
                                                view! { <span></span> }.into_view()
                                            }
                                        }}
                                    </button>
                                }
                            }
                        />
                    </div>
                },
                AnswerType::TrueFalse => view! {
                    <div class="true-false">
                        <button
                            class="choice-button"
                            class:selected=move || {
                                if let Some(Answer::Text(text)) = &selected_answer.get() {
                                    text == "true"
                                } else {
                                    false
                                }
                            }
                            on:click=move |_| set_selected_answer.set(Some(Answer::Text("true".to_string())))
                        >
                            "True"
                        </button>
                        <button
                            class="choice-button"
                            class:selected=move || {
                                if let Some(Answer::Text(text)) = &selected_answer.get() {
                                    text == "false"
                                } else {
                                    false
                                }
                            }
                            on:click=move |_| set_selected_answer.set(Some(Answer::Text("false".to_string())))
                        >
                            "False"
                        </button>
                    </div>
                },
                AnswerType::ShortAnswer => view! {
                    <div class="short-answer">
                        <input
                            type="text"
                            class="answer-input"
                            placeholder="Enter your answer"
                            on:input=move |ev| {
                                set_selected_answer.set(Some(Answer::Text(event_target_value(&ev))))
                            }
                        />
                    </div>
                },
                AnswerType::Essay => view! {
                    <div class="essay">
                        <textarea
                            class="answer-textarea"
                            placeholder="Enter your answer"
                            on:input=move |ev| {
                                set_selected_answer.set(Some(Answer::Text(event_target_value(&ev))))
                            }
                        ></textarea>
                    </div>
                },
                AnswerType::Matching => view! {
                    <div class="matching">
                        <p>"Matching questions are not yet supported in this view."</p>
                    </div>
                },
                AnswerType::Ordering => view! {
                    <div class="ordering">
                        <p>"Ordering questions are not yet supported in this view."</p>
                    </div>
                },
                AnswerType::DragDrop => view! {
                    <div class="drag-drop">
                        <DragDropQuestion
                            question=create_signal(question.clone())
                            on_answer=on_answer.clone()
                            is_submitted=create_signal(show_explanation.get())
                            is_correct=create_signal(None)
                        />
                    </div>
                },
                AnswerType::Hotspot => view! {
                    <div class="hotspot">
                        <HotspotQuestion
                            question=create_signal(question.clone())
                            on_answer=on_answer.clone()
                            is_submitted=create_signal(show_explanation.get())
                            is_correct=create_signal(None)
                        />
                    </div>
                },
                AnswerType::Drawing => view! {
                    <div class="drawing">
                        <DrawingQuestion
                            question=question.clone()
                            on_answer=on_answer.clone()
                            review_mode=show_explanation.get()
                            current_answer=selected_answer.get()
                        />
                    </div>
                },
                AnswerType::CodeExecution => view! {
                    <div class="code-execution">
                        <CodeExecutionQuestion
                            question=question.clone()
                            on_answer=on_answer.clone()
                            review_mode=show_explanation.get()
                            current_answer=selected_answer.get()
                        />
                    </div>
                },
                AnswerType::MathEquation => view! {
                    <div class="math-equation">
                        <MathEquationQuestion
                            question=question.clone()
                            on_answer=on_answer.clone()
                            review_mode=show_explanation.get()
                            current_answer=selected_answer.get()
                        />
                    </div>
                },
                AnswerType::Timeline => view! {
                    <div class="timeline">
                        <p>"Timeline questions are not yet supported in this view."</p>
                    </div>
                },
                AnswerType::DiagramLabeling => view! {
                    <div class="diagram-labeling">
                        <p>"Diagram labeling questions are not yet supported in this view."</p>
                    </div>
                },
            }}

            <button
                class="submit-answer"
                disabled=move || selected_answer.get().is_none()
                on:click=move |_| {
                    if let Some(answer) = selected_answer.get() {
                        on_answer.call(answer);
                        set_show_explanation.set(true);
                    }
                }
            >
                "Submit Answer"
            </button>

            {move || {
                if show_explanation.get() && question.explanation.is_some() {
                    view! {
                        <div class="question-explanation">
                            <h3>"Explanation"</h3>
                            <div class="explanation-content">
                                {question.explanation.clone().unwrap_or_default()}
                            </div>
                        </div>
                    }.into_view()
                } else {
                    view! { <div></div> }.into_view()
                }
            }}
        </div>
    }
}
