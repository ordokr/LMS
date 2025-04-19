#!/bin/bash
echo "Building Ordo Quiz Standalone..."

# Try to build from src-tauri directory
if [ -f "src-tauri/Cargo.toml" ]; then
    echo "Building from src-tauri directory..."
    cd src-tauri
    cargo build --release --bin quiz-standalone

    if [ $? -eq 0 ]; then
        echo "Build successful!"
        echo "The standalone executable is located at:"
        echo "$(pwd)/../target/release/quiz-standalone"

        echo "Creating desktop entry..."
        mkdir -p $HOME/.local/share/applications
        cat > $HOME/.local/share/applications/ordo-quiz.desktop << EOF
[Desktop Entry]
Name=Ordo Quiz
Exec=$(pwd)/../target/release/quiz-standalone
Icon=applications-education
Terminal=false
Type=Application
Categories=Education;
EOF

        echo "Creating symlink on desktop..."
        ln -sf "$(pwd)/../target/release/quiz-standalone" "$HOME/Desktop/ordo-quiz"
        chmod +x "$HOME/Desktop/ordo-quiz"

        echo "Done! You can now launch Ordo Quiz by double-clicking the 'ordo-quiz' icon on your desktop."
        cd ..
        exit 0
    else
        echo "Build failed in src-tauri directory. Trying root directory..."
        cd ..
    fi
fi

# Try to build from root directory
if [ -f "Cargo.toml" ]; then
    echo "Building from root directory..."
    cargo build --release --bin quiz-standalone

    if [ $? -eq 0 ]; then
        echo "Build successful!"
        echo "The standalone executable is located at:"
        echo "$(pwd)/target/release/quiz-standalone"

        echo "Creating desktop entry..."
        mkdir -p $HOME/.local/share/applications
        cat > $HOME/.local/share/applications/ordo-quiz.desktop << EOF
[Desktop Entry]
Name=Ordo Quiz
Exec=$(pwd)/target/release/quiz-standalone
Icon=applications-education
Terminal=false
Type=Application
Categories=Education;
EOF

        echo "Creating symlink on desktop..."
        ln -sf "$(pwd)/target/release/quiz-standalone" "$HOME/Desktop/ordo-quiz"
        chmod +x "$HOME/Desktop/ordo-quiz"

        echo "Done! You can now launch Ordo Quiz by double-clicking the 'ordo-quiz' icon on your desktop."
        exit 0
    else
        echo "Build failed. Please check the error messages above."
    fi
fi

echo "Could not find or build the Ordo Quiz module."
echo "Please make sure you have the necessary Cargo.toml files in your project."
read -p "Press Enter to continue..."
