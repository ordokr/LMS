use std::collections::HashMap;
use std::sync::Arc;
use anyhow::{Result, Context, anyhow};
use async_trait::async_trait;
use log::{info, error};
use serde::{Serialize, Deserialize};
use reqwest::Client;

/// Vector database options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorDatabaseOptions {
    pub db_type: DatabaseType,
    pub dimensions: usize,
    pub collection_name: String,
    pub qdrant_url: String,
    pub qdrant_api_key: Option<String>,
}

impl Default for VectorDatabaseOptions {
    fn default() -> Self {
        Self {
            db_type: DatabaseType::Memory,
            dimensions: 512,
            collection_name: "canvas_discourse_integration".to_string(),
            qdrant_url: "http://localhost:6333".to_string(),
            qdrant_api_key: None,
        }
    }
}

/// Supported database types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DatabaseType {
    #[serde(rename = "memory")]
    Memory,
    #[serde(rename = "qdrant")]
    Qdrant,
}

/// Document metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentMetadata {
    #[serde(flatten)]
    pub fields: HashMap<String, String>,
}

/// Document with vector embedding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    pub id: String,
    pub vector: Vec<f32>,
    pub metadata: DocumentMetadata,
}

/// Search query result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub document_id: String,
    pub score: f32,
    pub metadata: DocumentMetadata,
}

/// Search filter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchFilter {
    pub field: String,
    pub value: String,
}

/// Qdrant point payload
#[derive(Debug, Clone, Serialize, Deserialize)]
struct QdrantPayload {
    metadata: DocumentMetadata,
}

/// Qdrant point
#[derive(Debug, Clone, Serialize, Deserialize)]
struct QdrantPoint {
    id: String,
    vector: Vec<f32>,
    payload: QdrantPayload,
}

/// Qdrant search request
#[derive(Debug, Clone, Serialize, Deserialize)]
struct QdrantSearchRequest {
    vector: Vec<f32>,
    limit: usize,
    filter: Option<QdrantFilter>,
    with_payload: bool,
}

/// Qdrant filter
#[derive(Debug, Clone, Serialize, Deserialize)]
struct QdrantFilter {
    should: Vec<QdrantCondition>,
}

/// Qdrant filter condition
#[derive(Debug, Clone, Serialize, Deserialize)]
struct QdrantCondition {
    key: String,
    match_value: String,
}

/// Qdrant collection creation request
#[derive(Debug, Clone, Serialize, Deserialize)]
struct QdrantCreateCollectionRequest {
    vectors: QdrantVectorConfig,
    optimizers_config: QdrantOptimizerConfig,
    on_disk_payload: bool,
}

/// Qdrant vector configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
struct QdrantVectorConfig {
    size: usize,
    distance: String,
}

/// Qdrant optimizer configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
struct QdrantOptimizerConfig {
    default_segment_number: usize,
}

/// Qdrant index creation request
#[derive(Debug, Clone, Serialize, Deserialize)]
struct QdrantCreateIndexRequest {
    field_name: String,
    field_schema: String,
}

/// Qdrant upsert request
#[derive(Debug, Clone, Serialize, Deserialize)]
struct QdrantUpsertRequest {
    wait: bool,
    points: Vec<QdrantPoint>,
}

/// Qdrant search response
#[derive(Debug, Clone, Serialize, Deserialize)]
struct QdrantSearchResponse {
    result: Vec<QdrantSearchResult>,
}

/// Qdrant search result
#[derive(Debug, Clone, Serialize, Deserialize)]
struct QdrantSearchResult {
    id: String,
    score: f32,
    payload: QdrantPayload,
}

/// Vector database adapter trait
#[async_trait]
pub trait VectorDatabase: Send + Sync {
    /// Initialize the database
    async fn initialize(&mut self) -> Result<()>;
    
    /// Store an embedding vector
    async fn store_embedding(
        &self,
        id: &str,
        vector: Vec<f32>,
        metadata: DocumentMetadata,
    ) -> Result<bool>;
    
