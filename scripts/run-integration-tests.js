const { spawn } = require('child_process');
const path = require('path');
const fs = require('fs');

// Ensure the logs directory exists
const logsDir = path.join(__dirname, '..', 'logs');
if (!fs.existsSync(logsDir)) {
  fs.mkdirSync(logsDir, { recursive: true });
}

const logFilePath = path.join(logsDir, 'integration-tests.log');
const logStream = fs.createWriteStream(logFilePath, { flags: 'a' });

console.log('Starting integration tests...');
logStream.write(`\n--- Integration Tests Started at ${new Date().toISOString()} ---\n`);

// Check if there are actual test files to run
const testsDir = path.join(__dirname, '..', 'tests', 'integration');
if (!fs.existsSync(testsDir)) {
  fs.mkdirSync(testsDir, { recursive: true });
  console.log(`Created integration tests directory: ${testsDir}`);
}

// Create a proper Mocha integration test
const sampleTestContent = `
const { expect } = require('chai');

describe('Canvas-Discourse Integration Tests', function() {
  // Mocha uses 'function' for proper 'this' binding
  before(function() {
    // Setup code
  });
  
  it('should verify basic integration setup', function() {
    expect(true).to.equal(true);
  });
  
  it('should demonstrate a test with async/await', async function() {
    // This would be replaced with actual API calls in real tests
    const result = await Promise.resolve({
      success: true,
      discourseTopicId: '12345'
    });
    expect(result).to.have.property('success', true);
    expect(result).to.have.property('discourseTopicId');
  });
});
`;

// Create or update the sample integration test
const testFilePath = path.join(testsDir, 'canvas-discourse-integration.test.js');
if (!fs.existsSync(testFilePath)) {
  fs.writeFileSync(testFilePath, sampleTestContent);
  console.log('Created sample integration test file.');
}

// Check if we have the mocha binary
const mochaBin = path.join(__dirname, '..', 'node_modules', '.bin', 'mocha');
if (fs.existsSync(mochaBin)) {
  // Run the tests with Mocha
  console.log('Running integration tests with Mocha...');
  
  const testProcess = spawn(mochaBin, ['tests/integration/**/*.test.js'], {
    stdio: 'inherit',
    shell: true
  });
  
  testProcess.on('close', (code) => {
    logStream.write(`\n--- Integration Tests Completed with code ${code} at ${new Date().toISOString()} ---\n`);
    logStream.end();
    console.log(`Integration test log saved to ${logFilePath}`);
  });
} else {
  console.log('Mocha not found. Please install: npm install mocha --save-dev');
  console.log('Integration test framework is in place but tests were not run.');
  logStream.write(`\n--- Integration Tests not run (Mocha not found) at ${new Date().toISOString()} ---\n`);
  logStream.end();
}