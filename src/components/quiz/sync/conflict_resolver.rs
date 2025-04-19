use leptos::*;
use crate::models::quiz::{Quiz, Question};
use std::rc::Rc;

/// Props for the ConflictResolver component
#[derive(Props, Clone)]
pub struct ConflictResolverProps<T: Clone + 'static> {
    /// Local version of the entity
    pub local: Signal<T>,
    
    /// Remote version of the entity
    pub remote: Signal<T>,
    
    /// Entity ID
    pub entity_id: String,
    
    /// Entity type
    pub entity_type: String,
    
    /// Callback when the conflict is resolved
    pub on_resolve: Callback<ConflictResolution<T>>,
    
    /// Callback when the conflict resolution is cancelled
    #[prop(default = None)]
    pub on_cancel: Option<Callback<()>>,
    
    /// Function to render the entity
    pub render_entity: Rc<dyn Fn(T) -> View>,
    
    /// Function to merge the entities
    #[prop(default = None)]
    pub merge_entities: Option<Rc<dyn Fn(T, T) -> T>>,
}

/// Conflict resolution result
#[derive(Clone)]
pub enum ConflictResolution<T> {
    /// Use the local version
    UseLocal(T),
    
    /// Use the remote version
    UseRemote(T),
    
    /// Use a merged version
    UseMerged(T),
}

/// A component that helps resolve conflicts between local and remote versions
#[component]
pub fn ConflictResolver<T: Clone + 'static>(props: ConflictResolverProps<T>) -> impl IntoView {
    let ConflictResolverProps {
        local,
        remote,
        entity_id,
        entity_type,
        on_resolve,
        on_cancel,
        render_entity,
        merge_entities,
    } = props;
    
    // Handle use local
    let handle_use_local = move |_| {
        on_resolve.call(ConflictResolution::UseLocal(local.get()));
    };
    
    // Handle use remote
    let handle_use_remote = move |_| {
        on_resolve.call(ConflictResolution::UseRemote(remote.get()));
    };
    
    // Handle merge
    let handle_merge = move |_| {
        if let Some(merge_fn) = merge_entities.clone() {
            let merged = merge_fn(local.get(), remote.get());
            on_resolve.call(ConflictResolution::UseMerged(merged));
        }
    };
    
    // Handle cancel
    let handle_cancel = move |_| {
        if let Some(callback) = on_cancel.clone() {
            callback.call(());
        }
    };
    
    view! {
        <div class="conflict-resolver">
            <div class="conflict-header">
                <h2>Conflict Detected</h2>
                <p>
                    "There is a conflict between your local changes and the remote version of this "
                    {entity_type.clone()}
                    "."
                </p>
            </div>
            
            <div class="conflict-content">
                <div class="conflict-side local">
                    <h3>Your Version</h3>
                    <div class="entity-preview">
                        {(render_entity)(local.get())}
                    </div>
                </div>
                
                <div class="conflict-side remote">
                    <h3>Remote Version</h3>
                    <div class="entity-preview">
                        {(render_entity)(remote.get())}
                    </div>
                </div>
            </div>
            
            <div class="conflict-actions">
                <button
                    class="use-local-button"
                    on:click=handle_use_local
                >
                    "Use Your Version"
                </button>
                
                <button
                    class="use-remote-button"
                    on:click=handle_use_remote
                >
                    "Use Remote Version"
                </button>
                
                {merge_entities.is_some().then(|| view! {
                    <button
                        class="merge-button"
                        on:click=handle_merge
                    >
                        "Merge Changes"
                    </button>
                })}
                
                {on_cancel.is_some().then(|| view! {
                    <button
                        class="cancel-button"
                        on:click=handle_cancel
                    >
                        "Cancel"
                    </button>
                })}
            </div>
        </div>
    }
}

/// Props for the QuizConflictResolver component
#[derive(Props, Clone)]
pub struct QuizConflictResolverProps {
    /// Local version of the quiz
    pub local: Signal<Quiz>,
    
    /// Remote version of the quiz
    pub remote: Signal<Quiz>,
    
    /// Callback when the conflict is resolved
    pub on_resolve: Callback<ConflictResolution<Quiz>>,
    
    /// Callback when the conflict resolution is cancelled
    #[prop(default = None)]
    pub on_cancel: Option<Callback<()>>,
}

