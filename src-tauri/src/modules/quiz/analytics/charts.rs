use anyhow::{Result, anyhow};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use sqlx::SqlitePool;
use chrono::{NaiveDate, Utc, DateTime};
use tracing::{info, error};

/// Chart data types
#[derive(Debug, Serialize, Deserialize)]
pub enum ChartType {
    Bar,
    Line,
    Pie,
    Doughnut,
    Radar,
    PolarArea,
    Bubble,
    Scatter,
}

/// Chart data for the analytics dashboard
#[derive(Debug, Serialize, Deserialize)]
pub struct ChartData {
    pub chart_type: ChartType,
    pub title: String,
    pub labels: Vec<String>,
    pub datasets: Vec<Dataset>,
    pub x_axis_label: Option<String>,
    pub y_axis_label: Option<String>,
}

/// Dataset for chart data
#[derive(Debug, Serialize, Deserialize)]
pub struct Dataset {
    pub label: String,
    pub data: Vec<f64>,
    pub background_color: Option<Vec<String>>,
    pub border_color: Option<Vec<String>>,
    pub fill: Option<bool>,
}

/// Chart service for generating chart data
pub struct ChartService {
    pool: SqlitePool,
}

impl ChartService {
    /// Create a new chart service
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
    
    /// Generate activity by day chart
    pub async fn activity_by_day_chart(
        &self,
        user_id: Option<&str>,
        quiz_id: Option<&str>,
        start_date: Option<NaiveDate>,
        end_date: Option<NaiveDate>,
    ) -> Result<ChartData> {
        // Build date filter
        let date_filter = match (start_date, end_date) {
            (Some(start), Some(end)) => {
                format!("AND timestamp BETWEEN '{}' AND '{}'", 
                    start.format("%Y-%m-%d"),
                    end.format("%Y-%m-%d"))
            },
            (Some(start), None) => {
                format!("AND timestamp >= '{}'", start.format("%Y-%m-%d"))
            },
            (None, Some(end)) => {
                format!("AND timestamp <= '{}'", end.format("%Y-%m-%d"))
            },
            (None, None) => String::new(),
        };
        
        // Build user filter
        let user_filter = match user_id {
            Some(id) => format!("AND user_id = '{}'", id),
            None => String::new(),
        };
        
        // Build quiz filter
        let quiz_filter = match quiz_id {
            Some(id) => format!("AND quiz_id = '{}'", id),
            None => String::new(),
        };
        
        // Get activity by day
        let activity_by_day = sqlx::query!(
            r#"
            SELECT 
                strftime('%Y-%m-%d', timestamp) as day,
                COUNT(*) as count
            FROM quiz_activities
            WHERE 1=1
            "#.to_string() + &user_filter + &quiz_filter + &date_filter + "
            GROUP BY day
            ORDER BY day"
        )
        .fetch_all(&self.pool)
        .await?;
        
        // Extract labels and data
        let mut labels = Vec::new();
        let mut data = Vec::new();
        
        for row in activity_by_day {
            if let Some(day) = row.day {
                labels.push(day);
                data.push(row.count as f64);
            }
        }
        
        // Create chart data
        let chart_data = ChartData {
            chart_type: ChartType::Line,
            title: "Activity by Day".to_string(),
            labels,
            datasets: vec![Dataset {
                label: "Activities".to_string(),
                data,
                background_color: Some(vec!["rgba(75, 192, 192, 0.2)".to_string()]),
                border_color: Some(vec!["rgba(75, 192, 192, 1)".to_string()]),
                fill: Some(true),
            }],
            x_axis_label: Some("Date".to_string()),
            y_axis_label: Some("Activities".to_string()),
        };
        
        Ok(chart_data)
    }
    
