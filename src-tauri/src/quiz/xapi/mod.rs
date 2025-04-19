use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use anyhow::{Result, anyhow};
use reqwest::Client;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use tracing::{debug, info, warn, error};

/// xAPI statement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XApiStatement {
    /// Statement ID
    pub id: Option<String>,
    
    /// Actor
    pub actor: XApiActor,
    
    /// Verb
    pub verb: XApiVerb,
    
    /// Object
    pub object: XApiObject,
    
    /// Result
    pub result: Option<XApiResult>,
    
    /// Context
    pub context: Option<XApiContext>,
    
    /// Timestamp
    pub timestamp: Option<DateTime<Utc>>,
    
    /// Stored
    pub stored: Option<DateTime<Utc>>,
    
    /// Authority
    pub authority: Option<XApiActor>,
    
    /// Version
    pub version: Option<String>,
    
    /// Attachments
    pub attachments: Option<Vec<XApiAttachment>>,
}

/// xAPI actor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XApiActor {
    /// Actor name
    pub name: Option<String>,
    
    /// Actor mbox
    pub mbox: Option<String>,
    
    /// Actor mbox SHA1 sum
    pub mbox_sha1sum: Option<String>,
    
    /// Actor OpenID
    pub openid: Option<String>,
    
    /// Actor account
    pub account: Option<XApiAccount>,
    
    /// Actor object type
    #[serde(rename = "objectType")]
    pub object_type: Option<String>,
}

/// xAPI account
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XApiAccount {
    /// Account home page
    #[serde(rename = "homePage")]
    pub home_page: String,
    
    /// Account name
    pub name: String,
}

/// xAPI verb
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XApiVerb {
    /// Verb ID
    pub id: String,
    
    /// Verb display
    pub display: HashMap<String, String>,
}

/// xAPI object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XApiObject {
    /// Object ID
    pub id: String,
    
    /// Object definition
    pub definition: Option<XApiDefinition>,
    
    /// Object type
    #[serde(rename = "objectType")]
    pub object_type: Option<String>,
}

/// xAPI definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XApiDefinition {
    /// Definition name
    pub name: Option<HashMap<String, String>>,
    
    /// Definition description
    pub description: Option<HashMap<String, String>>,
    
    /// Definition type
    #[serde(rename = "type")]
    pub definition_type: Option<String>,
    
    /// Definition more info
    #[serde(rename = "moreInfo")]
    pub more_info: Option<String>,
    
    /// Definition extensions
    pub extensions: Option<HashMap<String, serde_json::Value>>,
    
    /// Definition interaction type
    #[serde(rename = "interactionType")]
    pub interaction_type: Option<String>,
    
    /// Definition correct response pattern
    #[serde(rename = "correctResponsesPattern")]
    pub correct_responses_pattern: Option<Vec<String>>,
    
    /// Definition choices
    pub choices: Option<Vec<XApiInteractionComponent>>,
    
    /// Definition scale
    pub scale: Option<Vec<XApiInteractionComponent>>,
    
    /// Definition source
    pub source: Option<Vec<XApiInteractionComponent>>,
    
    /// Definition target
    pub target: Option<Vec<XApiInteractionComponent>>,
    
    /// Definition steps
    pub steps: Option<Vec<XApiInteractionComponent>>,
}

/// xAPI interaction component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XApiInteractionComponent {
    /// Component ID
    pub id: String,
    
    /// Component description
    pub description: HashMap<String, String>,
}

/// xAPI result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XApiResult {
    /// Result score
    pub score: Option<XApiScore>,
    
    /// Result success
    pub success: Option<bool>,
    
    /// Result completion
    pub completion: Option<bool>,
    
    /// Result response
    pub response: Option<String>,
    
    /// Result duration
    pub duration: Option<String>,
    
    /// Result extensions
    pub extensions: Option<HashMap<String, serde_json::Value>>,
}

/// xAPI score
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XApiScore {
    /// Score scaled
    pub scaled: Option<f32>,
    
    /// Score raw
    pub raw: Option<f32>,
    
    /// Score min
    pub min: Option<f32>,
    
    /// Score max
    pub max: Option<f32>,
}

