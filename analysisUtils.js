const path = require('path');

/**
 * Utility class for analyzing project components (Models, APIs, UI, Tests).
 */
class AnalysisUtils {
    constructor(baseDir, fsUtils, config, metrics) {
        this.baseDir = baseDir;
        this.fsUtils = fsUtils; // FileSystemUtils instance
        this.config = config;   // Analyzer configuration
        this.metrics = metrics; // Metrics object to update
    }

    /**
     * Analyze data models found by FileSystemUtils.
     */
    async analyzeModels() {
        console.log("Analyzing models...");
        const fileContentsMap = this.fsUtils.getFileContentsMap();

        // Define patterns for model files (adjust as needed for your project)
        const modelFiles = this.fsUtils.findFilesByPatterns([
            /src-tauri[\\\/]src[\\\/]models/,
            /src-tauri[\\\/]src[\\\/]entities/,
            /src-tauri[\\\/]src[\\\/]schema/,
            /model\.rs$/,
            /entity\.rs$/,
            /schema\.rs$/,
            /shared[\\\/](src|models)/, // Include shared models
        ]);

        console.log(`Found ${modelFiles.length} potential model files for analysis`);

        for (const filePath of modelFiles) {
            const content = fileContentsMap.get(filePath);
            if (!content) continue;

            const relativePath = path.relative(this.baseDir, filePath);

            if (filePath.endsWith('.rs')) {
                this._analyzeRustModels(content, relativePath);
            }
            // Add logic for other languages if necessary
        }

        this._addOrUpdateKnownModels(); // Add/update manually defined important models

        console.log(`Finished model analysis: ${this.metrics.models.total} models found (${this.metrics.models.implemented} implemented)`);
    }

