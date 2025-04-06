use sea_orm::entity::prelude::*;
use chrono::Utc;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "discussion_topic_mappings")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    
    // Reference to the parent course mapping
    pub course_mapping_id: i32,
    
    // Canvas discussion topic identifiers
    #[sea_orm(unique)]
    pub canvas_discussion_id: String,
    pub canvas_discussion_title: String,
    
    // Discourse topic identifiers
    #[sea_orm(unique, nullable)]
    pub discourse_topic_id: Option<i32>,
    #[sea_orm(nullable)]
    pub discourse_topic_slug: Option<String>,
    
    // Timestamps
    pub created_at: DateTime,
    pub updated_at: DateTime,
    #[sea_orm(nullable)]
    pub last_sync_at: Option<DateTime>,
    
    // Sync status
    pub is_active: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::course_category_mapping::Entity",
        from = "Column::CourseMappingId",
        to = "super::course_category_mapping::Column::Id"
    )]
    CourseCategoryMapping,
}

impl Related<super::course_category_mapping::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CourseCategoryMapping.def()
    }
}

impl ActiveModelBehavior for ActiveModel {
    /// Called before insert
    fn before_save(mut self, insert: bool) -> Result<Self, DbErr> {
        let now = Utc::now().naive_utc();
        
        if insert {
            self.created_at = Set(now);
        }
        self.updated_at = Set(now);
        
        Ok(self)
    }
}

// Simplify access to the entity
pub use Entity as DiscussionTopicMapping;