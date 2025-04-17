#!/usr/bin/env python3
"""
Script to set up the documentation structure for the EduConnect project.
This script will:
1. Create the necessary directory structure
2. Create placeholder documentation files
3. Create a central reference hub
"""

import os
import shutil
from pathlib import Path
from datetime import datetime

# Define the root directory
ROOT_DIR = Path(os.path.abspath(os.path.join(os.path.dirname(__file__), '..')))
DOCS_DIR = ROOT_DIR / 'docs'

# Define the directory structure
DIRECTORIES = [
    DOCS_DIR / 'api',
    DOCS_DIR / 'architecture',
    DOCS_DIR / 'models',
    DOCS_DIR / 'integration',
    DOCS_DIR / 'technical',
    DOCS_DIR / 'visualizations' / 'api_map',
    DOCS_DIR / 'visualizations' / 'component_tree',
    DOCS_DIR / 'visualizations' / 'db_schema',
    DOCS_DIR / 'visualizations' / 'migration_roadmap',
    DOCS_DIR / 'analysis' / 'canvas',
    DOCS_DIR / 'analysis' / 'discourse',
    DOCS_DIR / 'analysis' / 'conflicts',
    DOCS_DIR / 'rag_knowledge_base' / 'canvas',
    DOCS_DIR / 'rag_knowledge_base' / 'discourse',
    DOCS_DIR / 'rag_knowledge_base' / 'integration',
    DOCS_DIR / 'development',
]

# Define placeholder files to create
PLACEHOLDER_FILES = [
    # Main documentation
    {'path': DOCS_DIR / 'central_reference_hub.md', 'title': 'EduConnect LMS & Forum: Central Reference Hub'},
    {'path': DOCS_DIR / 'README.md', 'title': 'EduConnect Documentation'},
    
    # API documentation
    {'path': DOCS_DIR / 'api' / 'overview.md', 'title': 'API Documentation'},
    {'path': DOCS_DIR / 'api' / 'reference.md', 'title': 'API Reference'},
    
    # Architecture documentation
    {'path': DOCS_DIR / 'architecture' / 'overview.md', 'title': 'Architecture Overview'},
    {'path': DOCS_DIR / 'architecture' / 'database.md', 'title': 'Database Architecture'},
    {'path': DOCS_DIR / 'architecture' / 'synchronization.md', 'title': 'Synchronization Architecture'},
    
    # Models documentation
    {'path': DOCS_DIR / 'models' / 'overview.md', 'title': 'Data Models'},
    {'path': DOCS_DIR / 'models' / 'database_schema.md', 'title': 'Database Schema'},
    
    # Integration documentation
    {'path': DOCS_DIR / 'integration' / 'overview.md', 'title': 'Integration Overview'},
    {'path': DOCS_DIR / 'integration' / 'roadmap.md', 'title': 'Implementation Roadmap'},
    
    # Technical documentation
    {'path': DOCS_DIR / 'technical' / 'overview.md', 'title': 'Technical Overview'},
    {'path': DOCS_DIR / 'technical' / 'implementation_details.md', 'title': 'Implementation Details'},
    {'path': DOCS_DIR / 'technical' / 'offline_readiness.md', 'title': 'Offline-First Readiness'},
    
    # Visualizations
    {'path': DOCS_DIR / 'visualizations' / 'README.md', 'title': 'Visualizations'},
    
    # Development guides
    {'path': DOCS_DIR / 'development' / 'setup.md', 'title': 'Development Environment Setup'},
    {'path': DOCS_DIR / 'development' / 'coding_standards.md', 'title': 'Coding Standards'},
    {'path': DOCS_DIR / 'development' / 'contribution.md', 'title': 'Contribution Guidelines'},
]

def create_directory_structure():
    """Create the directory structure."""
    print("Creating directory structure...")
    
    # Create the main docs directory if it doesn't exist
    if not DOCS_DIR.exists():
        DOCS_DIR.mkdir(parents=True)
        print(f"Created {DOCS_DIR}")
    
    # Create the directory structure
    for directory in DIRECTORIES:
        if not directory.exists():
            directory.mkdir(parents=True)
            print(f"Created {directory}")

def create_placeholder_files():
    """Create placeholder documentation files."""
    print("\nCreating placeholder documentation files...")
    
    for file_info in PLACEHOLDER_FILES:
        path = file_info['path']
        title = file_info['title']
        
        # Skip if file already exists
        if path.exists():
            print(f"Skipping {path} (already exists)")
            continue
        
        # Create the file with a basic template
        try:
            with open(path, 'w', encoding='utf-8') as f:
                f.write(f"# {title}\n\n")
                f.write(f"_Last updated: {datetime.now().strftime('%Y-%m-%d')}_\n\n")
                f.write("This is a placeholder document. Replace this content with actual documentation.\n")
            print(f"Created {path}")
        except Exception as e:
            print(f"Error creating {path}: {e}")