    /// Store multiple embedding vectors
    async fn store_embeddings(
        &self,
        documents: Vec<Document>,
    ) -> Result<bool>;
    
    /// Find similar vectors
    async fn find_similar(
        &self,
        vector: &[f32],
        limit: usize,
        filter: Option<SearchFilter>,
    ) -> Result<Vec<SearchResult>>;
    
    /// Delete a document
    async fn delete_document(&self, id: &str) -> Result<bool>;
    
    /// Get document by ID
    async fn get_document(&self, id: &str) -> Result<Option<Document>>;
}

/// Memory-based vector database
pub struct MemoryVectorDatabase {
    memory_index: HashMap<String, Document>,
    initialized: bool,
}

impl MemoryVectorDatabase {
    /// Create a new memory-based vector database
    pub fn new() -> Self {
        Self {
            memory_index: HashMap::new(),
            initialized: false,
        }
    }
}

#[async_trait]
impl VectorDatabase for MemoryVectorDatabase {
    async fn initialize(&mut self) -> Result<()> {
        if self.initialized {
            return Ok(());
        }
        
        self.memory_index = HashMap::new();
        info!("In-memory vector index initialized");
        
        self.initialized = true;
        Ok(())
    }
    
    async fn store_embedding(
        &self,
        id: &str,
        vector: Vec<f32>,
        metadata: DocumentMetadata,
    ) -> Result<bool> {
        let mut memory_index = self.memory_index.clone();
        
        memory_index.insert(
            id.to_string(),
            Document {
                id: id.to_string(),
                vector,
                metadata,
            },
        );
        
        info!("Stored embedding for document: {}", id);
        Ok(true)
    }
    
    async fn store_embeddings(
        &self,
        documents: Vec<Document>,
    ) -> Result<bool> {
        let mut memory_index = self.memory_index.clone();
        
        for doc in documents {
            memory_index.insert(doc.id.clone(), doc);
        }
        
        info!("Stored {} embeddings in memory", documents.len());
        Ok(true)
    }
    
    async fn find_similar(
        &self,
        vector: &[f32],
        limit: usize,
        filter: Option<SearchFilter>,
    ) -> Result<Vec<SearchResult>> {
        let mut results: Vec<(String, f32, DocumentMetadata)> = Vec::new();
        
        // Calculate cosine similarity for each vector in memory
        for (id, doc) in &self.memory_index {
            // Apply filter if specified
            if let Some(filter_condition) = &filter {
                if let Some(field_value) = doc.metadata.fields.get(&filter_condition.field) {
                    if field_value != &filter_condition.value {
                        continue;
                    }
                } else {
                    continue;
                }
            }
            
            let similarity = cosine_similarity(vector, &doc.vector);
            results.push((id.clone(), similarity, doc.metadata.clone()));
        }
        
        // Sort by similarity (descending)
        results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        
        // Take top 'limit' results
        let search_results: Vec<SearchResult> = results
            .iter()
            .take(limit)
            .map(|(id, score, metadata)| SearchResult {
                document_id: id.clone(),
                score: *score,
                metadata: metadata.clone(),
            })
            .collect();
        
        Ok(search_results)
    }
    
    async fn delete_document(&self, id: &str) -> Result<bool> {
        let mut memory_index = self.memory_index.clone();
        
        memory_index.remove(id);
        
        info!("Deleted document: {}", id);
        Ok(true)
    }
    
    async fn get_document(&self, id: &str) -> Result<Option<Document>> {
        Ok(self.memory_index.get(id).cloned())
    }
}

/// Qdrant vector database
pub struct QdrantVectorDatabase {
    client: Client,
    options: VectorDatabaseOptions,
    initialized: bool,
}

impl QdrantVectorDatabase {
    /// Create a new Qdrant vector database
    pub fn new(options: VectorDatabaseOptions) -> Self {
        Self {
            client: Client::new(),
            options,
            initialized: false,
        }
    }
    
    /// Get the base URL for Qdrant API
    fn base_url(&self) -> String {
        format!("{}/collections/{}", self.options.qdrant_url, self.options.collection_name)
    }
    
