use sqlx::{SqlitePool, Row, Error};
use std::collections::HashMap;

pub struct DbSchemaAnalyzer {
    pool: SqlitePool,
}

impl DbSchemaAnalyzer {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
    
    pub async fn get_tables(&self) -> Result<Vec<String>, Error> {
        // SQLite specific query to get all tables
        let rows = sqlx::query(
            "SELECT name FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%'"
        )
        .fetch_all(&self.pool)
        .await?;
        
        let tables = rows
            .iter()
            .map(|row| row.get::<String, _>("name"))
            .collect();
        
        Ok(tables)
    }
    
    pub async fn get_table_schema(&self, table_name: &str) -> Result<Vec<HashMap<String, String>>, Error> {
        // Get column information using PRAGMA
        let rows = sqlx::query(&format!("PRAGMA table_info({})", table_name))
            .fetch_all(&self.pool)
            .await?;
        
        let mut columns = Vec::new();
        
        for row in rows {
            let mut column = HashMap::new();
            column.insert("name".to_string(), row.get::<String, _>("name"));
            column.insert("type".to_string(), row.get::<String, _>("type"));
            column.insert("notnull".to_string(), row.get::<i64, _>("notnull").to_string());
            column.insert("pk".to_string(), row.get::<i64, _>("pk").to_string());
            
            columns.push(column);
        }
        
        Ok(columns)
    }
}