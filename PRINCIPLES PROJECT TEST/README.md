# C4 Rust Implementation

A Rust implementation of the C4 compiler by Robert Swierczek.

## Description

C4 is a small self-hosting C compiler that fits in 4 functions. This project is a Rust implementation designed to demonstrate the basic functionality while showing what's possible with Rust's safety features.

## Current Status

This implementation includes:

- Variable tracking and expression evaluation
- Control flow (if/else statements, while loops, for loops)
- Basic array support with initialization
- Function call support (simplified)
- Format string handling in printf statements
- Arithmetic operations (+, -, *, /)
- Comparison operators (>, <, >=, <=, ==, !=)

## Building the Project

To build the project, make sure you have Rust and Cargo installed, then run:

```bash
cargo build --release
```

This will create an executable in the `target/release` directory.

## Running the Compiler

You can run the compiled C4 with:

```bash
./target/release/c4_rust path/to/your/file.c
```

## Example

The implementation works with the provided example files:

- `hello.c` - A simple "Hello, World" program
- `test_variables.c` - A test program demonstrating variable operations, control flow, and arrays

```c
// hello.c
int main() {
    printf("Hello, world from c4!\n");
    return 0;
}
```

```c
// Variable example
int main() {
    int a = 5;
    int b = 7;
    
    printf("a + b = %d\n", a + b);  // Will print "a + b = 12"
    
    // Control flow works too!
    if (a < b) {
        printf("a is less than b\n");
    }
    
    return 0;
}
```

## Project Structure

- `src/main.rs` - The simplified implementation with basic functionality
- `src/c4.rs` - A more complete (but not yet fully functional) implementation of the full C4 compiler

## Development Roadmap

Future improvements planned:

1. Complete the full C4 implementation in Rust
2. Support for nested function calls
3. Enhanced type system implementation
4. Support for the full VM implementation
5. Proper compiler optimizations

## License

This project is licensed under the MIT License. 