    /// Build a request with authentication if needed
    fn build_request(&self, url: &str) -> reqwest::RequestBuilder {
        let mut builder = self.client.get(url);
        
        if let Some(api_key) = &self.options.qdrant_api_key {
            builder = builder.header("api-key", api_key);
        }
        
        builder
    }
}

#[async_trait]
impl VectorDatabase for QdrantVectorDatabase {
    async fn initialize(&mut self) -> Result<()> {
        if self.initialized {
            return Ok(());
        }
        
        // Check if collection exists
        let collection_url = format!("{}/collections/{}", 
            self.options.qdrant_url, 
            self.options.collection_name
        );
        
        let mut builder = self.client.get(&collection_url);
        
        if let Some(api_key) = &self.options.qdrant_api_key {
            builder = builder.header("api-key", api_key);
        }
        
        let response = builder.send().await;
        
        match response {
            Ok(resp) if resp.status().is_success() => {
                info!("Using existing Qdrant collection: {}", self.options.collection_name);
            }
            _ => {
                // Create new collection
                info!("Creating new Qdrant collection: {}", self.options.collection_name);
                
                let create_request = QdrantCreateCollectionRequest {
                    vectors: QdrantVectorConfig {
                        size: self.options.dimensions,
                        distance: "Cosine".to_string(),
                    },
                    optimizers_config: QdrantOptimizerConfig {
                        default_segment_number: 2,
                    },
                    on_disk_payload: true,
                };
                
                let mut builder = self.client.put(&collection_url);
                
                if let Some(api_key) = &self.options.qdrant_api_key {
                    builder = builder.header("api-key", api_key);
                }
                
                let response = builder
                    .json(&create_request)
                    .send()
                    .await
                    .context("Failed to create Qdrant collection")?;
                    
                if !response.status().is_success() {
                    let error_text = response.text().await.unwrap_or_default();
                    return Err(anyhow!("Failed to create Qdrant collection: {}", error_text));
                }
                
                // Create recommended indexes
                let index_url = format!("{}/index", &collection_url);
                
                for field in &["metadata.system", "metadata.category"] {
                    let index_request = QdrantCreateIndexRequest {
                        field_name: field.to_string(),
                        field_schema: "keyword".to_string(),
                    };
                    
                    let mut builder = self.client.put(&index_url);
                    
                    if let Some(api_key) = &self.options.qdrant_api_key {
                        builder = builder.header("api-key", api_key);
                    }
                    
                    let response = builder
                        .json(&index_request)
                        .send()
                        .await
                        .context(format!("Failed to create index for field: {}", field))?;
                        
                    if !response.status().is_success() {
                        let error_text = response.text().await.unwrap_or_default();
                        return Err(anyhow!("Failed to create index: {}", error_text));
                    }
                }
            }
        }
        
        info!("Qdrant connection established");
        self.initialized = true;
        Ok(())
    }
    
    async fn store_embedding(
        &self,
        id: &str,
        vector: Vec<f32>,
        metadata: DocumentMetadata,
    ) -> Result<bool> {
        let points_url = format!("{}/points", self.base_url());
        
        let upsert_request = QdrantUpsertRequest {
            wait: true,
            points: vec![
                QdrantPoint {
                    id: id.to_string(),
                    vector,
                    payload: QdrantPayload { metadata },
                }
            ],
        };
        
        let mut builder = self.client.put(&points_url);
        
        if let Some(api_key) = &self.options.qdrant_api_key {
            builder = builder.header("api-key", api_key);
        }
        
        let response = builder
            .json(&upsert_request)
            .send()
            .await
            .context("Failed to store embedding in Qdrant")?;
            
        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(anyhow!("Failed to store embedding: {}", error_text));
        }
        
