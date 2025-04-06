/**
 * Simple logger implementation for the Canvas-Discourse integration
 */
function createLogger(moduleName) {
  const loggerPrefix = `[${moduleName}]`;
  
  return {
    /**
     * Log an info message
     * @param {string} message - Message to log
     * @param {Object} [context={}] - Optional context data
     */
    info(message, context = {}) {
      console.log(`[INFO] ${loggerPrefix} ${message}`, context);
    },
    
    /**
     * Log an error message
     * @param {string} message - Message to log
     * @param {Object} [context={}] - Optional context data
     */
    error(message, context = {}) {
      console.error(`[ERROR] ${loggerPrefix} ${message}`, context);
    },
    
    /**
     * Log a warning message
     * @param {string} message - Message to log
     * @param {Object} [context={}] - Optional context data
     */
    warn(message, context = {}) {
      console.warn(`[WARN] ${loggerPrefix} ${message}`, context);
    },
    
    /**
     * Log a debug message
     * @param {string} message - Message to log
     * @param {Object} [context={}] - Optional context data
     */
    debug(message, context = {}) {
      if (process.env.DEBUG) {
        console.debug(`[DEBUG] ${loggerPrefix} ${message}`, context);
      }
    }
  };
}

module.exports = { createLogger };