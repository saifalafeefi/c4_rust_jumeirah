@echo off
echo Creating submission package...

mkdir -p submission
copy src\*.rs submission\
copy Cargo.toml submission\
copy README.md submission\
copy c4_rust_comparison.pdf submission\
copy hello.c submission\
copy simple_test_c4.c submission\
copy test_c4.c submission\

echo Package created in "submission" directory
echo Done! 