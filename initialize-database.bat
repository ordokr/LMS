@echo off
echo Initializing database for Ordo Quiz...

REM Check if src-tauri directory exists
if exist "src-tauri" (
    cd src-tauri
    
    REM Create database directory if it doesn't exist
    if not exist "data" (
        mkdir data
        echo Created data directory
    )
    
    REM Check if database file exists, if not create it
    if not exist "data\ordo_quiz.db" (
        echo Creating new database file...
        
        REM Create a minimal SQLite database with required tables
        echo .open data\ordo_quiz.db > create_db.sql
        echo CREATE TABLE IF NOT EXISTS courses ( >> create_db.sql
        echo   id INTEGER PRIMARY KEY AUTOINCREMENT, >> create_db.sql
        echo   name TEXT NOT NULL, >> create_db.sql
        echo   code TEXT, >> create_db.sql
        echo   description TEXT, >> create_db.sql
        echo   created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, >> create_db.sql
        echo   updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, >> create_db.sql
        echo   deleted_at TIMESTAMP >> create_db.sql
        echo ); >> create_db.sql
        
        echo CREATE TABLE IF NOT EXISTS enrollments ( >> create_db.sql
        echo   id INTEGER PRIMARY KEY AUTOINCREMENT, >> create_db.sql
        echo   user_id INTEGER NOT NULL, >> create_db.sql
        echo   course_id INTEGER NOT NULL, >> create_db.sql
        echo   role INTEGER NOT NULL, >> create_db.sql
        echo   status INTEGER NOT NULL, >> create_db.sql
        echo   created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, >> create_db.sql
        echo   updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, >> create_db.sql
        echo   FOREIGN KEY (course_id) REFERENCES courses(id) >> create_db.sql
        echo ); >> create_db.sql
        
        echo CREATE TABLE IF NOT EXISTS users ( >> create_db.sql
        echo   id INTEGER PRIMARY KEY AUTOINCREMENT, >> create_db.sql
        echo   username TEXT NOT NULL UNIQUE, >> create_db.sql
        echo   email TEXT NOT NULL UNIQUE, >> create_db.sql
        echo   password_hash TEXT NOT NULL, >> create_db.sql
        echo   role INTEGER NOT NULL, >> create_db.sql
        echo   created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, >> create_db.sql
        echo   updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP >> create_db.sql
        echo ); >> create_db.sql
        
        REM Execute the SQL script
        sqlite3 < create_db.sql
        
        REM Check if database was created successfully
        if exist "data\ordo_quiz.db" (
            echo Database created successfully
            del create_db.sql
        ) else (
            echo Failed to create database
        )
    ) else (
        echo Database file already exists
    )
    
    REM Update database connection string in code
    echo Updating database connection string...
    
    REM Create a minimal .env file with database connection
    echo DATABASE_URL=sqlite:data/ordo_quiz.db > .env
    echo JWT_SECRET=ordo_quiz_secret_key_for_development_only >> .env
    
    cd ..
) else (
    echo src-tauri directory not found
)

echo Database initialization complete
pause
