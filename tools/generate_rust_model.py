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

def extract_fields_from_js_file(file_path):
    """Extract fields from a JavaScript model file."""
    fields = []
    relationships = []
    methods = []
    field_types = {}  # Store field name -> detected type
    imports = []      # Store imports for context
    
    if not os.path.exists(file_path):
        return fields, relationships, methods, "UnknownClass", field_types, imports
    
    with open(file_path, 'r', encoding='utf-8', errors='ignore') as f:
        content = f.read()
    
    # Extract imports for context
    import_patterns = [
        r'import\s+{\s*([^}]+)\s*}\s+from\s+[\'"]([^\'"]+)[\'"]',  # import { X } from 'Y'
        r'import\s+(\w+)\s+from\s+[\'"]([^\'"]+)[\'"]',           # import X from 'Y'
        r'const\s+{\s*([^}]+)\s*}\s*=\s*require\([\'"]([^\'"]+)[\'"]\)',  # const { X } = require('Y')
        r'const\s+(\w+)\s*=\s*require\([\'"]([^\'"]+)[\'"]\)'     # const X = require('Y')
    ]
    
    for pattern in import_patterns:
        for match in re.finditer(pattern, content):
            if len(match.groups()) == 2:
                imports.append((match.group(1).strip(), match.group(2)))
    
    # Find class name (use filename if not found in content)
    class_name = os.path.basename(file_path).split('.')[0]
    class_match = re.search(r'class\s+(\w+)', content)
    if class_match:
        class_name = class_match.group(1)
    else:
        # Try to find export const/let/var ClassName
        export_match = re.search(r'(?:export\s+(?:default\s+)?)?(?:const|let|var)\s+(\w+)\s*=', content)
        if export_match:
            class_name = export_match.group(1)
    
    # Look for JSDoc comments for contextual information
    jsdoc_blocks = re.finditer(r'/\*\*\s*([\s\S]*?)\s*\*/', content)
    for block in jsdoc_blocks:
        jsdoc_content = block.group(1)
        
        # Look for @property or @param tags with types
        property_matches = re.finditer(r'@(?:property|param)\s+{([^}]+)}\s+(?:\[([^\]]+)\]|(\S+))', jsdoc_content)
        for prop_match in property_matches:
            prop_type = prop_match.group(1).strip()
            prop_name = prop_match.group(3) if prop_match.group(3) else prop_match.group(2)
            # Strip leading dots or other symbols
            prop_name = re.sub(r'^[^a-zA-Z0-9_]+', '', prop_name)
            if prop_name not in field_types:
                field_types[prop_name] = prop_type
    
    # Extract class properties/fields
    # ES6 class properties
    class_props = re.findall(r'(?:this\.)?(\w+)\s*=', content)
    fields.extend([prop for prop in class_props if prop not in ['constructor', 'super', 'prototype'] and not prop.startswith('_')])
    
    # Constructor assignments with type inference
    constructor_block = re.search(r'constructor\s*\([^)]*\)\s*{([^}]*)}', content)
    if constructor_block:
        constructor_content = constructor_block.group(1)
        # Look for this.x = y patterns
        for match in re.finditer(r'this\.(\w+)\s*=\s*([^;]+)', constructor_content):
            field_name = match.group(1)
            field_value = match.group(2).strip()
            
            if field_name not in fields and not field_name.startswith('_'):
                fields.append(field_name)
            
            # Try to infer type from assignment
            if field_name not in field_types:
                if field_value == '[]':
                    field_types[field_name] = 'Array'
                elif field_value == '{}':
                    field_types[field_name] = 'Object'
                elif field_value.isdigit():
                    field_types[field_name] = 'number'
                elif field_value in ['true', 'false']:
                    field_types[field_name] = 'boolean'
                elif field_value.startswith('"') or field_value.startswith("'"):
                    field_types[field_name] = 'string'
                elif field_value == 'null' or field_value == 'undefined':
                    field_types[field_name] = 'null'
                elif re.match(r'new\s+(\w+)', field_value):
                    # Extract class name from "new ClassName()"
                    class_match = re.match(r'new\s+(\w+)', field_value)
                    field_types[field_name] = class_match.group(1)
    
    # TypeScript properties (interface or class)
    ts_props = re.findall(r'(\w+)\s*:\s*([^;,]+)', content)
    for prop_name, prop_type in ts_props:
        if prop_name not in fields and not prop_name.startswith('_'):
            fields.append(prop_name)
        
        # Save type information
        if prop_name not in field_types:
            field_types[prop_name] = prop_type.strip()
    
    # Find methods with potential return type info
    # ES6 class methods
    method_matches = re.finditer(r'(?:/\*\*\s*([\s\S]*?)\s*\*/\s*)?(?:async\s+)?(\w+)\s*\([^)]*\)\s*{', content)
    for match in method_matches:
        jsdoc = match.group(1) if match.group(1) else ""
        method_name = match.group(2)
        
        if method_name not in ['constructor'] and not method_name.startswith('_'):
            methods.append(method_name)
            
            # Try to extract return type from JSDoc if available
            return_match = re.search(r'@returns?\s+{([^}]+)}', jsdoc)
            if return_match:
                field_types[f"{method_name}_return"] = return_match.group(1).strip()
    
    # Arrow function or function expression assigned to class property or prototype
    function_props = re.findall(r'(?:this\.)?(\w+)\s*=\s*(?:function|async function|\([^)]*\)\s*=>)', content)
    for func_name in function_props:
        if func_name not in methods and not func_name.startswith('_'):
            methods.append(func_name)
    
    # Prototype methods
    prototype_methods = re.findall(r'(?:prototype|__proto__)\.(\w+)\s*=\s*(?:function|async function)', content)
    methods.extend([m for m in prototype_methods if not m.startswith('_')])
    
    # Find relationships - now capturing relationship types
    relationship_data = []
    
    # Look for patterns that suggest relationships
    rel_patterns = [
        (r'this\.(\w+)\s*=\s*new\s+(\w+)', 'hasOne'),  # this.user = new User()
        (r'this\.(\w+)\s*=\s*\[\]', 'hasMany'),        # this.comments = []
        (r'this\.(\w+Id)\s*=', 'belongsTo'),           # this.userId =
        (r'this\.(\w+)_id\s*=', 'belongsTo')           # this.user_id =
    ]
    
    for pattern, rel_type in rel_patterns:
        for match in re.finditer(pattern, content):
            rel_name = match.group(1)
            related_model = None
            
            if len(match.groups()) > 1:
                related_model = match.group(2)
            elif rel_type == 'belongsTo':
                # For belongsTo, infer the model name from the field name
                if rel_name.endswith('Id'):
                    related_model = rel_name[:-2]  # Remove 'Id' suffix
                elif rel_name.endswith('_id'):
                    related_model = rel_name[:-3]  # Remove '_id' suffix
            
            if rel_name not in relationships and not rel_name.startswith('_'):
                relationships.append(rel_name)
                relationship_data.append((rel_name, rel_type, related_model))
    
    # Remove duplicates while preserving order
    fields = list(dict.fromkeys(fields))
    relationships = list(dict.fromkeys(relationships))
    methods = list(dict.fromkeys(methods))
    
    return fields, relationships, methods, class_name, field_types, relationship_data

