use sqlx::PgPool;
use uuid::Uuid;
use crate::models::forum::topic::Topic;

pub struct TopicRepository {
    pool: PgPool,
}

impl TopicRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_topic(&self, topic: &Topic) -> Result<Topic, sqlx::Error> {
        let created_topic = sqlx::query_as!(
            Topic,
            r#"
            INSERT INTO topics (
                id, title, slug, category_id, author_id, pinned, closed,
                assignment_id, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            RETURNING *
            "#,
            topic.id,
            topic.title,
            topic.slug,
            topic.category_id,
            topic.author_id,
            topic.pinned,
            topic.closed,
            topic.assignment_id,
            topic.created_at,
            topic.updated_at
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(created_topic)
    }

    pub async fn find_topic_by_id(&self, id: &Uuid) -> Result<Option<Topic>, sqlx::Error> {
        let topic = sqlx::query_as!(
            Topic,
            r#"
            SELECT 
                t.*,
                COUNT(DISTINCT p.id)::int AS "post_count!",
                COUNT(DISTINCT v.id)::int AS "view_count!"
            FROM topics t
            LEFT JOIN posts p ON p.topic_id = t.id
            LEFT JOIN topic_views v ON v.topic_id = t.id
            WHERE t.id = $1
            GROUP BY t.id
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(topic)
    }

    pub async fn find_topic_by_slug(&self, slug: &str) -> Result<Option<Topic>, sqlx::Error> {
        let topic = sqlx::query_as!(
            Topic,
            r#"
            SELECT 
                t.*,
                COUNT(DISTINCT p.id)::int AS "post_count!",
                COUNT(DISTINCT v.id)::int AS "view_count!"
            FROM topics t
            LEFT JOIN posts p ON p.topic_id = t.id
            LEFT JOIN topic_views v ON v.topic_id = t.id
            WHERE t.slug = $1
            GROUP BY t.id
            "#,
            slug
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(topic)
    }

    pub async fn list_topics_by_category(&self, category_id: &Uuid) -> Result<Vec<Topic>, sqlx::Error> {
        let topics = sqlx::query_as!(
            Topic,
            r#"
            SELECT 
                t.*,
                COUNT(DISTINCT p.id)::int AS "post_count!",
                COUNT(DISTINCT v.id)::int AS "view_count!"
            FROM topics t
            LEFT JOIN posts p ON p.topic_id = t.id
            LEFT JOIN topic_views v ON v.topic_id = t.id
            WHERE t.category_id = $1
            GROUP BY t.id
            ORDER BY t.pinned DESC, t.updated_at DESC
            "#,
            category_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(topics)
    }

    pub async fn find_topic_by_assignment(&self, assignment_id: &Uuid) -> Result<Option<Topic>, sqlx::Error> {
        let topic = sqlx::query_as!(
            Topic,
            r#"
            SELECT 
                t.*,
                COUNT(DISTINCT p.id)::int AS "post_count!",
                COUNT(DISTINCT v.id)::int AS "view_count!"
            FROM topics t
            LEFT JOIN posts p ON p.topic_id = t.id
            LEFT JOIN topic_views v ON v.topic_id = t.id
            WHERE t.assignment_id = $1
            GROUP BY t.id
            "#,
            assignment_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(topic)
    }

    pub async fn update_topic(&self, topic: &Topic) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE topics
            SET title = $1, slug = $2, category_id = $3, 
                pinned = $4, closed = $5, updated_at = $6
            WHERE id = $7
            "#,
            topic.title,
            topic.slug,
            topic.category_id,
            topic.pinned,
            topic.closed,
            topic.updated_at,
            topic.id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn record_topic_view(&self, topic_id: &Uuid, user_id: &Uuid) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO topic_views (topic_id, user_id, viewed_at)
            VALUES ($1, $2, NOW())
            ON CONFLICT (topic_id, user_id) DO UPDATE
            SET viewed_at = NOW()
            "#,
            topic_id,
            user_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn delete_topic(&self, id: &Uuid) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            DELETE FROM topics
            WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}