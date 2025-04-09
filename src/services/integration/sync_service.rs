use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::RwLock;
use serde::{Serialize, Deserialize};
use anyhow::{Result, anyhow, Context};
use lapin::{
    options::*, types::FieldTable, BasicProperties,
    Connection, ConnectionProperties, Channel, Consumer, 
    message::Delivery, ExchangeKind
};
use log::{info, error, warn};
use async_trait::async_trait;

use crate::api::{canvas_client::CanvasApi, discourse_client::DiscourseApi};
use crate::models::sync_state::SyncState;
use crate::models::sync_transaction::SyncTransaction;

/// Event priority levels for synchronization
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SyncPriority {
    Critical,
    High, 
    Background
}

impl SyncPriority {
    /// Convert priority to queue name
    pub fn queue_name(&self) -> String {
        match self {
            SyncPriority::Critical => "sync_critical".to_string(),
            SyncPriority::High => "sync_high".to_string(),
            SyncPriority::Background => "sync_background".to_string()
        }
    }
}

/// Types of entities that can be synchronized
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EntityType {
    User,
    Course,
    Assignment,
    Submission,
    Discussion,
    Post,
    Comment
}

impl EntityType {
    pub fn as_str(&self) -> &'static str {
        match self {
            EntityType::User => "user",
            EntityType::Course => "course",
            EntityType::Assignment => "assignment",
            EntityType::Submission => "submission",
            EntityType::Discussion => "discussion",
            EntityType::Post => "post",
            EntityType::Comment => "comment",
        }
    }

    pub fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "user" => Ok(EntityType::User),
            "course" => Ok(EntityType::Course),
            "assignment" => Ok(EntityType::Assignment),
            "submission" => Ok(EntityType::Submission),
            "discussion" => Ok(EntityType::Discussion),
            "post" => Ok(EntityType::Post),
            "comment" => Ok(EntityType::Comment),
            _ => Err(anyhow!("Invalid entity type: {}", s))
        }
    }
}

/// Operations that can be performed during synchronization
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SyncOperation {
    Create,
    Update,
    Delete,
    Sync,
}

impl SyncOperation {
    pub fn as_str(&self) -> &'static str {
        match self {
            SyncOperation::Create => "create",
            SyncOperation::Update => "update",
            SyncOperation::Delete => "delete",
            SyncOperation::Sync => "sync",
        }
    }

    pub fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "create" => Ok(SyncOperation::Create),
            "update" => Ok(SyncOperation::Update),
            "delete" => Ok(SyncOperation::Delete),
            "sync" => Ok(SyncOperation::Sync),
            _ => Err(anyhow!("Invalid operation: {}", s))
        }
    }
}

/// Source system for synchronization events
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SourceSystem {
    Canvas,
    Discourse,
}

impl SourceSystem {
    pub fn as_str(&self) -> &'static str {
        match self {
            SourceSystem::Canvas => "canvas",
            SourceSystem::Discourse => "discourse",
        }
    }

    pub fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "canvas" => Ok(SourceSystem::Canvas),
            "discourse" => Ok(SourceSystem::Discourse),
            _ => Err(anyhow!("Invalid source system: {}", s))
        }
    }
}

/// Synchronization event containing all data needed for processing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncEvent {
    pub entity_type: EntityType,
    pub entity_id: String,
    pub operation: SyncOperation,
    pub source_system: SourceSystem,
    pub target_system: Option<SourceSystem>,
    pub data: Option<serde_json::Value>,
    pub timestamp: String,
    pub transaction_id: String,
}

/// Interface for processing synchronization events
#[async_trait]
pub trait SyncEventProcessor: Send + Sync {
    async fn process_event(&self, event: SyncEvent) -> Result<()>;
}

/// Configuration for synchronization service
#[derive(Debug, Clone)]
pub struct SyncServiceConfig {
    pub rabbitmq_url: String,
    pub rabbitmq_prefetch_count: u16,
    pub processing_threads: usize,
}

impl Default for SyncServiceConfig {
    fn default() -> Self {
        Self {
            rabbitmq_url: "amqp://localhost".to_string(),
            rabbitmq_prefetch_count: 10,
            processing_threads: 4,
        }
    }
}

