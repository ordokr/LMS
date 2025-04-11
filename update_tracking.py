import re
import os
from datetime import datetime

def count_migrations(tracking_doc_path):
    """Count migrations by status in the tracking document."""
    try:
        with open(tracking_doc_path, 'r', encoding='utf-8') as f:
            content = f.read()
    except FileNotFoundError:
        print(f"Error: Tracking document not found at {tracking_doc_path}")
        return {}
    
    # Count completed migrations
    completed_pattern = r'\[\s*x\s*\]\s*(.*?\.js)\s*\|'
    completed = re.findall(completed_pattern, content)
    
    # Count in-progress migrations
    in_progress_pattern = r'\[\s*\s*\]\s*(.*?\.js)\s*\|'
    in_progress = re.findall(in_progress_pattern, content)
    
    return {
        "completed": len(completed),
        "in_progress": len(in_progress),
        "total": len(completed) + len(in_progress)
    }

def update_tracking_document(tracking_doc_path, stats):
    """Update the tracking document with completion statistics."""
    try:
        with open(tracking_doc_path, 'r', encoding='utf-8') as f:
            content = f.readlines()
    except FileNotFoundError:
        print(f"Error: Tracking document not found at {tracking_doc_path}")
        return False
    
    # Add or update statistics section
    stats_section = [
        "\n## Migration Progress\n\n",
        f"- Total JavaScript files: {stats['total']}\n",
        f"- Migration completed: {stats['completed']} ({stats['completed']*100/max(stats['total'], 1):.1f}%)\n",
        f"- Migration in progress: {stats['in_progress']}\n",
        f"- Last updated: {datetime.now().strftime('%Y-%m-%d %H:%M')}\n\n"
    ]
    
    # Check if stats section already exists
    stats_start = -1
    stats_end = -1
    for i, line in enumerate(content):
        if "## Migration Progress" in line:
            stats_start = i
        elif stats_start > -1 and i > stats_start and line.startswith("##"):
            stats_end = i
            break
    
    # Insert or replace stats section
    if stats_start > -1:
        if stats_end > -1:
            content = content[:stats_start] + stats_section + content[stats_end:]
        else:
            content = content[:stats_start] + stats_section
    else:
        # Insert after the first section
        first_heading = 0
        for i, line in enumerate(content):
            if line.startswith("# "):
                first_heading = i
                break
        
        # Find the next empty line after the heading
        for i in range(first_heading + 1, len(content)):
            if content[i].strip() == "":
                content = content[:i+1] + stats_section + content[i+1:]
                break
    
    # Write updated content back to file
    try:
        with open(tracking_doc_path, 'w', encoding='utf-8') as f:
            f.writelines(content)
        return True
    except Exception as e:
        print(f"Error updating tracking document: {e}")
        return False

def main():
    tracking_doc = "JavaScript to Rust Migration Tracking.md"
    
    print(f"Updating migration tracking document: {tracking_doc}")
    
    # Count migrations
    stats = count_migrations(tracking_doc)
    print(f"Found {stats['completed']} completed migrations out of {stats['total']} total")
    
    # Update tracking document
    if update_tracking_document(tracking_doc, stats):
        print(f"Successfully updated {tracking_doc}")
    else:
        print(f"Failed to update {tracking_doc}")

if __name__ == "__main__":
    main()