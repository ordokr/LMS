const { createLogger } = require('../utils/logger');
const { metrics } = require('./metrics');

class PerformanceMonitor {
  constructor(options = {}) {
    this.logger = createLogger('performance-monitor');
    this.thresholds = {
      apiResponseTime: options.apiResponseTime || 2000, // 2 seconds
      integrationSyncTime: options.integrationSyncTime || 5000, // 5 seconds
      ...options.thresholds
    };
    this.metrics = metrics;
  }

  /**
   * Monitors an API call and logs performance metrics
   * @param {string} apiName - Name of the API being called
   * @param {Function} apiCall - The async function to monitor
   * @param {Object} context - Additional context information
   * @returns {Promise<*>} - The result of the API call
   */
  async monitorApiCall(apiName, apiCall, context = {}) {
    const startTime = Date.now();
    let success = false;
    let result;
    
    try {
      result = await apiCall();
      success = true;
      return result;
    } catch (error) {
      this.logger.error(`API call to ${apiName} failed`, { error: error.message, context });
      this.metrics.increment(`api.${apiName}.error`);
      throw error;
    } finally {
      const duration = Date.now() - startTime;
      this.metrics.timing(`api.${apiName}.duration`, duration);
      
      if (duration > this.thresholds.apiResponseTime) {
        this.logger.warn(`API call to ${apiName} exceeded threshold`, {
          duration,
          threshold: this.thresholds.apiResponseTime,
          context
        });
      }
      
      if (success) {
        this.metrics.increment(`api.${apiName}.success`);
      }
      
      this.logger.debug(`API call to ${apiName} completed`, {
        duration,
        success,
        context
      });
    }
  }

  /**
   * Monitors an integration process and logs performance metrics
   * @param {string} processName - Name of the integration process
   * @param {Function} process - The async function to monitor
   * @param {Object} context - Additional context information
   * @returns {Promise<*>} - The result of the process
   */
  async monitorIntegrationProcess(processName, process, context = {}) {
    const startTime = Date.now();
    let success = false;
    let result;
    
    try {
      result = await process();
      success = true;
      return result;
    } catch (error) {
      this.logger.error(`Integration process ${processName} failed`, { error: error.message, context });
      this.metrics.increment(`integration.${processName}.error`);
      throw error;
    } finally {
      const duration = Date.now() - startTime;
      this.metrics.timing(`integration.${processName}.duration`, duration);
      
      if (duration > this.thresholds.integrationSyncTime) {
        this.logger.warn(`Integration process ${processName} exceeded threshold`, {
          duration,
          threshold: this.thresholds.integrationSyncTime,
          context
        });
      }
      
      if (success) {
        this.metrics.increment(`integration.${processName}.success`);
      }
      
      this.logger.info(`Integration process ${processName} completed`, {
        duration,
        success,
        context
      });
    }
  }
}

module.exports = { PerformanceMonitor };