/// Core synchronization service that handles data consistency between Canvas and Discourse
pub struct SyncService {
    config: SyncServiceConfig,
    connection: Option<Connection>,
    channel: RwLock<Option<Channel>>,
    canvas_api: Arc<dyn CanvasApi>,
    discourse_api: Arc<dyn DiscourseApi>,
    sync_state: Arc<SyncState>,
    sync_transaction: Arc<SyncTransaction>,
    is_processing: RwLock<bool>,
    message_processors: HashMap<String, Box<dyn SyncEventProcessor>>,
}

impl SyncService {
    /// Create a new synchronization service
    pub fn new(
        config: SyncServiceConfig,
        canvas_api: Arc<dyn CanvasApi>,
        discourse_api: Arc<dyn DiscourseApi>,
        sync_state: Arc<SyncState>,
        sync_transaction: Arc<SyncTransaction>,
    ) -> Self {
        Self {
            config,
            connection: None,
            channel: RwLock::new(None),
            canvas_api,
            discourse_api,
            sync_state,
            sync_transaction,
            is_processing: RwLock::new(false),
            message_processors: HashMap::new(),
        }
    }

    /// Initialize the synchronization service
    pub async fn initialize(&mut self) -> Result<()> {
        info!("Initializing synchronization service");
        
        // Connect to RabbitMQ
        let connection = Connection::connect(
            &self.config.rabbitmq_url,
            ConnectionProperties::default()
        ).await.context("Failed to connect to RabbitMQ")?;
        
        self.connection = Some(connection);
        
        // Create channel
        if let Some(conn) = &self.connection {
            let channel = conn.create_channel().await?;
            
            // Setup queues with different priorities
            channel.queue_declare(
                "sync_critical",
                QueueDeclareOptions { 
                    durable: true,
                    ..QueueDeclareOptions::default()
                },
                FieldTable::default()
            ).await?;
            
            channel.queue_declare(
                "sync_high", 
                QueueDeclareOptions { 
                    durable: true, 
                    ..QueueDeclareOptions::default()
                }, 
                FieldTable::default()
            ).await?;
            
            channel.queue_declare(
                "sync_background",
                QueueDeclareOptions { 
                    durable: true, 
                    ..QueueDeclareOptions::default()
                }, 
                FieldTable::default()
            ).await?;
            
            // Dead letter queue for failed synchronizations
            channel.queue_declare(
                "sync_failed",
                QueueDeclareOptions { 
                    durable: true, 
                    ..QueueDeclareOptions::default()
                }, 
                FieldTable::default()
            ).await?;
            
            *self.channel.write().await = Some(channel);
            
            info!("Synchronization service initialized successfully");
            return Ok(());
        }
        
        Err(anyhow!("Failed to initialize synchronization service"))
    }
    
    /// Register a message processor for a specific entity type
    pub fn register_processor<P>(&mut self, entity_type: EntityType, processor: P) 
    where 
        P: SyncEventProcessor + 'static 
    {
        self.message_processors.insert(
            entity_type.as_str().to_string(), 
            Box::new(processor)
        );
        info!("Registered processor for entity type: {:?}", entity_type);
    }

    /// Publish a synchronization event
    pub async fn publish_event(
        &self,
        priority: SyncPriority,
        entity_type: EntityType,
        operation: SyncOperation,
        source_system: SourceSystem,
        entity_id: &str,
        data: Option<serde_json::Value>,
    ) -> Result<String> {
        let channel_guard = self.channel.read().await;
        let channel = channel_guard.as_ref().ok_or_else(|| anyhow!("Channel not initialized"))?;
        
        let queue_name = priority.queue_name();
        
        // Generate a unique transaction ID
        let transaction_id = format!(
            "tx-{}-{}", 
            chrono::Utc::now().timestamp_millis(),
            uuid::Uuid::new_v4().to_string().split('-').next().unwrap_or("default")
        );
        
        // Create the event object
        let event = SyncEvent {
            entity_type: entity_type.clone(),
            entity_id: entity_id.to_string(),
            operation: operation.clone(),
            source_system: source_system.clone(),
            target_system: Some(match source_system {
                SourceSystem::Canvas => SourceSystem::Discourse,
                SourceSystem::Discourse => SourceSystem::Canvas,
            }),
            data,
            timestamp: chrono::Utc::now().to_rfc3339(),
            transaction_id: transaction_id.clone(),
        };
        
        // Serialize the event
        let payload = serde_json::to_vec(&event)
            .context("Failed to serialize sync event")?;
        
        // Publish the event
        channel.basic_publish(
            "",
            &queue_name,
            BasicPublishOptions::default(),
            &payload,
            BasicProperties::default()
                .with_delivery_mode(2) // persistent
                .with_content_type("application/json".into())
        ).await.context("Failed to publish sync event")?;
        
        info!(
            "Published {} sync event: {} - {:?} {:?} {}", 
            priority.queue_name(), 
            transaction_id, 
            entity_type,
            operation,
            entity_id
        );
        
        Ok(transaction_id)
    }

