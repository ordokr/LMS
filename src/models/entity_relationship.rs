use serde::{Serialize, Deserialize};
use uuid::Uuid;
use std::collections::HashMap;

/// Entity type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EntityType {
    /// User
    User,
    /// Course
    Course,
    /// Discussion
    Discussion,
    /// Comment
    Comment,
    /// Assignment
    Assignment,
    /// Submission
    Submission,
    /// Group
    Group,
    /// Page
    Page,
    /// File
    File,
    /// Module
    Module,
    /// Quiz
    Quiz,
    /// Announcement
    Announcement,
    /// Calendar Event
    CalendarEvent,
    /// Tag
    Tag,
}

impl EntityType {
    /// Convert to string
    pub fn as_str(&self) -> &'static str {
        match self {
            EntityType::User => "user",
            EntityType::Course => "course",
            EntityType::Discussion => "discussion",
            EntityType::Comment => "comment",
            EntityType::Assignment => "assignment",
            EntityType::Submission => "submission",
            EntityType::Group => "group",
            EntityType::Page => "page",
            EntityType::File => "file",
            EntityType::Module => "module",
            EntityType::Quiz => "quiz",
            EntityType::Announcement => "announcement",
            EntityType::CalendarEvent => "calendar_event",
            EntityType::Tag => "tag",
        }
    }

    /// Convert from string
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "user" => Some(EntityType::User),
            "course" => Some(EntityType::Course),
            "discussion" => Some(EntityType::Discussion),
            "comment" => Some(EntityType::Comment),
            "assignment" => Some(EntityType::Assignment),
            "submission" => Some(EntityType::Submission),
            "group" => Some(EntityType::Group),
            "page" => Some(EntityType::Page),
            "file" => Some(EntityType::File),
            "module" => Some(EntityType::Module),
            "quiz" => Some(EntityType::Quiz),
            "announcement" => Some(EntityType::Announcement),
            "calendar_event" => Some(EntityType::CalendarEvent),
            "tag" => Some(EntityType::Tag),
            _ => None,
        }
    }
}

/// Relationship type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RelationshipType {
    /// Parent-child relationship
    ParentChild,
    /// Belongs to relationship
    BelongsTo,
    /// Has many relationship
    HasMany,
    /// Has one relationship
    HasOne,
    /// Many-to-many relationship
    ManyToMany,
    /// Reference relationship
    Reference,
    /// Created by relationship
    CreatedBy,
    /// Assigned to relationship
    AssignedTo,
    /// Submitted by relationship
    SubmittedBy,
    /// Tagged with relationship
    TaggedWith,
    /// Attached to relationship
    AttachedTo,
    /// Member of relationship
    MemberOf,
    /// Leader of relationship
    LeaderOf,
}

impl RelationshipType {
    /// Convert to string
    pub fn as_str(&self) -> &'static str {
        match self {
            RelationshipType::ParentChild => "parent_child",
            RelationshipType::BelongsTo => "belongs_to",
            RelationshipType::HasMany => "has_many",
            RelationshipType::HasOne => "has_one",
            RelationshipType::ManyToMany => "many_to_many",
            RelationshipType::Reference => "reference",
            RelationshipType::CreatedBy => "created_by",
            RelationshipType::AssignedTo => "assigned_to",
            RelationshipType::SubmittedBy => "submitted_by",
            RelationshipType::TaggedWith => "tagged_with",
            RelationshipType::AttachedTo => "attached_to",
            RelationshipType::MemberOf => "member_of",
            RelationshipType::LeaderOf => "leader_of",
        }
    }

    /// Convert from string
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "parent_child" => Some(RelationshipType::ParentChild),
            "belongs_to" => Some(RelationshipType::BelongsTo),
            "has_many" => Some(RelationshipType::HasMany),
            "has_one" => Some(RelationshipType::HasOne),
            "many_to_many" => Some(RelationshipType::ManyToMany),
            "reference" => Some(RelationshipType::Reference),
            "created_by" => Some(RelationshipType::CreatedBy),
            "assigned_to" => Some(RelationshipType::AssignedTo),
            "submitted_by" => Some(RelationshipType::SubmittedBy),
            "tagged_with" => Some(RelationshipType::TaggedWith),
            "attached_to" => Some(RelationshipType::AttachedTo),
            "member_of" => Some(RelationshipType::MemberOf),
            "leader_of" => Some(RelationshipType::LeaderOf),
            _ => None,
        }
    }

    /// Get the inverse relationship type
    pub fn inverse(&self) -> Self {
        match self {
            RelationshipType::ParentChild => RelationshipType::ParentChild,
            RelationshipType::BelongsTo => RelationshipType::HasMany,
            RelationshipType::HasMany => RelationshipType::BelongsTo,
            RelationshipType::HasOne => RelationshipType::BelongsTo,
            RelationshipType::ManyToMany => RelationshipType::ManyToMany,
            RelationshipType::Reference => RelationshipType::Reference,
            RelationshipType::CreatedBy => RelationshipType::HasMany,
            RelationshipType::AssignedTo => RelationshipType::HasMany,
            RelationshipType::SubmittedBy => RelationshipType::HasMany,
            RelationshipType::TaggedWith => RelationshipType::HasMany,
            RelationshipType::AttachedTo => RelationshipType::HasMany,
            RelationshipType::MemberOf => RelationshipType::HasMany,
            RelationshipType::LeaderOf => RelationshipType::HasOne,
        }
    }
}