        info!("Stored embedding for document: {}", id);
        Ok(true)
    }
    
    async fn store_embeddings(
        &self,
        documents: Vec<Document>,
    ) -> Result<bool> {
        let points_url = format!("{}/points", self.base_url());
        
        let qdrant_points: Vec<QdrantPoint> = documents
            .into_iter()
            .map(|doc| QdrantPoint {
                id: doc.id,
                vector: doc.vector,
                payload: QdrantPayload { metadata: doc.metadata },
            })
            .collect();
        
        let upsert_request = QdrantUpsertRequest {
            wait: true,
            points: qdrant_points,
        };
        
        let mut builder = self.client.put(&points_url);
        
        if let Some(api_key) = &self.options.qdrant_api_key {
            builder = builder.header("api-key", api_key);
        }
        
        let response = builder
            .json(&upsert_request)
            .send()
            .await
            .context("Failed to store embeddings in Qdrant")?;
            
        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(anyhow!("Failed to store embeddings: {}", error_text));
        }
        
        info!("Stored {} embeddings in Qdrant", upsert_request.points.len());
        Ok(true)
    }
    
    async fn find_similar(
        &self,
        vector: &[f32],
        limit: usize,
        filter: Option<SearchFilter>,
    ) -> Result<Vec<SearchResult>> {
        let search_url = format!("{}/points/search", self.base_url());
        
        let qdrant_filter = filter.map(|f| QdrantFilter {
            should: vec![
                QdrantCondition {
                    key: f.field,
                    match_value: f.value,
                }
            ],
        });
        
        let search_request = QdrantSearchRequest {
            vector: vector.to_vec(),
            limit,
            filter: qdrant_filter,
            with_payload: true,
        };
        
        let mut builder = self.client.post(&search_url);
        
        if let Some(api_key) = &self.options.qdrant_api_key {
            builder = builder.header("api-key", api_key);
        }
        
        let response = builder
            .json(&search_request)
            .send()
            .await
            .context("Failed to search in Qdrant")?;
            
        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(anyhow!("Failed to search: {}", error_text));
        }
        
        let search_response: QdrantSearchResponse = response
            .json()
            .await
            .context("Failed to parse Qdrant search response")?;
            
        let results = search_response.result
            .into_iter()
            .map(|r| SearchResult {
                document_id: r.id,
                score: r.score,
                metadata: r.payload.metadata,
            })
            .collect();
            
        Ok(results)
    }
    
    async fn delete_document(&self, id: &str) -> Result<bool> {
        let delete_url = format!("{}/points/delete", self.base_url());
        
        let mut builder = self.client.post(&delete_url);
        
        if let Some(api_key) = &self.options.qdrant_api_key {
            builder = builder.header("api-key", api_key);
        }
        
        let response = builder
            .json(&serde_json::json!({
                "points": [id],
                "wait": true
            }))
            .send()
            .await
            .context("Failed to delete document from Qdrant")?;
            
        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(anyhow!("Failed to delete document: {}", error_text));
        }
        
        info!("Deleted document: {}", id);
        Ok(true)
    }
    
    async fn get_document(&self, id: &str) -> Result<Option<Document>> {
        let get_url = format!("{}/points/{}", self.base_url(), id);
        
        let mut builder = self.client.get(&get_url);
        
        if let Some(api_key) = &self.options.qdrant_api_key {
            builder = builder.header("api-key", api_key);
        }
        
        let response = builder
            .send()
            .await
            .context("Failed to get document from Qdrant")?;
            
        if response.status().is_client_error() {
            return Ok(None);
        }
        
        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(anyhow!("Failed to get document: {}", error_text));
        }
        
        #[derive(Deserialize)]
        struct QdrantGetResponse {
            result: QdrantPoint,
        }
        
        let get_response: QdrantGetResponse = response
            .json()
            .await
            .context("Failed to parse Qdrant get response")?;
            
        let point = get_response.result;
        
        Ok(Some(Document {
            id: point.id,
            vector: point.vector,
            metadata: point.payload.metadata,
        }))
    }
}

/// Vector Database Adapter
/// Provides storage and retrieval for vector embeddings using various backends
pub struct VectorDatabaseAdapter {
    options: VectorDatabaseOptions,
    db: Option<Arc<dyn VectorDatabase>>,
    initialized: bool,
}

