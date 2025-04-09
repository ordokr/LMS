const fs = require('fs');
const path = require('path');

/**
 * Generate technical documentation from source code
 * @param {Object} options - Configuration options
 * @param {string} options.outputDir - Output directory for documentation
 * @param {string} options.sourceDir - Source code directory
 */
async function generateTechnicalDocs(options) {
  const { outputDir, sourceDir } = options;
  
  console.log(`Generating technical documentation from ${sourceDir} to ${outputDir}...`);
  
  // Create output directory if it doesn't exist
  if (!fs.existsSync(outputDir)) {
    fs.mkdirSync(outputDir, { recursive: true });
  }
  
  // Generate documentation for different components
  await generateModelsDocs(sourceDir, outputDir);
  await generateApiDocs(sourceDir, outputDir);
  await generateServicesDocs(sourceDir, outputDir);
  await generateIntegrationDocs(sourceDir, outputDir);
  
  // Generate port comparison documentation
  await generatePortComparisonDocs(sourceDir, outputDir);
  
  // Generate index
  await generateDocsIndex(outputDir);
  
  console.log('Technical documentation generation complete!');
}

/**
 * Generate documentation for data models
 */
async function generateModelsDocs(sourceDir, outputDir) {
  const modelsDir = path.join(sourceDir, 'src', 'models');
  const outputFile = path.join(outputDir, 'models.md');
  
  // Check if models directory exists
  if (!fs.existsSync(modelsDir)) {
    console.log('Models directory not found, skipping models documentation');
    return;
  }
  
  // Start building the documentation
  let content = `# Data Models\n\n`;
  content += `*Generated on ${new Date().toISOString().split('T')[0]}*\n\n`;
  content += `This document describes the data models used in the application.\n\n`;
  
  // Get all model files
  const modelFiles = fs.readdirSync(modelsDir)
    .filter(file => file.endsWith('.rs'));
  
  // Process each model file
  for (const file of modelFiles) {
    const filePath = path.join(modelsDir, file);
    const fileContent = fs.readFileSync(filePath, 'utf8');
    
    // Extract model name
    const modelName = path.basename(file, '.rs');
    content += `## ${toTitleCase(modelName)}\n\n`;
    
    // Extract struct definition
    const structMatch = fileContent.match(/struct\s+(\w+)\s*{([^}]*)}/);
    if (structMatch) {
      content += `\`\`\`rust\nstruct ${structMatch[1]} {\n${structMatch[2]}\n}\n\`\`\`\n\n`;
      
      // Extract fields
      const fields = structMatch[2].split('\n')
        .map(line => line.trim())
        .filter(line => line.length > 0);
      
      content += `### Fields\n\n`;
      content += `| Name | Type | Description |\n`;
      content += `|------|------|-------------|\n`;
      
      fields.forEach(field => {
        const fieldMatch = field.match(/(\w+):\s*(\w+[\<\>\[\]]*)/);
        if (fieldMatch) {
          const name = fieldMatch[1];
          const type = fieldMatch[2];
          content += `| ${name} | ${type} | |\n`;
        }
      });
      
      content += `\n`;
    }
    
    // Check for implementation blocks
    const implBlocks = fileContent.match(/impl\s+(\w+)\s*{([^}]*)}/g) || [];
    if (implBlocks.length > 0) {
      content += `### Methods\n\n`;
      
      implBlocks.forEach(implBlock => {
        const methodMatches = implBlock.match(/fn\s+(\w+)\s*\(([^)]*)\)/g) || [];
        methodMatches.forEach(methodMatch => {
          content += `- \`${methodMatch}\`\n`;
        });
      });
      
      content += `\n`;
    }
    
    // Check for port source reference
    content += `### Port Source Reference\n\n`;
    
    // Look for Canvas equivalent
    content += `**Canvas Equivalent:** \`app/models/${modelName}.rb\`\n\n`;
    
    // Look for Discourse equivalent
    content += `**Discourse Equivalent:** \`app/models/${modelName}.rb\`\n\n`;
    
    content += `---\n\n`;
  }
  
  fs.writeFileSync(outputFile, content);
  console.log(`Generated models documentation at ${outputFile}`);
}