/// A specialized conflict resolver for quizzes
#[component]
pub fn QuizConflictResolver(props: QuizConflictResolverProps) -> impl IntoView {
    let QuizConflictResolverProps {
        local,
        remote,
        on_resolve,
        on_cancel,
    } = props;
    
    // Function to render a quiz
    let render_quiz = Rc::new(|quiz: Quiz| {
        view! {
            <div class="quiz-preview">
                <h4 class="quiz-title">{quiz.title}</h4>
                
                {quiz.description.clone().map(|desc| view! {
                    <p class="quiz-description">{desc}</p>
                })}
                
                <div class="quiz-meta">
                    <div class="meta-item">
                        <span class="meta-label">"Questions: "</span>
                        <span class="meta-value">{quiz.questions.len()}</span>
                    </div>
                    
                    <div class="meta-item">
                        <span class="meta-label">"Updated: "</span>
                        <span class="meta-value">{quiz.updated_at.format("%H:%M:%S %d/%m/%Y").to_string()}</span>
                    </div>
                    
                    <div class="meta-item">
                        <span class="meta-label">"Visibility: "</span>
                        <span class="meta-value">{format!("{:?}", quiz.visibility)}</span>
                    </div>
                </div>
                
                <div class="quiz-questions">
                    <h5>"Questions:"</h5>
                    <ul class="question-list">
                        {quiz.questions.iter().take(5).map(|question| {
                            view! {
                                <li class="question-item">
                                    <div class="question-content" inner_html=question.content.render()></div>
                                </li>
                            }
                        }).collect_view()}
                        
                        {(quiz.questions.len() > 5).then(|| view! {
                            <li class="more-questions">
                                "... and "
                                {quiz.questions.len() - 5}
                                " more questions"
                            </li>
                        })}
                    </ul>
                </div>
            </div>
        }.into_view()
    });
    
    // Function to merge quizzes
    let merge_quizzes = Rc::new(|local: Quiz, remote: Quiz| {
        // Create a new quiz with the newer metadata
        let base = if local.updated_at > remote.updated_at {
            local.clone()
        } else {
            remote.clone()
        };
        
        // Merge questions
        let mut questions = std::collections::HashMap::new();
        
        // Add all local questions
        for question in &local.questions {
            questions.insert(question.id, question.clone());
        }
        
        // Add or update with remote questions
        for question in &remote.questions {
            if let Some(local_question) = questions.get(&question.id) {
                // Question exists in both versions, use the newer one
                if question.updated_at > local_question.updated_at {
                    questions.insert(question.id, question.clone());
                }
            } else {
                // Question only exists in remote, add it
                questions.insert(question.id, question.clone());
            }
        }
        
        // Create the merged quiz
        Quiz {
            id: base.id,
            title: base.title,
            description: base.description.clone(),
            created_at: base.created_at,
            updated_at: chrono::Utc::now(), // Use current time for the merged version
            questions: questions.into_values().collect(),
            settings: base.settings.clone(),
            author_id: base.author_id,
            visibility: base.visibility,
            tags: base.tags.clone(),
            study_mode: base.study_mode,
        }
    });
    
    view! {
        <ConflictResolver
            local=local
            remote=remote
            entity_id=local.get().id.to_string()
            entity_type="quiz".to_string()
            on_resolve=on_resolve
            on_cancel=on_cancel
            render_entity=render_quiz
            merge_entities=Some(merge_quizzes)
        />
    }
}

/// Props for the QuestionConflictResolver component
#[derive(Props, Clone)]
pub struct QuestionConflictResolverProps {
    /// Local version of the question
    pub local: Signal<Question>,
    
    /// Remote version of the question
    pub remote: Signal<Question>,
    
    /// Callback when the conflict is resolved
    pub on_resolve: Callback<ConflictResolution<Question>>,
    
    /// Callback when the conflict resolution is cancelled
    #[prop(default = None)]
    pub on_cancel: Option<Callback<()>>,
}

/// A specialized conflict resolver for questions
#[component]
pub fn QuestionConflictResolver(props: QuestionConflictResolverProps) -> impl IntoView {
    let QuestionConflictResolverProps {
        local,
        remote,
        on_resolve,
        on_cancel,
    } = props;
    
    // Function to render a question
    let render_question = Rc::new(|question: Question| {
        view! {
            <div class="question-preview">
                <div class="question-content" inner_html=question.content.render()></div>
                
                <div class="question-meta">
                    <div class="meta-item">
                        <span class="meta-label">"Type: "</span>
                        <span class="meta-value">{format!("{:?}", question.answer_type)}</span>
                    </div>
                    
                    <div class="meta-item">
                        <span class="meta-label">"Updated: "</span>
                        <span class="meta-value">{question.updated_at.format("%H:%M:%S %d/%m/%Y").to_string()}</span>
                    </div>
                </div>
                
                <div class="question-choices">
                    <h5>"Choices:"</h5>
                    <ul class="choice-list">
                        {question.choices.iter().map(|choice| {
                            let is_correct = match &question.correct_answer {
                                crate::models::quiz::Answer::SingleChoice(id) => *id == choice.id,
                                crate::models::quiz::Answer::MultipleChoice(ids) => ids.contains(&choice.id),
                                _ => false,
                            };
                            
                            view! {
                                <li class=format!("choice-item {}", if is_correct { "correct" } else { "" })>
                                    <div class="choice-content" inner_html=choice.rich_text.clone().unwrap_or(choice.text.clone())></div>
                                </li>
                            }
                        }).collect_view()}
                    </ul>
                </div>
                
                {question.explanation.clone().map(|explanation| view! {
                    <div class="question-explanation">
                        <h5>"Explanation:"</h5>
                        <div class="explanation-content" inner_html=explanation></div>
                    </div>
                })}
            </div>
        }.into_view()
    });
    
    // Function to merge questions
    let merge_questions = Rc::new(|local: Question, remote: Question| {
        // Use the newer version as the base
        if local.updated_at > remote.updated_at {
            local
        } else {
            remote
        }
    });
    
    view! {
        <ConflictResolver
            local=local
            remote=remote
            entity_id=local.get().id.to_string()
            entity_type="question".to_string()
            on_resolve=on_resolve
            on_cancel=on_cancel
            render_entity=render_question
            merge_entities=Some(merge_questions)
        />
    }
}
