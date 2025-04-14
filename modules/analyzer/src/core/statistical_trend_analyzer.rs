use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use chrono::{DateTime, Local, NaiveDate, NaiveDateTime, Duration};
use serde::{Serialize, Deserialize};
use statrs::statistics::{Data, Distribution, Mean, Variance};
use statrs::statistics::Statistics;
use statrs::distribution::{Normal, ContinuousCDF};
use statrs::function::erf::erf;
use statrs::function::gamma::gamma;
use statrs::regression::linear::{SimpleLinearRegression, SimpleLinearModel};

use crate::core::analysis_result::AnalysisResult;
use crate::core::trend_analyzer::{TrendAnalyzer, AnalysisHistory, HistoryEntry};

/// Statistical trend analyzer
pub struct StatisticalTrendAnalyzer {
    /// Base directory for analysis
    base_dir: PathBuf,
    
    /// Trend analyzer
    trend_analyzer: TrendAnalyzer,
}

/// Trend forecast
#[derive(Debug, Clone)]
pub struct TrendForecast {
    /// Metric name
    pub metric: String,
    
    /// Current value
    pub current_value: f32,
    
    /// Forecasted value in 1 week
    pub forecast_1_week: f32,
    
    /// Forecasted value in 1 month
    pub forecast_1_month: f32,
    
    /// Forecasted value in 3 months
    pub forecast_3_months: f32,
    
    /// Confidence interval for 1 week forecast (lower, upper)
    pub confidence_1_week: (f32, f32),
    
    /// Confidence interval for 1 month forecast (lower, upper)
    pub confidence_1_month: (f32, f32),
    
    /// Confidence interval for 3 months forecast (lower, upper)
    pub confidence_3_months: (f32, f32),
    
    /// Growth rate (percentage per week)
    pub growth_rate: f32,
    
    /// Trend direction
    pub trend_direction: TrendDirection,
    
    /// Trend strength
    pub trend_strength: TrendStrength,
}

/// Trend direction
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TrendDirection {
    /// Increasing trend
    Increasing,
    
    /// Decreasing trend
    Decreasing,
    
    /// Stable trend
    Stable,
}

/// Trend strength
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TrendStrength {
    /// Strong trend
    Strong,
    
    /// Moderate trend
    Moderate,
    
    /// Weak trend
    Weak,
}

/// Statistical trend metrics
#[derive(Debug, Clone)]
pub struct StatisticalTrendMetrics {
    /// Trend forecasts
    pub forecasts: HashMap<String, TrendForecast>,
    
    /// Completion date estimate
    pub completion_date_estimate: Option<DateTime<Local>>,
    
    /// Completion date confidence interval (lower, upper)
    pub completion_date_confidence: Option<(DateTime<Local>, DateTime<Local>)>,
    
    /// Velocity (percentage points per week)
    pub velocity: f32,
    
    /// Acceleration (percentage points per week^2)
    pub acceleration: f32,
    
    /// Seasonality detected
    pub seasonality_detected: bool,
    
    /// Anomalies detected
    pub anomalies: Vec<(DateTime<Local>, String, f32)>,
}

impl StatisticalTrendAnalyzer {
    /// Create a new statistical trend analyzer
    pub fn new(base_dir: PathBuf) -> Self {
        Self {
            base_dir: base_dir.clone(),
            trend_analyzer: TrendAnalyzer::new(base_dir),
        }
    }
    
