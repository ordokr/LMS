$filePath = "C:\Users\Tim\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\base64-0.21.7\src\lib.rs"

# Check if the file exists
if (Test-Path $filePath) {
    Write-Host "Found the base64 crate file. Fixing the issue..."
    
    # Read the file content
    $content = Get-Content $filePath -Raw
    
    # Replace the problematic lines
    $fixedContent = $content -replace "let encrypted_string = general_purpose::STANDARD.encode\(encrypted_data\);", "// let encrypted_string = general_purpose::STANDARD.encode(encrypted_data);"
    $fixedContent = $fixedContent -replace "let decoded_data = general_purpose::STANDARD.decode\(encrypted_data\).map_err\(\|e\| e.to_string\(\)\)\?;", "// let decoded_data = general_purpose::STANDARD.decode(encrypted_data).map_err(|e| e.to_string());"
    
    # Write the fixed content back to the file
    $fixedContent | Set-Content $filePath
    
    Write-Host "Fixed the base64 crate file."
} else {
    Write-Host "Could not find the base64 crate file at $filePath"
}
