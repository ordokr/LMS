#!/usr/bin/env python3
"""
Script to organize documentation in the EduConnect project.
This script will:
1. Create the necessary directory structure
2. Move existing documentation to the appropriate directories
3. Update references in documentation files
"""

import os
import re
import shutil
from pathlib import Path
import sys

# Define the root directory
ROOT_DIR = Path(os.path.abspath(os.path.join(os.path.dirname(__file__), '..')))
DOCS_DIR = ROOT_DIR / 'docs'

# Define the directory structure
DIRECTORY_STRUCTURE = {
    'api': DOCS_DIR / 'api',
    'architecture': DOCS_DIR / 'architecture',
    'models': DOCS_DIR / 'models',
    'integration': DOCS_DIR / 'integration',
    'technical': DOCS_DIR / 'technical',
    'visualizations': {
        'root': DOCS_DIR / 'visualizations',
        'api_map': DOCS_DIR / 'visualizations' / 'api_map',
        'component_tree': DOCS_DIR / 'visualizations' / 'component_tree',
        'db_schema': DOCS_DIR / 'visualizations' / 'db_schema',
        'migration_roadmap': DOCS_DIR / 'visualizations' / 'migration_roadmap',
    },
    'analysis': {
        'root': DOCS_DIR / 'analysis',
        'canvas': DOCS_DIR / 'analysis' / 'canvas',
        'discourse': DOCS_DIR / 'analysis' / 'discourse',
        'conflicts': DOCS_DIR / 'analysis' / 'conflicts',
    },
    'rag_knowledge_base': {
        'root': DOCS_DIR / 'rag_knowledge_base',
        'canvas': DOCS_DIR / 'rag_knowledge_base' / 'canvas',
        'discourse': DOCS_DIR / 'rag_knowledge_base' / 'discourse',
        'integration': DOCS_DIR / 'rag_knowledge_base' / 'integration',
    },
    'development': DOCS_DIR / 'development',
}

# Define files to move
FILES_TO_MOVE = [
    # Central files
    {'source': DOCS_DIR / 'central_reference_hub.md', 'dest': DOCS_DIR / 'central_reference_hub.md'},
    {'source': DOCS_DIR / 'index.md', 'dest': DOCS_DIR / 'index.md'},

    # API documentation
    {'source': DOCS_DIR / 'api_documentation.md', 'dest': DOCS_DIR / 'api' / 'overview.md'},
    {'source': DOCS_DIR / 'api' / 'reference.md', 'dest': DOCS_DIR / 'api' / 'reference.md'},

    # Architecture documentation
    {'source': DOCS_DIR / 'architecture_overview.md', 'dest': DOCS_DIR / 'architecture' / 'overview.md'},
    {'source': DOCS_DIR / 'database_architecture.md', 'dest': DOCS_DIR / 'architecture' / 'database.md'},
    {'source': DOCS_DIR / 'synchronization_architecture.md', 'dest': DOCS_DIR / 'architecture' / 'synchronization.md'},

    # Models documentation
    {'source': DOCS_DIR / 'database_schema.md', 'dest': DOCS_DIR / 'models' / 'database_schema.md'},
    {'source': DOCS_DIR / 'unified_models.md', 'dest': DOCS_DIR / 'models' / 'unified_models.md'},

    # Integration documentation
    {'source': DOCS_DIR / 'implementation_roadmap.md', 'dest': DOCS_DIR / 'integration' / 'roadmap.md'},

    # Technical documentation
    {'source': DOCS_DIR / 'implementation_details.md', 'dest': DOCS_DIR / 'technical' / 'implementation_details.md'},
    {'source': DOCS_DIR / 'technical_debt_report.md', 'dest': DOCS_DIR / 'technical' / 'technical_debt_report.md'},
    {'source': DOCS_DIR / 'tests.md', 'dest': DOCS_DIR / 'technical' / 'tests.md'},
    {'source': DOCS_DIR / 'performance_report.md', 'dest': DOCS_DIR / 'technical' / 'performance_report.md'},
    {'source': DOCS_DIR / 'metrics_report.md', 'dest': DOCS_DIR / 'technical' / 'metrics_report.md'},

    # Visualizations
    {'source': DOCS_DIR / 'visualizations' / 'README.md', 'dest': DOCS_DIR / 'visualizations' / 'README.md'},

    # Analysis
    {'source': DOCS_DIR / 'analysis_summary' / 'canvas_analysis.md', 'dest': DOCS_DIR / 'analysis' / 'canvas' / 'analysis.md'},
    {'source': DOCS_DIR / 'analysis_summary' / 'discourse_analysis.md', 'dest': DOCS_DIR / 'analysis' / 'discourse' / 'analysis.md'},
    {'source': DOCS_DIR / 'analysis_summary' / 'conflicts' / 'port_conflicts.md', 'dest': DOCS_DIR / 'analysis' / 'conflicts' / 'conflicts.md'},
    {'source': DOCS_DIR / 'analysis_summary' / 'master_report.md', 'dest': DOCS_DIR / 'analysis' / 'master_report.md'},

    # Development
    {'source': DOCS_DIR / 'CONTRIBUTING.md', 'dest': DOCS_DIR / 'development' / 'contribution.md'},
    {'source': ROOT_DIR / 'MAINTENANCE.md', 'dest': DOCS_DIR / 'development' / 'maintenance.md'},
]