    /// Analyze trends
    pub fn analyze_trends(&self) -> Result<StatisticalTrendMetrics, String> {
        // Load history
        let history = self.trend_analyzer.load_history()?;
        
        // Check if we have enough data
        if history.entries.len() < 3 {
            return Err("Not enough data for statistical analysis. Need at least 3 data points.".to_string());
        }
        
        // Sort entries by timestamp (oldest first)
        let mut entries = history.entries.clone();
        entries.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
        
        // Initialize metrics
        let mut forecasts = HashMap::new();
        let mut anomalies = Vec::new();
        
        // Analyze overall progress
        let overall_progress_forecast = self.forecast_metric(&entries, "overall_progress", |entry| entry.overall_progress)?;
        forecasts.insert("overall_progress".to_string(), overall_progress_forecast);
        
        // Analyze models progress
        let models_forecast = self.forecast_metric(&entries, "models_percentage", |entry| entry.models_percentage)?;
        forecasts.insert("models_percentage".to_string(), models_forecast);
        
        // Analyze API endpoints progress
        let api_forecast = self.forecast_metric(&entries, "api_endpoints_percentage", |entry| entry.api_endpoints_percentage)?;
        forecasts.insert("api_endpoints_percentage".to_string(), api_forecast);
        
        // Analyze UI components progress
        let ui_forecast = self.forecast_metric(&entries, "ui_components_percentage", |entry| entry.ui_components_percentage)?;
        forecasts.insert("ui_components_percentage".to_string(), ui_forecast);
        
        // Analyze technical debt
        let tech_debt_forecast = self.forecast_metric(&entries, "tech_debt_issues", |entry| entry.tech_debt_issues as f32)?;
        forecasts.insert("tech_debt_issues".to_string(), tech_debt_forecast);
        
        // Calculate velocity and acceleration
        let velocity = self.calculate_velocity(&entries)?;
        let acceleration = self.calculate_acceleration(&entries)?;
        
        // Estimate completion date
        let (completion_date, completion_confidence) = self.estimate_completion_date(&entries, velocity, acceleration)?;
        
        // Check for seasonality
        let seasonality_detected = self.detect_seasonality(&entries)?;
        
        // Detect anomalies
        let anomalies = self.detect_anomalies(&entries)?;
        
        Ok(StatisticalTrendMetrics {
            forecasts,
            completion_date_estimate: completion_date,
            completion_date_confidence: completion_confidence,
            velocity,
            acceleration,
            seasonality_detected,
            anomalies,
        })
    }
    
    /// Forecast a metric
    fn forecast_metric<F>(&self, entries: &[HistoryEntry], metric_name: &str, value_fn: F) -> Result<TrendForecast, String>
    where
        F: Fn(&HistoryEntry) -> f32,
    {
        // Extract values and timestamps
        let values: Vec<f32> = entries.iter().map(|entry| value_fn(entry)).collect();
        let timestamps: Vec<f64> = entries.iter().map(|entry| entry.timestamp.timestamp() as f64).collect();
        
        // Calculate days since first entry
        let first_timestamp = timestamps[0];
        let days_since_first: Vec<f64> = timestamps.iter().map(|ts| (ts - first_timestamp) / (24.0 * 60.0 * 60.0)).collect();
        
        // Perform linear regression
        let regression = SimpleLinearRegression::new(&days_since_first, &values.iter().map(|v| *v as f64).collect::<Vec<f64>>())?;
        
        // Get current value
        let current_value = values.last().unwrap();
        
        // Calculate forecasts
        let last_day = days_since_first.last().unwrap();
        let forecast_1_week = regression.predict(last_day + 7.0) as f32;
        let forecast_1_month = regression.predict(last_day + 30.0) as f32;
        let forecast_3_months = regression.predict(last_day + 90.0) as f32;
        
        // Calculate standard error of the regression
        let mut sum_squared_errors = 0.0;
        for (i, day) in days_since_first.iter().enumerate() {
            let predicted = regression.predict(*day) as f32;
            let actual = values[i];
            sum_squared_errors += (predicted - actual).powi(2);
        }
        let standard_error = (sum_squared_errors / (days_since_first.len() - 2) as f32).sqrt();
        
        // Calculate confidence intervals (95% confidence)
        let t_value = 1.96; // Approximate t-value for 95% confidence
        let confidence_1_week = (
            forecast_1_week - t_value * standard_error,
            forecast_1_week + t_value * standard_error,
        );
        let confidence_1_month = (
            forecast_1_month - t_value * standard_error,
            forecast_1_month + t_value * standard_error,
        );
        let confidence_3_months = (
            forecast_3_months - t_value * standard_error,
            forecast_3_months + t_value * standard_error,
        );
        
        // Calculate growth rate (percentage per week)
        let growth_rate = (forecast_1_week - *current_value) / *current_value * 100.0;
        
        // Determine trend direction
        let trend_direction = if growth_rate > 1.0 {
            TrendDirection::Increasing
        } else if growth_rate < -1.0 {
            TrendDirection::Decreasing
        } else {
            TrendDirection::Stable
        };
        
        // Determine trend strength
        let trend_strength = if growth_rate.abs() > 5.0 {
            TrendStrength::Strong
        } else if growth_rate.abs() > 2.0 {
            TrendStrength::Moderate
        } else {
            TrendStrength::Weak
        };
        
        Ok(TrendForecast {
            metric: metric_name.to_string(),
            current_value: *current_value,
            forecast_1_week,
            forecast_1_month,
            forecast_3_months,
            confidence_1_week,
            confidence_1_month,
            confidence_3_months,
            growth_rate,
            trend_direction,
            trend_strength,
        })
    }
    
