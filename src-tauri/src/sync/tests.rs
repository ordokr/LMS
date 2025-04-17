use super::version_vector::{VersionVector, CausalRelation};
use super::operations::{SyncOperation, OperationType};
use super::conflicts::{ConflictResolver, ConflictResolution};
use std::collections::HashMap;
use serde_json::json;
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

    #[test]
    fn test_conflict_detection() {
        // Create two operations on the same entity
        let op1 = create_test_operation(
            "device1",
            OperationType::Update,
            "course",
            Some("course123"),
            json!({"name": "Math 101", "description": "Introduction to Math"}),
            HashMap::from([("device1".to_string(), 1)]),
        );

        let op2 = create_test_operation(
            "device2",
            OperationType::Update,
            "course",
            Some("course123"),
            json!({"name": "Mathematics 101", "credits": 3}),
            HashMap::from([("device2".to_string(), 1)]),
        );

        // These should be detected as concurrent operations (potential conflict)
        let conflict = ConflictResolver::detect_conflict(&op1, &op2);
        assert!(conflict.is_some());
    }

    #[test]
    fn test_no_conflict_with_causal_relation() {
        // Create two operations with a causal relationship
        let mut vv1 = HashMap::new();
        vv1.insert("device1".to_string(), 1);

        let mut vv2 = HashMap::new();
        vv2.insert("device1".to_string(), 1);
        vv2.insert("device2".to_string(), 1);

        let op1 = create_test_operation(
            "device1",
            OperationType::Update,
            "course",
            Some("course123"),
            json!({"name": "Math 101"}),
            vv1,
        );

        let op2 = create_test_operation(
            "device2",
            OperationType::Update,
            "course",
            Some("course123"),
            json!({"description": "Advanced course"}),
            vv2,
        );

        // op1 happens before op2, so no conflict
        let conflict = ConflictResolver::detect_conflict(&op1, &op2);
        assert!(conflict.is_none());
    }

    #[test]
    fn test_update_update_conflict_resolution() {
        // Create two concurrent update operations
        let op1 = create_test_operation(
            "device1",
            OperationType::Update,
            "course",
            Some("course123"),
            json!({"name": "Math 101", "description": "Introduction to Math"}),
            HashMap::from([("device1".to_string(), 1)]),
        );

        let op2 = create_test_operation(
            "device2",
            OperationType::Update,
            "course",
            Some("course123"),
            json!({"name": "Mathematics 101", "credits": 3}),
            HashMap::from([("device2".to_string(), 1)]),
        );

        // Resolve the conflict
        let resolution = ConflictResolver::resolve_conflict(&op1, &op2);
        assert_eq!(resolution, ConflictResolution::Merge);

        // Test the merged operation
        let merged_op = ConflictResolver::merge_updates(&op1, &op2);
        
        // Check that the merged payload contains all fields
        if let serde_json::Value::Object(map) = &merged_op.payload {
            assert_eq!(map.get("name").unwrap().as_str().unwrap(), "Mathematics 101");
            assert_eq!(map.get("description").unwrap().as_str().unwrap(), "Introduction to Math");
            assert_eq!(map.get("credits").unwrap().as_i64().unwrap(), 3);
        } else {
            panic!("Expected merged payload to be an object");
        }

        // Check that the merged vector clock contains both devices
        assert_eq!(merged_op.vector_clock.get("device1").unwrap(), &1);
        assert_eq!(merged_op.vector_clock.get("device2").unwrap(), &1);
    }

    #[test]
    fn test_create_delete_conflict_resolution() {
        // Create a create operation
        let op1 = create_test_operation(
            "device1",
            OperationType::Create,
            "course",
            Some("course123"),
            json!({"name": "Math 101"}),
            HashMap::from([("device1".to_string(), 1)]),
        );

        // Create a delete operation that happened later
        let mut op2 = create_test_operation(
            "device2",
            OperationType::Delete,
            "course",
            Some("course123"),
            json!(null),
            HashMap::from([("device2".to_string(), 1)]),
        );
        op2.timestamp = op1.timestamp + 100; // Make it later

        // Resolve the conflict - delete should win because it's later
        let resolution = ConflictResolver::resolve_conflict(&op1, &op2);
        assert_eq!(resolution, ConflictResolution::KeepSecond);
    }
}

// Helper function to create a test operation
fn create_test_operation(
    device_id: &str,
    operation_type: OperationType,
    entity_type: &str,
    entity_id: Option<&str>,
    payload: serde_json::Value,
    vector_clock: HashMap<String, i64>,
) -> SyncOperation {
    SyncOperation {
        id: Uuid::new_v4().to_string(),
        device_id: device_id.to_string(),
        user_id: 1,
        operation_type,
        entity_type: entity_type.to_string(),
        entity_id: entity_id.map(|s| s.to_string()),
        payload,
        timestamp: chrono::Utc::now().timestamp(),
        vector_clock,
        synced: false,
        synced_at: None,
    }
}
