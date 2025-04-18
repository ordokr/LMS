use super::ai_generation::{AIModelProvider, AIModelType, AIGenerationRequest};
use std::error::Error;

/// A mock AI model provider for testing
pub struct MockAIModelProvider;

impl AIModelProvider for MockAIModelProvider {
    fn generate_quiz(&self, request: &AIGenerationRequest) -> Result<serde_json::Value, Box<dyn Error + Send + Sync>> {
        // Create a mock response with the requested number of questions
        let mut questions = Vec::new();
        
        for i in 0..request.num_questions {
            let question_type = request.question_types.get(i as usize % request.question_types.len())
                .unwrap_or(&super::models::AnswerType::MultipleChoice);
            
            let question = match question_type {
                super::models::AnswerType::MultipleChoice => {
                    json!({
                        "text": format!("Sample multiple choice question #{}", i + 1),
                        "description": format!("This is a sample question about {}", request.topic_focus.clone().unwrap_or_else(|| "general knowledge".to_string())),
                        "type": "multiple_choice",
                        "points": 1,
                        "answers": [
                            {
                                "text": "Correct answer",
                                "correct": true
                            },
                            {
                                "text": "Wrong answer 1",
                                "correct": false
                            },
                            {
                                "text": "Wrong answer 2",
                                "correct": false
                            },
                            {
                                "text": "Wrong answer 3",
                                "correct": false
                            }
                        ]
                    })
                },
                super::models::AnswerType::TrueFalse => {
                    json!({
                        "text": format!("Sample true/false question #{}", i + 1),
                        "description": format!("This is a sample question about {}", request.topic_focus.clone().unwrap_or_else(|| "general knowledge".to_string())),
                        "type": "true_false",
                        "points": 1,
                        "answers": [
                            {
                                "text": "True",
                                "correct": true
                            },
                            {
                                "text": "False",
                                "correct": false
                            }
                        ]
                    })
                },
                super::models::AnswerType::ShortAnswer => {
                    json!({
                        "text": format!("Sample short answer question #{}", i + 1),
                        "description": format!("This is a sample question about {}", request.topic_focus.clone().unwrap_or_else(|| "general knowledge".to_string())),
                        "type": "short_answer",
                        "points": 1,
                        "answers": [
                            {
                                "text": "Sample answer",
                                "correct": true
                            }
                        ]
                    })
                },
                _ => {
                    json!({
                        "text": format!("Sample question #{}", i + 1),
                        "description": format!("This is a sample question about {}", request.topic_focus.clone().unwrap_or_else(|| "general knowledge".to_string())),
                        "type": "multiple_choice",
                        "points": 1,
                        "answers": [
                            {
                                "text": "Correct answer",
                                "correct": true
                            },
                            {
                                "text": "Wrong answer 1",
                                "correct": false
                            },
                            {
                                "text": "Wrong answer 2",
                                "correct": false
                            },
                            {
                                "text": "Wrong answer 3",
                                "correct": false
                            }
                        ]
                    })
                }
            };
            
            questions.push(question);
        }
        
        // Create the response
        let response = json!({
            "title": request.title,
            "description": request.description,
            "questions": questions,
            "metadata": {
                "source_type": format!("{:?}", request.source_type),
                "model_type": format!("{:?}", request.model_type),
                "difficulty_level": request.difficulty_level,
                "language": request.language,
                "generated_at": chrono::Utc::now().to_rfc3339()
            }
        });
        
        Ok(response)
    }
    
    fn get_name(&self) -> String {
        "Mock AI Provider".to_string()
    }
    
    fn get_type(&self) -> AIModelType {
        AIModelType::Custom
    }
}

/// OpenAI model provider
pub struct OpenAIModelProvider {
    api_key: String,
    model: String,
}

impl OpenAIModelProvider {
    pub fn new(api_key: String, model: String) -> Self {
        Self {
            api_key,
            model,
        }
    }
}

impl AIModelProvider for OpenAIModelProvider {
    fn generate_quiz(&self, request: &AIGenerationRequest) -> Result<serde_json::Value, Box<dyn Error + Send + Sync>> {
        // In a real implementation, this would make an API call to OpenAI
        // For now, we'll just return a mock response
        
        // Create a system prompt based on the request
        let system_prompt = format!(
            "You are a quiz generator. Create a quiz with {} questions about the following topic: {}. 
            The difficulty level is {} out of 5. 
            The quiz should be in {} language.
            Question types: {}.",
            request.num_questions,
            request.topic_focus.clone().unwrap_or_else(|| "general knowledge".to_string()),
            request.difficulty_level,
            request.language,
            request.question_types.iter().map(|qt| format!("{:?}", qt)).collect::<Vec<_>>().join(", ")
        );
        
        // Create a user prompt based on the source content
        let user_prompt = format!(
            "Generate a quiz based on this content: {}",
            request.source_content
        );
        
        // Log what would be sent to OpenAI
        println!("Would send to OpenAI:");
        println!("Model: {}", self.model);
        println!("System prompt: {}", system_prompt);
        println!("User prompt: {}", user_prompt);
        
        // For now, return a mock response
        let mock_provider = MockAIModelProvider;
        mock_provider.generate_quiz(request)
    }
    
    fn get_name(&self) -> String {
        format!("OpenAI ({})", self.model)
    }
    
    fn get_type(&self) -> AIModelType {
        AIModelType::OpenAI
    }
}

/// Anthropic model provider
pub struct AnthropicModelProvider {
    api_key: String,
    model: String,
}

impl AnthropicModelProvider {
    pub fn new(api_key: String, model: String) -> Self {
        Self {
            api_key,
            model,
        }
    }
}

impl AIModelProvider for AnthropicModelProvider {
    fn generate_quiz(&self, request: &AIGenerationRequest) -> Result<serde_json::Value, Box<dyn Error + Send + Sync>> {
        // In a real implementation, this would make an API call to Anthropic
        // For now, we'll just return a mock response
        
        // Create a system prompt based on the request
        let system_prompt = format!(
            "You are a quiz generator. Create a quiz with {} questions about the following topic: {}. 
            The difficulty level is {} out of 5. 
            The quiz should be in {} language.
            Question types: {}.",
            request.num_questions,
            request.topic_focus.clone().unwrap_or_else(|| "general knowledge".to_string()),
            request.difficulty_level,
            request.language,
            request.question_types.iter().map(|qt| format!("{:?}", qt)).collect::<Vec<_>>().join(", ")
        );
        
        // Create a user prompt based on the source content
        let user_prompt = format!(
            "Generate a quiz based on this content: {}",
            request.source_content
        );
        
        // Log what would be sent to Anthropic
        println!("Would send to Anthropic:");
        println!("Model: {}", self.model);
        println!("System prompt: {}", system_prompt);
        println!("User prompt: {}", user_prompt);
        
        // For now, return a mock response
        let mock_provider = MockAIModelProvider;
        mock_provider.generate_quiz(request)
    }
    
    fn get_name(&self) -> String {
        format!("Anthropic ({})", self.model)
    }
    
    fn get_type(&self) -> AIModelType {
        AIModelType::Anthropic
    }
}

// Helper macro for creating JSON
#[macro_export]
macro_rules! json {
    ($($json:tt)+) => {
        serde_json::json!($($json)+)
    };
}
