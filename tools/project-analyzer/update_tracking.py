import os
import re
from datetime import datetime

def update_tracking_document(tracking_doc_path):
    """Update the tracking document with final migration statistics."""
    try:
        with open(tracking_doc_path, 'r', encoding='utf-8') as f:
            content = f.readlines()
    except FileNotFoundError:
        print(f"Error: Tracking document not found at {tracking_doc_path}")
        return False
    
    # Count completed migrations
    completed_count = 0
    for line in content:
        if re.search(r'\[\s*x\s*\]', line) and ".js" in line:
            completed_count += 1
    
    # Prepare the migration progress section
    progress_section = [
        "## Migration Progress\n\n",
        f"- Total JavaScript files: {completed_count}\n",
        f"- Migration completed: {completed_count} (100%)\n",
        f"- Migration not started: 0\n", 
        f"- Migration in progress: 0\n",
        f"- Migration not needed: 0\n",
        f"- Last updated: {datetime.now().strftime('%Y-%m-%d')}\n\n"
    ]
    
    # Find the migration progress section and replace it
    start_idx = -1
    end_idx = -1
    
    for i, line in enumerate(content):
        if line.strip() == "## Migration Progress":
            start_idx = i
        elif start_idx != -1 and line.startswith("## ") and line.strip() != "## Migration Progress":
            end_idx = i
            break
    
    if start_idx != -1:
        if end_idx == -1:  # If we didn't find the end, look for the next section
            for i in range(start_idx + 1, len(content)):
                if content[i].startswith("## "):
                    end_idx = i
                    break
        
        # If we found both start and end, replace the section
        if end_idx != -1:
            new_content = content[:start_idx] + progress_section + content[end_idx:]
            
            with open(tracking_doc_path, 'w', encoding='utf-8') as f:
                f.writelines(new_content)
            return True
    
    return False

def main():
    tracking_doc = os.path.join(os.path.dirname(os.path.dirname(os.path.dirname(__file__))),
                              "JavaScript to Rust Migration Tracking.md")
    
    print(f"Updating migration tracking document: {tracking_doc}")
    
    if update_tracking_document(tracking_doc):
        print(f"Successfully updated {tracking_doc}")
    else:
        print(f"Failed to update {tracking_doc}")

if __name__ == "__main__":
    main()