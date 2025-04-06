/**
 * Vector Database Adapter
 * Provides storage and retrieval for vector embeddings using various backends
 */
const fs = require('fs-extra');
const path = require('path');
const { QdrantClient } = require('@qdrant/js-client-rest');

class VectorDatabaseAdapter {
  /**
   * Create a new vector database adapter
   * @param {Object} options - Configuration options
   * @param {string} options.dbType - Database type ('memory', 'qdrant')
   * @param {number} options.dimensions - Vector dimensions
   * @param {string} options.collectionName - Collection name for storing vectors
   * @param {string} options.qdrantUrl - Qdrant server URL (if using Qdrant)
   * @param {string} options.qdrantApiKey - Qdrant API key (if using cloud Qdrant)
   */
  constructor(options = {}) {
    this.options = Object.assign({
      dbType: 'memory',           // memory, qdrant
      dimensions: 512,            // embedding dimensions
      collectionName: 'canvas_discourse_integration',
      qdrantUrl: 'http://localhost:6333',
      qdrantApiKey: null,
    }, options);
    
    this.client = null;
    this.memoryIndex = {};        // For in-memory storage
    this.initialized = false;
    
    console.log(`Initializing vector database adapter with type: ${this.options.dbType}`);
  }
  
  /**
   * Initialize the database connection
   */
  async initialize() {
    if (this.initialized) return;
    
    try {
      switch(this.options.dbType) {
        case 'qdrant':
          await this.initializeQdrant();
          break;
        case 'memory':
        default:
          this.initializeMemoryIndex();
      }
      
      this.initialized = true;
      console.log(`Vector database initialized with ${this.options.dbType} backend`);
    } catch (error) {
      console.error(`Failed to initialize vector database: ${error.message}`);
      throw error;
    }
  }
  
  /**
   * Initialize in-memory vector storage
   */
  initializeMemoryIndex() {
    this.memoryIndex = {};
    console.log('In-memory vector index initialized');
  }
  
  /**
   * Initialize Qdrant vector database
   */
  async initializeQdrant() {
    try {
      // Create Qdrant client
      const clientOptions = {
        url: this.options.qdrantUrl
      };
      
      if (this.options.qdrantApiKey) {
        clientOptions.apiKey = this.options.qdrantApiKey;
      }
      
      this.client = new QdrantClient(clientOptions);
      
      // Check if collection exists, create if not
      try {
        const collection = await this.client.getCollection(this.options.collectionName);
        console.log(`Using existing Qdrant collection: ${this.options.collectionName}`);
      } catch (error) {
        console.log(`Creating new Qdrant collection: ${this.options.collectionName}`);
        
        await this.client.createCollection(this.options.collectionName, {
          vectors: {
            size: this.options.dimensions,
            distance: 'Cosine'
          },
          optimizers_config: {
            default_segment_number: 2
          },
          on_disk_payload: true
        });
        
        // Create recommended indexes for faster filtering
        await this.client.createPayloadIndex(this.options.collectionName, {
          field_name: 'metadata.system',
          field_schema: 'keyword'
        });
        
        await this.client.createPayloadIndex(this.options.collectionName, {
          field_name: 'metadata.category',
          field_schema: 'keyword'
        });
      }
      
      console.log('Qdrant connection established');
    } catch (error) {
      console.error(`Failed to initialize Qdrant: ${error.message}`);
      throw error;
    }
  }
  
  /**
   * Store a single embedding vector
   * @param {string} id - Document ID
   * @param {Array<number>} vector - Embedding vector
   * @param {Object} metadata - Document metadata
   */
  async storeEmbedding(id, vector, metadata = {}) {
    await this.initialize();
    
    try {
      switch(this.options.dbType) {
        case 'qdrant':
          await this.client.upsert(this.options.collectionName, {
            wait: true,
            points: [{
              id,
              vector,
              payload: { metadata }
            }]
          });
          break;
        case 'memory':
        default:
          this.memoryIndex[id] = { vector, metadata };
      }
      
      console.log(`Stored embedding for document: ${id}`);
      return true;
    } catch (error) {
      console.error(`Failed to store embedding: ${error.message}`);
      return false;
    }
  }
  
