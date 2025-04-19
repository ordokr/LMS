#!/bin/bash

# Ordo Quiz Module Utility Script
# ===============================

# Function to display the menu
show_menu() {
    echo "Ordo Quiz Module Utility Script"
    echo "==============================="
    echo
    echo "Please select an option:"
    echo "1. Build Ordo Quiz (Release)"
    echo "2. Build Ordo Quiz (Debug)"
    echo "3. Launch Ordo Quiz"
    echo "4. Clean temporary files"
    echo "5. Exit"
    echo
    read -p "Enter your choice (1-5): " choice
    
    case $choice in
        1) build_release ;;
        2) build_debug ;;
        3) launch ;;
        4) clean ;;
        5) exit 0 ;;
        *) echo "Invalid choice. Please try again."; show_menu ;;
    esac
}

# Function to build the release version
build_release() {
    echo "Building Ordo Quiz (Release)..."
    echo
    
    cd src-tauri
    cargo build --bin quiz-standalone --release
    
    if [ $? -ne 0 ]; then
        echo
        echo "Build failed. Please check the error messages above."
        read -p "Press Enter to continue..."
        cd ..
        show_menu
    fi
    
    cd ..
    
    echo
    echo "Build completed successfully."
    echo "The binary is available at: src-tauri/target/release/quiz-standalone"
    echo
    read -p "Press Enter to continue..."
    show_menu
}

# Function to build the debug version
build_debug() {
    echo "Building Ordo Quiz (Debug)..."
    echo
    
    cd src-tauri
    cargo build --bin quiz-standalone
    
    if [ $? -ne 0 ]; then
        echo
        echo "Build failed. Please check the error messages above."
        read -p "Press Enter to continue..."
        cd ..
        show_menu
    fi
    
    cd ..
    
    echo
    echo "Build completed successfully."
    echo "The binary is available at: src-tauri/target/debug/quiz-standalone"
    echo
    read -p "Press Enter to continue..."
    show_menu
}

# Function to launch the app
launch() {
    echo "Launching Ordo Quiz..."
    echo
    
    binary_found=false
    
    if [ -f "src-tauri/target/release/quiz-standalone" ]; then
        echo "Found release binary, launching..."
        "./src-tauri/target/release/quiz-standalone" &
        binary_found=true
    elif [ -f "src-tauri/target/debug/quiz-standalone" ]; then
        echo "Found debug binary, launching..."
        "./src-tauri/target/debug/quiz-standalone" &
        binary_found=true
    elif [ -f "target/release/quiz-standalone" ]; then
        echo "Found release binary in root target, launching..."
        "./target/release/quiz-standalone" &
        binary_found=true
    elif [ -f "target/debug/quiz-standalone" ]; then
        echo "Found debug binary in root target, launching..."
        "./target/debug/quiz-standalone" &
        binary_found=true
    fi
    
    if [ "$binary_found" = false ]; then
        echo "Binary not found. Would you like to build it now?"
        read -p "Enter 'y' to build or any other key to return to menu: " build_choice
        
        if [ "$build_choice" = "y" ] || [ "$build_choice" = "Y" ]; then
            build_debug
        else
            show_menu
        fi
    fi
    
    echo
    echo "Ordo Quiz has been launched."
    echo
    read -p "Press Enter to continue..."
    show_menu
}

# Function to clean temporary files
clean() {
    echo "Cleaning temporary files..."
    echo
    
    if [ -f "ordo_quiz.db" ]; then
        rm -f "ordo_quiz.db"
        echo "Removed ordo_quiz.db"
    fi
    
    if [ -f "ordo_quiz_test.db" ]; then
        rm -f "ordo_quiz_test.db"
        echo "Removed ordo_quiz_test.db"
    fi
    
    if [ -f "quenti.db" ]; then
        rm -f "quenti.db"
        echo "Removed quenti.db"
    fi
    
    if [ -d "ordo_quiz_data" ]; then
        rm -rf "ordo_quiz_data"
        echo "Removed ordo_quiz_data directory"
    fi
    
    if [ -d "quenti_data" ]; then
        rm -rf "quenti_data"
        echo "Removed quenti_data directory"
    fi
    
    if [ -d "test_data" ]; then
        rm -rf "test_data"
        echo "Removed test_data directory"
    fi
    
    echo
    echo "Cleanup completed."
    echo
    read -p "Press Enter to continue..."
    show_menu
}

# Check if a command was passed as an argument
if [ $# -gt 0 ]; then
    case $1 in
        build_release) build_release ;;
        build_debug) build_debug ;;
        launch) launch ;;
        clean) clean ;;
        *) show_menu ;;
    esac
else
    show_menu
fi