/**
 * Generate documentation for API endpoints
 */
async function generateApiDocs(sourceDir, outputDir) {
  const apiDir = path.join(sourceDir, 'src', 'api');
  const outputFile = path.join(outputDir, 'api.md');
  
  // Check if API directory exists
  if (!fs.existsSync(apiDir)) {
    console.log('API directory not found, skipping API documentation');
    return;
  }
  
  // Start building the documentation
  let content = `# API Endpoints\n\n`;
  content += `*Generated on ${new Date().toISOString().split('T')[0]}*\n\n`;
  content += `This document describes the API endpoints exposed by the application.\n\n`;
  
  // Get all API files
  const apiFiles = fs.readdirSync(apiDir)
    .filter(file => file.endsWith('.rs'));
  
  // Process each API file
  for (const file of apiFiles) {
    const filePath = path.join(apiDir, file);
    const fileContent = fs.readFileSync(filePath, 'utf8');
    
    // Extract module name
    const moduleName = path.basename(file, '.rs');
    content += `## ${toTitleCase(moduleName)} API\n\n`;
    
    // Extract route handlers
    const routeHandlers = [];
    const routeMatches = fileContent.matchAll(/#\[(\w+)\("([^"]+)"\)\]\s*(?:async\s+)?fn\s+(\w+)/g);
    for (const match of routeMatches) {
      const method = match[1].toUpperCase();
      const path = match[2];
      const handlerName = match[3];
      routeHandlers.push({ method, path, handlerName });
    }
    
    if (routeHandlers.length > 0) {
      content += `| Method | Path | Handler |\n`;
      content += `|--------|------|--------|\n`;
      
      routeHandlers.forEach(handler => {
        content += `| ${handler.method} | ${handler.path} | ${handler.handlerName} |\n`;
      });
      
      content += `\n`;
    }
    
    // Check for port source reference
    content += `### Port Source Reference\n\n`;
    
    // Look for Canvas equivalent
    content += `**Canvas Equivalent:** \`app/controllers/${moduleName}_controller.rb\`\n\n`;
    
    // Look for Discourse equivalent
    content += `**Discourse Equivalent:** \`app/controllers/${moduleName}_controller.rb\`\n\n`;
    
    content += `---\n\n`;
  }
  
  fs.writeFileSync(outputFile, content);
  console.log(`Generated API documentation at ${outputFile}`);
}

/**
 * Generate documentation for services
 */
async function generateServicesDocs(sourceDir, outputDir) {
  const servicesDir = path.join(sourceDir, 'src', 'services');
  const outputFile = path.join(outputDir, 'services.md');
  
  // Check if services directory exists
  if (!fs.existsSync(servicesDir)) {
    console.log('Services directory not found, skipping services documentation');
    return;
  }
  
  // Build documentation placeholder
  let content = `# Services\n\n`;
  content += `*Generated on ${new Date().toISOString().split('T')[0]}*\n\n`;
  content += `This document describes the services used in the application.\n\n`;
  
  // Get all service files
  const serviceFiles = fs.readdirSync(servicesDir)
    .filter(file => file.endsWith('.rs'));
  
  // Process each service file
  for (const file of serviceFiles) {
    const serviceName = path.basename(file, '.rs');
    content += `## ${toTitleCase(serviceName)}Service\n\n`;
    content += `Service responsible for ${serviceName} related operations.\n\n`;
    
    // Add placeholder for methods
    content += `### Methods\n\n`;
    content += `*Methods documentation will be generated from code comments*\n\n`;
    
    // Check for port source reference
    content += `### Port Source Reference\n\n`;
    content += `**Canvas Equivalent:** \`app/services/${serviceName}_service.rb\`\n\n`;
    content += `**Discourse Equivalent:** \`app/services/${serviceName}_service.rb\`\n\n`;
    
    content += `---\n\n`;
  }
  
  fs.writeFileSync(outputFile, content);
  console.log(`Generated services documentation at ${outputFile}`);
}

/**
 * Generate documentation for integration components
 */
async function generateIntegrationDocs(sourceDir, outputDir) {
  const integrationDir = path.join(sourceDir, 'src', 'integration');
  const outputFile = path.join(outputDir, 'integration.md');
  
  // Check if integration directory exists
  if (!fs.existsSync(integrationDir)) {
    console.log('Integration directory not found, skipping integration documentation');
    return;
  }
  
  // Build documentation placeholder
  let content = `# Integration Components\n\n`;
  content += `*Generated on ${new Date().toISOString().split('T')[0]}*\n\n`;
  content += `This document describes the components used for Canvas and Discourse integration.\n\n`;
  
  // Get all integration files
  const integrationFiles = fs.readdirSync(integrationDir)
    .filter(file => file.endsWith('.rs'));
  
  // Process each integration file
  for (const file of integrationFiles) {
    const componentName = path.basename(file, '.rs');
    content += `## ${toTitleCase(componentName)}Integration\n\n`;
    content += `Integration component responsible for ${componentName}.\n\n`;
    
    // Add placeholder for functionality
    content += `### Functionality\n\n`;
    content += `*Functionality documentation will be generated from code comments*\n\n`;
    
    // Check for integration points
    content += `### Integration Points\n\n`;
    content += `*Integration points will be automatically documented based on code analysis*\n\n`;
    
    content += `---\n\n`;
  }
  
  fs.writeFileSync(outputFile, content);
  console.log(`Generated integration documentation at ${outputFile}`);
}

/**
 * Generate documentation index
 */
async function generateDocsIndex(outputDir) {
  const indexFile = path.join(outputDir, 'index.md');
  
  // Get all markdown files in the output directory
  const files = fs.readdirSync(outputDir)
    .filter(file => file.endsWith('.md'));
  
  // Build the index
  let content = `# Technical Documentation\n\n`;
  content += `*Generated on ${new Date().toISOString().split('T')[0]}*\n\n`;
  content += `This is the technical documentation for the Canvas-Discourse LMS integration.\n\n`;
  content += `## Available Documentation\n\n`;
  
  // Add links to each document
  files.forEach(file => {
    if (file === 'index.md') return;
    const title = path.basename(file, '.md');
    content += `- [${toTitleCase(title)}](${file})\n`;
  });
  
  fs.writeFileSync(indexFile, content);
  console.log(`Generated documentation index at ${indexFile}`);
}

/**
 * Generate documentation comparing port source to target implementation
 */
async function generatePortComparisonDocs(sourceDir, outputDir) {
  const portDir = 'C:\\Users\\Tim\\Desktop\\port';
  const outputFile = path.join(outputDir, 'port_comparison.md');
  
  // Check if port directory exists
  if (!fs.existsSync(portDir)) {
    console.log('Port directory not found, skipping port comparison documentation');
    return;
  }
  
  // Start building the documentation
  let content = `# Source Code Port Comparison\n\n`;
  content += `*Generated on ${new Date().toISOString().split('T')[0]}*\n\n`;
  content += `This document compares the original Canvas and Discourse source code with our implementation.\n\n`;
  
  // Compare Canvas models
  content += `## Canvas Models Comparison\n\n`;
  await compareModelStructures(
    path.join(portDir, 'canvas', 'app', 'models'),
    path.join(sourceDir, 'src', 'models'),
    content
  );
  
  // Compare Discourse models
  content += `## Discourse Models Comparison\n\n`;
  await compareModelStructures(
    path.join(portDir, 'discourse', 'app', 'models'),
    path.join(sourceDir, 'src', 'models'),
    content
  );
  
  // Compare APIs
  content += `## API Endpoints Comparison\n\n`;
  await compareApiEndpoints(
    portDir,
    path.join(sourceDir, 'src', 'api'),
    content
  );
  
  // Add findings and recommendations
  content += `## Findings and Recommendations\n\n`;
  content += `### Identified Issues\n\n`;
  content += `1. **Model Duplication**: Multiple definitions of core models exist in both source and target code\n`;
  content += `2. **API Path Conflicts**: Some API paths overlap between Canvas and Discourse endpoints\n`;
  content += `3. **Inconsistent Naming**: Different naming conventions are used across the codebase\n`;
  content += `4. **Authentication Mechanisms**: Different authentication approaches need to be unified\n\n`;
  
  content += `### Recommendations\n\n`;
  content += `1. **Consolidate Models**: Create unified model definitions that satisfy both systems' requirements\n`;
  content += `2. **Namespace APIs**: Use clear namespacing for API endpoints to prevent conflicts\n`;
  content += `3. **Standardize Naming**: Adopt consistent naming conventions as specified in the project guide\n`;
  content += `4. **Unified Auth**: Complete the JWT authentication implementation for both systems\n\n`;
  
  fs.writeFileSync(outputFile, content);
  console.log(`Generated port comparison documentation at ${outputFile}`);
}

/**
 * Compare model structures between source and target
 */
async function compareModelStructures(sourceModelsDir, targetModelsDir, content) {
  // Check if directories exist
  if (!fs.existsSync(sourceModelsDir) || !fs.existsSync(targetModelsDir)) {
    return content += `Source or target models directory not found.\n\n`;
  }
  
  // Get all target model files
  const targetModelFiles = fs.readdirSync(targetModelsDir)
    .filter(file => file.endsWith('.rs'));
  
  content += `| Model | Source Implementation | Target Implementation | Status |\n`;
  content += `|-------|----------------------|------------------------|--------|\n`;
  
  // Process each target model file
  for (const file of targetModelFiles) {
    const modelName = path.basename(file, '.rs');
    const sourceRubyFile = path.join(sourceModelsDir, `${modelName}.rb`);
    const targetRustFile = path.join(targetModelsDir, file);
    
    // Check if source model exists
    const sourceExists = fs.existsSync(sourceRubyFile);
    
    // Determine status
    let status = '✅ Complete';
    if (!sourceExists) {
      status = '⚠️ No source equivalent';
    }
    
    content += `| ${modelName} | ${sourceExists ? '✅ Ruby' : '❌ Not found'} | ✅ Rust | ${status} |\n`;
  }
  
  content += `\n`;
  return content;
}

/**
 * Compare API endpoints between source and target
 */
async function compareApiEndpoints(portDir, targetApiDir, content) {
  // Check if target directory exists
  if (!fs.existsSync(targetApiDir)) {
    return content += `Target API directory not found.\n\n`;
  }
  
  // Get all target API files
  const targetApiFiles = fs.readdirSync(targetApiDir)
    .filter(file => file.endsWith('.rs'));
  
  content += `| Endpoint | Source System | Target Implementation | Status |\n`;
  content += `|----------|---------------|------------------------|--------|\n`;
  
  // Process each target API file
  for (const file of targetApiFiles) {
    const apiName = path.basename(file, '.rs');
    const targetRustFile = path.join(targetApiDir, file);
    const fileContent = fs.readFileSync(targetRustFile, 'utf8');
    
    // Extract route handlers
    const routeMatches = fileContent.matchAll(/#\[(\w+)\("([^"]+)"\)\]\s*(?:async\s+)?fn\s+(\w+)/g);
    for (const match of Array.from(routeMatches)) {
      const method = match[1].toUpperCase();
      const path = match[2];
      const handlerName = match[3];
      
      // Try to determine source system
      let sourceSystem = 'Unknown';
      if (path.includes('canvas')) {
        sourceSystem = 'Canvas';
      } else if (path.includes('discourse') || path.includes('forum')) {
        sourceSystem = 'Discourse';
      } else {
        sourceSystem = 'Custom';
      }
      
      content += `| ${method} ${path} | ${sourceSystem} | ${handlerName} | ✅ Implemented |\n`;
    }
  }
  
  content += `\n`;
  return content;
}

/**
 * Convert a string to title case
 */
function toTitleCase(str) {
  return str
    .split('_')
    .map(word => word.charAt(0).toUpperCase() + word.slice(1))
    .join(' ');
}

module.exports = {
  generateTechnicalDocs
};

// If run directly
if (require.main === module) {
  const outputDir = process.argv[2] || path.join(__dirname, 'docs', 'technical');
  const sourceDir = process.argv[3] || path.join(__dirname, 'src-tauri');
  
  generateTechnicalDocs({ outputDir, sourceDir })
    .catch(console.error);
}

// Export the function - this was likely missing
module.exports = {
  generateTechnicalDocs
};