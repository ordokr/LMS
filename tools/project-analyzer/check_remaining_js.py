import os
import re
from pathlib import Path

def find_all_js_files(root_dir):
    """Find all JavaScript files in the project directory."""
    js_files = []
    
    # Directories to exclude
    exclude_dirs = {
        'node_modules',
        'dist',
        'build',
        'coverage',
        '.git',
        'target',
        'js_backup'
    }
    
    for root, dirs, files in os.walk(root_dir):
        # Skip excluded directories
        dirs[:] = [d for d in dirs if d not in exclude_dirs]
        
        for file in files:
            if file.endswith('.js'):
                full_path = os.path.join(root, file)
                relative_path = os.path.relpath(full_path, root_dir)
                js_files.append(relative_path.replace('\\', '/'))
    
    return js_files

def find_tracked_js_files(tracking_doc_path):
    """Find all JavaScript files mentioned in the tracking document."""
    if not os.path.exists(tracking_doc_path):
        return []
    
    with open(tracking_doc_path, 'r', encoding='utf-8') as f:
        content = f.read()
    
    # Extract all JavaScript files from the tracking document
    js_pattern = r'(?:\[[\sx]\])?\s*([\w\./\-]+\.js)'
    return re.findall(js_pattern, content)

def check_for_missing_files():
    """Check for JavaScript files that might have been missed in the migration."""
    # Paths
    project_root = os.path.dirname(os.path.dirname(os.path.dirname(__file__)))
    tracking_doc = os.path.join(project_root, "JavaScript to Rust Migration Tracking.md")
    
    # Find all JavaScript files in the project
    print(f"Scanning for JavaScript files in: {project_root}")
    all_js_files = find_all_js_files(project_root)
    
    # Find all JavaScript files mentioned in the tracking document
    tracked_js_files = find_tracked_js_files(tracking_doc)
    
    # Find files that exist but aren't tracked
    untracked_files = [js for js in all_js_files if not any(
        tracked.endswith(js) or js.endswith(tracked) for tracked in tracked_js_files
    )]
    
    # Categorize the files
    config_files = []
    possible_missed_files = []
    
    for js_file in untracked_files:
        filename = os.path.basename(js_file)
        if filename in ['webpack.config.js', 'babel.config.js', 'jest.config.js', 'rollup.config.js']:
            config_files.append(js_file)
        elif 'test' in js_file.lower() or 'spec' in js_file.lower():
            # These are test files, which you mentioned should be migrated last
            possible_missed_files.append((js_file, 'Test file'))
        else:
            # Check file content to determine if it's a module that needs migration
            file_path = os.path.join(project_root, js_file)
            try:
                with open(file_path, 'r', encoding='utf-8') as f:
                    content = f.read()
                    
                    # Check for imports, exports, or core functionality
                    if (re.search(r'(import|export|require|module\.exports|class\s+\w+|function\s+\w+\s*\()', content) 
                        and not re.search(r'^\s*/\*\*\s*@deprecated', content)):
                        possible_missed_files.append((js_file, 'Source file with imports/exports'))
            except Exception as e:
                print(f"Error reading {js_file}: {e}")
    
    # Print results
    print(f"\nFound {len(all_js_files)} JavaScript files in total")
    print(f"Found {len(tracked_js_files)} JavaScript files in the tracking document")
    
    print(f"\nUntracked files: {len(untracked_files)}")
    
    if config_files:
        print(f"\nConfiguration files (no need to migrate):")
        for file in config_files:
            print(f"  - {file}")
    
    if possible_missed_files:
        print(f"\nPOTENTIAL MISSED FILES THAT MIGHT NEED MIGRATION:")
        for file, reason in possible_missed_files:
            print(f"  - {file} ({reason})")
    else:
        print("\nNo missed files found! Migration appears to be complete.")
    
    return possible_missed_files

if __name__ == "__main__":
    check_for_missing_files()