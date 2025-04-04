const fs = require('fs');
const path = require('path');

// Path to the project-analyzer.js file
const filePath = path.join(__dirname, 'project-analyzer.js');

// Read the content of the file
let content = fs.readFileSync(filePath, 'utf8');

// Regular expression to find string concatenation with variables
const stringConcatRegex = /"[^"]*"\s*\+\s*\(([^)]+)\)\s*\+\s*"[^"]*"/g;
const stringConcatRegex2 = /"([^"]*)"\s*\+\s*\(([^)]+)\)\s*\+\s*"([^"]*)"/g;
const stringConcatRegex3 = /"([^"]*)"\s*\+\s*\(([^)]+)\)/g;
const stringConcatRegex4 = /\(([^)]+)\)\s*\+\s*"([^"]*)"/g;

// Replace string concatenations with template literals
let modified = content;

modified = modified.replace(stringConcatRegex2, (match, before, variable, after) => {
  return `\`${before}${variable}${after}\``;
});

modified = modified.replace(stringConcatRegex3, (match, before, variable) => {
  return `\`${before}${variable}\``;
});

modified = modified.replace(stringConcatRegex4, (match, variable, after) => {
  return `\`${variable}${after}\``;
});

// Write the modified content back to the file
fs.writeFileSync(filePath + '.fixed', modified);

console.log('Fixed file written to:', filePath + '.fixed');
console.log('Please review the fixed file and replace the original if all changes look correct.');