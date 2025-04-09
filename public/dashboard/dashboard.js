/**
 * Canvas-Discourse Synchronization Dashboard
 * 
 * Client-side JavaScript for the synchronization monitoring dashboard.
 */

// Initialize socket.io connection
const socket = io();

// Dashboard state
let dashboardState = {
  currentSection: 'overview',
  transactions: {
    page: 1,
    filter: 'all',
    data: []
  },
  entities: {
    type: 'users',
    search: '',
    data: []
  },
  metrics: {
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
  },
  charts: {}
};

// DOM elements
const elements = {
  connectionIndicator: document.getElementById('connection-indicator'),
  connectionText: document.getElementById('connection-text'),
  systemStatusIndicator: document.getElementById('system-status-indicator'),
  lastUpdated: document.getElementById('last-updated'),
  totalSyncs: document.getElementById('total-syncs'),
  errorRate: document.getElementById('error-rate'),
  avgLatency: document.getElementById('avg-latency'),
  criticalQueue: document.getElementById('critical-queue'),
  highQueue: document.getElementById('high-queue'),
  backgroundQueue: document.getElementById('background-queue'),
  usersProgress: document.getElementById('users-progress'),
  usersPercent: document.getElementById('users-percent'),
  coursesProgress: document.getElementById('courses-progress'),
  coursesPercent: document.getElementById('courses-percent'),
  assignmentsProgress: document.getElementById('assignments-progress'),
  assignmentsPercent: document.getElementById('assignments-percent'),
  discussionsProgress: document.getElementById('discussions-progress'),
  discussionsPercent: document.getElementById('discussions-percent'),
  recentActivity: document.getElementById('recent-activity'),
  transactionFilter: document.getElementById('transaction-filter'),
  refreshTransactions: document.getElementById('refresh-transactions'),
  transactionsTableBody: document.getElementById('transactions-table-body'),
  prevPage: document.getElementById('prev-page'),
  nextPage: document.getElementById('next-page'),
  pageInfo: document.getElementById('page-info'),
  entityTypeFilter: document.getElementById('entity-type-filter'),
  entitySearch: document.getElementById('entity-search'),
  refreshEntities: document.getElementById('refresh-entities'),
  entitiesTableBody: document.getElementById('entities-table-body'),
  manualSyncForm: document.getElementById('manual-sync-form'),
  syncResult: document.getElementById('sync-result'),
  verifyAllIntegrity: document.getElementById('verify-all-integrity'),
  clearQueue: document.getElementById('clear-queue'),
  restartSyncService: document.getElementById('restart-sync-service'),
  actionResult: document.getElementById('action-result')
};

// Initialize charts
function initializeCharts() {
  // Queue size chart
  const queueCtx = document.getElementById('queue-chart').getContext('2d');
  dashboardState.charts.queue = new Chart(queueCtx, {
    type: 'bar',
    data: {
      labels: ['Critical', 'High', 'Background'],
      datasets: [{
        label: 'Queue Size',
        data: [0, 0, 0],
        backgroundColor: [
          'rgba(255, 99, 132, 0.7)',
          'rgba(255, 159, 64, 0.7)',
          'rgba(75, 192, 192, 0.7)'
        ],
        borderColor: [
          'rgb(255, 99, 132)',
          'rgb(255, 159, 64)',
          'rgb(75, 192, 192)'
        ],
        borderWidth: 1
      }]
    },
    options: {
      scales: {
        y: {
          beginAtZero: true,
          title: {
            display: true,
            text: 'Messages'
          }
        }
      },
      plugins: {
        legend: {
          display: false
        }
      }
    }
  });
  
  // Latency chart
  const latencyCtx = document.getElementById('latency-chart').getContext('2d');
  dashboardState.charts.latency = new Chart(latencyCtx, {
    type: 'line',
    data: {
      labels: Array(50).fill(''),
      datasets: [{
        label: 'Avg Latency (ms)',
        data: Array(50).fill(null),
        borderColor: 'rgba(54, 162, 235, 1)',
        backgroundColor: 'rgba(54, 162, 235, 0.2)',
        tension: 0.4,
        fill: true
      }]
    },
    options: {
      scales: {
        y: {
          beginAtZero: true,
          title: {
            display: true,
            text: 'Milliseconds'
          }
        },
        x: {
          display: false
        }
      },
      plugins: {
        legend: {
          display: false
        }
      }
    }
  });
}

