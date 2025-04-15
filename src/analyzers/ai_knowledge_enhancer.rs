use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Local, Utc};

use crate::analyzers::unified_analyzer::AnalysisResult;

/// AI Knowledge Enhancer that generates specialized documentation for AI agents
pub struct AiKnowledgeEnhancer {
    base_dir: PathBuf,
    knowledge_dir: PathBuf,
}

#[derive(Debug, Serialize, Deserialize)]
struct KnowledgeSection {
    title: String,
    content: String,
    keywords: Vec<String>,
    importance: u8, // 1-10 with 10 being highest
}

#[derive(Debug, Serialize, Deserialize)]
struct ProjectKnowledge {
    sections: Vec<KnowledgeSection>,
    last_updated: DateTime<Utc>,
}

impl AiKnowledgeEnhancer {
    pub fn new(base_dir: PathBuf) -> Self {
        let knowledge_dir = base_dir.join("ai_knowledge");
        
        Self {
            base_dir,
            knowledge_dir,
        }
    }
    
    /// Generate enhanced knowledge base for AI agents
    pub fn enhance_knowledge_base(&self, result: &AnalysisResult) -> Result<PathBuf, String> {
        println!("Enhancing AI knowledge base...");
        
        // Ensure knowledge directory exists
        if !self.knowledge_dir.exists() {
            fs::create_dir_all(&self.knowledge_dir)
                .map_err(|e| format!("Failed to create AI knowledge directory: {}", e))?;
        }
        
        // Generate structured knowledge from analysis
        let project_knowledge = self.extract_structured_knowledge(result);
        
        // Write to knowledge file
        let knowledge_file = self.knowledge_dir.join("project_knowledge.json");
        let json = serde_json::to_string_pretty(&project_knowledge)
            .map_err(|e| format!("Failed to serialize knowledge: {}", e))?;
        
        fs::write(&knowledge_file, json)
            .map_err(|e| format!("Failed to write knowledge file: {}", e))?;
        
        println!("AI knowledge base enhanced at {:?}", knowledge_file);
        
        Ok(knowledge_file)
    }
    
    /// Extract structured knowledge from analysis result
    fn extract_structured_knowledge(&self, result: &AnalysisResult) -> ProjectKnowledge {
        let mut sections = Vec::new();
        
        // Project overview section
        sections.push(KnowledgeSection {
            title: "Project Overview".to_string(),
            content: format!(
                "The project is currently in the {} phase with {:.1}% completion. \
                The most active area recently has been {}.",
                result.project_status.phase,
                result.project_status.completion_percentage,
                result.project_status.last_active_area
            ),
            keywords: vec!["overview".to_string(), "status".to_string(), "phase".to_string()],
            importance: 10,
        });
        
        // Architecture section
        sections.push(KnowledgeSection {
            title: "Architecture".to_string(),
            content: format!(
                "The project follows these architectural patterns: {}. \
                And adheres to these principles: {}.",
                result.architecture.patterns.join(", "),
                result.architecture.principles.join(", ")
            ),
            keywords: vec!["architecture".to_string(), "patterns".to_string(), "principles".to_string()],
            importance: 9,
        });
        
        // Models section
        sections.push(KnowledgeSection {
            title: "Data Models".to_string(),
            content: format!(
                "The project has implemented {}/{} data models ({:.1}% complete).",
                result.models.implemented,
                result.models.total,
                result.models.implementation_percentage
            ),
            keywords: vec!["models".to_string(), "data".to_string(), "entities".to_string()],
            importance: 8,
        });
        
        // API section
        sections.push(KnowledgeSection {
            title: "API Endpoints".to_string(),
            content: format!(
                "The project has implemented {}/{} API endpoints ({:.1}% complete).",
                result.api_endpoints.implemented,
                result.api_endpoints.total,
                result.api_endpoints.implementation_percentage
            ),
            keywords: vec!["api".to_string(), "endpoints".to_string(), "routes".to_string()],
            importance: 8,
        });
        
        // UI section
        sections.push(KnowledgeSection {
            title: "UI Components".to_string(),
            content: format!(
                "The project has implemented {}/{} UI components ({:.1}% complete).",
                result.ui_components.implemented,
                result.ui_components.total,
                result.ui_components.implementation_percentage
            ),
            keywords: vec!["ui".to_string(), "components".to_string(), "interface".to_string()],
            importance: 7,
        });
        
        // Code quality section
        let mut quality_content = String::from("Code quality metrics:\n");
        for (metric, value) in &result.code_quality.metrics {
            quality_content.push_str(&format!("- {}: {:.1}\n", metric, value));
        }
        
        sections.push(KnowledgeSection {
            title: "Code Quality".to_string(),
            content: quality_content,
            keywords: vec!["quality".to_string(), "metrics".to_string(), "complexity".to_string()],
            importance: 6,
        });
        
        // Testing section
        sections.push(KnowledgeSection {
            title: "Testing".to_string(),
            content: format!(
                "The project has {}/{} tests passing with {:.1}% coverage.",
                result.tests.passing,
                result.tests.total,
                result.tests.coverage
            ),
            keywords: vec!["tests".to_string(), "coverage".to_string(), "quality".to_string()],
            importance: 7,
        });
        
        // Integration section
        sections.push(KnowledgeSection {
            title: "Integration".to_string(),
            content: format!(
                "The project has implemented {}/{} integration points ({:.1}% complete).",
                result.integration.implemented,
                result.integration.total,
                result.integration.implementation_percentage
            ),
            keywords: vec!["integration".to_string(), "external".to_string(), "systems".to_string()],
            importance: 6,
        });
        
        // Blockchain section
        sections.push(KnowledgeSection {
            title: "Blockchain".to_string(),
            content: format!(
                "Blockchain implementation status: {}. Features: {}.",
                result.blockchain.implementation_status,
                result.blockchain.features.join(", ")
            ),
            keywords: vec!["blockchain".to_string(), "distributed".to_string(), "ledger".to_string()],
            importance: 5,
        });
        
        // Recommendations section
        let mut recommendations_content = String::from("Key recommendations:\n");
        for rec in &result.recommendations {
            recommendations_content.push_str(&format!(
                "- [Priority: {}] {}: {}\n", 
                rec.priority, 
                rec.title,
                rec.description
            ));
        }
        
        sections.push(KnowledgeSection {
            title: "Recommendations".to_string(),
            content: recommendations_content,
            keywords: vec!["recommendations".to_string(), "improvements".to_string(), "next".to_string()],
            importance: 9,
        });
        
        ProjectKnowledge {
            sections,
            last_updated: Utc::now(),
        }
    }
    
