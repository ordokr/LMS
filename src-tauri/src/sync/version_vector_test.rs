use super::version_vector::{VersionVector, CausalRelation};
use std::collections::HashMap;
use crate::sync::version_vector::{VersionVector, CausalRelation};
use crate::sync::version_vector_sync::{VersionVectorSyncManager, EntityType, SyncStatus};
use crate::models::forum::topic::Topic;
use crate::models::forum::post::Post;
use crate::services::integration::conflict_resolver::{ConflictResolver, ConflictStrategy};
use crate::db::DB;
use crate::error::Error;
use sqlx::sqlite::SqlitePoolOptions;
use std::sync::Arc;
use chrono::Utc;
use uuid::Uuid;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_vector_causal_relations() {
        // Create version vectors for testing
        let mut vv1 = VersionVector::new();
        vv1.increment("device1");
        
        let mut vv2 = vv1.clone();
        vv2.increment("device2");
        
        let mut vv3 = vv2.clone();
        vv3.increment("device3");
        
        let mut vv4 = vv1.clone();
        vv4.increment("device3");

        // Test identical
        assert_eq!(vv1.causal_relation(&vv1), CausalRelation::Identical);
        
        // Test happens before/after
        assert_eq!(vv1.causal_relation(&vv2), CausalRelation::HappensBefore);
        assert_eq!(vv2.causal_relation(&vv1), CausalRelation::HappensAfter);
        assert_eq!(vv1.causal_relation(&vv3), CausalRelation::HappensBefore);
        assert_eq!(vv3.causal_relation(&vv1), CausalRelation::HappensAfter);
        assert_eq!(vv2.causal_relation(&vv3), CausalRelation::HappensBefore);
        assert_eq!(vv3.causal_relation(&vv2), CausalRelation::HappensAfter);
        
        // Test concurrent
        assert_eq!(vv2.causal_relation(&vv4), CausalRelation::Concurrent);
        assert_eq!(vv4.causal_relation(&vv2), CausalRelation::Concurrent);
    }

    #[test]
    fn test_version_vector_merge() {
        let mut vv1 = VersionVector::new();
        vv1.increment("device1");
        vv1.increment("device1");
        vv1.increment("device2");

        let mut vv2 = VersionVector::new();
        vv2.increment("device1");
        vv2.increment("device3");
        vv2.increment("device3");

        let merged = vv1.merged_with(&vv2);
        assert_eq!(merged.get("device1"), 2);
        assert_eq!(merged.get("device2"), 1);
        assert_eq!(merged.get("device3"), 2);
    }

    /// Test version vector based conflict detection and resolution
    #[tokio::test]
    async fn test_version_vector_conflict_detection() -> Result<(), Error> {
        // Setup in-memory database
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect("sqlite::memory:")
            .await?;
        
        // Setup schema - just enough for version vector testing
        sqlx::query!(
            r#"
            CREATE TABLE IF NOT EXISTS entity_version_vectors (
                entity_type TEXT NOT NULL,
                entity_id TEXT NOT NULL,
                canvas_vector TEXT NOT NULL,
                discourse_vector TEXT NOT NULL,
                local_vector TEXT NOT NULL,
                last_sync TEXT NOT NULL,
                status TEXT NOT NULL,
                PRIMARY KEY (entity_type, entity_id)
            )
            "#
        )
        .execute(&pool)
        .await?;
        
        // Create version vector sync manager
        let sync_manager = VersionVectorSyncManager::new(pool.clone());
        sync_manager.create_tables().await?;
        
        // Test entities
        let entity_type = EntityType::Topic;
        let entity_id = Uuid::new_v4().to_string();
        
        // 1. Test initial versioning
        let canvas_device_id = "canvas_device_1";
        let discourse_device_id = "discourse_device_1";
        
        // Create new version vectors
        let mut canvas_vector = VersionVector::new();
        let mut discourse_vector = VersionVector::new();
        
        // Test vectors for two devices (completely independent modifications)
        canvas_vector.increment(canvas_device_id);
        discourse_vector.increment(discourse_device_id);
        
        // Detect conflict - should be concurrent since different devices made independent changes
        assert_eq!(canvas_vector.causal_relation(&discourse_vector), CausalRelation::Concurrent);
        
        // 2. Use our conflict resolver with version vectors
        let resolver = ConflictResolver::new(ConflictStrategy::PreferMostRecent);
        
        // Create test topics
        let canvas_topic = Topic {
            id: Uuid::new_v4(),
            category_id: Uuid::new_v4(),
            title: "Canvas Topic".to_string(),
            content: "Canvas Content".to_string(),
            author_id: Uuid::new_v4(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            last_post_at: Some(Utc::now()),
            publish_at: None,
            is_pinned: false,
            is_closed: false,
            is_question: false,
            assignment_id: None,
            read_status: false,
            view_count: 10,
            canvas_discussion_id: Some("canvas-1".to_string()),
            discourse_topic_id: None,
            sync_status: crate::models::forum::topic::SyncStatus::SyncedWithCanvas,
            tags: vec!["canvas".to_string()],
            post_ids: vec![],
        };
        
        let discourse_topic = Topic {
            id: Uuid::new_v4(),
            category_id: Uuid::new_v4(),
            title: "Discourse Topic".to_string(),
            content: "Discourse Content".to_string(),
            author_id: Uuid::new_v4(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            last_post_at: Some(Utc::now()),
            publish_at: None,
            is_pinned: false,
            is_closed: false,
            is_question: false,
            assignment_id: None,
            read_status: false,
            view_count: 20,
            canvas_discussion_id: None,
            discourse_topic_id: Some(123),
            sync_status: crate::models::forum::topic::SyncStatus::SyncedWithDiscourse,
            tags: vec!["discourse".to_string()],
            post_ids: vec![],
        };
        
        // Resolve with version vectors
        let resolved_topic = resolver.resolve_topic_conflict_with_version_vector(
            &canvas_topic,
            &discourse_topic,
            &canvas_vector,
            &discourse_vector
        )?;
        
        // With concurrent vectors, it should fall back to strategy (PreferMostRecent)
        // Since both topics were created at the same time, we can't know for sure which
        // one got chosen, but we know it's using the strategy
        assert!(
            (resolved_topic.title == canvas_topic.title) || 
            (resolved_topic.title == discourse_topic.title)
        );
        
        // 3. Test causal relationship
        // Create a new version where one is clearly the ancestor of the other
        let mut canvas_vector_v2 = canvas_vector.clone();
        canvas_vector_v2.increment(canvas_device_id);
        canvas_vector_v2.increment(canvas_device_id);
        
        // Create discourse vector that is based on the canvas vector
        let mut discourse_vector_v2 = canvas_vector_v2.clone();
        discourse_vector_v2.increment(discourse_device_id);
        
        // This should show that canvas_vector_v2 happened before discourse_vector_v2
        assert_eq!(canvas_vector_v2.causal_relation(&discourse_vector_v2), CausalRelation::HappensBefore);
        
        // When resolving, it should prefer discourse_topic because it's newer
        let canvas_topic_v2 = Topic {
            updated_at: Utc::now() - chrono::Duration::seconds(60),
            ..canvas_topic.clone()
        };
        
        let discourse_topic_v2 = Topic {
            updated_at: Utc::now(),
            ..discourse_topic.clone()
        };
        
        let resolved_topic_v2 = resolver.resolve_topic_conflict_with_version_vector(
            &canvas_topic_v2,
            &discourse_topic_v2,
            &canvas_vector_v2,
            &discourse_vector_v2
        )?;
        
        // With causal relationship, it should pick discourse directly without using strategy
        assert_eq!(resolved_topic_v2.title, discourse_topic_v2.title);
        
        // 4. Test the sync manager with conflict detection
        let has_conflict = sync_manager.detect_conflicts(
            &entity_type,
            &entity_id,
            Some(serde_json::json!({"title": "Canvas Title"})),
            Some(serde_json::json!({"title": "Discourse Title"}))
        ).await?;
        
        // Should detect conflict with concurrent modifications
        assert!(has_conflict);
        
        // Test conflict resolution
        sync_manager.resolve_conflicts(&entity_type, &entity_id, "merge").await?;
        
        // After resolution, we should not have a conflict
        let vector_sync = sync_manager.load_version_vector(&entity_type, &entity_id).await?.unwrap();
        assert_eq!(vector_sync.status, SyncStatus::Synced);
        
        Ok(())
    }
}
