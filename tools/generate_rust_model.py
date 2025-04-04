import sys
import os
import re
from find_model_file import find_model_file, camel_to_snake

def extract_fields_from_ruby_file(file_path):
    """Extract fields from a Ruby model file."""
    fields = []
    relationships = []
    methods = []
    
    if not os.path.exists(file_path):
        return fields, relationships, methods
    
    with open(file_path, 'r', encoding='utf-8', errors='ignore') as f:
        content = f.read()
    
    # Find class name
    class_match = re.search(r'class\s+(\w+)', content)
    class_name = class_match.group(1) if class_match else "UnknownClass"
    
    # Find attr_accessor and similar definitions
    attr_patterns = [
        r'attr_accessor\s+:([\w_,\s]+)',
        r'attr_reader\s+:([\w_,\s]+)',
        r'attr_writer\s+:([\w_,\s]+)'
    ]
    
    for pattern in attr_patterns:
        for match in re.finditer(pattern, content):
            attrs = match.group(1).split(',')
            for attr in attrs:
                attr = attr.strip()
                if attr:
                    fields.append(attr)
    
    # Find db columns (if any)
    db_columns = re.findall(r't\.(\w+)\s+:(\w+)', content)
    for col_type, col_name in db_columns:
        fields.append(col_name)
    
    # Find relationships
    relation_patterns = [
        r'has_many\s+:([\w_]+)',
        r'has_one\s+:([\w_]+)',
        r'belongs_to\s+:([\w_]+)'
    ]
    
    for pattern in relation_patterns:
        for match in re.finditer(pattern, content):
            rel = match.group(1).strip()
            if rel:
                relationships.append(rel)
    
    # Find methods
    method_matches = re.finditer(r'def\s+(\w+)', content)
    for match in method_matches:
        method_name = match.group(1)
        if not method_name.startswith('_'):
            methods.append(method_name)
    
    return fields, relationships, methods, class_name

def generate_rust_field(name, is_optional=True):
    """Generate a Rust field from a Ruby field name."""
    # Map Ruby types to Rust types (simplified)
    if name in ["id", "position"]:
        return f"pub {name}: {'Option<i64>' if is_optional else 'i64'},"
    elif name.endswith("_id"):
        return f"pub {name}: {'Option<i64>' if is_optional else 'i64'},"
    elif name.endswith("_at"):
        return f"pub {name}: Option<String>, // DateTime"
    elif name.endswith("_count"):
        return f"pub {name}: Option<i32>,"
    elif name in ["created_at", "updated_at"]:
        return f"pub {name}: Option<String>,"
    elif name in ["title", "name", "description"]:
        return f"pub {name}: {'Option<String>' if is_optional else 'String'},"
    else:
        return f"pub {name}: Option<String>, // Adjust type as needed"

def generate_rust_model(model_name, source_system="canvas"):
    """Generate a Rust model file from a Canvas/Discourse model."""
    # Find the source file
    source_file = find_model_file(model_name, source_system)
    if not source_file:
        print(f"No source file found for {model_name}")
        return
    
    print(f"Found source file: {source_file}")
    fields, relationships, methods, class_name = extract_fields_from_ruby_file(source_file)
    
    # Create Rust model
    rust_model = f"""// Auto-generated from {os.path.basename(source_file)}
// Source: {source_file}

use serde::{{Deserialize, Serialize}};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// {class_name} model - ported from {source_system.capitalize()}
/// Reference: {source_file}
pub struct {model_name} {{
    // Fields
"""
    
    # Add fields
    for field in sorted(set(fields)):
        rust_model += f"    {generate_rust_field(field)}\n"
    
    # Close the struct
    rust_model += "}\n\n"
    
    # Add implementation with methods
    rust_model += f"impl {model_name} {{\n"
    rust_model += f"    pub fn new() -> Self {{\n"
    rust_model += f"        Self {{\n"
    
    # Default values for fields
    for field in sorted(set(fields)):
        if field == "id":
            rust_model += "            id: 0,\n"
        else:
            rust_model += f"            {field}: None,\n"
    
    rust_model += "        }\n"
    rust_model += "    }\n\n"
    
    # Add method stubs
    for method in methods[:10]:  # Limit to 10 methods to avoid clutter
        rust_model += f"    // TODO: Implement {method} from {class_name}\n"
        rust_model += f"    pub fn {method}(&self) -> bool {{\n"
        rust_model += "        // Implementation needed\n"
        rust_model += "        false\n"
        rust_model += "    }\n\n"
    
    rust_model += "}\n"
    
    # Write to file
    output_dir = os.path.join("src", "models")
    if not os.path.exists(output_dir):
        os.makedirs(output_dir)
    
    model_file = snake_case(model_name)
    output_path = os.path.join(output_dir, f"{model_file}.rs")
    
    # Check if file already exists
    if os.path.exists(output_path):
        print(f"File {output_path} already exists. Adding _new suffix.")
        output_path = os.path.join(output_dir, f"{model_file}_new.rs")
    
    with open(output_path, 'w', encoding='utf-8') as f:
        f.write(rust_model)
    
    print(f"Generated model file: {output_path}")
    
    # Update mapping.md
    update_mapping_md(model_name, class_name, source_system, fields, len(methods))

def update_mapping_md(model_name, class_name, source_system, fields, method_count):
    """Update the mapping.md file with the new model."""
    mapping_path = "mapping.md"
    
    if not os.path.exists(mapping_path):
        print("mapping.md not found, skipping update")
        return
    
    with open(mapping_path, 'r', encoding='utf-8') as f:
        content = f.readlines()
    
    # Find the Core Models section
    core_models_idx = -1
    for i, line in enumerate(content):
        if "## Core Models" in line:
            core_models_idx = i
            break
    
    if core_models_idx == -1:
        print("Could not find Core Models section in mapping.md")
        return
    
    # Find the table header
    table_start_idx = -1
    for i in range(core_models_idx, len(content)):
        if "|--" in content[i]:
            table_start_idx = i - 1
            break
    
    if table_start_idx == -1:
        print("Could not find table in Core Models section")
        return
    
    # Create the new entry
    table_entry = f"| {source_system.capitalize()} `{class_name}` | `models/{snake_case(model_name)}.rs:{model_name}` | 0% | Auto-generated with {len(fields)} fields, {method_count} methods |\n"
    
    # Insert the new entry after the table header
    content.insert(table_start_idx + 2, table_entry)
    
    # Write the updated content
    with open(mapping_path, 'w', encoding='utf-8') as f:
        f.writelines(content)
    
    print(f"Updated mapping.md with {model_name} entry")

def snake_case(name):
    """Convert CamelCase to snake_case."""
    name = re.sub('(.)([A-Z][a-z]+)', r'\1_\2', name)
    return re.sub('([a-z0-9])([A-Z])', r'\1_\2', name).lower()

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: python generate_rust_model.py <model_name> [canvas|discourse]")
        sys.exit(1)
    
    model_name = sys.argv[1]
    source_system = sys.argv[2] if len(sys.argv) > 2 else "canvas"
    
    generate_rust_model(model_name, source_system)