const { PerformanceMonitor } = require('./performance');
const { metrics } = require('./metrics');
const { createLogger } = require('../utils/logger');
const http = require('http');

const logger = createLogger('monitoring-service');
const performanceMonitor = new PerformanceMonitor();

// Create a simple metrics server to expose metrics
const server = http.createServer((req, res) => {
  if (req.url === '/metrics') {
    res.setHeader('Content-Type', 'application/json');
    res.end(JSON.stringify(metrics.getAllMetrics(), null, 2));
  } else if (req.url === '/health') {
    res.setHeader('Content-Type', 'application/json');
    res.end(JSON.stringify({ status: 'ok', timestamp: new Date().toISOString() }));
  } else {
    res.statusCode = 404;
    res.end('Not Found');
  }
});

// Start the metrics server
const PORT = process.env.MONITORING_PORT || 3001;
server.listen(PORT, () => {
  logger.info(`Monitoring server started on port ${PORT}`);
  logger.info('Available endpoints:');
  logger.info(`- Health check: http://localhost:${PORT}/health`);
  logger.info(`- Metrics: http://localhost:${PORT}/metrics`);
});

// Handle graceful shutdown
process.on('SIGTERM', () => {
  logger.info('SIGTERM received, shutting down monitoring server');
  server.close(() => {
    logger.info('Monitoring server closed');
    process.exit(0);
  });
});

// Initialize system monitoring
logger.info('Starting system monitoring');
metrics.initializeSystemMetrics();

// Export for testing/importing elsewhere
module.exports = { server, performanceMonitor, metrics };