    /// Calculate velocity (percentage points per week)
    fn calculate_velocity(&self, entries: &[HistoryEntry]) -> Result<f32, String> {
        if entries.len() < 2 {
            return Err("Not enough data to calculate velocity.".to_string());
        }
        
        // Get the two most recent entries
        let latest = entries.last().unwrap();
        let previous = entries.get(entries.len() - 2).unwrap();
        
        // Calculate time difference in weeks
        let time_diff = (latest.timestamp - previous.timestamp).num_seconds() as f32 / (7.0 * 24.0 * 60.0 * 60.0);
        
        // Calculate progress difference
        let progress_diff = latest.overall_progress - previous.overall_progress;
        
        // Calculate velocity
        let velocity = progress_diff / time_diff;
        
        Ok(velocity)
    }
    
    /// Calculate acceleration (percentage points per week^2)
    fn calculate_acceleration(&self, entries: &[HistoryEntry]) -> Result<f32, String> {
        if entries.len() < 3 {
            return Err("Not enough data to calculate acceleration.".to_string());
        }
        
        // Get the three most recent entries
        let latest = entries.last().unwrap();
        let middle = entries.get(entries.len() - 2).unwrap();
        let earliest = entries.get(entries.len() - 3).unwrap();
        
        // Calculate time differences in weeks
        let time_diff_1 = (latest.timestamp - middle.timestamp).num_seconds() as f32 / (7.0 * 24.0 * 60.0 * 60.0);
        let time_diff_2 = (middle.timestamp - earliest.timestamp).num_seconds() as f32 / (7.0 * 24.0 * 60.0 * 60.0);
        
        // Calculate progress differences
        let progress_diff_1 = latest.overall_progress - middle.overall_progress;
        let progress_diff_2 = middle.overall_progress - earliest.overall_progress;
        
        // Calculate velocities
        let velocity_1 = progress_diff_1 / time_diff_1;
        let velocity_2 = progress_diff_2 / time_diff_2;
        
        // Calculate time difference between velocity measurements
        let time_diff = (time_diff_1 + time_diff_2) / 2.0;
        
        // Calculate acceleration
        let acceleration = (velocity_1 - velocity_2) / time_diff;
        
        Ok(acceleration)
    }
    
    /// Estimate completion date
    fn estimate_completion_date(&self, entries: &[HistoryEntry], velocity: f32, acceleration: f32) -> Result<(Option<DateTime<Local>>, Option<(DateTime<Local>, DateTime<Local>)>), String> {
        if entries.is_empty() {
            return Err("No data to estimate completion date.".to_string());
        }
        
        // Get the latest entry
        let latest = entries.last().unwrap();
        
        // Check if we're already at 100%
        if latest.overall_progress >= 100.0 {
            return Ok((Some(latest.timestamp), Some((latest.timestamp, latest.timestamp))));
        }
        
        // Check if velocity is zero or negative
        if velocity <= 0.0 {
            return Ok((None, None));
        }
        
        // Calculate remaining progress
        let remaining_progress = 100.0 - latest.overall_progress;
        
        // Calculate time to completion in weeks
        let mut time_to_completion = remaining_progress / velocity;
        
        // Adjust for acceleration
        if acceleration != 0.0 {
            // Solve quadratic equation: 0.5 * acceleration * t^2 + velocity * t - remaining_progress = 0
            let a = 0.5 * acceleration;
            let b = velocity;
            let c = -remaining_progress;
            
            let discriminant = b * b - 4.0 * a * c;
            
            if discriminant >= 0.0 {
                // Use the positive solution
                time_to_completion = (-b + discriminant.sqrt()) / (2.0 * a);
                
                // If negative, use the simple estimate
                if time_to_completion < 0.0 {
                    time_to_completion = remaining_progress / velocity;
                }
            }
        }
        
        // Calculate completion date
        let completion_date = latest.timestamp + Duration::seconds((time_to_completion * 7.0 * 24.0 * 60.0 * 60.0) as i64);
        
        // Calculate confidence interval
        let standard_error = self.calculate_standard_error(entries)?;
        let t_value = 1.96; // Approximate t-value for 95% confidence
        
        let lower_bound = time_to_completion - t_value * standard_error;
        let upper_bound = time_to_completion + t_value * standard_error;
        
        let lower_date = latest.timestamp + Duration::seconds((lower_bound * 7.0 * 24.0 * 60.0 * 60.0) as i64);
        let upper_date = latest.timestamp + Duration::seconds((upper_bound * 7.0 * 24.0 * 60.0 * 60.0) as i64);
        
        Ok((Some(completion_date), Some((lower_date, upper_date))))
    }
    