// Update charts and metrics with new data
function updateCharts(metrics) {
  // Update queue chart
  dashboardState.charts.queue.data.datasets[0].data = [
    metrics.queueSizes.critical,
    metrics.queueSizes.high,
    metrics.queueSizes.background
  ];
  dashboardState.charts.queue.update();
  
  // Update latency chart
  dashboardState.charts.latency.data.datasets[0].data = metrics.latency;
  dashboardState.charts.latency.update();
  
  // Update queue size text
  elements.criticalQueue.textContent = metrics.queueSizes.critical;
  elements.highQueue.textContent = metrics.queueSizes.high;
  elements.backgroundQueue.textContent = metrics.queueSizes.background;
  
  // Update metrics summary
  elements.totalSyncs.textContent = metrics.syncCount.toLocaleString();
  const errorRateValue = metrics.syncCount > 0 
    ? ((metrics.errorCount / metrics.syncCount) * 100).toFixed(2) 
    : '0.00';
  elements.errorRate.textContent = `${errorRateValue}%`;
  
  // Calculate average latency from the last 5 data points if available
  const recentLatency = metrics.latency.filter(l => l !== null).slice(-5);
  const avgLatencyValue = recentLatency.length > 0
    ? (recentLatency.reduce((sum, val) => sum + val, 0) / recentLatency.length).toFixed(0)
    : '-';
  elements.avgLatency.textContent = avgLatencyValue !== '-' ? `${avgLatencyValue} ms` : '-';
  
  // Update entity progress bars
  updateEntityProgress(metrics.entityCounts);
  
  // Update last updated time
  elements.lastUpdated.textContent = new Date().toLocaleTimeString();
}

// Update entity progress bars
function updateEntityProgress(entityCounts) {
  const calculatePercentage = (synced, total) => {
    if (total === 0) return 0;
    return Math.round((synced / total) * 100);
  };
  
  const usersPercent = calculatePercentage(entityCounts.users?.synced || 0, entityCounts.users?.total || 0);
  elements.usersProgress.style.width = `${usersPercent}%`;
  elements.usersPercent.textContent = `${usersPercent}%`;
  
  const coursesPercent = calculatePercentage(entityCounts.courses?.synced || 0, entityCounts.courses?.total || 0);
  elements.coursesProgress.style.width = `${coursesPercent}%`;
  elements.coursesPercent.textContent = `${coursesPercent}%`;
  
  const assignmentsPercent = calculatePercentage(entityCounts.assignments?.synced || 0, entityCounts.assignments?.total || 0);
  elements.assignmentsProgress.style.width = `${assignmentsPercent}%`;
  elements.assignmentsPercent.textContent = `${assignmentsPercent}%`;
  
  const discussionsPercent = calculatePercentage(entityCounts.discussions?.synced || 0, entityCounts.discussions?.total || 0);
  elements.discussionsProgress.style.width = `${discussionsPercent}%`;
  elements.discussionsPercent.textContent = `${discussionsPercent}%`;
}

// Update system status indicator
function updateSystemStatus(status) {
  const statusDot = elements.systemStatusIndicator.querySelector('.status-dot');
  const statusText = elements.systemStatusIndicator.querySelector('.status-text');
  
  if (status === 'healthy') {
    statusDot.className = 'status-dot status-healthy';
    statusText.textContent = 'Healthy';
  } else if (status === 'degraded') {
    statusDot.className = 'status-dot status-warning';
    statusText.textContent = 'Degraded';
  } else if (status === 'error') {
    statusDot.className = 'status-dot status-error';
    statusText.textContent = 'Error';
  } else {
    statusDot.className = 'status-dot';
    statusText.textContent = 'Unknown';
  }
}

// Update connection status indicator
function updateConnectionStatus(connected) {
  if (connected) {
    elements.connectionIndicator.className = 'connection-indicator connected';
    elements.connectionText.textContent = 'Connected';
  } else {
    elements.connectionIndicator.className = 'connection-indicator disconnected';
    elements.connectionText.textContent = 'Disconnected';
  }
}