    /// Generate activity by type chart
    pub async fn activity_by_type_chart(
        &self,
        user_id: Option<&str>,
        quiz_id: Option<&str>,
        start_date: Option<NaiveDate>,
        end_date: Option<NaiveDate>,
    ) -> Result<ChartData> {
        // Build date filter
        let date_filter = match (start_date, end_date) {
            (Some(start), Some(end)) => {
                format!("AND timestamp BETWEEN '{}' AND '{}'", 
                    start.format("%Y-%m-%d"),
                    end.format("%Y-%m-%d"))
            },
            (Some(start), None) => {
                format!("AND timestamp >= '{}'", start.format("%Y-%m-%d"))
            },
            (None, Some(end)) => {
                format!("AND timestamp <= '{}'", end.format("%Y-%m-%d"))
            },
            (None, None) => String::new(),
        };
        
        // Build user filter
        let user_filter = match user_id {
            Some(id) => format!("AND user_id = '{}'", id),
            None => String::new(),
        };
        
        // Build quiz filter
        let quiz_filter = match quiz_id {
            Some(id) => format!("AND quiz_id = '{}'", id),
            None => String::new(),
        };
        
        // Get activity by type
        let activity_by_type = sqlx::query!(
            r#"
            SELECT 
                activity_type,
                COUNT(*) as count
            FROM quiz_activities
            WHERE 1=1
            "#.to_string() + &user_filter + &quiz_filter + &date_filter + "
            GROUP BY activity_type
            ORDER BY count DESC"
        )
        .fetch_all(&self.pool)
        .await?;
        
        // Extract labels and data
        let mut labels = Vec::new();
        let mut data = Vec::new();
        
        for row in activity_by_type {
            labels.push(row.activity_type);
            data.push(row.count as f64);
        }
        
        // Generate colors
        let background_colors = generate_colors(labels.len(), 0.7);
        let border_colors = generate_colors(labels.len(), 1.0);
        
        // Create chart data
        let chart_data = ChartData {
            chart_type: ChartType::Pie,
            title: "Activity by Type".to_string(),
            labels,
            datasets: vec![Dataset {
                label: "Activities".to_string(),
                data,
                background_color: Some(background_colors),
                border_color: Some(border_colors),
                fill: None,
            }],
            x_axis_label: None,
            y_axis_label: None,
        };
        
        Ok(chart_data)
    }
    
    /// Generate quiz completion chart
    pub async fn quiz_completion_chart(
        &self,
        user_id: Option<&str>,
        start_date: Option<NaiveDate>,
        end_date: Option<NaiveDate>,
    ) -> Result<ChartData> {
        // Build date filter
        let date_filter = match (start_date, end_date) {
            (Some(start), Some(end)) => {
                format!("AND timestamp BETWEEN '{}' AND '{}'", 
                    start.format("%Y-%m-%d"),
                    end.format("%Y-%m-%d"))
            },
            (Some(start), None) => {
                format!("AND timestamp >= '{}'", start.format("%Y-%m-%d"))
            },
            (None, Some(end)) => {
                format!("AND timestamp <= '{}'", end.format("%Y-%m-%d"))
            },
            (None, None) => String::new(),
        };
        
        // Build user filter
        let user_filter = match user_id {
            Some(id) => format!("AND user_id = '{}'", id),
            None => String::new(),
        };
        
        // Get quiz completion data
        let quiz_completion = sqlx::query!(
            r#"
            SELECT 
                COUNT(DISTINCT CASE WHEN activity_type = 'quiz_started' THEN quiz_id || '-' || user_id END) as started,
                COUNT(DISTINCT CASE WHEN activity_type = 'quiz_completed' THEN quiz_id || '-' || user_id END) as completed,
                COUNT(DISTINCT CASE WHEN activity_type = 'quiz_abandoned' THEN quiz_id || '-' || user_id END) as abandoned
            FROM quiz_activities
            WHERE quiz_id IS NOT NULL
            "#.to_string() + &user_filter + &date_filter
        )
        .fetch_one(&self.pool)
        .await?;
        
        // Create chart data
        let chart_data = ChartData {
            chart_type: ChartType::Bar,
            title: "Quiz Completion".to_string(),
            labels: vec!["Started".to_string(), "Completed".to_string(), "Abandoned".to_string()],
            datasets: vec![Dataset {
                label: "Quizzes".to_string(),
                data: vec![
                    quiz_completion.started as f64,
                    quiz_completion.completed as f64,
                    quiz_completion.abandoned as f64,
                ],
                background_color: Some(vec![
                    "rgba(54, 162, 235, 0.7)".to_string(),
                    "rgba(75, 192, 192, 0.7)".to_string(),
                    "rgba(255, 99, 132, 0.7)".to_string(),
                ]),
                border_color: Some(vec![
                    "rgba(54, 162, 235, 1)".to_string(),
                    "rgba(75, 192, 192, 1)".to_string(),
                    "rgba(255, 99, 132, 1)".to_string(),
                ]),
                fill: None,
            }],
            x_axis_label: Some("Status".to_string()),
            y_axis_label: Some("Count".to_string()),
        };
        
        Ok(chart_data)
    }
    
