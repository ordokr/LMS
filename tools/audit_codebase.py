import os
import re
import json
import subprocess
from find_model_file import find_model_file

def scan_rust_models():
    """Scan all Rust models in the project."""
    models = []
    
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
                models.append({
                    "name": struct_name,
                    "file": file_path
                })
    
    return models

def run_full_audit():
    """Run a full audit of the codebase."""
    print("Running full codebase audit...")
    models = scan_rust_models()
    
    results = []
    
    for model in models:
        print(f"\nAnalyzing {model['name']} in {model['file']}...")
        
        # Try Canvas first
        canvas_file = find_model_file(model['name'], "canvas")
        if canvas_file:
            print(f"Found Canvas equivalent: {canvas_file}")
            
            # Run comparison
            try:
                output = subprocess.check_output([
                    "python", 
                    "tools/compare_models.py", 
                    model['file'], 
                    canvas_file,
                    model['name'],
                    os.path.basename(canvas_file).replace(".rb", "")
                ], universal_newlines=True)
                
                # Extract percentage
                percentage_match = re.search(r"Implementation Status: ([\d.]+)%", output)
                percentage = float(percentage_match.group(1)) if percentage_match else 0
                
                results.append({
                    "model": model['name'],
                    "file": model['file'],
                    "source": "Canvas",
                    "source_file": canvas_file,
                    "percentage": percentage,
                    "output": output
                })
                
                continue  # Skip Discourse check if Canvas match found
            except subprocess.CalledProcessError:
                print("Error running comparison")
        
        # Try Discourse next
        discourse_file = find_model_file(model['name'], "discourse")
        if discourse_file:
            print(f"Found Discourse equivalent: {discourse_file}")
            
            # Run comparison
            try:
                output = subprocess.check_output([
                    "python", 
                    "tools/compare_models.py", 
                    model['file'], 
                    discourse_file,
                    model['name'],
                    os.path.basename(discourse_file).replace(".rb", "")
                ], universal_newlines=True)
                
                # Extract percentage
                percentage_match = re.search(r"Implementation Status: ([\d.]+)%", output)
                percentage = float(percentage_match.group(1)) if percentage_match else 0
                
                results.append({
                    "model": model['name'],
                    "file": model['file'],
                    "source": "Discourse",
                    "source_file": discourse_file,
                    "percentage": percentage,
                    "output": output
                })
            except subprocess.CalledProcessError:
                print("Error running comparison")
    
    # Generate summary report
    print("\n\n========== AUDIT SUMMARY ==========")
    print(f"Analyzed {len(models)} models")
    
    # Sort by percentage
    results.sort(key=lambda x: x['percentage'])
    
    # Print low completion items
    low_completion = [r for r in results if r['percentage'] < 50]
    if low_completion:
        print(f"\nLOW COMPLETION MODELS ({len(low_completion)}):")
        for result in low_completion:
            print(f"- {result['model']} ({result['percentage']}% complete) - source: {result['source']}")
    
    # Print medium completion items
    med_completion = [r for r in results if 50 <= r['percentage'] < 80]
    if med_completion:
        print(f"\nMEDIUM COMPLETION MODELS ({len(med_completion)}):")
        for result in med_completion:
            print(f"- {result['model']} ({result['percentage']}% complete) - source: {result['source']}")
    
    # Print high completion items
    high_completion = [r for r in results if r['percentage'] >= 80]
    if high_completion:
        print(f"\nHIGH COMPLETION MODELS ({len(high_completion)}):")
        for result in high_completion:
            print(f"- {result['model']} ({result['percentage']}% complete) - source: {result['source']}")
    
    # Models with no match
    unmatched = [m for m in models if not any(r['model'] == m['name'] for r in results)]
    if unmatched:
        print(f"\nUNMATCHED MODELS ({len(unmatched)}):")
        for model in unmatched:
            print(f"- {model['name']} - No Canvas or Discourse equivalent found")
    
    # Save detailed report
    with open("audit_report.json", "w") as f:
        json.dump(results, f, indent=2)
    
    print("\nDetailed report saved to audit_report.json")
    
    # Update mapping.md with results
    update_mapping_with_audit(results)

def update_mapping_with_audit(results):
    """Update mapping.md with audit results."""
    mapping_path = "mapping.md"
    
    if not os.path.exists(mapping_path):
        print("mapping.md not found, skipping update")
        return
    
    with open(mapping_path, 'r', encoding='utf-8') as f:
        content = f.readlines()
    
    # Update each model entry
    for result in results:
        found = False
        source_text = f"{result['source']} `{os.path.basename(result['source_file']).replace('.rb', '')}`"
        
        for i, line in enumerate(content):
            if source_text in line and f"{result['model']}`" in line:
                # Update the percentage
                parts = line.split("|")
                if len(parts) >= 4:
                    parts[3] = f" {result['percentage']:.0f}% "
                    content[i] = "|".join(parts)
                    found = True
                    break
    
    # Save changes if any were made
    if any(result for result in results if result['percentage'] > 0):
        with open(mapping_path, 'w', encoding='utf-8') as f:
            f.writelines(content)
        print("\nUpdated mapping.md with latest completion percentages")

if __name__ == "__main__":
    run_full_audit()