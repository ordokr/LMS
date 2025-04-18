#[cfg(test)]
mod tests {
    use crate::core::config::Config;
    use crate::quiz::adaptive_learning::{AdaptiveLearningService, LearningPathNodeType, EdgeConditionType};
    use crate::quiz::models::{StudyMode, QuizVisibility};
    use crate::quiz::storage::HybridQuizStore;
    use std::sync::Arc;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_adaptive_learning_path_creation() {
        // Create a test config with in-memory database
        let config = Config {
            database: crate::core::config::DatabaseConfig {
                path: "sqlite::memory:".to_string(),
                max_connections: 5,
                ..Default::default()
            },
            ..Default::default()
        };

        // Create a store
        let store = Arc::new(HybridQuizStore::new(&config).unwrap());

        // Create an adaptive learning service
        let service = AdaptiveLearningService::new(
            store.get_sqlite_pool().clone(),
            store.clone()
        );

        // Create a learning path
        let path = service.create_path(
            "Test Learning Path".to_string(),
            Some("A test learning path".to_string()),
            Some(Uuid::new_v4()),
            "Computer Science".to_string(),
            vec!["test".to_string(), "learning".to_string()],
            StudyMode::MultipleChoice,
            QuizVisibility::Private,
            false
        ).await.unwrap();

        // Verify the path was created
        assert_eq!(path.title, "Test Learning Path");
        assert_eq!(path.subject, "Computer Science");
        assert_eq!(path.tags, vec!["test".to_string(), "learning".to_string()]);
        assert_eq!(path.nodes.len(), 0);
        assert_eq!(path.edges.len(), 0);

        // Add a start node
        let start_node = service.add_node(
            path.id,
            "Start".to_string(),
            Some("Start of the learning path".to_string()),
            LearningPathNodeType::Start,
            None,
            0.0,
            0.0,
            None,
            None
        ).await.unwrap();

        // Add a content node
        let content_node = service.add_node(
            path.id,
            "Introduction to Programming".to_string(),
            Some("Learn the basics of programming".to_string()),
            LearningPathNodeType::Content,
            Some(Uuid::new_v4()),
            100.0,
            100.0,
            None,
            None
        ).await.unwrap();

        // Add an edge from start to content
        let edge = service.add_edge(
            path.id,
            start_node.id,
            content_node.id,
            EdgeConditionType::Completion,
            None,
            Some("Start -> Intro".to_string())
        ).await.unwrap();

        // Get the updated path
        let updated_path = service.get_path(path.id).await.unwrap();

        // Verify the nodes and edges were added
        assert_eq!(updated_path.nodes.len(), 2);
        assert_eq!(updated_path.edges.len(), 1);
        
        // Verify the node types
        let start_node_from_path = updated_path.nodes.iter().find(|n| n.node_type == LearningPathNodeType::Start).unwrap();
        let content_node_from_path = updated_path.nodes.iter().find(|n| n.node_type == LearningPathNodeType::Content).unwrap();
        
        assert_eq!(start_node_from_path.title, "Start");
        assert_eq!(content_node_from_path.title, "Introduction to Programming");
        
        // Verify the edge
        let edge_from_path = &updated_path.edges[0];
        assert_eq!(edge_from_path.source_node_id, start_node.id);
        assert_eq!(edge_from_path.target_node_id, content_node.id);
        assert_eq!(edge_from_path.condition_type, EdgeConditionType::Completion);
    }
}