def create_central_reference_hub():
    """Create the central reference hub."""
    print("\nCreating central reference hub...")
    
    hub_path = DOCS_DIR / 'central_reference_hub.md'
    
    # Skip if file already exists
    if hub_path.exists():
        print(f"Skipping {hub_path} (already exists)")
        return
    
    try:
        with open(hub_path, 'w', encoding='utf-8') as f:
            f.write("# EduConnect LMS & Forum: Central Reference Hub\n\n")
            f.write(f"_Last updated: {datetime.now().strftime('%Y-%m-%d')}_\n\n")
            f.write("<img alt=\"Status: Early Development\" src=\"https://img.shields.io/badge/status-early%20development-orange\">\n\n")
            
            # Project Vision and Mission
            f.write("## 🚀 Project Vision & Mission\n\n")
            f.write("**EduConnect** is a modern learning management system that prioritizes offline-first functionality, enabling education to continue even in environments with limited or intermittent connectivity. It combines robust course management with integrated discussion forums to create a comprehensive learning ecosystem.\n\n")
            f.write("### Core Principles\n\n")
            f.write("1. **Offline-First**: All core functionality works without an internet connection\n")
            f.write("2. **Integrated Experience**: Seamless integration between LMS and forum components\n")
            f.write("3. **Performance**: Fast, responsive experience even on lower-end hardware\n")
            f.write("4. **Security**: Strong data protection and privacy controls\n")
            f.write("5. **Extensibility**: Modular architecture that allows for customization\n\n")
            
            # Project Goals
            f.write("### Project Goals\n\n")
            f.write("- Create a unified application that combines the best features of Canvas LMS and Discourse\n")
            f.write("- Ensure all functionality works offline with seamless synchronization when connectivity returns\n")
            f.write("- Provide a native desktop experience with better performance than web-based alternatives\n")
            f.write("- Implement a modern, intuitive UI that improves upon the original systems\n")
            f.write("- Build a solid foundation for future extensions and customizations\n\n")
            
            # Project Status
            f.write("## 📈 Project Status\n\n")
            f.write("- **Phase**: Early Development\n")
            f.write("- **Completion**: 13.2%\n")
            f.write("- **Last Active Area**: API Development\n\n")
            
            # Implementation Progress
            f.write("### Implementation Progress\n\n")
            f.write("```json\n")
            f.write("{\n")
            f.write("  \"foundation_complete\": true,\n")
            f.write("  \"model_implementation\": \"100.0%\",\n")
            f.write("  \"api_implementation\": \"0.0%\",\n")
            f.write("  \"ui_implementation\": \"67.0%\",\n")
            f.write("  \"test_coverage\": \"6.0%\",\n")
            f.write("  \"technical_debt\": \"56%\"\n")
            f.write("}\n")
            f.write("```\n\n")
            
            # Technology Stack
            f.write("## 🔧 Technology Stack\n\n")
            f.write("EduConnect is built with modern technologies that prioritize performance, security, and offline capabilities:\n\n")
            
            f.write("### Core Technologies\n\n")
            f.write("| Layer | Technology | Purpose |\n")
            f.write("|-------|------------|--------|\n")
            f.write("| **Frontend** | Leptos (Rust) | Reactive UI framework |\n")
            f.write("| **UI Styling** | Tailwind CSS | Utility-first CSS framework |\n")
            f.write("| **Desktop Shell** | Tauri | Native cross-platform wrapper |\n")
            f.write("| **Backend** | Rust | Performance-critical components |\n")
            f.write("| **Backend** | Haskell | Type-safe business logic |\n")
            f.write("| **Database** | SQLite | Local data storage |\n")
            f.write("| **ORM** | SQLx | Type-safe SQL |\n")
            f.write("| **Search** | MeiliSearch | Full-text search capabilities |\n")
            f.write("| **Authentication** | JWT | Secure user authentication |\n")
            f.write("| **Sync Engine** | Custom Rust | Conflict resolution system |\n\n")
            
            # Project Structure
            f.write("## 📚 Project Structure\n\n")
            f.write("The project follows a modular architecture with clear separation of concerns:\n\n")
            f.write("```plaintext\n")
            f.write("EduConnect/\n")
            f.write("├── src-tauri/         # Rust backend code\n")
            f.write("│   └── src/\n")
            f.write("│       ├── api/       # API endpoints\n")
            f.write("│       ├── core/      # Core business logic\n")
            f.write("│       ├── db/        # Database interactions\n")
            f.write("│       ├── models/    # Data models\n")
            f.write("│       └── sync/      # Synchronization engine\n")
            f.write("├── src/               # Frontend code (Leptos)\n")
            f.write("│   ├── components/    # Reusable UI components\n")
            f.write("│   ├── pages/         # Application pages\n")
            f.write("│   ├── models/        # Frontend data models\n")
            f.write("│   └── services/      # Frontend services\n")
            f.write("├── services/          # Integration services\n")
            f.write("│   └── integration/   # Canvas-Discourse integration\n")
            f.write("├── tools/             # Development and analysis tools\n")
            f.write("│   └── unified-analyzer/ # Codebase analysis tool\n")
            f.write("├── rag_knowledge_base/ # RAG documentation\n")
            f.write("│   └── integration/   # Integration-specific docs\n")
            f.write("├── docs/              # Generated documentation\n")
            f.write("│   ├── port/          # Port documentation\n")
            f.write("│   └── technical/     # Technical documentation\n")
            f.write("└── analysis_summary/  # Analysis results\n")
            f.write("    └── conflicts/     # Port conflict analysis\n")
            f.write("```\n\n")
            
            # Documentation Links
            f.write("## 📑 Documentation\n\n")
            f.write("### Generated Documentation\n\n")
            f.write("- [Architecture Documentation](architecture/overview.md)\n")
            f.write("- [Models Documentation](models/overview.md)\n")
            f.write("- [Integration Documentation](integration/overview.md)\n")
            f.write("- [API Documentation](api/overview.md)\n")
            f.write("- [Technical Documentation](technical/overview.md)\n\n")
            
            f.write("### Visualizations\n\n")
            f.write("- [API Map](visualizations/api_map/api_map.html)\n")
            f.write("- [Component Tree](visualizations/component_tree/component_tree.html)\n")
            f.write("- [Database Schema](visualizations/db_schema/db_schema.html)\n")
            f.write("- [Migration Roadmap](visualizations/migration_roadmap/migration_roadmap.html)\n\n")
            
            f.write("### Development Resources\n\n")
            f.write("- [Development Environment Setup](development/setup.md)\n")
            f.write("- [Coding Standards](development/coding_standards.md)\n")
            f.write("- [Contribution Guidelines](development/contribution.md)\n\n")
            
            # Implementation Priorities
            f.write("## 📌 Implementation Priorities\n\n")
            f.write("Current development focus areas:\n\n")
            f.write("1. **API**: Add authentication to remaining endpoints\n")
            f.write("2. **Models**: Implement remaining Canvas models\n")
            f.write("3. **Testing**: Increase test coverage\n")
            f.write("4. **Documentation**: Improve documentation\n\n")
            
            # Conclusion
            f.write("## 👋 Conclusion\n\n")
            f.write("EduConnect represents a significant advancement in learning management systems by prioritizing offline-first capabilities and integrating forum functionality directly into the core platform. By combining the best features of Canvas LMS and Discourse, while addressing their limitations, we're creating a more robust, performant, and accessible educational platform.\n\n")
            f.write("This central reference hub will be continuously updated as the project evolves. All documentation is automatically generated from the codebase analysis to ensure it remains accurate and up-to-date.\n")
        
        print(f"Created central reference hub at {hub_path}")
    except Exception as e:
        print(f"Error creating central reference hub: {e}")