def generate_rust_field(name, is_optional=True, field_type=None, related_model=None):
    """Generate a Rust field from a source field name.
    
    Args:
        name: The field name
        is_optional: Whether the field is optional
        field_type: The detected field type from source code
        related_model: Related model name if this is a relationship
    """
    # Map JavaScript/TypeScript/Ruby types to Rust types
    rust_type = None
    comment = None
    
    # Try to infer type from field_type
    if field_type:
        field_type = field_type.lower()
        if any(t in field_type for t in ['number', 'int', 'float', 'double']):
            if 'float' in field_type or 'double' in field_type:
                rust_type = 'f64'
            else:
                rust_type = 'i64'  # Default to i64 for numbers
        elif any(t in field_type for t in ['string', 'str', 'text']):
            rust_type = 'String'
        elif any(t in field_type for t in ['bool', 'boolean']):
            rust_type = 'bool'
        elif 'date' in field_type or 'time' in field_type:
            rust_type = 'String'  # Use String for now, comment will note it's a DateTime
            comment = 'DateTime'
        elif 'array' in field_type or field_type.startswith('['):
            # Try to extract the element type from Array<Type> or Type[]
            element_type = None
            if '<' in field_type:
                element_match = re.search(r'<([^>]+)>', field_type)
                if element_match:
                    element_type = element_match.group(1)
            elif '[' in field_type and ']' in field_type:
                element_match = re.search(r'\[\s*([^\]]+)\s*\]', field_type)
                if element_match:
                    element_type = element_match.group(1)
                    
            if element_type:
                # Generate appropriate Vec type based on element type
                element_rust_type = 'String'  # Default
                if any(t in element_type.lower() for t in ['number', 'int']):
                    element_rust_type = 'i64'
                elif any(t in element_type.lower() for t in ['float', 'double']):
                    element_rust_type = 'f64'
                elif any(t in element_type.lower() for t in ['bool', 'boolean']):
                    element_rust_type = 'bool'
                elif related_model:
                    element_rust_type = related_model  # Use the related model name
                    
                rust_type = f"Vec<{element_rust_type}>"
            else:
                rust_type = "Vec<String>"  # Default to Vec<String> if element type unknown
        elif related_model:
            # For object types that are related models
            rust_type = related_model
    
    # If we couldn't determine from field_type, use naming conventions
    if not rust_type:
        if name in ["id", "position"]:
            rust_type = 'i64'
        elif name.endswith("_id") or name.endswith("Id"):
            rust_type = 'i64'
        elif name.endswith("_at") or name.endswith("At"):
            rust_type = 'String'
            comment = 'DateTime'
        elif name.endswith("_count") or name.endswith("Count"):
            rust_type = 'i32'
        elif name in ["created_at", "updated_at", "createdAt", "updatedAt"]:
            rust_type = 'String'
            comment = 'DateTime'
        elif name in ["title", "name", "description"]:
            rust_type = 'String'
        elif name.startswith("is_") or name.startswith("has_") or name in ["active", "enabled", "visible", "published"]:
            rust_type = 'bool'
        else:
            rust_type = 'String'  # Default to String
    
    # Handle optional fields
    if is_optional and not rust_type.startswith('Vec<') and not rust_type.startswith('Option<'):
        rust_type = f"Option<{rust_type}>"
    
    field_line = f"pub {name}: {rust_type},"
    if comment:
        field_line += f" // {comment}"
    
    return field_line

