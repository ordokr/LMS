const path = require('path');
const fs = require('fs');

/**
 * Creates optimized GitHub Copilot guidance files
 */
class AIContextGenerator {
  constructor(baseDir) {
    this.baseDir = baseDir;
    this.docsDir = path.join(baseDir, 'docs');
    this.ragDir = path.join(baseDir, 'rag_knowledge_base');
  }

  async generateAIGuidance() {
    console.log("Generating AI guidance documents...");
    
    // Read source files
    const centralHub = this.readFile(path.join(this.docsDir, 'central_reference_hub.md'));
    const lastAnalysis = this.readFile(path.join(this.baseDir, 'LAST_ANALYSIS_RESULTS.md'));
    
    // Create AI_GUIDANCE.md - optimized for GitHub Copilot
    const guidance = this.createAIGuidance(centralHub, lastAnalysis);
    fs.writeFileSync(path.join(this.baseDir, 'AI_GUIDANCE.md'), guidance);
    console.log("AI guidance file created");
    
    // Update central_reference_hub.md with AI metadata
    this.enhanceCentralHub(path.join(this.docsDir, 'central_reference_hub.md'));
    console.log("Central reference hub enhanced with AI metadata");
    
    // Create component index for component-specific references
    this.createComponentIndex();
    console.log("Component index created");

    return true;
  }

  readFile(filePath) {
    try {
      return fs.readFileSync(filePath, 'utf8');
    } catch (err) {
      console.warn(`Could not read ${filePath}: ${err.message}`);
      return '';
    }
  }

  createAIGuidance(centralHub, lastAnalysis) {
    let content = `# AI Guidance for LMS Integration Project\n\n`;
    content += `<!-- AI_METADATA
version: 1.0
priority: highest
updated: ${new Date().toISOString().split('T')[0]}
role: guidance
-->\n\n`;
    
    content += `## Documentation Hierarchy\n\n`;
    content += `1. **Primary Source of Truth**: [central_reference_hub.md](docs/central_reference_hub.md)\n`;
    content += `2. **Current Project Status**: [LAST_ANALYSIS_RESULTS.md](LAST_ANALYSIS_RESULTS.md)\n`;
    content += `3. **Knowledge Base**: [rag_knowledge_base/](rag_knowledge_base/)\n\n`;
    
    content += `## Key Project Facts\n\n`;
    
    // Extract key project facts from central hub
    const modelCount = this.extractCount(centralHub, 'Models', 'Completion');
    const apiCount = this.extractCount(centralHub, 'API Endpoints', 'Completion');
    const uiCount = this.extractCount(centralHub, 'UI Components', 'Completion');
    
    content += `- Database: **SQLite with sqlx** (embedded file database)\n`;
    content += `- Models: ${modelCount.completed}/${modelCount.total} implemented (${modelCount.percentage}%)\n`;
    content += `- API Endpoints: ${apiCount.completed}/${apiCount.total} implemented (${apiCount.percentage}%)\n`;
    content += `- UI Components: ${uiCount.completed}/${uiCount.total} implemented (${uiCount.percentage}%)\n\n`;
    
    content += `## Current Development Phase\n\n`;
    const phase = this.extractPhase(centralHub);
    content += `${phase}\n\n`;
    
    content += `## Project Structure\n\n`;
    content += `- **Frontend**: Tauri with web frontend\n`;
    content += `- **Backend**: Rust with SQLite database\n`;
    content += `- **Documentation**: Markdown in docs/ and rag_knowledge_base/\n\n`;
    
    content += `## Component Implementation Status\n\n`;
    content += `See [Component Index](docs/ai/component_index.md) for details on component implementation status.\n\n`;
    
    content += `## API Guidelines\n\n`;
    content += `- All API endpoints should be defined in the central reference hub\n`;
    content += `- Follow RESTful patterns for API design\n`;
    content += `- All endpoints should include error handling\n\n`;
    
    content += `## Model Guidelines\n\n`;
    content += `- All models should match the specifications in central_reference_hub.md\n`;
    content += `- Use SQLite types compatible with sqlx\n\n`;
    
    return content;
  }

  enhanceCentralHub(hubPath) {
    const content = this.readFile(hubPath);
    if (!content) return false;
    
    // Check if AI metadata already exists
    if (content.includes('AI_METADATA')) return false;
    
    // Add AI metadata to the top
    const enhancedContent = `<!-- AI_METADATA
version: 1.0
priority: highest
updated: ${new Date().toISOString().split('T')[0]}
role: reference
status: authoritative
-->\n\n${content}`;
    
    fs.writeFileSync(hubPath, enhancedContent);
    return true;
  }
  
  createComponentIndex() {
    const aiDocsDir = path.join(this.docsDir, 'ai');
    if (!fs.existsSync(aiDocsDir)) {
      fs.mkdirSync(aiDocsDir, { recursive: true });
    }
    
    const centralHub = this.readFile(path.join(this.docsDir, 'central_reference_hub.md'));
    const componentSection = this.extractSection(centralHub, 'UI Component Reference');
    
    let content = `# Component Implementation Index\n\n`;
    content += `<!-- AI_METADATA
version: 1.0
priority: medium
updated: ${new Date().toISOString().split('T')[0]}
role: component_reference
-->\n\n`;
    
    content += `This document provides detailed information about component implementation status.\n\n`;
    content += `## Implementation Status\n\n`;
    content += componentSection;
    
    fs.writeFileSync(path.join(aiDocsDir, 'component_index.md'), content);
    return true;
  }
  
  extractCount(content, label, column) {
    // Extract counts from table row
    const regex = new RegExp(`\\| ${label} \\| ([0-9]+)% \\|`);
    const match = content.match(regex);
    const percentage = match ? parseInt(match[1]) : 0;
    
    // Calculate approximate total/completed based on percentage
    const total = 100;
    const completed = Math.round(percentage);
    
    return { total, completed, percentage };
  }
  
  extractPhase(content) {
    // Extract overall phase
    const regex = /\*\*Overall Phase:\*\* ([^\n]+)/;
    const match = content.match(regex);
    return match ? match[1] : "Unknown";
  }
  
  extractSection(content, sectionTitle) {
    // Extract a section from the content
    const sectionRegex = new RegExp(`## ${sectionTitle}\\n\\n([\\s\\S]*?)(?=\\n## |$)`);
    const match = content.match(sectionRegex);
    return match ? match[1].trim() : "";
  }
}

// Main execution
const baseDir = process.cwd();
const generator = new AIContextGenerator(baseDir);
generator.generateAIGuidance()
  .then(() => console.log("AI context generation complete"))
  .catch(err => console.error("Error generating AI context:", err));