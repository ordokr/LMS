use std::collections::{HashMap, BTreeMap};
use std::cmp::Ordering;
use serde::{Serialize, Deserialize};
use std::io::{self, Read, Write};
use std::time::{Duration, Instant};

/// Represents a version vector (vector clock) for tracking causality in distributed systems
/// with optimizations for large systems
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VersionVector {
    /// Maps device IDs to their logical clocks
    counters: HashMap<String, i64>,
    /// Last time this vector was accessed (for pruning)
    #[serde(skip)]
    last_accessed: Option<Instant>,
    /// Cached hash for quick equality checks
    #[serde(skip)]
    cached_hash: Option<u64>,
}

/// Represents the causal relationship between two version vectors
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CausalRelation {
    /// First happens before second (first is ancestor of second)
    HappensBefore,
    /// Second happens before first (second is ancestor of first)
    HappensAfter,
    /// Neither happens before the other (concurrent modifications)
    Concurrent,
    /// Identical version vectors
    Identical,
}

/// Compressed representation of a version vector for efficient storage and transmission
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressedVersionVector {
    /// Device IDs mapped to indices for compact representation
    device_map: HashMap<String, usize>,
    /// Reverse mapping from indices to device IDs
    reverse_map: Vec<String>,
    /// Compressed counters using run-length encoding
    compressed_counters: Vec<(usize, i64)>,
}

impl VersionVector {
    /// Create a new empty version vector
    pub fn new() -> Self {
        Self {
            counters: HashMap::new(),
            last_accessed: Some(Instant::now()),
            cached_hash: None,
        }
    }

    /// Create a version vector from an existing HashMap
    pub fn from_hashmap(counters: HashMap<String, i64>) -> Self {
        Self {
            counters,
            last_accessed: Some(Instant::now()),
            cached_hash: None,
        }
    }

    /// Update the last accessed time
    fn touch(&mut self) {
        self.last_accessed = Some(Instant::now());
        self.cached_hash = None; // Invalidate cache
    }

    /// Get the counter value for a specific device
    pub fn get(&self, device_id: &str) -> i64 {
        *self.counters.get(device_id).unwrap_or(&0)
    }

    /// Get the counter value for a specific device, updating access time
    pub fn get_with_touch(&mut self, device_id: &str) -> i64 {
        self.touch();
        self.get(device_id)
    }

    /// Increment the counter for a specific device
    pub fn increment(&mut self, device_id: &str) -> i64 {
        self.touch();
        let counter = self.counters.entry(device_id.to_string()).or_insert(0);
        *counter += 1;
        self.cached_hash = None; // Invalidate cache
        *counter
    }

    /// Merge this version vector with another one, taking the maximum values
    pub fn merge(&mut self, other: &VersionVector) {
        self.touch();
        let mut changed = false;

        for (device_id, &counter) in &other.counters {
            let entry = self.counters.entry(device_id.clone()).or_insert(0);
            if counter > *entry {
                *entry = counter;
                changed = true;
            }
        }

        if changed {
            self.cached_hash = None; // Invalidate cache
        }
    }

    /// Create a new version vector that is the result of merging this one with another
    pub fn merged_with(&self, other: &VersionVector) -> VersionVector {
        let mut result = self.clone();
        result.merge(other);
        result
    }

    /// Convert to a HashMap
    pub fn to_hashmap(&self) -> HashMap<String, i64> {
        self.counters.clone()
    }

    /// Prune entries that haven't changed in a long time
    /// Returns the number of entries pruned
    pub fn prune_inactive_entries(&mut self, min_value: i64) -> usize {
        let original_size = self.counters.len();

        // Remove entries with low values that likely won't be needed
        self.counters.retain(|_, &mut value| value > min_value);

        let new_size = self.counters.len();
        let pruned = original_size - new_size;

        if pruned > 0 {
            self.cached_hash = None; // Invalidate cache
        }

        pruned
    }

