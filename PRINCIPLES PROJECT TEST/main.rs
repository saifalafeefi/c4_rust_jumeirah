// Rust implementation of c4.c
// This is a minimal implementation to support basic functionality

mod c4;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut compiler = c4::C4::new();
    
    std::process::exit(compiler.run(args) as i32);
} 