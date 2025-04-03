use crate::models::admin::*;
use crate::models::user::{User, UserRole};
use crate::models::forum::Report;
use crate::error::AppError;
use gloo_net::http::{Request, RequestBuilder};
use chrono::{DateTime, Utc};

pub struct AdminService;

impl AdminService {
    // Dashboard
    pub async fn get_stats() -> Result<AdminStats, AppError> {
        let resp = Request::get("/api/admin/stats")
            .send()
            .await
            .map_err(|e| AppError::NetworkError(e.to_string()))?;

        if resp.status() != 200 {
            let error_text = resp.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(AppError::ApiError(error_text));
        }

        let stats = resp.json::<AdminStats>().await
            .map_err(|e| AppError::DeserializationError(e.to_string()))?;
        
        Ok(stats)
    }

    pub async fn get_activity_data(time_range: &str) -> Result<ActivityData, AppError> {
        let resp = Request::get(&format!("/api/admin/activity?range={}", time_range))
            .send()
            .await
            .map_err(|e| AppError::NetworkError(e.to_string()))?;

        if resp.status() != 200 {
            let error_text = resp.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(AppError::ApiError(error_text));
        }

        let data = resp.json::<ActivityData>().await
            .map_err(|e| AppError::DeserializationError(e.to_string()))?;
        
        Ok(data)
    }

    // User Management
    pub async fn get_users(page: usize, limit: usize, search: &str) -> Result<UserManagementPage, AppError> {
        let resp = Request::get(&format!("/api/admin/users?page={}&limit={}&q={}", page, limit, search))
            .send()
            .await
            .map_err(|e| AppError::NetworkError(e.to_string()))?;

        if resp.status() != 200 {
            let error_text = resp.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(AppError::ApiError(error_text));
        }

        let users_page = resp.json::<UserManagementPage>().await
            .map_err(|e| AppError::DeserializationError(e.to_string()))?;
        
        Ok(users_page)
    }

    pub async fn update_user_role(user_id: i64, role: UserRole) -> Result<(), AppError> {
        let resp = Request::put(&format!("/api/admin/users/{}/role", user_id))
            .json(&role)
            .map_err(|e| AppError::SerializationError(e.to_string()))?
            .send()
            .await
            .map_err(|e| AppError::NetworkError(e.to_string()))?;

        if resp.status() != 200 {
            let error_text = resp.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(AppError::ApiError(error_text));
        }
        
        Ok(())
    }

    pub async fn ban_user(user_id: i64) -> Result<(), AppError> {
        let resp = Request::put(&format!("/api/admin/users/{}/ban", user_id))
            .send()
            .await
            .map_err(|e| AppError::NetworkError(e.to_string()))?;

        if resp.status() != 200 {
            let error_text = resp.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(AppError::ApiError(error_text));
        }
        
        Ok(())
    }

    pub async fn unban_user(user_id: i64) -> Result<(), AppError> {
        let resp = Request::put(&format!("/api/admin/users/{}/unban", user_id))
            .send()
            .await
            .map_err(|e| AppError::NetworkError(e.to_string()))?;

        if resp.status() != 200 {
            let error_text = resp.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(AppError::ApiError(error_text));
        }
        
        Ok(())
    }

    // Content Moderation
    pub async fn get_reports(status: &str) -> Result<Vec<Report>, AppError> {
        let resp = Request::get(&format!("/api/admin/reports?status={}", status))
            .send()
            .await
            .map_err(|e| AppError::NetworkError(e.to_string()))?;

        if resp.status() != 200 {
            let error_text = resp.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(AppError::ApiError(error_text));
        }

        let reports = resp.json::<Vec<Report>>().await
            .map_err(|e| AppError::DeserializationError(e.to_string()))?;
        
        Ok(reports)
    }

