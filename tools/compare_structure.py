import os
import json

def analyze_structure(path, ignore_dirs=None):
    """Analyze project structure recursively"""
    if ignore_dirs is None:
        ignore_dirs = ['.git', 'node_modules', 'tmp']
    
    structure = {}
    
    for root, dirs, files in os.walk(path):
        dirs[:] = [d for d in dirs if d not in ignore_dirs]
        rel_path = os.path.relpath(root, path)
        if rel_path == '.':
            rel_path = ''
            
        structure[rel_path] = {
            "files": files,
            "dirs": [d for d in dirs]
        }
    
    return structure

# Analyze Canvas
canvas_structure = analyze_structure("C:/Users/Tim/Desktop/port/canvas")

# Analyze Discourse
discourse_structure = analyze_structure("C:/Users/Tim/Desktop/port/port")

# Analyze our project
our_structure = analyze_structure("C:/Users/Tim/Desktop/LMS")

# Output results
with open("structure_comparison.json", "w") as f:
    json.dump({
        "canvas": canvas_structure,
        "discourse": discourse_structure,
        "our_project": our_structure
    }, f, indent=2)

print("Structure comparison complete. See structure_comparison.json")