def create_readme():
    """Create the README.md file in the docs directory."""
    print("\nCreating README.md...")
    
    readme_path = DOCS_DIR / 'README.md'
    
    # Skip if file already exists
    if readme_path.exists():
        print(f"Skipping {readme_path} (already exists)")
        return
    
    try:
        with open(readme_path, 'w', encoding='utf-8') as f:
            f.write("# EduConnect Documentation\n\n")
            f.write("This directory contains all documentation for the EduConnect project. The documentation is organized into the following sections:\n\n")
            
            f.write("## Main Documentation\n\n")
            f.write("- [Central Reference Hub](central_reference_hub.md) - The main entry point for all documentation\n")
            f.write("- [API Documentation](api/overview.md) - Documentation for the API\n")
            f.write("- [Architecture Documentation](architecture/overview.md) - Documentation for the architecture\n")
            f.write("- [Models Documentation](models/overview.md) - Documentation for the data models\n")
            f.write("- [Integration Documentation](integration/overview.md) - Documentation for integration points\n")
            f.write("- [Technical Documentation](technical/overview.md) - Technical documentation\n\n")
            
            f.write("## Visualizations\n\n")
            f.write("- [API Map](visualizations/api_map/api_map.html) - Visualization of the API\n")
            f.write("- [Component Tree](visualizations/component_tree/component_tree.html) - Visualization of the component hierarchy\n")
            f.write("- [Database Schema](visualizations/db_schema/db_schema.html) - Visualization of the database schema\n")
            f.write("- [Migration Roadmap](visualizations/migration_roadmap/migration_roadmap.html) - Visualization of the migration roadmap\n\n")
            
            f.write("## Analysis Results\n\n")
            f.write("- [Canvas Analysis](analysis/canvas/analysis.md) - Analysis of the Canvas codebase\n")
            f.write("- [Discourse Analysis](analysis/discourse/analysis.md) - Analysis of the Discourse codebase\n")
            f.write("- [Conflict Analysis](analysis/conflicts/conflicts.md) - Analysis of conflicts between Canvas and Discourse\n\n")
            
            f.write("## RAG Knowledge Base\n\n")
            f.write("- [Canvas Knowledge Base](rag_knowledge_base/canvas/README.md) - Knowledge base for Canvas\n")
            f.write("- [Discourse Knowledge Base](rag_knowledge_base/discourse/README.md) - Knowledge base for Discourse\n")
            f.write("- [Integration Knowledge Base](rag_knowledge_base/integration/README.md) - Knowledge base for integration\n\n")
            
            f.write("## Development Guides\n\n")
            f.write("- [Development Setup](development/setup.md) - Guide for setting up the development environment\n")
            f.write("- [Coding Standards](development/coding_standards.md) - Coding standards for the project\n")
            f.write("- [Contribution Guidelines](development/contribution.md) - Guidelines for contributing to the project\n\n")
            
            f.write("## Documentation Generation\n\n")
            f.write("All documentation is automatically generated from the codebase analysis. The documentation is updated whenever the codebase is analyzed.\n\n")
            f.write("To regenerate the documentation, run:\n\n")
            f.write("```bash\n")
            f.write("cd tools/unified-analyzer\n")
            f.write("cargo run --bin unified-analyzer -- --analyze --path /path/to/project\n")
            f.write("```\n\n")
            f.write("This will analyze the codebase and generate updated documentation.\n")
        
        print(f"Created README.md at {readme_path}")
    except Exception as e:
        print(f"Error creating README.md: {e}")

