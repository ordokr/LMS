// cmi5 Statements
//
// This module provides functionality for creating and managing xAPI statements
// according to the cmi5 specification.

use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::quiz::cmi5::models::{Cmi5Context, Cmi5Verb, Cmi5Result, Cmi5Score};

/// cmi5 statement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cmi5Statement {
    /// Statement ID
    pub id: String,
    
    /// Actor
    pub actor: Actor,
    
    /// Verb
    pub verb: Cmi5Verb,
    
    /// Object
    pub object: Object,
    
    /// Result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Cmi5Result>,
    
    /// Context
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<Cmi5Context>,
    
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

/// Actor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Actor {
    /// Name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    
    /// Mailbox
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mbox: Option<String>,
    
    /// Account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<Account>,
    
    /// Object type
    #[serde(rename = "objectType")]
    pub object_type: String,
}

/// Account
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    /// Name
    pub name: String,
    
    /// Home page
    #[serde(rename = "homePage")]
    pub home_page: String,
}

/// Object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Object {
    /// ID
    pub id: String,
    
    /// Object type
    #[serde(rename = "objectType", skip_serializing_if = "Option::is_none")]
    pub object_type: Option<String>,
    
    /// Definition
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<ObjectDefinition>,
}

/// Object definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectDefinition {
    /// Name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<HashMap<String, String>>,
    
    /// Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<HashMap<String, String>>,
    
    /// Type
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub definition_type: Option<String>,
    
    /// More info
    #[serde(rename = "moreInfo", skip_serializing_if = "Option::is_none")]
    pub more_info: Option<String>,
    
    /// Extensions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<HashMap<String, serde_json::Value>>,
}

/// Statement type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatementType {
    /// Initialized
    Initialized,
    
    /// Completed
    Completed,
    
    /// Passed
    Passed,
    
    /// Failed
    Failed,
    
    /// Terminated
    Terminated,
    
    /// Satisfied
    Satisfied,
    
    /// Abandoned
    Abandoned,
    
    /// Waived
    Waived,
}

/// cmi5 statement builder
pub struct Cmi5StatementBuilder {
    /// Statement ID
    id: Option<String>,
    
    /// Actor
    actor: Option<Actor>,
    
    /// Verb
    verb: Option<Cmi5Verb>,
    
    /// Object
    object: Option<Object>,
    
    /// Result
    result: Option<Cmi5Result>,
    
    /// Context
    context: Option<Cmi5Context>,
    
    /// Timestamp
    timestamp: Option<DateTime<Utc>>,
}

impl Cmi5StatementBuilder {
    /// Create a new statement builder
    pub fn new() -> Self {
        Self {
            id: None,
            actor: None,
            verb: None,
            object: None,
            result: None,
            context: None,
            timestamp: None,
        }
    }
    
    /// Set the statement ID
    pub fn with_id(mut self, id: &str) -> Self {
        self.id = Some(id.to_string());
        self
    }
    
    /// Set the actor
    pub fn with_actor(mut self, actor: Actor) -> Self {
        self.actor = Some(actor);
        self
    }
    
    /// Set the actor from an email
    pub fn with_actor_email(mut self, email: &str, name: Option<&str>) -> Self {
        self.actor = Some(Actor {
            name: name.map(String::from),
            mbox: Some(format!("mailto:{}", email)),
            account: None,
            object_type: "Agent".to_string(),
        });
        self
    }
    
    /// Set the actor from an account
    pub fn with_actor_account(mut self, name: &str, home_page: &str, display_name: Option<&str>) -> Self {
        self.actor = Some(Actor {
            name: display_name.map(String::from),
            mbox: None,
            account: Some(Account {
                name: name.to_string(),
                home_page: home_page.to_string(),
            }),
            object_type: "Agent".to_string(),
        });
        self
    }
    
    /// Set the verb
    pub fn with_verb(mut self, verb: Cmi5Verb) -> Self {
        self.verb = Some(verb);
        self
    }
    
    /// Set the verb from ID and display
    pub fn with_verb_id(mut self, id: &str, display: HashMap<String, String>) -> Self {
        self.verb = Some(Cmi5Verb {
            id: id.to_string(),
            display,
        });
        self
    }
    
    /// Set the object
    pub fn with_object(mut self, object: Object) -> Self {
        self.object = Some(object);
        self
    }
    
    /// Set the object from ID
    pub fn with_object_id(mut self, id: &str) -> Self {
        self.object = Some(Object {
            id: id.to_string(),
            object_type: Some("Activity".to_string()),
            definition: None,
        });
        self
    }
    
