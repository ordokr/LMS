#!/usr/bin/env python
"""
Batch processor for generating multiple Rust models from Canvas models
"""
import sys
import os
import json
import subprocess
from pathlib import Path

# Get the absolute path to the main LMS directory
SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))
if os.path.basename(os.path.dirname(SCRIPT_DIR)) == "lms-integration":
    # If running from lms-integration/tools
    LMS_ROOT = os.path.dirname(os.path.dirname(SCRIPT_DIR))
else:
    # If running from LMS/tools
    LMS_ROOT = os.path.dirname(SCRIPT_DIR)

def process_batch(config_file):
    """Process a batch of model conversions from a config file"""
    # Resolve the config file path relative to the current directory
    if not os.path.isabs(config_file):
        config_file = os.path.join(os.getcwd(), config_file)
    
    try:
        with open(config_file, 'r') as f:
            config = json.load(f)
    except Exception as e:
        print(f"Error loading configuration file: {e}")
        return

    print(f"Found {len(config['models'])} models to process")
    print(f"LMS root directory: {LMS_ROOT}")
    success_count = 0
    
    # Get path to the generator script
    generator_script = os.path.join(LMS_ROOT, "tools", "generate_rust_model.py")
    if not os.path.exists(generator_script):
        print(f"Error: Could not find generator script at {generator_script}")
        return
        
    for model in config['models']:
        canvas_path = model['canvasModelPath']
        output_path = model['outputPath']
        
        # Resolve paths relative to the LMS root directory
        abs_canvas_path = os.path.join(LMS_ROOT, canvas_path)
        abs_output_path = os.path.join(LMS_ROOT, output_path)
        
        # Create output directory if it doesn't exist
        os.makedirs(os.path.dirname(abs_output_path), exist_ok=True)
        
        print(f"Processing: {canvas_path} -> {output_path}")
        
        try:
            result = subprocess.run(
                ["python", generator_script, abs_canvas_path, abs_output_path],
                check=True,
                capture_output=True,
                text=True
            )
            print(f"Success: {os.path.basename(canvas_path)}")
            success_count += 1
        except subprocess.CalledProcessError as e:
            print(f"Error processing {canvas_path}: {e.stderr}")
    
    print(f"Completed processing. {success_count}/{len(config['models'])} models generated successfully.")

if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("Usage: python batch_generate_models.py <config_file.json>")
        sys.exit(1)
    
    process_batch(sys.argv[1])
