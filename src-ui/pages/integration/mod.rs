pub mod discourse_page;
pub mod canvas_page;
pub mod dashboard;
pub mod settings_page;

// Re-export for easier import
pub use discourse_page::DiscourseIntegrationPage;
pub use canvas_page::CanvasIntegrationPage;
pub use dashboard::IntegrationDashboard;
pub use settings_page::IntegrationSettingsPage;