    /// Start processing synchronization events
    pub async fn start_processing(&self) -> Result<()> {
        let mut is_processing = self.is_processing.write().await;
        
        if *is_processing {
            warn!("Sync processing is already running");
            return Ok(());
        }

        *is_processing = true;
        info!("Starting sync event processing");

        let channel_guard = self.channel.read().await;
        let channel = channel_guard.as_ref().ok_or_else(|| anyhow!("Channel not initialized"))?;
        
        // Set QoS (prefetch count)
        channel.basic_qos(self.config.rabbitmq_prefetch_count, BasicQosOptions::default()).await?;
        
        // Process critical events (highest priority)
        let critical_consumer = channel.basic_consume(
            "sync_critical",
            "sync_service_critical",
            BasicConsumeOptions::default(),
            FieldTable::default()
        ).await?;
        
        self.spawn_consumer(critical_consumer, SyncPriority::Critical).await?;
        
        // Process high priority events
        let high_consumer = channel.basic_consume(
            "sync_high",
            "sync_service_high",
            BasicConsumeOptions::default(),
            FieldTable::default()
        ).await?;
        
        self.spawn_consumer(high_consumer, SyncPriority::High).await?;
        
        // Process background events (lowest priority)
        let background_consumer = channel.basic_consume(
            "sync_background",
            "sync_service_background",
            BasicConsumeOptions::default(),
            FieldTable::default()
        ).await?;
        
        self.spawn_consumer(background_consumer, SyncPriority::Background).await?;

        Ok(())
    }

    /// Spawn a consumer task for processing messages from a queue
    async fn spawn_consumer(&self, mut consumer: Consumer, priority: SyncPriority) -> Result<()> {
        // Clone Arc references for the task
        let canvas_api = self.canvas_api.clone();
        let discourse_api = self.discourse_api.clone();
        let sync_state = self.sync_state.clone();
        let sync_transaction = self.sync_transaction.clone();
        let channel_lock = self.channel.clone();
        let processors = self.message_processors.clone();
        
        tokio::spawn(async move {
            info!("Started consumer for {} queue", priority.queue_name());
            
            while let Some(delivery) = consumer.next().await {
                match delivery {
                    Ok(delivery) => {
                        if let Err(e) = process_delivery(
                            delivery, 
                            &priority,
                            &canvas_api,
                            &discourse_api,
                            &sync_state,
                            &sync_transaction,
                            &channel_lock,
                            &processors
                        ).await {
                            error!(
                                "Error processing message from {} queue: {}", 
                                priority.queue_name(), 
                                e
                            );
                        }
                    }
                    Err(e) => {
                        error!(
                            "Error receiving message from {} queue: {}", 
                            priority.queue_name(), 
                            e
                        );
                    }
                }
            }
            
            info!("Consumer for {} queue has stopped", priority.queue_name());
        });
        
        Ok(())
    }
    
