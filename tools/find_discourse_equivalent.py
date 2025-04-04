import sys
import os
import glob
import re

def search_in_files(search_term, directory, file_pattern="*.rb"):
    """Search for term in files matching pattern in the given directory."""
    results = []
    
    for root, _, _ in os.walk(directory):
        for file_path in glob.glob(os.path.join(root, file_pattern)):
            try:
                with open(file_path, 'r', encoding='utf-8', errors='ignore') as f:
                    content = f.read()
                    if search_term.lower() in content.lower():
                        # Get context around the term
                        lines = content.splitlines()
                        for i, line in enumerate(lines):
                            if search_term.lower() in line.lower():
                                context_start = max(0, i - 2)
                                context_end = min(len(lines), i + 3)
                                context = lines[context_start:context_end]
                                
                                rel_path = os.path.relpath(file_path, directory)
                                results.append({
                                    'file': rel_path,
                                    'line': i + 1,
                                    'context': context
                                })
            except Exception as e:
                print(f"Error reading {file_path}: {e}")
    
    return results

def find_discourse_equivalent(search_term):
    discourse_dir = r"C:\Users\Tim\Desktop\port\port"
    
    if not os.path.exists(discourse_dir):
        print(f"Error: Discourse source directory not found at {discourse_dir}")
        return
    
    print(f"Searching for '{search_term}' in Discourse source code...\n")
    
    # Search in models
    model_results = search_in_files(search_term, os.path.join(discourse_dir, "app", "models"))
    
    # Search in controllers
    controller_results = search_in_files(search_term, os.path.join(discourse_dir, "app", "controllers"))
    
    # Search in services
    service_results = search_in_files(search_term, os.path.join(discourse_dir, "app", "services"))
    
    # Display results
    if model_results:
        print(f"Found in Models ({len(model_results)} matches):")
        for result in model_results:
            print(f"\nFile: {result['file']}, Line: {result['line']}")
            print("-" * 60)
            for line in result['context']:
                print(f"  {line}")
            print("-" * 60)
    
    if controller_results:
        print(f"\nFound in Controllers ({len(controller_results)} matches):")
        for result in controller_results:
            print(f"\nFile: {result['file']}, Line: {result['line']}")
            print("-" * 60)
            for line in result['context']:
                print(f"  {line}")
            print("-" * 60)
    
    if service_results:
        print(f"\nFound in Services ({len(service_results)} matches):")
        for result in service_results:
            print(f"\nFile: {result['file']}, Line: {result['line']}")
            print("-" * 60)
            for line in result['context']:
                print(f"  {line}")
            print("-" * 60)
    
    if not model_results and not controller_results and not service_results:
        print("No matches found in Discourse source code.")
    else:
        total = len(model_results) + len(controller_results) + len(service_results)
        print(f"\nFound {total} total matches for '{search_term}'")
        
        # Map to our structure
        print("\nRecommended mapping to our codebase:")
        if any('user' in r['file'].lower() for r in model_results):
            print("- Add to src/models/auth.rs")
        if any(('topic' in r['file'].lower() or 'category' in r['file'].lower() or 
                'post' in r['file'].lower()) for r in model_results):
            print("- Add to src/models/forum.rs")
        if any('controller' in r['file'].lower() for r in controller_results):
            print("- Add functionality to src/services/forum_service.rs")
        if any('service' in r['file'].lower() for r in service_results):
            print("- Add service to src/services/")

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: python find_discourse_equivalent.py <search_term>")
        sys.exit(1)
    
    search_term = sys.argv[1]
    find_discourse_equivalent(search_term)