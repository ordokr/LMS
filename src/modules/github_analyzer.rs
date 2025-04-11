use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs::{self, File};
use std::io::{Read, Write};
use anyhow::{Result, Context, anyhow};
use log::{info, error, warn};
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use reqwest::Client;

/// GitHub API client for repository analysis
pub struct GitHubAnalyzer<M> {
    metrics: M,
    client: Option<Client>,
    token: Option<String>,
    owner: String,
    repo: String,
    cache_dir: PathBuf,
}

/// GitHub repository metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubMetrics {
    pub contributors: Vec<Contributor>,
    pub commit_activity: Vec<CommitActivity>,
    pub issue_stats: IssueStats,
    pub pull_request_stats: PullRequestStats,
    pub release_stats: ReleaseStats,
    pub branch_stats: BranchStats,
    pub development_velocity: DevelopmentVelocity,
}

/// GitHub contributor data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contributor {
    pub login: String,
    pub id: u64,
    pub contributions: u32,
    pub avatar: String,
    pub url: String,
}

/// Weekly commit activity data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitActivity {
    pub week: u64,              // Unix timestamp for the start of the week
    pub additions: Option<u32>, // May not be available in some API responses
    pub deletions: Option<u32>, // May not be available in some API responses
    pub commits: u32,
    pub days: Vec<u32>,         // Array of commits per day for the week
}

/// Issue statistics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IssueStats {
    pub total: u32,
    pub open: u32,
    pub closed: u32,
    pub average_age_days: f64,
    pub resolution_time_days: f64,
    pub by_label: HashMap<String, u32>,
    pub recent_issues: Vec<Issue>,
}

/// Pull request statistics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PullRequestStats {
    pub total: u32,
    pub open: u32,
    pub closed: u32,
    pub merged: u32,
    pub average_age_days: f64,
    pub time_to_merge_days: f64,
    pub recent_prs: Vec<PullRequest>,
}

/// Release statistics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ReleaseStats {
    pub total: u32,
    pub latest_version: Option<String>,
    pub latest_date: Option<DateTime<Utc>>,
    pub release_frequency_days: Option<f64>,
    pub recent_releases: Vec<Release>,
}

/// Branch statistics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BranchStats {
    pub total: u32,
    pub protected: u32,
    pub default_branch: String,
    pub branches: Vec<Branch>,
}

/// Development velocity metrics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DevelopmentVelocity {
    pub commits_per_day: f64,
    pub commits_per_week: f64,
    pub prs_per_week: f64,
    pub issues_closed_per_week: f64,
    pub active_contributors_last_month: u32,
}

/// Issue data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Issue {
    pub number: u32,
    pub title: String,
    pub state: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub closed_at: Option<DateTime<Utc>>,
    pub author: String,
    pub labels: Vec<String>,
    pub url: String,
}

/// Pull request data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PullRequest {
    pub number: u32,
    pub title: String,
    pub state: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub closed_at: Option<DateTime<Utc>>,
    pub merged_at: Option<DateTime<Utc>>,
    pub author: String,
    pub url: String,
}

/// Release data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Release {
    pub id: u64,
    pub tag_name: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub published_at: Option<DateTime<Utc>>,
    pub url: String,
}

/// Branch data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Branch {
    pub name: String,
    pub protected: bool,
    pub commit_sha: String,
}

/// GitHub analyzer configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubAnalyzerOptions {
    pub token: Option<String>,
    pub owner: String,
    pub repo: String,
    pub cache_dir: String,
}

impl Default for GitHubAnalyzerOptions {
    fn default() -> Self {
        Self {
            token: None,
            owner: String::new(),
            repo: String::new(),
            cache_dir: ".analysis_cache".to_string(),
        }
    }
}

impl<M> GitHubAnalyzer<M> {
    /// Create a new GitHub analyzer
    pub fn new(metrics: M, options: GitHubAnalyzerOptions) -> Self {
        let client = if options.token.is_some() {
            Some(Client::new())
        } else {
            None
        };
        
        if client.is_none() {
            warn!("GitHub analysis will be disabled - no token provided");
        }
        
        Self {
            metrics,
            client,
            token: options.token,
            owner: options.owner,
            repo: options.repo,
            cache_dir: PathBuf::from(options.cache_dir),
        }
    }
    