    /// Set the result
    pub fn with_result(mut self, result: Cmi5Result) -> Self {
        self.result = Some(result);
        self
    }
    
    /// Set the context
    pub fn with_context(mut self, context: Cmi5Context) -> Self {
        self.context = Some(context);
        self
    }
    
    /// Set the timestamp
    pub fn with_timestamp(mut self, timestamp: DateTime<Utc>) -> Self {
        self.timestamp = Some(timestamp);
        self
    }
    
    /// Build the statement
    pub fn build(self) -> Result<Cmi5Statement, &'static str> {
        let actor = self.actor.ok_or("Actor is required")?;
        let verb = self.verb.ok_or("Verb is required")?;
        let object = self.object.ok_or("Object is required")?;
        
        Ok(Cmi5Statement {
            id: self.id.unwrap_or_else(|| Uuid::new_v4().to_string()),
            actor,
            verb,
            object,
            result: self.result,
            context: self.context,
            timestamp: self.timestamp.unwrap_or_else(Utc::now),
        })
    }
}

/// Create an initialized statement
pub fn create_initialized_statement(actor_id: &str, activity_id: &str, registration_id: &str) -> Cmi5Statement {
    let mut display = HashMap::new();
    display.insert("en-US".to_string(), "initialized".to_string());
    
    Cmi5StatementBuilder::new()
        .with_actor_email(actor_id, None)
        .with_verb_id("http://adlnet.gov/expapi/verbs/initialized", display)
        .with_object_id(activity_id)
        .with_context(Cmi5Context {
            registration: registration_id.to_string(),
            context_activities: crate::quiz::cmi5::models::ContextActivities {
                category: vec![crate::quiz::cmi5::models::Activity {
                    id: "https://w3id.org/xapi/cmi5/context/categories/cmi5".to_string(),
                    definition: None,
                }],
                grouping: None,
                parent: None,
                other: None,
            },
            extensions: None,
        })
        .build()
        .unwrap()
}

/// Create a completed statement
pub fn create_completed_statement(actor_id: &str, activity_id: &str, registration_id: &str) -> Cmi5Statement {
    let mut display = HashMap::new();
    display.insert("en-US".to_string(), "completed".to_string());
    
    Cmi5StatementBuilder::new()
        .with_actor_email(actor_id, None)
        .with_verb_id("http://adlnet.gov/expapi/verbs/completed", display)
        .with_object_id(activity_id)
        .with_result(Cmi5Result {
            score: None,
            success: None,
            completion: Some(true),
            duration: None,
        })
        .with_context(Cmi5Context {
            registration: registration_id.to_string(),
            context_activities: crate::quiz::cmi5::models::ContextActivities {
                category: vec![crate::quiz::cmi5::models::Activity {
                    id: "https://w3id.org/xapi/cmi5/context/categories/cmi5".to_string(),
                    definition: None,
                }],
                grouping: None,
                parent: None,
                other: None,
            },
            extensions: None,
        })
        .build()
        .unwrap()
}

/// Create a passed statement
pub fn create_passed_statement(
    actor_id: &str,
    activity_id: &str,
    registration_id: &str,
    score: Option<Cmi5Score>,
) -> Cmi5Statement {
    let mut display = HashMap::new();
    display.insert("en-US".to_string(), "passed".to_string());
    
    Cmi5StatementBuilder::new()
        .with_actor_email(actor_id, None)
        .with_verb_id("http://adlnet.gov/expapi/verbs/passed", display)
        .with_object_id(activity_id)
        .with_result(Cmi5Result {
            score,
            success: Some(true),
            completion: None,
            duration: None,
        })
        .with_context(Cmi5Context {
            registration: registration_id.to_string(),
            context_activities: crate::quiz::cmi5::models::ContextActivities {
                category: vec![crate::quiz::cmi5::models::Activity {
                    id: "https://w3id.org/xapi/cmi5/context/categories/cmi5".to_string(),
                    definition: None,
                }],
                grouping: None,
                parent: None,
                other: None,
            },
            extensions: None,
        })
        .build()
        .unwrap()
}