impl VectorDatabaseAdapter {
    /// Create a new vector database adapter
    pub fn new(options: Option<VectorDatabaseOptions>) -> Self {
        let options = options.unwrap_or_default();
        
        info!("Initializing vector database adapter with type: {:?}", options.db_type);
        
        Self {
            options,
            db: None,
            initialized: false,
        }
    }
    
    /// Initialize the database connection
    pub async fn initialize(&mut self) -> Result<()> {
        if self.initialized {
            return Ok(());
        }
        
        match self.options.db_type {
            DatabaseType::Qdrant => {
                let mut db = QdrantVectorDatabase::new(self.options.clone());
                db.initialize().await?;
                self.db = Some(Arc::new(db));
            }
            DatabaseType::Memory => {
                let mut db = MemoryVectorDatabase::new();
                db.initialize().await?;
                self.db = Some(Arc::new(db));
            }
        }
        
        self.initialized = true;
        info!("Vector database initialized with {:?} backend", self.options.db_type);
        
        Ok(())
    }
    
    /// Store a single embedding vector
    pub async fn store_embedding(
        &self,
        id: &str,
        vector: Vec<f32>,
        metadata: HashMap<String, String>,
    ) -> Result<bool> {
        self.ensure_initialized().await?;
        
        let db = self.db.as_ref().unwrap();
        
        db.store_embedding(
            id,
            vector,
            DocumentMetadata { fields: metadata },
        ).await
    }
    
    /// Store multiple embedding vectors
    pub async fn store_embeddings(
        &self,
        documents: Vec<(String, Vec<f32>, HashMap<String, String>)>,
    ) -> Result<bool> {
        self.ensure_initialized().await?;
        
        let db = self.db.as_ref().unwrap();
        
        let docs: Vec<Document> = documents
            .into_iter()
            .map(|(id, vector, metadata)| Document {
                id,
                vector,
                metadata: DocumentMetadata { fields: metadata },
            })
            .collect();
            
        db.store_embeddings(docs).await
    }
    
    /// Find similar vectors
    pub async fn find_similar(
        &self,
        vector: &[f32],
        options: HashMap<String, serde_json::Value>,
    ) -> Result<Vec<SearchResult>> {
        self.ensure_initialized().await?;
        
        let db = self.db.as_ref().unwrap();
        
        // Extract options
        let limit = options
            .get("topK")
            .and_then(|v| v.as_u64())
            .unwrap_or(5) as usize;
            
        let filter = if let Some(filter_obj) = options.get("filter") {
            if let Some(obj) = filter_obj.as_object() {
                // Just use the first filter for simplicity
                if let Some((field, value)) = obj.iter().next() {
                    if let Some(value_str) = value.as_str() {
                        Some(SearchFilter {
                            field: field.clone(),
                            value: value_str.to_string(),
                        })
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        };
        
        db.find_similar(vector, limit, filter).await
    }
    
    /// Delete a document
    pub async fn delete_document(&self, id: &str) -> Result<bool> {
        self.ensure_initialized().await?;
        
        let db = self.db.as_ref().unwrap();
        db.delete_document(id).await
    }
    
    /// Get document by ID
    pub async fn get_document(&self, id: &str) -> Result<Option<Document>> {
        self.ensure_initialized().await?;
        
        let db = self.db.as_ref().unwrap();
        db.get_document(id).await
    }
    
    /// Ensure the database is initialized
    async fn ensure_initialized(&self) -> Result<()> {
        if !self.initialized {
            return Err(anyhow!("Vector database not initialized. Call initialize() first."));
        }
        
        Ok(())
    }
}

/// Calculate cosine similarity between two vectors
fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    let mut dot_product = 0.0;
    let mut norm_a = 0.0;
    let mut norm_b = 0.0;
    
    for i in 0..a.len().min(b.len()) {
        dot_product += a[i] * b[i];
        norm_a += a[i] * a[i];
        norm_b += b[i] * b[i];
    }
    
    if norm_a == 0.0 || norm_b == 0.0 {
        return 0.0;
    }
    
    dot_product / (norm_a.sqrt() * norm_b.sqrt())
}