    _analyzeRustModels(content, relativePath) {
        // Structs
        const structRegex = /(?:pub\s+)?struct\s+(\w+)(?:<[^>]*>)?\s*(?:\{|\()/g;
        let match;
        while ((match = structRegex.exec(content)) !== null) {
            const modelName = match[1];
            if (!['Config', 'Error', 'State', 'App', 'Args', 'Params'].some(util => modelName.includes(util))) {
                 // Basic completeness estimation
                const completeness = this._estimateRustModelCompleteness(content, modelName);
                this.addModel(modelName, relativePath, completeness);
            }
        }

        // Enums
        const enumRegex = /(?:pub\s+)?enum\s+(\w+)(?:<[^>]*>)?\s*\{/g;
        while ((match = enumRegex.exec(content)) !== null) {
            const enumName = match[1];
            if (!['Error', 'Result', 'Option', 'Response', 'Request', 'Payload', 'Query'].some(util => enumName.includes(util))) {
                 // Basic completeness estimation
                 const completeness = this._estimateRustModelCompleteness(content, enumName, true); // Mark as enum
                this.addModel(enumName, relativePath, completeness);
            }
        }
    }

     _estimateRustModelCompleteness(content, modelName, isEnum = false) {
        let score = isEnum ? 15 : 20; // Base score
        const modelRegex = new RegExp(`(?:struct|enum)\\s+${modelName}[^\{]*\\{([\\s\\S]*?)\\}`, 'm');
        const bodyMatch = content.match(modelRegex);
        if (!bodyMatch) return score;

        const body = bodyMatch[1];

        // Field count (simple estimation)
        const fieldCount = (body.match(/\bpub\b\s+\w+\s*:/g) || []).length;
        score += Math.min(15, fieldCount * 2);

        // Derive macros
        const deriveMatch = content.match(new RegExp(`#\\[derive\\(([^)]+)\\)\\]\\s*(?:pub\\s+)?(?:struct|enum)\\s+${modelName}`));
        if (deriveMatch) {
            const derives = deriveMatch[1].split(',').map(d => d.trim());
            if (derives.includes('Serialize')) score += 5;
            if (derives.includes('Deserialize')) score += 5;
            if (derives.includes('Debug')) score += 2;
            if (derives.includes('Clone')) score += 3;
            if (derives.includes('PartialEq')) score += 2;
            if (derives.includes('Eq')) score += 1;
            if (derives.includes('EntityModel')) score += 10; // SeaORM specific
        }

        // Basic documentation check
        const docCommentRegex = new RegExp(`///.*\n\\s*(?:#\\[derive.*\\]\n)?\\s*(?:pub\\s+)?(?:struct|enum)\\s+${modelName}`);
        if (docCommentRegex.test(content)) {
            score += 5;
        }

        // Check for associated impl blocks
        const implRegex = new RegExp(`impl(?:<[^>]*>)?\\s+${modelName}\\s*\\{`);
        if (implRegex.test(content)) {
            score += 10; // Presence of methods/associated functions
        }

        return Math.min(95, score);
    }


    _addOrUpdateKnownModels() {
        const knownModels = [
            { name: 'User', file: 'src-tauri/src/models/user.rs', completeness: 60 }, // Reduced from 75%
            { name: 'Course', file: 'src-tauri/src/models/course.rs', completeness: 55 }, // Reduced from 70%
            { name: 'Topic', file: 'src-tauri/src/models/topic.rs', completeness: 50 }, // Reduced from 65%
            { name: 'Post', file: 'src-tauri/src/models/post.rs', completeness: 45 }, // Reduced from 60%
            { name: 'Category', file: 'src-tauri/src/models/category.rs', completeness: 45 }, // Reduced from 60%
        ];

        for (const model of knownModels) {
            const existing = this.metrics.models.details.find(m => m.name === model.name);
            if (!existing) {
                this.addModel(model.name, model.file, model.completeness);
            } else if (model.completeness > existing.completeness) {
                 // Update if manual completeness is higher
                existing.completeness = model.completeness;
                 // Recalculate implemented count if threshold is now met/unmet (optional, depends on desired logic)
                 this._recalculateImplementedModels();
            }
        }
    }

     _recalculateImplementedModels() {
        this.metrics.models.implemented = this.metrics.models.details.filter(
            m => m.completeness >= this.config.implementationThreshold
        ).length;
    }


    /**
     * Add a model to metrics, avoiding duplicates.
     */
    addModel(name, filePath, completeness) {
        if (this.metrics.models.details.some(m => m.name === name && m.file === filePath)) {
            return; // Already added
        }

        this.metrics.models.total++;
        this.metrics.models.details.push({ name, file: filePath, completeness });

        if (completeness >= this.config.implementationThreshold) {
            this.metrics.models.implemented++;
        }
    }

    /**
     * Analyze API endpoints found by FileSystemUtils.
     */
    async analyzeApiEndpoints() {
        console.log("Analyzing API endpoints...");
        const fileContentsMap = this.fsUtils.getFileContentsMap();

        const apiFiles = this.fsUtils.findFilesByPatterns([
            /src-tauri[\\\/]src[\\\/]api/,
            /src-tauri[\\\/]src[\\\/]routes/,
            /src-tauri[\\\/]src[\\\/]main\.rs/,
            /controller\.rs$/,
            /handler\.rs$/,
            /router\.rs$/,
            /service\.rs$/,
        ]);

        console.log(`Found ${apiFiles.length} potential API files for analysis`);

        for (const filePath of apiFiles) {
            const content = fileContentsMap.get(filePath);
            if (!content) continue;
            const relativePath = path.relative(this.baseDir, filePath);

            if (filePath.endsWith('.rs')) {
                this._analyzeRustApiEndpoints(content, relativePath);
            }
            // Add logic for other languages if necessary
        }

        console.log(`Finished API endpoint analysis: ${this.metrics.apiEndpoints.total} endpoints found (${this.metrics.apiEndpoints.implemented} implemented)`);
        
        // Log more details about the detected endpoints and their scores
        console.log("API Endpoint scores (debug):");
        const implementationThreshold = this.config.implementationThreshold || 35;
        console.log(`Implementation threshold: ${implementationThreshold}`);
        
        // Log some sample scores
        const samplesToShow = Math.min(5, this.metrics.apiEndpoints.details.length);
        for (let i = 0; i < samplesToShow; i++) {
            const endpoint = this.metrics.apiEndpoints.details[i];
            console.log(`- ${endpoint.name}: ${endpoint.completeness}% (${endpoint.completeness >= implementationThreshold ? 'Implemented' : 'Not implemented'})`);
        }
    }

    _analyzeRustApiEndpoints(content, relativePath) {
        const routePatterns = [
            // Axum/Hyper/etc. route definitions
            /\.(?:route|get|post|put|delete|patch)\s*\(\s*['"](.*?)['"],\s*(.*?)(?:,|\))/g,
            // Tauri commands
            /#\[command\]\s*\n\s*(?:pub\s+)?(?:async\s+)?fn\s+(\w+)/g,
            // Actix-web style attributes
            /#\[(?:get|post|put|delete|patch)\s*\(\s*['"](.*?)['"]\)\]\s*(?:pub\s+)?(?:async\s+)?fn\s+(\w+)/g
        ];

        for (const regex of routePatterns) {
            let match;
            while ((match = regex.exec(content)) !== null) {
                let handlerName, routePath, completeness, featureArea;

                if (regex.source.includes('#\\[command\\]')) {
                    handlerName = match[1];
                    routePath = `[Tauri Command]`;
                    completeness = this.estimateApiCompleteness(content, handlerName, relativePath); // Estimate based on function body
                    featureArea = this.determineApiFeatureArea(handlerName, relativePath);
                } else if (regex.source.includes('#\\[(?:get|post')) {
                    routePath = match[1];
                    handlerName = match[2];
                     completeness = this.estimateApiCompleteness(content, handlerName, relativePath);
                    featureArea = this.determineApiFeatureArea(handlerName, relativePath, routePath);
                } else {
                    routePath = match[1];
                    const handlerPart = match[2].trim();
                    const handlerMatch = handlerPart.match(/(?:get|post|put|delete|patch)\s*\(\s*([^)]+)\s*\)/);
                    handlerName = handlerMatch ? handlerMatch[1].trim() : handlerPart;

                    if (handlerName.includes('Router::') || handlerName.includes("||") || ['move', 'user', 'with', 'state'].includes(handlerName.toLowerCase())) {
                        continue; // Skip middleware/complex closures
                    }
                     completeness = this.estimateApiCompleteness(content, handlerName, relativePath);
                    featureArea = this.determineApiFeatureArea(handlerName, relativePath, routePath);
                }
                 this.addApiEndpoint(handlerName, relativePath, completeness, featureArea, routePath);
            }
        }
    }

    /**
     * Estimate API implementation completeness. (Placeholder)
     */
    estimateApiCompleteness(body, functionName, filePath) {
        try {
            // Escape special regex characters in the function name
            const escapedFunctionName = functionName.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
            const functionBodyRegex = new RegExp(`fn\\s+${escapedFunctionName}[^{]*\\{([\\s\\S]*?)\\}`, 'm');
            
            const match = body.match(functionBodyRegex);
            if (!match) return 20; // Function signature found but body not extracted
            
            const functionBody = match[1];
            
            // Now add the actual implementation to estimate completeness
            let score = 35; // Base score for having a function body
            
            // Check complexity by counting lines and statements
            const lines = functionBody.split('\n').filter(line => line.trim().length > 0);
            score += Math.min(15, lines.length); // Add points based on lines of code (up to 15)
            
            // Check for return statements
            if (functionBody.includes('return')) {
                score += 5;
            }
            
            // Check for error handling
            if (functionBody.includes('error') || functionBody.includes('Error') || 
                functionBody.includes('err') || functionBody.includes('Err(') ||
                functionBody.includes('?') || functionBody.includes('Result')) {
                score += 10;
            }
            
            // Check for database operations
            if (functionBody.includes('query') || functionBody.includes('db.') || 
                functionBody.includes('select') || functionBody.includes('insert') ||
                functionBody.includes('update') || functionBody.includes('delete')) {
                score += 15;
            }
            
            // Check for models usage
            const modelUsage = this.metrics.models.details.some(model => 
                functionBody.includes(model.name));
            if (modelUsage) {
                score += 10;
            }
            
            // Check for response formatting
            if (functionBody.includes('json') || functionBody.includes('Response')) {
                score += 5;
            }
            
            // Cap at 95 to leave room for final improvements
            return Math.min(95, score);
        } catch (error) {
            console.warn(`Error analyzing API endpoint ${functionName}: ${error.message}`);
            return 25; // Return a default value
        }
    }

    /**
     * Add an API endpoint to metrics, avoiding duplicates.
     */
    addApiEndpoint(name, filePath, completeness, featureArea = 'other', routePath = null) {
         // Normalize name if it includes path separators
         const handlerName = name.split('::').pop();

         if (this.metrics.apiEndpoints.details.some(ep => ep.name === handlerName && ep.file === filePath)) {
            return; // Already added
        }

        this.metrics.apiEndpoints.total++;
        this.metrics.apiEndpoints.details.push({
            name: handlerName,
            file: filePath,
            completeness,
            featureArea,
            route: routePath
        });

        const apiImplementationThreshold = 35; // Lower threshold

        if (completeness >= apiImplementationThreshold) {
            this.metrics.apiEndpoints.implemented++;
            if (this.metrics.featureAreas[featureArea]) {
                this.metrics.featureAreas[featureArea].implemented++;
            }
        }
         // Increment total count for the feature area regardless of implementation status
         if (this.metrics.featureAreas[featureArea]) {
            this.metrics.featureAreas[featureArea].total++;
        }
    }

     /**
     * Determine feature area based on file path or name.
     */
     determineApiFeatureArea(name = '', filePath = '', routePath = '') {
        const lowerFilePath = filePath.toLowerCase();
        const lowerName = name.toLowerCase();
        const lowerRoute = (routePath || '').toLowerCase();

        if (lowerFilePath.includes('auth') || lowerName.includes('auth') || lowerRoute.includes('auth') || lowerName.includes('login') || lowerName.includes('register')) return 'auth';
        if (lowerFilePath.includes('forum') || lowerName.includes('forum') || lowerRoute.includes('forum') || lowerName.includes('topic') || lowerName.includes('post') || lowerName.includes('category')) return 'forum';
        if (lowerFilePath.includes('lms') || lowerName.includes('lms') || lowerRoute.includes('lms') || lowerName.includes('course') || lowerName.includes('module') || lowerName.includes('assignment') || lowerName.includes('submission')) return 'lms';
        if (lowerFilePath.includes('integrat') || lowerName.includes('integrat') || lowerRoute.includes('integrat') || lowerName.includes('sync') || lowerRoute.includes('sync')) return 'integration';

        return 'other';
    }


    /**
     * Analyze UI components found by FileSystemUtils.
     */
    async analyzeUIComponents() {
        console.log("Analyzing UI components (Leptos)...");
        const fileContentsMap = this.fsUtils.getFileContentsMap();

        // Focus on files likely containing Leptos components
        const uiFiles = this.fsUtils.findFilesByPatterns([
            /src[\\\/]components/,
            /src[\\\/]pages/,
            /src[\\\/]features/,
            /src[\\\/]app\.rs/,
        ]);

         console.log(`Found ${uiFiles.length} potential UI files for analysis`);

        for (const filePath of uiFiles) {
             if (!filePath.endsWith('.rs')) continue; // Only Rust files for Leptos

            const content = fileContentsMap.get(filePath);
            if (!content) continue;
            const relativePath = path.relative(this.baseDir, filePath);

            this._analyzeLeptosComponents(content, relativePath);
        }
         console.log(`Finished UI component analysis: ${this.metrics.uiComponents.total} components found (${this.metrics.uiComponents.implemented} implemented)`);
    }

     _analyzeLeptosComponents(content, relativePath) {
        const componentRegex = /#\[component(?:\([^)]*\))?\]\s*(?:pub\s+)?fn\s+([A-Z]\w*)/g;
        const intoViewRegex = /(?:pub\s+)?fn\s+([A-Z]\w*)\s*\([^)]*\)\s*->\s*impl\s+IntoView/g;
        let match;
        let componentsFound = new Set(); // Use Set to avoid duplicates within the same file scan

        while ((match = componentRegex.exec(content)) !== null) {
            componentsFound.add(match[1]);
        }
        while ((match = intoViewRegex.exec(content)) !== null) {
            componentsFound.add(match[1]); // Add even if found by #[component] already, Set handles duplicates
        }

        for (const componentName of componentsFound) {
            const completeness = this.estimateLeptosComponentCompleteness(content, componentName);
            this.addUIComponent(componentName, relativePath, completeness);
        }
    }


    /**
     * Estimate Leptos component completeness.
     */
    estimateLeptosComponentCompleteness(content, componentName) {
        let score = 20;
        const componentBodyMatch = content.match(new RegExp(`fn\\s+${componentName}[^\{]*\\{([\\s\\S]*?)\\}`, 'm'));
        if (!componentBodyMatch) return score;
        const body = componentBodyMatch[1];

        if (body.includes("create_signal")) score += 15;
        if (body.includes("create_resource")) score += 15;
        if (body.includes("create_action")) score += 10;
        if (body.includes("view!")) score += 10;
        if (body.includes("on:")) score += 5;
        if (body.includes("<")) score += 5; // Basic check for child components/HTML elements
        if (body.includes("prop:")) score += 5;
        if (body.includes("leptos::")) score += 3; // Using leptos crate directly

        return Math.min(95, score);
    }

    /**
     * Add a UI component to metrics, avoiding duplicates.
     */
    addUIComponent(name, filePath, completeness) {
        if (this.metrics.uiComponents.details.some(c => c.name === name && c.file === filePath)) {
            return; // Already added
        }

        this.metrics.uiComponents.total++;
        this.metrics.uiComponents.details.push({ name, file: filePath, completeness });

        if (completeness >= this.config.implementationThreshold) {
            this.metrics.uiComponents.implemented++;
        }
    }

    /**
     * Analyze tests found by FileSystemUtils.
     */
    async analyzeTests() {
        console.log("Analyzing tests...");
        const fileContentsMap = this.fsUtils.getFileContentsMap();

        const testFiles = this.fsUtils.findFilesByPatterns([
            /tests?[\\\/]/,
            /src[\\\/].*test\.rs$/,
            /#\[test\]/, // Look for #[test] attribute in any .rs file
        ]);
        const uniqueTestFiles = [...new Set(testFiles)]; // Deduplicate

        console.log(`Found ${uniqueTestFiles.length} potential test files/files containing tests`);

        for (const filePath of uniqueTestFiles) {
            const content = fileContentsMap.get(filePath);
            if (!content) continue;
            const relativePath = path.relative(this.baseDir, filePath);

            if (filePath.endsWith('.rs')) {
                this._analyzeRustTests(content, relativePath);
            }
            // Add logic for JS tests if necessary
        }

        // Basic coverage estimation (placeholder)
        this.metrics.tests.coverage = this.metrics.tests.total > 0
            ? Math.round((this.metrics.tests.passing / this.metrics.tests.total) * 15) // Very rough estimate
            : 0;

        console.log(`Finished test analysis: ${this.metrics.tests.total} tests found (${this.metrics.tests.passing} assumed passing)`);
    }

    _analyzeRustTests(content, relativePath) {
        const testFuncRegex = /#\[test\]\s*(?:async\s+)?fn\s+(\w+)\s*\(\)/g;
        let match;
        while ((match = testFuncRegex.exec(content)) !== null) {
            const testName = match[1];
            // Assume passing for now, real analysis would need test execution results
            this.addTest(testName, relativePath, true);
        }
    }

    /**
     * Add a test to metrics, avoiding duplicates.
     */
    addTest(name, filePath, passing = false) {
         if (this.metrics.tests.details.some(t => t.name === name && t.file === filePath)) {
            return; // Already added
        }

        this.metrics.tests.total++;
        this.metrics.tests.details.push({ name, file: filePath, passing });

        if (passing) {
            this.metrics.tests.passing++;
        }
    }

    /**
     * Analyze code quality metrics (complexity, tech debt)
     */
    async analyzeCodeQuality(astAnalyzer) {
        console.log("Analyzing code quality...");
        
        // Initialize metrics structure if it doesn't exist
        if (!this.metrics.codeQuality) {
            this.metrics.codeQuality = {
                complexity: {
                    average: 0,
                    high: 0,
                    files: []
                },
                techDebt: {
                    score: 0,
                    items: []
                }
            };
        }
        
        // Initialize counters
        let totalComplexity = 0;
        let fileCount = 0;
        let highComplexityCount = 0;
        const complexityThreshold = 10; // Lower threshold to catch more files
        
        // Debug counters
        let jsFilesProcessed = 0, rsFilesProcessed = 0, otherFilesProcessed = 0;
        
        const fileContentsMap = this.fsUtils.getFileContentsMap();
        for (const [filePath, content] of fileContentsMap.entries()) {
            if (!content || content.trim().length === 0) continue;
            
            const relativePath = path.relative(this.baseDir, filePath);
            let complexity = 1;
            
            // Calculate complexity based on file type
            if (filePath.endsWith('.js') || filePath.endsWith('.ts') || 
                filePath.endsWith('.jsx') || filePath.endsWith('.tsx')) {
                const ast = astAnalyzer.parseToAst(content, relativePath);
                if (ast) {
                    complexity = astAnalyzer.calculateComplexity(ast);
                    jsFilesProcessed++;
                }
            } else if (filePath.endsWith('.rs')) {
                complexity = astAnalyzer.estimateRustComplexity(content);
                rsFilesProcessed++;
            } else {
                // Basic complexity for other files
                complexity = Math.max(1, Math.floor(content.split('\n').length / 40));
                otherFilesProcessed++;
            }
            
            // Debug output for some files
            if (fileCount < 5 || complexity > complexityThreshold) {
                console.log(`File: ${relativePath}, Complexity: ${complexity}`);
            }
            
            // Update metrics
            totalComplexity += complexity;
            fileCount++;
            
            // Check for high complexity
            if (complexity > complexityThreshold) {
                highComplexityCount++;
                // Add to metrics...
            }
        }
        
        // Log processing stats
        console.log(`Processed ${fileCount} files (${jsFilesProcessed} JS/TS, ${rsFilesProcessed} Rust, ${otherFilesProcessed} other)`);
        
        // Calculate averages
        this.metrics.codeQuality.complexity.average = fileCount > 0 ? 
            Math.round((totalComplexity / fileCount) * 10) / 10 : 0;
        this.metrics.codeQuality.complexity.high = highComplexityCount;
        
        // ...rest of the method...
    }

    /**
     * Find relationships between model entities
     */
    async findModelRelationships() {
        console.log("Finding model relationships...");
        const fileContentsMap = this.fsUtils.getFileContentsMap();
        const modelNames = this.metrics.models.details.map(m => m.name);
        
        // Clear existing relationships
        this.metrics.relationships = [];
        
        // For each model file, look for references to other models
        this.metrics.models.details.forEach(model => {
            const filePath = path.join(this.baseDir, model.file);
            const content = fileContentsMap.get(filePath);
            if (!content) return;
    
            // Look for references to other models
            modelNames.forEach(otherModel => {
                if (model.name === otherModel) return; // Skip self references
                
                // Check for references in fields or comments
                const pattern = new RegExp(`${otherModel}(\\s|:|,|<|>|\\[|\\]|\\{|\\}|\\.|\\(|\\)|"|'|_id|Id|s|es)`, 'g');
                const matches = content.match(pattern);
                
                if (matches) {
                    // Check relationship type (basic heuristic)
                    let relationType = 'Reference';
                    if (content.includes(`Vec<${otherModel}>`) || 
                        content.includes(`${otherModel}[]`) || 
                        content.includes(`${otherModel}s`) ||
                        content.includes(`${otherModel.toLowerCase()}s`)) {
                      relationType = 'OneToMany';
                    }
                    
                    // Add relationship if not already exists
                    const existingRel = this.metrics.relationships.find(
                      r => r.from === model.name && r.to === otherModel
                    );
                    
                    if (!existingRel) {
                      this.metrics.relationships.push({
                        from: model.name,
                        to: otherModel,
                        type: relationType
                      });
                    }
                }
            });
        });
        
        console.log(`Found ${this.metrics.relationships.length} model relationships`);
    }
}

module.exports = AnalysisUtils;