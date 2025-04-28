# c4_rust_jumeirah
 Spring 2025 - Principles of Programming Languages - Project of converting the C4 compiler to Rust

A Rust implementation of the [C4 compiler](https://github.com/rswier/c4) originally written by Robert Swierczek.

## Overview

C4 is a small self-hosting C compiler that compiles a subset of C. This project reimplements C4 in Rust, maintaining its self-hosting capability and core functionality while leveraging Rust's safety features and modern programming paradigms.

## Project Structure

The project is organized into several key components:

- `src/main.rs`: Command-line argument handling and file loading
- `src/lexer.rs`: Tokenization of C source code
- `src/parser.rs`: Expression and statement parsing, code generation
- `src/vm.rs`: Virtual machine for executing the compiled code
- `src/lib.rs`: Library exports for testing

## Features

- Tokenizes and parses the same subset of C as the original C4
- Supports self-compilation (can compile its original C code)
- Uses a virtual machine to execute the compiled code
- Preserves the minimal design of C4 while using Rust idioms
- Includes robust error handling and bounds checking for stack safety

## Building

To build the project:

```bash
# Clone the repository
git clone https://github.com/your-username/c4_rust.git
cd c4_rust

# Build in debug mode
cargo build

# Build in release mode for better performance
cargo build --release
```

## Usage

```bash
# Run in debug mode
cargo run -- [options] <input-file>

# Or use the compiled binary directly
./target/debug/c4_rust [options] <input-file>
```

Options:
- `-s`: Print source and assembly
- `-d`: Print executed instructions
- `input-file`: C source file to compile

## Example

To compile and run the original C4 source:

```bash
# Assuming c4.c is in the current directory
cargo run -- c4.c

# Or with debugging information
cargo run -- -d c4.c

# Or with source and assembly output
cargo run -- -s c4.c
```

## Testing

To run the tests:

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_lexer_basic_tokens

# Run tests with output
cargo test -- --nocapture

# Use the PowerShell script to run all tests and sample programs
.\run_tests.ps1
```

### Test Files

The project includes several test files:

- `simple_test.c`: A basic "Hello, World!" program
- `test_program.c`: A more complex test program that exercises various C features
- `tests/simple_printf.c`: Tests simple printf functionality
- `tests/printf_test.c`: Tests more complex printf functionality

### Test Modules

The Rust tests are organized into several modules:

- `lexer_tests.rs`: Tests for tokenizing C source code
- `pointer_tests.rs`: Tests for pointer operations
- `function_tests.rs`: Tests for function parsing and generation
- `vm_tests.rs`: Tests for the virtual machine execution

## Recent Fixes and Improvements

- Fixed printf system call to handle format strings better
- Implemented proper lexer tests that match the lexer's actual behavior
- Corrected parameter handling in the parser
- Added error handling for invalid format string addresses
- Fixed unused variable warnings
- Enhanced code generation for pointers and function calls
- Created a convenient testing script to verify all functionality

## Current Implementation Status

- [x] Project structure set up
- [x] Command-line argument handling
- [x] Basic data structures defined
- [x] Lexer implementation
- [x] Symbol table implementation
- [x] Type system basic implementation
- [x] Expression parsing with precedence climbing
- [x] Statement parsing (if-else, while, blocks)
- [x] Function parameter parsing
- [x] Local variable handling
- [x] Pointer operations and dereferencing
- [x] VM implementation with stack-based architecture
- [x] Self-compilation support
- [x] Type casting implementation
- [x] System call implementation (printf, etc.)
- [x] Comprehensive test suite 