use std::collections::HashMap;

// Copy of the VersionVector implementation for testing
#[derive(Debug, Clone, PartialEq)]
struct VersionVector {
    counters: HashMap<String, i64>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum CausalRelation {
    HappensBefore,
    HappensAfter,
    Concurrent,
    Identical,
}

impl VersionVector {
    fn new() -> Self {
        Self {
            counters: HashMap::new(),
        }
    }

    fn from_hashmap(counters: HashMap<String, i64>) -> Self {
        Self { counters }
    }

    fn get(&self, device_id: &str) -> i64 {
        *self.counters.get(device_id).unwrap_or(&0)
    }

    fn increment(&mut self, device_id: &str) -> i64 {
        let counter = self.counters.entry(device_id.to_string()).or_insert(0);
        *counter += 1;
        *counter
    }

    fn merge(&mut self, other: &VersionVector) {
        for (device_id, &counter) in &other.counters {
            let entry = self.counters.entry(device_id.clone()).or_insert(0);
            *entry = std::cmp::max(*entry, counter);
        }
    }

    fn merged_with(&self, other: &VersionVector) -> VersionVector {
        let mut result = self.clone();
        result.merge(other);
        result
    }

    fn to_hashmap(&self) -> HashMap<String, i64> {
        self.counters.clone()
    }

    fn causal_relation(&self, other: &VersionVector) -> CausalRelation {
        if self == other {
            return CausalRelation::Identical;
        }

        let mut self_greater = false;
        let mut other_greater = false;

        // Check all keys in self
        for (device_id, &self_counter) in &self.counters {
            let other_counter = other.get(device_id);
            
            if self_counter > other_counter {
                self_greater = true;
            } else if self_counter < other_counter {
                other_greater = true;
            }
            
            // If we've found both directions are greater in some dimension,
            // they're concurrent
            if self_greater && other_greater {
                return CausalRelation::Concurrent;
            }
        }

        // Check keys in other that aren't in self
        for (device_id, &other_counter) in &other.counters {
            if !self.counters.contains_key(device_id) && other_counter > 0 {
                other_greater = true;
            }
            
            if self_greater && other_greater {
                return CausalRelation::Concurrent;
            }
        }

        // Determine the relationship based on which one is greater
        if self_greater {
            CausalRelation::HappensAfter
        } else if other_greater {
            CausalRelation::HappensBefore
        } else {
            // This should not happen if the vectors are different
            CausalRelation::Identical
        }
    }

    fn dominates(&self, other: &VersionVector) -> bool {
        let relation = self.causal_relation(other);
        relation == CausalRelation::HappensAfter || relation == CausalRelation::Identical
    }

    fn is_dominated_by(&self, other: &VersionVector) -> bool {
        let relation = self.causal_relation(other);
        relation == CausalRelation::HappensBefore || relation == CausalRelation::Identical
    }

    fn is_concurrent_with(&self, other: &VersionVector) -> bool {
        self.causal_relation(other) == CausalRelation::Concurrent
    }
}

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
    fn test_dominates() {
        let mut vv1 = VersionVector::new();
        vv1.increment("device1");
        
        let mut vv2 = vv1.clone();
        vv2.increment("device2");
        
        // vv2 dominates vv1
        assert!(vv2.dominates(&vv1));
        assert!(!vv1.dominates(&vv2));
        
        // vv1 is dominated by vv2
        assert!(vv1.is_dominated_by(&vv2));
        assert!(!vv2.is_dominated_by(&vv1));
        
        // Concurrent vectors don't dominate each other
        let mut vv3 = vv1.clone();
        vv3.increment("device3");
        
        assert!(!vv2.dominates(&vv3));
        assert!(!vv3.dominates(&vv2));
        assert!(vv2.is_concurrent_with(&vv3));
        assert!(vv3.is_concurrent_with(&vv2));
    }
}
