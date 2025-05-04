# run all tests
Write-Host "Running unit tests..." -ForegroundColor Green
cargo test

# run c test programs
Write-Host "Running C test programs..." -ForegroundColor Green

# Define all possible test files
$potentialFiles = @(
    "simple_test.c",
    "test_program.c",
    "combined_control_flow_test.c", 
    "complex_test.c",
    "multiple_variables_test.c",
    "nested_loop_test.c",
    "printf_test.c",
    "simple_array_test.c",
    "simple_comparison_test.c",
    "simple_for_test.c",
    "simple_while_test.c",
    "single_variable_test.c",
    "string_test.c"
)

# Filter to only include files that exist
$testFiles = @()
foreach ($file in $potentialFiles) {
    if (Test-Path $file) {
        $testFiles += $file
        Write-Host "Found test file: $file" -ForegroundColor Green
    }
    else {
        Write-Host "Test file not found (skipping): $file" -ForegroundColor Yellow
    }
}

Write-Host "Running $($testFiles.Count) test files" -ForegroundColor Green

# Run each test in normal mode
Write-Host "Running tests in normal mode..." -ForegroundColor Cyan
foreach ($file in $testFiles) {
    Write-Host "Testing: $file" -ForegroundColor Yellow
    cargo run $file
    Write-Host ""
}