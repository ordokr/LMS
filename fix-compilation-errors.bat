@echo off
echo Fixing common compilation errors for Ordo Quiz...

REM Check if src-tauri directory exists
if exist "src-tauri" (
    cd src-tauri
    
    REM Create missing directories and files
    if not exist "src\shared\models" (
        mkdir "src\shared\models" 2>NUL
        echo Created missing models directory
    )
    
    REM Create a minimal AppState.rs file if it doesn't exist
    if not exist "src\app_state.rs" (
        echo Creating minimal AppState.rs...
        echo // Minimal AppState implementation > src\app_state.rs
        echo use sqlx::SqlitePool; >> src\app_state.rs
        echo use std::sync::Arc; >> src\app_state.rs
        echo. >> src\app_state.rs
        echo #[derive(Debug)] >> src\app_state.rs
        echo pub struct AppState { >> src\app_state.rs
        echo     pub db_pool: SqlitePool, >> src\app_state.rs
        echo     pub jwt_secret: Vec^<u8^>, >> src\app_state.rs
        echo } >> src\app_state.rs
    )
    
    REM Create minimal models if they don't exist
    if not exist "src\shared\models\mod.rs" (
        echo Creating minimal models module...
        echo // Minimal models module > src\shared\models\mod.rs
        echo pub mod user; >> src\shared\models\mod.rs
        echo pub mod auth; >> src\shared\models\mod.rs
        echo pub mod course; >> src\shared\models\mod.rs
        echo pub mod forum; >> src\shared\models\mod.rs
    )
    
    REM Create minimal user model
    if not exist "src\shared\models\user.rs" (
        echo Creating minimal user model...
        echo // Minimal user model > src\shared\models\user.rs
        echo use serde::{Deserialize, Serialize}; >> src\shared\models\user.rs
        echo. >> src\shared\models\user.rs
        echo #[derive(Debug, Serialize, Deserialize, Clone)] >> src\shared\models\user.rs
        echo pub struct User { >> src\shared\models\user.rs
        echo     pub id: i64, >> src\shared\models\user.rs
        echo     pub username: String, >> src\shared\models\user.rs
        echo     pub email: String, >> src\shared\models\user.rs
        echo     pub role: UserRole, >> src\shared\models\user.rs
        echo } >> src\shared\models\user.rs
        echo. >> src\shared\models\user.rs
        echo #[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)] >> src\shared\models\user.rs
        echo pub enum UserRole { >> src\shared\models\user.rs
        echo     Student = 0, >> src\shared\models\user.rs
        echo     Teacher = 1, >> src\shared\models\user.rs
        echo     Admin = 2, >> src\shared\models\user.rs
        echo } >> src\shared\models\user.rs
        echo. >> src\shared\models\user.rs
        echo #[derive(Debug, Serialize, Deserialize)] >> src\shared\models\user.rs
        echo pub struct UserProfile { >> src\shared\models\user.rs
        echo     pub id: i64, >> src\shared\models\user.rs
        echo     pub username: String, >> src\shared\models\user.rs
        echo     pub email: String, >> src\shared\models\user.rs
        echo     pub role: UserRole, >> src\shared\models\user.rs
        echo } >> src\shared\models\user.rs
        echo. >> src\shared\models\user.rs
        echo #[derive(Debug, Serialize, Deserialize)] >> src\shared\models\user.rs
        echo pub struct LoginRequest { >> src\shared\models\user.rs
        echo     pub username: String, >> src\shared\models\user.rs
        echo     pub password: String, >> src\shared\models\user.rs
        echo } >> src\shared\models\user.rs
        echo. >> src\shared\models\user.rs
        echo #[derive(Debug, Serialize, Deserialize)] >> src\shared\models\user.rs
        echo pub struct RegisterRequest { >> src\shared\models\user.rs
        echo     pub username: String, >> src\shared\models\user.rs
        echo     pub email: String, >> src\shared\models\user.rs
        echo     pub password: String, >> src\shared\models\user.rs
        echo } >> src\shared\models\user.rs
    )
    
    REM Create minimal auth model
    if not exist "src\shared\models\auth.rs" (
        echo Creating minimal auth model...
        echo // Minimal auth model > src\shared\models\auth.rs
        echo use serde::{Deserialize, Serialize}; >> src\shared\models\auth.rs
        echo use crate::shared::models::user::{User, UserRole}; >> src\shared\models\auth.rs
        echo. >> src\shared\models\auth.rs
        echo #[derive(Debug, Serialize, Deserialize)] >> src\shared\models\auth.rs
        echo pub struct JwtClaims { >> src\shared\models\auth.rs
        echo     pub sub: String, >> src\shared\models\auth.rs
        echo     pub exp: u64, >> src\shared\models\auth.rs
        echo     pub role: UserRole, >> src\shared\models\auth.rs
        echo } >> src\shared\models\auth.rs
        echo. >> src\shared\models\auth.rs
        echo #[derive(Debug, Serialize, Deserialize)] >> src\shared\models\auth.rs
        echo pub struct UserAuthProfile { >> src\shared\models\auth.rs
        echo     pub id: i64, >> src\shared\models\auth.rs
        echo     pub username: String, >> src\shared\models\auth.rs
        echo     pub role: UserRole, >> src\shared\models\auth.rs
        echo } >> src\shared\models\auth.rs
        echo. >> src\shared\models\auth.rs
        echo #[derive(Debug, Serialize, Deserialize)] >> src\shared\models\auth.rs
        echo pub struct AuthResponse { >> src\shared\models\auth.rs
        echo     pub token: String, >> src\shared\models\auth.rs
        echo     pub user: User, >> src\shared\models\auth.rs
        echo } >> src\shared\models\auth.rs
    )
    
    REM Update Cargo.toml to include missing dependencies
    echo Updating Cargo.toml with missing dependencies...
    
    REM Check if argon2 is in Cargo.toml, if not add it
    findstr /C:"argon2 =" Cargo.toml >nul
    if errorlevel 1 (
        echo Adding argon2 dependency...
        echo argon2 = "0.5.2" >> Cargo.toml
    )
    
    REM Check if axum-extra is in Cargo.toml, if not add it
    findstr /C:"axum-extra =" Cargo.toml >nul
    if errorlevel 1 (
        echo Adding axum-extra dependency...
        echo axum-extra = { version = "0.7", features = ["typed-header"] } >> Cargo.toml
    )
    
    REM Check if time is in Cargo.toml, if not add it
    findstr /C:"time =" Cargo.toml >nul
    if errorlevel 1 (
        echo Adding time dependency...
        echo time = "0.3" >> Cargo.toml
    )
    
    cd ..
) else (
    echo src-tauri directory not found
)

echo Compilation error fixes complete
pause
