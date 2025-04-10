use crate::auth::jwt_service::{JwtService, Claims};
use crate::models::unified::Notification;
use crate::api::canvas_client::CanvasClient;
use crate::api::discourse_client::DiscourseClient;
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use tauri::{command, State};
use log::{info, error};
use chrono::{DateTime, Utc};

/// State structure for notification commands
pub struct NotificationState {
    pub jwt_service: Arc<JwtService>,
    pub canvas_client: Arc<CanvasClient>,
    pub discourse_client: Arc<DiscourseClient>,
}

impl Default for NotificationState {
    fn default() -> Self {
        // These values should be loaded from configuration in a real application
        let jwt_service = Arc::new(JwtService::new("jwt_secret_placeholder", 3600, 86400));
        let canvas_client = Arc::new(CanvasClient::new("https://canvas.example.com/api", "canvas_api_key_placeholder"));
        let discourse_client = Arc::new(DiscourseClient::new(
            "https://discourse.example.com",
            "discourse_api_key_placeholder",
            "system",
        ));
        
        Self {
            jwt_service,
            canvas_client,
            discourse_client,
        }
    }
}

/// Options for retrieving notifications
#[derive(Debug, Deserialize)]
pub struct NotificationOptions {
    pub read: Option<bool>,
    pub notification_type: Option<String>,
    pub since: Option<DateTime<Utc>>,
    pub limit: Option<u32>,
}

/// Response for notifications
#[derive(Debug, Serialize)]
pub struct NotificationsResponse {
    pub notifications: Vec<Notification>,
    pub unread_count: usize,
    pub error: Option<String>,
}

/// Mark as read request
#[derive(Debug, Deserialize)]
pub struct MarkAsReadRequest {
    pub source: String,
}

/// Mark as read response
#[derive(Debug, Serialize)]
pub struct MarkAsReadResponse {
    pub notification: Option<Notification>,
    pub success: bool,
    pub error: Option<String>,
}

/// Retrieve a list of notifications for the authenticated user
#[command]
pub async fn get_user_notifications(
    state: State<'_, NotificationState>,
    token: String,
    options: Option<NotificationOptions>,
) -> Result<NotificationsResponse, String> {
    info!("Retrieving user notifications");
    
    // Verify the JWT token and extract the user ID
    let claims = match state.jwt_service.verify_token(&token) {
        Ok(claims) => claims,
        Err(e) => {
            error!("JWT verification failed: {}", e);
            return Ok(NotificationsResponse {
                notifications: Vec::new(),
                unread_count: 0,
                error: Some("Authentication failed".to_string()),
            });
        }
    };
    
    let user_id = claims.sub.clone();
    
    // Parse options or use defaults
    let options = options.unwrap_or(NotificationOptions {
        read: None,
        notification_type: None,
        since: None,
        limit: Some(20),
    });
    
    // Track all notifications from different sources
    let mut all_notifications = Vec::new();
    let mut total_unread = 0;
    
    // Get Canvas notifications if the user has a Canvas ID
    if let Some(canvas_id) = claims.canvas_id {
        match state.canvas_client.get_user_notifications(&canvas_id).await {
            Ok(canvas_notifications) => {
                // Count unread notifications
                let unread_count = canvas_notifications.iter()
                    .filter(|n| !n.read)
                    .count();
                    
                total_unread += unread_count;
                all_notifications.extend(canvas_notifications);
            },
            Err(e) => {
                error!("Error fetching Canvas notifications: {}", e);
                // We continue to fetch other notifications even if Canvas fails
            }
        }
    }
    
    // Get Discourse notifications if the user has a Discourse ID
    if let Some(discourse_id) = claims.discourse_id {
        match state.discourse_client.get_user_notifications(&discourse_id).await {
            Ok(discourse_notifications) => {
                // Count unread notifications
                let unread_count = discourse_notifications.iter()
                    .filter(|n| !n.read)
                    .count();
                    
                total_unread += unread_count;
                all_notifications.extend(discourse_notifications);
            },
            Err(e) => {
                error!("Error fetching Discourse notifications: {}", e);
                // We continue even if Discourse fails
            }
        }
    }
    
    // Sort notifications by created date (most recent first)
    all_notifications.sort_by(|a, b| {
        b.created_at.cmp(&a.created_at)
    });
    
    // Apply filters based on options
    let filtered_notifications = all_notifications.into_iter()
        .filter(|n| {
            // Filter by read status if specified
            if let Some(read) = options.read {
                if n.read != read {
                    return false;
                }
            }
            
            // Filter by notification type if specified
            if let Some(ref notification_type) = options.notification_type {
                if n.notification_type != *notification_type {
                    return false;
                }
            }
            
            // Filter by date if specified
            if let Some(since) = options.since {
                if n.created_at < since {
                    return false;
                }
            }
            
            true
        })
        .collect::<Vec<_>>();
    
    // Apply limit if specified
    let limit = options.limit.unwrap_or(20) as usize;
    let limited_notifications = if filtered_notifications.len() > limit {
        filtered_notifications[0..limit].to_vec()
    } else {
        filtered_notifications
    };
    
    info!("Retrieved {} notifications, {} unread", limited_notifications.len(), total_unread);
    
    Ok(NotificationsResponse {
        notifications: limited_notifications,
        unread_count: total_unread,
        error: None,
    })
}