/// xAPI context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XApiContext {
    /// Context registration
    pub registration: Option<String>,
    
    /// Context instructor
    pub instructor: Option<XApiActor>,
    
    /// Context team
    pub team: Option<XApiActor>,
    
    /// Context context activities
    #[serde(rename = "contextActivities")]
    pub context_activities: Option<XApiContextActivities>,
    
    /// Context revision
    pub revision: Option<String>,
    
    /// Context platform
    pub platform: Option<String>,
    
    /// Context language
    pub language: Option<String>,
    
    /// Context statement
    pub statement: Option<XApiStatementReference>,
    
    /// Context extensions
    pub extensions: Option<HashMap<String, serde_json::Value>>,
}

/// xAPI context activities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XApiContextActivities {
    /// Parent activities
    pub parent: Option<Vec<XApiObject>>,
    
    /// Grouping activities
    pub grouping: Option<Vec<XApiObject>>,
    
    /// Category activities
    pub category: Option<Vec<XApiObject>>,
    
    /// Other activities
    pub other: Option<Vec<XApiObject>>,
}

/// xAPI statement reference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XApiStatementReference {
    /// Statement ID
    pub id: String,
    
    /// Object type
    #[serde(rename = "objectType")]
    pub object_type: String,
}

/// xAPI attachment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XApiAttachment {
    /// Attachment usage type
    #[serde(rename = "usageType")]
    pub usage_type: String,
    
    /// Attachment display
    pub display: HashMap<String, String>,
    
    /// Attachment description
    pub description: Option<HashMap<String, String>>,
    
    /// Attachment content type
    #[serde(rename = "contentType")]
    pub content_type: String,
    
    /// Attachment length
    pub length: i32,
    
    /// Attachment SHA2
    pub sha2: String,
    
    /// Attachment file URL
    #[serde(rename = "fileUrl")]
    pub file_url: Option<String>,
    
    /// Attachment content
    pub content: Option<String>,
}

/// xAPI client configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XApiClientConfig {
    /// LRS endpoint URL
    pub endpoint: String,
    
    /// LRS username
    pub username: String,
    
    /// LRS password
    pub password: String,
    
    /// LRS version
    pub version: String,
}

/// xAPI client
pub struct XApiClient {
    /// HTTP client
    client: Client,
    
    /// LRS endpoint URL
    endpoint: String,
    
    /// Authorization header
    auth_header: String,
    
    /// LRS version
    version: String,
}

impl XApiClient {
    /// Create a new xAPI client
    pub fn new(config: XApiClientConfig) -> Result<Self> {
        // Create HTTP client
        let client = Client::new();
        
        // Create authorization header
        let auth = format!("{}:{}", config.username, config.password);
        let auth_header = format!("Basic {}", base64::encode(auth));
        
        Ok(Self {
            client,
            endpoint: config.endpoint,
            auth_header,
            version: config.version,
        })
    }
    
    /// Send a statement to the LRS
    pub async fn send_statement(&self, statement: &XApiStatement) -> Result<String> {
        // Create headers
        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, HeaderValue::from_str(&self.auth_header)?);
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert("X-Experience-API-Version", HeaderValue::from_str(&self.version)?);
        
        // Send the request
        let response = self.client.post(format!("{}/statements", self.endpoint))
            .headers(headers)
            .json(statement)
            .send()
            .await?;
        
        // Check the response
        if !response.status().is_success() {
            return Err(anyhow!("Failed to send statement: {}", response.status()));
        }
        
        // Parse the response
        let statement_id: String = response.json().await?;
        