    /// Compress the version vector for efficient storage or transmission
    pub fn compress(&self) -> CompressedVersionVector {
        // Create a mapping of device IDs to indices
        let mut device_map = HashMap::new();
        let mut reverse_map = Vec::new();

        // Sort the entries by device ID for consistent compression
        let mut sorted_entries: Vec<_> = self.counters.iter().collect();
        sorted_entries.sort_by(|(a, _), (b, _)| a.cmp(b));

        // Build the device mapping
        for (i, (device_id, _)) in sorted_entries.iter().enumerate() {
            device_map.insert((*device_id).clone(), i);
            reverse_map.push((*device_id).clone());
        }

        // Create compressed counters using run-length encoding
        let mut compressed_counters = Vec::new();
        let mut current_index = 0;

        while current_index < sorted_entries.len() {
            let (_, &current_value) = sorted_entries[current_index];
            let mut run_length = 1;

            // Count consecutive entries with the same value
            while current_index + run_length < sorted_entries.len() {
                let (_, &next_value) = sorted_entries[current_index + run_length];
                if next_value != current_value {
                    break;
                }
                run_length += 1;
            }

            // Add the run to the compressed representation
            compressed_counters.push((run_length, current_value));
            current_index += run_length;
        }

        CompressedVersionVector {
            device_map,
            reverse_map,
            compressed_counters,
        }
    }

    /// Create a version vector from a compressed representation
    pub fn from_compressed(compressed: &CompressedVersionVector) -> Self {
        let mut counters = HashMap::new();
        let mut current_index = 0;

        for &(run_length, value) in &compressed.compressed_counters {
            for i in 0..run_length {
                let device_id = &compressed.reverse_map[current_index + i];
                counters.insert(device_id.clone(), value);
            }
            current_index += run_length;
        }

        Self {
            counters,
            last_accessed: Some(Instant::now()),
            cached_hash: None,
        }
    }

    /// Calculate a hash of this version vector for quick equality checks
    pub fn hash(&mut self) -> u64 {
        if let Some(hash) = self.cached_hash {
            return hash;
        }

        // Simple hash function for the version vector
        let mut hash: u64 = 0;

        // Sort the entries for consistent hashing
        let mut sorted_entries: Vec<_> = self.counters.iter().collect();
        sorted_entries.sort_by(|(a, _), (b, _)| a.cmp(b));

        for (device_id, &counter) in sorted_entries {
            // Combine the device ID and counter into the hash
            let device_hash = device_id.bytes().fold(0u64, |acc, b| acc.wrapping_mul(31).wrapping_add(b as u64));
            hash = hash.wrapping_mul(37).wrapping_add(device_hash);
            hash = hash.wrapping_mul(37).wrapping_add(counter as u64);
        }

        self.cached_hash = Some(hash);
        hash
    }

    /// Get the size of this version vector (number of entries)
    pub fn size(&self) -> usize {
        self.counters.len()
    }

    /// Serialize to a compact binary representation
    pub fn to_bytes(&self) -> io::Result<Vec<u8>> {
        let mut buffer = Vec::new();

        // Write the number of entries
        let count = self.counters.len() as u32;
        buffer.extend_from_slice(&count.to_le_bytes());

        // Sort the entries for consistent serialization
        let mut sorted_entries: Vec<_> = self.counters.iter().collect();
        sorted_entries.sort_by(|(a, _), (b, _)| a.cmp(b));

        for (device_id, &counter) in sorted_entries {
            // Write the device ID length and bytes
            let id_len = device_id.len() as u16;
            buffer.extend_from_slice(&id_len.to_le_bytes());
            buffer.extend_from_slice(device_id.as_bytes());

            // Write the counter value
            buffer.extend_from_slice(&counter.to_le_bytes());
        }

        Ok(buffer)
    }

    /// Deserialize from a compact binary representation
    pub fn from_bytes(bytes: &[u8]) -> io::Result<Self> {
        if bytes.len() < 4 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid version vector data"));
        }

