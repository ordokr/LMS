import os
import re
import shutil
from datetime import datetime

def find_migrated_js_files(tracking_doc_path):
    """Find all migrated JavaScript files from the tracking document."""
    try:
        with open(tracking_doc_path, 'r', encoding='utf-8') as f:
            content = f.read()
    except FileNotFoundError:
        print(f"Error: Tracking document not found at {tracking_doc_path}")
        return []
    
    # Extract completed migrations (marked with [x])
    pattern = r'\[\s*x\s*\]\s*(.*?\.js)\s*\|'
    matches = re.findall(pattern, content)
    return matches

def backup_and_delete_js_files(js_files, backup_dir, dry_run=True):
    """Backup and delete JavaScript files."""
    if not os.path.exists(backup_dir) and not dry_run:
        os.makedirs(backup_dir)
    
    results = {"backed_up": [], "deleted": [], "not_found": []}
    
    for js_file in js_files:
        # Make paths relative to the project root
        full_path = os.path.join(os.path.dirname(os.path.dirname(os.path.dirname(__file__))), js_file)
        
        if os.path.exists(full_path):
            print(f"Found: {js_file}")
            
            if not dry_run:
                # Create backup with timestamp
                timestamp = datetime.now().strftime('%Y%m%d_%H%M%S')
                filename = os.path.basename(js_file)
                backup_path = os.path.join(backup_dir, f"{filename}.{timestamp}.bak")
                
                # Copy file to backup location
                shutil.copy2(full_path, backup_path)
                results["backed_up"].append(backup_path)
                print(f"  Backed up to: {backup_path}")
                
                # Delete the original
                os.remove(full_path)
                results["deleted"].append(js_file)
                print(f"  Deleted: {js_file}")
            else:
                print(f"  [DRY RUN] Would backup and delete {js_file}")
        else:
            print(f"Not found: {js_file}")
            results["not_found"].append(js_file)
    
    return results

def main():
    # Path is relative to where the script is located
    tracking_doc = os.path.join(os.path.dirname(os.path.dirname(os.path.dirname(__file__))), 
                               "JavaScript to Rust Migration Tracking.md")
    backup_dir = os.path.join(os.path.dirname(os.path.dirname(os.path.dirname(__file__))), "js_backup")
    dry_run = False  # Set to False to actually delete files
    
    print(f"JavaScript to Rust Migration Cleanup")
    print(f"Reading migration tracking from: {tracking_doc}")
    print(f"Backup directory: {backup_dir}")
    if dry_run:
        print("DRY RUN MODE: No files will be deleted")
    
    # Find migrated JavaScript files
    js_files = find_migrated_js_files(tracking_doc)
    print(f"Found {len(js_files)} migrated JavaScript files")
    
    # Backup and delete files
    results = backup_and_delete_js_files(js_files, backup_dir, dry_run)
    
    # Print summary
    print("\nSummary:")
    print(f"  Files backed up: {len(results['backed_up'])}")
    print(f"  Files deleted: {len(results['deleted'])}")
    print(f"  Files not found: {len(results['not_found'])}")
    
    if dry_run:
        print("\nThis was a dry run. Edit this script and set dry_run=False to actually delete files.")

if __name__ == "__main__":
    main()