    pub async fn approve_report(report_id: i64, content_type: &str, content_id: i64) -> Result<(), AppError> {
        let resp = Request::put(&format!("/api/admin/reports/{}/approve", report_id))
            .json(&serde_json::json!({
                "content_type": content_type,
                "content_id": content_id
            }))
            .map_err(|e| AppError::SerializationError(e.to_string()))?
            .send()
            .await
            .map_err(|e| AppError::NetworkError(e.to_string()))?;

        if resp.status() != 200 {
            let error_text = resp.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(AppError::ApiError(error_text));
        }
        
        Ok(())
    }

    pub async fn dismiss_report(report_id: i64) -> Result<(), AppError> {
        let resp = Request::put(&format!("/api/admin/reports/{}/dismiss", report_id))
            .send()
            .await
            .map_err(|e| AppError::NetworkError(e.to_string()))?;

        if resp.status() != 200 {
            let error_text = resp.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(AppError::ApiError(error_text));
        }
        
        Ok(())
    }

    // Reported content
    pub async fn get_reported_content(status: ReportStatus) -> Result<Vec<ReportedContent>, AppError> {
        let status_str = match status {
            ReportStatus::Pending => "pending",
            ReportStatus::Resolved => "resolved",
            ReportStatus::Dismissed => "dismissed",
        };

        let resp = Request::get(&format!("/api/admin/reported-content?status={}", status_str))
            .send()
            .await
            .map_err(|e| AppError::NetworkError(e.to_string()))?;

        if resp.status() != 200 {
            let error_text = resp.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(AppError::ApiError(error_text));
        }

        let content = resp.json::<Vec<ReportedContent>>().await
            .map_err(|e| AppError::DeserializationError(e.to_string()))?;
        
        Ok(content)
    }

    pub async fn process_report(report_id: i64, decision: ReportDecision, note: String) -> Result<(), AppError> {
        let resp = Request::put(&format!("/api/admin/reported-content/{}/process", report_id))
            .json(&serde_json::json!({
                "decision": decision,
                "note": note
            }))
            .map_err(|e| AppError::SerializationError(e.to_string()))?
            .send()
            .await
            .map_err(|e| AppError::NetworkError(e.to_string()))?;

        if resp.status() != 200 {
            let error_text = resp.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(AppError::ApiError(error_text));
        }
        
        Ok(())
    }

    // Activity logs
    pub async fn get_activity_logs(
        page: usize, 
        limit: usize,
        activity_type: Option<ActivityType>,
        user: Option<String>,
        date_from: Option<String>,
        date_to: Option<String>
    ) -> Result<ActivityLogPage, AppError> {
        let mut url = format!("/api/admin/logs?page={}&limit={}", page, limit);
        
        if let Some(t) = activity_type {
            url.push_str(&format!("&type={:?}", t));
        }
        
        if let Some(u) = user {
            url.push_str(&format!("&user={}", u));
        }
        
        if let Some(from) = date_from {
            url.push_str(&format!("&from={}", from));
        }
        
        if let Some(to) = date_to {
            url.push_str(&format!("&to={}", to));
        }

        let resp = Request::get(&url)
            .send()
            .await
            .map_err(|e| AppError::NetworkError(e.to_string()))?;

        if resp.status() != 200 {
            let error_text = resp.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(AppError::ApiError(error_text));
        }

        let logs = resp.json::<ActivityLogPage>().await
            .map_err(|e| AppError::DeserializationError(e.to_string()))?;
        
        Ok(logs)
    }
    
    // Forum settings
    pub async fn get_forum_settings() -> Result<ForumSettings, AppError> {
        let resp = Request::get("/api/admin/settings")
            .send()
            .await
            .map_err(|e| AppError::NetworkError(e.to_string()))?;

        if resp.status() != 200 {
            let error_text = resp.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(AppError::ApiError(error_text));
        }

        let settings = resp.json::<ForumSettings>().await
            .map_err(|e| AppError::DeserializationError(e.to_string()))?;
        
        Ok(settings)
    }