    /// Generate time distribution chart
    pub async fn time_distribution_chart(
        &self,
        user_id: Option<&str>,
        quiz_id: Option<&str>,
        start_date: Option<NaiveDate>,
        end_date: Option<NaiveDate>,
    ) -> Result<ChartData> {
        // Build date filter
        let date_filter = match (start_date, end_date) {
            (Some(start), Some(end)) => {
                format!("AND timestamp BETWEEN '{}' AND '{}'", 
                    start.format("%Y-%m-%d"),
                    end.format("%Y-%m-%d"))
            },
            (Some(start), None) => {
                format!("AND timestamp >= '{}'", start.format("%Y-%m-%d"))
            },
            (None, Some(end)) => {
                format!("AND timestamp <= '{}'", end.format("%Y-%m-%d"))
            },
            (None, None) => String::new(),
        };
        
        // Build user filter
        let user_filter = match user_id {
            Some(id) => format!("AND user_id = '{}'", id),
            None => String::new(),
        };
        
        // Build quiz filter
        let quiz_filter = match quiz_id {
            Some(id) => format!("AND quiz_id = '{}'", id),
            None => String::new(),
        };
        
        // Get activity by hour
        let activity_by_hour = sqlx::query!(
            r#"
            SELECT 
                CAST(strftime('%H', timestamp) AS INTEGER) as hour,
                COUNT(*) as count
            FROM quiz_activities
            WHERE 1=1
            "#.to_string() + &user_filter + &quiz_filter + &date_filter + "
            GROUP BY hour
            ORDER BY hour"
        )
        .fetch_all(&self.pool)
        .await?;
        
        // Create a map for all 24 hours
        let mut hour_map = HashMap::new();
        for hour in 0..24 {
            hour_map.insert(hour, 0);
        }
        
        // Fill in the data
        for row in activity_by_hour {
            if let Some(hour) = row.hour {
                hour_map.insert(hour, row.count);
            }
        }
        
        // Extract labels and data
        let mut labels = Vec::new();
        let mut data = Vec::new();
        
        for hour in 0..24 {
            labels.push(format!("{:02}:00", hour));
            data.push(*hour_map.get(&hour).unwrap_or(&0) as f64);
        }
        
        // Create chart data
        let chart_data = ChartData {
            chart_type: ChartType::Bar,
            title: "Activity by Hour".to_string(),
            labels,
            datasets: vec![Dataset {
                label: "Activities".to_string(),
                data,
                background_color: Some(vec!["rgba(153, 102, 255, 0.7)".to_string()]),
                border_color: Some(vec!["rgba(153, 102, 255, 1)".to_string()]),
                fill: None,
            }],
            x_axis_label: Some("Hour".to_string()),
            y_axis_label: Some("Activities".to_string()),
        };
        
        Ok(chart_data)
    }
}

/// Generate colors for charts
fn generate_colors(count: usize, alpha: f32) -> Vec<String> {
    let mut colors = Vec::new();
    
    // Base colors
    let base_colors = [
        (255, 99, 132),   // Red
        (54, 162, 235),   // Blue
        (255, 206, 86),   // Yellow
        (75, 192, 192),   // Green
        (153, 102, 255),  // Purple
        (255, 159, 64),   // Orange
        (199, 199, 199),  // Gray
        (83, 102, 255),   // Indigo
        (255, 99, 255),   // Pink
        (0, 168, 133),    // Teal
    ];
    
    for i in 0..count {
        let (r, g, b) = base_colors[i % base_colors.len()];
        colors.push(format!("rgba({}, {}, {}, {})", r, g, b, alpha));
    }
    
    colors
}
