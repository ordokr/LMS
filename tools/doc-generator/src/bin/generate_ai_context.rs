use std::path::{Path, PathBuf};
use std::fs;
use anyhow::{Result, Context};
use chrono::Local;
use regex::Regex;

/// AI Context Generator for creating optimized GitHub Copilot guidance files
pub struct AIContextGenerator {
    base_dir: PathBuf,
    docs_dir: PathBuf,
    rag_dir: PathBuf,
}

/// Count information extracted from documentation
struct CountInfo {
    total: i32,
    completed: i32,
    percentage: i32,
}

impl AIContextGenerator {
    /// Create a new AI Context Generator
    pub fn new<P: AsRef<Path>>(base_dir: P) -> Self {
        let base_dir = base_dir.as_ref().to_path_buf();
        let docs_dir = base_dir.join("docs");
        let rag_dir = base_dir.join("rag_knowledge_base");
        
        Self {
            base_dir,
            docs_dir,
            rag_dir,
        }
    }
    
    /// Generate AI guidance documents
    pub fn generate_ai_guidance(&self) -> Result<bool> {
        println!("Generating AI guidance documents...");
        
        // Read source files
        let central_hub = self.read_file(&self.docs_dir.join("central_reference_hub.md"))?;
        let last_analysis = self.read_file(&self.base_dir.join("LAST_ANALYSIS_RESULTS.md"))
            .unwrap_or_default();
        
        // Create AI_GUIDANCE.md - optimized for GitHub Copilot
        let guidance = self.create_ai_guidance(&central_hub, &last_analysis)?;
        fs::write(self.base_dir.join("AI_GUIDANCE.md"), guidance)
            .context("Failed to write AI guidance file")?;
        println!("AI guidance file created");
        
        // Update central_reference_hub.md with AI metadata
        self.enhance_central_hub(&self.docs_dir.join("central_reference_hub.md"))?;
        println!("Central reference hub enhanced with AI metadata");
        
        // Create component index for component-specific references
        self.create_component_index(&central_hub)?;
        println!("Component index created");
        
        Ok(true)
    }
    
    /// Read a file and return its contents
    fn read_file(&self, file_path: &Path) -> Result<String> {
        match fs::read_to_string(file_path) {
            Ok(content) => Ok(content),
            Err(err) => {
                eprintln!("Could not read {}: {}", file_path.display(), err);
                Err(anyhow::anyhow!("Could not read {}: {}", file_path.display(), err))
            }
        }
    }
    
    /// Create AI guidance content
    fn create_ai_guidance(&self, central_hub: &str, last_analysis: &str) -> Result<String> {
        let today = Local::now().format("%Y-%m-%d").to_string();
        
        let mut content = String::from("# AI Guidance for LMS Integration Project\n\n");
        content.push_str(&format!("<!-- AI_METADATA\nversion: 1.0\npriority: highest\nupdated: {}\nrole: guidance\n-->\n\n", today));
        
        content.push_str("## Documentation Hierarchy\n\n");
        content.push_str("1. **Primary Source of Truth**: [central_reference_hub.md](docs/central_reference_hub.md)\n");
        content.push_str("2. **Current Project Status**: [LAST_ANALYSIS_RESULTS.md](LAST_ANALYSIS_RESULTS.md)\n");
        content.push_str("3. **Knowledge Base**: [rag_knowledge_base/](rag_knowledge_base/)\n\n");
        
        content.push_str("## Key Project Facts\n\n");
        
        // Extract key project facts from central hub
        let model_count = self.extract_count(central_hub, "Models", "Completion")?;
        let api_count = self.extract_count(central_hub, "API Endpoints", "Completion")?;
        let ui_count = self.extract_count(central_hub, "UI Components", "Completion")?;
        
        content.push_str("- Database: **SQLite with sqlx** (embedded file database)\n");
        content.push_str(&format!("- Models: {}/{} implemented ({}%)\n", 
            model_count.completed, model_count.total, model_count.percentage));
        content.push_str(&format!("- API Endpoints: {}/{} implemented ({}%)\n", 
            api_count.completed, api_count.total, api_count.percentage));
        content.push_str(&format!("- UI Components: {}/{} implemented ({}%)\n\n", 
            ui_count.completed, ui_count.total, ui_count.percentage));
        
        content.push_str("## Current Development Phase\n\n");
        let phase = self.extract_phase(central_hub)?;
        content.push_str(&format!("{}\n\n", phase));
        
        content.push_str("## Project Structure\n\n");
        content.push_str("- **Frontend**: Tauri with web frontend\n");
        content.push_str("- **Backend**: Rust with SQLite database\n");
        content.push_str("- **Documentation**: Markdown in docs/ and rag_knowledge_base/\n\n");
        
        content.push_str("## Component Implementation Status\n\n");
        content.push_str("See [Component Index](docs/ai/component_index.md) for details on component implementation status.\n\n");
        
        content.push_str("## API Guidelines\n\n");
        content.push_str("- All API endpoints should be defined in the central reference hub\n");
        content.push_str("- Follow RESTful patterns for API design\n");
        content.push_str("- All endpoints should include error handling\n\n");
        
        content.push_str("## Model Guidelines\n\n");
        content.push_str("- All models should match the specifications in central_reference_hub.md\n");
        content.push_str("- Use SQLite types compatible with sqlx\n\n");
        
        Ok(content)
    }
    