    pub async fn update_forum_settings(settings: ForumSettings) -> Result<ForumSettings, AppError> {
        let resp = Request::put("/api/admin/settings")
            .json(&settings)
            .map_err(|e| AppError::SerializationError(e.to_string()))?
            .send()
            .await
            .map_err(|e| AppError::NetworkError(e.to_string()))?;

        if resp.status() != 200 {
            let error_text = resp.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(AppError::ApiError(error_text));
        }

        let updated_settings = resp.json::<ForumSettings>().await
            .map_err(|e| AppError::DeserializationError(e.to_string()))?;
        
        Ok(updated_settings)
    }

    // Notification settings
    pub async fn get_notification_settings() -> Result<NotificationSettings, AppError> {
        let resp = Request::get("/api/admin/notifications/settings")
            .send()
            .await
            .map_err(|e| AppError::NetworkError(e.to_string()))?;

        if resp.status() != 200 {
            let error_text = resp.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(AppError::ApiError(error_text));
        }

        let settings = resp.json::<NotificationSettings>().await
            .map_err(|e| AppError::DeserializationError(e.to_string()))?;
        
        Ok(settings)
    }

    pub async fn update_notification_settings(settings: NotificationSettings) -> Result<NotificationSettings, AppError> {
        let resp = Request::put("/api/admin/notifications/settings")
            .json(&settings)
            .map_err(|e| AppError::SerializationError(e.to_string()))?
            .send()
            .await
            .map_err(|e| AppError::NetworkError(e.to_string()))?;

        if resp.status() != 200 {
            let error_text = resp.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(AppError::ApiError(error_text));
        }

        let updated_settings = resp.json::<NotificationSettings>().await
            .map_err(|e| AppError::DeserializationError(e.to_string()))?;
        
        Ok(updated_settings)
    }

    pub async fn send_test_email() -> Result<(), AppError> {
        let resp = Request::post("/api/admin/notifications/test-email")
            .send()
            .await
            .map_err(|e| AppError::NetworkError(e.to_string()))?;

        if resp.status() != 200 {
            let error_text = resp.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(AppError::ApiError(error_text));
        }
        
        Ok(())
    }

    // User Groups
    pub async fn get_user_groups() -> Result<Vec<UserGroup>, AppError> {
        let resp = Request::get("/api/admin/groups")
            .send()
            .await
            .map_err(|e| AppError::NetworkError(e.to_string()))?;

        if resp.status() != 200 {
            let error_text = resp.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(AppError::ApiError(error_text));
        }

        let groups = resp.json::<Vec<UserGroup>>().await
            .map_err(|e| AppError::DeserializationError(e.to_string()))?;
        
        Ok(groups)
    }

    pub async fn get_user_group(group_id: i64) -> Result<UserGroup, AppError> {
        let resp = Request::get(&format!("/api/admin/groups/{}", group_id))
            .send()
            .await
            .map_err(|e| AppError::NetworkError(e.to_string()))?;

        if resp.status() != 200 {
            let error_text = resp.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(AppError::ApiError(error_text));
        }

        let group = resp.json::<UserGroup>().await
            .map_err(|e| AppError::DeserializationError(e.to_string()))?;
        
        Ok(group)
    }

    pub async fn create_user_group(group: UserGroupCreate) -> Result<UserGroup, AppError> {
        let resp = Request::post("/api/admin/groups")
            .json(&group)
            .map_err(|e| AppError::SerializationError(e.to_string()))?
            .send()
            .await
            .map_err(|e| AppError::NetworkError(e.to_string()))?;

        if resp.status() != 201 {
            let error_text = resp.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(AppError::ApiError(error_text));
        }

        let created_group = resp.json::<UserGroup>().await
            .map_err(|e| AppError::DeserializationError(e.to_string()))?;
        
        Ok(created_group)
    }

