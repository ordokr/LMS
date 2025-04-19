# Ordo Quiz Standalone Module

This document provides an overview of the Ordo Quiz standalone module, its features, and how to use it.

## Overview

The Ordo Quiz standalone module is a detachable component of the Ordo LMS that allows users to create, take, and manage quizzes independently of the main application. It features offline-first functionality with synchronization capabilities, making it ideal for use in environments with limited or intermittent internet connectivity.

## Features

- **Offline-First**: Create and take quizzes even without an internet connection
- **Synchronization**: Automatically sync changes when connectivity is restored
- **Standalone Operation**: Run independently of the main Ordo LMS application
- **Data Persistence**: Local SQLite database for reliable data storage
- **Cross-Platform**: Works on Windows, macOS, and Linux

## Getting Started

### Installation

1. Download the appropriate launcher for your platform:
   - Windows: `launch-ordo-quiz.bat`
   - macOS/Linux: `launch-ordo-quiz.sh`

2. Make the launcher executable (macOS/Linux only):
   ```bash
   chmod +x launch-ordo-quiz.sh
   ```

3. Run the launcher:
   - Windows: Double-click `launch-ordo-quiz.bat`
   - macOS/Linux: Run `./launch-ordo-quiz.sh`

### Creating a Quiz

1. Launch the Ordo Quiz standalone application
2. Click the "Create New Quiz" button
3. Fill in the quiz details:
   - Title
   - Description
   - Time limit
   - Passing score
   - Question shuffling preference
   - Results display preference
4. Click "Create Quiz" to save the quiz

### Taking a Quiz

1. Launch the Ordo Quiz standalone application
2. Browse the list of available quizzes
3. Click the "Start Quiz" button on the quiz you want to take
4. Answer the questions within the time limit
5. Submit your answers to see your results

### Offline Mode

The application automatically detects your network status, but you can also manually toggle offline mode:

1. Click the "Go Offline" button in the status bar
2. Create or take quizzes as normal
3. Changes will be stored locally
4. Click "Go Online" when connectivity is restored
5. Click "Sync Now" to synchronize your changes with the server

## Technical Details

### Architecture

The Ordo Quiz standalone module is built using:

- **Rust**: Core application logic and database interactions
- **Tauri**: Cross-platform desktop application framework
- **SQLite**: Local database for data persistence
- **HTML/CSS/JavaScript**: User interface

### Data Synchronization

The synchronization process works as follows:

1. Changes made while offline are stored in the local database
2. A record of each change is also stored in the sync directory
3. When online, the application processes each sync record
4. Changes are sent to the server via the API
5. Once synchronized, the sync records are removed

### File Structure

- `src-tauri/src/bin/quiz-standalone.rs`: Main application entry point
- `src-tauri/src/bin/quiz-standalone-ui/`: UI files
- `src-tauri/src/quiz/`: Core quiz functionality
- `src-tauri/src/launchers/`: Launcher scripts

## Integration with Ordo LMS

The standalone module can be launched from within the main Ordo LMS application:

1. Navigate to the Quiz section in Ordo LMS
2. Click the "Launch Standalone" button
3. The standalone application will open with your account already authenticated
4. Changes made in the standalone app will be synchronized with the main application

## Troubleshooting

### Common Issues

- **Application won't start**: Ensure you have the required dependencies installed
- **Sync fails**: Check your internet connection and try again
- **Database errors**: Try restarting the application

### Logs

Log files are stored in:
- Windows: `%APPDATA%\ordo-quiz\logs\`
- macOS: `~/Library/Application Support/ordo-quiz/logs/`
- Linux: `~/.config/ordo-quiz/logs/`

## Future Enhancements

Planned enhancements for future versions:

- Mobile application support
- Enhanced offline capabilities
- Additional question types
- Improved analytics
- Integration with external learning management systems
