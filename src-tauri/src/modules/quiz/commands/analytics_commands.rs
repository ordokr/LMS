use tauri::{State, command};
use std::sync::Arc;
use anyhow::Result;
use serde::{Serialize, Deserialize};
use chrono::NaiveDate;

use crate::app_state::AppState;
use crate::modules::quiz::analytics::{
    dashboard::{AnalyticsDashboard, DashboardData},
    charts::{ChartService, ChartData},
    reports::{ReportService, ReportData, ReportType}
};

/// Get dashboard data
#[command]
pub async fn get_dashboard_data(
    user_id: Option<String>,
    quiz_id: Option<String>,
    start_date: Option<String>,
    end_date: Option<String>,
    app_state: State<'_, Arc<AppState>>
) -> Result<DashboardData, String> {
    let db_pool = app_state.db_pool.clone();
    let dashboard = AnalyticsDashboard::new(db_pool);
    
    // Parse dates if provided
    let start_date = start_date.and_then(|d| NaiveDate::parse_from_str(&d, "%Y-%m-%d").ok());
    let end_date = end_date.and_then(|d| NaiveDate::parse_from_str(&d, "%Y-%m-%d").ok());
    
    dashboard.get_dashboard_data(
        user_id.as_deref(),
        quiz_id.as_deref(),
        start_date,
        end_date
    )
    .await
    .map_err(|e| e.to_string())
}

/// Get activity by day chart
#[command]
pub async fn get_activity_by_day_chart(
    user_id: Option<String>,
    quiz_id: Option<String>,
    start_date: Option<String>,
    end_date: Option<String>,
    app_state: State<'_, Arc<AppState>>
) -> Result<ChartData, String> {
    let db_pool = app_state.db_pool.clone();
    let chart_service = ChartService::new(db_pool);
    
    // Parse dates if provided
    let start_date = start_date.and_then(|d| NaiveDate::parse_from_str(&d, "%Y-%m-%d").ok());
    let end_date = end_date.and_then(|d| NaiveDate::parse_from_str(&d, "%Y-%m-%d").ok());
    
    chart_service.activity_by_day_chart(
        user_id.as_deref(),
        quiz_id.as_deref(),
        start_date,
        end_date
    )
    .await
    .map_err(|e| e.to_string())
}

/// Get activity by type chart
#[command]
pub async fn get_activity_by_type_chart(
    user_id: Option<String>,
    quiz_id: Option<String>,
    start_date: Option<String>,
    end_date: Option<String>,
    app_state: State<'_, Arc<AppState>>
) -> Result<ChartData, String> {
    let db_pool = app_state.db_pool.clone();
    let chart_service = ChartService::new(db_pool);
    
    // Parse dates if provided
    let start_date = start_date.and_then(|d| NaiveDate::parse_from_str(&d, "%Y-%m-%d").ok());
    let end_date = end_date.and_then(|d| NaiveDate::parse_from_str(&d, "%Y-%m-%d").ok());
    
    chart_service.activity_by_type_chart(
        user_id.as_deref(),
        quiz_id.as_deref(),
        start_date,
        end_date
    )
    .await
    .map_err(|e| e.to_string())
}

/// Get quiz completion chart
#[command]
pub async fn get_quiz_completion_chart(
    user_id: Option<String>,
    start_date: Option<String>,
    end_date: Option<String>,
    app_state: State<'_, Arc<AppState>>
) -> Result<ChartData, String> {
    let db_pool = app_state.db_pool.clone();
    let chart_service = ChartService::new(db_pool);
    
    // Parse dates if provided
    let start_date = start_date.and_then(|d| NaiveDate::parse_from_str(&d, "%Y-%m-%d").ok());
    let end_date = end_date.and_then(|d| NaiveDate::parse_from_str(&d, "%Y-%m-%d").ok());
    
    chart_service.quiz_completion_chart(
        user_id.as_deref(),
        start_date,
        end_date
    )
    .await
    .map_err(|e| e.to_string())
}

/// Get time distribution chart
#[command]
pub async fn get_time_distribution_chart(
    user_id: Option<String>,
    quiz_id: Option<String>,
    start_date: Option<String>,
    end_date: Option<String>,
    app_state: State<'_, Arc<AppState>>
) -> Result<ChartData, String> {
    let db_pool = app_state.db_pool.clone();
    let chart_service = ChartService::new(db_pool);
    
    // Parse dates if provided
    let start_date = start_date.and_then(|d| NaiveDate::parse_from_str(&d, "%Y-%m-%d").ok());
    let end_date = end_date.and_then(|d| NaiveDate::parse_from_str(&d, "%Y-%m-%d").ok());
    
    chart_service.time_distribution_chart(
        user_id.as_deref(),
        quiz_id.as_deref(),
        start_date,
        end_date
    )
    .await
    .map_err(|e| e.to_string())
}

/// Generate user activity report
#[command]
pub async fn generate_user_activity_report(
    user_id: String,
    start_date: Option<String>,
    end_date: Option<String>,
    app_state: State<'_, Arc<AppState>>
) -> Result<ReportData, String> {
    let db_pool = app_state.db_pool.clone();
    let report_service = ReportService::new(db_pool);
    
    // Parse dates if provided
    let start_date = start_date.and_then(|d| NaiveDate::parse_from_str(&d, "%Y-%m-%d").ok());
    let end_date = end_date.and_then(|d| NaiveDate::parse_from_str(&d, "%Y-%m-%d").ok());
    
    report_service.generate_user_activity_report(
        &user_id,
        start_date,
        end_date
    )
    .await
    .map_err(|e| e.to_string())
}

/// Generate quiz performance report
#[command]
pub async fn generate_quiz_performance_report(
    quiz_id: String,
    start_date: Option<String>,
    end_date: Option<String>,
    app_state: State<'_, Arc<AppState>>
) -> Result<ReportData, String> {
    let db_pool = app_state.db_pool.clone();
    let report_service = ReportService::new(db_pool);
    
    // Parse dates if provided
    let start_date = start_date.and_then(|d| NaiveDate::parse_from_str(&d, "%Y-%m-%d").ok());
    let end_date = end_date.and_then(|d| NaiveDate::parse_from_str(&d, "%Y-%m-%d").ok());
    
    report_service.generate_quiz_performance_report(
        &quiz_id,
        start_date,
        end_date
    )
    .await
    .map_err(|e| e.to_string())
}

/// Generate question analysis report
#[command]
pub async fn generate_question_analysis_report(
    quiz_id: String,
    app_state: State<'_, Arc<AppState>>
) -> Result<ReportData, String> {
    let db_pool = app_state.db_pool.clone();
    let report_service = ReportService::new(db_pool);
    
    report_service.generate_question_analysis_report(&quiz_id)
        .await
        .map_err(|e| e.to_string())
}

/// Generate time analysis report
#[command]
pub async fn generate_time_analysis_report(
    user_id: Option<String>,
    quiz_id: Option<String>,
    start_date: Option<String>,
    end_date: Option<String>,
    app_state: State<'_, Arc<AppState>>
) -> Result<ReportData, String> {
    let db_pool = app_state.db_pool.clone();
    let report_service = ReportService::new(db_pool);
    
    // Parse dates if provided
    let start_date = start_date.and_then(|d| NaiveDate::parse_from_str(&d, "%Y-%m-%d").ok());
    let end_date = end_date.and_then(|d| NaiveDate::parse_from_str(&d, "%Y-%m-%d").ok());
    
    report_service.generate_time_analysis_report(
        user_id.as_deref(),
        quiz_id.as_deref(),
        start_date,
        end_date
    )
    .await
    .map_err(|e| e.to_string())
}
