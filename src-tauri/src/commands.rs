use tauri::{State, command};
use crate::blockchain::{HybridChain, crdt_ops::LmsCrdtOps};
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;
use crate::blockchain::{
    BlockchainError,
    UserEvent,
    SyncPriority,
    AdaptiveSyncManager
};
use crate::parser_integration::{CompletionRuleParser, QueryParser};

pub struct AppState {
    chain: Arc<Mutex<HybridChain>>,
    crdt_ops: Arc<Mutex<LmsCrdtOps>>,
}

#[command]
pub async fn award_badge(
    badge_id: String,
    student_id: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let badge_uuid = Uuid::parse_str(&badge_id).map_err(|e| e.to_string())?;
    let student_uuid = Uuid::parse_str(&student_id).map_err(|e| e.to_string())?;
    
    // Update CRDT state
    {
        let mut ops = state.crdt_ops.lock().await;
        ops.award_badge(student_uuid, badge_uuid)
            .map_err(|e| e.to_string())?;
    }
    
    // Create a new block to record this action
    {
        let mut chain = state.chain.lock().await;
        chain.create_block()
            .map_err(|e| e.to_string())?;
    }
    
    Ok(())
}

#[command]
pub async fn create_certificate(
    student_id: String,
    course_id: String,
    metadata: String,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let student_uuid = Uuid::parse_str(&student_id).map_err(|e| e.to_string())?;
    let course_uuid = Uuid::parse_str(&course_id).map_err(|e| e.to_string())?;
    
    // Create certificate in CRDT
    let certificate_id = {
        let mut ops = state.crdt_ops.lock().await;
        ops.create_certificate(student_uuid, course_uuid, &metadata)
            .map_err(|e| e.to_string())?
    };
    
    // Record the certificate creation in blockchain
    {
        let mut chain = state.chain.lock().await;
        chain.create_block()
            .map_err(|e| e.to_string())?;
    }
    
    Ok(certificate_id.to_string())
}

#[command]
pub async fn sync_with_peers(
    state: State<'_, AppState>,
) -> Result<(), String> {
    // This would trigger P2P synchronization
    // For now, just a placeholder
    println!("Triggering P2P sync...");
    Ok(())
}

#[command]
pub async fn record_achievement(
    student_id: String,
    course_id: String,
    achievement_type: String,
    state: State<'_, AppState>,
) -> Result<String, String> {
    // Create event
    let event = UserEvent::BadgeAwarded(crate::blockchain::sync::BadgeAwarded {
        student_id: student_id.clone(),
        course_id: course_id.clone(),
        achievement_type: achievement_type.clone(),
        timestamp: chrono::Utc::now().timestamp(),
    });
    
    // Use sync manager to handle the event
    let result = state.sync_manager.sync_event(&event).await
        .map_err(|e| e.to_string())?;
    
    Ok(format!("Achievement recorded for student {} in course {}", student_id, course_id))
}

#[command]
pub async fn verify_achievement(
    student_id: String,
    course_id: String,
    achievement_type: String,
    state: State<'_, AppState>,
) -> Result<bool, String> {
    // Implementation to verify an achievement from blockchain
    // This would access the chain to check if the achievement exists
    
    // Placeholder implementation
    Ok(true)
}

#[command]
pub fn parse_completion_rule(rule_text: String) -> Result<String, String> {
    let parser = CompletionRuleParser {};
    
    match parser.parse_rule(&rule_text) {
        Ok(requirement) => {
            serde_json::to_string(&requirement)
                .map_err(|e| format!("JSON serialization error: {}", e))
        },
        Err(e) => Err(format!("Parse error: {}", e)),
    }
}

#[command]
pub fn parse_and_optimize_query(query_text: String) -> Result<String, String> {
    let parser = QueryParser {};
    
    // Parse the query
    let query = parser.parse_query(&query_text)
        .map_err(|e| format!("Parse error: {}", e))?;
        
    // Optimize the query
    let optimized = parser.optimize_query(&query)
        .map_err(|e| format!("Optimization error: {}", e))?;
    
    // Return the optimized query
    serde_json::to_string(&optimized)
        .map_err(|e| format!("JSON serialization error: {}", e))
}