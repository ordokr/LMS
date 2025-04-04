const fs = require('fs');
const path = require('path');

function fixTemplateStrings(filePath) {
  console.log(`Fixing template literals in ${filePath}...`);
  
  // Read the file
  let content = fs.readFileSync(filePath, 'utf8');
  
  // Create a backup
  fs.writeFileSync(`${filePath}.bak`, content);
  console.log(`Backup created at ${filePath}.bak`);
  
  // Process the content line by line
  const lines = content.split('\n');
  const fixedLines = [];
  
  for (let i = 0; i < lines.length; i++) {
    const line = lines[i];
    
    // Check if the line contains a template literal
    if (line.includes('`') && line.includes('${')) {
      try {
        // Try to convert template literals to string concatenation
        let fixedLine = line;
        const parts = [];
        let currentPos = 0;
        
        // Find all template literals
        while (true) {
          const startPos = fixedLine.indexOf('`', currentPos);
          if (startPos === -1) break;
          
          const endPos = fixedLine.indexOf('`', startPos + 1);
          if (endPos === -1) break;
          
          // Extract the template literal
          const template = fixedLine.substring(startPos, endPos + 1);
          
          // Convert ${...} expressions to concatenation
          let convertedStr = template.substring(1, template.length - 1); // Remove backticks
          convertedStr = convertedStr.replace(/\${([^}]*)}/g, '" + ($1) + "');
          
          // Replace in the line
          fixedLine = 
            fixedLine.substring(0, startPos) + 
            '"' + convertedStr + '"' + 
            fixedLine.substring(endPos + 1);
            
          currentPos = startPos + convertedStr.length + 2; // +2 for the added quotes
        }
        
        fixedLines.push(fixedLine);
      } catch (err) {
        console.log(`Error processing line ${i+1}, keeping original: ${line}`);
        fixedLines.push(line);
      }
    } else {
      // Keep the line as is
      fixedLines.push(line);
    }
  }
  
  // Write the fixed content
  fs.writeFileSync(filePath, fixedLines.join('\n'));
  console.log(`Fixed template literals in ${filePath}`);
}

const filePath = path.join(__dirname, 'project-analyzer.js');
fixTemplateStrings(filePath);
console.log('Done!');