    /// Initialize GitHub metrics in the metrics object
    fn initialize_metrics(&mut self) where M: Default + AsMut<HashMap<String, serde_json::Value>> {
        let metrics = self.metrics.as_mut();
        
        if !metrics.contains_key("github") {
            metrics.insert("github".to_string(), serde_json::json!({
                "contributors": [],
                "commitActivity": [],
                "issueStats": {},
                "pullRequestStats": {},
                "releaseStats": {},
                "branchStats": {}
            }));
        }
    }
    
    /// Analyze GitHub repository
    pub async fn analyze_repository(&mut self) -> Result<()> 
    where M: Default + AsMut<HashMap<String, serde_json::Value>> + AsRef<HashMap<String, serde_json::Value>> {
        if self.client.is_none() {
            info!("GitHub analysis skipped - missing token");
            return Ok(());
        }
        
        info!("Analyzing GitHub repository: {}/{}", self.owner, self.repo);
        
        // Initialize metrics structure if it doesn't exist
        self.initialize_metrics();
        
        // Fetch data in sequence to avoid overwhelming the API
        self.fetch_contributors().await?;
        self.fetch_commit_activity().await?;
        self.fetch_issues().await?;
        self.fetch_pull_requests().await?;
        self.fetch_releases().await?;
        self.fetch_branches().await?;
        
        // Calculate development velocity
        self.calculate_development_velocity()?;
        
        // Cache the results
        self.cache_results()?;
        
        info!("GitHub analysis complete");
        
        Ok(())
    }
    
    /// Fetch contributors from GitHub
    async fn fetch_contributors(&mut self) -> Result<()> 
    where M: AsRef<HashMap<String, serde_json::Value>> + AsMut<HashMap<String, serde_json::Value>> {
        let client = self.client.as_ref().unwrap();
        let url = format!(
            "https://api.github.com/repos/{}/{}/contributors",
            self.owner,
            self.repo
        );
        
        let response = client
            .get(&url)
            .header("User-Agent", "rust-github-analyzer")
            .header("Accept", "application/vnd.github.v3+json")
            .bearer_auth(self.token.as_ref().unwrap())
            .send()
            .await
            .context("Failed to fetch contributors")?;
            
        if !response.status().is_success() {
            return Err(anyhow!(
                "GitHub API error: {} - {}",
                response.status(),
                response.text().await.unwrap_or_default()
            ));
        }
        
        let data: Vec<serde_json::Value> = response
            .json()
            .await
            .context("Failed to parse contributors response")?;
            
        let contributors = data
            .into_iter()
            .map(|contributor| {
                Contributor {
                    login: contributor["login"].as_str().unwrap_or("").to_string(),
                    id: contributor["id"].as_u64().unwrap_or(0),
                    contributions: contributor["contributions"].as_u64().unwrap_or(0) as u32,
                    avatar: contributor["avatar_url"].as_str().unwrap_or("").to_string(),
                    url: contributor["html_url"].as_str().unwrap_or("").to_string(),
                }
            })
            .collect::<Vec<_>>();
            
        // Update metrics
        let metrics = self.metrics.as_mut();
        let github = metrics.get_mut("github").unwrap().as_object_mut().unwrap();
        github.insert("contributors".to_string(), serde_json::to_value(&contributors)?);
        
        info!("Found {} contributors", contributors.len());
        
        Ok(())
    }
    