def generate_rust_model(model_name, source_system="canvas", output_path=None):
    """Generate a Rust model file from a Canvas/Discourse model."""
    # Find the source file
    source_file = find_model_file(model_name, source_system)
    if not source_file:
        print(f"No source file found for {model_name}")
        return
    
    print(f"Found source file: {source_file}")
    
    # Set default output path if not provided
    if output_path is None:
        snake_model_name = camel_to_snake(model_name)
        output_path = f"src/models/{source_system}/{snake_model_name}.rs"
    
    # Determine extraction method based on file extension
    file_ext = os.path.splitext(source_file)[1].lower()
    if file_ext in ['.js', '.jsx', '.ts', '.tsx']:
        extracted_data = extract_fields_from_js_file(source_file)
        if len(extracted_data) == 6:
            fields, relationships, methods, class_name, field_types, relationship_data = extracted_data
        else:
            fields, relationships, methods, class_name = extracted_data
            field_types = {}
            relationship_data = []
    else:
        fields, relationships, methods, class_name = extract_fields_from_ruby_file(source_file)
        field_types = {}
        relationship_data = []
    
    # Create Rust model
    rust_model = f"""// Auto-generated from {os.path.basename(source_file)}
// Source: {source_file}

use serde::{{Deserialize, Serialize}};
"""

    # Add additional imports based on relationships
    related_models = set()
    for _, _, related_model in relationship_data:
        if related_model and related_model != model_name:
            related_models.add(related_model)
    
    if related_models:
        rust_model += "\n// Related model imports\n"
        for related in sorted(related_models):
            snake_related = camel_to_snake(related)
            rust_model += f"use crate::models::{snake_related}::{related};\n"
        rust_model += "\n"

    rust_model += f"""
#[derive(Debug, Clone, Serialize, Deserialize)]
/// {class_name} model - ported from {source_system.capitalize()}
/// Reference: {source_file}
pub struct {model_name} {{
    // Fields
"""
    
    # Add fields with improved type handling
    for field in sorted(set(fields)):
        # Find if this field is a relationship
        related_model = None
        for rel_name, _, rel_model in relationship_data:
            if rel_name == field:
                related_model = rel_model
                break
        
        field_type = field_types.get(field)
        rust_model += f"    {generate_rust_field(field, is_optional=True, field_type=field_type, related_model=related_model)}\n"
    
    # Close the struct
    rust_model += "}\n\n"
    
    # Add implementation with methods
    rust_model += f"impl {model_name} {{\n"
    rust_model += f"    pub fn new() -> Self {{\n"
    rust_model += f"        Self {{\n"
    
    # Default values for fields with improved type handling
    for field in sorted(set(fields)):
        field_type = field_types.get(field)
        
        # Generate appropriate default value based on field type
        if field == "id":
            rust_model += "            id: 0,\n"
        elif field_type:
            if any(t in str(field_type).lower() for t in ['array', '[]', 'vec']):
                rust_model += f"            {field}: Vec::new(),\n"
            elif not any(t in str(field_type).lower() for t in ['option', 'optional']):
                if any(t in str(field_type).lower() for t in ['string', 'str']):
                    rust_model += f"            {field}: String::new(),\n"
                elif any(t in str(field_type).lower() for t in ['bool', 'boolean']):
                    rust_model += f"            {field}: false,\n"
                elif any(t in str(field_type).lower() for t in ['number', 'int', 'float']):
                    rust_model += f"            {field}: 0,\n"
                else:
                    rust_model += f"            {field}: None,\n"
            else:
                rust_model += f"            {field}: None,\n"
        else:
            rust_model += f"            {field}: None,\n"
    
    rust_model += "        }\n"
    rust_model += "    }\n\n"
    
    # Add relationship helper methods if we have relationship data
    if relationship_data:
        for rel_name, rel_type, rel_model in relationship_data:
            if rel_model:
                if rel_type == 'hasMany':
                    rust_model += f"    /// Get {rel_name} - hasMany relationship to {rel_model}\n"
                    rust_model += f"    pub fn {rel_name}(&self) -> &Vec<{rel_model}> {{\n"
                    rust_model += f"        &self.{rel_name}\n"
                    rust_model += "    }\n\n"
                elif rel_type == 'hasOne':
                    rust_model += f"    /// Get {rel_name} - hasOne relationship to {rel_model}\n"
                    rust_model += f"    pub fn {rel_name}(&self) -> Option<&{rel_model}> {{\n"
                    rust_model += f"        self.{rel_name}.as_ref()\n"
                    rust_model += "    }\n\n"
                elif rel_type == 'belongsTo':
                    # For belongsTo relations, might have either model or just ID
                    if rel_name.endswith('Id') or rel_name.endswith('_id'):
                        rust_model += f"    /// Get {rel_name} - belongsTo relationship to {rel_model}\n"
                        rust_model += f"    pub fn {rel_name}(&self) -> Option<i64> {{\n"
                        rust_model += f"        self.{rel_name}\n"
                        rust_model += "    }\n\n"
                    else:
                        rust_model += f"    /// Get {rel_name} - belongsTo relationship to {rel_model}\n"
                        rust_model += f"    pub fn {rel_name}(&self) -> Option<&{rel_model}> {{\n"
                        rust_model += f"        self.{rel_name}.as_ref()\n"
                        rust_model += "    }\n\n"
    
    # Add method stubs with improved return type detection  
    for method in methods[:10]:  # Limit to 10 methods to avoid clutter
        # Check if we have return type info
        return_type = field_types.get(f"{method}_return")
        rust_return_type = "bool"  # Default
        
        if return_type:
            if any(t in str(return_type).lower() for t in ['void', 'undefined', 'null']):
                rust_return_type = "()"
            elif any(t in str(return_type).lower() for t in ['string', 'str']):
                rust_return_type = "String"
            elif any(t in str(return_type).lower() for t in ['number', 'int']):
                rust_return_type = "i64"
            elif any(t in str(return_type).lower() for t in ['float', 'double']):
                rust_return_type = "f64"
            elif any(t in str(return_type).lower() for t in ['bool', 'boolean']):
                rust_return_type = "bool"
            elif any(t in str(return_type).lower() for t in ['array', 'vec']):
                rust_return_type = "Vec<String>"
            elif 'promise' in str(return_type).lower():
                # Async method
                if 'void' in str(return_type).lower():
                    rust_return_type = "impl Future<Output = ()>"
                else:
                    rust_return_type = "impl Future<Output = bool>"
        
        rust_model += f"    // TODO: Implement {method} from {class_name}\n"
        rust_model += f"    pub fn {method}(&self) -> {rust_return_type} {{\n"
        rust_model += "        // Implementation needed\n"
        if rust_return_type == "()":
            rust_model += "\n"
        elif rust_return_type.startswith("impl Future"):
            rust_model += "        async { () }\n"  # Placeholder for async
        elif rust_return_type == "String":
            rust_model += "        String::new()\n"
        elif rust_return_type == "Vec<String>":
            rust_model += "        Vec::new()\n"
        elif rust_return_type in ["i64", "f64"]:
            rust_model += "        0\n"
        else:
            rust_model += "        false\n"
        rust_model += "    }\n\n"
    
    rust_model += "}\n"
      # Write to file
    output_dir = os.path.dirname(output_path)
    if not os.path.exists(output_dir):
        os.makedirs(output_dir, exist_ok=True)
    
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
    snake_name = camel_to_snake(model_name)
    table_entry = f"| {source_system.capitalize()} `{class_name}` | `models/{snake_name}.rs:{model_name}` | 0% | Auto-generated with {len(fields)} fields, {method_count} methods |\n"
    
    # Insert the new entry after the table header
    content.insert(table_start_idx + 2, table_entry)
    
    # Write the updated content
    with open(mapping_path, 'w', encoding='utf-8') as f:
        f.writelines(content)
    
    print(f"Updated mapping.md with {model_name} entry")

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: python generate_rust_model.py <model_name> [canvas|discourse]")
        sys.exit(1)
    
    model_name = sys.argv[1]
    source_system = sys.argv[2] if len(sys.argv) > 2 else "canvas"
    
    generate_rust_model(model_name, source_system)