use tauri::{command, State};
use crate::core::monitoring::{
    get_monitoring_data, get_monitoring_history, save_monitoring_snapshot,
    update_health_metrics, record_sync_attempt, record_conflict,
    record_conflict_resolution, update_connection_status, cleanup_old_monitoring_data,
    MonitoringData,
};

#[command]
pub async fn get_system_monitoring_data() -> Result<MonitoringData, String> {
    get_monitoring_data()
}

#[command]
pub async fn get_system_monitoring_history(days: u32) -> Result<Vec<MonitoringData>, String> {
    get_monitoring_history(days)
}

#[command]
pub async fn save_system_monitoring_snapshot() -> Result<(), String> {
    save_monitoring_snapshot()
}

#[command]
pub async fn update_system_health(
    canvas_api_healthy: bool,
    discourse_api_healthy: bool,
    local_db_healthy: bool,
    sync_engine_healthy: bool,
) -> Result<(), String> {
    update_health_metrics(canvas_api_healthy, discourse_api_healthy, local_db_healthy, sync_engine_healthy)
}

#[command]
pub async fn record_system_sync_attempt(success: bool, duration_ms: u64) -> Result<(), String> {
    record_sync_attempt(success, duration_ms)
}

#[command]
pub async fn record_system_conflict(auto_resolved: bool) -> Result<(), String> {
    record_conflict(auto_resolved)
}

#[command]
pub async fn record_system_conflict_resolution(count: u64) -> Result<(), String> {
    record_conflict_resolution(count)
}

#[command]
pub async fn update_system_connection_status(is_online: bool, minutes: u64) -> Result<(), String> {
    update_connection_status(is_online, minutes)
}

#[command]
pub async fn cleanup_system_monitoring_data(days_to_keep: u32) -> Result<usize, String> {
    cleanup_old_monitoring_data(days_to_keep)
}