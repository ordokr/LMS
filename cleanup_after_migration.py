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
        # Root directory files
        "technical-docs-generator.js",
        "summary-report-generator.js",
        "visual-dashboard-generator.js",
        "port-conflict-analyzer.js",
        "generate-port-docs.js",
        "test-wasm-integration.js",
        "cleanup-docs.js",
        "check-wasm-files.js",
        "app.js",
        "analyze.js",
        
        # Source files
        "src/api/canvasApi.js",
        "src/api/discourseApi.js",
        "src/controllers/authController.js",
        "src/routes/authRoutes.js",
        "src/utils/logger.js",
        "src/utils/namingConventions.js",
        "src/middleware/authMiddleware.js",
        "src/webhooks/canvas.js",
        "src/auth/jwtService.js",
        "src/services/canvasAuthService.js",
        "src/services/discourseSSOService.js",
        "src/services/auth.js",
        "src/services/integration.js",
        "src/services/modelSyncService.js",
        "src/services/notificationService.js",
        "src/services/webhookService.js",
        
        # Models
        "src/models/unifiedModels/User.js",
        "src/models/unifiedModels/Notification.js",
        "src/models/unifiedModels/Discussion.js",
        "src/models/unifiedModels/Course.js",
        "src/models/unifiedModels/Assignment.js",
        "src/models/unified/BaseModel.js",
        "src/models/unified/UserModel.js",
        "src/models/unifiedModels/index.js",
        "src/models/ModelFactory.js",
        "src/models/index.js",
        "src/models/unifiedModels/Announcement.js",
        "src/models/unifiedModels/Module.js",
        "src/models/unifiedModels/Grade.js",
        "src/models/unifiedModels/Group.js",
        "src/models/unifiedModels/Quiz.js",
        "src/models/unifiedModels/Submission.js",
        "src/models/unifiedModels/Calendar.js",
        
        # Services and integration
        "services/integration/sync_service.js",
        "services/integration/sync_state.js",
        "services/integration/sync_transaction.js",
        "services/integration/mapping/course-category-mapper.js",
        "services/integration/model_mapper.js",
        "services/integration/api_integration.js",
        "services/integration/auth/jwt-provider.js",
        "services/database.js",
        "services/monitoring/sync_monitor.js",
        "services/monitoring/sync_dashboard.js",
        
        # Routes
        "routes/monitoring.js",
        
        # Migration file
        "Migration/fileSystemUtils.js"
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