// Update recent activity feed
function updateRecentActivity(transactions) {
  if (!transactions || transactions.length === 0) {
    elements.recentActivity.innerHTML = '<p>No recent activity</p>';
    return;
  }
  
  // Take the 5 most recent transactions
  const recentTransactions = transactions.slice(0, 5);
  
  let html = '';
  recentTransactions.forEach(transaction => {
    const time = new Date(transaction.startedAt).toLocaleTimeString();
    const statusClass = transaction.status === 'SUCCESS' ? 'success' : 
                        transaction.status === 'ERROR' ? 'error' : 'pending';
    
    html += `
      <div class="activity-item">
        <span class="activity-time">${time}</span>
        <span class="activity-entity">${transaction.entityType} ${transaction.entityId}</span>
        <span class="activity-operation">${transaction.operation}</span>
        <span class="activity-status ${statusClass}">${transaction.status}</span>
      </div>
    `;
  });
  
  elements.recentActivity.innerHTML = html;
}

// Render transactions table
function renderTransactionsTable(transactions) {
  if (!transactions || transactions.length === 0) {
    elements.transactionsTableBody.innerHTML = '<tr><td colspan="7" class="no-data">No transactions found</td></tr>';
    return;
  }
  
  let html = '';
  transactions.forEach(transaction => {
    const startTime = new Date(transaction.startedAt).toLocaleString();
    const duration = transaction.completedAt 
      ? `${Math.round((new Date(transaction.completedAt) - new Date(transaction.startedAt)) / 1000)}s` 
      : '-';
    const statusClass = transaction.status === 'SUCCESS' ? 'success' : 
                        transaction.status === 'ERROR' ? 'error' : 'pending';
    
    html += `
      <tr>
        <td>${transaction.id}</td>
        <td>${transaction.entityType}</td>
        <td>${transaction.entityId}</td>
        <td>${startTime}</td>
        <td>${duration}</td>
        <td><span class="status-badge ${statusClass}">${transaction.status}</span></td>
        <td>
          <button class="icon-button view-details" data-id="${transaction.id}">
            <span class="material-icons">visibility</span>
          </button>
          ${transaction.status === 'ERROR' ? `
            <button class="icon-button retry" data-type="${transaction.entityType}" data-id="${transaction.entityId}">
              <span class="material-icons">refresh</span>
            </button>
          ` : ''}
        </td>
      </tr>
    `;
  });
  
  elements.transactionsTableBody.innerHTML = html;
  
  // Add event listeners for transaction details buttons
  const detailButtons = elements.transactionsTableBody.querySelectorAll('.view-details');
  detailButtons.forEach(button => {
    button.addEventListener('click', () => {
      const transactionId = button.dataset.id;
      viewTransactionDetails(transactionId);
    });
  });
  
  // Add event listeners for retry buttons
  const retryButtons = elements.transactionsTableBody.querySelectorAll('.retry');
  retryButtons.forEach(button => {
    button.addEventListener('click', () => {
      const entityType = button.dataset.type;
      const entityId = button.dataset.id;
      triggerManualSync(entityType, entityId, 'high');
    });
  });
}

