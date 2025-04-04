const fs = require('fs');
const path = require('path');
const glob = require('glob');

function updateStatus() {
  // Base directory of the project
  const baseDir = path.resolve(__dirname);
  
  // Implementation metrics - set reasonable defaults based on manual counts
  const metrics = {
    modelCount: 7, // Based on the table in project_status.md
    implementedModels: 4, // User, Category, Topic, Post with partial implementation
    apiEndpoints: 6, // Based on the table
    implementedEndpoints: 5, // All except Courses
    uiComponents: 5, // Based on the table
    implementedComponents: 4, // All except Course List
  };
  
  // Calculate percentages with safety checks to avoid NaN
  const modelPercentage = metrics.modelCount > 0 
    ? Math.round((metrics.implementedModels / metrics.modelCount) * 100) 
    : 0;
    
  const apiPercentage = metrics.apiEndpoints > 0 
    ? Math.round((metrics.implementedEndpoints / metrics.apiEndpoints) * 100) 
    : 0;
    
  const uiPercentage = metrics.uiComponents > 0 
    ? Math.round((metrics.implementedComponents / metrics.uiComponents) * 100) 
    : 0;
  
  // Update the status in the markdown file
  let statusDoc = fs.readFileSync('project_status.md', 'utf8');
  
  // Update the JavaScript object in the status section
  statusDoc = statusDoc.replace(
    /modelImplementation: "[^"]+"/,
    `modelImplementation: "${modelPercentage}%"`
  );
  
  statusDoc = statusDoc.replace(
    /apiImplementation: "[^"]+"/,
    `apiImplementation: "${apiPercentage}%"`
  );
  
  statusDoc = statusDoc.replace(
    /uiImplementation: "[^"]+"/,
    `uiImplementation: "${uiPercentage}%"`
  );
  
  // Write back the updated status
  fs.writeFileSync('project_status.md', statusDoc);
  
  console.log(`Project status document updated successfully!`);
  console.log(`Models: ${modelPercentage}%, API: ${apiPercentage}%, UI: ${uiPercentage}%`);
}

updateStatus();