  /**
   * Store multiple embedding vectors in batch
   * @param {Array<Object>} embeddings - Array of embeddings to store
   * @param {string} embeddings[].id - Document ID
   * @param {Array<number>} embeddings[].vector - Embedding vector
   * @param {Object} embeddings[].metadata - Document metadata
   */
  async bulkStoreEmbeddings(embeddings) {
    await this.initialize();
    
    if (!embeddings || embeddings.length === 0) {
      console.warn('No embeddings provided for bulk storage');
      return false;
    }
    
    try {
      const batchSize = 100; // Process in batches to avoid overloading
      const batches = [];
      
      // Split into batches
      for (let i = 0; i < embeddings.length; i += batchSize) {
        batches.push(embeddings.slice(i, i + batchSize));
      }
      
      // Process each batch
      for (const batch of batches) {
        switch(this.options.dbType) {
          case 'qdrant':
            await this.client.upsert(this.options.collectionName, {
              wait: true,
              points: batch.map(item => ({
                id: typeof item.id === 'string' ? item.id : String(item.id),
                vector: item.vector,
                payload: { metadata: item.metadata }
              }))
            });
            break;
          case 'memory':
          default:
            batch.forEach(item => {
              this.memoryIndex[item.id] = { 
                vector: item.vector, 
                metadata: item.metadata 
              };
            });
        }
        
        console.log(`Stored batch of ${batch.length} embeddings`);
      }
      
      return true;
    } catch (error) {
      console.error(`Failed to bulk store embeddings: ${error.message}`);
      return false;
    }
  }
  
  /**
   * Import embeddings from a JSON file
   * @param {string} filePath - Path to JSON file with embeddings
   */
  async importEmbeddingsFromFile(filePath) {
    try {
      const fileContent = await fs.readFile(filePath, 'utf8');
      const embeddingsData = JSON.parse(fileContent);
      
      // Convert object format to array format with IDs
      const embeddingsArray = Object.entries(embeddingsData).map(([id, vector]) => ({
        id,
        vector,
        metadata: {
          source: id,
          importedFrom: path.basename(filePath)
        }
      }));
      
      console.log(`Importing ${embeddingsArray.length} embeddings from ${filePath}`);
      
      return await this.bulkStoreEmbeddings(embeddingsArray);
    } catch (error) {
      console.error(`Failed to import embeddings from file: ${error.message}`);
      return false;
    }
  }
  
  /**
   * Find similar documents by vector similarity
   * @param {Array<number>} queryVector - Query embedding vector
   * @param {Object} options - Search options
   * @param {number} options.topK - Number of results to return
   * @param {Object} options.filter - Metadata filters
   * @returns {Array<Object>} Array of similar documents with scores
   */
  async findSimilar(queryVector, options = {}) {
    await this.initialize();
    
    const searchOptions = Object.assign({
      topK: 5,
      filter: {}
    }, options);
    
    try {
      switch(this.options.dbType) {
        case 'qdrant':
          return await this.findSimilarQdrant(queryVector, searchOptions);
        case 'memory':
        default:
          return this.findSimilarMemory(queryVector, searchOptions);
      }
    } catch (error) {
      console.error(`Error during similarity search: ${error.message}`);
      return [];
    }
  }
  
  /**
   * Find similar documents in Qdrant
   */
  async findSimilarQdrant(queryVector, options) {
    // Build filter if needed
    let filter = null;
    
    if (Object.keys(options.filter).length > 0) {
      // Convert simple filters to Qdrant filter format
      filter = { must: [] };
      
      Object.entries(options.filter).forEach(([key, value]) => {
        if (Array.isArray(value)) {
          // Handle array values (OR condition)
          filter.must.push({
            should: value.map(val => ({
              key: `metadata.${key}`,
              match: { value: val }
            }))
          });
        } else {
          // Handle single value
          filter.must.push({
            key: `metadata.${key}`,
            match: { value }
          });
        }
      });
    }
    
    // Query Qdrant
    const result = await this.client.search(this.options.collectionName, {
      vector: queryVector,
      limit: options.topK,
      filter,
      with_payload: true
    });
    
    // Format results to match our standard interface
    return result.map(match => ({
      id: match.id,
      score: match.score,
      metadata: match.payload.metadata
    }));
  }
  
