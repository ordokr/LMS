/**
 * JavaScript to Rust Migration Cleanup Script
 * 
 * This script removes JavaScript files that have been successfully migrated to Rust.
 * It checks if each file exists before attempting to delete it, avoiding errors.
 */

const fs = require('fs');
const path = require('path');

// List of JavaScript files that have been migrated to Rust
const migratedFiles = [
    'services/integration/mapping/course-category-mapper.js',
    'services/integration/model_mapper.js',
    'services/integration/api_integration.js',
    'services/integration/auth/jwt-provider.js',
    'services/monitoring/sync_dashboard.js',
    'services/monitoring/sync_monitor.js',
    'src/services/webhookService.js',
    'src/services/notificationService.js',
    'src/services/modelSyncService.js',
    'src/services/canvasAuthService.js',
    'src/services/discourseSSOService.js',
    'src/services/auth.js',
    'src/services/integration.js',
    'src/webhooks/canvas.js'
];

// Base directory (assuming script is run from the project root)
const baseDir = process.cwd();

// Counter for deleted files
let deletedCount = 0;
let notFoundCount = 0;

console.log('Starting cleanup of migrated JavaScript files...');

// Process each file
migratedFiles.forEach(relativeFilePath => {
    const filePath = path.join(baseDir, relativeFilePath);
    
    try {
        // Check if file exists
        if (fs.existsSync(filePath)) {
            // Delete the file
            fs.unlinkSync(filePath);
            console.log(`✅ Deleted: ${relativeFilePath}`);
            deletedCount++;
        } else {
            console.log(`⚠️ Not found: ${relativeFilePath}`);
            notFoundCount++;
        }
    } catch (error) {
        console.error(`❌ Error deleting ${relativeFilePath}:`, error.message);
    }
});

console.log(`\nCleanup completed:`);
console.log(`- ${deletedCount} files deleted`);
console.log(`- ${notFoundCount} files not found (may have been deleted previously)`);
