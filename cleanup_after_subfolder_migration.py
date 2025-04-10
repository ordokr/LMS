#!/usr/bin/env python3
"""
Script to clean up obsolete JavaScript files in subfolders after Rust migration
"""
import os
import sys

def main():
    # Define the project root
    project_root = "."
      # Dictionary mapping of migrated JavaScript files to their Rust equivalents
    migrated_files = {
        # Models
        "src/models/unifiedModels/User.js": "src/models/user.rs",
        "src/models/unifiedModels/Notification.js": "src/models/notification.rs",
        "src/models/unifiedModels/Discussion.js": "src/models/discussion.rs",
        "src/models/unifiedModels/Course.js": "src/models/course.rs",
        "src/models/unifiedModels/Assignment.js": "src/models/assignment.rs",
        "src/models/unifiedModels/index.js": "src/models/mod.rs",
        
        # APIs
        "src/api/canvasApi.js": "src/api/canvas_api.rs",
        "src/api/discourseApi.js": "src/api/discourse_api.rs",
        
        # Utilities
        "src/utils/logger.js": "src/utils/logger.rs",
        "src/utils/namingConventions.js": "src/utils/naming_conventions.rs", 
        
        # Middleware
        "src/middleware/authMiddleware.js": "src/middleware/auth_middleware.rs",
        
        # Routes
        "routes/monitoring.js": "src/routes/monitoring.rs",
        
        # Services
        "services/integration/sync_service.js": "services/integration/sync_service.rs",
        "services/integration/sync_state.js": "services/integration/sync_state.rs",
        "services/integration/sync_transaction.js": "services/integration/sync_transaction.rs"
    }
    
    # Count of deleted files
    deleted_count = 0
    
    print("Cleaning up obsolete JavaScript files in subfolders after Rust migration...")
    
    # Delete migrated files
    for js_file, rs_file in migrated_files.items():
        js_path = os.path.join(project_root, js_file)
        rs_path = os.path.join(project_root, rs_file)
        
        if os.path.exists(js_path) and os.path.exists(rs_path):
            try:
                os.remove(js_path)
                print(f"✓ Deleted: {js_file} (migrated to {rs_file})")
                deleted_count += 1
            except Exception as e:
                print(f"✗ Error deleting {js_file}: {str(e)}")
        elif os.path.exists(js_path):
            print(f"! Warning: {js_file} exists but Rust equivalent {rs_file} not found")
        elif os.path.exists(rs_path):
            print(f"! Info: Rust file {rs_file} exists but JavaScript original {js_file} not found")
    
    # Summary
    print(f"\nMigration cleanup complete. Deleted {deleted_count} obsolete JavaScript files from subfolders.")
    
    print("\nSubfolder migration to Rust complete!")

if __name__ == "__main__":
    main()
