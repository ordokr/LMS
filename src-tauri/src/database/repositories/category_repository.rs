use crate::models::Category;
use chrono::{DateTime, Utc};
use rusqlite::{params, Connection, Result, Row};

pub struct CategoryRepository<'a> {
    conn: &'a Connection,
}

impl<'a> CategoryRepository<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }
    
    // Convert a database row to a Category model
    fn row_to_category(row: &Row) -> Result<Category> {
        Ok(Category {
            id: Some(row.get(0)?),
            name: row.get(1)?,
            slug: row.get(2)?,
            description: row.get(3)?,
            color: row.get(4)?,
            text_color: row.get(5)?,
            parent_id: row.get(6)?,
            position: row.get(7)?,
            created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(8)?)
                .map_err(|e| rusqlite::Error::FromSqlConversionFailure(8, rusqlite::types::Type::Text, Box::new(e)))?
                .with_timezone(&Utc),
            updated_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(9)?)
                .map_err(|e| rusqlite::Error::FromSqlConversionFailure(9, rusqlite::types::Type::Text, Box::new(e)))?
                .with_timezone(&Utc),
            is_deleted: row.get(10)?,
        })
    }
    
    // Create a new category
    pub fn create(&self, category: &Category) -> Result<i64> {
        self.conn.execute(
            "INSERT INTO categories 
            (name, slug, description, color, text_color, parent_id, position, created_at, updated_at, is_deleted) 
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            params![
                category.name, category.slug, category.description, category.color, 
                category.text_color, category.parent_id, category.position, 
                category.created_at.to_rfc3339(), category.updated_at.to_rfc3339(), 
                category.is_deleted
            ],
        )?;
        
        Ok(self.conn.last_insert_rowid())
    }
    
    // Get a category by ID
    pub fn find_by_id(&self, id: i64) -> Result<Option<Category>> {
        let mut stmt = self.conn.prepare(
            "SELECT * FROM categories WHERE id = ? AND is_deleted = 0"
        )?;
        
        let mut rows = stmt.query([id])?;
        
        if let Some(row) = rows.next()? {
            Ok(Some(Self::row_to_category(row)?))
        } else {
            Ok(None)
        }
    }
    
    // Get a category by slug
    pub fn find_by_slug(&self, slug: &str) -> Result<Option<Category>> {
        let mut stmt = self.conn.prepare(
            "SELECT * FROM categories WHERE slug = ? AND is_deleted = 0"
        )?;
        
        let mut rows = stmt.query([slug])?;
        
        if let Some(row) = rows.next()? {
            Ok(Some(Self::row_to_category(row)?))
        } else {
            Ok(None)
        }
    }
    
    // Update a category
    pub fn update(&self, category: &Category) -> Result<()> {
        if category.id.is_none() {
            return Err(rusqlite::Error::InvalidParameterName("Category ID is required for update".to_string()));
        }
        
        self.conn.execute(
            "UPDATE categories SET 
            name = ?1, slug = ?2, description = ?3, color = ?4, 
            text_color = ?5, parent_id = ?6, position = ?7, updated_at = ?8 
            WHERE id = ?9",
            params![
                category.name, category.slug, category.description, category.color, 
                category.text_color, category.parent_id, category.position, 
                Utc::now().to_rfc3339(), category.id
            ],
        )?;
        
        Ok(())
    }
    
    // Soft delete a category
    pub fn delete(&self, id: i64) -> Result<()> {
        self.conn.execute(
            "UPDATE categories SET is_deleted = 1, updated_at = ? WHERE id = ?",
            params![Utc::now().to_rfc3339(), id],
        )?;
        
        Ok(())
    }
    
    // List all active categories
    pub fn list_all(&self) -> Result<Vec<Category>> {
        let mut stmt = self.conn.prepare(
            "SELECT * FROM categories 
             WHERE is_deleted = 0 
             ORDER BY position ASC, name ASC"
        )?;
        
        let rows = stmt.query_map([], |row| {
            Self::row_to_category(row)
        })?;
        
        let mut categories = Vec::new();
        for category_result in rows {
            categories.push(category_result?);
        }
        
        Ok(categories)
    }
    
    // Get top-level categories (no parent)
    pub fn list_top_level(&self) -> Result<Vec<Category>> {
        let mut stmt = self.conn.prepare(
            "SELECT * FROM categories 
             WHERE parent_id IS NULL AND is_deleted = 0 
             ORDER BY position ASC, name ASC"
        )?;
        
        let rows = stmt.query_map([], |row| {
            Self::row_to_category(row)
        })?;
        
        let mut categories = Vec::new();
        for category_result in rows {
            categories.push(category_result?);
        }
        
        Ok(categories)
    }
    
    // Get subcategories for a parent category
    pub fn list_subcategories(&self, parent_id: i64) -> Result<Vec<Category>> {
        let mut stmt = self.conn.prepare(
            "SELECT * FROM categories 
             WHERE parent_id = ? AND is_deleted = 0 
             ORDER BY position ASC, name ASC"
        )?;
        
        let rows = stmt.query_map([parent_id], |row| {
            Self::row_to_category(row)
        })?;
        
        let mut categories = Vec::new();
        for category_result in rows {
            categories.push(category_result?);
        }
        
        Ok(categories)
    }
}