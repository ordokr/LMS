// Module for launching external applications
pub mod quenti_launcher; // Kept for backward compatibility
pub mod ordo_quiz_launcher;

// Re-export the launcher function
pub use ordo_quiz_launcher::launch_ordo_quiz_standalone;
// For backward compatibility
pub use quenti_launcher::launch_quenti_standalone;
