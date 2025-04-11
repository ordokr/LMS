#!/usr/bin/env python3
"""
Script to clean up JavaScript files that have been migrated to Rust.
"""

import os
import re
import sys
import shutil
import argparse
from datetime import datetime

def parse_args():
    parser = argparse.ArgumentParser(description='Clean up migrated JavaScript files')
    parser.add_argument('--tracking-doc', default='JavaScript to Rust Migration Tracking.md',
                        help='Path to the migration tracking document')
    parser.add_argument('--backup-dir', default='js_backup',
                        help='Directory to store backups of deleted JS files')
    parser.add_argument('--dry-run', action='store_true',
                        help='Perform a dry run without actually deleting files')
    parser.add_argument('--force', action='store_true',
                        help='Force deletion without confirming each file')
    return parser.parse_args()

def extract_completed_migrations(tracking_doc):
    """Extract completed migrations from the tracking document."""
    if not os.path.exists(tracking_doc):
        print(f"Error: Tracking document not found: {tracking_doc}")
        return []
    
    with open(tracking_doc, 'r', encoding='utf-8') as f:
        content = f.read()
    
    # Find completed migrations (marked with [x])
    completed_regex = r'\[\s*x\s*\]\s*(.*?\.js)\s*\|'
    matches = re.findall(completed_regex, content)
    
    return matches

def backup_file(file_path, backup_dir):
    """Create a backup of a file before deletion."""
    if not os.path.exists(backup_dir):
        os.makedirs(backup_dir)
    
    # Create a backup path with timestamp
    timestamp = datetime.now().strftime('%Y%m%d%H%M%S')
    filename = os.path.basename(file_path)
    backup_path = os.path.join(backup_dir, f"{filename}.{timestamp}.bak")
    
    # Copy the file to backup location
    shutil.copy2(file_path, backup_path)
    
    return backup_path

def clean_migrated_files(completed_js_files, backup_dir, dry_run=False, force=False):
    """Clean up JavaScript files that have been migrated."""
    cleaned_files = []
    skipped_files = []
    not_found_files = []
    
    for js_file in completed_js_files:
        if os.path.exists(js_file):
            print(f"Found completed migration: {js_file}")
            
            if not force:
                confirm = input(f"Delete this file? (y/n): ").lower() == 'y'
            else:
                confirm = True
                
            if confirm:
                if not dry_run:
                    # Backup the file
                    backup_path = backup_file(js_file, backup_dir)
                    print(f"  Backed up to: {backup_path}")
                    
                    # Delete the original file
                    os.remove(js_file)
                    print(f"  Deleted: {js_file}")
                else:
                    print(f"  [DRY RUN] Would delete: {js_file}")
                
                cleaned_files.append(js_file)
            else:
                print(f"  Skipped: {js_file}")
                skipped_files.append(js_file)
        else:
            print(f"Completed file not found: {js_file}")
            not_found_files.append(js_file)
    
    return cleaned_files, skipped_files, not_found_files

def main():
    args = parse_args()
    
    print(f"Cleaning up migrated JavaScript files...")
    print(f"Using tracking document: {args.tracking_doc}")
    print(f"Backup directory: {args.backup_dir}")
    if args.dry_run:
        print("DRY RUN MODE: No files will be deleted")
    
    # Extract completed migrations
    completed_js_files = extract_completed_migrations(args.tracking_doc)
    print(f"Found {len(completed_js_files)} completed migrations in tracking document")
    
    # Clean up migrated files
    cleaned, skipped, not_found = clean_migrated_files(
        completed_js_files, args.backup_dir, args.dry_run, args.force
    )
    
    # Print summary
    print("\nCleanup Summary:")
    print(f"  Cleaned: {len(cleaned)} files")
    print(f"  Skipped: {len(skipped)} files")
    print(f"  Not found: {len(not_found)} files")
    
    if args.dry_run:
        print("\nNOTE: This was a dry run. Run without --dry-run to actually delete files.")
    
    return 0

if __name__ == "__main__":
    sys.exit(main())