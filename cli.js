#!/usr/bin/env node
const { program } = require('commander');
const path = require('path');
const fs = require('fs'); // Add this import
const UnifiedProjectAnalyzer = require('./unified-project-analyzer');

// Setup the program
program
  .name('lms-analyzer')
  .description('LMS Project Analysis Tools')
  .version('1.0.0');

// Add analyze command
program
  .command('analyze')
  .description('Perform full code analysis')
  .option('-i, --incremental', 'Perform incremental analysis')
  .option('--no-ai', 'Disable AI analysis features')
  .action(async (options) => {
    try {
      const analyzer = new UnifiedProjectAnalyzer(process.cwd(), {}, { 
        incremental: options.incremental,
        useAI: options.ai
      });
      
      await analyzer.analyze();
      await analyzer.generateAllReports();
      
      console.log("Analysis completed successfully");
    } catch (error) {
      console.error("Analysis failed:", error.message);
      console.error(error.stack);
      process.exit(1);
    }
  });

// Add generate-rag command - UPDATED FOR PORT DIRECTORY
program
  .command('generate-rag')
  .description('Generate RAG knowledge base documents')
  .option('-d, --dir <directory>', 'Base directory', process.cwd())
  .action(async (options) => {
    try {
      console.log(`Generating RAG documents in ${options.dir}`);
      
      // Create the analyzer with source systems configuration
      // UPDATED: Using port directory structure
      const analyzer = new UnifiedProjectAnalyzer(options.dir, {
        canvas: { path: path.join('C:\\Users\\Tim\\Desktop\\port\\canvas') },
        discourse: { path: path.join('C:\\Users\\Tim\\Desktop\\port\\discourse') }
      });
      
      // Generate RAG documents
      await analyzer.generateRagDocuments();
      
      console.log("RAG document generation completed successfully");
    } catch (error) {
      console.error("RAG document generation failed:", error.message);
      console.error(error.stack);
      process.exit(1);
    }
  });

// Add rag-query command - UPDATED FOR PORT DIRECTORY
program
  .command('rag-query')
  .description('Query the RAG system for information')
  .argument('<query>', 'The query to search for')
  .option('-s, --system <system>', 'Filter by system (canvas, discourse, integration)')
  .option('-c, --category <category>', 'Filter by category (models, architecture, apis, etc.)')
  .option('-t, --top <number>', 'Number of results to return', parseInt, 3)
  .option('--context', 'Generate context for LLM', false)
  .option('--fallback', 'Use fallback search if vector search fails', false)
  .action(async (query, options) => {
    try {
      console.log(`Querying RAG system: "${query}"`);
      
      const baseDir = process.cwd();
      const analyzer = new UnifiedProjectAnalyzer(baseDir, {
        canvas: { path: path.join('C:\\Users\\Tim\\Desktop\\port\\canvas') },
        discourse: { path: path.join('C:\\Users\\Tim\\Desktop\\port\\discourse') }
      });
      
      // Build filter
      const filters = {};
      if (options.system) filters.system = options.system;
      if (options.category) filters.category = options.category;
      
      let results;
      
      try {
        // Try vector search first
        results = await analyzer.queryRag(query, {
          filters,
          topK: options.top,
          generateContext: options.context
        });
      } catch (error) {
        console.log("Vector search failed, using text-based fallback search...");
        
        // Create a simple fallback search function
        const fallbackSearch = async () => {
          const ragDir = path.join(baseDir, 'rag_knowledge_base');
          const documents = [];
          
          // Helper function to recursively search directories
          const searchDir = (dir) => {
            const files = fs.readdirSync(dir, { withFileTypes: true });
            
            for (const file of files) {
              const fullPath = path.join(dir, file.name);
              
              if (file.isDirectory()) {
                searchDir(fullPath);
              } else if (file.name.endsWith('.md')) {
                const content = fs.readFileSync(fullPath, 'utf8');
                const relativePath = path.relative(ragDir, fullPath);
                
                // Super simple keyword matching
                const lowerContent = content.toLowerCase();
                const lowerQuery = query.toLowerCase();
                const queryWords = lowerQuery.split(/\s+/).filter(w => w.length > 2);
                
                // Calculate a simple relevance score based on keyword occurrences
                let score = 0;
                let matches = 0;
                
                for (const word of queryWords) {
                  const wordMatches = (lowerContent.match(new RegExp(word, 'g')) || []).length;
                  if (wordMatches > 0) {
                    matches++;
                    score += wordMatches;
                  }
                }
                
                // Only include documents with at least one matching term
                if (matches > 0) {
                  const system = relativePath.split(path.sep)[0] || 'unknown';
                  const category = relativePath.split(path.sep)[1] || 'general';
                  
                  // Filter by system and category if specified
                  if ((filters.system && system !== filters.system) || 
                      (filters.category && category !== filters.category)) {
                    continue;
                  }
                  
                  documents.push({
                    id: relativePath,
                    score: score / queryWords.length,
                    content: content,
                    metadata: {
                      system,
                      category,
                      path: fullPath
                    }
                  });
                }
              }
            }
          };
          
          try {
            searchDir(ragDir);
            
            // Sort by score and limit to topK
            const sortedDocs = documents.sort((a, b) => b.score - a.score)
                                        .slice(0, options.top);
            
            return {
              query,
              timestamp: new Date().toISOString(),
              resultsCount: sortedDocs.length,
              documents: sortedDocs
            };
          } catch (err) {
            console.error("Fallback search error:", err);
            return {
              query,
              timestamp: new Date().toISOString(),
              resultsCount: 0,
              documents: []
            };
          }
        };
        
        // Run the fallback search
        results = await fallbackSearch();
      }
      
      // Display results
      console.log(`\nFound ${results.resultsCount} relevant documents for: "${query}"\n`);
      
      if (results.documents.length === 0) {
        console.log("No results found. Try a different query or filters.");
        return;
      }
      
      // Show results
      results.documents.forEach((doc, index) => {
        console.log(`${index + 1}. ${doc.id} (Score: ${doc.score.toFixed(2)})`);
        console.log(`   System: ${doc.metadata?.system || 'unknown'}, Category: ${doc.metadata?.category || 'unknown'}`);
        
        // Show snippet
        if (doc.content) {
          const snippet = doc.content.substring(0, 150).replace(/\n/g, ' ') + '...';
          console.log(`   ${snippet}`);
        }
        console.log();
      });
      
      if (options.context && results.context) {
        console.log("\n--- LLM Context ---\n");
        console.log(results.context);
      }
    } catch (error) {
      console.error("RAG query failed:", error.message);
      console.error(error.stack);
      process.exit(1);
    }
  });