# Define directories to copy recursively
DIRS_TO_COPY = [
    # RAG knowledge base
    {'source': DOCS_DIR / 'rag_knowledge_base', 'dest': DOCS_DIR / 'rag_knowledge_base'},

    # Visualizations
    {'source': DOCS_DIR / 'visualizations', 'dest': DOCS_DIR / 'visualizations'},
]

def create_directory_structure():
    """Create the directory structure."""
    print("Creating directory structure...")

    # Create the main docs directory if it doesn't exist
    if not DOCS_DIR.exists():
        DOCS_DIR.mkdir(parents=True)
        print(f"Created {DOCS_DIR}")

    # Create the directory structure
    for key, value in DIRECTORY_STRUCTURE.items():
        if isinstance(value, dict):
            # Create the root directory
            if not value['root'].exists():
                value['root'].mkdir(parents=True)
                print(f"Created {value['root']}")

            # Create the subdirectories
            for subkey, subvalue in value.items():
                if subkey != 'root' and not subvalue.exists():
                    subvalue.mkdir(parents=True)
                    print(f"Created {subvalue}")
        else:
            # Create the directory
            if not value.exists():
                value.mkdir(parents=True)
                print(f"Created {value}")

def move_files():
    """Move files to the appropriate directories."""
    print("\nMoving files...")

    for file_info in FILES_TO_MOVE:
        source = file_info['source']
        dest = file_info['dest']

        # Skip if source doesn't exist
        if not source.exists():
            print(f"Skipping {source} (not found)")
            continue

        # Create the destination directory if it doesn't exist
        if not dest.parent.exists():
            dest.parent.mkdir(parents=True)
            print(f"Created {dest.parent}")

        # Copy the file
        try:
            shutil.copy2(source, dest)
            print(f"Copied {source} to {dest}")
        except Exception as e:
            print(f"Error copying {source} to {dest}: {e}")

def copy_directories():
    """Copy directories recursively."""
    print("\nCopying directories...")

    for dir_info in DIRS_TO_COPY:
        source = dir_info['source']
        dest = dir_info['dest']

        # Skip if source doesn't exist
        if not source.exists():
            print(f"Skipping {source} (not found)")
            continue

        # Create the destination directory if it doesn't exist
        if not dest.parent.exists():
            dest.parent.mkdir(parents=True)
            print(f"Created {dest.parent}")

        # Copy the directory
        try:
            if dest.exists():
                shutil.rmtree(dest)
            shutil.copytree(source, dest)
            print(f"Copied {source} to {dest}")
        except Exception as e:
            print(f"Error copying {source} to {dest}: {e}")

def update_references():
    """Update references in documentation files."""
    print("\nUpdating references in documentation files...")

    # Find all markdown files in the docs directory
    markdown_files = []
    for root, _, files in os.walk(DOCS_DIR):
        for file in files:
            if file.endswith('.md'):
                markdown_files.append(Path(root) / file)

    # Define patterns to search for and their replacements
    patterns = [
        # Update references to central_reference_hub.md
        (r'\]\(central_reference_hub\.md\)', '](../central_reference_hub.md)'),
        (r'\]\(\.\/central_reference_hub\.md\)', '](../central_reference_hub.md)'),

        # Update references to index.md
        (r'\]\(index\.md\)', '](../index.md)'),
        (r'\]\(\.\/index\.md\)', '](../index.md)'),

        # Update references to api_documentation.md
        (r'\]\(api_documentation\.md\)', '](../api/overview.md)'),
        (r'\]\(\.\/api_documentation\.md\)', '](../api/overview.md)'),

        # Update references to architecture_overview.md
        (r'\]\(architecture_overview\.md\)', '](../architecture/overview.md)'),
        (r'\]\(\.\/architecture_overview\.md\)', '](../architecture/overview.md)'),

        # Update references to database_schema.md
        (r'\]\(database_schema\.md\)', '](../models/database_schema.md)'),
        (r'\]\(\.\/database_schema\.md\)', '](../models/database_schema.md)'),

        # Update references to implementation_roadmap.md
        (r'\]\(implementation_roadmap\.md\)', '](../integration/roadmap.md)'),
        (r'\]\(\.\/implementation_roadmap\.md\)', '](../integration/roadmap.md)'),

        # Update references to technical_debt_report.md
        (r'\]\(technical_debt_report\.md\)', '](../technical/technical_debt_report.md)'),
        (r'\]\(\.\/technical_debt_report\.md\)', '](../technical/technical_debt_report.md)'),
    ]

    # Update references in each file
    for file_path in markdown_files:
        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                content = f.read()

            # Apply each pattern
            updated_content = content
            for pattern, replacement in patterns:
                updated_content = re.sub(pattern, replacement, updated_content)

            # Write the updated content back to the file
            if updated_content != content:
                with open(file_path, 'w', encoding='utf-8') as f:
                    f.write(updated_content)
                print(f"Updated references in {file_path}")
        except Exception as e:
            print(f"Error updating references in {file_path}: {e}")

def main():
    """Main function."""
    print("Organizing documentation in the EduConnect project...")

    # Create the directory structure
    create_directory_structure()

    # Move files
    move_files()

    # Copy directories
    copy_directories()

    # Update references
    update_references()

    print("\nDocumentation organization complete!")

if __name__ == "__main__":
    try:
        print(f"Root directory: {ROOT_DIR}")
        print(f"Docs directory: {DOCS_DIR}")
        main()
    except Exception as e:
        import traceback
        print(f"Error: {e}")
        traceback.print_exc()
