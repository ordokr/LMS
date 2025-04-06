const { execSync } = require('child_process');
const fs = require('fs');
const path = require('path');

console.log('Setting up development environment...');

// Check if we're using npm or yarn
const useYarn = fs.existsSync(path.join(process.cwd(), 'yarn.lock'));
const packageManager = useYarn ? 'yarn' : 'npm';

// Install dependencies
console.log(`Installing dependencies using ${packageManager}...`);
try {
  execSync(`${packageManager} install`, { stdio: 'inherit' });
  console.log('Dependencies installed successfully.');
} catch (error) {
  console.error('Failed to install dependencies:', error.message);
  process.exit(1);
}

// Create required directories
const directories = [
  'logs',
  'data',
  'data/feedback',
  'tests/unit',
  'tests/integration',
  'docs',
  'build-output'
];

console.log('Creating required directories...');
for (const dir of directories) {
  const dirPath = path.join(process.cwd(), dir);
  if (!fs.existsSync(dirPath)) {
    fs.mkdirSync(dirPath, { recursive: true });
    console.log(`Created directory: ${dir}`);
  }
}

// Check for binaries in node_modules/.bin
const binPath = path.join(process.cwd(), 'node_modules', '.bin');
const eslintPath = path.join(binPath, 'eslint' + (process.platform === 'win32' ? '.cmd' : ''));
const jestPath = path.join(binPath, 'jest' + (process.platform === 'win32' ? '.cmd' : ''));

console.log('Checking for required tools...');

if (!fs.existsSync(eslintPath)) {
  console.log('ESLint not found. To install: npm install eslint --save-dev');
} else {
  console.log('ESLint found: ' + eslintPath);
}

if (!fs.existsSync(jestPath)) {
  console.log('Jest not found. To install: npm install jest --save-dev');
} else {
  console.log('Jest found: ' + jestPath);
}

console.log('Development environment setup complete!');
console.log('\nYou can now run the build with:');
console.log(`  ${packageManager} run build`);