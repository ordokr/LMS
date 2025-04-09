/**
 * Synchronization Monitoring Dashboard Service
 * 
 * Provides real-time monitoring and management capabilities for the
 * Canvas-Discourse integration synchronization process.
 */

const express = require('express');
const http = require('http');
const socketio = require('socket.io');
const { SyncTransaction } = require('../integration/sync_transaction');
const { SyncState } = require('../integration/sync_state');
const { SyncService } = require('../integration/sync_service');

class SyncDashboard {
  constructor(port = 3030) {
    this.port = port;
    this.app = express();
    this.server = http.createServer(this.app);
    this.io = socketio(this.server);
    
    this.setupRoutes();
    this.setupWebSockets();
    this.setupMetricsCollection();
  }
  
  /**
   * Initialize the dashboard service
   */
  async start() {
    this.server.listen(this.port, () => {
      console.log(`Sync monitoring dashboard running on port ${this.port}`);
    });
    
    // Start collecting metrics
    this.startMetricsCollection();
  }
  
  /**
   * Set up Express routes for the dashboard API
   */
  setupRoutes() {
    // Serve static dashboard files
    this.app.use(express.static('public/dashboard'));
    this.app.use(express.json());
    
    // API endpoints for synchronization data
    this.app.get('/api/sync/status', async (req, res) => {
      try {
        const syncState = new SyncState();
        const status = await syncState.getOverallStatus();
        res.json(status);
      } catch (error) {
        res.status(500).json({ error: error.message });
      }
    });
    
    this.app.get('/api/sync/transactions', async (req, res) => {
      try {
        const syncTransaction = new SyncTransaction();
        const transactions = await syncTransaction.getRecentTransactions(20);
        res.json(transactions);
      } catch (error) {
        res.status(500).json({ error: error.message });
      }
    });
    
    this.app.get('/api/sync/entities/:type', async (req, res) => {
      try {
        const syncState = new SyncState();
        const entities = await syncState.getEntitiesByType(req.params.type);
        res.json(entities);
      } catch (error) {
        res.status(500).json({ error: error.message });
      }
    });
    
    // Endpoint to trigger manual sync for an entity
    this.app.post('/api/sync/trigger', async (req, res) => {
      try {
        const { entityType, entityId, priority } = req.body;
        const syncService = new SyncService();
        await syncService.publishSyncEvent({
          entityType,
          entityId,
          operation: 'SYNC',
          priority: priority || 'high'
        });
        res.json({ success: true, message: `Sync triggered for ${entityType} ${entityId}` });
      } catch (error) {
        res.status(500).json({ error: error.message });
      }
    });
  }
  
  /**
   * Set up WebSocket connections for real-time updates
   */
  setupWebSockets() {
    this.io.on('connection', (socket) => {
      console.log('Client connected to sync dashboard');
      
      // Send initial data
      this.sendInitialData(socket);
      
      // Listen for client requests
      socket.on('requestTransactions', async () => {
        const syncTransaction = new SyncTransaction();
        const transactions = await syncTransaction.getRecentTransactions(20);
        socket.emit('transactions', transactions);
      });
      
      socket.on('disconnect', () => {
        console.log('Client disconnected from sync dashboard');
      });
    });
  }
  
  /**
   * Send initial data to newly connected clients
   */
  async sendInitialData(socket) {
    try {
      const syncState = new SyncState();
      const status = await syncState.getOverallStatus();
      socket.emit('syncStatus', status);
      
      const syncTransaction = new SyncTransaction();
      const transactions = await syncTransaction.getRecentTransactions(20);
      socket.emit('transactions', transactions);
      
      const metrics = this.getLatestMetrics();
      socket.emit('metrics', metrics);
    } catch (error) {
      console.error('Error sending initial data:', error);
    }
  }
  
  /**
   * Set up metrics collection system
   */
  setupMetricsCollection() {
    this.metrics = {
      syncCount: 0,
      errorCount: 0,
      latency: [],
      queueSizes: {
        critical: 0,
        high: 0,
        background: 0
      },
      entityCounts: {
        users: 0,
        courses: 0,
        assignments: 0,
        discussions: 0
      }
    };
  }
  
  /**
   * Start collecting metrics from various services
   */
  startMetricsCollection() {
    // Poll metrics every 5 seconds
    this.metricsInterval = setInterval(async () => {
      try {
        await this.collectMetrics();
        this.io.emit('metrics', this.getLatestMetrics());
      } catch (error) {
        console.error('Error collecting metrics:', error);
      }
    }, 5000);
  }
  
  /**
   * Collect current metrics from sync services
   */
  async collectMetrics() {
    try {
      const syncService = new SyncService();
      const queueStatus = await syncService.getQueueStatus();
      
      this.metrics.queueSizes = {
        critical: queueStatus.critical || 0,
        high: queueStatus.high || 0,
        background: queueStatus.background || 0
      };
      
      // Update sync counts
      const syncState = new SyncState();
      const stats = await syncState.getStatistics();
      this.metrics.syncCount = stats.totalSyncs || 0;
      this.metrics.errorCount = stats.errorCount || 0;
      
      // Update entity counts
      const entityStats = await syncState.getEntityStatistics();
      this.metrics.entityCounts = entityStats;
      
      // Calculate average latency from recent transactions
      const syncTransaction = new SyncTransaction();
      const recentTransactions = await syncTransaction.getRecentTransactions(100);
      const latencies = recentTransactions
        .filter(t => t.completedAt && t.startedAt)
        .map(t => new Date(t.completedAt) - new Date(t.startedAt));
      
      if (latencies.length > 0) {
        const avgLatency = latencies.reduce((sum, val) => sum + val, 0) / latencies.length;
        // Keep last 50 data points for graphing
        this.metrics.latency.push(avgLatency);
        if (this.metrics.latency.length > 50) {
          this.metrics.latency.shift();
        }
      }
    } catch (error) {
      console.error('Error collecting metrics:', error);
    }
  }
  
  /**
   * Get the latest metrics for sending to clients
   */
  getLatestMetrics() {
    return this.metrics;
  }
  
  /**
   * Stop the dashboard service
   */
  async stop() {
    if (this.metricsInterval) {
      clearInterval(this.metricsInterval);
    }
    
    return new Promise((resolve) => {
      this.server.close(() => {
        console.log('Sync monitoring dashboard stopped');
        resolve();
      });
    });
  }
}

module.exports = { SyncDashboard };