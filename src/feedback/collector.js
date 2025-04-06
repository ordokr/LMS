const fs = require('fs').promises;
const path = require('path');
const { createLogger } = require('../utils/logger');

class FeedbackCollector {
  constructor(options = {}) {
    this.storagePath = options.storagePath || path.join(__dirname, '../../data/feedback');
    this.logger = createLogger('feedback-collector');
    this.categories = [
      'ui',
      'performance',
      'features',
      'bugs',
      'general'
    ];
    
    // Don't await this in the constructor - it returns a promise
    // but we don't want to make the constructor async
    this.initializeStorage();
  }
  
  async initializeStorage() {
    try {
      await fs.mkdir(this.storagePath, { recursive: true });
      this.logger.info(`Feedback storage directory created at ${this.storagePath}`);
      
      // Create some sample feedback data for testing
      await this.createSampleFeedback();
    } catch (error) {
      this.logger.error(`Failed to initialize feedback storage: ${error.message}`);
    }
  }
  
  async createSampleFeedback() {
    // Create some sample feedback entries if directory is empty
    try {
      const files = await fs.readdir(this.storagePath);
      if (files.length === 0) {
        this.logger.info("Creating sample feedback data...");
        
        const sampleFeedback = [
          {
            id: "feedback_20250404T120000_abc12",
            userId: "user123",
            category: "ui",
            content: "The discussion integration has greatly improved workflow for students.",
            rating: 5,
            timestamp: "2025-04-04T12:00:00Z",
            status: "reviewed"
          },
          {
            id: "feedback_20250404T130000_def34",
            userId: "user456",
            category: "performance",
            content: "Sometimes the forum posts take too long to sync with Canvas.",
            rating: 3,
            timestamp: "2025-04-04T13:00:00Z",
            status: "new"
          },
          {
            id: "feedback_20250404T140000_ghi56",
            userId: "user789",
            category: "features",
            content: "Would love to see better integration with assignment submissions.",
            rating: 4,
            timestamp: "2025-04-04T14:00:00Z", 
            status: "in-progress"
          }
        ];
        
        for (const feedback of sampleFeedback) {
          await fs.writeFile(
            path.join(this.storagePath, `${feedback.id}.json`),
            JSON.stringify(feedback, null, 2)
          );
        }
        
        this.logger.info(`Created ${sampleFeedback.length} sample feedback items`);
      }
    } catch (error) {
      this.logger.error(`Failed to create sample feedback: ${error.message}`);
    }
  }
  
  /**
   * Store user feedback
   * @param {Object} feedback - The feedback data
   * @param {string} feedback.userId - User identifier
   * @param {string} feedback.category - Feedback category
   * @param {string} feedback.content - Feedback content
   * @param {number} feedback.rating - Optional rating (1-5)
   * @returns {Promise<Object>} - The stored feedback with ID
   */
  async storeFeedback(feedback) {
    if (!feedback.userId) {
      throw new Error('User ID is required');
    }
    
    if (!feedback.content) {
      throw new Error('Feedback content is required');
    }
    
    if (!this.categories.includes(feedback.category)) {
      throw new Error(`Category must be one of: ${this.categories.join(', ')}`);
    }
    
    const timestamp = new Date().toISOString();
    const id = `feedback_${timestamp.replace(/[:.]/g, '')}_${Math.random().toString(36).substr(2, 5)}`;
    
    const feedbackData = {
      id,
      userId: feedback.userId,
      category: feedback.category,
      content: feedback.content,
      rating: feedback.rating || null,
      timestamp,
      status: 'new'
    };
    
    try {
      await fs.mkdir(this.storagePath, { recursive: true });
      await fs.writeFile(
        path.join(this.storagePath, `${id}.json`),
        JSON.stringify(feedbackData, null, 2)
      );
      
      this.logger.info(`Feedback stored with ID ${id}`, { category: feedback.category });
      return feedbackData;
    } catch (error) {
      this.logger.error(`Failed to store feedback: ${error.message}`);
      throw new Error('Failed to store feedback');
    }
  }
  
  /**
   * Get all feedback items, optionally filtered by category
   * @param {Object} options - Filter options
   * @param {string} options.category - Optional category filter
   * @param {string} options.status - Optional status filter
   * @returns {Promise<Array>} - Array of feedback items
   */
  async getAllFeedback(options = {}) {
    try {
      // First ensure the directory exists
      await fs.mkdir(this.storagePath, { recursive: true });
      
      const files = await fs.readdir(this.storagePath);
      const feedbackItems = [];
      
      for (const file of files) {
        if (!file.endsWith('.json')) continue;
        
        const content = await fs.readFile(path.join(this.storagePath, file), 'utf8');
        const feedback = JSON.parse(content);
        
        if (options.category && feedback.category !== options.category) continue;
        if (options.status && feedback.status !== options.status) continue;
        
        feedbackItems.push(feedback);
      }
      
      return feedbackItems.sort((a, b) => new Date(b.timestamp) - new Date(a.timestamp));
    } catch (error) {
      this.logger.error(`Failed to get feedback items: ${error.message}`);
      return []; // Return empty array instead of throwing to avoid breaking dashboard generation
    }
  }
  
  /**
   * Generate a feedback summary by category
   * @returns {Promise<Object>} - Summary of feedback by category
   */
  async generateFeedbackSummary() {
    const feedback = await this.getAllFeedback();
    const summary = {
      totalItems: feedback.length,
      categoryCounts: {},
      averageRatings: {},
      recentFeedback: feedback.slice(0, 5)
    };
    
    // Initialize categories
    for (const category of this.categories) {
      summary.categoryCounts[category] = 0;
      summary.averageRatings[category] = { sum: 0, count: 0 };
    }
    
    // Populate summary
    for (const item of feedback) {
      summary.categoryCounts[item.category] = (summary.categoryCounts[item.category] || 0) + 1;
      
      if (item.rating) {
        summary.averageRatings[item.category].sum += item.rating;
        summary.averageRatings[item.category].count += 1;
      }
    }
    
    // Calculate averages
    for (const category of this.categories) {
      const { sum, count } = summary.averageRatings[category];
      summary.averageRatings[category] = count > 0 ? Math.round((sum / count) * 10) / 10 : null;
    }
    
    return summary;
  }
}

module.exports = { FeedbackCollector };