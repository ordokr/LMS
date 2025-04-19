use super::models::{Quiz, Question, Answer, QuizVisibility, StudyMode};
use sqlx::{SqlitePool, Row, Sqlite};
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use anyhow::Result;
use tokio::sync::RwLock;
use uuid::Uuid;
use tracing::{debug, info, warn};

/// Cache entry with expiration and access tracking
struct CacheEntry<T> {
    data: T,
    expires_at: Instant,
    last_accessed: Instant,
    access_count: u32,
}

/// Query optimizer for quiz-related database operations
pub struct QuizQueryOptimizer {
    pool: SqlitePool,
    quiz_cache: RwLock<HashMap<String, CacheEntry<Vec<Quiz>>>>,
    question_cache: RwLock<HashMap<Uuid, CacheEntry<Vec<Question>>>>,
    cache_ttl: Duration,
    max_cache_size: usize,
    cache_hits: Arc<Mutex<u64>>,
    cache_misses: Arc<Mutex<u64>>,
    lru_queue: RwLock<VecDeque<String>>,
}

impl QuizQueryOptimizer {
    pub fn new(pool: SqlitePool) -> Self {
        Self {
            pool,
            quiz_cache: RwLock::new(HashMap::new()),
            question_cache: RwLock::new(HashMap::new()),
            cache_ttl: Duration::from_secs(300), // 5 minute cache
            max_cache_size: 1000, // Maximum number of cached queries
            cache_hits: Arc::new(Mutex::new(0)),
            cache_misses: Arc::new(Mutex::new(0)),
            lru_queue: RwLock::new(VecDeque::new()),
        }
    }

    /// Configure the cache settings
    pub fn with_cache_config(mut self, ttl: Duration, max_size: usize) -> Self {
        self.cache_ttl = ttl;
        self.max_cache_size = max_size;
        self
    }