    /// Calculate standard error of progress predictions
    fn calculate_standard_error(&self, entries: &[HistoryEntry]) -> Result<f32, String> {
        if entries.len() < 3 {
            return Err("Not enough data to calculate standard error.".to_string());
        }
        
        // Extract values and timestamps
        let values: Vec<f32> = entries.iter().map(|entry| entry.overall_progress).collect();
        let timestamps: Vec<f64> = entries.iter().map(|entry| entry.timestamp.timestamp() as f64).collect();
        
        // Calculate days since first entry
        let first_timestamp = timestamps[0];
        let days_since_first: Vec<f64> = timestamps.iter().map(|ts| (ts - first_timestamp) / (24.0 * 60.0 * 60.0)).collect();
        
        // Perform linear regression
        let regression = SimpleLinearRegression::new(&days_since_first, &values.iter().map(|v| *v as f64).collect::<Vec<f64>>())?;
        
        // Calculate standard error of the regression
        let mut sum_squared_errors = 0.0;
        for (i, day) in days_since_first.iter().enumerate() {
            let predicted = regression.predict(*day) as f32;
            let actual = values[i];
            sum_squared_errors += (predicted - actual).powi(2);
        }
        let standard_error = (sum_squared_errors / (days_since_first.len() - 2) as f32).sqrt();
        
        Ok(standard_error)
    }
    
    /// Detect seasonality in the data
    fn detect_seasonality(&self, entries: &[HistoryEntry]) -> Result<bool, String> {
        if entries.len() < 7 {
            return Ok(false);
        }
        
        // Extract overall progress values
        let values: Vec<f32> = entries.iter().map(|entry| entry.overall_progress).collect();
        
        // Calculate autocorrelation for different lags
        let max_lag = entries.len() / 3;
        let mut autocorrelations = Vec::new();
        
        for lag in 1..=max_lag {
            let mut sum_product = 0.0;
            let mut sum_squared = 0.0;
            
            for i in 0..(entries.len() - lag) {
                sum_product += values[i] * values[i + lag];
                sum_squared += values[i].powi(2);
            }
            
            let autocorrelation = sum_product / sum_squared;
            autocorrelations.push(autocorrelation);
        }
        
        // Check for significant autocorrelation at weekly intervals
        let weekly_lag = 7;
        if weekly_lag < autocorrelations.len() && autocorrelations[weekly_lag - 1] > 0.5 {
            return Ok(true);
        }
        
        Ok(false)
    }
    
    /// Detect anomalies in the data
    fn detect_anomalies(&self, entries: &[HistoryEntry]) -> Result<Vec<(DateTime<Local>, String, f32)>, String> {
        if entries.len() < 5 {
            return Ok(Vec::new());
        }
        
        let mut anomalies = Vec::new();
        
        // Detect anomalies in overall progress
        self.detect_metric_anomalies(entries, "overall_progress", |entry| entry.overall_progress, &mut anomalies)?;
        
        // Detect anomalies in models progress
        self.detect_metric_anomalies(entries, "models_percentage", |entry| entry.models_percentage, &mut anomalies)?;
        
        // Detect anomalies in API endpoints progress
        self.detect_metric_anomalies(entries, "api_endpoints_percentage", |entry| entry.api_endpoints_percentage, &mut anomalies)?;
        
        // Detect anomalies in UI components progress
        self.detect_metric_anomalies(entries, "ui_components_percentage", |entry| entry.ui_components_percentage, &mut anomalies)?;
        
        // Detect anomalies in technical debt
        self.detect_metric_anomalies(entries, "tech_debt_issues", |entry| entry.tech_debt_issues as f32, &mut anomalies)?;
        
        Ok(anomalies)
    }
    
