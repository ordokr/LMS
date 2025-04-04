// filepath: c:\Users\Tim\Desktop\LMS\cleanup-docs.js

const fs = require('fs');
const path = require('path');

/**
 * Script to consolidate documentation and ensure project_status.md is the
 * single source of truth for project information
 */
function cleanupDocumentation() {
  const baseDir = path.resolve(__dirname);
  const docsDir = path.join(baseDir, 'docs');
  
  // Ensure docs directory exists
  if (!fs.existsSync(docsDir)) {
    fs.mkdirSync(docsDir);
  }
  
  // List of files that should be moved to docs directory
  const filesToMove = [
    'implementation_details.md',
    'relationship_map.md'
  ];
  
  // Move files to docs directory
  for (const file of filesToMove) {
    const filePath = path.join(baseDir, file);
    if (fs.existsSync(filePath)) {
      const targetPath = path.join(docsDir, file);
      fs.copyFileSync(filePath, targetPath);
      fs.unlinkSync(filePath);
      console.log(`Moved ${file} to docs directory`);
    }
  }
  
  // Update project_status.md to reference moved documents
  const statusFile = path.join(baseDir, 'project_status.md');
  if (fs.existsSync(statusFile)) {
    let statusContent = fs.readFileSync(statusFile, 'utf8');
    
    // Add reference to implementation details if not exists
    if (!statusContent.includes('docs/implementation_details.md')) {
      if (!statusContent.includes('## 📊 Detailed Implementation')) {
        statusContent += '\n\n---\n\n## 📊 Detailed Implementation\n\n';
        statusContent += '_For complete implementation details, see [Implementation Details](./docs/implementation_details.md)_\n';
      }
    }
    
    // Add reference to relationship map if not exists
    if (!statusContent.includes('docs/relationships.md') && !statusContent.includes('docs/relationship_map.md')) {
      if (!statusContent.includes('## 🔄 Relationship Map')) {
        statusContent += '\n\n---\n\n## 🔄 Relationship Map\n\n';
        statusContent += '_For detailed relationship maps, see [Relationship Map](./docs/relationships.md)_\n';
      }
    }
    
    // Add note about this being the single source of truth
    if (!statusContent.includes('# This document is the single source of truth')) {
      statusContent = `<!-- 
# This document is the single source of truth for project status
# Auto-generated sections will be updated by scripts
# Manual edits to auto-generated sections may be overwritten
-->\n\n${statusContent}`;
    }
    
    // Write back updated content
    fs.writeFileSync(statusFile, statusContent);
    console.log('Updated project_status.md with references to documentation');
  }
  
  // Create a README.md file in the root if it doesn't exist
  const readmeFile = path.join(baseDir, 'README.md');
  if (!fs.existsSync(readmeFile)) {
    const readmeContent = `# Canvas LMS Desktop Client & Forum Integration

## Project Overview

This project integrates a forum system with Canvas LMS to provide a seamless desktop experience.

## Documentation

For complete project status and implementation details, see [Project Status](./project_status.md).

This document serves as the **single source of truth** for the project status and implementation details.

## Development

For development workflow and guidelines, refer to the [Project Status](./project_status.md) document.
`;
    fs.writeFileSync(readmeFile, readmeContent);
    console.log('Created README.md with reference to project_status.md');
  }
}

cleanupDocumentation();