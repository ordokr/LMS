import os
import re
from datetime import datetime

def find_migrated_files(tracking_doc_path):
    """Find all migrated JavaScript files from the tracking document."""
    try:
        with open(tracking_doc_path, 'r', encoding='utf-8') as f:
            content = f.read()
    except FileNotFoundError:
        print(f"Error: Tracking document not found at {tracking_doc_path}")
        return []
    
    # Extract completed migrations (marked with [x])
    pattern = r'\[\s*x\s*\]\s*(.*?\.js)\s*\|\s*(.*?)\s*\|'
    matches = re.findall(pattern, content)
    return matches

def categorize_migrations(migrations):
    """Categorize migrations by component type."""
    categories = {
        "services": [],
        "models": [],
        "controllers": [],
        "routes": [],
        "utils": [],
        "middleware": [],
        "tools": [],
        "other": []
    }
    
    for js_file, rust_file in migrations:
        if "services" in js_file or "Service" in js_file:
            categories["services"].append((js_file, rust_file))
        elif "models" in js_file or "Model" in js_file:
            categories["models"].append((js_file, rust_file))
        elif "controllers" in js_file or "Controller" in js_file:
            categories["controllers"].append((js_file, rust_file))
        elif "routes" in js_file or "Route" in js_file:
            categories["routes"].append((js_file, rust_file))
        elif "utils" in js_file or "Utils" in js_file:
            categories["utils"].append((js_file, rust_file))
        elif "middleware" in js_file or "Middleware" in js_file:
            categories["middleware"].append((js_file, rust_file))
        elif "tools" in js_file or js_file.startswith("tools/"):
            categories["tools"].append((js_file, rust_file))
        else:
            categories["other"].append((js_file, rust_file))
    
    return categories

def generate_report(migrations, categories):
    """Generate a final migration report."""
    report = []
    report.append("# JavaScript to Rust Migration Report\n")
    report.append(f"_Generated: {datetime.now().strftime('%Y-%m-%d')}_\n\n")
    
    # Summary
    report.append("## Migration Summary\n\n")
    report.append(f"- **Total migrated files:** {len(migrations)}\n")
    for category, files in categories.items():
        if files:
            report.append(f"- **{category.capitalize()}:** {len(files)} files\n")
    report.append("\n")
    
    # Category details
    for category, files in categories.items():
        if files:
            report.append(f"## {category.capitalize()} Migrations\n\n")
            report.append("| JavaScript File | Rust Equivalent |\n")
            report.append("|----------------|------------------|\n")
            
            for js_file, rust_file in files:
                report.append(f"| {js_file} | {rust_file} |\n")
            
            report.append("\n")
    
    # Benefits
    report.append("## Migration Benefits\n\n")
    report.append("The JavaScript to Rust migration has provided the following benefits:\n\n")
    report.append("1. **Improved Performance:** Rust's compile-time optimizations and zero-cost abstractions have significantly improved runtime performance.\n")
    report.append("2. **Enhanced Type Safety:** Rust's strong type system has eliminated many runtime errors that were common in the JavaScript codebase.\n")
    report.append("3. **Memory Safety:** Rust's ownership model prevents memory leaks and data races, making the application more robust.\n")
    report.append("4. **Better Concurrency:** Rust's async/await and threading models provide safer and more efficient concurrency.\n")
    report.append("5. **Improved Maintainability:** With better tooling and compile-time checks, the codebase is now easier to maintain and extend.\n\n")
    
    # Next steps
    report.append("## Next Steps\n\n")
    report.append("- Optimize performance-critical paths using Rust-specific optimizations\n")
    report.append("- Add comprehensive tests using Rust's testing frameworks\n")
    report.append("- Implement new features using Rust's robust ecosystem\n")
    report.append("- Document the migrated codebase using Rust's documentation tools\n")
    
    return "\n".join(report)

def main():
    # Path is relative to where the script is located
    tracking_doc = os.path.join(os.path.dirname(os.path.dirname(os.path.dirname(__file__))), 
                               "JavaScript to Rust Migration Tracking.md")
    report_path = os.path.join(os.path.dirname(os.path.dirname(os.path.dirname(__file__))), 
                              "JavaScript to Rust Migration Report.md")
    
    print(f"Generating migration report from: {tracking_doc}")
    
    # Find migrated files
    migrations = find_migrated_files(tracking_doc)
    print(f"Found {len(migrations)} migrated files")
    
    # Categorize migrations
    categories = categorize_migrations(migrations)
    
    # Generate report
    report = generate_report(migrations, categories)
    
    # Write report to file
    with open(report_path, 'w', encoding='utf-8') as f:
        f.write(report)
    
    print(f"Successfully generated report: {report_path}")

if __name__ == "__main__":
    main()