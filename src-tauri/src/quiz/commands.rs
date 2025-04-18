use super::models::{Quiz, Question, Answer, QuestionContent, AnswerType, StudyMode, QuizVisibility, FlashcardData};
use super::session::QuizSession;
use super::storage::HybridQuizStore;
use super::analytics::{TimePeriod, UserStudyStats, QuizAnalytics};
use super::export::{ExportOptions, ExportFormat};
use super::QuizEngine;
use tauri::{State, api::path};
use uuid::Uuid;
use std::sync::Arc;
use std::path::PathBuf;

#[tauri::command]
pub async fn create_quiz(
    title: String,
    description: Option<String>,
    author_id: Option<String>,
    study_mode: String,
    store: State<'_, Arc<HybridQuizStore>>,
) -> Result<String, String> {
    let author_uuid = if let Some(id) = author_id {
        Some(Uuid::parse_str(&id).map_err(|e| e.to_string())?)
    } else {
        None
    };

    let study_mode = match study_mode.as_str() {
        "flashcards" => StudyMode::Flashcards,
        "multiple_choice" => StudyMode::MultipleChoice,
        "written" => StudyMode::Written,
        "mixed" => StudyMode::Mixed,
        _ => StudyMode::MultipleChoice,
    };

    let mut quiz = Quiz::new(title, author_uuid);
    quiz.description = description;
    quiz.study_mode = study_mode;

    store.store_quiz(&quiz)
        .await
        .map_err(|e| e.to_string())?;

    Ok(quiz.id.to_string())
}

