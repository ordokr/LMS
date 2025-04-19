use std::path::PathBuf;
use std::fs;
use std::process::Command;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Starting Ordo Quiz Module...");

    // Create test directories
    let data_dir = PathBuf::from("ordo_quiz_data");
    if !data_dir.exists() {
        fs::create_dir_all(&data_dir)?;
    }
    
    let migrations_dir = PathBuf::from("migrations");
    if !migrations_dir.exists() {
        fs::create_dir_all(&migrations_dir)?;
    }
    
    // Initialize database
    let db_path = "ordo_quiz.db";
    if fs::metadata(db_path).is_ok() {
        println!("Using existing database: {}", db_path);
    } else {
        println!("Creating new database: {}", db_path);
        
        // Apply migration
        let status = Command::new("sqlite3")
            .arg(db_path)
            .arg(".read migrations/20240421_ordo_quiz_schema.sql")
            .status()?;
        
        if !status.success() {
            return Err("Failed to apply migration".into());
        }
        
        println!("Applied migration to database: {}", db_path);
        
        // Insert test data
        let status = Command::new("sqlite3")
            .arg(db_path)
            .arg(".read migrations/test_data.sql")
            .status()?;
        
        if !status.success() {
            return Err("Failed to insert test data".into());
        }
        
        println!("Inserted test data into database: {}", db_path);
    }
    
    // Launch the UI
    println!("Launching Ordo Quiz UI...");
    
    // Open the HTML file in the default browser
    let ui_path = PathBuf::from("ordo_quiz_ui/index.html");
    if !ui_path.exists() {
        println!("Creating UI directory...");
        fs::create_dir_all("ordo_quiz_ui")?;
        
        // Create the HTML file
        let html_content = include_str!("src-tauri/src/bin/quiz-standalone-ui/index.html");
        fs::write(&ui_path, html_content)?;
        
        println!("Created UI file: {:?}", ui_path);
    }
    
    // Open the HTML file in the default browser
    #[cfg(target_os = "windows")]
    {
        Command::new("cmd")
            .args(&["/C", "start", ui_path.to_str().unwrap()])
            .spawn()?;
    }
    
    #[cfg(target_os = "macos")]
    {
        Command::new("open")
            .arg(ui_path.to_str().unwrap())
            .spawn()?;
    }
    
    #[cfg(target_os = "linux")]
    {
        Command::new("xdg-open")
            .arg(ui_path.to_str().unwrap())
            .spawn()?;
    }
    
    println!("Ordo Quiz UI launched successfully!");
    println!("Press Enter to exit...");
    
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    
    println!("Shutting down Ordo Quiz...");
    
    Ok(())
}
