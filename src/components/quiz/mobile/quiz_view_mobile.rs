use leptos::*;
use crate::components::quiz::mobile::touch_controls::{SwipeContainer, TouchFriendlyButton, MobileTabBar, MobileTab};
use crate::models::quiz::{Quiz, Question, Answer, QuizSession};
use std::rc::Rc;

/// Props for the MobileQuizView component
#[derive(Props, Clone)]
pub struct MobileQuizViewProps {
    /// The quiz to display
    pub quiz: Signal<Quiz>,
    
    /// The current session
    pub session: Signal<QuizSession>,
    
    /// The current question index
    pub current_question: Signal<usize>,
    
    /// Callback when an answer is submitted
    pub on_answer_submit: Callback<(usize, Answer)>,
    
    /// Callback when the quiz is completed
    pub on_complete: Callback<()>,
    
    /// Callback to navigate to the next question
    pub on_next: Callback<()>,
    
    /// Callback to navigate to the previous question
    pub on_previous: Callback<()>,
    
    /// Whether the quiz is in review mode
    #[prop(default = Signal::derive(move || false))]
    pub review_mode: Signal<bool>,
    
    /// Whether the app is offline
    #[prop(default = Signal::derive(move || false))]
    pub is_offline: Signal<bool>,
}

/// A mobile-optimized quiz view component
#[component]
pub fn MobileQuizView(props: MobileQuizViewProps) -> impl IntoView {
    let MobileQuizViewProps {
        quiz,
        session,
        current_question,
        on_answer_submit,
        on_complete,
        on_next,
        on_previous,
        review_mode,
        is_offline,
    } = props;
    
    // Derived signals
    let questions = create_memo(move |_| quiz.get().questions.clone());
    let question_count = create_memo(move |_| questions.get().len());
    let current_question_data = create_memo(move |_| {
        let index = current_question.get();
        let questions = questions.get();
        if index < questions.len() {
            Some(questions[index].clone())
        } else {
            None
        }
    });
    
    // Track user's answer for the current question
    let current_answer = create_rw_signal(None::<Answer>);
    
    // Reset answer when question changes
    create_effect(move |_| {
        let _ = current_question.get();
        current_answer.set(None);
    });
    
    // Handle swipe navigation
    let handle_swipe_left = move || {
        if current_question.get() < question_count.get() - 1 {
            on_next.call(());
        }
    };
    
    let handle_swipe_right = move || {
        if current_question.get() > 0 {
            on_previous.call(());
        }
    };
    
    // Handle answer selection
    let handle_answer_select = move |answer: Answer| {
        current_answer.set(Some(answer.clone()));
        on_answer_submit.call((current_question.get(), answer));
    };
    
    // Calculate progress percentage
    let progress_percentage = create_memo(move |_| {
        let total = question_count.get();
        if total == 0 {
            0
        } else {
            ((current_question.get() + 1) as f32 / total as f32 * 100.0) as i32
        }
    });
    
    // Create tab items for the mobile tab bar
    let tabs = vec![
        MobileTab {
            label: "Questions".to_string(),
            icon: r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"></circle><path d="M9.09 9a3 3 0 0 1 5.83 1c0 2-3 3-3 3"></path><line x1="12" y1="17" x2="12.01" y2="17"></line></svg>"#.to_string(),
        },
        MobileTab {
            label: "Progress".to_string(),
            icon: r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="12" y1="20" x2="12" y2="10"></line><line x1="18" y1="20" x2="18" y2="4"></line><line x1="6" y1="20" x2="6" y2="16"></line></svg>"#.to_string(),
        },
        MobileTab {
            label: "Info".to_string(),
            icon: r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"></circle><line x1="12" y1="16" x2="12" y2="12"></line><line x1="12" y1="8" x2="12.01" y2="8"></line></svg>"#.to_string(),
        },
    ];
    
    // Active tab state
    let active_tab = create_rw_signal(0);
    let handle_tab_change = move |index: usize| {
        active_tab.set(index);
    };
    
    view! {
        <div class="mobile-quiz-view">
            // Offline indicator
            {move || {
                if is_offline.get() {
                    view! {
                        <div class="offline-indicator">
                            <span>You are offline</span>
                            <span class="sync-status">Changes will sync when you're back online</span>
                        </div>
                    }.into_view()
                } else {
                    view! {}.into_view()
                }
            }}
            
            // Quiz progress bar
            <div class="quiz-progress-bar">
                <div class="quiz-progress-fill" style=move || format!("width: {}%;", progress_percentage.get())></div>
            </div>
            
            // Quiz content
            <div class="quiz-content">
                {move || {
                    match active_tab.get() {
                        // Questions tab
                        0 => {
                            view! {
                                <SwipeContainer
                                    on_swipe_left=handle_swipe_left
                                    on_swipe_right=handle_swipe_right
                                    class="question-swipe-container"
                                >
                                    {move || {
                                        if let Some(question) = current_question_data.get() {
                                            view! {
                                                <div class="question-container">
                                                    <h2 class="question-number">Question {current_question.get() + 1} of {question_count.get()}</h2>
                                                    <div class="question-text" inner_html=question.content.render()></div>
                                                    
                                                    <div class="question-choices">
                                                        {question.choices.iter().map(|choice| {
                                                            let choice = choice.clone();
                                                            let question = question.clone();
                                                            let is_selected = create_memo(move |_| {
                                                                if let Some(answer) = current_answer.get() {
                                                                    match answer {
                                                                        Answer::SingleChoice(id) => id == choice.id,
                                                                        Answer::MultipleChoice(ids) => ids.contains(&choice.id),
                                                                        _ => false,
                                                                    }
                                                                } else {
                                                                    false
                                                                }
                                                            });
                                                            
                                                            let is_correct = create_memo(move |_| {
                                                                if review_mode.get() {
                                                                    match &question.correct_answer {
                                                                        Answer::SingleChoice(id) => *id == choice.id,
                                                                        Answer::MultipleChoice(ids) => ids.contains(&choice.id),
                                                                        _ => false,
                                                                    }
                                                                } else {
                                                                    false
                                                                }
                                                            });
                                                            
                                                            let handle_choice_click = move |_| {
                                                                if !review_mode.get() {
                                                                    match question.answer_type {
                                                                        crate::models::quiz::AnswerType::MultipleChoice => {
                                                                            handle_answer_select(Answer::SingleChoice(choice.id));
                                                                        },
                                                                        crate::models::quiz::AnswerType::MultipleSelect => {
                                                                            let new_answer = match current_answer.get() {
                                                                                Some(Answer::MultipleChoice(mut ids)) => {
                                                                                    if ids.contains(&choice.id) {
                                                                                        ids.retain(|id| *id != choice.id);
                                                                                    } else {
                                                                                        ids.push(choice.id);
                                                                                    }
                                                                                    Answer::MultipleChoice(ids)
                                                                                },
                                                                                _ => Answer::MultipleChoice(vec![choice.id]),
                                                                            };
                                                                            handle_answer_select(new_answer);
                                                                        },
                                                                        _ => {},
                                                                    }
                                                                }
                                                            };
                                                            
                                                            view! {
                                                                <div 
                                                                    class=move || format!("choice-item {} {} {}", 
                                                                        if is_selected.get() { "selected" } else { "" },
                                                                        if review_mode.get() { 
                                                                            if is_correct.get() { "correct" } 
                                                                            else if is_selected.get() { "incorrect" }
                                                                            else { "" }
                                                                        } else { "" },
                                                                        if choice.image_url.is_some() { "with-image" } else { "" }
                                                                    )
                                                                    on:click=handle_choice_click
                                                                >
                                                                    {choice.image_url.clone().map(|url| view! {
                                                                        <div class="choice-image">
                                                                            <img src=url alt=choice.text.clone() />
                                                                        </div>
                                                                    })}
                                                                    <div class="choice-text" inner_html=choice.rich_text.clone().unwrap_or(choice.text.clone())></div>
                                                                </div>
                                                            }
                                                        }).collect_view()}
                                                    </div>
                                                    
                                                    {move || {
                                                        if review_mode.get() && question.explanation.is_some() {
                                                            view! {
                                                                <div class="question-explanation">
                                                                    <h3>Explanation</h3>
                                                                    <div inner_html=question.explanation.clone().unwrap()></div>
                                                                </div>
                                                            }.into_view()
                                                        } else {
                                                            view! {}.into_view()
                                                        }
                                                    }}
                                                </div>
                                            }.into_view()
                                        } else {
                                            view! {
                                                <div class="quiz-complete">
                                                    <h2>Quiz Complete!</h2>
                                                    <p>You've answered all the questions.</p>
                                                    <TouchFriendlyButton
                                                        text="View Results".to_string()
                                                        on_click=Callback::new(move |_| on_complete.call(()))
                                                        class="primary-button".to_string()
                                                    />
                                                </div>
                                            }.into_view()
                                        }
                                    }}
                                </SwipeContainer>
                            }.into_view()
                        },
                        
                        // Progress tab
                        1 => {
                            view! {
                                <div class="quiz-progress-view">
                                    <h2>Your Progress</h2>
                                    <div class="progress-summary">
                                        <div class="progress-stat">
                                            <span class="stat-value">{current_question.get() + 1}</span>
                                            <span class="stat-label">Current Question</span>
                                        </div>
                                        <div class="progress-stat">
                                            <span class="stat-value">{question_count.get()}</span>
                                            <span class="stat-label">Total Questions</span>
                                        </div>
                                        <div class="progress-stat">
                                            <span class="stat-value">{progress_percentage.get()}%</span>
                                            <span class="stat-label">Complete</span>
                                        </div>
                                    </div>
                                    
                                    <div class="question-navigator">
                                        <h3>Jump to Question</h3>
                                        <div class="question-dots">
                                            {(0..question_count.get()).map(|index| {
                                                let is_current = create_memo(move |_| current_question.get() == index);
                                                let is_answered = create_memo(move |_| {
                                                    let session_data = session.get();
                                                    session_data.answers.contains_key(&questions.get()[index].id)
                                                });
                                                
                                                let handle_dot_click = move |_| {
                                                    if current_question.get() > index {
                                                        for _ in 0..(current_question.get() - index) {
                                                            on_previous.call(());
                                                        }
                                                    } else if current_question.get() < index {
                                                        for _ in 0..(index - current_question.get()) {
                                                            on_next.call(());
                                                        }
                                                    }
                                                    active_tab.set(0); // Switch back to questions tab
                                                };
                                                
                                                view! {
                                                    <div 
                                                        class=move || format!("question-dot {} {}", 
                                                            if is_current.get() { "current" } else { "" },
                                                            if is_answered.get() { "answered" } else { "" }
                                                        )
                                                        on:click=handle_dot_click
                                                    >
                                                        <span class="dot-number">{index + 1}</span>
                                                    </div>
                                                }
                                            }).collect_view()}
                                        </div>
                                    </div>
                                </div>
                            }.into_view()
                        },
                        
                        // Info tab
                        2 => {
                            view! {
                                <div class="quiz-info-view">
                                    <h2 class="quiz-title">{quiz.get().title}</h2>
                                    
                                    {quiz.get().description.map(|desc| view! {
                                        <div class="quiz-description" inner_html=desc></div>
                                    })}
                                    
                                    <div class="quiz-metadata">
                                        <div class="metadata-item">
                                            <span class="metadata-label">Questions:</span>
                                            <span class="metadata-value">{question_count.get()}</span>
                                        </div>
                                        
                                        <div class="metadata-item">
                                            <span class="metadata-label">Created:</span>
                                            <span class="metadata-value">{quiz.get().created_at.format("%B %d, %Y").to_string()}</span>
                                        </div>
                                        
                                        {quiz.get().tags.len() > 0 then || view! {
                                            <div class="quiz-tags">
                                                {quiz.get().tags.iter().map(|tag| {
                                                    view! { <span class="quiz-tag">{tag}</span> }
                                                }).collect_view()}
                                            </div>
                                        }}
                                    </div>
                                    
                                    <div class="quiz-actions">
                                        <TouchFriendlyButton
                                            text="Resume Quiz".to_string()
                                            on_click=Callback::new(move |_| active_tab.set(0))
                                            class="primary-button".to_string()
                                        />
                                    </div>
                                </div>
                            }.into_view()
                        },
                        
                        // Default case
                        _ => view! {}.into_view(),
                    }
                }}
            </div>
            
            // Navigation buttons (only shown on questions tab)
            {move || {
                if active_tab.get() == 0 {
                    view! {
                        <div class="quiz-nav">
                            <TouchFriendlyButton
                                text="Previous".to_string()
                                on_click=Callback::new(move |_| on_previous.call(()))
                                class="nav-button previous".to_string()
                                disabled=current_question.get() == 0
                            />
                            
                            <TouchFriendlyButton
                                text=if current_question.get() < question_count.get() - 1 { "Next".to_string() } else { "Finish".to_string() }
                                on_click=Callback::new(move |_| {
                                    if current_question.get() < question_count.get() - 1 {
                                        on_next.call(());
                                    } else {
                                        on_complete.call(());
                                    }
                                })
                                class="nav-button next primary-button".to_string()
                                disabled=!review_mode.get() && current_answer.get().is_none()
                            />
                        </div>
                    }.into_view()
                } else {
                    view! {}.into_view()
                }
            }}
            
            // Mobile tab bar
            <MobileTabBar
                active_tab=active_tab.into()
                on_tab_change=Callback::new(handle_tab_change)
                tabs=tabs
            />
        </div>
    }
}
