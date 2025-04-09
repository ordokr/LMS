/**
 * Base class for all analysis modules
 * Provides a standardized interface that all analyzers should implement
 */
class AnalysisModule {
  /**
   * Create a new analysis module
   * @param {Object} metrics - The metrics object to update
   * @param {Object} config - Configuration options
   */
  constructor(metrics, config = {}) {
    this.metrics = metrics;
    this.config = config;
    this.initialized = false;
  }

  /**
   * Initialize the module
   * @returns {Promise<void>}
   */
  async initialize() {
    this.initialized = true;
    return Promise.resolve();
  }

  /**
   * Execute the analysis
   * @param {Object} context - Analysis context (files, ASTs, etc.)
   * @returns {Promise<Object>} - Analysis results
   */
  async analyze(context) {
    if (!this.initialized) {
      await this.initialize();
    }
    return Promise.resolve({});
  }

  /**
   * Get analysis results
   * @returns {Object} - Analysis results
   */
  getResults() {
    return {};
  }

  /**
   * Clean up resources
   * @returns {Promise<void>}
   */
  async cleanup() {
    return Promise.resolve();
  }

  /**
   * Get the name of this analyzer
   * @returns {string} - Analyzer name
   */
  getName() {
    return this.constructor.name;
  }
}

module.exports = AnalysisModule;
