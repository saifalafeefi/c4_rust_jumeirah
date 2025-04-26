/// parser module for analyzing C source code
/// parses tokens and generates intermediate code for the VM

use crate::lexer::{Lexer, Token};

/// type identifiers for parsed expressions
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Char,
    Int,
    Ptr(Box<Type>),
}

impl Type {
    pub fn is_ptr(&self) -> bool {
        matches!(self, Type::Ptr(_))
    }
    
    pub fn base_type(&self) -> Option<Box<Type>> {
        match self {
            Type::Ptr(base) => Some(base.clone()),
            _ => None,
        }
    }
    
    pub fn size(&self) -> usize {
        match self {
            Type::Char => 1,
            Type::Int | Type::Ptr(_) => std::mem::size_of::<i64>(),
        }
    }
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
    pub name: String,
    pub class: SymbolClass,
    pub typ: Type,
    pub value: i64,
    // for local variables that are shadowing global ones
    pub prev_class: Option<SymbolClass>,
    pub prev_type: Option<Type>,
    pub prev_value: Option<i64>,
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
    pub fn init(&mut self) -> Result<(), String> {
        // Add keywords to symbol table
        self.add_keyword("char", 128 + 6)?;  // Token::Char
        self.add_keyword("else", 128 + 7)?;  // Token::Else
        self.add_keyword("enum", 128 + 8)?;  // Token::Enum
        self.add_keyword("if", 128 + 9)?;    // Token::If
        self.add_keyword("int", 128 + 10)?;  // Token::Int
        self.add_keyword("return", 128 + 11)?; // Token::Return
        self.add_keyword("sizeof", 128 + 12)?; // Token::Sizeof
        self.add_keyword("while", 128 + 13)?;  // Token::While
        self.add_keyword("void", 128 + 18)?;   // Token::Void
        
        // Add system calls
        self.add_syscall("open", OpCode::OPEN as i64)?;
        self.add_syscall("read", OpCode::READ as i64)?;
        self.add_syscall("close", OpCode::CLOS as i64)?;
        self.add_syscall("printf", OpCode::PRTF as i64)?;
        self.add_syscall("malloc", OpCode::MALC as i64)?;
        self.add_syscall("free", OpCode::FREE as i64)?;
        self.add_syscall("memset", OpCode::MSET as i64)?;
        self.add_syscall("memcmp", OpCode::MCMP as i64)?;
        self.add_syscall("exit", OpCode::EXIT as i64)?;
        
        // Start tokenizing
        self.lexer.next();
        
        Ok(())
    }
    
    /// add a keyword to the symbol table
    fn add_keyword(&mut self, name: &str, token_value: i64) -> Result<(), String> {
        let symbol = Symbol {
            name: name.to_string(),
            class: SymbolClass::Num,
            typ: Type::Int,
            value: token_value,
            prev_class: None,
            prev_type: None,
            prev_value: None,
        };
        
        self.symbols.push(symbol);
        Ok(())
    }
    
    /// add a system call to the symbol table
    fn add_syscall(&mut self, name: &str, id: i64) -> Result<(), String> {
        let symbol = Symbol {
            name: name.to_string(),
            class: SymbolClass::Sys,
            typ: Type::Int,
            value: id,
            prev_class: None,
            prev_type: None,
            prev_value: None,
        };
        
        self.symbols.push(symbol);
        Ok(())
    }
    
    /// find a symbol in the symbol table by name
    fn find_symbol(&self, name: &str) -> Option<&Symbol> {
        self.symbols.iter().find(|s| s.name == name)
    }
    
    /// add a new symbol to the symbol table
    fn add_symbol(&mut self, name: &str, class: SymbolClass, typ: Type, value: i64) -> Result<&mut Symbol, String> {
        // Check if symbol already exists
        if let Some(_) = self.find_symbol(name) {
            return Err(format!("Symbol '{}' already defined", name));
        }
        
        // Add new symbol
        let symbol = Symbol {
            name: name.to_string(),
            class,
            typ,
            value,
            prev_class: None,
            prev_type: None,
            prev_value: None,
        };
        
        self.symbols.push(symbol);
        Ok(self.symbols.last_mut().unwrap())
    }
    
    /// get the current token
    fn token(&self) -> Token {
        self.lexer.token()
    }
    
    /// advance to next token
    fn next(&mut self) -> Token {
        self.lexer.next()
    }
    
    /// expect a specific token and advance to next token
    fn expect(&mut self, token: Token, error_msg: &str) -> Result<(), String> {
        if self.token() == token {
            self.next();
            Ok(())
        } else {
            Err(format!("Line {}: {}", self.lexer.line(), error_msg))
        }
    }
    
    /// parse all declarations and return the generated code
    pub fn parse(&mut self) -> Result<(Vec<i64>, Vec<u8>), String> {
        self.init()?;
        
        // Main parsing loop for declarations
        while self.token() != Token::Eof {
            self.declaration()?;
        }
        
        // Find main function
        let main_sym = self.find_symbol("main").ok_or("main() not defined")?;
        if main_sym.class != SymbolClass::Fun {
            return Err("main is not a function".to_string());
        }
        
        // Return the generated code and data segments
        Ok((self.code.clone(), self.data.clone()))
    }
    
