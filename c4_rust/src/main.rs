/// main entry point for the c4 compiler
/// handles command line arguments and initializes the compiler

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
    let mut arg_index = 1;
    
    // parse command line flags
    while arg_index < args.len() && args[arg_index].starts_with('-') {
        match args[arg_index].as_str() {
            "-s" => {
                src = true;
                arg_index += 1;
            },
            "-d" => {
                debug = true;
                arg_index += 1;
            },
            _ => {
                eprintln!("Unknown option: {}", args[arg_index]);
                arg_index += 1;
            }
        }
    }
    
    if arg_index >= args.len() {
        eprintln!("usage: c4_rust [-s] [-d] file ...");
        process::exit(1);
    }
    
    let filename = &args[arg_index];
    
    // read source file
    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(_) => {
            eprintln!("could not open({})", filename);
            process::exit(1);
        }
    };
    
    let mut source = String::new();
    if let Err(_) = file.read_to_string(&mut source) {
        eprintln!("could not read file");
        process::exit(1);
    }
    
    // initialize compiler and run
    if let Err(err) = vm::run(&source, src, debug) {
        eprintln!("{}", err);
        process::exit(1);
    }
}