    pub async fn update_user_group(group_id: i64, group: UserGroupUpdate) -> Result<UserGroup, AppError> {
        let resp = Request::put(&format!("/api/admin/groups/{}", group_id))
            .json(&group)
            .map_err(|e| AppError::SerializationError(e.to_string()))?
            .send()
            .await
            .map_err(|e| AppError::NetworkError(e.to_string()))?;

        if resp.status() != 200 {
            let error_text = resp.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(AppError::ApiError(error_text));
        }

        let updated_group = resp.json::<UserGroup>().await
            .map_err(|e| AppError::DeserializationError(e.to_string()))?;
        
        Ok(updated_group)
    }

    pub async fn delete_user_group(group_id: i64) -> Result<(), AppError> {
        let resp = Request::delete(&format!("/api/admin/groups/{}", group_id))
            .send()
            .await
            .map_err(|e| AppError::NetworkError(e.to_string()))?;

        if resp.status() != 200 {
            let error_text = resp.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(AppError::ApiError(error_text));
        }
        
        Ok(())
    }

    pub async fn get_group_members(group_id: i64) -> Result<Vec<GroupMember>, AppError> {
        let resp = Request::get(&format!("/api/admin/groups/{}/members", group_id))
            .send()
            .await
            .map_err(|e| AppError::NetworkError(e.to_string()))?;

        if resp.status() != 200 {
            let error_text = resp.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(AppError::ApiError(error_text));
        }

        let members = resp.json::<Vec<GroupMember>>().await
            .map_err(|e| AppError::DeserializationError(e.to_string()))?;
        
        Ok(members)
    }

    pub async fn add_user_to_group(group_id: i64, user_id: i64) -> Result<(), AppError> {
        let resp = Request::post(&format!("/api/admin/groups/{}/members", group_id))
            .json(&serde_json::json!({ "user_id": user_id }))
            .map_err(|e| AppError::SerializationError(e.to_string()))?
            .send()
            .await
            .map_err(|e| AppError::NetworkError(e.to_string()))?;

        if resp.status() != 200 {
            let error_text = resp.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(AppError::ApiError(error_text));
        }
        
        Ok(())
    }

    pub async fn remove_user_from_group(group_id: i64, user_id: i64) -> Result<(), AppError> {
        let resp = Request::delete(&format!("/api/admin/groups/{}/members/{}", group_id, user_id))
            .send()
            .await
            .map_err(|e| AppError::NetworkError(e.to_string()))?;

        if resp.status() != 200 {
            let error_text = resp.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(AppError::ApiError(error_text));
        }
        
        Ok(())
    }

    pub async fn search_users(query: String) -> Result<Vec<crate::models::user::User>, AppError> {
        let resp = Request::get(&format!("/api/admin/users/search?q={}", query))
            .send()
            .await
            .map_err(|e| AppError::NetworkError(e.to_string()))?;

        if resp.status() != 200 {
            let error_text = resp.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(AppError::ApiError(error_text));
        }

        let users = resp.json::<Vec<crate::models::user::User>>().await
            .map_err(|e| AppError::DeserializationError(e.to_string()))?;
        
        Ok(users)
    }

    // Site Customization
    pub async fn get_site_customization() -> Result<SiteCustomization, AppError> {
        let resp = Request::get("/api/admin/customization")
            .send()
            .await
            .map_err(|e| AppError::NetworkError(e.to_string()))?;

        if resp.status() != 200 {
            let error_text = resp.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(AppError::ApiError(error_text));
        }

        let customization = resp.json::<SiteCustomization>().await
            .map_err(|e| AppError::DeserializationError(e.to_string()))?;
        
        Ok(customization)
    }

