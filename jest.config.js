module.exports = {
  testEnvironment: 'node',
  collectCoverage: true,
  coverageDirectory: 'coverage',
  coverageReporters: ['text', 'lcov', 'clover'],
  collectCoverageFrom: [
    'src/**/*.js',
    '!src/**/*.test.js',
    '!**/node_modules/**',
  ],
  transformIgnorePatterns: [
    "/node_modules/(?!chai)"
  ],
  transform: {
    "^.+\\.(js|jsx|ts|tsx)$": "babel-jest"
  },
  testMatch: ['**/__tests__/**/*.js', '**/?(*.)+(spec|test).js'],
  testPathIgnorePatterns: ['/node_modules/'],
  setupFilesAfterEnv: ['./jest.setup.js'],
  verbose: true,
  moduleFileExtensions: ["js", "json", "jsx", "ts", "tsx", "node"],
  setupFiles: ["./jest.setup.js"],
  moduleDirectories: ['node_modules', 'services']
};