    /// Optimized quiz fetch with caching and batch loading
    pub async fn fetch_quizzes(&self, filters: &QuizFilters) -> Result<Vec<Quiz>> {
        let cache_key = self.generate_cache_key(filters);

        // Check cache first
        let cached_result = {
            let cache = self.quiz_cache.read().await;
            if let Some(entry) = cache.get(&cache_key) {
                if entry.expires_at > Instant::now() {
                    // Update access stats
                    let mut hits = self.cache_hits.lock().unwrap();
                    *hits += 1;
                    debug!("Cache hit for key: {}", cache_key);
                    Some(entry.data.clone())
                } else {
                    None
                }
            } else {
                None
            }
        };

        if let Some(quizzes) = cached_result {
            // Update access time and count asynchronously
            let cache_key_clone = cache_key.clone();
            tokio::spawn(async move {
                if let Ok(mut cache) = self.quiz_cache.write().await {
                    if let Some(entry) = cache.get_mut(&cache_key_clone) {
                        entry.last_accessed = Instant::now();
                        entry.access_count += 1;
                    }
                }
            });
            return Ok(quizzes);
        }

        // Cache miss, update stats
        {
            let mut misses = self.cache_misses.lock().unwrap();
            *misses += 1;
        }

        // Build optimized query with parameterized filters
        let mut query_builder = String::from(
            "SELECT q.*, COUNT(qs.id) as question_count
            FROM quizzes q
            LEFT JOIN questions qs ON q.id = qs.quiz_id"
        );

        // Add WHERE clauses based on filters
        let mut conditions = Vec::new();
        let mut params = Vec::new();

        if let Some(visibility) = &filters.visibility {
            conditions.push("q.visibility = ?");
            params.push(visibility.to_string());
        }

        if let Some(study_mode) = &filters.study_mode {
            conditions.push("q.study_mode = ?");
            params.push(study_mode.to_string());
        }

        if let Some(author_id) = &filters.author_id {
            conditions.push("q.author_id = ?");
            params.push(author_id.to_string());
        }

        if !conditions.is_empty() {
            query_builder.push_str(" WHERE ");
            query_builder.push_str(&conditions.join(" AND "));
        }

        // Add GROUP BY, ORDER BY, and LIMIT
        query_builder.push_str("
            GROUP BY q.id
            ORDER BY q.created_at DESC
            LIMIT ?
        ");

        // Build and execute the query
        let mut query = sqlx::query(&query_builder);

        // Bind all parameters
        for param in params {
            query = query.bind(param);
        }

        // Bind limit and offset
        query = query.bind(filters.limit);

        // Execute query and transform results
        let rows = query.fetch_all(&self.pool).await?;

        let quizzes = rows.iter()
            .map(|row| Quiz {
                id: Uuid::parse_str(row.get::<String, _>("id").as_str()).unwrap_or_else(|_| Uuid::nil()),
                title: row.get("title"),
                description: row.get("description"),
                question_count: Some(row.get::<i64, _>("question_count") as usize),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
                visibility: row.try_get("visibility")
                    .unwrap_or(QuizVisibility::Public),
                study_mode: row.try_get("study_mode")
                    .unwrap_or(StudyMode::MultipleChoice),
                author_id: row.try_get::<String, _>("author_id")
                    .ok()
                    .and_then(|id| Uuid::parse_str(&id).ok()),
                // Other fields with defaults
                questions: Vec::new(),
                settings: Default::default(),
                tags: Vec::new(),
            })
            .collect::<Vec<Quiz>>();

        // Cache results
        self.cache_quiz_results(cache_key, quizzes.clone()).await;

        Ok(quizzes)
    }

    /// Batch load questions for multiple quizzes with caching
    pub async fn batch_load_questions(&self, quiz_ids: &[Uuid]) -> Result<HashMap<Uuid, Vec<Question>>> {
        // Check cache first for each quiz ID
        let mut result = HashMap::new();
        let mut uncached_ids = Vec::new();

        // Try to get cached questions
        {
            let cache = self.question_cache.read().await;
            for &quiz_id in quiz_ids {
                if let Some(entry) = cache.get(&quiz_id) {
                    if entry.expires_at > Instant::now() {
                        // Cache hit
                        let mut hits = self.cache_hits.lock().unwrap();
                        *hits += 1;
                        result.insert(quiz_id, entry.data.clone());
                    } else {
                        // Expired cache
                        uncached_ids.push(quiz_id);
                    }
                } else {
                    // Cache miss
                    uncached_ids.push(quiz_id);
                }
            }
        }

        // If all quiz IDs were in cache, return early
        if uncached_ids.is_empty() {
            return Ok(result);
        }

        // Update cache miss count
        {
            let mut misses = self.cache_misses.lock().unwrap();
            *misses += uncached_ids.len() as u64;
        }

        // Convert UUIDs to strings for the query
        let id_strings: Vec<String> = uncached_ids.iter().map(|id| id.to_string()).collect();

        // Build the query with a proper IN clause
        let placeholders: Vec<String> = (1..=id_strings.len()).map(|i| format!("?{}", i)).collect();
        let query_str = format!(
            "SELECT * FROM questions
            WHERE quiz_id IN ({})
            ORDER BY position",
            placeholders.join(", ")
        );

        // Build and execute the query
        let mut query = sqlx::query(&query_str);

        // Bind all quiz IDs
        for id in &id_strings {
            query = query.bind(id);
        }

        let rows = query.fetch_all(&self.pool).await?;

        // Group questions by quiz_id
        let mut questions_by_quiz: HashMap<Uuid, Vec<Question>> = HashMap::new();
        for row in rows {
            let quiz_id_str: String = row.get("quiz_id");
            let quiz_id = Uuid::parse_str(&quiz_id_str).unwrap_or_else(|_| Uuid::nil());

            let question = Question {
                id: Uuid::parse_str(row.get::<String, _>("id").as_str()).unwrap_or_else(|_| Uuid::nil()),
                quiz_id,
                content: row.get("content"),
                answer_type: row.get("answer_type"),
                choices: Vec::new(), // Will be loaded separately if needed
                correct_answer: Default::default(), // Will be loaded separately if needed
                explanation: row.get("explanation"),
            };

            questions_by_quiz
                .entry(quiz_id)
                .or_insert_with(Vec::new)
                .push(question);
        }

        // Cache the results
        self.cache_question_results(&questions_by_quiz).await;

        // Merge with cached results
        for (quiz_id, questions) in questions_by_quiz {
            result.insert(quiz_id, questions);
        }

        Ok(result)
    }

    /// Load questions for a single quiz with caching
    pub async fn load_quiz_questions(&self, quiz_id: Uuid) -> Result<Vec<Question>> {
        // Check cache first
        let cached_result = {
            let cache = self.question_cache.read().await;
            if let Some(entry) = cache.get(&quiz_id) {
                if entry.expires_at > Instant::now() {
                    // Update access stats
                    let mut hits = self.cache_hits.lock().unwrap();
                    *hits += 1;
                    Some(entry.data.clone())
                } else {
                    None
                }
            } else {
                None
            }
        };

        if let Some(questions) = cached_result {
            return Ok(questions);
        }

        // Cache miss, update stats
        {
            let mut misses = self.cache_misses.lock().unwrap();
            *misses += 1;
        }

        // Fetch questions from database
        let query = sqlx::query(
            r#"
            SELECT * FROM questions
            WHERE quiz_id = ?
            ORDER BY position
            "#
        )
        .bind(quiz_id.to_string());

        let rows = query.fetch_all(&self.pool).await?;

        let questions = rows.iter()
            .map(|row| Question {
                id: Uuid::parse_str(row.get::<String, _>("id").as_str()).unwrap_or_else(|_| Uuid::nil()),
                quiz_id,
                content: row.get("content"),
                answer_type: row.get("answer_type"),
                choices: Vec::new(), // Will be loaded separately if needed
                correct_answer: Default::default(), // Will be loaded separately if needed
                explanation: row.get("explanation"),
            })
            .collect::<Vec<Question>>();

        // Cache the results
        let mut cache = self.question_cache.write().await;
        cache.insert(quiz_id, CacheEntry {
            data: questions.clone(),
            expires_at: Instant::now() + self.cache_ttl,
            last_accessed: Instant::now(),
            access_count: 1,
        });

        Ok(questions)
    }

    /// Cache quiz results with proper expiration
    async fn cache_quiz_results(&self, key: String, quizzes: Vec<Quiz>) {
        let mut cache = self.quiz_cache.write().await;
        let mut lru_queue = self.lru_queue.write().await;

        // Check if we need to evict entries
        if cache.len() >= self.max_cache_size {
            // Evict least recently used entries
            while cache.len() >= self.max_cache_size && !lru_queue.is_empty() {
                if let Some(lru_key) = lru_queue.pop_front() {
                    cache.remove(&lru_key);
                    debug!("Evicted LRU cache entry: {}", lru_key);
                }
            }
        }

        // Add to cache
        cache.insert(key.clone(), CacheEntry {
            data: quizzes,
            expires_at: Instant::now() + self.cache_ttl,
            last_accessed: Instant::now(),
            access_count: 1,
        });

        // Add to LRU queue
        lru_queue.push_back(key);
    }

    /// Cache question results for multiple quizzes
    async fn cache_question_results(&self, questions_by_quiz: &HashMap<Uuid, Vec<Question>>) {
        let mut cache = self.question_cache.write().await;

        // Check if we need to evict entries
        if cache.len() + questions_by_quiz.len() > self.max_cache_size {
            // Simple strategy: remove oldest entries first
            let mut entries: Vec<_> = cache.iter().collect();
            entries.sort_by_key(|(_, entry)| entry.last_accessed);

            // Remove oldest entries until we have enough space
            let to_remove = (cache.len() + questions_by_quiz.len()) - self.max_cache_size;
            for i in 0..to_remove {
                if i < entries.len() {
                    let (key, _) = entries[i];
                    cache.remove(key);
                    debug!("Evicted old cache entry for quiz: {}", key);
                }
            }
        }

        // Add all new entries to cache
        for (quiz_id, questions) in questions_by_quiz {
            cache.insert(*quiz_id, CacheEntry {
                data: questions.clone(),
                expires_at: Instant::now() + self.cache_ttl,
                last_accessed: Instant::now(),
                access_count: 1,
            });
        }
    }

    /// Clear expired cache entries
    pub async fn clear_expired_cache(&self) {
        let now = Instant::now();

        // Clear expired quiz cache entries
        {
            let mut cache = self.quiz_cache.write().await;
            let expired_keys: Vec<String> = cache.iter()
                .filter(|(_, entry)| entry.expires_at <= now)
                .map(|(key, _)| key.clone())
                .collect();

            for key in expired_keys {
                cache.remove(&key);
                debug!("Removed expired cache entry: {}", key);
            }
        }

        // Clear expired question cache entries
        {
            let mut cache = self.question_cache.write().await;
            let expired_keys: Vec<Uuid> = cache.iter()
                .filter(|(_, entry)| entry.expires_at <= now)
                .map(|(key, _)| *key)
                .collect();

            for key in expired_keys {
                cache.remove(&key);
                debug!("Removed expired question cache for quiz: {}", key);
            }
        }
    }

    /// Get cache statistics
    pub fn get_cache_stats(&self) -> (u64, u64, f64) {
        let hits = *self.cache_hits.lock().unwrap();
        let misses = *self.cache_misses.lock().unwrap();
        let total = hits + misses;
        let hit_rate = if total > 0 { hits as f64 / total as f64 } else { 0.0 };

        (hits, misses, hit_rate)
    }

    fn generate_cache_key(&self, filters: &QuizFilters) -> String {
        let visibility = filters.visibility.as_ref().map_or("all", |v| v.as_str());
        let study_mode = filters.study_mode.as_ref().map_or("all", |s| s.as_str());
        let author = filters.author_id.as_ref().map_or("all", |a| a.to_string());

        format!("quizzes:{}:{}:{}:{}:{}",
            visibility,
            study_mode,
            author,
            filters.limit,
            filters.offset
        )
    }
}

#[derive(Debug, Clone)]
pub struct QuizFilters {
    pub visibility: Option<QuizVisibility>,
    pub study_mode: Option<StudyMode>,
    pub author_id: Option<Uuid>,
    pub limit: i64,
    pub offset: i64,
    pub search_term: Option<String>,
    pub tags: Option<Vec<String>>,
}

impl QuizFilters {
    pub fn new() -> Self {
        Self {
            visibility: None,
            study_mode: None,
            author_id: None,
            limit: 50,
            offset: 0,
            search_term: None,
            tags: None,
        }
    }

    pub fn with_visibility(mut self, visibility: QuizVisibility) -> Self {
        self.visibility = Some(visibility);
        self
    }

    pub fn with_study_mode(mut self, study_mode: StudyMode) -> Self {
        self.study_mode = Some(study_mode);
        self
    }

    pub fn with_author(mut self, author_id: Uuid) -> Self {
        self.author_id = Some(author_id);
        self
    }

    pub fn with_limit(mut self, limit: i64) -> Self {
        self.limit = limit;
        self
    }

    pub fn with_offset(mut self, offset: i64) -> Self {
        self.offset = offset;
        self
    }

    pub fn with_search(mut self, term: String) -> Self {
        self.search_term = Some(term);
        self
    }

    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = Some(tags);
        self
    }
}