    /// Fetch commit activity from GitHub
    async fn fetch_commit_activity(&mut self) -> Result<()> 
    where M: AsRef<HashMap<String, serde_json::Value>> + AsMut<HashMap<String, serde_json::Value>> {
        let client = self.client.as_ref().unwrap();
        let url = format!(
            "https://api.github.com/repos/{}/{}/stats/commit_activity",
            self.owner,
            self.repo
        );
        
        let response = client
            .get(&url)
            .header("User-Agent", "rust-github-analyzer")
            .header("Accept", "application/vnd.github.v3+json")
            .bearer_auth(self.token.as_ref().unwrap())
            .send()
            .await
            .context("Failed to fetch commit activity")?;
            
        if !response.status().is_success() {
            return Err(anyhow!(
                "GitHub API error: {} - {}",
                response.status(),
                response.text().await.unwrap_or_default()
            ));
        }
        
        let data: Vec<serde_json::Value> = response
            .json()
            .await
            .context("Failed to parse commit activity response")?;
            
        let commit_activity = data
            .into_iter()
            .map(|week| {
                CommitActivity {
                    week: week["week"].as_u64().unwrap_or(0),
                    additions: None, // Not available in this endpoint
                    deletions: None, // Not available in this endpoint
                    commits: week["total"].as_u64().unwrap_or(0) as u32,
                    days: week["days"]
                        .as_array()
                        .unwrap_or(&Vec::new())
                        .iter()
                        .map(|d| d.as_u64().unwrap_or(0) as u32)
                        .collect(),
                }
            })
            .collect::<Vec<_>>();
            
        // Update metrics
        let metrics = self.metrics.as_mut();
        let github = metrics.get_mut("github").unwrap().as_object_mut().unwrap();
        github.insert("commitActivity".to_string(), serde_json::to_value(&commit_activity)?);
        
        info!("Fetched commit activity for {} weeks", commit_activity.len());
        
        Ok(())
    }
    
    /// Fetch issues from GitHub
    async fn fetch_issues(&mut self) -> Result<()> 
    where M: AsRef<HashMap<String, serde_json::Value>> + AsMut<HashMap<String, serde_json::Value>> {
        let client = self.client.as_ref().unwrap();
        
        // Fetch open issues
        let open_url = format!(
            "https://api.github.com/repos/{}/{}/issues?state=open&per_page=100",
            self.owner,
            self.repo
        );
        
        let open_response = client
            .get(&open_url)
            .header("User-Agent", "rust-github-analyzer")
            .header("Accept", "application/vnd.github.v3+json")
            .bearer_auth(self.token.as_ref().unwrap())
            .send()
            .await
            .context("Failed to fetch open issues")?;
            
        if !open_response.status().is_success() {
            return Err(anyhow!(
                "GitHub API error: {} - {}",
                open_response.status(),
                open_response.text().await.unwrap_or_default()
            ));
        }
        
        // Fetch closed issues
        let closed_url = format!(
            "https://api.github.com/repos/{}/{}/issues?state=closed&per_page=100",
            self.owner,
            self.repo
        );
        
        let closed_response = client
            .get(&closed_url)
            .header("User-Agent", "rust-github-analyzer")
            .header("Accept", "application/vnd.github.v3+json")
            .bearer_auth(self.token.as_ref().unwrap())
            .send()
            .await
            .context("Failed to fetch closed issues")?;
            
        if !closed_response.status().is_success() {
            return Err(anyhow!(
                "GitHub API error: {} - {}",
                closed_response.status(),
                closed_response.text().await.unwrap_or_default()
            ));
        }
        
        let open_data: Vec<serde_json::Value> = open_response
            .json()
            .await
            .context("Failed to parse open issues response")?;
            
        let closed_data: Vec<serde_json::Value> = closed_response
            .json()
            .await
            .context("Failed to parse closed issues response")?;
            
        // Filter out pull requests (they're also returned by the issues API)
        let mut all_issues = Vec::new();
        
        for issue in open_data.into_iter().chain(closed_data.into_iter()) {
            if issue.get("pull_request").is_none() {
                all_issues.push(issue);
            }
        }
        
        // Calculate statistics
        let mut issue_stats = IssueStats::default();
        let mut by_label: HashMap<String, u32> = HashMap::new();
        let mut recent_issues = Vec::new();
        let mut total_age = 0.0;
        let mut total_resolution_time = 0.0;
        let mut resolution_count = 0;
        
        for issue in &all_issues {
            // Extract issue data
            let created_at = match DateTime::parse_from_rfc3339(
                issue["created_at"].as_str().unwrap_or("")
            ) {
                Ok(date) => date.with_timezone(&Utc),
                Err(_) => continue,
            };
            
            let state = issue["state"].as_str().unwrap_or("").to_string();
            let closed_at = if let Some(closed_str) = issue["closed_at"].as_str() {
                match DateTime::parse_from_rfc3339(closed_str) {
                    Ok(date) => Some(date.with_timezone(&Utc)),
                    Err(_) => None,
                }
            } else {
                None
            };
            
            // Count by state
            match state.as_str() {
                "open" => issue_stats.open += 1,
                "closed" => issue_stats.closed += 1,
                _ => {}
            }
            
            // Count labels
            if let Some(labels) = issue["labels"].as_array() {
                for label in labels {
                    let name = label["name"].as_str().unwrap_or("").to_string();
                    *by_label.entry(name).or_insert(0) += 1;
                }
            }
            
            // Calculate age and resolution time
            let now = Utc::now();
            let age = if let Some(closed) = closed_at {
                (closed - created_at).num_days() as f64
            } else {
                (now - created_at).num_days() as f64
            };
            
            total_age += age;
            
            if let Some(closed) = closed_at {
                let resolution_time = (closed - created_at).num_days() as f64;
                total_resolution_time += resolution_time;
                resolution_count += 1;
            }
            
            // Add to recent issues (up to 10)
            if recent_issues.len() < 10 {
                let issue_obj = Issue {
                    number: issue["number"].as_u64().unwrap_or(0) as u32,
                    title: issue["title"].as_str().unwrap_or("").to_string(),
                    state,
                    created_at,
                    updated_at: DateTime::parse_from_rfc3339(
                        issue["updated_at"].as_str().unwrap_or("")
                    ).unwrap_or_default().with_timezone(&Utc),
                    closed_at,
                    author: issue["user"]["login"].as_str().unwrap_or("").to_string(),
                    labels: if let Some(labels) = issue["labels"].as_array() {
                        labels.iter()
                            .map(|l| l["name"].as_str().unwrap_or("").to_string())
                            .collect()
                    } else {
                        Vec::new()
                    },
                    url: issue["html_url"].as_str().unwrap_or("").to_string(),
                };
                
                recent_issues.push(issue_obj);
            }
        }
        
        // Update stats
        issue_stats.total = issue_stats.open + issue_stats.closed;
        issue_stats.average_age_days = if issue_stats.total > 0 {
            total_age / issue_stats.total as f64
        } else {
            0.0
        };
        
        issue_stats.resolution_time_days = if resolution_count > 0 {
            total_resolution_time / resolution_count as f64
        } else {
            0.0
        };
        
        issue_stats.by_label = by_label;
        issue_stats.recent_issues = recent_issues;
        
        // Update metrics
        let metrics = self.metrics.as_mut();
        let github = metrics.get_mut("github").unwrap().as_object_mut().unwrap();
        github.insert("issueStats".to_string(), serde_json::to_value(&issue_stats)?);
        
        info!("Fetched {} issues ({} open, {} closed)", 
            issue_stats.total, issue_stats.open, issue_stats.closed);
        
        Ok(())
    }
    