    /// Get queue status information
    pub async fn get_queue_status(&self) -> Result<HashMap<String, u32>> {
        let channel_guard = self.channel.read().await;
        let channel = channel_guard.as_ref().ok_or_else(|| anyhow!("Channel not initialized"))?;
        
        let mut status = HashMap::new();
        
        // Get queue lengths
        let critical = channel.queue_declare(
            "sync_critical", 
            QueueDeclareOptions { 
                passive: true,
                ..QueueDeclareOptions::default()
            }, 
            FieldTable::default()
        ).await?;
        
        let high = channel.queue_declare(
            "sync_high", 
            QueueDeclareOptions { 
                passive: true,
                ..QueueDeclareOptions::default()
            }, 
            FieldTable::default()
        ).await?;
        
        let background = channel.queue_declare(
            "sync_background", 
            QueueDeclareOptions { 
                passive: true,
                ..QueueDeclareOptions::default()
            }, 
            FieldTable::default()
        ).await?;
        
        status.insert("critical".to_string(), critical.message_count());
        status.insert("high".to_string(), high.message_count());
        status.insert("background".to_string(), background.message_count());
        
        Ok(status)
    }
    
    /// Process pending retries for failed synchronizations
    pub async fn process_pending_retries(&self, limit: usize) -> Result<HashMap<String, usize>> {
        info!("Processing pending synchronization retries (limit: {})", limit);
        
        // Get all entities that need resynchronization
        let pending_items = self.sync_state.get_pending_syncs(limit).await?;
        let mut results = HashMap::new();
        
        for item in pending_items {
            let entity_type = EntityType::from_str(&item.entity_type)?;
            let source_system = SourceSystem::from_str(&item.source_system)?;
            
            // Determine priority based on entity type
            let priority = match entity_type {
                EntityType::Submission => SyncPriority::Critical,
                EntityType::User | EntityType::Course | EntityType::Assignment => SyncPriority::High,
                _ => SyncPriority::Background
            };
            
            // Republish the event for processing
            if let Err(e) = self.publish_event(
                priority.clone(),
                entity_type.clone(),
                SyncOperation::Sync,
                source_system,
                &item.entity_id,
                None
            ).await {
                error!("Failed to reprocess entity {:?} {}: {}", entity_type, item.entity_id, e);
                continue;
            }
            
            // Count retries by priority
            *results.entry(priority.queue_name()).or_insert(0) += 1;
        }
        
        info!("Reprocessed {} pending items: {:?}", 
            results.values().sum::<usize>(),
            results
        );
        
        Ok(results)
    }

    /// Stop the synchronization service
    pub async fn stop(&self) -> Result<()> {
        info!("Stopping synchronization service");
        
        // Set the processing flag to false
        *self.is_processing.write().await = false;
        
        // Close the channel
        if let Some(channel) = self.channel.write().await.take() {
            channel.close(0, "shutdown").await?;
        }
        
        // Close the connection
        if let Some(conn) = &self.connection {
            conn.close(0, "shutdown").await?;
        }
        
        info!("Synchronization service stopped");
        Ok(())
    }
}

/// Process a delivery from RabbitMQ
async fn process_delivery(
    delivery: Delivery,
    priority: &SyncPriority,
    canvas_api: &Arc<dyn CanvasApi>,
    discourse_api: &Arc<dyn DiscourseApi>,
    sync_state: &Arc<SyncState>,
    sync_transaction: &Arc<SyncTransaction>,
    channel_lock: &RwLock<Option<Channel>>,
    processors: &HashMap<String, Box<dyn SyncEventProcessor>>,
) -> Result<()> {
    // Parse the message
    let event: SyncEvent = match serde_json::from_slice(&delivery.data) {
        Ok(event) => event,
        Err(e) => {
            // Acknowledge invalid messages to remove them from the queue
            let channel = channel_lock.read().await;
            if let Some(channel) = channel.as_ref() {
                channel.basic_ack(delivery.delivery_tag, BasicAckOptions::default()).await?;
            }
            return Err(anyhow!("Failed to parse sync event: {}", e));
        }
    };
    
    info!(
        "Processing {} sync event: {} - {:?} {:?} {}", 
        priority.queue_name(), 
        event.transaction_id, 
        event.entity_type,
        event.operation,
        event.entity_id
    );
    
    let result = process_event(
        &event, 
        canvas_api,
        discourse_api,
        sync_state,
        sync_transaction,
        processors
    ).await;
    
    let channel = channel_lock.read().await;
    if let Some(channel) = channel.as_ref() {
        match result {
            Ok(_) => {
                // Acknowledge successful processing
                channel.basic_ack(delivery.delivery_tag, BasicAckOptions::default()).await?;
                info!("Successfully processed sync event: {}", event.transaction_id);
            },
            Err(e) => {
                // Handle failed processing
                error!("Failed to process sync event {}: {}", event.transaction_id, e);
                
                // Update sync state to record the failure
                if let Err(state_err) = sync_state.update_sync_status(
                    &event.entity_type.as_str(),
                    &event.entity_id,
                    &event.source_system.as_str(),
                    None,
                    "FAILED",
                    Some(&e.to_string())
                ).await {
                    error!("Failed to update sync state: {}", state_err);
                }
                
                // Send to dead letter queue
                let failed_payload = serde_json::to_vec(&serde_json::json!({
                    "event": event,
                    "error": e.to_string(),
                    "timestamp": chrono::Utc::now().to_rfc3339()
                }))?;
                
                channel.basic_publish(
                    "",
                    "sync_failed",
                    BasicPublishOptions::default(),
                    &failed_payload,
                    BasicProperties::default()
                        .with_delivery_mode(2) // persistent
                        .with_content_type("application/json".into())
                ).await?;
                
                // Acknowledge the message to remove from the original queue
                channel.basic_ack(delivery.delivery_tag, BasicAckOptions::default()).await?;
            }
        }
    }
    
    Ok(())
}

