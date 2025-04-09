import express from 'express';
import bodyParser from 'body-parser';
import mongoose from 'mongoose';
import cors from 'cors';
import morgan from 'morgan';
import dotenv from 'dotenv';
import authRoutes from './routes/authRoutes';
import webhookRoutes from './routes/webhookRoutes';
import notificationRoutes from './routes/notificationRoutes';

dotenv.config();

const app = express();

app.use(bodyParser.json());
app.use(cors());
app.use(morgan('dev'));

// Update MongoDB connection logic to handle test environment
if (process.env.NODE_ENV !== 'test') {
  // Only connect to MongoDB in non-test environment
  const MONGODB_URI = process.env.MONGODB_URI || 'mongodb://localhost:27017/lms-integration';
  
  mongoose.connect(MONGODB_URI)
    .then(() => console.log('Connected to MongoDB'))
    .catch(err => console.error('MongoDB connection error:', err));
}

app.use('/api/v1/auth', authRoutes);
app.use('/api/v1/webhooks', webhookRoutes);
app.use('/api/v1/notifications', notificationRoutes);

const PORT = process.env.PORT || 5000;

// Create server only if not in test environment or if explicitly requested
let server;
if (process.env.NODE_ENV !== 'test') {
  server = app.listen(PORT, () => {
    console.log(`Server is running on port ${PORT}`);
  });
}

// Export for testing
module.exports = { app, server };