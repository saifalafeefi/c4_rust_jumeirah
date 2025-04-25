# C4 Rust

A Rust implementation of the [C4 compiler](https://github.com/rswier/c4) originally written by Robert Swierczek.

## Overview

C4 is a small self-hosting C compiler that compiles a subset of C. This project reimplements C4 in Rust, maintaining its self-hosting capability and core functionality while leveraging Rust's safety features and modern programming paradigms.

## Features

- Tokenizes and parses the same subset of C as the original C4
- Supports self-compilation (can compile its original C code)
- Uses a virtual machine to execute the compiled code
- Preserves the minimal design of C4 while using Rust idioms

## Building

To build the project:

```
cargo build --release
```

## Usage

```
c4_rust [-s] [-d] file
```

Options:
- `-s`: Print source and assembly
- `-d`: Print executed instructions
- `file`: C source file to compile

## Example

To compile and run the original C4 source:

```
c4_rust c4.c
```

## Tests

To run the tests:

```
cargo test
```

## License

Same as the original C4. 