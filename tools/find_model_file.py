#!/usr/bin/env python
"""
Utility for finding the appropriate model file for a given model name
"""
import os
import re
import glob
from pathlib import Path

def camel_to_snake(name):
    """Convert CamelCase to snake_case."""
    name = re.sub('(.)([A-Z][a-z]+)', r'\1_\2', name)
    return re.sub('([a-z0-9])([A-Z])', r'\1_\2', name).lower()

def snake_to_camel(name):
    """Convert snake_case to CamelCase."""
    components = name.split('_')
    return ''.join(x.title() for x in components)

def find_model_file(model_name, source_system="canvas"):
    """Find a model file based on model name and source system."""
    # Try different casing conventions
    possible_names = [
        model_name,
        model_name.lower(),
        model_name.upper(),
        camel_to_snake(model_name),
        snake_to_camel(model_name)
    ]
    
    # Search directories based on source system
    search_dirs = []
    if source_system.lower() == "canvas":
        search_dirs = [
            "src/models/unifiedModels",
            "src/models/unified",
            "src/models/canvas"
        ]
    elif source_system.lower() == "discourse":
        search_dirs = [
            "src/models/discourse",
            "src/models/forum"
        ]
    else:
        search_dirs = [
            f"src/models/{source_system}",
            "src/models"
        ]
    
    # Add the source system name itself as a potential directory
    search_dirs.append(source_system)
    
    # Add any JS or TS directories
    search_dirs.extend([
        "src",
        "lms-integration",
        "modules"
    ])
    
    # Common file extensions
    extensions = [".js", ".ts", ".rb", ".py"]
    
    # Try to find the file
    for directory in search_dirs:
        if not os.path.exists(directory):
            continue
            
        for name in possible_names:
            for ext in extensions:
                # Direct match (with extension)
                path = os.path.join(directory, f"{name}{ext}")
                if os.path.exists(path):
                    return path
                    
                # Try with alternative casing
                for file_path in glob.glob(os.path.join(directory, f"*{ext}")):
                    filename = os.path.basename(file_path).split('.')[0]
                    if filename.lower() == name.lower():
                        return file_path
    
    # Try a more generic search if the direct approach fails
    for name in possible_names:
        # Look for files that might contain the model name
        for pattern in [f"**/*{name}*.*", f"**/{name}*.*"]:
            for file_path in glob.glob(pattern, recursive=True):
                if any(file_path.endswith(ext) for ext in extensions):
                    return file_path
    
    # Not found
    return None