// Add this command to your existing CLI commands
program
  .command('update-integration')
  .description('Update Canvas-Discourse integration documentation')
  .action(async () => {
    try {
      console.log("Updating Canvas-Discourse integration documentation...");
      
      const IntegrationReportGenerator = require('./modules/integration-report-generator');
      const generator = new IntegrationReportGenerator({
        baseDir: process.cwd(),
        ragKnowledgeBase: 'rag_knowledge_base',
        outputDir: 'docs'
      });
      
      // Generate the report
      const reportPath = await generator.generateReport();
      
      // Update the central reference hub
      const hubPath = path.join(process.cwd(), 'docs', 'central_reference_hub.md');
      generator.updateCentralReferenceHub(hubPath, reportPath);
      
      console.log("Canvas-Discourse integration documentation updated successfully");
    } catch (error) {
      console.error("Failed to update integration documentation:", error.message);
      console.error(error.stack);
      process.exit(1);
    }
  });

// Add this command to your CLI
program
  .command('update-tech-docs')
  .description('Update technical implementation documentation from source code')
  .option('-p, --patterns <patterns>', 'Comma-separated glob patterns for source files', 
    'services/integration/**/*.js,plugins/discourse/**/*.rb')
  .option('-o, --output <directory>', 'Output directory for documentation', 
    'rag_knowledge_base/integration')
  .action(async (options) => {
    try {
      console.log("Updating technical implementation documentation...");
      
      const TechnicalDocsGenerator = require('./modules/technical-docs-generator');
      const generator = new TechnicalDocsGenerator({
        baseDir: process.cwd(),
        outputDir: path.join(process.cwd(), options.output),
        sourcePatterns: options.patterns.split(',')
      });
      
      // Generate the documentation
      const docPath = await generator.generate();
      
      console.log(`Technical implementation documentation generated at: ${docPath}`);
      
      // Update the central reference hub
      const IntegrationReportGenerator = require('./modules/integration-report-generator');
      const integrationGenerator = new IntegrationReportGenerator({
        baseDir: process.cwd(),
        ragKnowledgeBase: 'rag_knowledge_base',
        outputDir: 'docs'
      });
      
      const reportPath = await integrationGenerator.generateReport();
      
      // Update the hub
      const hubPath = path.join(process.cwd(), 'docs', 'central_reference_hub.md');
      integrationGenerator.updateCentralReferenceHub(hubPath, reportPath);
      
      console.log("Canvas-Discourse integration documentation updated successfully");
    } catch (error) {
      console.error("Failed to update technical documentation:", error.message);
      console.error(error.stack);
      process.exit(1);
    }
  });

// Parse arguments
program.parse(process.argv);

// Display help if no command specified
if (!process.argv.slice(2).length) {
  program.outputHelp();
}