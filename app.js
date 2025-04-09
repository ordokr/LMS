const express = require('express');
const app = express();
const port = 3000;

// Import routes
const monitoringRoutes = require('./routes/monitoring');

// Middleware
app.use(express.json());

// Register routes
app.use('/api/monitoring', monitoringRoutes);

// Start server
app.listen(port, () => {
  console.log(`Server is running on port ${port}`);
});