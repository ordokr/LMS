use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs,
    io::{self},
    path::PathBuf,
};
use walkdir::WalkDir;
use regex::Regex;
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct Topic {
    pub id: String,
    pub title: String,
    pub author: String,
    pub content: String,
    pub tags: Vec<String>,
    pub category: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    pub id: String,
    pub topic_id: String,
    pub author: String,
    pub content: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Category {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub parent_category_id: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DiscourseAnalyzer {
    pub topics: HashMap<String, Topic>,
    pub posts: HashMap<String, Post>,
    pub categories: HashMap<String, Category>,
}

impl DiscourseAnalyzer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn analyze(&self, project_path: &str) -> Result<String, DiscourseError> {
        let mut result = DiscourseAnalyzer::default();
        let discourse_dir = PathBuf::from(project_path);

        println!("Looking for Discourse directory at: {}", discourse_dir.display());

        if !discourse_dir.exists() {
            println!("Discourse directory not found at: {}", discourse_dir.display());
            return Ok("Discourse directory not found".to_string());
        }

        println!("Found Discourse directory at: {}", discourse_dir.display());

        // Analyze topics
        result.analyze_topics(&discourse_dir)?;

        // Analyze posts
        result.analyze_posts(&discourse_dir)?;

        // Analyze categories
        result.analyze_categories(&discourse_dir)?;

        Ok(serde_json::to_string_pretty(&result)?)
    }

    fn analyze_topics(&mut self, discourse_dir: &PathBuf) -> Result<(), DiscourseError> {
        let topics_dir = discourse_dir.join("app").join("views").join("topics");
        if !topics_dir.exists() {
            return Ok(());
        }

        println!("Analyzing topics directory: {}", topics_dir.display());

        for entry in WalkDir::new(&topics_dir)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |ext| ext == "erb" || ext == "html") {
                println!("Found topic file: {}", path.display());
                if let Ok(content) = fs::read_to_string(path) {
                    println!("File content length: {} bytes", content.len());

                    // Extract title - more flexible approach
                    let title_regex = Regex::new(r#"<title[^>]*>(.*?)</title>"#).unwrap();
                    let title = title_regex
                        .captures(&content)
                        .and_then(|cap| cap.get(1).map(|m| m.as_str().to_string()))
                        .unwrap_or_else(|| "Untitled Topic".to_string());

                    println!("Found topic title: {}", title);

                    // Extract author - more flexible approach
                    let author_regex = Regex::new(r#"<span[^>]*itemprop="author"[^>]*href="/u/([^"]+)"[^>]*>([^<]*)</span>"#).unwrap();
                    let author = if let Some(author_captures) = author_regex.captures(&content) {
                        author_captures.get(1).map_or("", |m| m.as_str()).to_string()
                    } else {
                        // Try alternative pattern
                        let alt_author_regex = Regex::new(r#"<span[^>]*itemprop="author"[^>]*>([^<]*)</span>"#).unwrap();
                        alt_author_regex
                            .captures(&content)
                            .and_then(|cap| cap.get(1).map(|m| m.as_str().to_string()))
                            .unwrap_or_else(|| {
                                // Try another alternative
                                let href_author_regex = Regex::new(r#"href="/u/([^"]+)""#).unwrap();
                                href_author_regex
                                    .captures(&content)
                                    .and_then(|cap| cap.get(1).map(|m| m.as_str().to_string()))
                                    .unwrap_or_else(|| "Unknown".to_string())
                            })
                    };

                    println!("Found topic author: {}", author);

                    // Generate a simple ID based on the file name
                    let id = path.file_stem().unwrap_or_default().to_string_lossy().to_string();
                    println!("Topic ID: {}", id);

                    // Extract tags if available
                    let tags_regex = Regex::new(r#"<span class="tag"[^>]*>(.*?)</span>"#).unwrap();
                    let tags: Vec<String> = tags_regex
                        .captures_iter(&content)
                        .filter_map(|cap| cap.get(1).map(|m| m.as_str().to_string()))
                        .collect();

                    println!("Found {} tags", tags.len());
                    for tag in &tags {
                        println!("  Tag: {}", tag);
                    }

                    // Extract category if available
                    let category_regex = Regex::new(r#"<a class="category"[^>]*>(.*?)</a>"#).unwrap();
                    let category = category_regex
                        .captures(&content)
                        .and_then(|cap| cap.get(1).map(|m| m.as_str().to_string()));

                    if let Some(cat) = &category {
                        println!("Found category: {}", cat);
                    }

                    // Extract the main content
                    let content_regex = Regex::new(r#"<div class="topic-body clearfix"[^>]*>(.*?)</div>"#).unwrap();
                    let topic_content = content_regex
                        .captures(&content)
                        .and_then(|cap| cap.get(1).map(|m| m.as_str().to_string()))
                        .unwrap_or_else(|| "No content".to_string());

                    println!("Content length: {} characters", topic_content.len());

                    self.topics.insert(
                        id.clone(),
                        Topic {
                            id,
                            title,
                            author,
                            content: topic_content,
                            tags,
                            category,
                        },
                    );
                }
            }
        }

        Ok(())
    }

    fn analyze_posts(&mut self, discourse_dir: &PathBuf) -> Result<(), DiscourseError> {
        let posts_dir = discourse_dir.join("app").join("views").join("posts");
        if !posts_dir.exists() {
            return Ok(());
        }

        println!("Analyzing posts directory: {}", posts_dir.display());

        for entry in WalkDir::new(&posts_dir)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |ext| ext == "erb" || ext == "html") {
                println!("Found post file: {}", path.display());
                if let Ok(content) = fs::read_to_string(path) {
                    // Extract post information - more flexible regex
                    let post_regex = Regex::new(r#"<div class="topic-body clearfix"[^>]*>([\s\S]*?)</div>"#).unwrap();

                    if let Some(captures) = post_regex.captures(&content) {
                        let post_content = captures.get(1).map_or("", |m| m.as_str()).to_string();
                        println!("Found post content: {} characters", post_content.len());

                        // Extract author - more flexible approach
                        let author_regex = Regex::new(r#"<a[^>]*href="/u/([^"]+)"[^>]*>"#).unwrap();
                        let author = author_regex
                            .captures(&content)
                            .and_then(|cap| cap.get(1).map(|m| m.as_str().to_string()))
                            .unwrap_or_else(|| "Unknown".to_string());

                        println!("Found post author: {}", author);

                        // Generate a simple ID based on the file name
                        let id = path.file_stem().unwrap_or_default().to_string_lossy().to_string();
                        println!("Post ID: {}", id);

                        // Try to extract topic ID from the file name or path
                        let topic_id = path
                            .parent()
                            .and_then(|p| p.file_name())
                            .unwrap_or_default()
                            .to_string_lossy()
                            .to_string();

                        println!("Topic ID: {}", topic_id);

                        // Extract created_at if available
                        let date_regex = Regex::new(r#"<time[^>]*>(.*?)</time>"#).unwrap();
                        let created_at = date_regex
                            .captures(&content)
                            .and_then(|cap| cap.get(1).map(|m| m.as_str().to_string()))
                            .unwrap_or_else(|| "Unknown".to_string());

                        println!("Created at: {}", created_at);

                        self.posts.insert(
                            id.clone(),
                            Post {
                                id,
                                topic_id,
                                author,
                                content: post_content,
                                created_at,
                            },
                        );
                    } else {
                        println!("No post content found in the file");
                    }
                }
            }
        }

        Ok(())
    }

    fn analyze_categories(&mut self, discourse_dir: &PathBuf) -> Result<(), DiscourseError> {
        let categories_dir = discourse_dir.join("app").join("views").join("categories");
        if !categories_dir.exists() {
            return Ok(());
        }

        println!("Analyzing categories directory: {}", categories_dir.display());

        for entry in WalkDir::new(&categories_dir)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |ext| ext == "erb" || ext == "html") {
                println!("Found category file: {}", path.display());
                if let Ok(content) = fs::read_to_string(path) {
                    // Extract category information - more flexible regex
                    let category_regex = Regex::new(r#"<h3[^>]*>(.*?)</h3>.*?<div class="category-description"[^>]*>(.*?)</div>"#).unwrap();

                    if let Some(captures) = category_regex.captures(&content) {
                        let name = captures.get(1).map_or("", |m| m.as_str()).to_string();
                        let description = captures.get(2).map(|m| m.as_str().to_string());

                        println!("Found category: {}", name);
                        if let Some(desc) = &description {
                            println!("Description: {}", desc);
                        }

                        // Generate a simple ID based on the file name
                        let id = path.file_stem().unwrap_or_default().to_string_lossy().to_string();
                        println!("Category ID: {}", id);

                        // Try to extract parent category ID if available
                        let parent_regex = Regex::new(r#"<a[^>]*parent-category[^>]*>(.*?)</a>"#).unwrap();
                        let parent_category_id = parent_regex
                            .captures(&content)
                            .and_then(|cap| cap.get(1).map(|m| m.as_str().to_string()));

                        if let Some(parent) = &parent_category_id {
                            println!("Parent category: {}", parent);
                        }

                        self.categories.insert(
                            id.clone(),
                            Category {
                                id,
                                name,
                                description,
                                parent_category_id,
                            },
                        );
                    } else {
                        // Try alternative pattern for categories
                        let alt_category_regex = Regex::new(r#"<a class="category"[^>]*>(.*?)</a>"#).unwrap();
                        if let Some(captures) = alt_category_regex.captures(&content) {
                            let name = captures.get(1).map_or("", |m| m.as_str()).to_string();
                            println!("Found category (alt): {}", name);

                            // Generate a simple ID based on the file name
                            let id = path.file_stem().unwrap_or_default().to_string_lossy().to_string();

                            self.categories.insert(
                                id.clone(),
                                Category {
                                    id,
                                    name,
                                    description: None,
                                    parent_category_id: None,
                                },
                            );
                        } else {
                            println!("No category information found in the file");
                        }
                    }
                }
            }
        }

        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum DiscourseError {
    #[error("IO error: {0}")]
    IoError(#[from] io::Error),
    #[error("Regex error: {0}")]
    RegexError(#[from] regex::Error),
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
}