    /// parse a declaration (variable or function)
    fn declaration(&mut self) -> Result<(), String> {
        let mut base_type = Type::Int; // default to int
        
        // Parse type
        if self.token() == Token::Int {
            base_type = Type::Int;
            self.next();
        } else if self.token() == Token::Char {
            base_type = Type::Char;
            self.next();
        } else if self.token() == Token::Void {
            base_type = Type::Int; // void is treated as int in symbol table, but differently in function calls
            self.next();
        } else if self.token() == Token::Enum {
            self.parse_enum()?;
            return Ok(());
        }
        
        // Parse declarator list
        while self.token() != Token::Semicolon && self.token() != Token::RightBrace {
            let mut typ = base_type.clone();
            
            // Parse pointer levels
            while self.token() == Token::Mul {
                self.next();
                typ = Type::Ptr(Box::new(typ));
            }
            
            // Expect identifier
            if let Token::Id(id) = self.token() {
                let name = self.get_id_name(id);
                self.next();
                
                // Function definition
                if self.token() == Token::LeftParen {
                    self.parse_function(name, typ)?;
                    return Ok(());
                }
                
                // Variable declaration - first get the data length
                let data_len = self.data.len();
                let type_size = typ.size();
                
                // Add symbol to table
                self.add_symbol(&name, SymbolClass::Glo, typ, data_len as i64)?;
                
                // Add space in data segment
                self.data.resize(data_len + type_size, 0);
            } else {
                return Err(format!("Line {}: Expected identifier in declaration", self.lexer.line()));
            }
            
            // Handle multiple variables in one declaration
            if self.token() == Token::Comma {
                self.next();
                continue;
            }
            
            break;
        }
        
        // End of declaration
        self.expect(Token::Semicolon, "Expected semicolon after variable declaration")?;
        
        Ok(())
    }
    
    /// parse an enum declaration
    fn parse_enum(&mut self) -> Result<(), String> {
        self.next(); // Skip 'enum'
        
        // Optional enum name (ignored in C4)
        if let Token::Id(_) = self.token() {
            self.next();
        }
        
        // Enum body
        self.expect(Token::LeftBrace, "Expected '{' after enum")?;
        
        let mut value = 0;
        while self.token() != Token::RightBrace {
            // Enum member must be an identifier
            if let Token::Id(id) = self.token() {
                let name = self.get_id_name(id);
                self.next();
                
                // Check for explicit value
                if self.token() == Token::Assign {
                    self.next();
                    if let Token::Num(val) = self.token() {
                        value = val;
                        self.next();
                    } else {
                        return Err(format!("Line {}: Expected numeric value after '='", self.lexer.line()));
                    }
                }
                
                // Add enum value to symbol table
                self.add_symbol(&name, SymbolClass::Num, Type::Int, value)?;
                
                // Increment value for next enum member
                value += 1;
                
                // Comma or end of enum
                if self.token() == Token::Comma {
                    self.next();
                }
            } else {
                return Err(format!("Line {}: Expected identifier in enum declaration", self.lexer.line()));
            }
        }
        
        self.next(); // Skip closing brace
        self.expect(Token::Semicolon, "Expected semicolon after enum declaration")?;
        
        Ok(())
    }
    
    /// parse a function definition
    fn parse_function(&mut self, name: String, return_type: Type) -> Result<(), String> {
        // Mark current position in the code segment
        let fn_pos = self.code.len();
        
        // Add function to symbol table
        let _symbol = self.add_symbol(&name, SymbolClass::Fun, return_type, fn_pos as i64)?;
        
        // Parse parameter list
        self.next(); // Skip '('
        let mut _param_count = 0;
        
        // Parameter handling would go here (simplified)
        while self.token() != Token::RightParen {
            // Skip parameter handling for now
            self.next();
            _param_count += 1;
            
            // Handle commas between parameters
            if self.token() == Token::Comma {
                self.next();
            }
        }
        
        self.next(); // Skip ')'
        
        // Function body handling would go here (simplified)
        self.expect(Token::LeftBrace, "Expected '{' to start function body")?;
        
        // Add placeholder for function body
        // Here we'd handle local variables and statements
        
        // Skip to end of function
        let mut brace_level = 1;
        while brace_level > 0 {
            if self.token() == Token::LeftBrace {
                brace_level += 1;
            } else if self.token() == Token::RightBrace {
                brace_level -= 1;
            } else if self.token() == Token::Eof {
                return Err(format!("Line {}: Unexpected end of file in function body", self.lexer.line()));
            }
            self.next();
        }
        
        Ok(())
    }
    
    /// get the name of an identifier from its hash
    fn get_id_name(&self, id: usize) -> String {
        // In our implementation, we just generate a placeholder name
        // In the real implementation, we'd look this up from a symbol table
        format!("id_{}", id)
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
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_symbol_table() {
        let mut parser = Parser::new("int main() {}", false);
        parser.init().unwrap();
        
        // Verify keywords are added
        assert!(parser.find_symbol("int").is_some());
        assert!(parser.find_symbol("char").is_some());
        assert!(parser.find_symbol("if").is_some());
        
        // Verify syscalls are added
        assert!(parser.find_symbol("printf").is_some());
        assert!(parser.find_symbol("malloc").is_some());
        
        // Add a new symbol
        parser.add_symbol("test_var", SymbolClass::Glo, Type::Int, 0).unwrap();
        let sym = parser.find_symbol("test_var").unwrap();
        assert_eq!(sym.name, "test_var");
        assert_eq!(sym.class, SymbolClass::Glo);
        assert!(matches!(sym.typ, Type::Int));
    }
    
    #[test]
    fn test_type_size() {
        assert_eq!(Type::Char.size(), 1);
        assert_eq!(Type::Int.size(), 8); // 64-bit system
        assert_eq!(Type::Ptr(Box::new(Type::Char)).size(), 8);
    }
} 