    /// Detect anomalies in a specific metric
    fn detect_metric_anomalies<F>(&self, entries: &[HistoryEntry], metric_name: &str, value_fn: F, anomalies: &mut Vec<(DateTime<Local>, String, f32)>) -> Result<(), String>
    where
        F: Fn(&HistoryEntry) -> f32,
    {
        // Extract values
        let values: Vec<f32> = entries.iter().map(|entry| value_fn(entry)).collect();
        
        // Calculate mean and standard deviation
        let mean = values.iter().sum::<f32>() / values.len() as f32;
        let variance = values.iter().map(|v| (v - mean).powi(2)).sum::<f32>() / values.len() as f32;
        let std_dev = variance.sqrt();
        
        // Check for anomalies (values more than 2 standard deviations from the mean)
        for (i, value) in values.iter().enumerate() {
            let z_score = (value - mean) / std_dev;
            
            if z_score.abs() > 2.0 {
                anomalies.push((entries[i].timestamp, metric_name.to_string(), *value));
            }
        }
        
        Ok(())
    }
    
    /// Generate a statistical trend report
    pub fn generate_report(&self) -> Result<String, String> {
        // Analyze trends
        let metrics = self.analyze_trends()?;
        
        // Generate the report
        let mut report = String::new();
        
        // Header
        report.push_str("# Statistical Trend Analysis Report\n\n");
        report.push_str(&format!("_Generated on: {}_\n\n", Local::now().format("%Y-%m-%d")));
        
        // Velocity and Acceleration
        report.push_str("## Project Velocity\n\n");
        report.push_str(&format!("- **Current Velocity**: {:.2} percentage points per week\n", metrics.velocity));
        report.push_str(&format!("- **Acceleration**: {:.2} percentage points per week²\n", metrics.acceleration));
        
        let velocity_description = if metrics.velocity > 5.0 {
            "The project is progressing at a rapid pace."
        } else if metrics.velocity > 2.0 {
            "The project is progressing at a moderate pace."
        } else if metrics.velocity > 0.0 {
            "The project is progressing at a slow pace."
        } else {
            "The project is not making progress or is regressing."
        };
        
        let acceleration_description = if metrics.acceleration > 0.5 {
            "The project is accelerating significantly."
        } else if metrics.acceleration > 0.1 {
            "The project is accelerating moderately."
        } else if metrics.acceleration > -0.1 {
            "The project velocity is relatively stable."
        } else if metrics.acceleration > -0.5 {
            "The project is decelerating moderately."
        } else {
            "The project is decelerating significantly."
        };
        
        report.push_str(&format!("\n{} {}\n\n", velocity_description, acceleration_description));
        
        // Estimated Completion Date
        report.push_str("## Estimated Completion Date\n\n");
        
        if let Some(completion_date) = metrics.completion_date_estimate {
            report.push_str(&format!("**Estimated Completion Date**: {}\n", completion_date.format("%Y-%m-%d")));
            
            if let Some((lower_date, upper_date)) = metrics.completion_date_confidence {
                report.push_str(&format!("**95% Confidence Interval**: {} to {}\n", lower_date.format("%Y-%m-%d"), upper_date.format("%Y-%m-%d")));
            }
            
            // Calculate days until completion
            let days_until_completion = (completion_date - Local::now()).num_days();
            
            report.push_str(&format!("\nAt the current pace, the project will be completed in approximately {} days.\n", days_until_completion));
        } else {
            report.push_str("**Estimated Completion Date**: Unable to estimate completion date due to insufficient progress.\n");
        }
        
        report.push_str("\n");
        
        // Metric Forecasts
        report.push_str("## Metric Forecasts\n\n");
        
        for (metric_name, forecast) in &metrics.forecasts {
            let formatted_metric = match metric_name.as_str() {
                "overall_progress" => "Overall Progress",
                "models_percentage" => "Models Implementation",
                "api_endpoints_percentage" => "API Endpoints Implementation",
                "ui_components_percentage" => "UI Components Implementation",
                "tech_debt_issues" => "Technical Debt Issues",
                _ => metric_name,
            };
            
            let trend_direction = match forecast.trend_direction {
                TrendDirection::Increasing => "Increasing",
                TrendDirection::Decreasing => "Decreasing",
                TrendDirection::Stable => "Stable",
            };
            
            let trend_strength = match forecast.trend_strength {
                TrendStrength::Strong => "Strong",
                TrendStrength::Moderate => "Moderate",
                TrendStrength::Weak => "Weak",
            };
            
            report.push_str(&format!("### {}\n\n", formatted_metric));
            report.push_str(&format!("- **Current Value**: {:.2}\n", forecast.current_value));
            report.push_str(&format!("- **Trend**: {} ({})\n", trend_direction, trend_strength));
            report.push_str(&format!("- **Growth Rate**: {:.2}% per week\n", forecast.growth_rate));
            report.push_str(&format!("- **Forecast (1 week)**: {:.2} (95% CI: {:.2} to {:.2})\n", 
                forecast.forecast_1_week,
                forecast.confidence_1_week.0,
                forecast.confidence_1_week.1));
            report.push_str(&format!("- **Forecast (1 month)**: {:.2} (95% CI: {:.2} to {:.2})\n", 
                forecast.forecast_1_month,
                forecast.confidence_1_month.0,
                forecast.confidence_1_month.1));
            report.push_str(&format!("- **Forecast (3 months)**: {:.2} (95% CI: {:.2} to {:.2})\n", 
                forecast.forecast_3_months,
                forecast.confidence_3_months.0,
                forecast.confidence_3_months.1));
            
            report.push_str("\n");
        }
        
        // Seasonality
        report.push_str("## Seasonality Analysis\n\n");
        
        if metrics.seasonality_detected {
            report.push_str("**Seasonality Detected**: Yes\n\n");
            report.push_str("The project shows signs of weekly seasonality in progress. This may indicate regular sprint cycles or weekly development patterns.\n\n");
        } else {
            report.push_str("**Seasonality Detected**: No\n\n");
            report.push_str("No significant seasonality detected in the project progress data.\n\n");
        }
        
        // Anomalies
        report.push_str("## Anomaly Detection\n\n");
        
        if metrics.anomalies.is_empty() {
            report.push_str("No significant anomalies detected in the project data.\n\n");
        } else {
            report.push_str("The following anomalies were detected in the project data:\n\n");
            report.push_str("| Date | Metric | Value |\n");
            report.push_str("|------|--------|-------|\n");
            
            for (timestamp, metric, value) in &metrics.anomalies {
                let formatted_metric = match metric.as_str() {
                    "overall_progress" => "Overall Progress",
                    "models_percentage" => "Models Implementation",
                    "api_endpoints_percentage" => "API Endpoints Implementation",
                    "ui_components_percentage" => "UI Components Implementation",
                    "tech_debt_issues" => "Technical Debt Issues",
                    _ => metric,
                };
                
                report.push_str(&format!("| {} | {} | {:.2} |\n",
                    timestamp.format("%Y-%m-%d"),
                    formatted_metric,
                    value));
            }
            
            report.push_str("\nThese anomalies may indicate significant events or changes in the project.\n\n");
        }
        
        // Recommendations
        report.push_str("## Recommendations\n\n");
        
        // Generate recommendations based on the analysis
        let mut recommendations = Vec::new();
        
        // Velocity recommendations
        if metrics.velocity < 1.0 {
            recommendations.push("Increase development velocity by focusing on high-impact features and reducing overhead.".to_string());
        } else if metrics.velocity > 10.0 {
            recommendations.push("Maintain the current high velocity, but ensure quality is not being sacrificed for speed.".to_string());
        }
        
        // Acceleration recommendations
        if metrics.acceleration < -0.5 {
            recommendations.push("Address the decreasing velocity trend by identifying and removing bottlenecks.".to_string());
        } else if metrics.acceleration > 0.5 {
            recommendations.push("Capitalize on the increasing velocity trend by continuing current practices.".to_string());
        }
        
        // Completion date recommendations
        if let Some(completion_date) = metrics.completion_date_estimate {
            let days_until_completion = (completion_date - Local::now()).num_days();
            
            if days_until_completion > 180 {
                recommendations.push("Consider reducing project scope or increasing resources to meet earlier deadlines.".to_string());
            } else if days_until_completion < 30 {
                recommendations.push("Focus on quality assurance and testing as the project approaches completion.".to_string());
            }
        }
        
        // Technical debt recommendations
        if let Some(tech_debt_forecast) = metrics.forecasts.get("tech_debt_issues") {
            if tech_debt_forecast.trend_direction == TrendDirection::Increasing && tech_debt_forecast.trend_strength != TrendStrength::Weak {
                recommendations.push("Address the increasing technical debt trend by allocating time for debt reduction.".to_string());
            }
        }
        
        // Add general recommendations
        recommendations.push("Continue collecting project metrics to improve forecast accuracy.".to_string());
        recommendations.push("Use the forecasts to adjust project planning and resource allocation.".to_string());
        
        for recommendation in recommendations {
            report.push_str(&format!("- {}\n", recommendation));
        }
        
        Ok(report)
    }
}
