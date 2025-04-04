import sys
import os
import re

def read_file(path):
    if not os.path.exists(path):
        print(f"Error: File not found: {path}")
        return None
    
    with open(path, 'r', encoding='utf-8') as f:
        return f.read()

def extract_rust_struct(content, struct_name):
    """Extract a specific struct from Rust content"""
    pattern = rf'#\[derive.*?\]\s*pub struct {struct_name}\s*\{{(.*?)\}}(?:\s*impl|$)'
    match = re.search(pattern, content, re.DOTALL)
    
    if not match:
        print(f"Error: Could not find struct {struct_name} in the file")
        return None
        
    return match.group(0)

def extract_ruby_class(content, class_name):
    """Extract a specific class from Ruby content"""
    pattern = rf'class {class_name}.*?(?:^end|\Z)'
    match = re.search(pattern, content, re.DOTALL | re.MULTILINE)
    
    if not match:
        print(f"Error: Could not find class {class_name} in the file")
        return None
        
    return match.group(0)

def extract_model_name(content, is_rust=False):
    if is_rust:
        # Find struct or enum definitions in Rust
        matches = re.findall(r'(?:pub\s+)?(?:struct|enum)\s+(\w+)', content)
    else:
        # Find class definitions in Ruby
        matches = re.findall(r'class\s+(\w+)', content)
    
    return matches[0] if matches else "Unknown"

def compare_models(our_file, original_file, our_model_name=None, original_model_name=None):
    our_content = read_file(our_file)
    original_content = read_file(original_file)
    
    if not our_content or not original_content:
        return
    
    # Extract specific model if name is provided
    if our_model_name:
        our_content = extract_rust_struct(our_content, our_model_name)
        if not our_content:
            return
    
    if original_model_name:
        original_content = extract_ruby_class(original_content, original_model_name)
        if not original_content:
            return
    
    our_model = extract_model_name(our_content, is_rust=True) if our_content else "Unknown"
    original_model = extract_model_name(original_content) if original_content else "Unknown"
    
    print(f"Comparing models: {our_model} (Rust) ↔ {original_model} (Original)")
    print("\n" + "="*60 + "\n")
    
    # Extract fields from our Rust model
    rust_fields = re.findall(r'pub\s+(\w+):\s+([^,\n]+)', our_content) if our_content else []
    rust_fields = [(name, type_) for name, type_ in rust_fields]
    
    print(f"Rust Model Fields ({len(rust_fields)}):")
    for name, type_ in rust_fields:
        print(f"  - {name}: {type_.strip()}")
    
    print("\n" + "="*60 + "\n")
    
    # Extract fields from Ruby model
    ruby_fields = []
    if original_content:
        # Check for attr_accessor
        attr_patterns = [
            r'attr_accessor\s+:([\w_]+)',
            r'attr_reader\s+:([\w_]+)',
            r'attr_writer\s+:([\w_]+)'
        ]
        
        for pattern in attr_patterns:
            attrs = re.findall(pattern, original_content)
            for attr in attrs:
                for a in attr.split(","):
                    ruby_fields.append(a.strip())
        
        # Check for database columns
        if "create_table" in original_content or "add_column" in original_content:
            columns = re.findall(r't\.(\w+)\s+:(\w+)', original_content)
            ruby_fields.extend([col[1] for col in columns])
            
        # Check for has_many, belongs_to, etc.
        relation_patterns = [
            r'(?:has_many|has_one|belongs_to)\s+:([\w_]+)',
        ]
        
        for pattern in relation_patterns:
            relations = re.findall(pattern, original_content)
            ruby_fields.extend(relations)
        
        # Class variables can be field-like
        class_vars = re.findall(r'@@([\w_]+)', original_content)
        ruby_fields.extend(class_vars)
        
        # Instance variables often represent fields
        instance_vars = re.findall(r'@([\w_]+)[^=]', original_content)
        ruby_fields.extend(instance_vars)
        
        ruby_fields = list(set(ruby_fields))  # Remove duplicates
    
    print(f"Original Model Fields ({len(ruby_fields)}):")
    for field in sorted(ruby_fields):
        print(f"  - {field}")
    
    print("\n" + "="*60 + "\n")
    
    # Compare fields
    our_fields = {name.lower() for name, _ in rust_fields}
    their_fields = {field.lower() for field in ruby_fields}
    
    missing = their_fields - our_fields
    extra = our_fields - their_fields
    
    if missing:
        print(f"Missing Fields ({len(missing)}):")
        for field in sorted(missing):
            print(f"  - {field}")
        print()
    
    if extra:
        print(f"Extra Fields in Our Implementation ({len(extra)}):")
        for field in sorted(extra):
            print(f"  - {field}")
        print()

    # Calculate percentage of original fields implemented
    if their_fields:
        implemented = len(their_fields) - len(missing)
        percentage = (implemented / len(their_fields)) * 100
        print(f"Implementation Status: {percentage:.1f}% of original fields")
    
    # Look for methods in our implementation
    rust_methods = re.findall(r'fn\s+(\w+)', our_content) if our_content else []
    if rust_methods:
        print("\nImplemented Methods:")
        for method in rust_methods:
            if method != "main":  # Skip main function
                print(f"  - {method}")
    
    # Look for methods in original
    ruby_methods = re.findall(r'def\s+(\w+)', original_content) if original_content else []
    important_methods = [m for m in ruby_methods if not (m.startswith('_') or m == 'initialize')]
    
    if important_methods:
        print("\nImportant Original Methods to Consider:")
        for method in important_methods[:10]:  # Show first 10 to avoid clutter
            print(f"  - {method}")
        if len(important_methods) > 10:
            print(f"  - ... and {len(important_methods) - 10} more")
    
    print("\nAnalysis complete. Update mapping.md with these findings.")

if __name__ == "__main__":
    if len(sys.argv) < 3:
        print("Usage: python compare_models.py <our_file_path> <original_file_path> [our_model_name] [original_model_name]")
        sys.exit(1)
    
    our_file = sys.argv[1]
    original_file = sys.argv[2]
    our_model_name = sys.argv[3] if len(sys.argv) > 3 else None
    original_model_name = sys.argv[4] if len(sys.argv) > 4 else None
    
    compare_models(our_file, original_file, our_model_name, original_model_name)