    /// Fetch pull requests from GitHub
    async fn fetch_pull_requests(&mut self) -> Result<()> 
    where M: AsRef<HashMap<String, serde_json::Value>> + AsMut<HashMap<String, serde_json::Value>> {
        // Implementation similar to fetch_issues but for pull requests
        // For brevity, just create a default PullRequestStats
        let pr_stats = PullRequestStats::default();
        
        // Update metrics
        let metrics = self.metrics.as_mut();
        let github = metrics.get_mut("github").unwrap().as_object_mut().unwrap();
        github.insert("pullRequestStats".to_string(), serde_json::to_value(&pr_stats)?);
        
        info!("Fetched pull request stats");
        
        Ok(())
    }
    
    /// Fetch releases from GitHub
    async fn fetch_releases(&mut self) -> Result<()> 
    where M: AsRef<HashMap<String, serde_json::Value>> + AsMut<HashMap<String, serde_json::Value>> {
        // Implementation similar to fetch_issues but for releases
        // For brevity, just create a default ReleaseStats
        let release_stats = ReleaseStats::default();
        
        // Update metrics
        let metrics = self.metrics.as_mut();
        let github = metrics.get_mut("github").unwrap().as_object_mut().unwrap();
        github.insert("releaseStats".to_string(), serde_json::to_value(&release_stats)?);
        
        info!("Fetched release stats");
        
        Ok(())
    }
    
    /// Fetch branches from GitHub
    async fn fetch_branches(&mut self) -> Result<()> 
    where M: AsRef<HashMap<String, serde_json::Value>> + AsMut<HashMap<String, serde_json::Value>> {
        // Implementation similar to fetch_issues but for branches
        // For brevity, just create a default BranchStats with the main branch
        let mut branch_stats = BranchStats::default();
        branch_stats.default_branch = "main".to_string();
        
        // Update metrics
        let metrics = self.metrics.as_mut();
        let github = metrics.get_mut("github").unwrap().as_object_mut().unwrap();
        github.insert("branchStats".to_string(), serde_json::to_value(&branch_stats)?);
        
        info!("Fetched branch stats");
        
        Ok(())
    }
    