/// Mark a notification as read
#[command]
pub async fn mark_notification_as_read(
    state: State<'_, NotificationState>,
    token: String,
    notification_id: String,
    request: MarkAsReadRequest,
) -> Result<MarkAsReadResponse, String> {
    info!("Marking notification {} as read from source {}", notification_id, request.source);
    
    // Verify the JWT token
    match state.jwt_service.verify_token(&token) {
        Ok(_) => {
            // Critical: Validate source parameter
            if request.source.is_empty() {
                error!("Source parameter is required");
                return Ok(MarkAsReadResponse {
                    notification: None,
                    success: false,
                    error: Some("Source parameter is required".to_string()),
                });
            }
            
            // Mark notification as read based on source
            match request.source.as_str() {
                "canvas" => {
                    match state.canvas_client.mark_notification_as_read(&notification_id).await {
                        Ok(notification) => {
                            info!("Successfully marked Canvas notification as read");
                            Ok(MarkAsReadResponse {
                                notification: Some(notification),
                                success: true,
                                error: None,
                            })
                        },
                        Err(e) => {
                            error!("Error marking Canvas notification as read: {}", e);
                            Ok(MarkAsReadResponse {
                                notification: None,
                                success: false,
                                error: Some(format!("Error: {}", e)),
                            })
                        }
                    }
                },
                "discourse" => {
                    match state.discourse_client.mark_notification_as_read(&notification_id).await {
                        Ok(notification) => {
                            info!("Successfully marked Discourse notification as read");
                            Ok(MarkAsReadResponse {
                                notification: Some(notification),
                                success: true,
                                error: None,
                            })
                        },
                        Err(e) => {
                            error!("Error marking Discourse notification as read: {}", e);
                            Ok(MarkAsReadResponse {
                                notification: None,
                                success: false,
                                error: Some(format!("Error: {}", e)),
                            })
                        }
                    }
                },
                _ => {
                    error!("Invalid source: {}", request.source);
                    Ok(MarkAsReadResponse {
                        notification: None,
                        success: false,
                        error: Some("Invalid notification source".to_string()),
                    })
                }
            }
        },
        Err(e) => {
            error!("JWT verification failed: {}", e);
            Ok(MarkAsReadResponse {
                notification: None,
                success: false,
                error: Some("Authentication failed".to_string()),
            })
        }
    }
}

/// Create a new notification
#[command]
pub async fn create_notification(
    state: State<'_, NotificationState>,
    token: String,
    source: String,
    notification_data: serde_json::Value,
) -> Result<MarkAsReadResponse, String> {
    info!("Creating notification in source {}", source);
    
    // Verify the JWT token
    match state.jwt_service.verify_token(&token) {
        Ok(_) => {
            // Create notification based on source
            match source.as_str() {
                "canvas" => {
                    match state.canvas_client.create_notification(notification_data).await {
                        Ok(notification) => {
                            info!("Successfully created Canvas notification");
                            Ok(MarkAsReadResponse {
                                notification: Some(notification),
                                success: true,
                                error: None,
                            })
                        },
                        Err(e) => {
                            error!("Error creating Canvas notification: {}", e);
                            Ok(MarkAsReadResponse {
                                notification: None,
                                success: false,
                                error: Some(format!("Error: {}", e)),
                            })
                        }
                    }
                },
                "discourse" => {
                    match state.discourse_client.create_notification(notification_data).await {
                        Ok(notification) => {
                            info!("Successfully created Discourse notification");
                            Ok(MarkAsReadResponse {
                                notification: Some(notification),
                                success: true,
                                error: None,
                            })
                        },
                        Err(e) => {
                            error!("Error creating Discourse notification: {}", e);
                            Ok(MarkAsReadResponse {
                                notification: None,
                                success: false,
                                error: Some(format!("Error: {}", e)),
                            })
                        }
                    }
                },
                _ => {
                    error!("Invalid source: {}", source);
                    Ok(MarkAsReadResponse {
                        notification: None,
                        success: false,
                        error: Some("Invalid notification source".to_string()),
                    })
                }
            }
        },
        Err(e) => {
            error!("JWT verification failed: {}", e);
            Ok(MarkAsReadResponse {
                notification: None,
                success: false,
                error: Some("Authentication failed".to_string()),
            })
        }
    }
}

/// Register the notification commands with Tauri
pub fn register_commands(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