/// Process a synchronization event
async fn process_event(
    event: &SyncEvent,
    canvas_api: &Arc<dyn CanvasApi>,
    discourse_api: &Arc<dyn DiscourseApi>,
    sync_state: &Arc<SyncState>,
    sync_transaction: &Arc<SyncTransaction>,
    processors: &HashMap<String, Box<dyn SyncEventProcessor>>
) -> Result<()> {
    // Create a transaction
    let transaction = sync_transaction.begin_transaction(
        &event.entity_type.as_str(),
        &event.entity_id,
        &event.operation.as_str(),
        &event.source_system.as_str()
    ).await?;
    
    // Record transaction start
    sync_transaction.record_step(
        &transaction.id,
        "STARTED",
        &format!("{:?} operation started", event.operation)
    ).await?;
    
    // Check if we have a processor for this entity type
    if let Some(processor) = processors.get(event.entity_type.as_str()) {
        // Use the registered processor to handle the event
        let result = processor.process_event(event.clone()).await;
        
        match result {
            Ok(_) => {
                // Mark as successful
                sync_transaction.commit(&transaction.id).await?;
                sync_state.update_sync_status(
                    &event.entity_type.as_str(),
                    &event.entity_id,
                    &event.source_system.as_str(),
                    None,
                    "SYNCED",
                    None
                ).await?;
                Ok(())
            }
            Err(e) => {
                // Mark as failed
                sync_transaction.rollback(&transaction.id, &e.to_string()).await?;
                Err(e)
            }
        }
    } else {
        // Fallback processor for unregistered entity types
        match event.entity_type {
            EntityType::User => {
                sync_user(event, canvas_api, discourse_api, sync_state, sync_transaction).await?;
            }
            EntityType::Course => {
                sync_course(event, canvas_api, discourse_api, sync_state, sync_transaction).await?;
            }
            EntityType::Assignment => {
                sync_assignment(event, canvas_api, discourse_api, sync_state, sync_transaction).await?;
            }
            EntityType::Submission => {
                sync_submission(event, canvas_api, discourse_api, sync_state, sync_transaction).await?;
            }
            EntityType::Discussion => {
                sync_discussion(event, canvas_api, discourse_api, sync_state, sync_transaction).await?;
            }
            _ => {
                return Err(anyhow!("No processor registered for entity type: {:?}", event.entity_type));
            }
        }
        
        // Mark transaction as successful
        sync_transaction.commit(&transaction.id).await?;
        
        Ok(())
    }
}