        let mut counters = HashMap::new();
        let mut pos = 0;

        // Read the number of entries
        let count = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        pos += 4;

        for _ in 0..count {
            if pos + 2 > bytes.len() {
                return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid version vector data"));
            }

            // Read the device ID length
            let id_len = u16::from_le_bytes([bytes[pos], bytes[pos + 1]]) as usize;
            pos += 2;

            if pos + id_len > bytes.len() {
                return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid version vector data"));
            }

            // Read the device ID
            let device_id = String::from_utf8_lossy(&bytes[pos..pos + id_len]).to_string();
            pos += id_len;

            if pos + 8 > bytes.len() {
                return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid version vector data"));
            }

            // Read the counter value
            let counter = i64::from_le_bytes([
                bytes[pos], bytes[pos + 1], bytes[pos + 2], bytes[pos + 3],
                bytes[pos + 4], bytes[pos + 5], bytes[pos + 6], bytes[pos + 7],
            ]);
            pos += 8;

            counters.insert(device_id, counter);
        }

        Ok(Self {
            counters,
            last_accessed: Some(Instant::now()),
            cached_hash: None,
        })
    }

    /// Create a delta update between this version vector and another
    pub fn create_delta(&self, other: &VersionVector) -> HashMap<String, i64> {
        let mut delta = HashMap::new();

        // Find entries that are different
        for (device_id, &counter) in &other.counters {
            let self_counter = self.get(device_id);
            if counter > self_counter {
                delta.insert(device_id.clone(), counter);
            }
        }

        delta
    }

    /// Apply a delta update to this version vector
    pub fn apply_delta(&mut self, delta: &HashMap<String, i64>) {
        self.touch();
        let mut changed = false;

        for (device_id, &counter) in delta {
            let entry = self.counters.entry(device_id.clone()).or_insert(0);
            if counter > *entry {
                *entry = counter;
                changed = true;
            }
        }

        if changed {
            self.cached_hash = None; // Invalidate cache
        }
    }

    /// Determine the causal relationship between this version vector and another
    pub fn causal_relation(&self, other: &VersionVector) -> CausalRelation {
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

    /// Check if this version vector dominates another (happens after or identical)
    pub fn dominates(&self, other: &VersionVector) -> bool {
        let relation = self.causal_relation(other);
        relation == CausalRelation::HappensAfter || relation == CausalRelation::Identical
    }

    /// Check if this version vector is dominated by another (happens before or identical)
    pub fn is_dominated_by(&self, other: &VersionVector) -> bool {
        let relation = self.causal_relation(other);
        relation == CausalRelation::HappensBefore || relation == CausalRelation::Identical
    }

    /// Check if this version vector is concurrent with another
    pub fn is_concurrent_with(&self, other: &VersionVector) -> bool {
        self.causal_relation(other) == CausalRelation::Concurrent
    }
}

impl Default for VersionVector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_version_vector() {
        let vv = VersionVector::new();
        assert_eq!(vv.counters.len(), 0);
    }

    #[test]
    fn test_increment() {
        let mut vv = VersionVector::new();
        assert_eq!(vv.increment("device1"), 1);
        assert_eq!(vv.increment("device1"), 2);
        assert_eq!(vv.increment("device2"), 1);
        assert_eq!(vv.get("device1"), 2);
        assert_eq!(vv.get("device2"), 1);
        assert_eq!(vv.get("device3"), 0);
    }

    #[test]
    fn test_merge() {
        let mut vv1 = VersionVector::new();
        vv1.increment("device1");
        vv1.increment("device1");
        vv1.increment("device2");

        let mut vv2 = VersionVector::new();
        vv2.increment("device1");
        vv2.increment("device3");
        vv2.increment("device3");

        vv1.merge(&vv2);
        assert_eq!(vv1.get("device1"), 2);
        assert_eq!(vv1.get("device2"), 1);
        assert_eq!(vv1.get("device3"), 2);
    }

    #[test]
    fn test_causal_relations() {
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
}
