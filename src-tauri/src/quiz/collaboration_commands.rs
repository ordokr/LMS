use super::QuizEngine;
use super::collaboration::{CollaborationRole, QuizCollaborator, CollaborationInvitation, QuizComment};
use uuid::Uuid;
use tauri::State;
use serde_json;

// Collaborator commands

#[tauri::command]
pub async fn add_collaborator(
    quiz_id: String,
    user_id: String,
    role: String,
    engine: State<'_, QuizEngine>,
) -> Result<serde_json::Value, String> {
    let quiz_uuid = Uuid::parse_str(&quiz_id).map_err(|e| e.to_string())?;
    let user_uuid = Uuid::parse_str(&user_id).map_err(|e| e.to_string())?;
    
    let role = match role.as_str() {
        "Owner" => CollaborationRole::Owner,
        "Editor" => CollaborationRole::Editor,
        "Viewer" => CollaborationRole::Viewer,
        _ => return Err("Invalid role".to_string()),
    };
    
    let collaborator = engine.add_collaborator(quiz_uuid, user_uuid, role)
        .await
        .map_err(|e| e.to_string())?;
    
    serde_json::to_value(collaborator).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_collaborator_role(
    quiz_id: String,
    user_id: String,
    role: String,
    updated_by: String,
    engine: State<'_, QuizEngine>,
) -> Result<serde_json::Value, String> {
    let quiz_uuid = Uuid::parse_str(&quiz_id).map_err(|e| e.to_string())?;
    let user_uuid = Uuid::parse_str(&user_id).map_err(|e| e.to_string())?;
    let updated_by_uuid = Uuid::parse_str(&updated_by).map_err(|e| e.to_string())?;
    
    let role = match role.as_str() {
        "Owner" => CollaborationRole::Owner,
        "Editor" => CollaborationRole::Editor,
        "Viewer" => CollaborationRole::Viewer,
        _ => return Err("Invalid role".to_string()),
    };
    
    let collaborator = engine.update_collaborator_role(quiz_uuid, user_uuid, role, updated_by_uuid)
        .await
        .map_err(|e| e.to_string())?;
    
    serde_json::to_value(collaborator).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn remove_collaborator(
    quiz_id: String,
    user_id: String,
    removed_by: String,
    engine: State<'_, QuizEngine>,
) -> Result<(), String> {
    let quiz_uuid = Uuid::parse_str(&quiz_id).map_err(|e| e.to_string())?;
    let user_uuid = Uuid::parse_str(&user_id).map_err(|e| e.to_string())?;
    let removed_by_uuid = Uuid::parse_str(&removed_by).map_err(|e| e.to_string())?;
    
    engine.remove_collaborator(quiz_uuid, user_uuid, removed_by_uuid)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_collaborator(
    quiz_id: String,
    user_id: String,
    engine: State<'_, QuizEngine>,
) -> Result<serde_json::Value, String> {
    let quiz_uuid = Uuid::parse_str(&quiz_id).map_err(|e| e.to_string())?;
    let user_uuid = Uuid::parse_str(&user_id).map_err(|e| e.to_string())?;
    
    let collaborator = engine.get_collaborator(quiz_uuid, user_uuid)
        .await
        .map_err(|e| e.to_string())?;
    
    serde_json::to_value(collaborator).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_collaborators(
    quiz_id: String,
    engine: State<'_, QuizEngine>,
) -> Result<Vec<serde_json::Value>, String> {
    let quiz_uuid = Uuid::parse_str(&quiz_id).map_err(|e| e.to_string())?;
    
    let collaborators = engine.get_collaborators(quiz_uuid)
        .await
        .map_err(|e| e.to_string())?;
    
    let result = collaborators.into_iter()
        .map(|c| serde_json::to_value(c))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    
    Ok(result)
}

// Invitation commands

#[tauri::command]
pub async fn invite_user(
    quiz_id: String,
    inviter_id: String,
    invitee_id: String,
    role: String,
    engine: State<'_, QuizEngine>,
) -> Result<serde_json::Value, String> {
    let quiz_uuid = Uuid::parse_str(&quiz_id).map_err(|e| e.to_string())?;
    let inviter_uuid = Uuid::parse_str(&inviter_id).map_err(|e| e.to_string())?;
    let invitee_uuid = Uuid::parse_str(&invitee_id).map_err(|e| e.to_string())?;
    
    let role = match role.as_str() {
        "Owner" => CollaborationRole::Owner,
        "Editor" => CollaborationRole::Editor,
        "Viewer" => CollaborationRole::Viewer,
        _ => return Err("Invalid role".to_string()),
    };
    
    let invitation = engine.invite_user(quiz_uuid, inviter_uuid, invitee_uuid, role)
        .await
        .map_err(|e| e.to_string())?;
    
    serde_json::to_value(invitation).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn invite_by_email(
    quiz_id: String,
    inviter_id: String,
    email: String,
    role: String,
    engine: State<'_, QuizEngine>,
) -> Result<serde_json::Value, String> {
    let quiz_uuid = Uuid::parse_str(&quiz_id).map_err(|e| e.to_string())?;
    let inviter_uuid = Uuid::parse_str(&inviter_id).map_err(|e| e.to_string())?;
    
    let role = match role.as_str() {
        "Owner" => CollaborationRole::Owner,
        "Editor" => CollaborationRole::Editor,
        "Viewer" => CollaborationRole::Viewer,
        _ => return Err("Invalid role".to_string()),
    };
    
    let invitation = engine.invite_by_email(quiz_uuid, inviter_uuid, email, role)
        .await
        .map_err(|e| e.to_string())?;
    
    serde_json::to_value(invitation).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn accept_invitation(
    invitation_id: String,
    user_id: String,
    engine: State<'_, QuizEngine>,
) -> Result<serde_json::Value, String> {
    let invitation_uuid = Uuid::parse_str(&invitation_id).map_err(|e| e.to_string())?;
    let user_uuid = Uuid::parse_str(&user_id).map_err(|e| e.to_string())?;
    
    let collaborator = engine.accept_invitation(invitation_uuid, user_uuid)
        .await
        .map_err(|e| e.to_string())?;
    
    serde_json::to_value(collaborator).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn decline_invitation(
    invitation_id: String,
    user_id: String,
    engine: State<'_, QuizEngine>,
) -> Result<(), String> {
    let invitation_uuid = Uuid::parse_str(&invitation_id).map_err(|e| e.to_string())?;
    let user_uuid = Uuid::parse_str(&user_id).map_err(|e| e.to_string())?;
    
    engine.decline_invitation(invitation_uuid, user_uuid)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn cancel_invitation(
    invitation_id: String,
    user_id: String,
    engine: State<'_, QuizEngine>,
) -> Result<(), String> {
    let invitation_uuid = Uuid::parse_str(&invitation_id).map_err(|e| e.to_string())?;
    let user_uuid = Uuid::parse_str(&user_id).map_err(|e| e.to_string())?;
    
    engine.cancel_invitation(invitation_uuid, user_uuid)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_invitation(
    invitation_id: String,
    engine: State<'_, QuizEngine>,
) -> Result<serde_json::Value, String> {
    let invitation_uuid = Uuid::parse_str(&invitation_id).map_err(|e| e.to_string())?;
    
    let invitation = engine.get_invitation(invitation_uuid)
        .await
        .map_err(|e| e.to_string())?;
    
    serde_json::to_value(invitation).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_invitations_for_quiz(
    quiz_id: String,
    engine: State<'_, QuizEngine>,
) -> Result<Vec<serde_json::Value>, String> {
    let quiz_uuid = Uuid::parse_str(&quiz_id).map_err(|e| e.to_string())?;
    
    let invitations = engine.get_invitations_for_quiz(quiz_uuid)
        .await
        .map_err(|e| e.to_string())?;
    
    let result = invitations.into_iter()
        .map(|i| serde_json::to_value(i))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    
    Ok(result)
}

#[tauri::command]
pub async fn get_pending_invitations_for_user(
    user_id: String,
    engine: State<'_, QuizEngine>,
) -> Result<Vec<serde_json::Value>, String> {
    let user_uuid = Uuid::parse_str(&user_id).map_err(|e| e.to_string())?;
    
    let invitations = engine.get_pending_invitations_for_user(user_uuid)
        .await
        .map_err(|e| e.to_string())?;
    
    let result = invitations.into_iter()
        .map(|i| serde_json::to_value(i))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    
    Ok(result)
}

// Comment commands

#[tauri::command]
pub async fn add_comment(
    quiz_id: String,
    user_id: String,
    content: String,
    engine: State<'_, QuizEngine>,
) -> Result<serde_json::Value, String> {
    let quiz_uuid = Uuid::parse_str(&quiz_id).map_err(|e| e.to_string())?;
    let user_uuid = Uuid::parse_str(&user_id).map_err(|e| e.to_string())?;
    
    let comment = engine.add_comment(quiz_uuid, user_uuid, content)
        .await
        .map_err(|e| e.to_string())?;
    
    serde_json::to_value(comment).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn add_question_comment(
    quiz_id: String,
    question_id: String,
    user_id: String,
    content: String,
    engine: State<'_, QuizEngine>,
) -> Result<serde_json::Value, String> {
    let quiz_uuid = Uuid::parse_str(&quiz_id).map_err(|e| e.to_string())?;
    let question_uuid = Uuid::parse_str(&question_id).map_err(|e| e.to_string())?;
    let user_uuid = Uuid::parse_str(&user_id).map_err(|e| e.to_string())?;
    
    let comment = engine.add_question_comment(quiz_uuid, question_uuid, user_uuid, content)
        .await
        .map_err(|e| e.to_string())?;
    
    serde_json::to_value(comment).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn add_reply(
    parent_id: String,
    user_id: String,
    content: String,
    engine: State<'_, QuizEngine>,
) -> Result<serde_json::Value, String> {
    let parent_uuid = Uuid::parse_str(&parent_id).map_err(|e| e.to_string())?;
    let user_uuid = Uuid::parse_str(&user_id).map_err(|e| e.to_string())?;
    
    let comment = engine.add_reply(parent_uuid, user_uuid, content)
        .await
        .map_err(|e| e.to_string())?;
    
    serde_json::to_value(comment).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_comment(
    comment_id: String,
    user_id: String,
    content: String,
    engine: State<'_, QuizEngine>,
) -> Result<serde_json::Value, String> {
    let comment_uuid = Uuid::parse_str(&comment_id).map_err(|e| e.to_string())?;
    let user_uuid = Uuid::parse_str(&user_id).map_err(|e| e.to_string())?;
    
    let comment = engine.update_comment(comment_uuid, user_uuid, content)
        .await
        .map_err(|e| e.to_string())?;
    
    serde_json::to_value(comment).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_comment(
    comment_id: String,
    user_id: String,
    engine: State<'_, QuizEngine>,
) -> Result<(), String> {
    let comment_uuid = Uuid::parse_str(&comment_id).map_err(|e| e.to_string())?;
    let user_uuid = Uuid::parse_str(&user_id).map_err(|e| e.to_string())?;
    
    engine.delete_comment(comment_uuid, user_uuid)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_comment(
    comment_id: String,
    engine: State<'_, QuizEngine>,
) -> Result<serde_json::Value, String> {
    let comment_uuid = Uuid::parse_str(&comment_id).map_err(|e| e.to_string())?;
    
    let comment = engine.get_comment(comment_uuid)
        .await
        .map_err(|e| e.to_string())?;
    
    serde_json::to_value(comment).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_comments_for_quiz(
    quiz_id: String,
    engine: State<'_, QuizEngine>,
) -> Result<Vec<serde_json::Value>, String> {
    let quiz_uuid = Uuid::parse_str(&quiz_id).map_err(|e| e.to_string())?;
    
    let comments = engine.get_comments_for_quiz(quiz_uuid)
        .await
        .map_err(|e| e.to_string())?;
    
    let result = comments.into_iter()
        .map(|c| serde_json::to_value(c))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    
    Ok(result)
}

#[tauri::command]
pub async fn get_comments_for_question(
    question_id: String,
    engine: State<'_, QuizEngine>,
) -> Result<Vec<serde_json::Value>, String> {
    let question_uuid = Uuid::parse_str(&question_id).map_err(|e| e.to_string())?;
    
    let comments = engine.get_comments_for_question(question_uuid)
        .await
        .map_err(|e| e.to_string())?;
    
    let result = comments.into_iter()
        .map(|c| serde_json::to_value(c))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    
    Ok(result)
}

#[tauri::command]
pub async fn get_replies(
    comment_id: String,
    engine: State<'_, QuizEngine>,
) -> Result<Vec<serde_json::Value>, String> {
    let comment_uuid = Uuid::parse_str(&comment_id).map_err(|e| e.to_string())?;
    
    let replies = engine.get_replies(comment_uuid)
        .await
        .map_err(|e| e.to_string())?;
    
    let result = replies.into_iter()
        .map(|r| serde_json::to_value(r))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    
    Ok(result)
}
