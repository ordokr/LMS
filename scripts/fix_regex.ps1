$file = "src\components\forum\thread_detail.rs"
if (-not (Test-Path $file)) {
    Write-Host "Error: thread_detail.rs not found." -ForegroundColor Red
    exit 1
}

$content = Get-Content -Path $file -Raw

# Insert a '#' after the initial r"
$content = $content -replace 'r"', 'r#"'

# Replace the trailing " with "# just before ).unwrap();
# (This assumes the literal ends right before the first occurrence of ) followed by .unwrap())
$content = $content -replace '"(?=\)\.unwrap\(\))', '"#'


if ($content -ne $null) {
    Write-Host "Content after regex replacement:" -ForegroundColor Yellow
    Write-Host $content -ForegroundColor Yellow
}
else {
    Write-Host "Error: Content is null after regex replacement." -ForegroundColor Red
    exit 1
}                                   

Set-Content -Path $file -Value $content
Write-Host "Regex in thread_detail.rs fixed successfully." -ForegroundColor Green