        Ok(statement_id)
    }
    
    /// Get a statement from the LRS
    pub async fn get_statement(&self, statement_id: &str) -> Result<XApiStatement> {
        // Create headers
        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, HeaderValue::from_str(&self.auth_header)?);
        headers.insert("X-Experience-API-Version", HeaderValue::from_str(&self.version)?);
        
        // Send the request
        let response = self.client.get(format!("{}/statements?statementId={}", self.endpoint, statement_id))
            .headers(headers)
            .send()
            .await?;
        
        // Check the response
        if !response.status().is_success() {
            return Err(anyhow!("Failed to get statement: {}", response.status()));
        }
        
        // Parse the response
        let statement: XApiStatement = response.json().await?;
        
        Ok(statement)
    }
    
    /// Query statements from the LRS
    pub async fn query_statements(&self, params: &HashMap<String, String>) -> Result<Vec<XApiStatement>> {
        // Create headers
        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, HeaderValue::from_str(&self.auth_header)?);
        headers.insert("X-Experience-API-Version", HeaderValue::from_str(&self.version)?);
        
        // Build the query URL
        let mut url = format!("{}/statements", self.endpoint);
        if !params.is_empty() {
            url.push('?');
            for (i, (key, value)) in params.iter().enumerate() {
                if i > 0 {
                    url.push('&');
                }
                url.push_str(&format!("{}={}", key, value));
            }
        }
        
        // Send the request
        let response = self.client.get(&url)
            .headers(headers)
            .send()
            .await?;
        
        // Check the response
        if !response.status().is_success() {
            return Err(anyhow!("Failed to query statements: {}", response.status()));
        }
        
        // Parse the response
        let result: serde_json::Value = response.json().await?;
        
        // Extract the statements
        let statements = result["statements"].as_array()
            .ok_or_else(|| anyhow!("Invalid response format"))?
            .iter()
            .map(|statement| serde_json::from_value::<XApiStatement>(statement.clone()))
            .collect::<Result<Vec<_>, _>>()?;
        
        Ok(statements)
    }
}

/// xAPI statement builder
pub struct XApiStatementBuilder {
    /// Statement
    statement: XApiStatement,
}

impl XApiStatementBuilder {
    /// Create a new statement builder
    pub fn new() -> Self {
        Self {
            statement: XApiStatement {
                id: None,
                actor: XApiActor {
                    name: None,
                    mbox: None,
                    mbox_sha1sum: None,
                    openid: None,
                    account: None,
                    object_type: Some("Agent".to_string()),
                },
                verb: XApiVerb {
                    id: "".to_string(),
                    display: HashMap::new(),
                },
                object: XApiObject {
                    id: "".to_string(),
                    definition: None,
                    object_type: Some("Activity".to_string()),
                },
                result: None,
                context: None,
                timestamp: Some(Utc::now()),
                stored: None,
                authority: None,
                version: Some("1.0.3".to_string()),
                attachments: None,
            },
        }
    }
    
    /// Set the statement ID
    pub fn with_id(mut self, id: &str) -> Self {
        self.statement.id = Some(id.to_string());
        self
    }
    
    /// Set the actor
    pub fn with_actor(mut self, actor: XApiActor) -> Self {
        self.statement.actor = actor;
        self
    }
    
    /// Set the actor from a user
    pub fn with_actor_from_user(mut self, user_id: &str, user_name: &str, user_email: &str) -> Self {
        self.statement.actor = XApiActor {
            name: Some(user_name.to_string()),
            mbox: Some(format!("mailto:{}", user_email)),
            mbox_sha1sum: None,
            openid: None,
            account: None,
            object_type: Some("Agent".to_string()),
        };
        self
    }
    
    /// Set the verb
    pub fn with_verb(mut self, verb: XApiVerb) -> Self {
        self.statement.verb = verb;
        self
    }
    
    /// Set the verb from ID and display
    pub fn with_verb_from_id(mut self, id: &str, display: &str) -> Self {
        self.statement.verb = XApiVerb {
            id: id.to_string(),
            display: {
                let mut map = HashMap::new();
                map.insert("en-US".to_string(), display.to_string());
                map
            },
        };
        self
    }
    
    /// Set the object
    pub fn with_object(mut self, object: XApiObject) -> Self {
        self.statement.object = object;
        self
    }
    
    /// Set the object from ID and name
    pub fn with_object_from_id(mut self, id: &str, name: &str, description: Option<&str>) -> Self {
        self.statement.object = XApiObject {
            id: id.to_string(),
            definition: Some(XApiDefinition {
                name: Some({
                    let mut map = HashMap::new();
                    map.insert("en-US".to_string(), name.to_string());
                    map
                }),
                description: description.map(|desc| {
                    let mut map = HashMap::new();
                    map.insert("en-US".to_string(), desc.to_string());
                    map
                }),
                definition_type: None,
                more_info: None,
                extensions: None,
                interaction_type: None,
                correct_responses_pattern: None,
                choices: None,
                scale: None,
                source: None,
                target: None,
                steps: None,
            }),
            object_type: Some("Activity".to_string()),
        };
        self
    }
    