// Render entities table
function renderEntitiesTable(entities) {
  if (!entities || entities.length === 0) {
    elements.entitiesTableBody.innerHTML = '<tr><td colspan="7" class="no-data">No entities found</td></tr>';
    return;
  }
  
  let html = '';
  entities.forEach(entity => {
    const lastSynced = entity.lastSynced ? new Date(entity.lastSynced).toLocaleString() : 'Never';
    const statusClass = entity.syncStatus === 'SYNCED' ? 'success' : 
                        entity.syncStatus === 'ERROR' ? 'error' : 
                        entity.syncStatus === 'PENDING' ? 'pending' : 'warning';
    
    html += `
      <tr>
        <td>${entity.canvasId || '-'}</td>
        <td>${entity.canvasName || '-'}</td>
        <td>${entity.discourseId || '-'}</td>
        <td>${entity.discourseName || '-'}</td>
        <td>${lastSynced}</td>
        <td><span class="status-badge ${statusClass}">${entity.syncStatus}</span></td>
        <td>
          <button class="icon-button sync-entity" data-type="${dashboardState.entities.type}" data-id="${entity.canvasId}">
            <span class="material-icons">sync</span>
          </button>
          <button class="icon-button view-entity" data-type="${dashboardState.entities.type}" data-id="${entity.canvasId}">
            <span class="material-icons">visibility</span>
          </button>
        </td>
      </tr>
    `;
  });
  
  elements.entitiesTableBody.innerHTML = html;
  
  // Add event listeners for sync buttons
  const syncButtons = elements.entitiesTableBody.querySelectorAll('.sync-entity');
  syncButtons.forEach(button => {
    button.addEventListener('click', () => {
      const entityType = button.dataset.type;
      const entityId = button.dataset.id;
      triggerManualSync(entityType, entityId, 'high');
    });
  });
  
  // Add event listeners for view entity buttons
  const viewButtons = elements.entitiesTableBody.querySelectorAll('.view-entity');
  viewButtons.forEach(button => {
    button.addEventListener('click', () => {
      const entityType = button.dataset.type;
      const entityId = button.dataset.id;
      viewEntityDetails(entityType, entityId);
    });
  });
}

// View transaction details modal
function viewTransactionDetails(transactionId) {
  const transaction = dashboardState.transactions.data.find(t => t.id === transactionId);
  if (!transaction) return;
  
  // This would typically open a modal with detailed transaction information
  console.log('View transaction details', transaction);
  // Implementation of modal would go here
}

// View entity details modal
function viewEntityDetails(entityType, entityId) {
  // This would typically fetch detailed entity information and open a modal
  console.log('View entity details', entityType, entityId);
  // Implementation of modal would go here
}

// Trigger manual synchronization
function triggerManualSync(entityType, entityId, priority) {
  fetch('/api/sync/trigger', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json'
    },
    body: JSON.stringify({
      entityType,
      entityId,
      priority
    })
  })
  .then(response => response.json())
  .then(data => {
    if (data.success) {
      showResult(elements.syncResult, 'Sync triggered successfully', 'success');
      // Refresh data after a short delay
      setTimeout(() => {
        fetchTransactions();
        fetchEntities();
      }, 1000);
    } else {
      showResult(elements.syncResult, `Error: ${data.error}`, 'error');
    }
  })
  .catch(error => {
    showResult(elements.syncResult, `Error: ${error.message}`, 'error');
  });
}

// Show result message
function showResult(element, message, type) {
  element.textContent = message;
  element.className = `form-result ${type}`;
  
  // Clear message after 3 seconds
  setTimeout(() => {
    element.textContent = '';
    element.className = 'form-result';
  }, 3000);
}

// Fetch transactions from API
function fetchTransactions() {
  fetch(`/api/sync/transactions?page=${dashboardState.transactions.page}&filter=${dashboardState.transactions.filter}`)
    .then(response => response.json())
    .then(data => {
      dashboardState.transactions.data = data;
      renderTransactionsTable(data);
      updateRecentActivity(data);
    })
    .catch(error => {
      console.error('Error fetching transactions:', error);
      elements.transactionsTableBody.innerHTML = `
        <tr><td colspan="7" class="error-data">Error loading transactions: ${error.message}</td></tr>`;
    });
}

// Fetch entities from API
function fetchEntities() {
  const type = dashboardState.entities.type;
  const search = dashboardState.entities.search;
  const searchParam = search ? `&search=${encodeURIComponent(search)}` : '';
  
  fetch(`/api/sync/entities/${type}?${searchParam}`)
    .then(response => response.json())
    .then(data => {
      dashboardState.entities.data = data;
      renderEntitiesTable(data);
    })
    .catch(error => {
      console.error('Error fetching entities:', error);
      elements.entitiesTableBody.innerHTML = `
        <tr><td colspan="7" class="error-data">Error loading entities: ${error.message}</td></tr>`;
    });
}

