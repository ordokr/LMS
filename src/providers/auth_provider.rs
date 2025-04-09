// Add this function to your existing auth provider file

/// Helper function to get the current user ID from auth context
pub fn get_current_user_id() -> Option<String> {
    use_context::<AuthContext>()
        .and_then(|ctx| ctx.user.get())
        .map(|user| user.id.clone())
}