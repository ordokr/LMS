use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use lazy_static::lazy_static;
use rusqlite::{params, Connection};
use crate::db::establish_connection;

/// Struct representing system health metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemHealth {
    pub canvas_api_healthy: bool,
    pub discourse_api_healthy: bool,
    pub local_db_healthy: bool,
    pub sync_engine_healthy: bool,
    pub last_checked: DateTime<Utc>,
}

/// Struct for sync statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncStatistics {
    pub total_sync_attempts: u64,
    pub successful_syncs: u64,
    pub failed_syncs: u64,
    pub pending_syncs: u64,
    pub last_successful_sync: Option<DateTime<Utc>>,
    pub average_sync_duration_ms: Option<u64>,
}

/// Struct for monitoring sync conflicts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictMetrics {
    pub total_conflicts_detected: u64,
    pub auto_resolved_conflicts: u64,
    pub manually_resolved_conflicts: u64,
    pub pending_conflicts: u64,
}

/// Comprehensive monitoring data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringData {
    pub health: SystemHealth,
    pub sync_stats: SyncStatistics,
    pub conflict_metrics: ConflictMetrics,
    pub offline_duration_minutes: u64,
    pub online_duration_minutes: u64,
    pub database_size_kb: u64,
    pub timestamp: DateTime<Utc>,
}

lazy_static! {
    static ref MONITORING_DATA: Mutex<MonitoringData> = Mutex::new(MonitoringData {
        health: SystemHealth {
            canvas_api_healthy: true,
            discourse_api_healthy: true,
            local_db_healthy: true,
            sync_engine_healthy: true,
            last_checked: Utc::now(),
        },
        sync_stats: SyncStatistics {
            total_sync_attempts: 0,
            successful_syncs: 0,
            failed_syncs: 0,
            pending_syncs: 0,
            last_successful_sync: None,
            average_sync_duration_ms: None,
        },
        conflict_metrics: ConflictMetrics {
            total_conflicts_detected: 0,
            auto_resolved_conflicts: 0,
            manually_resolved_conflicts: 0,
            pending_conflicts: 0,
        },
        offline_duration_minutes: 0,
        online_duration_minutes: 0,
        database_size_kb: 0,
        timestamp: Utc::now(),
    });
}

/// Initialize the monitoring system
pub fn initialize_monitoring() -> Result<(), String> {
    let conn = establish_connection().map_err(|e| format!("Failed to connect to database: {}", e))?;
    
    // Create monitoring tables if they don't exist
    conn.execute(
        "CREATE TABLE IF NOT EXISTS monitoring_history (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            health_snapshot TEXT NOT NULL,
            sync_stats TEXT NOT NULL,
            conflict_metrics TEXT NOT NULL,
            offline_duration_minutes INTEGER NOT NULL,
            online_duration_minutes INTEGER NOT NULL,
            database_size_kb INTEGER NOT NULL,
            timestamp TEXT NOT NULL
        )",
        [],
    ).map_err(|e| format!("Failed to create monitoring tables: {}", e))?;
    
    // Initialize with current data
    update_health_metrics(true, true, true, true).map_err(|e| format!("Failed to update health metrics: {}", e))?;
    
    Ok(())
}

/// Update health status metrics
pub fn update_health_metrics(
    canvas_api_healthy: bool,
    discourse_api_healthy: bool,
    local_db_healthy: bool,
    sync_engine_healthy: bool,
) -> Result<(), String> {
    let mut data = MONITORING_DATA.lock().map_err(|_| "Failed to acquire lock on monitoring data")?;
    data.health = SystemHealth {
        canvas_api_healthy,
        discourse_api_healthy,
        local_db_healthy,
        sync_engine_healthy,
        last_checked: Utc::now(),
    };
    data.timestamp = Utc::now();
    
    Ok(())
}

/// Record a sync attempt result
pub fn record_sync_attempt(success: bool, duration_ms: u64) -> Result<(), String> {
    let mut data = MONITORING_DATA.lock().map_err(|_| "Failed to acquire lock on monitoring data")?;
    
    data.sync_stats.total_sync_attempts += 1;
    
    if success {
        data.sync_stats.successful_syncs += 1;
        data.sync_stats.last_successful_sync = Some(Utc::now());
    } else {
        data.sync_stats.failed_syncs += 1;
    }
    
    // Update average duration
    if let Some(avg) = data.sync_stats.average_sync_duration_ms {
        let total = avg * (data.sync_stats.total_sync_attempts - 1) + duration_ms;
        data.sync_stats.average_sync_duration_ms = Some(total / data.sync_stats.total_sync_attempts);
    } else {
        data.sync_stats.average_sync_duration_ms = Some(duration_ms);
    }
    
    data.timestamp = Utc::now();
    
    Ok(())
}

/// Record a conflict event
pub fn record_conflict(auto_resolved: bool) -> Result<(), String> {
    let mut data = MONITORING_DATA.lock().map_err(|_| "Failed to acquire lock on monitoring data")?;
    
    data.conflict_metrics.total_conflicts_detected += 1;
    
    if auto_resolved {
        data.conflict_metrics.auto_resolved_conflicts += 1;
    } else {
        data.conflict_metrics.pending_conflicts += 1;
    }
    
    data.timestamp = Utc::now();
    
    Ok(())
}

