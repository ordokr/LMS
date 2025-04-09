export const config = {
  JWT_SECRET: process.env.JWT_SECRET || 'your-development-secret-key',
  JWT_EXPIRATION: process.env.JWT_EXPIRATION || '24h',
  DISCOURSE_URL: process.env.DISCOURSE_URL || 'https://discourse.example.com',
  CANVAS_API_URL: process.env.CANVAS_API_URL || 'https://canvas.example.com/api'
};