/**
 * RAG Retriever
 * Handles query processing and document retrieval for RAG
 */
const path = require('path');
const fs = require('fs-extra');

class RagRetriever {
  /**
   * Create a new RAG retriever
   * @param {Object} vectorDB - Vector database adapter
   * @param {Object} options - Configuration options
   */
  constructor(vectorDB, options = {}) {
    this.vectorDB = vectorDB;
    this.options = Object.assign({
      topK: 5,
      reranking: true,
      minScore: 0.7,
      keywordBoost: true,
      recencyBoost: true,
      chunkMerging: true,
      ragDir: 'rag_knowledge_base',
      encoder: null
    }, options);
    
    this.encoder = this.options.encoder;
    this.initialized = false;
  }
  
  /**
   * Initialize the retriever
   */
  async initialize() {
    if (this.initialized) return;
    
    try {
      // Load encoder model if needed
      if (!this.encoder) {
        try {
          // First check if we should use the MLAnalyzer
          if (this.options.mlAnalyzer && 
              typeof this.options.mlAnalyzer.generateDocumentEmbedding === 'function') {
            console.log("Using MLAnalyzer for embedding generation");
            this.encoder = {
              embed: async (text) => {
                const embedding = await this.options.mlAnalyzer.generateDocumentEmbedding(text);
                return { array: async () => [embedding] };
              }
            };
          } else {
            // Try to load TensorFlow.js first
            console.log("Loading TensorFlow.js...");
            try {
              // Try to load tfjs-node first (faster)
              require('@tensorflow/tfjs-node');
            } catch (err) {
              console.log("TensorFlow.js Node not available, using browser version");
              require('@tensorflow/tfjs');
            }
            
            console.log("Loading Universal Sentence Encoder...");
            this.encoder = await require('@tensorflow-models/universal-sentence-encoder').load();
            console.log("Universal Sentence Encoder loaded successfully");
          }
        } catch (err) {
          console.error("Error loading encoder for RAG:", err.message);
          console.log("Using fallback embedding method");
          // Very simple fallback embedding (not for production use)
          this.encoder = {
            embed: async (text) => {
              // Simple hash function to generate pseudo-embeddings
              const hash = [...text].reduce((acc, char) => {
                return ((acc << 5) - acc) + char.charCodeAt(0);
              }, 0);
              
              // Generate a deterministic but simple vector based on the hash
              const vector = new Array(512).fill(0).map((_, i) => {
                return Math.sin(hash * (i + 1) * 0.01) * 0.5 + 0.5;
              });
              
              return { array: async () => [vector] };
            }
          };
        }
      }
      
      this.initialized = true;
    } catch (err) {
      console.error("Failed to initialize RAG retriever:", err.message);
      throw err;
    }
  }
  
  /**
   * Generate embedding for query text
   * @param {string} query - Query text
   * @returns {Array<number>} Embedding vector
   */
  async generateQueryEmbedding(query) {
    try {
      const embedding = await this.encoder.embed(query);
      const embeddingArray = await embedding.array();
      return embeddingArray[0];
    } catch (err) {
      console.error("Error generating query embedding:", err.message);
      return null;
    }
  }
  
  /**
   * Retrieve relevant documents for a query
   * @param {string} query - User query
   * @param {Object} options - Query options
   * @returns {Array<Object>} Relevant documents
   */
  async retrieveDocuments(query, options = {}) {
    await this.initialize();
    
    const queryOptions = { ...this.options, ...options };
    
    console.log(`Retrieving documents for query: "${query}"`);
    
    // Generate query embedding
    const queryEmbedding = await this.generateQueryEmbedding(query);
    if (!queryEmbedding) {
      throw new Error("Failed to generate query embedding");
    }
    
    // Apply filters
    const filters = this.buildFilters(queryOptions.filters);
    
    // Retrieve similar documents
    const results = await this.vectorDB.findSimilar(
      queryEmbedding, 
      {
        topK: queryOptions.topK * (queryOptions.reranking ? 2 : 1), // Get more results for reranking
        filter: filters
      }
    );
    
    if (results.length === 0) {
      console.log("No documents found for query");
      return [];
    }
    
    console.log(`Found ${results.length} initial matches`);
    
    // Rerank results if enabled
    const finalResults = queryOptions.reranking ? 
      await this.rerankResults(query, results, queryOptions) : 
      results;
    
    // Filter by minimum score
    const filteredResults = finalResults.filter(
      result => result.score >= queryOptions.minScore
    );
    
    // Fetch document content if not included in results
    const resultsWithContent = await this.fetchDocumentContent(filteredResults);
    
    // Merge chunks from the same document if enabled
    const mergedResults = queryOptions.chunkMerging ? 
      this.mergeDocumentChunks(resultsWithContent) : 
      resultsWithContent;
    
    console.log(`Retrieved ${mergedResults.length} relevant documents`);
    return mergedResults;
  }
  
