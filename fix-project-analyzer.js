const fs = require('fs');

// Read the file
const filePath = 'project-analyzer.js';
let content = fs.readFileSync(filePath, 'utf8');

// Create backup
fs.writeFileSync(`${filePath}.bak`, content);
console.log(`Created backup as ${filePath}.bak`);

// Fix line 816 and similar patterns
content = content.replace(
  /modelImplementation: ""\s*\+\s*modelPercentage%"`/g,
  `modelImplementation: \\"" + modelPercentage + "%\\""`
);

content = content.replace(
  /apiImplementation: ""\s*\+\s*apiPercentage%"`/g,
  `apiImplementation: \\"" + apiPercentage + "%\\""`
);

content = content.replace(
  /uiImplementation: ""\s*\+\s*uiPercentage%"`/g,
  `uiImplementation: \\"" + uiPercentage + "%\\""`
);

content = content.replace(
  /testCoverage: ""\s*\+\s*this\.metrics\.tests\.coverage%"`/g,
  `testCoverage: \\"" + this.metrics.tests.coverage + "%\\""`
);

// Fix other potential syntax errors (report += "| ${feature.name} | " + ...)
content = content.replace(
  /report \+= "\| \${([^}]+)}/g, 
  'report += "| " + ($1'
);

// Write the fixed file
fs.writeFileSync(filePath, content);
console.log(`Fixed string concatenation errors in ${filePath}`);