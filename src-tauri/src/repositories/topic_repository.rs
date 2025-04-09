use sqlx::{Sqlite, SqlitePool};
use uuid::Uuid;
use chrono::Utc;
use crate::error::Error;
use crate::models::forum::topic::{Topic, TopicRequest, TopicSummary};
use crate::models::forum::post::Post;
use crate::utils::slugify::slugify;

pub struct TopicRepository {
    pool: SqlitePool,
}

impl TopicRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
    
    pub async fn create(&self, user_id: &str, request: TopicRequest) -> Result<Topic, Error> {
        let topic_id = Uuid::new_v4().to_string();
        let now = Utc::now();
        let slug = slugify(&request.title);
        
        // Start a transaction
        let mut tx = self.pool.begin().await?;
        
        // Insert the topic
        let topic = sqlx::query_as!(
            Topic,
            r#"
            INSERT INTO forum_topics (
                id, title, slug, category_id, user_id,
                closed, pinned, pinned_globally, visible,
                views, posts_count, like_count, 
                created_at, updated_at, bumped_at, highest_post_number
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            RETURNING *
            "#,
            topic_id,
            request.title,
            slug,
            request.category_id,
            user_id,
            false, // closed
            false, // pinned
            false, // pinned_globally
            true,  // visible
            0,     // views
            1,     // posts_count (including first post)
            0,     // like_count
            now.to_rfc3339(),
            now.to_rfc3339(),
            now.to_rfc3339(),
            1      // highest_post_number
        )
        .fetch_one(&mut tx)
        .await?;
        
        // Create the first post
        let post_id = Uuid::new_v4().to_string();
        let post = sqlx::query_as!(
            Post,
            r#"
            INSERT INTO forum_posts (
                id, topic_id, user_id, post_number,
                raw, html, cooked,
                created_at, updated_at
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
            RETURNING *
            "#,
            post_id,
            topic_id,
            user_id,
            1, // First post
            request.raw,
            request.raw, // Simple conversion for now, in reality we'd convert markdown to HTML
            request.raw, // Cooked version (sanitized HTML)
            now.to_rfc3339(),
            now.to_rfc3339()
        )
        .fetch_one(&mut tx)
        .await?;
        
        // Add tags if provided
        if let Some(tags) = request.tags {
            for tag in tags {
                // Create tag if it doesn't exist
                let tag_id = sqlx::query_scalar!(
                    "SELECT id FROM forum_tags WHERE name = ?",
                    tag
                )
                .fetch_optional(&mut tx)
                .await?
                .unwrap_or_else(|| {
                    let tag_id = Uuid::new_v4().to_string();
                    let _ = sqlx::query!(
                        "INSERT INTO forum_tags (id, name, created_at) VALUES (?, ?, ?)",
                        tag_id,
                        tag,
                        now.to_rfc3339()
                    )
                    .execute(&mut tx);
                    tag_id
                });
                
                // Link tag to topic
                sqlx::query!(
                    "INSERT INTO forum_topic_tags (topic_id, tag_id) VALUES (?, ?)",
                    topic_id,
                    tag_id
                )
                .execute(&mut tx)
                .await?;
            }
        }
        
        // Update category post count
        sqlx::query!(
            r#"
            UPDATE forum_categories
            SET 
                topic_count = topic_count + 1,
                post_count = post_count + 1,
                updated_at = ?
            WHERE id = ?
            "#,
            now.to_rfc3339(),
            request.category_id
        )
        .execute(&mut tx)
        .await?;
        
        // Commit the transaction
        tx.commit().await?;
        
        // Return the topic with first post
        let mut topic = topic;
        topic.posts = Some(vec![post]);
        Ok(topic)
    }
    
    pub async fn get_by_id(&self, id: &str) -> Result<Topic, Error> {
        let topic = sqlx::query_as!(
            Topic,
            r#"
            SELECT * FROM forum_topics
            WHERE id = ? AND visible = true AND deleted_at IS NULL
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| Error::NotFound(format!("Topic not found: {}", id)))?;
        
        // Increment view count
        sqlx::query!(
            "UPDATE forum_topics SET views = views + 1 WHERE id = ?",
            id
        )
        .execute(&self.pool)
        .await?;
        
        // Load posts
        let posts = sqlx::query_as!(
            Post,
            r#"
            SELECT * FROM forum_posts
            WHERE topic_id = ? AND deleted_at IS NULL
            ORDER BY post_number
            "#,
            id
        )
        .fetch_all(&self.pool)
        .await?;
        
        // Load category details
        let category = sqlx::query_as!(
            crate::models::forum::category::Category,
            "SELECT * FROM forum_categories WHERE id = ?",
            topic.category_id
        )
        .fetch_one(&self.pool)
        .await?;
        
        // Load author details
        let user = sqlx::query_as!(
            crate::models::user::User,
            "SELECT * FROM users WHERE id = ?",
            topic.user_id
        )
        .fetch_one(&self.pool)
        .await?;
        
        // Combine everything
        let mut topic = topic;
        topic.posts = Some(posts);
        topic.category = Some(category);
        topic.user = Some(user);
        
        Ok(topic)
    }
    
    pub async fn list_by_category(&self, category_id: &str, page: i64, per_page: i64) -> Result<Vec<TopicSummary>, Error> {
        let offset = (page - 1) * per_page;
        
        let topics = sqlx::query!(
            r#"
            SELECT 
                t.id, t.title, t.slug, t.category_id, t.user_id,
                t.closed, t.pinned, t.visible, t.created_at, t.posts_count,
                t.views, t.last_posted_at, t.excerpt,
                u.display_name as user_display_name,
                c.name as category_name
            FROM forum_topics t
            JOIN users u ON t.user_id = u.id
            JOIN forum_categories c ON t.category_id = c.id
            WHERE t.category_id = ? AND t.visible = true AND t.deleted_at IS NULL
            ORDER BY t.pinned_globally DESC, t.pinned DESC, t.bumped_at DESC
            LIMIT ? OFFSET ?
            "#,
            category_id,
            per_page,
            offset
        )
        .fetch_all(&self.pool)
        .await?;
        
        let topic_summaries = topics.into_iter().map(|row| {
            TopicSummary {
                id: row.id,
                title: row.title,
                slug: row.slug,
                category_id: row.category_id,
                user_id: row.user_id,
                closed: row.closed,
                pinned: row.pinned,
                visible: row.visible,
                created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at)
                    .unwrap_or_else(|_| chrono::DateTime::parse_from_rfc3339("2023-01-01T00:00:00Z").unwrap())
                    .with_timezone(&chrono::Utc),
                posts_count: row.posts_count,
                views: row.views,
                last_posted_at: row.last_posted_at.map(|date| {
                    chrono::DateTime::parse_from_rfc3339(&date)
                        .unwrap_or_else(|_| chrono::DateTime::parse_from_rfc3339("2023-01-01T00:00:00Z").unwrap())
                        .with_timezone(&chrono::Utc)
                }),
                excerpt: row.excerpt,
                user_display_name: row.user_display_name,
                category_name: row.category_name,
            }
        }).collect();
        
        Ok(topic_summaries)
    }
    
    pub async fn list_latest(&self, page: i64, per_page: i64) -> Result<Vec<TopicSummary>, Error> {
        let offset = (page - 1) * per_page;
        
        let topics = sqlx::query!(
            r#"
            SELECT 
                t.id, t.title, t.slug, t.category_id, t.user_id,
                t.closed, t.pinned, t.visible, t.created_at, t.posts_count,
                t.views, t.last_posted_at, t.excerpt,
                u.display_name as user_display_name,
                c.name as category_name
            FROM forum_topics t
            JOIN users u ON t.user_id = u.id
            JOIN forum_categories c ON t.category_id = c.id
            WHERE t.visible = true AND t.deleted_at IS NULL
            ORDER BY t.pinned_globally DESC, t.bumped_at DESC
            LIMIT ? OFFSET ?
            "#,
            per_page,
            offset
        )
        .fetch_all(&self.pool)
        .await?;
        
        let topic_summaries = topics.into_iter().map(|row| {
            TopicSummary {
                id: row.id,
                title: row.title,
                slug: row.slug,
                category_id: row.category_id,
                user_id: row.user_id,
                closed: row.closed,
                pinned: row.pinned,
                visible: row.visible,
                created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at)
                    .unwrap_or_else(|_| chrono::DateTime::parse_from_rfc3339("2023-01-01T00:00:00Z").unwrap())
                    .with_timezone(&chrono::Utc),
                posts_count: row.posts_count,
                views: row.views,
                last_posted_at: row.last_posted_at.map(|date| {
                    chrono::DateTime::parse_from_rfc3339(&date)
                        .unwrap_or_else(|_| chrono::DateTime::parse_from_rfc3339("2023-01-01T00:00:00Z").unwrap())
                        .with_timezone(&chrono::Utc)
                }),
                excerpt: row.excerpt,
                user_display_name: row.user_display_name,
                category_name: row.category_name,
            }
        }).collect();
        
        Ok(topic_summaries)
    }
}