/// Synchronize a user between Canvas and Discourse
async fn sync_user(
    event: &SyncEvent,
    canvas_api: &Arc<dyn CanvasApi>,
    discourse_api: &Arc<dyn DiscourseApi>,
    sync_state: &Arc<SyncState>,
    sync_transaction: &Arc<SyncTransaction>
) -> Result<()> {
    info!("Syncing user {}: {:?} operation", event.entity_id, event.operation);
    
    match event.source_system {
        SourceSystem::Canvas => {
            // Sync from Canvas to Discourse
            match event.operation {
                SyncOperation::Create | SyncOperation::Update | SyncOperation::Sync => {
                    // Get user data from Canvas
                    let canvas_user = canvas_api.get_user(&event.entity_id).await?;
                    
                    // Check if user exists in Discourse
                    let discourse_user = discourse_api.get_user_by_external_id(&format!("canvas_{}", event.entity_id)).await;
                    
                    if let Ok(user) = discourse_user {
                        // Update existing user
                        let discourse_data = canvas_to_discourse_user(&canvas_user)?;
                        
                        discourse_api.update_user(&user.id.to_string(), &discourse_data).await?;
                        info!("Updated user in Discourse: {}", user.id);
                        
                        // Update mapping
                        sync_state.update_sync_status(
                            event.entity_type.as_str(),
                            &event.entity_id,
                            event.source_system.as_str(),
                            Some(&user.id.to_string()),
                            "SYNCED",
                            None
                        ).await?;
                    } else {
                        // Create new user
                        let discourse_data = canvas_to_discourse_user(&canvas_user)?;
                        
                        let created_user = discourse_api.create_user(&discourse_data).await?;
                        info!("Created user in Discourse: {}", created_user.id);
                        
                        // Store mapping
                        sync_state.update_sync_status(
                            event.entity_type.as_str(),
                            &event.entity_id,
                            event.source_system.as_str(),
                            Some(&created_user.id.to_string()),
                            "SYNCED",
                            None
                        ).await?;
                    }
                    
                    Ok(())
                }
                SyncOperation::Delete => {
                    // Find user in Discourse by external ID
                    let discourse_user = discourse_api.get_user_by_external_id(&format!("canvas_{}", event.entity_id)).await;
                    
                    if let Ok(user) = discourse_user {
                        // Deactivate user in Discourse
                        discourse_api.deactivate_user(&user.id.to_string()).await?;
                        info!("Deactivated user in Discourse: {}", user.id);
                    }
                    
                    Ok(())
                }
            }
        }
        SourceSystem::Discourse => {
            // Sync from Discourse to Canvas
            // This would be implemented based on your requirements for bidirectional sync
            Err(anyhow!("Synchronization from Discourse to Canvas not implemented for users"))
        }
    }
}

/// Convert Canvas user data to Discourse format
fn canvas_to_discourse_user(canvas_user: &serde_json::Value) -> Result<serde_json::Value> {
    // Extract required fields
    let name = canvas_user.get("name")
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow!("Missing name in Canvas user"))?;
    
    let email = canvas_user.get("email")
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow!("Missing email in Canvas user"))?;
    
    let id = canvas_user.get("id")
        .and_then(|v| v.as_str().or_else(|| v.as_u64().map(|n| n.to_string().as_str())))
        .ok_or_else(|| anyhow!("Missing id in Canvas user"))?;
    
    // Create username from email
    let email_parts: Vec<&str> = email.split('@').collect();
    let username = if email_parts.len() > 0 {
        // Replace non-alphanumeric chars with underscore
        let clean_username = email_parts[0]
            .chars()
            .map(|c| if c.is_alphanumeric() || c == '_' { c } else { '_' })
            .collect::<String>();
            
        if clean_username.is_empty() {
            format!("canvas_user_{}", id)
        } else {
            clean_username
        }
    } else {
        format!("canvas_user_{}", id)
    };
    
    // Build Discourse user object
    let discourse_user = serde_json::json!({
        "name": name,
        "username": username,
        "email": email,
        "password": format!("canvas_{}", chrono::Utc::now().timestamp()),
        "active": canvas_user.get("workflow_state").and_then(|v| v.as_str()) != Some("deleted"),
        "approved": true,
        "custom_fields": {
            "canvas_user_id": id
        }
    });
    
    Ok(discourse_user)
}

