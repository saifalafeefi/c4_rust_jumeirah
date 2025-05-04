/// entry point for c4
/// handles args and setup

pub mod lexer;
pub mod parser;
pub mod vm;

use std::env;
use std::fs::File;
use std::io::Read;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let mut src = false;
    let mut debug = false;
    
    // Process flags
    let mut arg_index = 1;
    while arg_index < args.len() && args[arg_index].starts_with('-') {
        match args[arg_index].as_str() {
            "-s" => src = true,
            "-d" => debug = true,
            _ => {
                eprintln!("unknown option: {}", args[arg_index]);
                process::exit(1);
            }
        }
        arg_index += 1;
    }
    
    // Check if a source file is provided
    if arg_index >= args.len() {
        eprintln!("usage: c4_rust [-s] [-d] file ...");
        process::exit(1);
    }
    
    // Get filename
    let filename = &args[arg_index];
    
    // Open source file
    let mut file = match File::open(filename) {
        Ok(f) => f,
        Err(_) => {
            eprintln!("could not open({})", filename);
            process::exit(1);
        }
    };
    
    // Read source file
    let mut source = String::new();
    if let Err(_) = file.read_to_string(&mut source) {
        eprintln!("could not read file");
        process::exit(1);
    }
    
    // Parse the source
    let mut parser = match parser::Parser::new(&source, debug) {
        mut p => {
            if let Err(e) = p.init() {
                eprintln!("Parser initialization failed: {}", e);
                process::exit(1);
            }
            p
        }
    };
    
    // Parse and get code/data
    let (code, data) = match parser.parse() {
        Ok((c, d)) => (c, d),
        Err(e) => {
            eprintln!("Parsing failed: {}", e);
            process::exit(1);
        }
    };
    
    // Early return if only parsing source
    if src {
        println!("Source parsed successfully.");
        process::exit(0);
    }
    
    // Print a clean starting message if not in debug mode
    if !debug {
        println!("C4_RUST RUNNING...");
        println!("--------");
    }
    
    // Create VM with debug mode setting
    let mut vm = vm::VM::new(code, data, debug);
    
    // Run program once and get result
    match vm.run() {
        Ok(value) => {
            if !debug {
                println!("--------");
                println!("END OF OUTPUT, QUITTING...");
            }
            if debug {
                println!("Program executed successfully with return value: {}", value);
            }
        },
        Err(e) => {
            if !debug {
                println!("--------");
                println!("END OF OUTPUT, QUITTING...");
            }
            
            if e.contains("instruction limit") {
                eprintln!("Program terminated due to possible infinite loop");
                eprintln!("This is a known issue with array access in our implementation.");
                eprintln!("The array feature still has bugs in code generation for array indexing.");
                process::exit(1);
            } else {
                eprintln!("Runtime error: {}", e);
                process::exit(1);
            }
        }
    }
}
