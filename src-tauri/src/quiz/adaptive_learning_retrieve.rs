use super::adaptive_learning::{AdaptiveLearningService, AdaptiveLearningPath, LearningPathNode, LearningPathEdge, LearningPathNodeType, EdgeConditionType, UserLearningPathProgress, LearningPathRecommendation};
use uuid::Uuid;
use std::error::Error;
use chrono::{DateTime, Utc};

impl AdaptiveLearningService {
    /// Get a learning path by ID
    pub async fn get_path(
        &self,
        path_id: Uuid,
    ) -> Result<AdaptiveLearningPath, Box<dyn Error + Send + Sync>> {
        let row = sqlx::query!(
            r#"
            SELECT id, title, description, author_id, subject, tags, default_study_mode, 
                   default_visibility, is_public, usage_count, rating, created_at, updated_at,
                   nodes, edges
            FROM quiz_adaptive_learning_paths
            WHERE id = ?
            "#,
            path_id.to_string()
        )
        .fetch_optional(&self.db_pool)
        .await?;
        
        if let Some(row) = row {
            // Parse the tags
            let tags: Vec<String> = if let Some(tags_str) = &row.tags {
                serde_json::from_str(tags_str)?
            } else {
                Vec::new()
            };
            
            // Parse the study mode
            let default_study_mode = row.default_study_mode.parse()?;
            
            // Parse the visibility
            let default_visibility = row.default_visibility.parse()?;
            
            // Parse the nodes
            let nodes: Vec<LearningPathNode> = if let Some(nodes_str) = &row.nodes {
                serde_json::from_str(nodes_str)?
            } else {
                Vec::new()
            };
            
            // Parse the edges
            let edges: Vec<LearningPathEdge> = if let Some(edges_str) = &row.edges {
                serde_json::from_str(edges_str)?
            } else {
                Vec::new()
            };
            
            // Create the path
            let path = AdaptiveLearningPath {
                id: Uuid::parse_str(&row.id)?,
                title: row.title,
                description: row.description,
                author_id: row.author_id.map(|id| Uuid::parse_str(&id)).transpose()?,
                subject: row.subject,
                tags,
                nodes,
                edges,
                default_study_mode,
                default_visibility,
                is_public: row.is_public != 0,
                usage_count: row.usage_count,
                rating: row.rating,
                created_at: row.created_at.parse::<DateTime<Utc>>()?,
                updated_at: row.updated_at.parse::<DateTime<Utc>>()?,
            };
            
            Ok(path)
        } else {
            Err("Learning path not found".into())
        }
    }
    
    /// Get a node by ID
    pub async fn get_node(
        &self,
        node_id: Uuid,
    ) -> Result<LearningPathNode, Box<dyn Error + Send + Sync>> {
        // Get all paths
        let paths = self.get_all_paths(None, None).await?;
        
        // Find the node in any path
        for path in paths {
            for node in &path.nodes {
                if node.id == node_id {
                    return Ok(node.clone());
                }
            }
        }
        
        Err("Node not found".into())
    }
    
    /// Get an edge by ID
    pub async fn get_edge(
        &self,
        edge_id: Uuid,
    ) -> Result<LearningPathEdge, Box<dyn Error + Send + Sync>> {
        // Get all paths
        let paths = self.get_all_paths(None, None).await?;
        
        // Find the edge in any path
        for path in paths {
            for edge in &path.edges {
                if edge.id == edge_id {
                    return Ok(edge.clone());
                }
            }
        }
        
        Err("Edge not found".into())
    }
    
