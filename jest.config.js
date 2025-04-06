module.exports = {
  testEnvironment: 'node',
  
  // Transform files including node_modules/chai
  transformIgnorePatterns: [
    "/node_modules/(?!chai)"
  ],
  
  // Use Babel to transform ESM modules
  transform: {
    "^.+\\.(js|jsx|ts|tsx)$": "babel-jest"
  },
  
  // Only run test files with .test.js or .spec.js extensions
  testMatch: [
    "**/tests/unit/**/*.test.js",
    "**/tests/unit/**/*.spec.js"
  ],
  
  // Exclude integration tests from Jest (run them separately with Mocha)
  testPathIgnorePatterns: [
    "/node_modules/",
    "/tests/integration/"
  ],
  
  moduleFileExtensions: ["js", "json", "jsx", "ts", "tsx", "node"]
};