import os
import sys
import re

def find_model_file(model_name, source_system="canvas"):
    """Find the most likely source file for a model name."""
    # Determine source directory
    if source_system.lower() == "canvas":
        source_dir = r"C:\Users\Tim\Desktop\port\canvas\app\models"
    else:  # discourse
        source_dir = r"C:\Users\Tim\Desktop\port\port\app\models"
    
    # Check if source dir exists
    if not os.path.exists(source_dir):
        print(f"Source directory {source_dir} not found")
        return None
    
    # Try exact match first (model_name.rb)
    snake_case = camel_to_snake(model_name)
    exact_match = os.path.join(source_dir, f"{snake_case}.rb")
    if os.path.exists(exact_match):
        return exact_match
    
    # Try to find by class name in files
    for root, _, files in os.walk(source_dir):
        for file in files:
            if not file.endswith(".rb"):
                continue
            
            file_path = os.path.join(root, file)
            with open(file_path, 'r', encoding='utf-8', errors='ignore') as f:
                content = f.read()
                # Check for class definition
                if re.search(fr'class\s+{model_name}\b', content):
                    return file_path
    
    # If still not found, try partial matches in filenames
    potential_matches = []
    for root, _, files in os.walk(source_dir):
        for file in files:
            if not file.endswith(".rb"):
                continue
            
            if snake_case in file:
                potential_matches.append(os.path.join(root, file))
    
    if potential_matches:
        return potential_matches[0]  # Return first match
    
    return None

def camel_to_snake(name):
    """Convert CamelCase to snake_case."""
    name = re.sub('(.)([A-Z][a-z]+)', r'\1_\2', name)
    return re.sub('([a-z0-9])([A-Z])', r'\1_\2', name).lower()

if __name__ == "__main__":
    if len(sys.argv) < 3:
        print("Usage: python find_model_file.py <model_name> <source_system>")
        sys.exit(1)
    
    model_name = sys.argv[1]
    source_system = sys.argv[2]
    result = find_model_file(model_name, source_system)
    
    if result:
        print(result)
    else:
        print("None")