const fs = require('fs');
const path = require('path');

const filePath = path.join(__dirname, 'project-analyzer.js');
let content = fs.readFileSync(filePath, 'utf8');

// Create a backup
fs.writeFileSync(filePath + '.bak', content);
console.log('Created backup at:', filePath + '.bak');

// Replace all template literal syntax with string concatenation
content = content.replace(/`([^`]*)\${([^}]*)}/g, (match, before, expr) => {
  return '"' + before + '" + ' + expr;
});

content = content.replace(/`([^`]*)\${([^}]*)}([^`]*)`/g, (match, before, expr, after) => {
  return '"' + before + '" + ' + expr + ' + "' + after + '"';
});

// Write back the file
fs.writeFileSync(filePath, content);
console.log('Fixed template literals in the file');
console.log('Done!');