use super::ai_generation::{AIGenerationService, AIGenerationRequest};
use super::models::{Quiz, Question, Answer};
use uuid::Uuid;
use std::error::Error;
use chrono::Utc;

impl AIGenerationService {
    /// Create a quiz from an AI generation response
    pub async fn create_quiz_from_response(
        &self,
        request: &AIGenerationRequest,
        response: &serde_json::Value,
    ) -> Result<Uuid, Box<dyn Error + Send + Sync>> {
        // Parse the response to extract quiz data
        // This is a generic implementation that expects a specific JSON structure
        // Real implementations would adapt to the specific AI model's response format
        
        // Extract quiz title and description
        let title = request.title.clone();
        let description = request.description.clone();
        
        // Create a new quiz
        let quiz_id = Uuid::new_v4();
        let now = Utc::now();
        
        let mut quiz = Quiz {
            id: quiz_id,
            title,
            description,
            author_id: request.user_id,
            created_at: Some(now),
            updated_at: Some(now),
            questions: Vec::new(),
            study_mode: request.study_mode.clone(),
            visibility: request.visibility.clone(),
            settings: Default::default(),
        };
        
        // Try to extract questions from the response
        if let Some(questions_array) = response.get("questions").and_then(|q| q.as_array()) {
            for (index, question_value) in questions_array.iter().enumerate() {
                if let Some(question) = self.parse_question(question_value, quiz_id, index as i32) {
                    quiz.questions.push(question);
                }
            }
        } else {
            // Fallback: try to extract questions from the top level
            // This is a simplified approach - real implementations would be more robust
            if let Some(question_text) = response.get("question").and_then(|q| q.as_str()) {
                let question_id = Uuid::new_v4();
                let mut question = Question {
                    id: question_id,
                    quiz_id,
                    text: question_text.to_string(),
                    description: None,
                    answer_type: request.question_types.first().cloned().unwrap_or_default(),
                    answers: Vec::new(),
                    position: 0,
                    points: 1,
                    content: Default::default(),
                    created_at: Some(now),
                    updated_at: Some(now),
                };
                
                // Try to extract answers
                if let Some(answers_array) = response.get("answers").and_then(|a| a.as_array()) {
                    for (answer_index, answer_value) in answers_array.iter().enumerate() {
                        if let Some(answer_text) = answer_value.get("text").and_then(|t| t.as_str()) {
                            let is_correct = answer_value.get("correct").and_then(|c| c.as_bool()).unwrap_or(false);
                            
                            let answer = Answer {
                                id: Uuid::new_v4(),
                                question_id,
                                text: answer_text.to_string(),
                                is_correct,
                                position: answer_index as i32,
                                created_at: Some(now),
                                updated_at: Some(now),
                            };
                            
                            question.answers.push(answer);
                        }
                    }
                }
                
                quiz.questions.push(question);
            }
        }
        
        // Check if we have any questions
        if quiz.questions.is_empty() {
            return Err("Failed to extract questions from AI response".into());
        }
        
        // Save the quiz
        self.quiz_store.save_quiz(&quiz).await?;
        
        Ok(quiz_id)
    }
    
    /// Parse a question from a JSON value
    fn parse_question(&self, question_value: &serde_json::Value, quiz_id: Uuid, position: i32) -> Option<Question> {
        let question_text = question_value.get("text").and_then(|t| t.as_str())?;
        let question_id = Uuid::new_v4();
        let now = Utc::now();
        
        // Try to get the answer type
        let answer_type_str = question_value.get("type").and_then(|t| t.as_str()).unwrap_or("multiple_choice");
        let answer_type = match answer_type_str {
            "multiple_choice" => super::models::AnswerType::MultipleChoice,
            "true_false" => super::models::AnswerType::TrueFalse,
            "short_answer" => super::models::AnswerType::ShortAnswer,
            "essay" => super::models::AnswerType::Essay,
            "matching" => super::models::AnswerType::Matching,
            "ordering" => super::models::AnswerType::Ordering,
            _ => super::models::AnswerType::MultipleChoice,
        };
        
        let mut question = Question {
            id: question_id,
            quiz_id,
            text: question_text.to_string(),
            description: question_value.get("description").and_then(|d| d.as_str()).map(|s| s.to_string()),
            answer_type,
            answers: Vec::new(),
            position,
            points: question_value.get("points").and_then(|p| p.as_i64()).unwrap_or(1) as i32,
            content: Default::default(),
            created_at: Some(now),
            updated_at: Some(now),
        };
        
        // Try to extract answers
        if let Some(answers_array) = question_value.get("answers").and_then(|a| a.as_array()) {
            for (answer_index, answer_value) in answers_array.iter().enumerate() {
                if let Some(answer_text) = answer_value.get("text").and_then(|t| t.as_str()) {
                    let is_correct = answer_value.get("correct").and_then(|c| c.as_bool()).unwrap_or(false);
                    
                    let answer = Answer {
                        id: Uuid::new_v4(),
                        question_id,
                        text: answer_text.to_string(),
                        is_correct,
                        position: answer_index as i32,
                        created_at: Some(now),
                        updated_at: Some(now),
                    };
                    
                    question.answers.push(answer);
                }
            }
        }
        
        Some(question)
    }
}