/// Record conflict resolution
pub fn record_conflict_resolution(count: u64) -> Result<(), String> {
    let mut data = MONITORING_DATA.lock().map_err(|_| "Failed to acquire lock on monitoring data")?;
    
    let resolved = std::cmp::min(count, data.conflict_metrics.pending_conflicts);
    data.conflict_metrics.manually_resolved_conflicts += resolved;
    data.conflict_metrics.pending_conflicts -= resolved;
    
    data.timestamp = Utc::now();
    
    Ok(())
}

/// Update connection status tracking
pub fn update_connection_status(is_online: bool, minutes: u64) -> Result<(), String> {
    let mut data = MONITORING_DATA.lock().map_err(|_| "Failed to acquire lock on monitoring data")?;
    
    if is_online {
        data.online_duration_minutes += minutes;
    } else {
        data.offline_duration_minutes += minutes;
    }
    
    data.timestamp = Utc::now();
    
    Ok(())
}

/// Get current monitoring data
pub fn get_monitoring_data() -> Result<MonitoringData, String> {
    let data = MONITORING_DATA.lock().map_err(|_| "Failed to acquire lock on monitoring data")?;
    Ok(data.clone())
}

/// Save monitoring snapshot to SQLite
pub fn save_monitoring_snapshot() -> Result<(), String> {
    let data = MONITORING_DATA.lock().map_err(|_| "Failed to acquire lock on monitoring data")?;
    let conn = establish_connection().map_err(|e| format!("Failed to connect to database: {}", e))?;
    
    let health_json = serde_json::to_string(&data.health)
        .map_err(|e| format!("Failed to serialize health data: {}", e))?;
    
    let sync_stats_json = serde_json::to_string(&data.sync_stats)
        .map_err(|e| format!("Failed to serialize sync stats: {}", e))?;
    
    let conflict_metrics_json = serde_json::to_string(&data.conflict_metrics)
        .map_err(|e| format!("Failed to serialize conflict metrics: {}", e))?;
    
    conn.execute(
        "INSERT INTO monitoring_history 
        (health_snapshot, sync_stats, conflict_metrics, offline_duration_minutes, 
         online_duration_minutes, database_size_kb, timestamp) 
        VALUES (?, ?, ?, ?, ?, ?, ?)",
        params![
            health_json,
            sync_stats_json,
            conflict_metrics_json,
            data.offline_duration_minutes,
            data.online_duration_minutes,
            data.database_size_kb,
            data.timestamp.to_rfc3339()
        ],
    ).map_err(|e| format!("Failed to save monitoring snapshot: {}", e))?;
    
    Ok(())
}

/// Get monitoring history for a specific time range
pub fn get_monitoring_history(days: u32) -> Result<Vec<MonitoringData>, String> {
    let conn = establish_connection().map_err(|e| format!("Failed to connect to database: {}", e))?;
    
    let mut stmt = conn.prepare(
        "SELECT health_snapshot, sync_stats, conflict_metrics, 
         offline_duration_minutes, online_duration_minutes, database_size_kb, timestamp
         FROM monitoring_history
         WHERE timestamp > datetime('now', ?1)
         ORDER BY timestamp DESC"
    ).map_err(|e| format!("Failed to prepare statement: {}", e))?;
    
    let days_param = format!("-{} days", days);
    
    let rows = stmt.query_map(params![days_param], |row| {
        let health_json: String = row.get(0)?;
        let sync_stats_json: String = row.get(1)?;
        let conflict_metrics_json: String = row.get(2)?;
        let offline_duration_minutes: u64 = row.get(3)?;
        let online_duration_minutes: u64 = row.get(4)?;
        let database_size_kb: u64 = row.get(5)?;
        let timestamp_str: String = row.get(6)?;
        
        let health: SystemHealth = serde_json::from_str(&health_json)
            .map_err(|_| rusqlite::Error::InvalidParameterName("Failed to parse health data".to_string()))?;
        
        let sync_stats: SyncStatistics = serde_json::from_str(&sync_stats_json)
            .map_err(|_| rusqlite::Error::InvalidParameterName("Failed to parse sync stats".to_string()))?;
        
        let conflict_metrics: ConflictMetrics = serde_json::from_str(&conflict_metrics_json)
            .map_err(|_| rusqlite::Error::InvalidParameterName("Failed to parse conflict metrics".to_string()))?;
        
        let timestamp = DateTime::parse_from_rfc3339(&timestamp_str)
            .map_err(|_| rusqlite::Error::InvalidParameterName("Failed to parse timestamp".to_string()))?
            .with_timezone(&Utc);
        
        Ok(MonitoringData {
            health,
            sync_stats,
            conflict_metrics,
            offline_duration_minutes,
            online_duration_minutes,
            database_size_kb,
            timestamp,
        })
    }).map_err(|e| format!("Failed to execute query: {}", e))?;
    
    let mut result = Vec::new();
    for row in rows {
        result.push(row.map_err(|e| format!("Failed to process row: {}", e))?);
    }
    
    Ok(result)
}

/// Clean up old monitoring data
pub fn cleanup_old_monitoring_data(days_to_keep: u32) -> Result<usize, String> {
    let conn = establish_connection().map_err(|e| format!("Failed to connect to database: {}", e))?;
    
    let days_param = format!("-{} days", days_to_keep);
    
    let result = conn.execute(
        "DELETE FROM monitoring_history WHERE timestamp < datetime('now', ?1)",
        params![days_param],
    ).map_err(|e| format!("Failed to delete old monitoring data: {}", e))?;
    
    Ok(result)
}