def move_existing_markdown_files():
    """Move existing markdown files to the docs directory."""
    print("\nMoving existing markdown files to the docs directory...")
    
    # Find all markdown files in the project (excluding the docs directory)
    markdown_files = []
    for root, _, files in os.walk(ROOT_DIR):
        # Skip the docs directory and its subdirectories
        if str(DOCS_DIR) in str(root):
            continue
        
        # Skip node_modules, target, and .git directories
        if 'node_modules' in root or 'target' in root or '.git' in root:
            continue
        
        for file in files:
            if file.endswith('.md'):
                markdown_files.append(Path(root) / file)
    
    # Move each markdown file to the docs directory
    for file_path in markdown_files:
        # Skip README.md in the root directory
        if file_path.name == 'README.md' and file_path.parent == ROOT_DIR:
            print(f"Skipping {file_path} (root README.md)")
            continue
        
        # Determine the destination path
        rel_path = file_path.relative_to(ROOT_DIR)
        dest_path = DOCS_DIR / rel_path
        
        # Create the destination directory if it doesn't exist
        if not dest_path.parent.exists():
            dest_path.parent.mkdir(parents=True)
        
        # Copy the file
        try:
            # Create a backup of the file
            backup_path = file_path.with_suffix('.md.bak')
            shutil.copy2(file_path, backup_path)
            
            # Copy the file to the docs directory
            shutil.copy2(file_path, dest_path)
            print(f"Copied {file_path} to {dest_path}")
            
            # Remove the original file
            os.remove(file_path)
            print(f"Removed {file_path}")
        except Exception as e:
            print(f"Error moving {file_path}: {e}")

def main():
    """Main function."""
    print("Setting up documentation for the EduConnect project...")
    
    # Create the directory structure
    create_directory_structure()
    
    # Create placeholder files
    create_placeholder_files()
    
    # Create the central reference hub
    create_central_reference_hub()
    
    # Create the README.md file
    create_readme()
    
    # Move existing markdown files
    move_existing_markdown_files()
    
    print("\nDocumentation setup complete!")

if __name__ == "__main__":
    try:
        print(f"Root directory: {ROOT_DIR}")
        print(f"Docs directory: {DOCS_DIR}")
        main()
    except Exception as e:
        import traceback
        print(f"Error: {e}")
        traceback.print_exc()