    /// Set the result
    pub fn with_result(mut self, result: XApiResult) -> Self {
        self.statement.result = Some(result);
        self
    }
    
    /// Set the result from score and completion
    pub fn with_result_from_score(mut self, score: f32, max_score: f32, success: bool, completion: bool) -> Self {
        self.statement.result = Some(XApiResult {
            score: Some(XApiScore {
                scaled: Some(score / max_score),
                raw: Some(score),
                min: Some(0.0),
                max: Some(max_score),
            }),
            success: Some(success),
            completion: Some(completion),
            response: None,
            duration: None,
            extensions: None,
        });
        self
    }
    
    /// Set the context
    pub fn with_context(mut self, context: XApiContext) -> Self {
        self.statement.context = Some(context);
        self
    }
    
    /// Set the timestamp
    pub fn with_timestamp(mut self, timestamp: DateTime<Utc>) -> Self {
        self.statement.timestamp = Some(timestamp);
        self
    }
    
    /// Build the statement
    pub fn build(self) -> XApiStatement {
        self.statement
    }
}

/// Create a quiz started statement
pub fn create_quiz_started_statement(user_id: &str, user_name: &str, user_email: &str, quiz_id: &str, quiz_name: &str) -> XApiStatement {
    XApiStatementBuilder::new()
        .with_actor_from_user(user_id, user_name, user_email)
        .with_verb_from_id("http://adlnet.gov/expapi/verbs/attempted", "attempted")
        .with_object_from_id(
            &format!("http://example.com/quizzes/{}", quiz_id),
            quiz_name,
            Some("A quiz activity")
        )
        .build()
}

/// Create a quiz completed statement
pub fn create_quiz_completed_statement(user_id: &str, user_name: &str, user_email: &str, quiz_id: &str, quiz_name: &str, score: f32, max_score: f32, success: bool) -> XApiStatement {
    XApiStatementBuilder::new()
        .with_actor_from_user(user_id, user_name, user_email)
        .with_verb_from_id("http://adlnet.gov/expapi/verbs/completed", "completed")
        .with_object_from_id(
            &format!("http://example.com/quizzes/{}", quiz_id),
            quiz_name,
            Some("A quiz activity")
        )
        .with_result_from_score(score, max_score, success, true)
        .build()
}

/// Create a question answered statement
pub fn create_question_answered_statement(user_id: &str, user_name: &str, user_email: &str, quiz_id: &str, quiz_name: &str, question_id: &str, question_text: &str, response: &str, success: bool) -> XApiStatement {
    XApiStatementBuilder::new()
        .with_actor_from_user(user_id, user_name, user_email)
        .with_verb_from_id("http://adlnet.gov/expapi/verbs/answered", "answered")
        .with_object_from_id(
            &format!("http://example.com/quizzes/{}/questions/{}", quiz_id, question_id),
            question_text,
            None
        )
        .with_result(XApiResult {
            score: None,
            success: Some(success),
            completion: Some(true),
            response: Some(response.to_string()),
            duration: None,
            extensions: None,
        })
        .with_context(XApiContext {
            registration: None,
            instructor: None,
            team: None,
            context_activities: Some(XApiContextActivities {
                parent: Some(vec![XApiObject {
                    id: format!("http://example.com/quizzes/{}", quiz_id),
                    definition: Some(XApiDefinition {
                        name: Some({
                            let mut map = HashMap::new();
                            map.insert("en-US".to_string(), quiz_name.to_string());
                            map
                        }),
                        description: None,
                        definition_type: None,
                        more_info: None,
                        extensions: None,
                        interaction_type: None,
                        correct_responses_pattern: None,
                        choices: None,
                        scale: None,
                        source: None,
                        target: None,
                        steps: None,
                    }),
                    object_type: Some("Activity".to_string()),
                }]),
                grouping: None,
                category: None,
                other: None,
            }),
            revision: None,
            platform: Some("Quiz Module".to_string()),
            language: Some("en-US".to_string()),
            statement: None,
            extensions: None,
        })
        .build()
}
