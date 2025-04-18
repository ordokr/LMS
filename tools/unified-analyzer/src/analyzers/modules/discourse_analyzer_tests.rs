rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
    use std::path::PathBuf;

    fn setup() -> PathBuf {
        let temp_dir = PathBuf::from("discourse_test_files");
        fs::create_dir_all(&temp_dir).unwrap();

        // User file
        let user_content = r#"
            <div>@testuser</div>
            <div>@anotheruser</div>
        "#;
        let mut file = File::create(temp_dir.join("user.html")).unwrap();
        file.write_all(user_content.as_bytes()).unwrap();

        // Topic file
        let topic_content = r#"
            <title>Test Topic</title>
            <span itemprop="author" itemscope itemtype="http://schema.org/Person">
                <a itemprop="url" href="/u/testauthor"></a>
            </span>
        "#;
        let mut file = File::create(temp_dir.join("topic.html")).unwrap();
        file.write_all(topic_content.as_bytes()).unwrap();

        // Post file
        let post_content = r#"
            <div class="topic-body clearfix" itemprop="text">
                Test Post Content
            </div>
            <a itemprop="url" href="/u/postauthor"></a>
        "#;
        let mut file = File::create(temp_dir.join("post.html")).unwrap();
        file.write_all(post_content.as_bytes()).unwrap();

        temp_dir
    }

    fn teardown(temp_dir: PathBuf) {
        fs::remove_dir_all(temp_dir).unwrap();
    }

    #[test]
    fn test_user_identification() {
        let temp_dir = setup();
        let analyzer = DiscourseAnalyzer::default();
        let result = analyzer.analyze(temp_dir.to_str().unwrap()).unwrap();
        let analysis: DiscourseAnalysis = serde_json::from_str(&result).unwrap();
        assert_eq!(analysis.users.len(), 2);
        assert_eq!(analysis.users[0].username, "testuser");
        assert_eq!(analysis.users[1].username, "anotheruser");
        teardown(temp_dir);
    }

    #[test]
    fn test_topic_identification() {
        let temp_dir = setup();
        let analyzer = DiscourseAnalyzer::default();
        let result = analyzer.analyze(temp_dir.to_str().unwrap()).unwrap();
        let analysis: DiscourseAnalysis = serde_json::from_str(&result).unwrap();
        assert_eq!(analysis.topics.len(), 1);
        assert_eq!(analysis.topics[0].title, "Test Topic");
        assert_eq!(analysis.topics[0].author, "testauthor");
        teardown(temp_dir);
    }

    #[test]
    fn test_post_identification() {
        let temp_dir = setup();
        let analyzer = DiscourseAnalyzer::default();
        let result = analyzer.analyze(temp_dir.to_str().unwrap()).unwrap();
        let analysis: DiscourseAnalysis = serde_json::from_str(&result).unwrap();
        assert_eq!(analysis.posts.len(), 1);
        assert_eq!(analysis.posts[0].content, "Test Post Content");
        assert_eq!(analysis.posts[0].author, "postauthor");
        assert_eq!(analysis.posts[0].topic, "Unknown Topic");
        teardown(temp_dir);
    }

    use regex::Regex;
    use serde::{Deserialize, Serialize};
    use std::{
        collections::HashMap,
        fs,
        io::{self, Error},
        path::{Path, PathBuf},
    };
    use lazy_static::lazy_static;
    use walkdir::WalkDir;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct DiscourseUser {
        pub username: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct DiscourseTopic {
        pub title: String,
        pub author: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct DiscoursePost {
        pub content: String,
        pub topic: String,
        pub author: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct DiscourseAnalysis {
        pub users: Vec<DiscourseUser>,
        pub topics: Vec<DiscourseTopic>,
        pub posts: Vec<DiscoursePost>,
    }

    #[derive(Debug, Default)]
    pub struct DiscourseAnalyzer {}

    impl DiscourseAnalyzer {
        pub fn analyze(&self, project_path: &str) -> Result<String, DiscourseError> {
            let mut users: Vec<DiscourseUser> = Vec::new();
            let mut topics: Vec<DiscourseTopic> = Vec::new();
            let mut posts: Vec<DiscoursePost> = Vec::new();

            for entry in WalkDir::new(project_path)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                let path = entry.path();
                if path.is_file() {
                    if let Some(file_name) = path.file_name() {
                        if let Some(file_name_str) = file_name.to_str() {
                            if file_name_str.ends_with(".html") {
                                let content = fs::read_to_string(path)?;

                                // Extract users
                                users.extend(self.extract_users(&content));

                                // Extract topics
                                topics.extend(self.extract_topics(&content));

                                // Extract posts
                                posts.extend(self.extract_posts(&content));
                            }
                        }
                    }
                }
            }

            let result = serde_json::json!({
                "users": users,
                "topics": topics,
                "posts": posts,
            });

            Ok(serde_json::to_string_pretty(&result)?)
        }

        // Helper functions for extracting information using regex

        // Extracts users from the content
        fn extract_users(&self, content: &str) -> Vec<DiscourseUser> {
            lazy_static! {
                // Regex to find usernames in the format "@username"
                static ref USER_REGEX: Regex = Regex::new(r"@([a-zA-Z0-9_]+)").unwrap();
            }
            USER_REGEX.captures_iter(content)
                .filter_map(|caps| caps.get(1))
                .map(|m| DiscourseUser { username: m.as_str().to_string() })
                .collect()
        }

        // Extracts topics from the content
        fn extract_topics(&self, content: &str) -> Vec<DiscourseTopic> {
            lazy_static! {
                // Regex to find topics with titles and authors
                static ref TOPIC_REGEX: Regex = Regex::new(r"<title>(.*?)</title>.*?<span itemprop=\"author\" itemscope itemtype=\"http://schema.org/Person\"><a itemprop=\"url\" href=\"/u/(.*?)\">").unwrap();
            }
            TOPIC_REGEX.captures_iter(content)
                .filter_map(|caps| {
                    if let (Some(title), Some(author)) = (caps.get(1), caps.get(2)) {
                        Some(DiscourseTopic {
                            title: title.as_str().to_string(),
                            author: author.as_str().to_string(),
                        })
                    } else {
                        None
                    }
                })
                .collect()
        }

        // Extracts posts from the content
        fn extract_posts(&self, content: &str) -> Vec<DiscoursePost> {
            lazy_static! {
                // Regex to find posts with content, topic, and author
                static ref POST_REGEX: Regex = Regex::new(r"<div class=\"topic-body clearfix\" itemprop=\"text\">([\s\S]*?)</div>.*?<a itemprop=\"url\" href=\"/u/(.*?)\">").unwrap();
            }
            POST_REGEX.captures_iter(content)
                .filter_map(|caps| {
                    if let (Some(content), Some(author)) = (caps.get(1), caps.get(2)) {
                        // Extract topic from the URL or other means if available
                        let topic = "Unknown Topic".to_string(); // Placeholder
                        Some(DiscoursePost {
                            content: content.as_str().to_string(),
                            topic,
                            author: author.as_str().to_string(),
                        })
                    } else {
                        None
                    }
                })
                .collect()
        }
    }

    #[derive(Debug)]
    pub enum DiscourseError {
        IoError(io::Error),
        RegexError(String),
        JsonError(serde_json::Error),
        WalkDirError(walkdir::Error),
    }

    impl From<io::Error> for DiscourseError {
        fn from(error: io::Error) -> Self {
            DiscourseError::IoError(error)
        }
    }

    impl From<regex::Error> for DiscourseError {
        fn from(error: regex::Error) -> Self {
            DiscourseError::RegexError(error.to_string())
        }
    }
}