/// Entity relationship
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityRelationship {
    /// Source entity type
    pub source_type: EntityType,
    /// Source entity ID
    pub source_id: Uuid,
    /// Target entity type
    pub target_type: EntityType,
    /// Target entity ID
    pub target_id: Uuid,
    /// Relationship type
    pub relationship_type: RelationshipType,
    /// Relationship metadata
    pub metadata: HashMap<String, String>,
}

impl EntityRelationship {
    /// Create a new entity relationship
    pub fn new(
        source_type: EntityType,
        source_id: Uuid,
        target_type: EntityType,
        target_id: Uuid,
        relationship_type: RelationshipType,
    ) -> Self {
        Self {
            source_type,
            source_id,
            target_type,
            target_id,
            relationship_type,
            metadata: HashMap::new(),
        }
    }

    /// Add metadata
    pub fn with_metadata(mut self, key: &str, value: &str) -> Self {
        self.metadata.insert(key.to_string(), value.to_string());
        self
    }

    /// Get the inverse relationship
    pub fn inverse(&self) -> Self {
        Self::new(
            self.target_type,
            self.target_id,
            self.source_type,
            self.source_id,
            self.relationship_type.inverse(),
        )
    }
}

/// Entity relationship graph
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EntityRelationshipGraph {
    /// Relationships
    pub relationships: Vec<EntityRelationship>,
}

impl EntityRelationshipGraph {
    /// Create a new entity relationship graph
    pub fn new() -> Self {
        Self {
            relationships: Vec::new(),
        }
    }

    /// Add a relationship
    pub fn add_relationship(&mut self, relationship: EntityRelationship) {
        self.relationships.push(relationship);
    }

    /// Get relationships by source
    pub fn get_relationships_by_source(
        &self,
        source_type: EntityType,
        source_id: Uuid,
    ) -> Vec<&EntityRelationship> {
        self.relationships
            .iter()
            .filter(|r| r.source_type == source_type && r.source_id == source_id)
            .collect()
    }

    /// Get relationships by target
    pub fn get_relationships_by_target(
        &self,
        target_type: EntityType,
        target_id: Uuid,
    ) -> Vec<&EntityRelationship> {
        self.relationships
            .iter()
            .filter(|r| r.target_type == target_type && r.target_id == target_id)
            .collect()
    }

    /// Get relationships by type
    pub fn get_relationships_by_type(
        &self,
        relationship_type: RelationshipType,
    ) -> Vec<&EntityRelationship> {
        self.relationships
            .iter()
            .filter(|r| r.relationship_type == relationship_type)
            .collect()
    }

    /// Get relationships between entities
    pub fn get_relationships_between(
        &self,
        source_type: EntityType,
        source_id: Uuid,
        target_type: EntityType,
        target_id: Uuid,
    ) -> Vec<&EntityRelationship> {
        self.relationships
            .iter()
            .filter(|r| {
                r.source_type == source_type
                    && r.source_id == source_id
                    && r.target_type == target_type
                    && r.target_id == target_id
            })
            .collect()
    }

    /// Check if a relationship exists
    pub fn has_relationship(
        &self,
        source_type: EntityType,
        source_id: Uuid,
        target_type: EntityType,
        target_id: Uuid,
        relationship_type: RelationshipType,
    ) -> bool {
        self.relationships.iter().any(|r| {
            r.source_type == source_type
                && r.source_id == source_id
                && r.target_type == target_type
                && r.target_id == target_id
                && r.relationship_type == relationship_type
        })
    }

    /// Remove relationships by source
    pub fn remove_relationships_by_source(
        &mut self,
        source_type: EntityType,
        source_id: Uuid,
    ) -> Vec<EntityRelationship> {
        let mut removed = Vec::new();
        self.relationships.retain(|r| {
            if r.source_type == source_type && r.source_id == source_id {
                removed.push(r.clone());
                false
            } else {
                true
            }
        });
        removed
    }

    /// Remove relationships by target
    pub fn remove_relationships_by_target(
        &mut self,
        target_type: EntityType,
        target_id: Uuid,
    ) -> Vec<EntityRelationship> {
        let mut removed = Vec::new();
        self.relationships.retain(|r| {
            if r.target_type == target_type && r.target_id == target_id {
                removed.push(r.clone());
                false
            } else {
                true
            }
        });
        removed
    }

    /// Remove relationships between entities
    pub fn remove_relationships_between(
        &mut self,
        source_type: EntityType,
        source_id: Uuid,
        target_type: EntityType,
        target_id: Uuid,
    ) -> Vec<EntityRelationship> {
        let mut removed = Vec::new();
        self.relationships.retain(|r| {
            if r.source_type == source_type
                && r.source_id == source_id
                && r.target_type == target_type
                && r.target_id == target_id
            {
                removed.push(r.clone());
                false
            } else {
                true
            }
        });
        removed
    }
}
