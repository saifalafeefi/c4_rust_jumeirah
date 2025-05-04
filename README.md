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

- **Lexing and Parsing**: Tokenizes and parses a significant subset of C, including:
  - Keywords: `int`, `char`, `if`, `else`, `while`, `for`, `return`, `sizeof`, `enum`, `void`.
  - Operators: Arithmetic (`+`, `-`, `*`, `/`, `%`), comparison (`==`, `!=`, `<`, `>`, `<=`, `>=`), logical (`&&`, `||`, `!`), bitwise (`&`, `|`, `^`, `~`, `<<`, `>>`), assignment (`=`, `+=`, `-=`, etc.), increment/decrement (`++`, `--`), address-of (`&`), dereference (`*`).
  - Literals: Integers (decimal, hex, octal), character literals (`'c'`), string literals (`"string"`).
  - Control Flow: `if-else`, `while` loops, `for` loops, `return` statements, blocks (`{}`).
  - Declarations: Global and local variables (int, char), pointers (`*`), arrays (`[]`), function definitions and calls, `enum` declarations.
  - Basic `printf` support for `%d` and `%s` format specifiers.
- **Virtual Machine**: Executes the compiled bytecode using a stack-based architecture. Supports basic system calls like `printf`, `exit`.
- **Error Handling**: Uses Rust's `Result` type for error propagation during parsing and execution.
- **Testing**: Includes unit tests for lexer, parser components, VM execution, pointer operations, memory access, and basic self-hosting checks.

## Known Limitations

- **Self-Hosting**: While the parser can process most of the original `c4.c` source, it currently skips or has workarounds for specific complex expressions involving intricate pointer arithmetic and bitwise operations (notably around lines 58-61 and 73 in `c4.c`). Full self-compilation equivalent to the original C4 is not yet achieved due to these complex C idioms.
- **Array Implementation**: Array indexing is supported, but the underlying memory management and code generation for complex array operations might still have bugs or lead to VM issues like infinite loops in specific scenarios (as noted in `main.rs`). The `simple_array_test.c` works, but more complex uses might fail.
- **String Escapes**: Basic string escapes (`\n`, `\t`, `\\`, `\"`, `\'`, `\0`) are handled, but more complex C escape sequences (hex, octal) might not be fully supported.
- **Memory Model**: The VM uses separate data and stack segments with a simplified memory model compared to a real C environment. Direct memory manipulation beyond stack operations and basic data access might behave differently.
- **System Calls**: Only a subset of the original C4 system calls (`printf`, `exit`, `malloc`, `memset`, `memcmp`) are implemented. File I/O (`open`, `read`, `close`) is stubbed.

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
# Run in normal mode (clean output)
cargo run <input-file>

# Run with debug mode to see detailed execution information
cargo run -- -d <input-file>

# Run with source and assembly output
cargo run -- -s <input-file>

# Or use the compiled binary directly
./target/debug/c4_rust <input-file>
./target/debug/c4_rust -d <input-file>
./target/debug/c4_rust -s <input-file>
```

### Command-Line Options

- `-d`: Debug mode - print detailed execution information including parser debug info, VM instruction traces and memory operation details
- `-s`: Source mode - print parsed source and generated assembly
- `<input-file>`: C source file to compile and run

### Example Programs

The project includes several example C programs that can be used to test the compiler:

```bash
# Run a simple "Hello, World!" program
cargo run single_variable_test.c

# Run a more complex test
cargo run complex_test.c

# Run with debug information
cargo run -- -d single_variable_test.c
```

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

## Documentation

Generate and view the documentation with:

```bash
# Generate documentation
cargo doc --no-deps

# Open documentation in browser
cargo doc --no-deps --open
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

# Test self-hosting capability
cargo test test_self_hosting_capabilities -- --nocapture

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
- `vm_memory_tests.rs`: Tests for VM memory functions (load_int, load_char, store_int, store_char)
- `self_hosting_test.rs`: Tests that verify the compiler's ability to parse its own source code

## Recent Fixes and Improvements

- Fixed printf system call to handle format strings better
- Implemented proper lexer tests that match the lexer's actual behavior
- Corrected parameter handling in the parser
- Added error handling for invalid format string addresses
- Fixed unused variable warnings
- Enhanced code generation for pointers and function calls
- Created a convenient testing script to verify all functionality
- Added tests for VM memory functions to improve test coverage
- Implemented self-hosting capability testing
- Generated comprehensive code documentation

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
- [x] Code documentation
- [x] Self-hosting verification testing
- [x] Basic Type System (int, char, pointers)
- [x] Expression Parsing (arithmetic, logical, bitwise, assignment, function calls, sizeof, casting)
- [x] Statement Parsing (if-else, while, for, return, blocks, expression statements)
- [x] Variable Declarations (global, local, parameters)
- [x] Pointer Operations (address-of `&`, dereference `*`, basic arithmetic)
- [x] Array Declarations and Indexing (`[]`)
- [x] String Literals and Basic `printf` (`%d`, `%s`)
- [x] Enum Declarations
- [x] VM Implementation (stack machine, basic instruction set)
- [x] System Calls (`printf`, `exit`, `malloc`, `memset`, `memcmp`)
- [x] Partial Self-Hosting Capability (parses most of `c4.c` with known skips)
- [x] Unit Testing (lexer, parser, VM, pointers, memory)
- [x] Code Documentation (`cargo doc`)
- [ ] Full Self-Hosting Equivalence
- [ ] Complete System Call Implementation (File I/O)
- [ ] Robust Array Handling for all cases
- [ ] Advanced String Escape Sequence Support

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Robert Swierczek for the original C4 compiler
- University assignment that inspired this implementation 