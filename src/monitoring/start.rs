use std::env;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;
use tokio::signal;
use warp::{Filter, Reply};
use serde_json::json;
use chrono::Utc;

use crate::utils::logger::create_logger;
use crate::monitoring::metrics::{MetricsCollector, create_metrics_collector};
use crate::monitoring::performance::{PerformanceMonitor, create_performance_monitor};

/// Start the monitoring service
pub async fn start_monitoring_service() -> Result<(), Box<dyn std::error::Error>> {
    let logger = create_logger("monitoring-service");
    let metrics = create_metrics_collector();
    let performance_monitor = create_performance_monitor();
    
    // Get port from environment variable or use default
    let port: u16 = env::var("MONITORING_PORT")
        .unwrap_or_else(|_| "3001".to_string())
        .parse()
        .unwrap_or(3001);
    
    // Define API routes
    let metrics_route = warp::path("metrics")
        .and(warp::get())
        .and(with_metrics(metrics.clone()))
        .and_then(metrics_handler);
    
    let health_route = warp::path("health")
        .and(warp::get())
        .map(|| {
            let response = json!({
                "status": "ok",
                "timestamp": Utc::now().to_rfc3339()
            });
            warp::reply::json(&response)
        });
    
    let routes = metrics_route
        .or(health_route)
        .with(warp::log("monitoring"));
    
    // Get address
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port);
    
    // Initialize metrics
    if let Ok(mut m) = metrics.lock() {
        m.gauge("system.start_time", Utc::now().timestamp_millis() as f64);
    }
    
    slog::info!(logger, "Starting monitoring server on port {}", port);
    slog::info!(logger, "Available endpoints:");
    slog::info!(logger, "- Health check: http://localhost:{}/health", port);
    slog::info!(logger, "- Metrics: http://localhost:{}/metrics", port);
    
    // Start server
    let (_, server) = warp::serve(routes)
        .bind_with_graceful_shutdown(addr, async {
            signal::ctrl_c().await.expect("Failed to listen for ctrl+c");
            slog::info!(logger, "Shutdown signal received, stopping monitoring server");
        });
        
    slog::info!(logger, "Starting system monitoring");
    
    // Run the server
    server.await;
    
    slog::info!(logger, "Monitoring server stopped");
    
    Ok(())
}

/// Pass metrics to handlers
fn with_metrics(
    metrics: Arc<Mutex<MetricsCollector>>,
) -> impl Filter<Extract = (Arc<Mutex<MetricsCollector>>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || metrics.clone())
}

/// Handle metrics requests
async fn metrics_handler(metrics: Arc<Mutex<MetricsCollector>>) -> Result<impl Reply, warp::Rejection> {
    if let Ok(m) = metrics.lock() {
        let all_metrics = m.get_all_metrics();
        Ok(warp::reply::json(&all_metrics))
    } else {
        let error_response = json!({
            "error": "Failed to acquire metrics lock"
        });
        Ok(warp::reply::json(&error_response))
    }
}

/// Monitoring service configuration
pub struct MonitoringService {
    metrics: Arc<Mutex<MetricsCollector>>,
    performance_monitor: PerformanceMonitor,
    port: u16,
}

impl MonitoringService {
    /// Create a new monitoring service
    pub fn new(port: Option<u16>) -> Self {
        let metrics = create_metrics_collector();
        let performance_monitor = create_performance_monitor();
        let port = port.unwrap_or_else(|| {
            env::var("MONITORING_PORT")
                .unwrap_or_else(|_| "3001".to_string())
                .parse()
                .unwrap_or(3001)
        });
        
        Self {
            metrics,
            performance_monitor,
            port,
        }
    }
    
    /// Get the metrics collector
    pub fn get_metrics(&self) -> Arc<Mutex<MetricsCollector>> {
        self.metrics.clone()
    }
    
    /// Get the performance monitor
    pub fn get_performance_monitor(&self) -> &PerformanceMonitor {
        &self.performance_monitor
    }
    
    /// Start the monitoring service
    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        start_monitoring_service().await
    }
}
