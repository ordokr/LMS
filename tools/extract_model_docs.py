import os
import re
import sys
import subprocess
from find_model_file import find_model_file, camel_to_snake

def extract_ruby_docs(file_path):
    """Extract documentation from a Ruby file."""
    if not os.path.exists(file_path):
        return "File not found"
    
    try:
        with open(file_path, 'r', encoding='utf-8', errors='ignore') as f:
            content = f.read()
    except Exception as e:
        return f"Error reading file: {e}"
    
    # Extract class name
    class_match = re.search(r'class\s+(\w+)', content)
    class_name = class_match.group(1) if class_match else "Unknown"
    
    # Extract comments before class definition
    class_pos = content.find(f"class {class_name}")
    if class_pos > 0:
        comments_text = content[:class_pos].strip()
        comments = [line for line in comments_text.split("\n") if line.strip() and not line.strip().startswith("#!")]
    else:
        comments = []
    
    # Extract method documentation
    methods = []
    method_matches = re.finditer(r'^\s*#\s*(.*?)\n\s*def\s+(\w+)', content, re.MULTILINE | re.DOTALL)
    for match in method_matches:
        doc = match.group(1).strip()
        method_name = match.group(2)
        if doc and not method_name.startswith("_"):
            methods.append({
                "name": method_name,
                "doc": doc
            })
    
    # Generate markdown
    markdown = f"# {class_name}\n\n"
    
    if comments:
        markdown += "## Description\n\n"
        for comment in comments:
            clean_comment = comment.strip()
            if clean_comment.startswith('#'):
                clean_comment = clean_comment[1:].strip()
            markdown += f"{clean_comment}\n"
        markdown += "\n"
    
    # Add schema info if available
    schema_match = re.search(r'(?:create_table|add_column).*?:(\w+)', content)
    if schema_match:
        table_name = schema_match.group(1)
        markdown += f"## Database Table\n\n`{table_name}`\n\n"
    
    # Add relations
    relations = []
    relation_matches = re.finditer(r'(has_many|has_one|belongs_to)\s+:(\w+)', content)
    for match in relation_matches:
        rel_type = match.group(1)
        rel_target = match.group(2)
        relations.append({
            "type": rel_type,
            "target": rel_target
        })
    
    if relations:
        markdown += "## Relationships\n\n"
        for relation in relations:
            markdown += f"- {relation['type']} :{relation['target']}\n"
        markdown += "\n"
    
    # Add methods
    if methods:
        markdown += "## Methods\n\n"
        for method in methods:
            markdown += f"### {method['name']}\n\n{method['doc']}\n\n"
    
    return markdown

def generate_model_docs():
    """Generate documentation for all models."""
    # Create docs directory
    docs_dir = "docs/models"
    os.makedirs(docs_dir, exist_ok=True)
    
    # Scan Rust models
    rust_models = []
    for root, _, files in os.walk("src/models"):
        for file in files:
            if not file.endswith(".rs"):
                continue
                
            file_path = os.path.join(root, file)
            with open(file_path, 'r', encoding='utf-8') as f:
                content = f.read()
                
            # Find struct definitions
            struct_matches = re.finditer(r'pub struct ([A-Z][a-zA-Z0-9_]*)', content)
            for match in struct_matches:
                struct_name = match.group(1)
                rust_models.append({
                    "name": struct_name,
                    "file": file_path
                })
    
    # Generate README.md for models
    with open(f"{docs_dir}/README.md", "w", encoding='utf-8') as readme:
        readme.write("# Model Documentation\n\n")
        readme.write("This directory contains documentation extracted from the original Canvas and Discourse source code.\n\n")
        readme.write("## Models\n\n")
        
        for model in sorted(rust_models, key=lambda x: x['name']):
            # Find source files
            canvas_file = find_model_file(model['name'], "canvas")
            discourse_file = find_model_file(model['name'], "discourse")
            
            readme.write(f"### {model['name']}\n\n")
            readme.write(f"- [Our Implementation]({os.path.relpath(model['file'], 'docs/models')})\n")
            
            if canvas_file:
                canvas_doc_path = f"{camel_to_snake(model['name'])}_canvas.md"
                readme.write(f"- [Canvas Source]({canvas_doc_path})\n")
                
                # Generate Canvas doc
                canvas_doc = extract_ruby_docs(canvas_file)
                with open(f"{docs_dir}/{canvas_doc_path}", "w", encoding='utf-8') as f:
                    f.write(canvas_doc)
            
            if discourse_file:
                discourse_doc_path = f"{camel_to_snake(model['name'])}_discourse.md"
                readme.write(f"- [Discourse Source]({discourse_doc_path})\n")
                
                # Generate Discourse doc
                discourse_doc = extract_ruby_docs(discourse_file)
                with open(f"{docs_dir}/{discourse_doc_path}", "w", encoding='utf-8') as f:
                    f.write(discourse_doc)
            
            readme.write("\n")
    
    print(f"Generated documentation for {len(rust_models)} models in docs/models/")

if __name__ == "__main__":
    generate_model_docs()