use serde::{Serialize, Deserialize};
use diff_struct::{Diff, DiffPatch};
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::blockchain::core::HybridChain;
use crate::blockchain::error::BlockchainError;

#[derive(Clone, Serialize, Deserialize, DiffPatch)]
pub struct ForumPost {
    pub id: String,
    pub content: String,
    pub author: String,
    pub timestamp: i64,
    pub version: u64,
}

#[derive(Clone, Serialize, Deserialize, DiffPatch)]
pub struct CourseAchievement {
    pub student_id: String,
    pub course_id: String,
    pub achievement_type: String,
    pub metadata: String,
    pub timestamp: i64,
}

pub struct DifferentialAnchoring {
    last_anchor: chrono::DateTime<chrono::Utc>,
    pending_diffs: Vec<AnyDiff>,
    chain: Arc<Mutex<HybridChain>>,
}

pub enum AnyDiff {
    ForumPost(Diff<ForumPost>),
    Achievement(Diff<CourseAchievement>),
}

impl DifferentialAnchoring {
    pub fn new(chain: Arc<Mutex<HybridChain>>) -> Self {
        Self {
            last_anchor: chrono::Utc::now(),
            pending_diffs: Vec::new(),
            chain,
        }
    }
    
    pub fn add_forum_diff(&mut self, original: &ForumPost, updated: &ForumPost) {
        let diff = Diff::new(original, updated);
        self.pending_diffs.push(AnyDiff::ForumPost(diff));
    }
    
    pub fn add_achievement_diff(&mut self, original: &CourseAchievement, updated: &CourseAchievement) {
        let diff = Diff::new(original, updated);
        self.pending_diffs.push(AnyDiff::Achievement(diff));
    }
    
    pub async fn anchor_changes(&mut self) -> Result<(), BlockchainError> {
        if self.pending_diffs.is_empty() {
            return Ok(());
        }
        
        // Serialize all diffs to a compact format
        let mut hasher = blake3::Hasher::new();
        
        for diff in &self.pending_diffs {
            match diff {
                AnyDiff::ForumPost(d) => {
                    // Hash the diff
                    let diff_bytes = bincode::serialize(d).unwrap();
                    hasher.update(&diff_bytes);
                }
                AnyDiff::Achievement(d) => {
                    // Hash the diff
                    let diff_bytes = bincode::serialize(d).unwrap();
                    hasher.update(&diff_bytes);
                }
            }
        }
        
        let batch_hash = hasher.finalize();
        
        // Create a blockchain block with just the hash of all diffs
        let mut chain = self.chain.lock().await;
        chain.create_block_with_hash(batch_hash.as_bytes()).await?;
        
        // Clear pending diffs
        self.pending_diffs.clear();
        self.last_anchor = chrono::Utc::now();
        
        Ok(())
    }
}