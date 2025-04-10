#!/usr/bin/env python3
"""
Script to clean up obsolete JavaScript files after Rust migration
"""
import os
import sys

def main():
    # Define the project root
    project_root = "."
    
    # List of obsolete JavaScript files that have been migrated to Rust
    obsolete_files = [
        "technical-docs-generator.js",
        "summary-report-generator.js",
        "visual-dashboard-generator.js",
        "generate-port-docs.js",
        "test-wasm-integration.js",
        "cleanup-docs.js",
        "check-wasm-files.js",
        "app.js"
    ]
    
    # Files to preserve (configuration files)
    preserve_files = [
        "vite.config.js",
        "jest.setup.js",
        "jest.config.js",
        "babel.config.js",
        ".eslintrc.js"
    ]
    
    # Count of deleted files
    deleted_count = 0
    
    print("Cleaning up obsolete JavaScript files after Rust migration...")
    
    # Delete obsolete files
    for file_name in obsolete_files:
        file_path = os.path.join(project_root, file_name)
        if os.path.exists(file_path):
            try:
                os.remove(file_path)
                print(f"✓ Deleted: {file_name}")
                deleted_count += 1
            except Exception as e:
                print(f"✗ Error deleting {file_name}: {str(e)}")
        else:
            print(f"! Not found: {file_name}")
    
    # Summary
    print(f"\nMigration cleanup complete. Deleted {deleted_count} obsolete JavaScript files.")
    print("Preserved the following configuration files:")
    for file_name in preserve_files:
        file_path = os.path.join(project_root, file_name)
        if os.path.exists(file_path):
            print(f"- {file_name}")
    
    print("\nMigration to Rust complete!")

if __name__ == "__main__":
    main()