#[tauri::command]
pub async fn get_quiz(
    quiz_id: String,
    store: State<'_, Arc<HybridQuizStore>>,
) -> Result<Quiz, String> {
    let id = Uuid::parse_str(&quiz_id).map_err(|e| e.to_string())?;
    store.get_quiz(id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn list_quizzes(
    limit: usize,
    offset: usize,
    store: State<'_, Arc<HybridQuizStore>>,
) -> Result<Vec<Quiz>, String> {
    store.list_quizzes(limit, offset)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn add_question(
    quiz_id: String,
    question_text: String,
    answer_type: String,
    store: State<'_, Arc<HybridQuizStore>>,
) -> Result<String, String> {
    let quiz_uuid = Uuid::parse_str(&quiz_id).map_err(|e| e.to_string())?;

    let mut quiz = store.get_quiz(quiz_uuid)
        .await
        .map_err(|e| e.to_string())?;

    let answer_type = match answer_type.as_str() {
        "multiple_choice" => AnswerType::MultipleChoice,
        "true_false" => AnswerType::TrueFalse,
        "short_answer" => AnswerType::ShortAnswer,
        "essay" => AnswerType::Essay,
        "matching" => AnswerType::Matching,
        "ordering" => AnswerType::Ordering,
        _ => AnswerType::MultipleChoice,
    };

    let content = QuestionContent {
        text: question_text,
        rich_text: None,
        image_url: None,
        audio_url: None,
    };

    let question = Question::new(quiz_uuid, content, answer_type);
    let question_id = question.id;

    quiz.add_question(question);

    store.store_quiz(&quiz)
        .await
        .map_err(|e| e.to_string())?;

    Ok(question_id.to_string())
}

#[tauri::command]
pub async fn add_choice(
    quiz_id: String,
    question_id: String,
    choice_text: String,
    store: State<'_, Arc<HybridQuizStore>>,
) -> Result<String, String> {
    let quiz_uuid = Uuid::parse_str(&quiz_id).map_err(|e| e.to_string())?;
    let question_uuid = Uuid::parse_str(&question_id).map_err(|e| e.to_string())?;

    let mut quiz = store.get_quiz(quiz_uuid)
        .await
        .map_err(|e| e.to_string())?;

    let question = quiz.questions.iter_mut()
        .find(|q| q.id == question_uuid)
        .ok_or_else(|| format!("Question not found: {}", question_id))?;

    let choice_id = question.add_choice(choice_text);

    store.store_quiz(&quiz)
        .await
        .map_err(|e| e.to_string())?;

    Ok(choice_id.to_string())
}

#[tauri::command]
pub async fn set_correct_answer(
    quiz_id: String,
    question_id: String,
    answer_type: String,
    answer_value: String,
    store: State<'_, Arc<HybridQuizStore>>,
) -> Result<(), String> {
    let quiz_uuid = Uuid::parse_str(&quiz_id).map_err(|e| e.to_string())?;
    let question_uuid = Uuid::parse_str(&question_id).map_err(|e| e.to_string())?;

    let mut quiz = store.get_quiz(quiz_uuid)
        .await
        .map_err(|e| e.to_string())?;

    let question = quiz.questions.iter_mut()
        .find(|q| q.id == question_uuid)
        .ok_or_else(|| format!("Question not found: {}", question_id))?;

    let answer = match answer_type.as_str() {
        "choice" => {
            let choice_id = Uuid::parse_str(&answer_value).map_err(|e| e.to_string())?;
            Answer::Choice(choice_id)
        },
        "text" => Answer::Text(answer_value),
        _ => return Err(format!("Unsupported answer type: {}", answer_type)),
    };

    question.set_correct_answer(answer);

    store.store_quiz(&quiz)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn start_quiz_session(
    quiz_id: String,
    user_id: String,
    store: State<'_, Arc<HybridQuizStore>>,
) -> Result<QuizSession, String> {
    let quiz_uuid = Uuid::parse_str(&quiz_id).map_err(|e| e.to_string())?;
    let user_uuid = Uuid::parse_str(&user_id).map_err(|e| e.to_string())?;

    let quiz = store.get_quiz(quiz_uuid)
        .await
        .map_err(|e| e.to_string())?;

    let session = QuizSession::with_quiz(&quiz, user_uuid);

    store.store_session(&session)
        .await
        .map_err(|e| e.to_string())?;

    Ok(session)
}

#[tauri::command]
pub async fn submit_answer(
    session_id: String,
    question_id: String,
    answer_type: String,
    answer_value: String,
    store: State<'_, Arc<HybridQuizStore>>,
) -> Result<bool, String> {
    let session_uuid = Uuid::parse_str(&session_id).map_err(|e| e.to_string())?;
    let question_uuid = Uuid::parse_str(&question_id).map_err(|e| e.to_string())?;

    let mut session = store.get_session(session_uuid)
        .await
        .map_err(|e| e.to_string())?;

    let quiz = store.get_quiz(session.quiz_id)
        .await
        .map_err(|e| e.to_string())?;

    let answer = match answer_type.as_str() {
        "choice" => {
            let choice_id = Uuid::parse_str(&answer_value).map_err(|e| e.to_string())?;
            Answer::Choice(choice_id)
        },
        "text" => Answer::Text(answer_value),
        _ => return Err(format!("Unsupported answer type: {}", answer_type)),
    };

    let result = session.submit_answer(question_uuid, answer, &quiz)?;

    store.update_session(&session)
        .await
        .map_err(|e| e.to_string())?;

    Ok(result)
}

#[tauri::command]
pub async fn complete_session(
    session_id: String,
    store: State<'_, Arc<HybridQuizStore>>,
) -> Result<f32, String> {
    let session_uuid = Uuid::parse_str(&session_id).map_err(|e| e.to_string())?;

    let mut session = store.get_session(session_uuid)
        .await
        .map_err(|e| e.to_string())?;

    let score = session.complete()?;

    store.update_session(&session)
        .await
        .map_err(|e| e.to_string())?;

    Ok(score)
}

#[tauri::command]
pub async fn get_session_progress(
    session_id: String,
    store: State<'_, Arc<HybridQuizStore>>,
) -> Result<f32, String> {
    let session_uuid = Uuid::parse_str(&session_id).map_err(|e| e.to_string())?;

    let session = store.get_session(session_uuid)
        .await
        .map_err(|e| e.to_string())?;

    Ok(session.get_progress())
}

#[tauri::command]
pub async fn delete_quiz(
    quiz_id: String,
    store: State<'_, Arc<HybridQuizStore>>,
) -> Result<(), String> {
    let quiz_uuid = Uuid::parse_str(&quiz_id).map_err(|e| e.to_string())?;

    store.delete_quiz(quiz_uuid)
        .await
        .map_err(|e| e.to_string())
}

// Flashcard commands

#[tauri::command]
pub async fn rate_flashcard(
    question_id: String,
    user_id: String,
    rating: i32,
    engine: State<'_, QuizEngine>,
) -> Result<FlashcardData, String> {
    let question_uuid = Uuid::parse_str(&question_id).map_err(|e| e.to_string())?;
    let user_uuid = Uuid::parse_str(&user_id).map_err(|e| e.to_string())?;

    engine.rate_flashcard(question_uuid, user_uuid, rating)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_flashcard_session(
    user_id: String,
    limit: usize,
    engine: State<'_, QuizEngine>,
) -> Result<(QuizSession, Vec<Question>), String> {
    let user_uuid = Uuid::parse_str(&user_id).map_err(|e| e.to_string())?;

    engine.create_flashcard_session(user_uuid, limit)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_flashcard_stats(
    user_id: String,
    engine: State<'_, QuizEngine>,
) -> Result<super::spaced_repetition::FlashcardStatistics, String> {
    let user_uuid = Uuid::parse_str(&user_id).map_err(|e| e.to_string())?;

    engine.get_flashcard_stats(user_uuid)
        .await
        .map_err(|e| e.to_string())
}

// Analytics commands

#[tauri::command]
pub async fn get_user_stats(
    user_id: String,
    period: String,
    engine: State<'_, QuizEngine>,
) -> Result<UserStudyStats, String> {
    let user_uuid = Uuid::parse_str(&user_id).map_err(|e| e.to_string())?;

    let period = match period.as_str() {
        "day" => TimePeriod::Day,
        "week" => TimePeriod::Week,
        "month" => TimePeriod::Month,
        "year" => TimePeriod::Year,
        _ => TimePeriod::AllTime,
    };

    engine.get_user_stats(user_uuid, period)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_quiz_analytics(
    quiz_id: String,
    period: String,
    engine: State<'_, QuizEngine>,
) -> Result<QuizAnalytics, String> {
    let quiz_uuid = Uuid::parse_str(&quiz_id).map_err(|e| e.to_string())?;

    let period = match period.as_str() {
        "day" => TimePeriod::Day,
        "week" => TimePeriod::Week,
        "month" => TimePeriod::Month,
        "year" => TimePeriod::Year,
        _ => TimePeriod::AllTime,
    };

    engine.get_quiz_analytics(quiz_uuid, period)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn generate_user_report(
    user_id: String,
    period: String,
    engine: State<'_, QuizEngine>,
) -> Result<Vec<u8>, String> {
    let user_uuid = Uuid::parse_str(&user_id).map_err(|e| e.to_string())?;

    let period = match period.as_str() {
        "day" => TimePeriod::Day,
        "week" => TimePeriod::Week,
        "month" => TimePeriod::Month,
        "year" => TimePeriod::Year,
        _ => TimePeriod::AllTime,
    };

    engine.generate_user_report(user_uuid, period)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn generate_quiz_report(
    quiz_id: String,
    period: String,
    engine: State<'_, QuizEngine>,
) -> Result<Vec<u8>, String> {
    let quiz_uuid = Uuid::parse_str(&quiz_id).map_err(|e| e.to_string())?;

    let period = match period.as_str() {
        "day" => TimePeriod::Day,
        "week" => TimePeriod::Week,
        "month" => TimePeriod::Month,
        "year" => TimePeriod::Year,
        _ => TimePeriod::AllTime,
    };

    engine.generate_quiz_report(quiz_uuid, period)
        .await
        .map_err(|e| e.to_string())
}

// Notification commands

#[tauri::command]
pub async fn get_quiz_notifications(
    user_id: String,
    limit: Option<i64>,
    offset: Option<i64>,
    engine: State<'_, QuizEngine>,
) -> Result<Vec<serde_json::Value>, String> {
    let user_uuid = Uuid::parse_str(&user_id).map_err(|e| e.to_string())?;

    let notifications = engine.get_notifications_for_user(user_uuid, limit, offset)
        .await
        .map_err(|e| e.to_string())?;

    let result = notifications.into_iter()
        .map(|n| serde_json::to_value(n))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    Ok(result)
}

#[tauri::command]
pub async fn get_unread_notification_count(
    user_id: String,
    engine: State<'_, QuizEngine>,
) -> Result<i64, String> {
    let user_uuid = Uuid::parse_str(&user_id).map_err(|e| e.to_string())?;

    engine.get_unread_count_for_user(user_uuid)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn mark_notification_as_read(
    notification_id: String,
    engine: State<'_, QuizEngine>,
) -> Result<(), String> {
    let notification_uuid = Uuid::parse_str(&notification_id).map_err(|e| e.to_string())?;

    engine.mark_notification_as_read(notification_uuid)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn mark_all_notifications_as_read(
    user_id: String,
    engine: State<'_, QuizEngine>,
) -> Result<(), String> {
    let user_uuid = Uuid::parse_str(&user_id).map_err(|e| e.to_string())?;

    engine.mark_all_notifications_as_read(user_uuid)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_notification(
    notification_id: String,
    engine: State<'_, QuizEngine>,
) -> Result<(), String> {
    let notification_uuid = Uuid::parse_str(&notification_id).map_err(|e| e.to_string())?;

    engine.delete_notification(notification_uuid)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_all_notifications(
    user_id: String,
    engine: State<'_, QuizEngine>,
) -> Result<(), String> {
    let user_uuid = Uuid::parse_str(&user_id).map_err(|e| e.to_string())?;

    engine.delete_all_notifications_for_user(user_uuid)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn check_quiz_notifications(
    engine: State<'_, QuizEngine>,
) -> Result<(), String> {
    // Check for due soon quizzes
    engine.check_due_soon_quizzes()
        .await
        .map_err(|e| e.to_string())?;

    // Check for overdue quizzes
    engine.check_overdue_quizzes()
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

// Course Integration commands

#[tauri::command]
pub async fn add_quiz_to_course(
    quiz_id: String,
    course_id: String,
    module_id: Option<String>,
    section_id: Option<String>,
    position: Option<i32>,
    engine: State<'_, QuizEngine>,
) -> Result<serde_json::Value, String> {
    let quiz_uuid = Uuid::parse_str(&quiz_id).map_err(|e| e.to_string())?;
    let course_uuid = Uuid::parse_str(&course_id).map_err(|e| e.to_string())?;

    let module_uuid = if let Some(id) = module_id {
        Some(Uuid::parse_str(&id).map_err(|e| e.to_string())?)
    } else {
        None
    };

    let section_uuid = if let Some(id) = section_id {
        Some(Uuid::parse_str(&id).map_err(|e| e.to_string())?)
    } else {
        None
    };

    let mapping = engine.add_quiz_to_course(quiz_uuid, course_uuid, module_uuid, section_uuid, position)
        .await
        .map_err(|e| e.to_string())?;

    serde_json::to_value(mapping).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn remove_quiz_from_course(
    mapping_id: String,
    engine: State<'_, QuizEngine>,
) -> Result<(), String> {
    let mapping_uuid = Uuid::parse_str(&mapping_id).map_err(|e| e.to_string())?;

    engine.remove_quiz_from_course(mapping_uuid)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_quiz_course_mapping(
    mapping: serde_json::Value,
    engine: State<'_, QuizEngine>,
) -> Result<(), String> {
    let mapping: course_integration::QuizCourseMapping = serde_json::from_value(mapping)
        .map_err(|e| e.to_string())?;

    engine.update_quiz_course_mapping(&mapping)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_quizzes_for_course(
    course_id: String,
    engine: State<'_, QuizEngine>,
) -> Result<Vec<serde_json::Value>, String> {
    let course_uuid = Uuid::parse_str(&course_id).map_err(|e| e.to_string())?;

    let mappings = engine.get_quizzes_for_course(course_uuid)
        .await
        .map_err(|e| e.to_string())?;

    let result = mappings.into_iter()
        .map(|m| serde_json::to_value(m))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    Ok(result)
}

#[tauri::command]
pub async fn get_courses_for_quiz(
    quiz_id: String,
    engine: State<'_, QuizEngine>,
) -> Result<Vec<serde_json::Value>, String> {
    let quiz_uuid = Uuid::parse_str(&quiz_id).map_err(|e| e.to_string())?;

    let courses = engine.get_courses_for_quiz(quiz_uuid)
        .await
        .map_err(|e| e.to_string())?;

    let result = courses.into_iter()
        .map(|(m, c)| {
            let mut obj = serde_json::Map::new();
            obj.insert("mapping".to_string(), serde_json::to_value(m).unwrap());
            obj.insert("course".to_string(), serde_json::to_value(c).unwrap());
            serde_json::Value::Object(obj)
        })
        .collect::<Vec<_>>();

    Ok(result)
}

#[tauri::command]
pub async fn get_quiz_with_context(
    mapping_id: String,
    student_id: Option<String>,
    engine: State<'_, QuizEngine>,
) -> Result<serde_json::Value, String> {
    let mapping_uuid = Uuid::parse_str(&mapping_id).map_err(|e| e.to_string())?;

    let student_uuid = if let Some(id) = student_id {
        Some(Uuid::parse_str(&id).map_err(|e| e.to_string())?)
    } else {
        None
    };

    let context = engine.get_quiz_with_context(mapping_uuid, student_uuid)
        .await
        .map_err(|e| e.to_string())?;

    serde_json::to_value(context).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_student_quizzes(
    course_id: String,
    student_id: String,
    engine: State<'_, QuizEngine>,
) -> Result<Vec<serde_json::Value>, String> {
    let course_uuid = Uuid::parse_str(&course_id).map_err(|e| e.to_string())?;
    let student_uuid = Uuid::parse_str(&student_id).map_err(|e| e.to_string())?;

    let quizzes = engine.get_student_quizzes(course_uuid, student_uuid)
        .await
        .map_err(|e| e.to_string())?;

    let result = quizzes.into_iter()
        .map(|q| serde_json::to_value(q))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    Ok(result)
}

#[tauri::command]
pub async fn assign_quiz_to_student(
    mapping_id: String,
    student_id: String,
    engine: State<'_, QuizEngine>,
) -> Result<serde_json::Value, String> {
    let mapping_uuid = Uuid::parse_str(&mapping_id).map_err(|e| e.to_string())?;
    let student_uuid = Uuid::parse_str(&student_id).map_err(|e| e.to_string())?;

    let assignment = engine.assign_quiz_to_student(mapping_uuid, student_uuid)
        .await
        .map_err(|e| e.to_string())?;

    serde_json::to_value(assignment).map_err(|e| e.to_string())
}

// Export/Import commands

#[tauri::command]
pub async fn export_quiz(
    quiz_id: String,
    format: String,
    engine: State<'_, QuizEngine>,
) -> Result<Vec<u8>, String> {
    let quiz_uuid = Uuid::parse_str(&quiz_id).map_err(|e| e.to_string())?;

    let export_format = match format.as_str() {
        "json" => ExportFormat::Json,
        "csv" => ExportFormat::Csv,
        "markdown" => ExportFormat::Markdown,
        "anki" => ExportFormat::Anki,
        "quizlet" => ExportFormat::Quizlet,
        _ => ExportFormat::Json,
    };

    engine.export_quiz(quiz_uuid, export_format).await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn export_quiz_to_file(
    quiz_id: String,
    format: String,
    file_path: Option<String>,
    engine: State<'_, QuizEngine>,
) -> Result<String, String> {
    let quiz_uuid = Uuid::parse_str(&quiz_id).map_err(|e| e.to_string())?;

    let export_format = match format.as_str() {
        "json" => ExportFormat::Json,
        "csv" => ExportFormat::Csv,
        "markdown" => ExportFormat::Markdown,
        "anki" => ExportFormat::Anki,
        "quizlet" => ExportFormat::Quizlet,
        _ => ExportFormat::Json,
    };

    // Determine file extension
    let extension = match export_format {
        ExportFormat::Json => "json",
        ExportFormat::Csv => "csv",
        ExportFormat::Markdown => "md",
        ExportFormat::Anki => "txt",
        ExportFormat::Quizlet => "txt",
    };

    // Get the path to save the file
    let path = if let Some(path) = file_path {
        PathBuf::from(path)
    } else {
        // Use the downloads directory
        let downloads_dir = path::download_dir().ok_or_else(|| "Could not find downloads directory".to_string())?;
        downloads_dir.join(format!("quiz_{}.{}", quiz_id, extension))
    };

    engine.export_quiz_to_file(quiz_uuid, &path, export_format).await
        .map_err(|e| e.to_string())?;

    Ok(path.to_string_lossy().to_string())
}

#[tauri::command]
pub async fn export_quiz_with_options(
    quiz_id: String,
    format: String,
    include_explanations: bool,
    include_metadata: bool,
    include_statistics: bool,
    include_images: bool,
    include_audio: bool,
    engine: State<'_, QuizEngine>,
) -> Result<Vec<u8>, String> {
    let quiz_uuid = Uuid::parse_str(&quiz_id).map_err(|e| e.to_string())?;

    let export_format = match format.as_str() {
        "json" => ExportFormat::Json,
        "csv" => ExportFormat::Csv,
        "markdown" => ExportFormat::Markdown,
        "anki" => ExportFormat::Anki,
        "quizlet" => ExportFormat::Quizlet,
        _ => ExportFormat::Json,
    };

    let options = ExportOptions {
        format: export_format,
        include_explanations,
        include_metadata,
        include_statistics,
        include_images,
        include_audio,
    };

    engine.export_quiz_with_options(quiz_uuid, options).await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn import_quiz_from_file(
    file_path: String,
    engine: State<'_, QuizEngine>,
) -> Result<String, String> {
    let path = PathBuf::from(file_path);

    let quiz_id = engine.import_quiz_from_file(&path).await
        .map_err(|e| e.to_string())?;

    Ok(quiz_id.to_string())
}

#[tauri::command]
pub async fn import_quiz(
    data: Vec<u8>,
    format: String,
    engine: State<'_, QuizEngine>,
) -> Result<String, String> {
    let import_format = match format.as_str() {
        "json" => ExportFormat::Json,
        "csv" => ExportFormat::Csv,
        "markdown" => ExportFormat::Markdown,
        "anki" => ExportFormat::Anki,
        "quizlet" => ExportFormat::Quizlet,
        _ => ExportFormat::Json,
    };

    let quiz_id = engine.import_quiz(&data, import_format).await
        .map_err(|e| e.to_string())?;

    Ok(quiz_id.to_string())
}