  /**
   * Build filter expression
   * @param {Object} filters - Filter criteria
   * @returns {Object} Database-specific filters
   */
  buildFilters(filters = {}) {
    if (!filters || Object.keys(filters).length === 0) return {};
    
    // Convert simple filters object to database-specific format
    const dbFilters = {};
    
    // Handle system filter specifically
    if (filters.system) {
      dbFilters["system"] = Array.isArray(filters.system) ? 
        filters.system : [filters.system];
    }
    
    // Handle category filter
    if (filters.category) {
      dbFilters["category"] = Array.isArray(filters.category) ? 
        filters.category : [filters.category];
    }
    
    // Handle other filters
    for (const [key, value] of Object.entries(filters)) {
      if (key !== 'system' && key !== 'category') {
        dbFilters[key] = value;
      }
    }
    
    return dbFilters;
  }
  
  /**
   * Rerank results using additional heuristics
   * @param {string} query - Original query
   * @param {Array<Object>} results - Initial results
   * @param {Object} options - Reranking options
   * @returns {Array<Object>} Reranked results
   */
  async rerankResults(query, results, options) {
    // Extract keywords from query
    const keywords = this.extractKeywords(query);
    
    // Score results with hybrid approach
    const scoredResults = results.map(result => {
      let finalScore = result.score; // Start with vector similarity score
      
      // Apply keyword boosting if enabled
      if (options.keywordBoost) {
        let keywordBoost = 0;
        
        // Check how many keywords match in the content
        const resultText = JSON.stringify(result.metadata).toLowerCase();
        keywords.forEach(keyword => {
          if (resultText.includes(keyword.toLowerCase())) {
            keywordBoost += 0.02; // Small boost per keyword match
          }
        });
        
        finalScore += keywordBoost;
      }
      
      // Apply recency boost if enabled and lastUpdated is available
      if (options.recencyBoost && result.metadata.lastUpdated) {
        const age = (Date.now() - new Date(result.metadata.lastUpdated).getTime()) / (1000 * 60 * 60 * 24); // Age in days
        const recencyBoost = Math.max(0, 0.05 * (1 - Math.min(age / 30, 1))); // Up to 0.05 boost for recent docs
        finalScore += recencyBoost;
      }
      
      // Boost system-specific content if specified in query
      const systemMatches = {
        canvas: /canvas|lms|course|learning/i,
        discourse: /discourse|forum|topic|discussion/i,
        integration: /integrat|connect|combine/i
      };
      
      for (const [system, pattern] of Object.entries(systemMatches)) {
        if (pattern.test(query) && result.metadata.system === system) {
          finalScore += 0.03; // Boost for matching system context
        }
      }
      
      return {
        ...result,
        originalScore: result.score,
        score: Math.min(finalScore, 1.0) // Cap at 1.0
      };
    });
    
    // Sort by new score and return top K
    return scoredResults
      .sort((a, b) => b.score - a.score)
      .slice(0, options.topK);
  }
  
  /**
   * Extract keywords from query
   * @param {string} query - User query
   * @returns {Array<string>} Extracted keywords
   */
  extractKeywords(query) {
    // Simple keyword extraction - remove stopwords and extract meaningful terms
    const stopwords = new Set([
      'the', 'and', 'or', 'a', 'an', 'in', 'on', 'at', 'to', 'for', 
      'with', 'by', 'about', 'as', 'how', 'what', 'when', 'where', 
      'who', 'which', 'why', 'can', 'could', 'should', 'would', 'do',
      'does', 'did', 'i', 'you', 'he', 'she', 'we', 'they', 'it'
    ]);
    
    return query
      .toLowerCase()
      .split(/\W+/)
      .filter(word => word.length > 2 && !stopwords.has(word));
  }
  
  /**
   * Fetch document content for results
   * @param {Array<Object>} results - Query results
   * @returns {Array<Object>} Results with content
   */
  async fetchDocumentContent(results) {
    // For results that don't already have content, load it from files
    const resultsWithContent = [];
    
    for (const result of results) {
      let content = null;
      
      // If metadata contains the source file, load content from there
      if (result.metadata && result.metadata.source) {
        try {
          content = await fs.readFile(result.metadata.source, 'utf8');
        } catch (error) {
          console.error(`Error loading content for ${result.id}: ${error.message}`);
          // Try alternative source path
          try {
            const altPath = path.join(this.options.ragDir, result.id);
            content = await fs.readFile(altPath, 'utf8');
          } catch (innerError) {
            // If still can't find content, leave as null
          }
        }
      }
      
      // Add result with content
      resultsWithContent.push({
        ...result,
        content
      });
    }
    
    return resultsWithContent;
  }
  
