use tauri::{command, State};
use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::Utc;
use crate::models::forum::category::{Category, CategoryRequest};
use crate::error::Error;
use crate::services::slugify::slugify;

/// Get all categories with optional parent filtering
#[command]
pub async fn list_categories(
    parent_id: Option<String>,
    db: State<'_, SqlitePool>
) -> Result<Vec<Category>, Error> {
    let mut query = "
        SELECT 
            id, name, slug, description, parent_id, created_at
        FROM categories
        WHERE 1=1
    ".to_string();
    
    let mut args = Vec::<String>::new();
    
    // Add parent filter if provided
    if let Some(pid) = &parent_id {
        if pid.is_empty() {
            // Get root categories (no parent)
            query.push_str(" AND parent_id IS NULL");
        } else {
            // Get child categories of specific parent
            query.push_str(" AND parent_id = ?");
            args.push(pid.clone());
        }
    }
    
    // Add order by
    query.push_str(" ORDER BY name ASC");
    
    // Build the query with dynamic arguments
    let mut sql_query = sqlx::query_as::<_, Category>(&query);
    
    for arg in args {
        sql_query = sql_query.bind(arg);
    }
    
    let categories = sql_query.fetch_all(&**db).await?;
    
    Ok(categories)
}

/// Get a single category by id or slug
#[command]
pub async fn get_category(
    id_or_slug: String,
    db: State<'_, SqlitePool>
) -> Result<Category, Error> {
    let category = sqlx::query_as!(
        Category,
        r#"
        SELECT 
            id, name, slug, description, parent_id, created_at
        FROM categories
        WHERE id = ? OR slug = ?
        "#,
        id_or_slug, id_or_slug
    )
    .fetch_optional(&**db)
    .await?
    .ok_or_else(|| Error::NotFound("Category not found".into()))?;
    
    Ok(category)
}

/// Create a new category
#[command]
pub async fn create_category(
    user_id: String, // For permission checking
    category_request: CategoryRequest,
    db: State<'_, SqlitePool>
) -> Result<Category, Error> {
    // In a real app, check if user has permission to create categories
    
    // Generate a new UUID for the category
    let category_id = Uuid::new_v4().to_string();
    
    // Generate a slug from the name
    let slug = slugify(&category_request.name);
    
    // Get current timestamp
    let now = Utc::now();
    let now_str = now.to_rfc3339();
    
    // Check if parent exists (if provided)
    if let Some(parent_id) = &category_request.parent_id {
        let parent_exists = sqlx::query!(
            "SELECT id FROM categories WHERE id = ?",
            parent_id
        )
        .fetch_optional(&**db)
        .await?
        .is_some();
        
        if !parent_exists {
            return Err(Error::NotFound("Parent category not found".into()));
        }
    }
    
    // Insert the category
    sqlx::query!(
        r#"
        INSERT INTO categories (
            id, name, slug, description, parent_id, created_at
        ) VALUES (?, ?, ?, ?, ?, ?)
        "#,
        category_id,
        category_request.name,
        slug,
        category_request.description,
        category_request.parent_id,
        now_str
    )
    .execute(&**db)
    .await?;
    
    // Return the newly created category
    get_category(category_id, db).await
}

