use sqlx::PgPool;
use uuid::Uuid;
use crate::models::post::Post;

pub struct PostRepository {
    pool: PgPool,
}

impl PostRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_post(&self, post: &Post) -> Result<Post, sqlx::Error> {
        let created_post = sqlx::query_as!(
            Post,
            r#"
            INSERT INTO posts (
                id, topic_id, author_id, content, is_first_post, 
                parent_id, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING *
            "#,
            post.id,
            post.topic_id,
            post.author_id,
            post.content,
            post.is_first_post,
            post.parent_id,
            post.created_at,
            post.updated_at
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(created_post)
    }

    pub async fn find_post_by_id(&self, id: &Uuid) -> Result<Option<Post>, sqlx::Error> {
        let post = sqlx::query_as!(
            Post,
            r#"
            SELECT * FROM posts
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(post)
    }

    pub async fn list_posts_by_topic(&self, topic_id: &Uuid) -> Result<Vec<Post>, sqlx::Error> {
        let posts = sqlx::query_as!(
            Post,
            r#"
            SELECT * FROM posts
            WHERE topic_id = $1
            ORDER BY 
                is_first_post DESC, 
                parent_id NULLS FIRST, 
                created_at ASC
            "#,
            topic_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(posts)
    }

    pub async fn update_post(&self, post: &Post) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE posts
            SET content = $1, updated_at = $2
            WHERE id = $3
            "#,
            post.content,
            post.updated_at,
            post.id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn delete_post(&self, post_id: &Uuid) -> Result<(), sqlx::Error> {
        // In a real application, you might want to soft delete posts
        // or prevent deletion of the first post in a topic
        
        sqlx::query!(
            r#"
            DELETE FROM posts
            WHERE id = $1 AND is_first_post = false
            "#,
            post_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}