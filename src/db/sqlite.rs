use sqlx::{SqlitePool, sqlite::SqlitePoolOptions, Error as SqlxError};

pub async fn init_connection(path: &str) -> Result<SqlitePool, SqlxError> {
    SqlitePoolOptions::new()
        .max_connections(5)
        .connect(path)
        .await
}

// Example repository trait for Course
#[async_trait::async_trait]
pub trait CourseRepository {
    async fn create_course(&self, name: &str, code: &str, description: Option<&str>) -> Result<i64, SqlxError>;
    async fn get_course(&self, id: i64) -> Result<Option<CourseRow>, SqlxError>;
    async fn list_courses(&self) -> Result<Vec<CourseRow>, SqlxError>;
}

#[derive(Debug, sqlx::FromRow)]
pub struct CourseRow {
    pub id: i64,
    pub name: String,
    pub code: String,
    pub description: Option<String>,
}

// Example implementation (to be completed with real SQL)
pub struct SqliteCourseRepository<'a> {
    pub pool: &'a SqlitePool,
}

#[async_trait::async_trait]
impl<'a> CourseRepository for SqliteCourseRepository<'a> {
    async fn create_course(&self, name: &str, code: &str, description: Option<&str>) -> Result<i64, SqlxError> {
        let rec = sqlx::query!(
            "INSERT INTO courses (name, code, description) VALUES (?, ?, ?) RETURNING id",
            name, code, description
        )
        .fetch_one(self.pool)
        .await?;
        Ok(rec.id)
    }
    async fn get_course(&self, id: i64) -> Result<Option<CourseRow>, SqlxError> {
        let row = sqlx::query_as!(CourseRow, "SELECT id, name, code, description FROM courses WHERE id = ?", id)
            .fetch_optional(self.pool)
            .await?;
        Ok(row)
    }
    async fn list_courses(&self) -> Result<Vec<CourseRow>, SqlxError> {
        let rows = sqlx::query_as!(CourseRow, "SELECT id, name, code, description FROM courses")
            .fetch_all(self.pool)
            .await?;
        Ok(rows)
    }
}
