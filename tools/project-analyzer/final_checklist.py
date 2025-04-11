import os
import re
import json
from datetime import datetime

def check_migration_completion():
    """Create a comprehensive migration completion checklist."""
    # Path to project root
    project_root = os.path.dirname(os.path.dirname(os.path.dirname(__file__)))
    
    checklist = [
        "# JavaScript to Rust Migration Completion Checklist\n",
        f"_Generated: {datetime.now().strftime('%Y-%m-%d')}_\n\n"
    ]
    
    # 1. Check if all JavaScript files have been migrated
    js_files = []
    for root, _, files in os.walk(project_root):
        if any(excluded in root for excluded in [
            'node_modules', 'dist', 'build', 'coverage', '.git', 'target', 'js_backup'
        ]):
            continue
        
        js_files.extend([
            os.path.relpath(os.path.join(root, f), project_root).replace('\\', '/')
            for f in files if f.endswith('.js')
        ])
    
    if js_files:
        checklist.append("## 1. JavaScript Files Still Present\n\n")
        checklist.append(f"Found {len(js_files)} JavaScript files still in the project:\n\n")
        for js_file in sorted(js_files):
            checklist.append(f"- [ ] {js_file}\n")
        
        checklist.append("\nReview these files to determine if they need migration or can be removed.\n\n")
    else:
        checklist.append("## 1. JavaScript Files Still Present\n\n")
        checklist.append("✅ No JavaScript files found in the project! All files have been migrated or removed.\n\n")
    
    # 2. Check package.json for dependencies
    package_json_path = os.path.join(project_root, "package.json")
    if os.path.exists(package_json_path):
        try:
            with open(package_json_path, 'r', encoding='utf-8') as f:
                package_data = json.load(f)
            
            if 'dependencies' in package_data and package_data['dependencies']:
                checklist.append("## 2. Package Dependencies\n\n")
                checklist.append("The following JavaScript dependencies are still in package.json:\n\n")
                for dep, version in package_data['dependencies'].items():
                    checklist.append(f"- [ ] {dep} ({version})\n")
                
                checklist.append("\nReview these dependencies to determine if they need to be replaced with Rust equivalents.\n\n")
            else:
                checklist.append("## 2. Package Dependencies\n\n")
                checklist.append("✅ No JavaScript dependencies found in package.json!\n\n")
        except Exception as e:
            checklist.append("## 2. Package Dependencies\n\n")
            checklist.append(f"❌ Error reading package.json: {e}\n\n")
    else:
        checklist.append("## 2. Package Dependencies\n\n")
        checklist.append("✅ No package.json file found (expected for a fully migrated Rust project).\n\n")
    
    # 3. Check Rust project structure
    cargo_toml_path = os.path.join(project_root, "Cargo.toml")
    if os.path.exists(cargo_toml_path):
        checklist.append("## 3. Rust Project Structure\n\n")
        checklist.append("- [x] Cargo.toml exists at project root\n")
        
        # Check for key Rust directories and files
        key_paths = [
            ("src directory", os.path.join(project_root, "src")),
            ("src/main.rs or src/lib.rs", os.path.join(project_root, "src", "main.rs")),
            ("Tests directory", os.path.join(project_root, "tests"))
        ]
        
        for name, path in key_paths:
            if os.path.exists(path):
                checklist.append(f"- [x] {name} exists\n")
            else:
                alt_path = path.replace("main.rs", "lib.rs")
                if name == "src/main.rs or src/lib.rs" and os.path.exists(alt_path):
                    checklist.append(f"- [x] {name} exists (lib.rs)\n")
                else:
                    checklist.append(f"- [ ] {name} is missing\n")
        
        checklist.append("\n")
    else:
        checklist.append("## 3. Rust Project Structure\n\n")
        checklist.append("❌ No Cargo.toml found at project root! This is unexpected for a migrated Rust project.\n\n")
    
    # 4. Final verification steps
    checklist.append("## 4. Final Verification Steps\n\n")
    checklist.append("- [ ] All unit tests have been migrated and pass\n")
    checklist.append("- [ ] Integration tests have been migrated and pass\n")
    checklist.append("- [ ] The application builds successfully with `cargo build --release`\n")
    checklist.append("- [ ] The application runs without errors\n")
    checklist.append("- [ ] Performance metrics have been collected to verify improvements\n")
    checklist.append("- [ ] Documentation has been updated to reflect the Rust implementation\n")
    
    # Write checklist to file
    checklist_path = os.path.join(project_root, "Migration Completion Checklist.md")
    with open(checklist_path, 'w', encoding='utf-8') as f:
        f.writelines(checklist)
    
    print(f"Generated migration completion checklist: {checklist_path}")
    return checklist_path

if __name__ == "__main__":
    check_migration_completion()