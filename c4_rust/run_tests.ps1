# run all tests
Write-Host "Running unit tests..." -ForegroundColor Green
# Set RUSTFLAGS to suppress all warnings
$env:RUSTFLAGS = "-A warnings"
cargo test -q 2>$null

# run c test programs
Write-Host "Running C test programs..." -ForegroundColor Green

# Define test directory path
$testDir = "tests/C_files"

# Get all test files matching the pattern test_*.c and sort them numerically based on the number in the filename
$testFiles = Get-ChildItem -Path $testDir -Filter "test_*.c" | 
             ForEach-Object { 
                # Extract the number from the filename
                $number = [int]($_.BaseName -replace 'test_', '')
                # Create object with original file and its numeric value
                [PSCustomObject]@{
                    FullName = $_.FullName
                    Number = $number
                }
             } | 
             # Sort by the numeric value
             Sort-Object -Property Number |
             # Return just the full path
             ForEach-Object { $_.FullName }

# Report found files
foreach ($file in $testFiles) {
    Write-Host "Found test file: $file" -ForegroundColor Green
}

Write-Host "Running $($testFiles.Count) test files" -ForegroundColor Green

# Run each test in normal mode
Write-Host "Running tests in normal mode..." -ForegroundColor Cyan
foreach ($file in $testFiles) {
    Write-Host "Testing: $file" -ForegroundColor Yellow
    # Run with warnings suppressed and errors redirected
    cargo run -q $file 2>$null
    Write-Host ""
}

# Restore RUSTFLAGS when done
$env:RUSTFLAGS = ""