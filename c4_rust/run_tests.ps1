# run all tests
Write-Host "Running unit tests..." -ForegroundColor Green
cargo test

# run c test programs
Write-Host "Running C test programs..." -ForegroundColor Green
cargo run -- simple_test.c
cargo run -- -d simple_test.c
cargo run -- test_program.c 