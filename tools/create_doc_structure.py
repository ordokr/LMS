#!/usr/bin/env python3
"""
Script to create the documentation directory structure for the EduConnect project.
"""

import os
from pathlib import Path

# Define the root directory
ROOT_DIR = Path(os.path.abspath(os.path.join(os.path.dirname(__file__), '..')))
DOCS_DIR = ROOT_DIR / 'docs'

# Define the directory structure
DIRECTORY_STRUCTURE = [
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

def create_directory_structure():
    """Create the directory structure."""
    print("Creating directory structure...")
    
    # Create the main docs directory if it doesn't exist
    if not DOCS_DIR.exists():
        DOCS_DIR.mkdir(parents=True)
        print(f"Created {DOCS_DIR}")
    
    # Create the directory structure
    for directory in DIRECTORY_STRUCTURE:
        if not directory.exists():
            directory.mkdir(parents=True)
            print(f"Created {directory}")

def main():
    """Main function."""
    print("Creating documentation directory structure for the EduConnect project...")
    
    # Create the directory structure
    create_directory_structure()
    
    print("\nDirectory structure creation complete!")

if __name__ == "__main__":
    try:
        print(f"Root directory: {ROOT_DIR}")
        print(f"Docs directory: {DOCS_DIR}")
        main()
    except Exception as e:
        import traceback
        print(f"Error: {e}")
        traceback.print_exc()