    /// Enhance central hub with AI metadata
    fn enhance_central_hub(&self, hub_path: &Path) -> Result<bool> {
        let content = self.read_file(hub_path)?;
        
        // Check if AI metadata already exists
        if content.contains("AI_METADATA") {
            return Ok(false);
        }
        
        // Add AI metadata to the top
        let today = Local::now().format("%Y-%m-%d").to_string();
        let enhanced_content = format!(
            "<!-- AI_METADATA\nversion: 1.0\npriority: highest\nupdated: {}\nrole: reference\nstatus: authoritative\n-->\n\n{}",
            today, content
        );
        
        fs::write(hub_path, enhanced_content)
            .context(format!("Failed to write enhanced content to {}", hub_path.display()))?;
        
        Ok(true)
    }
    
    /// Create component index
    fn create_component_index(&self, central_hub: &str) -> Result<bool> {
        let ai_docs_dir = self.docs_dir.join("ai");
        if !ai_docs_dir.exists() {
            fs::create_dir_all(&ai_docs_dir)
                .context(format!("Failed to create directory at {}", ai_docs_dir.display()))?;
        }
        
        let component_section = self.extract_section(central_hub, "UI Component Reference")?;
        
        let today = Local::now().format("%Y-%m-%d").to_string();
        let mut content = String::from("# Component Implementation Index\n\n");
        content.push_str(&format!("<!-- AI_METADATA\nversion: 1.0\npriority: medium\nupdated: {}\nrole: component_reference\n-->\n\n", today));
        
        content.push_str("This document provides detailed information about component implementation status.\n\n");
        content.push_str("## Implementation Status\n\n");
        content.push_str(&component_section);
        
        fs::write(ai_docs_dir.join("component_index.md"), content)
            .context("Failed to write component index")?;
        
        Ok(true)
    }
    
    /// Extract counts from content
    fn extract_count(&self, content: &str, label: &str, column: &str) -> Result<CountInfo> {
        // Extract counts from table row
        let regex_pattern = format!(r"\| {} \| ([0-9]+)% \|", regex::escape(label));
        let regex = Regex::new(&regex_pattern)
            .context(format!("Failed to create regex pattern for {}", label))?;
        
        let captures = regex.captures(content);
        let percentage = captures
            .and_then(|cap| cap.get(1))
            .and_then(|m| m.as_str().parse::<i32>().ok())
            .unwrap_or(0);
        
        // Calculate approximate total/completed based on percentage
        let total = 100;
        let completed = percentage;
        
        Ok(CountInfo {
            total,
            completed,
            percentage,
        })
    }
    
    /// Extract phase information
    fn extract_phase(&self, content: &str) -> Result<String> {
        // Extract overall phase
        let regex = Regex::new(r"\*\*Overall Phase:\*\* ([^\n]+)")
            .context("Failed to create regex pattern for phase extraction")?;
        
        let captures = regex.captures(content);
        let phase = captures
            .and_then(|cap| cap.get(1))
            .map(|m| m.as_str().to_string())
            .unwrap_or_else(|| "Unknown".to_string());
        
        Ok(phase)
    }
    
    /// Extract a section from content
    fn extract_section(&self, content: &str, section_title: &str) -> Result<String> {
        // Extract a section from the content
        let regex_pattern = format!(r"## {}\n\n([\s\S]*?)(?=\n## |$)", regex::escape(section_title));
        let regex = Regex::new(&regex_pattern)
            .context(format!("Failed to create regex pattern for section {}", section_title))?;
        
        let captures = regex.captures(content);
        let section = captures
            .and_then(|cap| cap.get(1))
            .map(|m| m.as_str().trim().to_string())
            .unwrap_or_default();
        
        Ok(section)
    }
}

fn main() -> Result<()> {
    let current_dir = std::env::current_dir()?;
    let generator = AIContextGenerator::new(current_dir);
    
    generator.generate_ai_guidance()?;
    println!("AI context generation complete");
    
    Ok(())
}