    /// Generate AI agent guidance document
    pub fn generate_agent_guidance(&self, result: &AnalysisResult) -> Result<PathBuf, String> {
        println!("Generating AI agent guidance...");
        
        // Ensure docs directory exists
        let docs_dir = self.base_dir.join("docs");
        if !docs_dir.exists() {
            fs::create_dir_all(&docs_dir)
                .map_err(|e| format!("Failed to create docs directory: {}", e))?;
        }
        
        // Generate guidance content
        let mut content = String::from("# AI Development Guidance\n\n");
        content.push_str("This document provides guidance for AI agents working on this project.\n\n");
        
        // Project overview
        content.push_str("## Project Overview\n\n");
        content.push_str(&format!(
            "The project is currently in the **{}** phase with **{:.1}%** completion. \
            The most active area recently has been **{}**.\n\n",
            result.project_status.phase,
            result.project_status.completion_percentage,
            result.project_status.last_active_area
        ));
        
        // Architecture guidance
        content.push_str("## Architecture Guidance\n\n");
        content.push_str("When developing new features or modifying existing ones, adhere to the following principles:\n\n");
        
        for principle in &result.architecture.principles {
            content.push_str(&format!("- **{}**\n", principle));
        }
        
        content.push_str("\nImplement these design patterns when appropriate:\n\n");
        
        for pattern in &result.architecture.patterns {
            content.push_str(&format!("- **{}**\n", pattern));
        }
        
        // Development priorities
        content.push_str("\n## Development Priorities\n\n");
        content.push_str("Focus on these areas in order of priority:\n\n");
        
        // Sort recommendations by priority
        let mut recommendations = result.recommendations.clone();
        recommendations.sort_by(|a, b| b.priority.cmp(&a.priority));
        
        for rec in recommendations.iter().take(5) {
            content.push_str(&format!("1. **{}**: {}\n", rec.title, rec.description));
        }
        
        // Code quality guidance
        content.push_str("\n## Code Quality Guidance\n\n");
        content.push_str("Maintain or improve these code quality metrics:\n\n");
        
        for (metric, value) in &result.code_quality.metrics {
            content.push_str(&format!("- **{}**: {:.1} or better\n", metric, value));
        }
        
        // Write to guidance file
        let guidance_file = docs_dir.join("ai_agent_guidance.md");
        fs::write(&guidance_file, content)
            .map_err(|e| format!("Failed to write guidance file: {}", e))?;
        
        println!("AI agent guidance generated at {:?}", guidance_file);
        
        Ok(guidance_file)
    }
}
