const fs = require('fs');
const path = require('path');

function fixProjectAnalyzerFile() {
  const filePath = path.join(__dirname, 'project-analyzer.js');
  console.log(`Fixing file: ${filePath}`);
  
  // Create a backup
  const backupPath = path.join(__dirname, 'project-analyzer.js.backup');
  fs.copyFileSync(filePath, backupPath);
  console.log(`Backup created at: ${backupPath}`);
  
  // Read the file content
  let content = fs.readFileSync(filePath, 'utf8');
  
  // Fix the specific replace statements that are causing issues
  content = content.replace(
    /statusContent = statusContent\.replace\(\s*\/modelImplementation: "[^"]+"/,
    'statusContent = statusContent.replace(\n    /modelImplementation: "[^"]+"/,'
  );
  
  content = content.replace(
    /`modelImplementation: "\${modelPercentage}%"`/g,
    '"modelImplementation: \\"" + modelPercentage + "\\%\\""'
  );
  
  content = content.replace(
    /`apiImplementation: "\${apiPercentage}%"`/g,
    '"apiImplementation: \\"" + apiPercentage + "\\%\\""'
  );
  
  content = content.replace(
    /`uiImplementation: "\${uiPercentage}%"`/g,
    '"uiImplementation: \\"" + uiPercentage + "\\%\\""'
  );
  
  content = content.replace(
    /`testCoverage: "\${this\.metrics\.tests\.coverage}%"`/g,
    '"testCoverage: \\"" + this.metrics.tests.coverage + "\\%\\""'
  );
  
  // Fix report generation template strings
  content = content.replace(
    /report \+= `## Models \(\${modelPercentage}% Complete\)\\n\\n`;/g,
    'report += "## Models (" + modelPercentage + "% Complete)\\n\\n";'
  );
  
  content = content.replace(
    /report \+= `## API Endpoints \(\${apiPercentage}% Complete\)\\n\\n`;/g,
    'report += "## API Endpoints (" + apiPercentage + "% Complete)\\n\\n";'
  );
  
  content = content.replace(
    /report \+= `## UI Components \(\${uiPercentage}% Complete\)\\n\\n`;/g,
    'report += "## UI Components (" + uiPercentage + "% Complete)\\n\\n";'
  );
  
  content = content.replace(
    /report \+= `## Forum Features \(\${forumPercentage}% Complete\)\\n\\n`;/g,
    'report += "## Forum Features (" + forumPercentage + "% Complete)\\n\\n";'
  );
  
  content = content.replace(
    /report \+= `## LMS Integration \(\${lmsPercentage}% Complete\)\\n\\n`;/g,
    'report += "## LMS Integration (" + lmsPercentage + "% Complete)\\n\\n";'
  );
  
  content = content.replace(
    /report \+= `## Tests \(\${this\.metrics\.tests\.coverage}% Coverage\)\\n\\n`;/g,
    'report += "## Tests (" + this.metrics.tests.coverage + "% Coverage)\\n\\n";'
  );
  
  // Fix other template literals
  content = content.replace(
    /map \+= `# Code Relationship Map\\n_Generated on \${new Date\(\)\.toISOString\(\)\.split\('T'\)\[0\]}_\\n\\n`;/g,
    'map += "# Code Relationship Map\\n_Generated on " + new Date().toISOString().split("T")[0] + "_\\n\\n";'
  );
  
  content = content.replace(
    /report \+= `# Detailed Implementation Report\\n_Generated on \${new Date\(\)\.toISOString\(\)\.split\('T'\)\[0\]}_\\n\\n`;/g,
    'report += "# Detailed Implementation Report\\n_Generated on " + new Date().toISOString().split("T")[0] + "_\\n\\n";'
  );
  
  content = content.replace(
    /report \+= `# Comprehensive Implementation Report\\n_Generated on \${new Date\(\)\.toISOString\(\)\.split\('T'\)\[0\]}_\\n\\n`;/g,
    'report += "# Comprehensive Implementation Report\\n_Generated on " + new Date().toISOString().split("T")[0] + "_\\n\\n";'
  );
  
  // Write the fixed content back to the file
  fs.writeFileSync(filePath, content);
  console.log('File fixed successfully!');
}

// Run the fix
fixProjectAnalyzerFile();