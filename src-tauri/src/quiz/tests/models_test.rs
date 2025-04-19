#[cfg(test)]
mod tests {
    use crate::quiz::models::{Quiz, Question, Answer, QuestionContent, AnswerType, StudyMode, QuizVisibility};
    use uuid::Uuid;
    use chrono::Utc;

    #[test]
    fn test_quiz_creation() {
        let quiz_id = Uuid::new_v4();
        let author_id = Uuid::new_v4();
        let title = "Test Quiz".to_string();
        
        let quiz = Quiz {
            id: quiz_id,
            title: title.clone(),
            description: Some("Test Description".to_string()),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            questions: Vec::new(),
            settings: Default::default(),
            author_id: Some(author_id),
            visibility: QuizVisibility::Public,
            tags: vec!["test".to_string(), "unit".to_string()],
            study_mode: StudyMode::MultipleChoice,
        };
        
        assert_eq!(quiz.id, quiz_id);
        assert_eq!(quiz.title, title);
        assert_eq!(quiz.author_id, Some(author_id));
        assert_eq!(quiz.visibility, QuizVisibility::Public);
        assert_eq!(quiz.study_mode, StudyMode::MultipleChoice);
        assert_eq!(quiz.questions.len(), 0);
        assert_eq!(quiz.tags.len(), 2);
    }
    
    #[test]
    fn test_question_creation() {
        let quiz_id = Uuid::new_v4();
        let question_id = Uuid::new_v4();
        
        let content = QuestionContent {
            text: "What is the capital of France?".to_string(),
            rich_text: Some("<p>What is the capital of <strong>France</strong>?</p>".to_string()),
            image_url: None,
            audio_url: None,
        };
        
        let choices = vec![
            crate::quiz::models::Choice {
                id: Uuid::new_v4(),
                text: "Paris".to_string(),
                rich_text: None,
                image_url: None,
            },
            crate::quiz::models::Choice {
                id: Uuid::new_v4(),
                text: "London".to_string(),
                rich_text: None,
                image_url: None,
            },
            crate::quiz::models::Choice {
                id: Uuid::new_v4(),
                text: "Berlin".to_string(),
                rich_text: None,
                image_url: None,
            },
        ];
        
        let correct_answer = Answer::SingleChoice(choices[0].id);
        
        let question = Question {
            id: question_id,
            quiz_id,
            content,
            answer_type: AnswerType::MultipleChoice,
            choices: choices.clone(),
            correct_answer,
            explanation: Some("Paris is the capital of France.".to_string()),
        };
        
        assert_eq!(question.id, question_id);
        assert_eq!(question.quiz_id, quiz_id);
        assert_eq!(question.answer_type, AnswerType::MultipleChoice);
        assert_eq!(question.choices.len(), 3);
        
        // Test content rendering
        assert!(question.content.render().contains("France"));
    }
    
    #[test]
    fn test_answer_validation() {
        let choice_id = Uuid::new_v4();
        let correct_answer = Answer::SingleChoice(choice_id);
        
        // Test correct answer
        let user_answer = Answer::SingleChoice(choice_id);
        assert!(correct_answer.validate(&user_answer));
        
        // Test incorrect answer
        let wrong_choice_id = Uuid::new_v4();
        let wrong_answer = Answer::SingleChoice(wrong_choice_id);
        assert!(!correct_answer.validate(&wrong_answer));
        
        // Test multiple choice answers
        let choices = vec![Uuid::new_v4(), Uuid::new_v4()];
        let correct_multiple = Answer::MultipleChoice(choices.clone());
        
        // Correct order shouldn't matter
        let user_multiple = Answer::MultipleChoice(vec![choices[1], choices[0]]);
        assert!(correct_multiple.validate(&user_multiple));
        
        // Missing choice should fail
        let incomplete = Answer::MultipleChoice(vec![choices[0]]);
        assert!(!correct_multiple.validate(&incomplete));
        
        // Extra choice should fail
        let extra = Answer::MultipleChoice(vec![choices[0], choices[1], Uuid::new_v4()]);
        assert!(!correct_multiple.validate(&extra));
        
        // Different answer types should fail
        assert!(!correct_answer.validate(&correct_multiple));
    }
    
    #[test]
    fn test_quiz_visibility() {
        assert_eq!(QuizVisibility::Public.as_str(), "public");
        assert_eq!(QuizVisibility::Private.as_str(), "private");
        assert_eq!(QuizVisibility::Unlisted.as_str(), "unlisted");
        assert_eq!(QuizVisibility::Class.as_str(), "class");
        
        // Test from_str
        assert_eq!(QuizVisibility::from_str("public"), QuizVisibility::Public);
        assert_eq!(QuizVisibility::from_str("private"), QuizVisibility::Private);
        assert_eq!(QuizVisibility::from_str("unlisted"), QuizVisibility::Unlisted);
        assert_eq!(QuizVisibility::from_str("class"), QuizVisibility::Class);
        assert_eq!(QuizVisibility::from_str("unknown"), QuizVisibility::Private); // Default
    }
    
    #[test]
    fn test_study_mode() {
        assert_eq!(StudyMode::Flashcards.as_str(), "flashcards");
        assert_eq!(StudyMode::MultipleChoice.as_str(), "multiple_choice");
        assert_eq!(StudyMode::Written.as_str(), "written");
        assert_eq!(StudyMode::Mixed.as_str(), "mixed");
        
        // Test from_str
        assert_eq!(StudyMode::from_str("flashcards"), StudyMode::Flashcards);
        assert_eq!(StudyMode::from_str("multiple_choice"), StudyMode::MultipleChoice);
        assert_eq!(StudyMode::from_str("written"), StudyMode::Written);
        assert_eq!(StudyMode::from_str("mixed"), StudyMode::Mixed);
        assert_eq!(StudyMode::from_str("unknown"), StudyMode::MultipleChoice); // Default
    }
}
