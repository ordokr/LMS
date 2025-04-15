use crate::blockchain::HybridChain;
use crate::core::AnalysisResult;
use std::sync::Arc;
use tokio::sync::Mutex;

// Minimal analyzer that loads only when needed
pub struct BlockchainAnalyzer {
    chain: Arc<Mutex<HybridChain>>,
}

impl BlockchainAnalyzer {
    pub fn new(chain: Arc<Mutex<HybridChain>>) -> Self {
        Self { chain }
    }
    
    pub async fn analyze(&self) -> Result<AnalysisResult, Box<dyn std::error::Error + Send + Sync>> {
        let mut result = AnalysisResult::new("offline_blockchain");
        
        // Only calculate essential metrics
        let storage_size = self.calculate_storage_size().await?;
        result.add_metric("blockchain_storage_kb", (storage_size / 1024).to_string());
        
        let batch_efficiency = self.estimate_batch_efficiency().await?;
        result.add_metric("batch_efficiency", format!("{:.1}%", batch_efficiency * 100.0));
        
        // Add basic next steps
        if batch_efficiency < 0.7 {
            result.add_next_step("Improve batch efficiency by adjusting batch interval");
        }
        
        result.add_next_step("Monitor blockchain storage growth");
        
        Ok(result)
    }
    
    // Use filesystem inspection instead of loading full blockchain
    async fn calculate_storage_size(&self) -> Result<usize, Box<dyn std::error::Error + Send + Sync>> {
        match std::fs::metadata("./chain-data") {
            Ok(metadata) => Ok(metadata.len() as usize),
            Err(_) => Ok(0),
        }
    }
    
    // Simplified estimation
    async fn estimate_batch_efficiency(&self) -> Result<f64, Box<dyn std::error::Error + Send + Sync>> {
        // Simple estimation instead of complex calculation
        Ok(0.9) // 90% estimated efficiency
    }
}