  /**
   * Find similar documents in memory
   */
  findSimilarMemory(queryVector, options) {
    // Calculate cosine similarity for all vectors
    const results = Object.entries(this.memoryIndex).map(([id, entry]) => {
      const similarity = this.cosineSimilarity(queryVector, entry.vector);
      return {
        id,
        score: similarity,
        metadata: entry.metadata
      };
    });
    
    // Apply filters if needed
    let filtered = results;
    if (Object.keys(options.filter).length > 0) {
      filtered = results.filter(item => {
        return Object.entries(options.filter).every(([key, value]) => {
          // Handle array values (OR condition)
          if (Array.isArray(value)) {
            return value.includes(item.metadata[key]);
          }
          // Handle single value
          return item.metadata[key] === value;
        });
      });
    }
    
    // Sort by similarity score (descending) and take top K
    return filtered
      .sort((a, b) => b.score - a.score)
      .slice(0, options.topK);
  }
  
  /**
   * Calculate cosine similarity between two vectors
   */
  cosineSimilarity(vecA, vecB) {
    if (!vecA || !vecB || vecA.length !== vecB.length) return 0;
    
    let dotProduct = 0;
    let normA = 0;
    let normB = 0;
    
    for (let i = 0; i < vecA.length; i++) {
      dotProduct += vecA[i] * vecB[i];
      normA += vecA[i] * vecA[i];
      normB += vecB[i] * vecB[i];
    }
    
    // Handle division by zero
    if (normA === 0 || normB === 0) return 0;
    
    return dotProduct / (Math.sqrt(normA) * Math.sqrt(normB));
  }
  
  /**
   * Delete embeddings from the database
   * @param {string|Array<string>} ids - ID or array of IDs to delete
   */
  async deleteEmbeddings(ids) {
    await this.initialize();
    
    const idArray = Array.isArray(ids) ? ids : [ids];
    
    try {
      switch(this.options.dbType) {
        case 'qdrant':
          await this.client.delete(this.options.collectionName, {
            wait: true,
            points: idArray.map(id => typeof id === 'string' ? id : String(id))
          });
          break;
        case 'memory':
        default:
          idArray.forEach(id => {
            delete this.memoryIndex[id];
          });
      }
      
      console.log(`Deleted ${idArray.length} embeddings`);
      return true;
    } catch (error) {
      console.error(`Failed to delete embeddings: ${error.message}`);
      return false;
    }
  }
  
  /**
   * Clear all embeddings in the database
   */
  async clearAllEmbeddings() {
    await this.initialize();
    
    try {
      switch(this.options.dbType) {
        case 'qdrant':
          // Recreate the collection is the fastest way to clear it
          await this.client.deleteCollection(this.options.collectionName);
          this.initialized = false;
          await this.initialize(); // Reinitialize
          break;
        case 'memory':
        default:
          this.memoryIndex = {};
      }
      
      console.log('Cleared all embeddings');
      return true;
    } catch (error) {
      console.error(`Failed to clear embeddings: ${error.message}`);
      return false;
    }
  }
  
  /**
   * Get database stats
   */
  async getStats() {
    await this.initialize();
    
    try {
      switch(this.options.dbType) {
        case 'qdrant':
          const collInfo = await this.client.getCollection(this.options.collectionName);
          return {
            vectorCount: collInfo.vectors_count,
            status: collInfo.status,
            dimensions: this.options.dimensions,
            dbType: 'qdrant',
            memoryUsage: collInfo.disk_data_size || 0
          };
        case 'memory':
        default:
          return {
            vectorCount: Object.keys(this.memoryIndex).length,
            status: 'active',
            dimensions: this.options.dimensions,
            dbType: 'memory',
            memoryUsage: JSON.stringify(this.memoryIndex).length
          };
      }
    } catch (error) {
      console.error(`Failed to get database stats: ${error.message}`);
      return {
        status: 'error',
        error: error.message
      };
    }
  }
}

module.exports = VectorDatabaseAdapter;
