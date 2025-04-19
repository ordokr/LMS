use leptos::*;
use crate::models::quiz::{Quiz, Question, Answer, QuizSession};
use crate::components::quiz::QuizView;
use crate::components::quiz::mobile::MobileQuizView;
use uuid::Uuid;
use std::rc::Rc;

/// Props for the ResponsiveQuizView component
#[derive(Props, Clone)]
pub struct ResponsiveQuizViewProps {
    /// The quiz to display
    pub quiz: Signal<Quiz>,
    
    /// The current session
    pub session: Signal<Option<QuizSession>>,
    
    /// Callback when an answer is submitted
    pub on_submit_answer: Callback<(Uuid, Answer)>,
    
    /// Callback when the quiz is completed
    pub on_complete: Callback<()>,
    
    /// Whether the app is offline
    #[prop(default = Signal::derive(move || false))]
    pub is_offline: Signal<bool>,
}

/// A responsive quiz view that uses the mobile version on small screens
#[component]
pub fn ResponsiveQuizView(props: ResponsiveQuizViewProps) -> impl IntoView {
    let ResponsiveQuizViewProps {
        quiz,
        session,
        on_submit_answer,
        on_complete,
        is_offline,
    } = props;
    
    // Track current question index
    let (current_question, set_current_question) = create_signal(0);
    
    // Track whether we're in review mode
    let (review_mode, set_review_mode) = create_signal(false);
    
    // Handle answer submission
    let handle_answer_submit = move |(index, answer): (usize, Answer)| {
        let question = quiz.get().questions.get(index).cloned();
        if let Some(q) = question {
            on_submit_answer.call((q.id, answer));
        }
    };
    
    // Handle next question
    let handle_next = move || {
        set_current_question.update(|q| {
            if *q < quiz.get().questions.len() - 1 {
                *q += 1;
            }
        });
    };
    
    // Handle previous question
    let handle_previous = move || {
        set_current_question.update(|q| {
            if *q > 0 {
                *q -= 1;
            }
        });
    };
    
    // Handle quiz completion
    let handle_complete = move || {
        set_review_mode.set(true);
        on_complete.call(());
    };
    
    // Get the unwrapped session
    let unwrapped_session = create_memo(move |_| {
        session.get().unwrap_or_else(|| QuizSession {
            id: Uuid::nil(),
            quiz_id: quiz.get().id,
            user_id: Uuid::nil(),
            started_at: chrono::Utc::now(),
            completed_at: None,
            answers: std::collections::HashMap::new(),
            score: None,
            completed: false,
        })
    });
    
    // Detect if we're on a mobile device
    let (is_mobile, set_is_mobile) = create_signal(false);
    
    // Set up media query for mobile detection
    create_effect(move |_| {
        let window = web_sys::window().expect("no global `window` exists");
        let media_query = window
            .match_media("(max-width: 768px)")
            .expect("failed to create media query");
        
        set_is_mobile.set(media_query.matches());
        
        // Set up listener for media query changes
        let callback = Closure::wrap(Box::new(move |e: web_sys::MediaQueryListEvent| {
            set_is_mobile.set(e.matches());
        }) as Box<dyn FnMut(_)>);
        
        if let Some(add_listener) = js_sys::Reflect::get(
            &media_query,
            &wasm_bindgen::JsValue::from_str("addListener"),
        ).ok() {
            let _ = js_sys::Reflect::apply(
                &add_listener.into(),
                &media_query,
                &js_sys::Array::of1(&callback.as_ref().unchecked_ref()),
            );
        }
        
        callback.forget(); // Prevent callback from being dropped
    });
    
    view! {
        {move || {
            if is_mobile.get() {
                // Mobile view
                view! {
                    <MobileQuizView
                        quiz=quiz
                        session=unwrapped_session.into()
                        current_question=current_question.into()
                        on_answer_submit=Callback::new(handle_answer_submit)
                        on_complete=Callback::new(handle_complete)
                        on_next=Callback::new(handle_next)
                        on_previous=Callback::new(handle_previous)
                        review_mode=review_mode.into()
                        is_offline=is_offline
                    />
                }.into_view()
            } else {
                // Desktop view
                view! {
                    <QuizView
                        quiz=quiz
                        session=session
                        on_submit_answer=on_submit_answer
                        on_complete=on_complete
                    />
                }.into_view()
            }
        }}
    }
}