/// Synchronize a course between Canvas and Discourse
async fn sync_course(
    event: &SyncEvent,
    canvas_api: &Arc<dyn CanvasApi>,
    discourse_api: &Arc<dyn DiscourseApi>,
    sync_state: &Arc<SyncState>,
    sync_transaction: &Arc<SyncTransaction>
) -> Result<()> {
    info!("Syncing course {}: {:?} operation", event.entity_id, event.operation);
    
    // Similar to sync_user but for courses
    // This would map Canvas courses to Discourse categories
    
    match event.operation {
        SyncOperation::Create | SyncOperation::Update | SyncOperation::Sync => {
            // Get course data from Canvas
            let canvas_course = canvas_api.get_course(&event.entity_id).await?;
            
            let category_result = discourse_api.get_category_by_custom_field("canvas_course_id", &event.entity_id).await;
            
            match category_result {
                Ok(category) => {
                    // Update existing category
                    let category_data = canvas_to_discourse_category(&canvas_course)?;
                    discourse_api.update_category(&category.id.to_string(), &category_data).await?;
                    info!("Updated category in Discourse: {}", category.id);
                    
                    // Update mapping
                    sync_state.update_sync_status(
                        event.entity_type.as_str(),
                        &event.entity_id,
                        event.source_system.as_str(),
                        Some(&category.id.to_string()),
                        "SYNCED",
                        None
                    ).await?;
                },
                Err(_) => {
                    // Create new category
                    let category_data = canvas_to_discourse_category(&canvas_course)?;
                    let created_category = discourse_api.create_category(&category_data).await?;
                    info!("Created category in Discourse: {}", created_category.id);
                    
                    // Store mapping
                    sync_state.update_sync_status(
                        event.entity_type.as_str(),
                        &event.entity_id,
                        event.source_system.as_str(),
                        Some(&created_category.id.to_string()),
                        "SYNCED",
                        None
                    ).await?;
                }
            }
            
            Ok(())
        },
        SyncOperation::Delete => {
            // Find category in Discourse
            let category = discourse_api.get_category_by_custom_field("canvas_course_id", &event.entity_id).await;
            
            if let Ok(category) = category {
                // Archive category
                discourse_api.archive_category(&category.id.to_string()).await?;
                info!("Archived category in Discourse: {}", category.id);
            }
            
            Ok(())
        }
    }
}

/// Convert Canvas course to Discourse category format
fn canvas_to_discourse_category(canvas_course: &serde_json::Value) -> Result<serde_json::Value> {
    // Extract required fields
    let name = canvas_course.get("name")
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow!("Missing name in Canvas course"))?;
    
    let id = canvas_course.get("id")
        .and_then(|v| v.as_str().or_else(|| v.as_u64().map(|n| n.to_string().as_str())))
        .ok_or_else(|| anyhow!("Missing id in Canvas course"))?;
    
    let course_code = canvas_course.get("course_code")
        .and_then(|v| v.as_str())
        .unwrap_or(name);
    
    // Create slug from course code
    let slug = slugify(course_code);
    
    // Get description from syllabus body or public description
    let description = canvas_course.get("public_description")
        .and_then(|v| v.as_str())
        .or_else(|| canvas_course.get("syllabus_body").and_then(|v| v.as_str()))
        .unwrap_or("");
    
    // Build Discourse category object
    let discourse_category = serde_json::json!({
        "name": name,
        "slug": slug,
        "color": "0088CC",
        "text_color": "FFFFFF",
        "description": description,
        "custom_fields": {
            "canvas_course_id": id,
            "canvas_course_code": course_code
        }
    });
    
    Ok(discourse_category)
}

/// Convert string to URL-safe slug
fn slugify(text: &str) -> String {
    text.to_lowercase()
        .chars()
        .filter_map(|c| {
            if c.is_alphanumeric() {
                Some(c)
            } else if c.is_whitespace() || c == '-' || c == '_' {
                Some('-')
            } else {
                None
            }
        })
        .collect::<String>()
        .replace("--", "-")
}

