import os
import re
from collections import defaultdict

def count_files_by_extension(root_dir, extensions=None):
    """Count files by extension in the project directory."""
    if extensions is None:
        extensions = ['.js', '.rs']
    
    file_counts = defaultdict(int)
    
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
    
    # Process files
    for root, dirs, files in os.walk(root_dir):
        # Skip excluded directories
        dirs[:] = [d for d in dirs if d not in exclude_dirs]
        
        for file in files:
            _, ext = os.path.splitext(file)
            if ext in extensions:
                file_counts[ext] += 1
    
    return file_counts

def analyze_file_organization():
    """Analyze and compare JavaScript and Rust file organization."""
    # Path to project root
    project_root = os.path.dirname(os.path.dirname(os.path.dirname(__file__)))
    
    # Count files by extension
    file_counts = count_files_by_extension(project_root)
    
    # Compare JavaScript and Rust files by directory
    dir_structure = defaultdict(lambda: {'js': 0, 'rs': 0})
    
    for root, _, files in os.walk(project_root):
        rel_path = os.path.relpath(root, project_root)
        if rel_path == '.':
            rel_path = 'root'
        
        # Skip excluded directories
        if any(excluded in root for excluded in [
            'node_modules', 'dist', 'build', 'coverage', '.git', 'target', 'js_backup'
        ]):
            continue
        
        # Count files
        js_count = sum(1 for f in files if f.endswith('.js'))
        rs_count = sum(1 for f in files if f.endswith('.rs'))
        
        if js_count > 0 or rs_count > 0:
            dir_structure[rel_path]['js'] = js_count
            dir_structure[rel_path]['rs'] = rs_count
    
    # Print results
    print("\nFile Count Verification:")
    print(f"Total JavaScript files: {file_counts.get('.js', 0)}")
    print(f"Total Rust files: {file_counts.get('.rs', 0)}")
    
    print("\nDirectory Analysis:")
    for dir_path, counts in sorted(dir_structure.items()):
        if counts['js'] > 0 or counts['rs'] > 0:
            print(f"  {dir_path}: {counts['js']} JS, {counts['rs']} RS")
    
    # Highlight directories with JavaScript files but no Rust files
    js_only_dirs = [dir_path for dir_path, counts in dir_structure.items() 
                   if counts['js'] > 0 and counts['rs'] == 0]
    
    if js_only_dirs:
        print("\nWARNING: These directories have JavaScript files but no Rust files:")
        for dir_path in js_only_dirs:
            print(f"  - {dir_path} ({dir_structure[dir_path]['js']} JS files)")
    else:
        print("\nAll directories with JavaScript files also have Rust files!")

if __name__ == "__main__":
    analyze_file_organization()