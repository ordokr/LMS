@echo off
echo Building minimal Ordo Quiz standalone version...

REM First, initialize the database
call initialize-database.bat

REM Then, fix compilation errors
call fix-compilation-errors.bat

REM Check if src-tauri directory exists
if exist "src-tauri" (
    cd src-tauri
    
    echo Creating minimal quiz-standalone binary...
    
    REM Create a minimal main.rs for quiz-standalone if it doesn't exist
    if not exist "src\bin\quiz_standalone.rs" (
        echo Creating minimal quiz-standalone entry point...
        
        REM Create bin directory if it doesn't exist
        if not exist "src\bin" (
            mkdir "src\bin"
        )
        
        echo // Minimal Ordo Quiz standalone entry point > src\bin\quiz_standalone.rs
        echo use std::sync::Arc; >> src\bin\quiz_standalone.rs
        echo use sqlx::SqlitePool; >> src\bin\quiz_standalone.rs
        echo. >> src\bin\quiz_standalone.rs
        echo #[tokio::main] >> src\bin\quiz_standalone.rs
        echo async fn main() -^> Result^<(), Box^<dyn std::error::Error^>^> { >> src\bin\quiz_standalone.rs
        echo     println!("Starting Ordo Quiz..."); >> src\bin\quiz_standalone.rs
        echo. >> src\bin\quiz_standalone.rs
        echo     // Initialize database connection >> src\bin\quiz_standalone.rs
        echo     let db_url = "sqlite:data/ordo_quiz.db?mode=rwc"; >> src\bin\quiz_standalone.rs
        echo     let db_pool = SqlitePool::connect(db_url).await?; >> src\bin\quiz_standalone.rs
        echo. >> src\bin\quiz_standalone.rs
        echo     // Create app state >> src\bin\quiz_standalone.rs
        echo     let app_state = Arc::new(lms_lib::AppState { >> src\bin\quiz_standalone.rs
        echo         db_pool, >> src\bin\quiz_standalone.rs
        echo         jwt_secret: "ordo_quiz_secret_key".as_bytes().to_vec(), >> src\bin\quiz_standalone.rs
        echo     }); >> src\bin\quiz_standalone.rs
        echo. >> src\bin\quiz_standalone.rs
        echo     // Start the application >> src\bin\quiz_standalone.rs
        echo     println!("Ordo Quiz is running!"); >> src\bin\quiz_standalone.rs
        echo     println!("Press Ctrl+C to exit"); >> src\bin\quiz_standalone.rs
        echo. >> src\bin\quiz_standalone.rs
        echo     // Keep the application running >> src\bin\quiz_standalone.rs
        echo     tokio::signal::ctrl_c().await?; >> src\bin\quiz_standalone.rs
        echo     println!("Shutting down Ordo Quiz..."); >> src\bin\quiz_standalone.rs
        echo. >> src\bin\quiz_standalone.rs
        echo     Ok(()) >> src\bin\quiz_standalone.rs
        echo } >> src\bin\quiz_standalone.rs
    )
    
    REM Update lib.rs to export AppState
    echo Updating lib.rs...
    
    if exist "src\lib.rs" (
        echo mod app_state; >> src\lib.rs
        echo pub use app_state::AppState; >> src\lib.rs
    ) else (
        echo // Minimal lib.rs > src\lib.rs
        echo mod app_state; >> src\lib.rs
        echo pub use app_state::AppState; >> src\lib.rs
        echo. >> src\lib.rs
        echo // Re-export essential components >> src\lib.rs
        echo pub mod shared; >> src\lib.rs
    )
    
    REM Build the minimal version
    echo Building minimal version...
    cargo build --bin quiz-standalone
    
    if %ERRORLEVEL% EQU 0 (
        echo Build successful!
        echo You can now run the Ordo Quiz standalone with:
        echo   .\target\debug\quiz-standalone.exe
    ) else (
        echo Build failed. Please check the error messages above.
    )
    
    cd ..
) else (
    echo src-tauri directory not found
)

echo Build process complete
pause
