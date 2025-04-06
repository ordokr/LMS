const os = require('os');
const { createLogger } = require('../utils/logger');

class MetricsCollector {
  constructor() {
    this.metrics = new Map();
    this.logger = createLogger('metrics-collector');
    this.counters = new Map();
    this.timers = new Map();
    this.gauges = new Map();
    
    // Initialize system metrics
    this.initializeSystemMetrics();
  }
  
  initializeSystemMetrics() {
    // Add some initial system metrics
    this.gauge('system.memory.free', os.freemem());
    this.gauge('system.memory.total', os.totalmem());
    this.gauge('system.load.1m', os.loadavg()[0]);
    
    // Add sample API metrics for demonstration
    this.timing('api.canvas.courses.get', 245);
    this.timing('api.canvas.announcements.list', 189);
    this.timing('api.discourse.topics.create', 320);
    this.timing('integration.announcements-sync', 550);
    
    this.increment('api.success.count', 42);
    this.increment('api.error.count', 3);
  }
  
  increment(metricName, value = 1) {
    const current = this.counters.get(metricName) || 0;
    this.counters.set(metricName, current + value);
    return current + value;
  }
  
  decrement(metricName, value = 1) {
    const current = this.counters.get(metricName) || 0;
    this.counters.set(metricName, current - value);
    return current - value;
  }
  
  gauge(metricName, value) {
    this.gauges.set(metricName, value);
    return value;
  }
  
  timing(metricName, timeMs) {
    const timings = this.timers.get(metricName) || [];
    timings.push(timeMs);
    // Keep only the last 100 timings
    if (timings.length > 100) {
      timings.shift();
    }
    this.timers.set(metricName, timings);
    return timeMs;
  }
  
  getMetricSummary(metricName) {
    if (this.counters.has(metricName)) {
      return { type: 'counter', value: this.counters.get(metricName) };
    }
    
    if (this.gauges.has(metricName)) {
      return { type: 'gauge', value: this.gauges.get(metricName) };
    }
    
    if (this.timers.has(metricName)) {
      const timings = this.timers.get(metricName);
      return {
        type: 'timer',
        count: timings.length,
        min: Math.min(...timings),
        max: Math.max(...timings),
        avg: timings.reduce((sum, t) => sum + t, 0) / timings.length
      };
    }
    
    return null;
  }
  
  getAllMetrics() {
    const metrics = {};
    
    for (const [name, value] of this.counters.entries()) {
      metrics[name] = { type: 'counter', value };
    }
    
    for (const [name, value] of this.gauges.entries()) {
      metrics[name] = { type: 'gauge', value };
    }
    
    for (const [name, timings] of this.timers.entries()) {
      if (timings && timings.length > 0) {
        metrics[name] = {
          type: 'timer',
          count: timings.length,
          min: Math.min(...timings),
          max: Math.max(...timings),
          avg: timings.reduce((sum, t) => sum + t, 0) / timings.length
        };
      }
    }
    
    return metrics;
  }
}

// Create a singleton instance
const metrics = new MetricsCollector();

module.exports = { metrics, MetricsCollector };