/// Synchronize an assignment between Canvas and Discourse
async fn sync_assignment(
    event: &SyncEvent,
    canvas_api: &Arc<dyn CanvasApi>,
    discourse_api: &Arc<dyn DiscourseApi>,
    sync_state: &Arc<SyncState>,
    sync_transaction: &Arc<SyncTransaction>
) -> Result<()> {
    info!("Syncing assignment {}: {:?} operation", event.entity_id, event.operation);
    
    // This would be a placeholder implementation
    // Would map Canvas assignments to Discourse topics
    Ok(())
}

/// Synchronize a submission between Canvas and Discourse
async fn sync_submission(
    event: &SyncEvent,
    canvas_api: &Arc<dyn CanvasApi>,
    discourse_api: &Arc<dyn DiscourseApi>,
    sync_state: &Arc<SyncState>,
    sync_transaction: &Arc<SyncTransaction>
) -> Result<()> {
    info!("Syncing submission {}: {:?} operation", event.entity_id, event.operation);
    
    // This would be a placeholder implementation
    // Would map Canvas submissions to Discourse posts
    Ok(())
}

/// Synchronize a discussion between Canvas and Discourse
async fn sync_discussion(
    event: &SyncEvent,
    canvas_api: &Arc<dyn CanvasApi>,
    discourse_api: &Arc<dyn DiscourseApi>,
    sync_state: &Arc<SyncState>,
    sync_transaction: &Arc<SyncTransaction>
) -> Result<()> {
    info!("Syncing discussion {}: {:?} operation", event.entity_id, event.operation);
    
    // This would be a placeholder implementation
    // Would map Canvas discussions to Discourse topics
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;
    use mockall::mock;
    
    // Define mock Canvas API for testing
    mock! {
        CanvasApi {}
        #[async_trait]
        impl CanvasApi for MockCanvasApi {
            async fn get_user(&self, id: &str) -> Result<serde_json::Value>;
            async fn get_course(&self, id: &str) -> Result<serde_json::Value>;
            // Add other methods as needed
        }
    }
    
    // Define mock Discourse API for testing
    mock! {
        DiscourseApi {}
        #[async_trait]
        impl DiscourseApi for MockDiscourseApi {
            async fn get_user_by_external_id(&self, id: &str) -> Result<serde_json::Value>;
            async fn create_user(&self, data: &serde_json::Value) -> Result<serde_json::Value>;
            async fn update_user(&self, id: &str, data: &serde_json::Value) -> Result<serde_json::Value>;
            async fn deactivate_user(&self, id: &str) -> Result<()>;
            
            async fn get_category_by_custom_field(&self, field: &str, value: &str) -> Result<serde_json::Value>;
            async fn create_category(&self, data: &serde_json::Value) -> Result<serde_json::Value>;
            async fn update_category(&self, id: &str, data: &serde_json::Value) -> Result<serde_json::Value>;
            async fn archive_category(&self, id: &str) -> Result<()>;
            // Add other methods as needed
        }
    }
    
    #[tokio::test]
    async fn test_canvas_to_discourse_user() {
        // Test user conversion function
        let canvas_user = serde_json::json!({
            "id": "123",
            "name": "Test User",
            "email": "test@example.com",
            "workflow_state": "active"
        });
        
        let discourse_user = canvas_to_discourse_user(&canvas_user).unwrap();
        
        assert_eq!(discourse_user["name"], "Test User");
        assert_eq!(discourse_user["username"], "test");
        assert_eq!(discourse_user["email"], "test@example.com");
        assert_eq!(discourse_user["active"], true);
        assert_eq!(discourse_user["custom_fields"]["canvas_user_id"], "123");
    }
    
    #[tokio::test]
    async fn test_canvas_to_discourse_category() {
        // Test course conversion function
        let canvas_course = serde_json::json!({
            "id": "456",
            "name": "Introduction to Rust",
            "course_code": "RUST-101",
            "public_description": "Learn Rust programming"
        });
        
        let discourse_category = canvas_to_discourse_category(&canvas_course).unwrap();
        
        assert_eq!(discourse_category["name"], "Introduction to Rust");
        assert_eq!(discourse_category["slug"], "rust-101");
        assert_eq!(discourse_category["description"], "Learn Rust programming");
        assert_eq!(discourse_category["custom_fields"]["canvas_course_id"], "456");
        assert_eq!(discourse_category["custom_fields"]["canvas_course_code"], "RUST-101");
    }
}