/// Update an existing category
#[command]
pub async fn update_category(
    id: String,
    user_id: String, // For permission checking
    category_request: CategoryRequest,
    db: State<'_, SqlitePool>
) -> Result<Category, Error> {
    // In a real app, check if user has permission to update categories
    
    // Check if category exists
    let category_exists = sqlx::query!(
        "SELECT id FROM categories WHERE id = ?",
        id
    )
    .fetch_optional(&**db)
    .await?
    .is_some();
    
    if !category_exists {
        return Err(Error::NotFound("Category not found".into()));
    }
    
    // Check if parent exists (if provided)
    if let Some(parent_id) = &category_request.parent_id {
        // Prevent circular references - category can't be its own parent
        if parent_id == &id {
            return Err(Error::BadRequest("Category cannot be its own parent".into()));
        }
        
        let parent_exists = sqlx::query!(
            "SELECT id FROM categories WHERE id = ?",
            parent_id
        )
        .fetch_optional(&**db)
        .await?
        .is_some();
        
        if !parent_exists {
            return Err(Error::NotFound("Parent category not found".into()));
        }
        
        // Also check for circular references deeper in the hierarchy
        // This is a simplified check - a real implementation would be more thorough
        let mut current_id = parent_id.clone();
        while !current_id.is_empty() {
            let parent = sqlx::query!(
                "SELECT parent_id FROM categories WHERE id = ?",
                current_id
            )
            .fetch_optional(&**db)
            .await?;
            
            if let Some(p) = parent {
                if let Some(pid) = p.parent_id {
                    if pid == id {
                        return Err(Error::BadRequest("Circular category reference detected".into()));
                    }
                    current_id = pid;
                } else {
                    break;
                }
            } else {
                break;
            }
        }
    }
    
    // Generate a slug from the name
    let slug = slugify(&category_request.name);
    
    // Update the category
    sqlx::query!(
        r#"
        UPDATE categories SET
            name = ?,
            slug = ?,
            description = ?,
            parent_id = ?
        WHERE id = ?
        "#,
        category_request.name,
        slug,
        category_request.description,
        category_request.parent_id,
        id
    )
    .execute(&**db)
    .await?;
    
    // Return the updated category
    get_category(id, db).await
}

/// Delete a category
#[command]
pub async fn delete_category(
    id: String,
    user_id: String, // For permission checking
    db: State<'_, SqlitePool>
) -> Result<(), Error> {
    // In a real app, check if user has permission to delete categories
    
    // Start a transaction
    let mut tx = db.begin().await?;
    
    // Check if category exists
    let category_exists = sqlx::query!(
        "SELECT id FROM categories WHERE id = ?",
        id
    )
    .fetch_optional(&mut *tx)
    .await?
    .is_some();
    
    if !category_exists {
        return Err(Error::NotFound("Category not found".into()));
    }
    
    // Check if category has topics
    let has_topics = sqlx::query!(
        "SELECT COUNT(*) as count FROM topics WHERE category_id = ? AND deleted_at IS NULL",
        id
    )
    .fetch_one(&mut *tx)
    .await?
    .count > 0;
    
    if has_topics {
        return Err(Error::BadRequest("Cannot delete category with topics".into()));
    }
    
    // Check if category has children
    let has_children = sqlx::query!(
        "SELECT COUNT(*) as count FROM categories WHERE parent_id = ?",
        id
    )
    .fetch_one(&mut *tx)
    .await?
    .count > 0;
    
    if has_children {
        return Err(Error::BadRequest("Cannot delete category with child categories".into()));
    }
    
    // Delete the category
    sqlx::query!(
        "DELETE FROM categories WHERE id = ?",
        id
    )
    .execute(&mut *tx)
    .await?;
    
    // Commit the transaction
    tx.commit().await?;
    
    Ok(())
}

/// Get topics for a category
#[command]
pub async fn get_category_topics(
    category_id: String,
    page: Option<u32>,
    per_page: Option<u32>,
    db: State<'_, SqlitePool>
) -> Result<Vec<crate::models::forum::topic::TopicSummary>, Error> {
    let page = page.unwrap_or(1);
    let per_page = per_page.unwrap_or(20);
    let offset = (page - 1) * per_page;
    
    let topics = sqlx::query_as!(
        crate::models::forum::topic::TopicSummary,
        r#"
        SELECT 
            t.id, t.title, t.slug, t.category_id, t.user_id, 
            t.closed, t.pinned, t.visible, t.created_at, 
            t.posts_count, t.views, t.last_posted_at, t.excerpt,
            u.display_name as user_display_name,
            c.name as category_name
        FROM topics t
        JOIN users u ON t.user_id = u.id
        JOIN categories c ON t.category_id = c.id
        WHERE t.category_id = ? AND t.deleted_at IS NULL
        ORDER BY t.pinned_globally DESC, t.pinned DESC, t.bumped_at DESC
        LIMIT ? OFFSET ?
        "#,
        category_id,
        per_page,
        offset
    )
    .fetch_all(&**db)
    .await?;
    
    Ok(topics)
}