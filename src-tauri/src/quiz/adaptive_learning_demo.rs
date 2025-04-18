use super::adaptive_learning::{AdaptiveLearningService, LearningPathNodeType, EdgeConditionType};
use super::models::{StudyMode, QuizVisibility};
use uuid::Uuid;
use std::sync::Arc;
use log::{info, error};

/// Creates a sample adaptive learning path for demonstration purposes
pub async fn create_demo_learning_path(service: Arc<AdaptiveLearningService>) {
    info!("Creating demo adaptive learning path...");
    
    // Create a sample author ID
    let author_id = Uuid::new_v4();
    
    // Create a learning path
    match service.create_path(
        "Introduction to Programming".to_string(),
        Some("A beginner-friendly path to learn programming concepts".to_string()),
        Some(author_id),
        "Computer Science".to_string(),
        vec!["programming".to_string(), "beginner".to_string()],
        StudyMode::MultipleChoice,
        QuizVisibility::Public,
        true
    ).await {
        Ok(path) => {
            info!("Created demo learning path with ID: {}", path.id);
            
            // Add nodes
            let start_node = match service.add_node(
                path.id,
                "Start".to_string(),
                Some("Beginning of your programming journey".to_string()),
                LearningPathNodeType::Start,
                None,
                100.0,
                100.0,
                None,
                None
            ).await {
                Ok(node) => {
                    info!("Created start node with ID: {}", node.id);
                    node
                },
                Err(e) => {
                    error!("Failed to create start node: {}", e);
                    return;
                }
            };
            
            let variables_node = match service.add_node(
                path.id,
                "Variables and Data Types".to_string(),
                Some("Learn about variables and data types in programming".to_string()),
                LearningPathNodeType::Content,
                None,
                300.0,
                100.0,
                None,
                None
            ).await {
                Ok(node) => {
                    info!("Created variables node with ID: {}", node.id);
                    node
                },
                Err(e) => {
                    error!("Failed to create variables node: {}", e);
                    return;
                }
            };
            
            let quiz_node = match service.add_node(
                path.id,
                "Variables Quiz".to_string(),
                Some("Test your knowledge of variables and data types".to_string()),
                LearningPathNodeType::Quiz,
                None,
                500.0,
                100.0,
                Some(0.7),
                None
            ).await {
                Ok(node) => {
                    info!("Created quiz node with ID: {}", node.id);
                    node
                },
                Err(e) => {
                    error!("Failed to create quiz node: {}", e);
                    return;
                }
            };
            
            let conditionals_node = match service.add_node(
                path.id,
                "Conditional Statements".to_string(),
                Some("Learn about if/else statements and logical operators".to_string()),
                LearningPathNodeType::Content,
                None,
                700.0,
                100.0,
                None,
                None
            ).await {
                Ok(node) => {
                    info!("Created conditionals node with ID: {}", node.id);
                    node
                },
                Err(e) => {
                    error!("Failed to create conditionals node: {}", e);
                    return;
                }
            };
            
            let end_node = match service.add_node(
                path.id,
                "Completion".to_string(),
                Some("You've completed the introduction to programming!".to_string()),
                LearningPathNodeType::End,
                None,
                900.0,
                100.0,
                None,
                None
            ).await {
                Ok(node) => {
                    info!("Created end node with ID: {}", node.id);
                    node
                },
                Err(e) => {
                    error!("Failed to create end node: {}", e);
                    return;
                }
            };
            
            // Add edges
            match service.add_edge(
                path.id,
                start_node.id,
                variables_node.id,
                EdgeConditionType::Completion,
                None,
                Some("Start -> Variables".to_string())
            ).await {
                Ok(_) => info!("Created edge: Start -> Variables"),
                Err(e) => error!("Failed to create edge: {}", e)
            }
            
            match service.add_edge(
                path.id,
                variables_node.id,
                quiz_node.id,
                EdgeConditionType::Completion,
                None,
                Some("Variables -> Quiz".to_string())
            ).await {
                Ok(_) => info!("Created edge: Variables -> Quiz"),
                Err(e) => error!("Failed to create edge: {}", e)
            }
            
            match service.add_edge(
                path.id,
                quiz_node.id,
                conditionals_node.id,
                EdgeConditionType::Score,
                Some(serde_json::json!({ "min_score": 0.7 })),
                Some("Quiz -> Conditionals (Score >= 70%)".to_string())
            ).await {
                Ok(_) => info!("Created edge: Quiz -> Conditionals"),
                Err(e) => error!("Failed to create edge: {}", e)
            }
            
            match service.add_edge(
                path.id,
                conditionals_node.id,
                end_node.id,
                EdgeConditionType::Completion,
                None,
                Some("Conditionals -> End".to_string())
            ).await {
                Ok(_) => info!("Created edge: Conditionals -> End"),
                Err(e) => error!("Failed to create edge: {}", e)
            }
            
            info!("Demo adaptive learning path created successfully!");
        },
        Err(e) => {
            error!("Failed to create demo learning path: {}", e);
        }
    }
}