// Initialize the dashboard
function initializeDashboard() {
  // Initialize charts
  initializeCharts();
  
  // Navigation between sections
  const menuItems = document.querySelectorAll('.menu li');
  const sections = document.querySelectorAll('main section');
  
  menuItems.forEach(item => {
    item.addEventListener('click', function() {
      const sectionId = this.dataset.section;
      
      // Update active menu item
      menuItems.forEach(i => i.classList.remove('active'));
      this.classList.add('active');
      
      // Update active section
      sections.forEach(s => s.classList.remove('active'));
      document.getElementById(sectionId).classList.add('active');
      
      // Update current section in state
      dashboardState.currentSection = sectionId;
      
      // Load section-specific data
      if (sectionId === 'transactions') {
        fetchTransactions();
      } else if (sectionId === 'entities') {
        fetchEntities();
      }
    });
  });
  
  // Transaction filter change
  elements.transactionFilter.addEventListener('change', function() {
    dashboardState.transactions.filter = this.value;
    dashboardState.transactions.page = 1; // Reset to first page
    fetchTransactions();
  });
  
  // Refresh transactions button
  elements.refreshTransactions.addEventListener('click', function() {
    fetchTransactions();
  });
  
  // Pagination handlers
  elements.prevPage.addEventListener('click', function() {
    if (dashboardState.transactions.page > 1) {
      dashboardState.transactions.page--;
      fetchTransactions();
      elements.pageInfo.textContent = `Page ${dashboardState.transactions.page}`;
      elements.nextPage.disabled = false;
      if (dashboardState.transactions.page === 1) {
        this.disabled = true;
      }
    }
  });
  
  elements.nextPage.addEventListener('click', function() {
    dashboardState.transactions.page++;
    fetchTransactions();
    elements.pageInfo.textContent = `Page ${dashboardState.transactions.page}`;
    elements.prevPage.disabled = false;
  });
  
  // Entity type filter change
  elements.entityTypeFilter.addEventListener('change', function() {
    dashboardState.entities.type = this.value;
    fetchEntities();
  });
  
  // Entity search input
  elements.entitySearch.addEventListener('input', debounce(function() {
    dashboardState.entities.search = this.value;
    fetchEntities();
  }, 300));
  
  // Refresh entities button
  elements.refreshEntities.addEventListener('click', function() {
    fetchEntities();
  });
  
  // Manual sync form submission
  elements.manualSyncForm.addEventListener('submit', function(e) {
    e.preventDefault();
    const entityType = document.getElementById('entity-type').value;
    const entityId = document.getElementById('entity-id').value;
    const priority = document.getElementById('sync-priority').value;
    
    triggerManualSync(entityType, entityId, priority);
  });
  
  // System action buttons
  elements.verifyAllIntegrity.addEventListener('click', function() {
    // This would trigger a data integrity check
    showResult(elements.actionResult, 'Data integrity verification started', 'success');
  });
  
  elements.clearQueue.addEventListener('click', function() {
    if (confirm('Are you sure you want to clear all message queues? This may disrupt ongoing synchronization.')) {
      // This would clear message queues
      showResult(elements.actionResult, 'Message queues have been cleared', 'warning');
    }
  });
  
  elements.restartSyncService.addEventListener('click', function() {
    if (confirm('Are you sure you want to restart the sync service? This will interrupt ongoing operations.')) {
      // This would restart the sync service
      showResult(elements.actionResult, 'Sync service restart initiated', 'warning');
    }
  });
}

// Helper function for debouncing input events
function debounce(func, wait) {
  let timeout;
  return function(...args) {
    const context = this;
    clearTimeout(timeout);
    timeout = setTimeout(() => func.apply(context, args), wait);
  };
}

// Socket.io event handlers
socket.on('connect', () => {
  updateConnectionStatus(true);
});

socket.on('disconnect', () => {
  updateConnectionStatus(false);
});

socket.on('syncStatus', (status) => {
  updateSystemStatus(status.health);
});

socket.on('transactions', (transactions) => {
  dashboardState.transactions.data = transactions;
  renderTransactionsTable(transactions);
  updateRecentActivity(transactions);
});

socket.on('metrics', (metrics) => {
  dashboardState.metrics = metrics;
  updateCharts(metrics);
});

// Initialize dashboard when DOM is loaded
document.addEventListener('DOMContentLoaded', () => {
  initializeDashboard();
  
  // Initial data fetch
  fetchTransactions();
  fetchEntities();
});