    pub async fn update_site_customization(customization: SiteCustomization) -> Result<SiteCustomization, AppError> {
        let resp = Request::put("/api/admin/customization")
            .json(&customization)
            .map_err(|e| AppError::SerializationError(e.to_string()))?
            .send()
            .await
            .map_err(|e| AppError::NetworkError(e.to_string()))?;

        if resp.status() != 200 {
            let error_text = resp.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(AppError::ApiError(error_text));
        }

        let updated_customization = resp.json::<SiteCustomization>().await
            .map_err(|e| AppError::DeserializationError(e.to_string()))?;
        
        Ok(updated_customization)
    }

    pub async fn reset_site_customization() -> Result<SiteCustomization, AppError> {
        let resp = Request::post("/api/admin/customization/reset")
            .send()
            .await
            .map_err(|e| AppError::NetworkError(e.to_string()))?;

        if resp.status() != 200 {
            let error_text = resp.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(AppError::ApiError(error_text));
        }

        let default_customization = resp.json::<SiteCustomization>().await
            .map_err(|e| AppError::DeserializationError(e.to_string()))?;
        
        Ok(default_customization)
    }

    // Data Export/Import
    pub async fn export_data(options: ExportOptions) -> Result<String, AppError> {
        let resp = Request::post("/api/admin/export")
            .json(&options)
            .map_err(|e| AppError::SerializationError(e.to_string()))?
            .send()
            .await
            .map_err(|e| AppError::NetworkError(e.to_string()))?;

        if resp.status() != 200 {
            let error_text = resp.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(AppError::ApiError(error_text));
        }

        let result = resp.json::<serde_json::Value>().await
            .map_err(|e| AppError::DeserializationError(e.to_string()))?;
        
        let download_url = result.get("downloadUrl")
            .and_then(|v| v.as_str())
            .ok_or_else(|| AppError::ApiError("Invalid response format".to_string()))?;
        
        Ok(download_url.to_string())
    }

    pub async fn import_data(form_data: web_sys::FormData) -> Result<ImportStats, AppError> {
        let resp = Request::post("/api/admin/import")
            .body(form_data)
            .map_err(|e| AppError::SerializationError(e.to_string()))?
            .send()
            .await
            .map_err(|e| AppError::NetworkError(e.to_string()))?;

        if resp.status() != 200 {
            let error_text = resp.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(AppError::ApiError(error_text));
        }

        let import_stats = resp.json::<ImportStats>().await
            .map_err(|e| AppError::DeserializationError(e.to_string()))?;
        
        Ok(import_stats)
    }

    pub async fn get_backup_history() -> Result<Vec<BackupInfo>, AppError> {
        let resp = Request::get("/api/admin/backups")
            .send()
            .await
            .map_err(|e| AppError::NetworkError(e.to_string()))?;

        if resp.status() != 200 {
            let error_text = resp.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(AppError::ApiError(error_text));
        }

        let backups = resp.json::<Vec<BackupInfo>>().await
            .map_err(|e| AppError::DeserializationError(e.to_string()))?;
        
        Ok(backups)
    }

    pub async fn download_backup(backup_id: String) -> Result<String, AppError> {
        let resp = Request::get(&format!("/api/admin/backups/{}/download", backup_id))
            .send()
            .await
            .map_err(|e| AppError::NetworkError(e.to_string()))?;

        if resp.status() != 200 {
            let error_text = resp.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(AppError::ApiError(error_text));
        }

        let result = resp.json::<serde_json::Value>().await
            .map_err(|e| AppError::DeserializationError(e.to_string()))?;
        
        let download_url = result.get("downloadUrl")
            .and_then(|v| v.as_str())
            .ok_or_else(|| AppError::ApiError("Invalid response format".to_string()))?;
        
        Ok(download_url.to_string())
    }

    pub async fn delete_backup(backup_id: String) -> Result<(), AppError> {
        let resp = Request::delete(&format!("/api/admin/backups/{}", backup_id))
            .send()
            .await
            .map_err(|e| AppError::NetworkError(e.to_string()))?;

        if resp.status() != 200 {
            let error_text = resp.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(AppError::ApiError(error_text));
        }
        
        Ok(())
    }
}