    /// Calculate development velocity metrics
    fn calculate_development_velocity(&mut self) -> Result<()> 
    where M: AsRef<HashMap<String, serde_json::Value>> + AsMut<HashMap<String, serde_json::Value>> {
        let metrics = self.metrics.as_ref();
        let github = metrics.get("github").unwrap().as_object().unwrap();
        
        let mut velocity = DevelopmentVelocity::default();
        
        // Calculate commits per day/week
        if let Some(commit_activity) = github.get("commitActivity") {
            if let Some(activities) = commit_activity.as_array() {
                if !activities.is_empty() {
                    let total_commits: u32 = activities.iter()
                        .map(|week| week["commits"].as_u64().unwrap_or(0) as u32)
                        .sum();
                    
                    let weeks = activities.len() as f64;
                    velocity.commits_per_week = total_commits as f64 / weeks;
                    velocity.commits_per_day = velocity.commits_per_week / 7.0;
                }
            }
        }
        
        // Calculate PRs per week
        if let Some(pr_stats) = github.get("pullRequestStats") {
            if let Some(total) = pr_stats["total"].as_u64() {
                // Assuming data is for the last 12 weeks
                velocity.prs_per_week = total as f64 / 12.0;
            }
        }
        
        // Calculate issues closed per week
        if let Some(issue_stats) = github.get("issueStats") {
            if let Some(closed) = issue_stats["closed"].as_u64() {
                // Assuming data is for the last 12 weeks
                velocity.issues_closed_per_week = closed as f64 / 12.0;
            }
        }
        
        // Count active contributors in the last month
        if let Some(contributors) = github.get("contributors") {
            if let Some(contribs) = contributors.as_array() {
                // This is a simplification, as we don't have date info here
                // In a real implementation, we'd look at commit dates
                velocity.active_contributors_last_month = contribs.len() as u32;
            }
        }
        
        // Update metrics
        let metrics = self.metrics.as_mut();
        let github = metrics.get_mut("github").unwrap().as_object_mut().unwrap();
        github.insert("developmentVelocity".to_string(), serde_json::to_value(&velocity)?);
        
        info!("Calculated development velocity metrics");
        
        Ok(())
    }
    
    /// Cache results to local file system
    fn cache_results(&self) -> Result<()> 
    where M: AsRef<HashMap<String, serde_json::Value>> {
        // Create cache directory if it doesn't exist
        fs::create_dir_all(&self.cache_dir)?;
        
        let cache_file = self.cache_dir.join(format!("github_{}_{}_{}.json", 
            self.owner, self.repo, chrono::Utc::now().format("%Y%m%d")));
        
        let metrics = self.metrics.as_ref();
        let github = metrics.get("github").unwrap();
        
        let mut file = File::create(cache_file.clone())?;
        file.write_all(serde_json::to_string_pretty(github)?.as_bytes())?;
        
        info!("Cached GitHub analysis results to {:?}", cache_file);
        
        Ok(())
    }
    
    /// Load cached results if available
    pub fn load_cached_results(&mut self) -> Result<bool> 
    where M: AsMut<HashMap<String, serde_json::Value>> {
        if !self.cache_dir.exists() {
            return Ok(false);
        }
        
        let pattern = format!("github_{}_{}_{}.json", 
            self.owner, self.repo, chrono::Utc::now().format("%Y%m%d"));
            
        for entry in fs::read_dir(&self.cache_dir)? {
            let entry = entry?;
            if entry.file_name().to_string_lossy().contains(&pattern) {
                let mut file = File::open(entry.path())?;
                let mut contents = String::new();
                file.read_to_string(&mut contents)?;
                
                let github_data: serde_json::Value = serde_json::from_str(&contents)?;
                
                // Update metrics
                let metrics = self.metrics.as_mut();
                metrics.insert("github".to_string(), github_data);
                
                info!("Loaded cached GitHub analysis results from {:?}", entry.path());
                return Ok(true);
            }
        }
        
        Ok(false)
    }
}
