/// parser module for analyzing C source code
/// parses tokens and generates intermediate code for the VM

use crate::lexer::{Lexer, Token};

/// type identifiers for parsed expressions
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Char,
    Int,
    Ptr(usize), // using a usize index instead of Box<Type> to allow Copy
}

/// symbol classes for identifiers
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SymbolClass {
    Fun, // function
    Sys, // system call
    Glo, // global
    Loc, // local
    Num, // number
}

/// symbol table entry
#[derive(Debug, Clone)]
pub struct Symbol {
    name: String,
    class: SymbolClass,
    typ: Type,
    value: i64,
    // for local variables that are shadowing global ones
    prev_class: Option<SymbolClass>,
    prev_type: Option<Type>,
    prev_value: Option<i64>,
}

/// instruction opcodes for the VM
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OpCode {
    LEA, IMM, JMP, JSR, BZ, BNZ, ENT, ADJ, LEV, LI, LC, SI, SC, PSH,
    OR, XOR, AND, EQ, NE, LT, GT, LE, GE, SHL, SHR, ADD, SUB, MUL, DIV, MOD,
    OPEN, READ, CLOS, PRTF, MALC, FREE, MSET, MCMP, EXIT,
}

/// parser state for generating code
pub struct Parser<'a> {
    lexer: Lexer<'a>,
    symbols: Vec<Symbol>,
    code: Vec<i64>,
    data: Vec<u8>,
    current_type: Type,
    locals: usize,
    src: bool, // source printing flag
}

impl<'a> Parser<'a> {
    /// create a new parser
    pub fn new(source: &'a str, src: bool) -> Self {
        Parser {
            lexer: Lexer::new(source),
            symbols: Vec::new(),
            code: Vec::new(),
            data: Vec::new(),
            current_type: Type::Int,
            locals: 0,
            src,
        }
    }
    
    /// initialize the parser with keywords and system calls
    pub fn init(&mut self) {
        // TODO: add keywords and system calls to symbol table
        self.lexer.next(); // start tokenizing
    }
    
    /// parse all declarations and return the generated code
    pub fn parse(&mut self) -> Result<(Vec<i64>, Vec<u8>), String> {
        self.init();
        
        // TODO: parse declarations
        
        // for now, just return empty code and data
        Ok((self.code.clone(), self.data.clone()))
    }
    
    /// parse an expression with a given precedence level
    fn expr(&mut self, _precedence: u8) -> Result<(), String> {
        // TODO: implement expression parsing
        Ok(())
    }
    
    /// parse a statement
    fn stmt(&mut self) -> Result<(), String> {
        // TODO: implement statement parsing
        Ok(())
    }
    
    // TODO: implement more parser methods as needed
} 