#!/bin/bash
echo "Building Quenti Standalone..."

cd ~/Desktop/quenti
cargo build --release --bin quiz-standalone

if [ $? -eq 0 ]; then
    echo "Build successful!"
    echo "The standalone executable is located at:"
    echo "$HOME/Desktop/quenti/target/release/quiz-standalone"
    
    echo "Creating desktop entry..."
    cat > $HOME/.local/share/applications/quenti-quiz.desktop << EOF
[Desktop Entry]
Name=Quenti Quiz
Exec=$HOME/Desktop/quenti/target/release/quiz-standalone
Icon=applications-education
Terminal=false
Type=Application
Categories=Education;
EOF
    
    echo "Creating symlink on desktop..."
    ln -sf $HOME/Desktop/quenti/target/release/quiz-standalone $HOME/Desktop/quenti-quiz
    chmod +x $HOME/Desktop/quenti-quiz
    
    echo "Done! You can now launch Quenti by double-clicking the 'quenti-quiz' icon on your desktop."
else
    echo "Build failed. Please check the error messages above."
    read -p "Press Enter to continue..."
fi