/// Create a failed statement
pub fn create_failed_statement(
    actor_id: &str,
    activity_id: &str,
    registration_id: &str,
    score: Option<Cmi5Score>,
) -> Cmi5Statement {
    let mut display = HashMap::new();
    display.insert("en-US".to_string(), "failed".to_string());
    
    Cmi5StatementBuilder::new()
        .with_actor_email(actor_id, None)
        .with_verb_id("http://adlnet.gov/expapi/verbs/failed", display)
        .with_object_id(activity_id)
        .with_result(Cmi5Result {
            score,
            success: Some(false),
            completion: None,
            duration: None,
        })
        .with_context(Cmi5Context {
            registration: registration_id.to_string(),
            context_activities: crate::quiz::cmi5::models::ContextActivities {
                category: vec![crate::quiz::cmi5::models::Activity {
                    id: "https://w3id.org/xapi/cmi5/context/categories/cmi5".to_string(),
                    definition: None,
                }],
                grouping: None,
                parent: None,
                other: None,
            },
            extensions: None,
        })
        .build()
        .unwrap()
}

/// Create a terminated statement
pub fn create_terminated_statement(actor_id: &str, activity_id: &str, registration_id: &str) -> Cmi5Statement {
    let mut display = HashMap::new();
    display.insert("en-US".to_string(), "terminated".to_string());
    
    Cmi5StatementBuilder::new()
        .with_actor_email(actor_id, None)
        .with_verb_id("http://adlnet.gov/expapi/verbs/terminated", display)
        .with_object_id(activity_id)
        .with_context(Cmi5Context {
            registration: registration_id.to_string(),
            context_activities: crate::quiz::cmi5::models::ContextActivities {
                category: vec![crate::quiz::cmi5::models::Activity {
                    id: "https://w3id.org/xapi/cmi5/context/categories/cmi5".to_string(),
                    definition: None,
                }],
                grouping: None,
                parent: None,
                other: None,
            },
            extensions: None,
        })
        .build()
        .unwrap()
}

/// Create a satisfied statement
pub fn create_satisfied_statement(actor_id: &str, activity_id: &str, registration_id: &str) -> Cmi5Statement {
    let mut display = HashMap::new();
    display.insert("en-US".to_string(), "satisfied".to_string());
    
    Cmi5StatementBuilder::new()
        .with_actor_email(actor_id, None)
        .with_verb_id("https://w3id.org/xapi/adl/verbs/satisfied", display)
        .with_object_id(activity_id)
        .with_context(Cmi5Context {
            registration: registration_id.to_string(),
            context_activities: crate::quiz::cmi5::models::ContextActivities {
                category: vec![crate::quiz::cmi5::models::Activity {
                    id: "https://w3id.org/xapi/cmi5/context/categories/cmi5".to_string(),
                    definition: None,
                }],
                grouping: None,
                parent: None,
                other: None,
            },
            extensions: None,
        })
        .build()
        .unwrap()
}

/// Create an abandoned statement
pub fn create_abandoned_statement(actor_id: &str, activity_id: &str, registration_id: &str) -> Cmi5Statement {
    let mut display = HashMap::new();
    display.insert("en-US".to_string(), "abandoned".to_string());
    
    Cmi5StatementBuilder::new()
        .with_actor_email(actor_id, None)
        .with_verb_id("https://w3id.org/xapi/adl/verbs/abandoned", display)
        .with_object_id(activity_id)
        .with_context(Cmi5Context {
            registration: registration_id.to_string(),
            context_activities: crate::quiz::cmi5::models::ContextActivities {
                category: vec![crate::quiz::cmi5::models::Activity {
                    id: "https://w3id.org/xapi/cmi5/context/categories/cmi5".to_string(),
                    definition: None,
                }],
                grouping: None,
                parent: None,
                other: None,
            },
            extensions: None,
        })
        .build()
        .unwrap()
}

/// Create a waived statement
pub fn create_waived_statement(
    actor_id: &str,
    activity_id: &str,
    registration_id: &str,
    reason: &str,
) -> Cmi5Statement {
    let mut display = HashMap::new();
    display.insert("en-US".to_string(), "waived".to_string());
    
    let mut extensions = HashMap::new();
    extensions.insert(
        "https://w3id.org/xapi/cmi5/context/extensions/reason".to_string(),
        serde_json::Value::String(reason.to_string()),
    );
    
    Cmi5StatementBuilder::new()
        .with_actor_email(actor_id, None)
        .with_verb_id("https://w3id.org/xapi/adl/verbs/waived", display)
        .with_object_id(activity_id)
        .with_context(Cmi5Context {
            registration: registration_id.to_string(),
            context_activities: crate::quiz::cmi5::models::ContextActivities {
                category: vec![crate::quiz::cmi5::models::Activity {
                    id: "https://w3id.org/xapi/cmi5/context/categories/cmi5".to_string(),
                    definition: None,
                }],
                grouping: None,
                parent: None,
                other: None,
            },
            extensions: Some(extensions),
        })
        .build()
        .unwrap()
}