    /// Get all learning paths
    pub async fn get_all_paths(
        &self,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<AdaptiveLearningPath>, Box<dyn Error + Send + Sync>> {
        let limit = limit.unwrap_or(20);
        let offset = offset.unwrap_or(0);
        
        let rows = sqlx::query!(
            r#"
            SELECT id, title, description, author_id, subject, tags, default_study_mode, 
                   default_visibility, is_public, usage_count, rating, created_at, updated_at,
                   nodes, edges
            FROM quiz_adaptive_learning_paths
            ORDER BY created_at DESC
            LIMIT ? OFFSET ?
            "#,
            limit,
            offset
        )
        .fetch_all(&self.db_pool)
        .await?;
        
        let mut paths = Vec::new();
        
        for row in rows {
            // Parse the tags
            let tags: Vec<String> = if let Some(tags_str) = &row.tags {
                serde_json::from_str(tags_str)?
            } else {
                Vec::new()
            };
            
            // Parse the study mode
            let default_study_mode = row.default_study_mode.parse()?;
            
            // Parse the visibility
            let default_visibility = row.default_visibility.parse()?;
            
            // Parse the nodes
            let nodes: Vec<LearningPathNode> = if let Some(nodes_str) = &row.nodes {
                serde_json::from_str(nodes_str)?
            } else {
                Vec::new()
            };
            
            // Parse the edges
            let edges: Vec<LearningPathEdge> = if let Some(edges_str) = &row.edges {
                serde_json::from_str(edges_str)?
            } else {
                Vec::new()
            };
            
            // Create the path
            let path = AdaptiveLearningPath {
                id: Uuid::parse_str(&row.id)?,
                title: row.title,
                description: row.description,
                author_id: row.author_id.map(|id| Uuid::parse_str(&id)).transpose()?,
                subject: row.subject,
                tags,
                nodes,
                edges,
                default_study_mode,
                default_visibility,
                is_public: row.is_public != 0,
                usage_count: row.usage_count,
                rating: row.rating,
                created_at: row.created_at.parse::<DateTime<Utc>>()?,
                updated_at: row.updated_at.parse::<DateTime<Utc>>()?,
            };
            
            paths.push(path);
        }
        
        Ok(paths)
    }
    
    /// Get all public learning paths
    pub async fn get_public_paths(
        &self,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<AdaptiveLearningPath>, Box<dyn Error + Send + Sync>> {
        let limit = limit.unwrap_or(20);
        let offset = offset.unwrap_or(0);
        
        let rows = sqlx::query!(
            r#"
            SELECT id, title, description, author_id, subject, tags, default_study_mode, 
                   default_visibility, is_public, usage_count, rating, created_at, updated_at,
                   nodes, edges
            FROM quiz_adaptive_learning_paths
            WHERE is_public = 1
            ORDER BY usage_count DESC, rating DESC
            LIMIT ? OFFSET ?
            "#,
            limit,
            offset
        )
        .fetch_all(&self.db_pool)
        .await?;
        
        let mut paths = Vec::new();
        
        for row in rows {
            // Parse the tags
            let tags: Vec<String> = if let Some(tags_str) = &row.tags {
                serde_json::from_str(tags_str)?
            } else {
                Vec::new()
            };
            
            // Parse the study mode
            let default_study_mode = row.default_study_mode.parse()?;
            
            // Parse the visibility
            let default_visibility = row.default_visibility.parse()?;
            
            // Parse the nodes
            let nodes: Vec<LearningPathNode> = if let Some(nodes_str) = &row.nodes {
                serde_json::from_str(nodes_str)?
            } else {
                Vec::new()
            };
            
            // Parse the edges
            let edges: Vec<LearningPathEdge> = if let Some(edges_str) = &row.edges {
                serde_json::from_str(edges_str)?
            } else {
                Vec::new()
            };
            
            // Create the path
            let path = AdaptiveLearningPath {
                id: Uuid::parse_str(&row.id)?,
                title: row.title,
                description: row.description,
                author_id: row.author_id.map(|id| Uuid::parse_str(&id)).transpose()?,
                subject: row.subject,
                tags,
                nodes,
                edges,
                default_study_mode,
                default_visibility,
                is_public: row.is_public != 0,
                usage_count: row.usage_count,
                rating: row.rating,
                created_at: row.created_at.parse::<DateTime<Utc>>()?,
                updated_at: row.updated_at.parse::<DateTime<Utc>>()?,
            };
            
            paths.push(path);
        }
        
        Ok(paths)
    }
    
    /// Search learning paths by title, description, subject, or tags
    pub async fn search_paths(
        &self,
        query: &str,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<AdaptiveLearningPath>, Box<dyn Error + Send + Sync>> {
        let limit = limit.unwrap_or(20);
        let offset = offset.unwrap_or(0);
        
        // Prepare the search query
        let search_query = format!("%{}%", query);
        
        let rows = sqlx::query!(
            r#"
            SELECT id, title, description, author_id, subject, tags, default_study_mode, 
                   default_visibility, is_public, usage_count, rating, created_at, updated_at,
                   nodes, edges
            FROM quiz_adaptive_learning_paths
            WHERE is_public = 1
            AND (title LIKE ? OR description LIKE ? OR subject LIKE ? OR tags LIKE ?)
            ORDER BY usage_count DESC, rating DESC
            LIMIT ? OFFSET ?
            "#,
            search_query,
            search_query,
            search_query,
            search_query,
            limit,
            offset
        )
        .fetch_all(&self.db_pool)
        .await?;
        
        let mut paths = Vec::new();
        
        for row in rows {
            // Parse the tags
            let tags: Vec<String> = if let Some(tags_str) = &row.tags {
                serde_json::from_str(tags_str)?
            } else {
                Vec::new()
            };
            
            // Parse the study mode
            let default_study_mode = row.default_study_mode.parse()?;
            
            // Parse the visibility
            let default_visibility = row.default_visibility.parse()?;
            
            // Parse the nodes
            let nodes: Vec<LearningPathNode> = if let Some(nodes_str) = &row.nodes {
                serde_json::from_str(nodes_str)?
            } else {
                Vec::new()
            };
            
            // Parse the edges
            let edges: Vec<LearningPathEdge> = if let Some(edges_str) = &row.edges {
                serde_json::from_str(edges_str)?
            } else {
                Vec::new()
            };
            
            // Create the path
            let path = AdaptiveLearningPath {
                id: Uuid::parse_str(&row.id)?,
                title: row.title,
                description: row.description,
                author_id: row.author_id.map(|id| Uuid::parse_str(&id)).transpose()?,
                subject: row.subject,
                tags,
                nodes,
                edges,
                default_study_mode,
                default_visibility,
                is_public: row.is_public != 0,
                usage_count: row.usage_count,
                rating: row.rating,
                created_at: row.created_at.parse::<DateTime<Utc>>()?,
                updated_at: row.updated_at.parse::<DateTime<Utc>>()?,
            };
            
            paths.push(path);
        }
        
        Ok(paths)
    }
    
    /// Get learning paths by author
    pub async fn get_paths_by_author(
        &self,
        author_id: Uuid,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<AdaptiveLearningPath>, Box<dyn Error + Send + Sync>> {
        let limit = limit.unwrap_or(20);
        let offset = offset.unwrap_or(0);
        
        let rows = sqlx::query!(
            r#"
            SELECT id, title, description, author_id, subject, tags, default_study_mode, 
                   default_visibility, is_public, usage_count, rating, created_at, updated_at,
                   nodes, edges
            FROM quiz_adaptive_learning_paths
            WHERE author_id = ?
            ORDER BY created_at DESC
            LIMIT ? OFFSET ?
            "#,
            author_id.to_string(),
            limit,
            offset
        )
        .fetch_all(&self.db_pool)
        .await?;
        
        let mut paths = Vec::new();
        
        for row in rows {
            // Parse the tags
            let tags: Vec<String> = if let Some(tags_str) = &row.tags {
                serde_json::from_str(tags_str)?
            } else {
                Vec::new()
            };
            
            // Parse the study mode
            let default_study_mode = row.default_study_mode.parse()?;
            
            // Parse the visibility
            let default_visibility = row.default_visibility.parse()?;
            
            // Parse the nodes
            let nodes: Vec<LearningPathNode> = if let Some(nodes_str) = &row.nodes {
                serde_json::from_str(nodes_str)?
            } else {
                Vec::new()
            };
            
            // Parse the edges
            let edges: Vec<LearningPathEdge> = if let Some(edges_str) = &row.edges {
                serde_json::from_str(edges_str)?
            } else {
                Vec::new()
            };
            
            // Create the path
            let path = AdaptiveLearningPath {
                id: Uuid::parse_str(&row.id)?,
                title: row.title,
                description: row.description,
                author_id: row.author_id.map(|id| Uuid::parse_str(&id)).transpose()?,
                subject: row.subject,
                tags,
                nodes,
                edges,
                default_study_mode,
                default_visibility,
                is_public: row.is_public != 0,
                usage_count: row.usage_count,
                rating: row.rating,
                created_at: row.created_at.parse::<DateTime<Utc>>()?,
                updated_at: row.updated_at.parse::<DateTime<Utc>>()?,
            };
            
            paths.push(path);
        }
        
        Ok(paths)
    }
}
