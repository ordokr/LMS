#!/usr/bin/env python
"""
Batch processor for generating multiple Rust models from Canvas models
"""
import sys
import os
import json
import subprocess
from pathlib import Path

def process_batch(config_file):
    """Process a batch of model conversions from a config file"""
    try:
        with open(config_file, 'r') as f:
            config = json.load(f)
    except Exception as e:
        print(f"Error loading configuration file: {e}")
        return

    print(f"Found {len(config['models'])} models to process")
    success_count = 0
    
    for model in config['models']:
        canvas_path = model['canvasModelPath']
        output_path = model['outputPath']
        print(f"Processing: {canvas_path} -> {output_path}")
        
        try:
            result = subprocess.run(
                ["python", "tools/generate_rust_model.py", canvas_path, output_path],
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