  /**
   * Merge chunks from the same document
   * @param {Array<Object>} results - Query results with content
   * @returns {Array<Object>} Merged results
   */
  mergeDocumentChunks(results) {
    // Group results by source document
    const documentGroups = {};
    
    results.forEach(result => {
      // Extract document ID without chunk information
      const documentId = result.id.split('_chunk')[0];
      
      if (!documentGroups[documentId]) {
        documentGroups[documentId] = {
          id: documentId,
          score: result.score, // Use highest score
          metadata: { ...result.metadata },
          chunks: []
        };
        
        // Remove chunk-specific metadata
        if (documentGroups[documentId].metadata.chunkIndex !== undefined) {
          delete documentGroups[documentId].metadata.chunkIndex;
          delete documentGroups[documentId].metadata.totalChunks;
        }
      } else if (result.score > documentGroups[documentId].score) {
        // Keep the highest score among chunks
        documentGroups[documentId].score = result.score;
      }
      
      // Add this chunk
      documentGroups[documentId].chunks.push({
        id: result.id,
        content: result.content,
        score: result.score,
        metadata: result.metadata
      });
    });
    
    // Convert groups back to results array
    return Object.values(documentGroups).map(group => {
      // Sort chunks by their original order if available
      group.chunks.sort((a, b) => {
        const indexA = a.metadata.chunkIndex !== undefined ? a.metadata.chunkIndex : 0;
        const indexB = b.metadata.chunkIndex !== undefined ? b.metadata.chunkIndex : 0;
        return indexA - indexB;
      });
      
      // Combine content from all chunks
      const content = group.chunks.map(chunk => chunk.content).join('\n\n');
      
      return {
        id: group.id,
        score: group.score,
        metadata: group.metadata,
        content,
        chunks: group.chunks
      };
    }).sort((a, b) => b.score - a.score);
  }
  
  /**
   * Search for specific information and generate answers
   * @param {string} query - User query
   * @param {Object} options - Search options
   * @returns {Object} Search results with answer
   */
  async search(query, options = {}) {
    const documents = await this.retrieveDocuments(query, options);
    
    return {
      query,
      timestamp: new Date().toISOString(),
      resultsCount: documents.length,
      documents
    };
  }
  
  /**
   * Generate context for an LLM from retrieved documents
   * @param {Array<Object>} documents - Retrieved documents
   * @param {Object} options - Context generation options
   * @returns {string} Formatted context
   */
  generateLlmContext(documents, options = {}) {
    const contextOptions = Object.assign({
      maxContextLength: 10000,
      includeMetadata: true,
      formatType: 'markdown'
    }, options);
    
    let context = '';
    let currentLength = 0;
    
    // Sort documents by score (highest first)
    const sortedDocs = [...documents].sort((a, b) => b.score - a.score);
    
    for (const doc of sortedDocs) {
      // Format document content based on format type
      let docContext = '';
      
      if (contextOptions.formatType === 'markdown') {
        docContext += `## ${doc.id}\n\n`;
        
        if (contextOptions.includeMetadata && doc.metadata) {
          docContext += `**Source:** ${doc.metadata.source || 'Unknown'}\n`;
          docContext += `**System:** ${doc.metadata.system || 'Unknown'}\n`;
          docContext += `**Category:** ${doc.metadata.category || 'General'}\n`;
          docContext += `**Relevance Score:** ${doc.score.toFixed(2)}\n\n`;
        }
        
        docContext += `${doc.content || 'No content available'}\n\n`;
        docContext += `---\n\n`;
      } else {
        // Plain text format
        docContext += `Source: ${doc.id}\n`;
        if (contextOptions.includeMetadata && doc.metadata) {
          docContext += `Source: ${doc.metadata.source || 'Unknown'}\n`;
        }
        docContext += `\n${doc.content || 'No content available'}\n\n`;
        docContext += `----------\n\n`;
      }
      
      // Check if adding this document would exceed max context length
      if (currentLength + docContext.length > contextOptions.maxContextLength) {
        // If we already have some context, stop here
        if (currentLength > 0) break;
        
        // If this is the first document and it's too long, truncate it
        docContext = docContext.substring(0, contextOptions.maxContextLength);
      }
      
      context += docContext;
      currentLength += docContext.length;
    }
    
    return context;
  }
}

module.exports = RagRetriever;