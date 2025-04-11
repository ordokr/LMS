#!/usr/bin/env python3
"""
Script to clean up JavaScript files that have been successfully migrated to Rust.
This script creates backups of all files before deletion and logs all operations.
"""

import os
import re
import sys
import shutil
import argparse
import json
from datetime import datetime
from pathlib import Path

def parse_args():
    parser = argparse.ArgumentParser(description='Clean up migrated JavaScript files')
    parser.add_argument('--tracking-doc', default='JavaScript to Rust Migration Tracking.md',
                        help='Path to the migration tracking document')
    parser.add_argument('--backup-dir', default='js_backup',
                        help='Directory to store backups of deleted JS files')
    parser.add_argument('--log-file', default='migration_cleanup.log',
                        help='File to log cleanup operations')
    parser.add_argument('--dry-run', action='store_true',
                        help='Perform a dry run without actually deleting files')
    parser.add_argument('--force', action='store_true',
                        help='Force deletion without confirming each file')
    parser.add_argument('--report', action='store_true',
                        help='Generate a JSON report of cleanup operations')
    return parser.parse_args()

def setup_logger(log_file):
    """Setup a simple logger to track operations."""
    import logging
    logging.basicConfig(
        filename=log_file,
        level=logging.INFO,
        format='%(asctime)s - %(levelname)s - %(message)s',
        datefmt='%Y-%m-%d %H:%M:%S'
    )
    return logging.getLogger('migration_cleanup')

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

def create_directory_structure(backup_dir):
    """Create a structured backup directory based on date."""
    timestamp = datetime.now().strftime('%Y%m%d_%H%M%S')
    backup_path = os.path.join(backup_dir, timestamp)
    os.makedirs(backup_path, exist_ok=True)
    return backup_path

def backup_file(file_path, backup_dir):
    """Create a backup of a file before deletion, preserving directory structure."""
    # Create a relative path for the backup
    rel_path = os.path.normpath(file_path)
    if os.path.isabs(rel_path):
        # If absolute path, strip the drive letter (on Windows) and leading separator
        if os.name == 'nt' and rel_path[1:].startswith(':\\'):
            rel_path = os.path.splitdrive(rel_path)[1].lstrip('\\/')
        else:
            rel_path = rel_path.lstrip('\\/')
    
    # Create the full backup path
    backup_path = os.path.join(backup_dir, rel_path)
    
    # Ensure the directory structure exists
    os.makedirs(os.path.dirname(backup_path), exist_ok=True)
    
    # Copy the file to backup location
    shutil.copy2(file_path, backup_path)
    
    return backup_path

def clean_migrated_files(completed_js_files, backup_dir, logger, dry_run=False, force=False):
    """Clean up JavaScript files that have been migrated."""
    cleaned_files = []
    skipped_files = []
    not_found_files = []
    
    # Create backup directory structure
    timestamp_backup_dir = create_directory_structure(backup_dir)
    
    for js_file in completed_js_files:
        # Normalize path separators
        js_file = os.path.normpath(js_file)
        
        if os.path.exists(js_file):
            print(f"Found completed migration: {js_file}")
            logger.info(f"Found completed migration: {js_file}")
            
            if not force:
                confirm = input(f"Delete this file? (y/n): ").lower() == 'y'
            else:
                confirm = True
                
            if confirm:
                if not dry_run:
                    # Backup the file
                    backup_path = backup_file(js_file, timestamp_backup_dir)
                    print(f"  Backed up to: {backup_path}")
                    logger.info(f"Backed up {js_file} to {backup_path}")
                    
                    # Delete the original file
                    os.remove(js_file)
                    print(f"  Deleted: {js_file}")
                    logger.info(f"Deleted {js_file}")
                else:
                    print(f"  [DRY RUN] Would delete: {js_file}")
                    logger.info(f"[DRY RUN] Would delete: {js_file}")
                
                cleaned_files.append(js_file)
            else:
                print(f"  Skipped: {js_file}")
                logger.info(f"Skipped (user choice): {js_file}")
                skipped_files.append(js_file)
        else:
            print(f"Completed file not found: {js_file}")
            logger.warning(f"Completed file not found: {js_file}")
            not_found_files.append(js_file)
    
    return cleaned_files, skipped_files, not_found_files, timestamp_backup_dir

def generate_report(cleaned, skipped, not_found, backup_dir, dry_run):
    """Generate a JSON report of the cleanup operation."""
    report = {
        "timestamp": datetime.now().isoformat(),
        "cleanupType": "dry_run" if dry_run else "actual",
        "backupDirectory": backup_dir,
        "statistics": {
            "cleaned": len(cleaned),
            "skipped": len(skipped),
            "notFound": len(not_found),
            "total": len(cleaned) + len(skipped) + len(not_found)
        },
        "files": {
            "cleaned": cleaned,
            "skipped": skipped,
            "notFound": not_found
        }
    }
    
    # Save the report to a file
    report_file = f"cleanup_report_{datetime.now().strftime('%Y%m%d_%H%M%S')}.json"
    with open(report_file, 'w', encoding='utf-8') as f:
        json.dump(report, f, indent=2)
    
    print(f"Generated report: {report_file}")
    return report_file

def main():
    args = parse_args()
    
    # Setup logger
    logger = setup_logger(args.log_file)
    logger.info(f"Starting JavaScript cleanup process")
    
    print(f"Cleaning up migrated JavaScript files...")
    print(f"Using tracking document: {args.tracking_doc}")
    print(f"Backup directory: {args.backup_dir}")
    if args.dry_run:
        print("DRY RUN MODE: No files will be deleted")
    
    # Extract completed migrations
    completed_js_files = extract_completed_migrations(args.tracking_doc)
    print(f"Found {len(completed_js_files)} completed migrations in tracking document")
    logger.info(f"Found {len(completed_js_files)} completed migrations in tracking document")
    
    # Clean up migrated files
    cleaned, skipped, not_found, backup_dir = clean_migrated_files(
        completed_js_files, args.backup_dir, logger, args.dry_run, args.force
    )
    
    # Print summary
    print("\nCleanup Summary:")
    print(f"  Cleaned: {len(cleaned)} files")
    print(f"  Skipped: {len(skipped)} files")
    print(f"  Not found: {len(not_found)} files")
    
    logger.info(f"Cleanup completed - Cleaned: {len(cleaned)}, Skipped: {len(skipped)}, Not found: {len(not_found)}")
    
    if args.dry_run:
        print("\nNOTE: This was a dry run. Run without --dry-run to actually delete files.")
    
    # Generate report if requested
    if args.report:
        report_file = generate_report(cleaned, skipped, not_found, backup_dir, args.dry_run)
        logger.info(f"Generated report: {report_file}")
    
    return 0

if __name__ == "__main__":
    sys.exit(main())