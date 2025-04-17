use super::version_vector::{VersionVector, CausalRelation};
use std::collections::HashMap;

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
}
