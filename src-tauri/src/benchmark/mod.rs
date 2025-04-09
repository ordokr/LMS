use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};
use tokio::sync::mpsc;
use sqlx::{Pool, Sqlite};
use serde::{Serialize, Deserialize};
use log::{info, warn};
use std::sync::Arc;
use std::collections::HashMap;
use tokio::time::sleep;

// Benchmark configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkConfig {
    pub duration_seconds: u64,
    pub users: usize,
    pub ramp_up_seconds: u64,
    pub scenarios: Vec<Scenario>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scenario {
    pub name: String,
    pub weight: usize,
    pub actions: Vec<Action>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Action {
    LoadHomepage,
    LoadCategory { id: i64 },
    LoadTopic { id: i64 },
    CreateTopic { category_id: i64 },
    CreatePost { topic_id: i64 },
    Search { query: String },
}

// Benchmark results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkResults {
    pub total_requests: usize,
    pub successful_requests: usize,
    pub failed_requests: usize,
    pub duration_ms: u64,
    pub requests_per_second: f64,
    pub latencies: LatencyStats,
    pub results_by_action: HashMap<String, ActionStats>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LatencyStats {
    pub min_ms: u64,
    pub max_ms: u64,
    pub avg_ms: u64,
    pub percentile_50ms: u64,
    pub percentile_90ms: u64,
    pub percentile_95ms: u64,
    pub percentile_99ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionStats {
    pub count: usize,
    pub success_count: usize,
    pub failure_count: usize,
    pub avg_latency_ms: u64,
}

// Results collector
struct ResultsCollector {
    total_requests: AtomicUsize,
    successful_requests: AtomicUsize,
    failed_requests: AtomicUsize,
    latencies: std::sync::Mutex<Vec<u64>>,
    action_results: std::sync::Mutex<HashMap<String, Vec<u64>>>,
    action_failures: std::sync::Mutex<HashMap<String, usize>>,
}

// Benchmark runner
pub struct BenchmarkRunner {
    config: BenchmarkConfig,
    pool: Arc<Pool<Sqlite>>,
    collector: Arc<ResultsCollector>,
}

impl BenchmarkRunner {
    pub fn new(config: BenchmarkConfig, pool: Arc<Pool<Sqlite>>) -> Self {
        Self {
            config,
            pool,
            collector: Arc::new(ResultsCollector {
                total_requests: AtomicUsize::new(0),
                successful_requests: AtomicUsize::new(0),
                failed_requests: AtomicUsize::new(0),
                latencies: std::sync::Mutex::new(Vec::new()),
                action_results: std::sync::Mutex::new(HashMap::new()),
                action_failures: std::sync::Mutex::new(HashMap::new()),
            }),
        }
    }
    
    // Run the benchmark with a configurable number of concurrent users
    pub async fn run(&self) -> BenchmarkResults {
        info!("Starting benchmark with {} users for {} seconds", 
            self.config.users, self.config.duration_seconds);
            
        // Channel for sending completion signals
        let (tx, mut rx) = mpsc::channel::<()>(self.config.users);
        let start_time = Instant::now();
        
        // Spawn user simulation tasks
        for i in 0..self.config.users {
            let tx = tx.clone();
            let collector = self.collector.clone();
            let pool = self.pool.clone();
            let scenarios = self.config.scenarios.clone();
            let duration = Duration::from_secs(self.config.duration_seconds);
            let ramp_up = self.config.ramp_up_seconds;
            
            tokio::spawn(async move {
                // Ramp up gradually
                if ramp_up > 0 {
                    let delay = (ramp_up as f64 / self.config.users as f64) * i as f64;
                    sleep(Duration::from_secs_f64(delay)).await;
                }
                
                let end_time = Instant::now() + duration;
                let mut rng = rand::thread_rng();
                
                // Scenario selection based on weights
                let total_weight: usize = scenarios.iter().map(|s| s.weight).sum();
                
                // Run user simulation until time is up
                while Instant::now() < end_time {
                    // Select scenario based on weight
                    let rand_val = rand::Rng::gen_range(&mut rng, 0..total_weight);
                    let mut current_weight = 0;
                    let mut selected_scenario = &scenarios[0];
                    
                    for scenario in &scenarios {
                        current_weight += scenario.weight;
                        if rand_val < current_weight {
                            selected_scenario = scenario;
                            break;
                        }
                    }
                    
                    // Execute actions in the scenario
                    for action in &selected_scenario.actions {
                        let action_name = format!("{:?}", action);
                        let start = Instant::now();
                        let result = Self::execute_action(&pool, action).await;
                        let latency_ms = start.elapsed().as_millis() as u64;
                        
                        collector.total_requests.fetch_add(1, Ordering::Relaxed);
                        collector.latencies.lock().unwrap().push(latency_ms);
                        
                        // Record action-specific stats
                        let mut action_results = collector.action_results.lock().unwrap();
                        let latencies = action_results.entry(action_name.clone()).or_insert_with(Vec::new);
                        latencies.push(latency_ms);
                        
                        if result.is_ok() {
                            collector.successful_requests.fetch_add(1, Ordering::Relaxed);
                        } else {
                            collector.failed_requests.fetch_add(1, Ordering::Relaxed);
                            let mut failures = collector.action_failures.lock().unwrap();
                            *failures.entry(action_name).or_insert(0) += 1;
                        }
                    }
                    
                    // Small delay between scenario iterations to simulate user think time
                    sleep(Duration::from_millis(rand::Rng::gen_range(&mut rng, 500..2000))).await;
                }
                
                // Signal completion
                let _ = tx.send(()).await;
            });
        }
        
        // Wait for all users to complete
        drop(tx); // Close sender
        
        while rx.recv().await.is_some() {
            // Process completions
        }
        
        // Calculate final results
        let duration_ms = start_time.elapsed().as_millis() as u64;
        let total_requests = self.collector.total_requests.load(Ordering::Relaxed);
        let successful_requests = self.collector.successful_requests.load(Ordering::Relaxed);
        let failed_requests = self.collector.failed_requests.load(Ordering::Relaxed);
        
        let latencies = self.collector.latencies.lock().unwrap();
        let mut latencies_sorted = latencies.clone();
        latencies_sorted.sort();
        
        let min_ms = latencies_sorted.first().cloned().unwrap_or(0);
        let max_ms = latencies_sorted.last().cloned().unwrap_or(0);
        let avg_ms = if latencies.is_empty() {
            0
        } else {
            latencies.iter().sum::<u64>() / latencies.len() as u64
        };
        
        // Calculate percentiles
        let percentile_50ms = Self::percentile(&latencies_sorted, 50);
        let percentile_90ms = Self::percentile(&latencies_sorted, 90);
        let percentile_95ms = Self::percentile(&latencies_sorted, 95);
        let percentile_99ms = Self::percentile(&latencies_sorted, 99);
        
        // Calculate per-action stats
        let action_results = self.collector.action_results.lock().unwrap();
        let action_failures = self.collector.action_failures.lock().unwrap();
        
        let mut results_by_action = HashMap::new();
        
        for (action, latencies) in action_results.iter() {
            let success_count = latencies.len();
            let failure_count = action_failures.get(action).cloned().unwrap_or(0);
            let count = success_count + failure_count;
            
            let avg_latency_ms = if success_count > 0 {
                latencies.iter().sum::<u64>() / success_count as u64
            } else {
                0
            };
            
            results_by_action.insert(action.clone(), ActionStats {
                count,
                success_count,
                failure_count,
                avg_latency_ms,
            });
        }
        
        BenchmarkResults {
            total_requests,
            successful_requests,
            failed_requests,
            duration_ms,
            requests_per_second: total_requests as f64 / (duration_ms as f64 / 1000.0),
            latencies: LatencyStats {
                min_ms,
                max_ms,
                avg_ms,
                percentile_50ms,
                percentile_90ms,
                percentile_95ms,
                percentile_99ms,
            },
            results_by_action,
        }
    }
    
    // Execute a single action in the benchmark
    async fn execute_action(pool: &Pool<Sqlite>, action: &Action) -> Result<(), String> {
        match action {
            Action::LoadHomepage => {
                // Simulate homepage load
                sqlx::query("SELECT id, name FROM categories LIMIT 10")
                    .fetch_all(pool)
                    .await
                    .map_err(|e| e.to_string())?;
                
                sqlx::query("SELECT id, title, created_at FROM topics ORDER BY created_at DESC LIMIT 10")
                    .fetch_all(pool)
                    .await
                    .map_err(|e| e.to_string())?;
                
                Ok(())
            },
            Action::LoadCategory { id } => {
                // Simulate category page load
                sqlx::query("SELECT * FROM categories WHERE id = ?")
                    .bind(id)
                    .fetch_optional(pool)
                    .await
                    .map_err(|e| e.to_string())?;
                
                sqlx::query("SELECT * FROM topics WHERE category_id = ? ORDER BY created_at DESC LIMIT 20")
                    .bind(id)
                    .fetch_all(pool)
                    .await
                    .map_err(|e| e.to_string())?;
                
                Ok(())
            },
            Action::LoadTopic { id } => {
                // Simulate topic page load
                sqlx::query("SELECT * FROM topics WHERE id = ?")
                    .bind(id)
                    .fetch_optional(pool)
                    .await
                    .map_err(|e| e.to_string())?;
                
                sqlx::query("SELECT * FROM posts WHERE topic_id = ? ORDER BY created_at ASC LIMIT 30")
                    .bind(id)
                    .fetch_all(pool)
                    .await
                    .map_err(|e| e.to_string())?;
                
                Ok(())
            },
            Action::CreateTopic { category_id } => {
                // Simulate topic creation
                let title = format!("Benchmark Topic {}", rand::random::<u64>());
                let content = "This is a benchmark topic content.".repeat(5);
                
                sqlx::query(
                    "INSERT INTO topics (title, content, category_id, user_id, created_at, updated_at) 
                     VALUES (?, ?, ?, 1, datetime('now'), datetime('now'))"
                )
                .bind(title)
                .bind(content)
                .bind(category_id)
                .execute(pool)
                .await
                .map_err(|e| e.to_string())?;
                
                Ok(())
            },
            Action::CreatePost { topic_id } => {
                // Simulate post creation
                let content = "This is a benchmark reply.".repeat(3);
                
                sqlx::query(
                    "INSERT INTO posts (content, topic_id, user_id, created_at, updated_at) 
                     VALUES (?, ?, 1, datetime('now'), datetime('now'))"
                )
                .bind(content)
                .bind(topic_id)
                .execute(pool)
                .await
                .map_err(|e| e.to_string())?;
                
                // Update topic updated_at timestamp
                sqlx::query("UPDATE topics SET updated_at = datetime('now') WHERE id = ?")
                    .bind(topic_id)
                    .execute(pool)
                    .await
                    .map_err(|e| e.to_string())?;
                
                Ok(())
            },
            Action::Search { query } => {
                // Simulate search
                sqlx::query("SELECT id, title FROM topics WHERE title LIKE ? OR content LIKE ? LIMIT 20")
                    .bind(format!("%{}%", query))
                    .bind(format!("%{}%", query))
                    .fetch_all(pool)
                    .await
                    .map_err(|e| e.to_string())?;
                
                Ok(())
            },
        }
    }
    
    // Calculate percentile from a sorted array
    fn percentile(sorted_values: &[u64], percentile: usize) -> u64 {
        if sorted_values.is_empty() {
            return 0;
        }
        
        let len = sorted_values.len();
        let index = (len * percentile) / 100;
        if index >= len {
            sorted_values[len - 1]
        } else {
            sorted_values[index]
        }
    }
}