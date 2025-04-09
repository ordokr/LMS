fn main() {
    tauri::Builder::default()
        // Add our forum API commands
        .invoke_handler(tauri::generate_handler![
            // Existing commands
            // ...
            
            // Forum commands
            crate::api::forum_client::get_categories,
            crate::api::forum_client::get_category,
            crate::api::forum_client::get_topics,
            crate::api::forum_client::get_topics_by_category,
            crate::api::forum_client::get_topic,
            crate::api::forum_client::create_topic,
            crate::api::forum_client::update_topic,
            crate::api::forum_client::delete_topic,
            crate::api::forum_client::create_category,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}