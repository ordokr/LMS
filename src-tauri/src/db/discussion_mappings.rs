use diesel::prelude::*;
use crate::db::{DbPool, schema::discussion_mappings};
use crate::models::discussion_mapping::DiscussionMapping;
use chrono::Utc;
use crate::error::Error;

pub async fn create_discussion_mapping(
    pool: &DbPool,
    mapping: &DiscussionMapping,
) -> Result<DiscussionMapping, Error> {
    let conn = pool.get()?;
    
    diesel::insert_into(discussion_mappings::table)
        .values(mapping)
        .execute(&conn)?;
    
    Ok(mapping.clone())
}

pub async fn get_discussion_mapping(
    pool: &DbPool,
    id: &str,
) -> Result<DiscussionMapping, Error> {
    let conn = pool.get()?;
    
    let mapping = discussion_mappings::table
        .filter(discussion_mappings::id.eq(id))
        .first(&conn)?;
    
    Ok(mapping)
}

pub async fn update_discussion_mapping(
    pool: &DbPool,
    mapping: &DiscussionMapping,
) -> Result<DiscussionMapping, Error> {
    let conn = pool.get()?;
    
    diesel::update(discussion_mappings::table)
        .filter(discussion_mappings::id.eq(&mapping.id))
        .set(mapping)
        .execute(&conn)?;
    
    Ok(mapping.clone())
}

pub async fn delete_discussion_mapping(
    pool: &DbPool,
    id: &str,
) -> Result<(), Error> {
    let conn = pool.get()?;
    
    diesel::delete(discussion_mappings::table)
        .filter(discussion_mappings::id.eq(id))
        .execute(&conn)?;
    
    Ok(())
}

pub async fn get_discussion_mappings_by_course(
    pool: &DbPool,
    course_category_id: &str,
) -> Result<Vec<DiscussionMapping>, Error> {
    let conn = pool.get()?;
    
    let mappings = discussion_mappings::table
        .filter(discussion_mappings::course_category_id.eq(course_category_id))
        .load::<DiscussionMapping>(&conn)?;
    
    Ok(mappings)
}

pub async fn get_all_discussion_mappings(
    pool: &DbPool,
) -> Result<Vec<DiscussionMapping>, Error> {
    let conn = pool.get()?;
    
    let mappings = discussion_mappings::table
        .load::<DiscussionMapping>(&conn)?;
    
    Ok(mappings)
}

pub async fn update_sync_timestamp(
    pool: &DbPool,
    id: &str,
) -> Result<(), Error> {
    let conn = pool.get()?;
    
    diesel::update(discussion_mappings::table)
        .filter(discussion_mappings::id.eq(id))
        .set(discussion_mappings::last_sync.eq(Utc::now()))
        .execute(&conn)?;
    
    Ok(())
}