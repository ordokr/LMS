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

def find_canvas_equivalent(search_term):
    canvas_dir = r"C:\Users\Tim\Desktop\port\canvas"
    
    if not os.path.exists(canvas_dir):
        print(f"Error: Canvas source directory not found at {canvas_dir}")
        return
    
    print(f"Searching for '{search_term}' in Canvas LMS source code...\n")
    
    # Search in models
    model_results = search_in_files(search_term, os.path.join(canvas_dir, "app", "models"))
    
    # Search in controllers
    controller_results = search_in_files(search_term, os.path.join(canvas_dir, "app", "controllers"))
    
    # Search in API
    api_results = search_in_files(search_term, os.path.join(canvas_dir, "app", "controllers", "api"))
    
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
    
    if api_results:
        print(f"\nFound in API ({len(api_results)} matches):")
        for result in api_results:
            print(f"\nFile: {result['file']}, Line: {result['line']}")
            print("-" * 60)
            for line in result['context']:
                print(f"  {line}")
            print("-" * 60)
    
    if not model_results and not controller_results and not api_results:
        print("No matches found in Canvas source code.")
    else:
        total = len(model_results) + len(controller_results) + len(api_results)
        print(f"\nFound {total} total matches for '{search_term}'")
        
        # Map to our structure
        print("\nRecommended mapping to our codebase:")
        if any('user' in r['file'].lower() for r in model_results):
            print("- Add to src/models/auth.rs")
        if any('course' in r['file'].lower() or 'assignment' in r['file'].lower() for r in model_results):
            print("- Add to src/models/lms.rs")
        if any('controller' in r['file'].lower() for r in controller_results):
            print("- Add functionality to src/services/lms_service.rs")
        if any('api' in r['file'].lower() for r in api_results):
            print("- Add API endpoint to src-tauri/src/api/")

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: python find_canvas_equivalent.py <search_term>")
        sys.exit(1)
    
    search_term = sys.argv[1]
    find_canvas_equivalent(search_term)