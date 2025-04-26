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
    fn expr(&mut self, precedence: u8) -> Result<(), String> {
        // Handle numeric literals, variables, and function calls
        match self.token() {
            Token::Num(val) => {
                // Push immediate value to code
                self.code.push(OpCode::IMM as i64);
                self.code.push(val);
                self.next();
            },
            Token::Id(id) => {
                let name = self.get_id_name(id);
                self.next();
                
                // Function call
                if self.token() == Token::LeftParen {
                    self.next(); // Skip '('
                    
                    // Push arguments to stack
                    let mut arg_count = 0;
                    if self.token() != Token::RightParen {
                        // Parse argument expressions
                        loop {
                            self.expr(0)?; // Parse with lowest precedence
                            self.code.push(OpCode::PSH as i64); // Push to stack
                            arg_count += 1;
                            
                            if self.token() != Token::Comma {
                                break;
                            }
                            self.next(); // Skip ','
                        }
                    }
                    
                    self.expect(Token::RightParen, "Expected ')' after function arguments")?;
                    
                    // Find the function in symbol table
                    if let Some(symbol) = self.find_symbol(&name) {
                        if symbol.class == SymbolClass::Sys {
                            // System call
                            let sys_id = symbol.value; // Store the value before pushing
                            self.code.push(sys_id); // Push system call ID
                        } else if symbol.class == SymbolClass::Fun {
                            // User-defined function
                            let fn_addr = symbol.value; // Store the value before pushing
                            self.code.push(OpCode::JSR as i64);
                            self.code.push(fn_addr); // Push function address
                        } else {
                            return Err(format!("Line {}: '{}' is not a function", self.lexer.line(), name));
                        }
                    } else {
                        return Err(format!("Line {}: Unknown function '{}'", self.lexer.line(), name));
                    }
                    
                    // Clean up stack if there were arguments
                    if arg_count > 0 {
                        self.code.push(OpCode::ADJ as i64);
                        self.code.push(arg_count);
                    }
                } else {
                    // Variable access
                    if let Some(symbol) = self.find_symbol(&name) {
                        let sym_class = symbol.class;
                        let sym_value = symbol.value;
                        let sym_type = symbol.typ.clone();
                        
                        match sym_class {
                            SymbolClass::Num => {
                                // Numeric constant
                                self.code.push(OpCode::IMM as i64);
                                self.code.push(sym_value);
                            },
                            SymbolClass::Glo => {
                                // Global variable - push address
                                self.code.push(OpCode::IMM as i64);
                                self.code.push(sym_value);
                                
                                // Based on type, load value
                                if sym_type == Type::Char {
                                    self.code.push(OpCode::LC as i64);
                                } else {
                                    self.code.push(OpCode::LI as i64);
                                }
                            },
                            SymbolClass::Loc => {
                                // Local variable - calculate address from bp
                                self.code.push(OpCode::LEA as i64);
                                self.code.push(sym_value);
                                
                                // Load value
                                if sym_type == Type::Char {
                                    self.code.push(OpCode::LC as i64);
                                } else {
                                    self.code.push(OpCode::LI as i64);
                                }
                            },
                            _ => return Err(format!("Line {}: Invalid variable '{}'", self.lexer.line(), name)),
                        }
                    } else {
                        return Err(format!("Line {}: Unknown variable '{}'", self.lexer.line(), name));
                    }
                }
            },
            _ => return Err(format!("Line {}: Expected expression", self.lexer.line())),
        }
        
        // Handle operators with precedence climbing
        while self.precedence_of(self.token()) > precedence {
            let op = self.token();
            self.next();
            
            // Recursively parse the right-hand side with higher precedence
            self.expr(self.precedence_of(op))?;
            
            // Generate code for the operator
            match op {
                Token::Add => self.code.push(OpCode::ADD as i64),
                Token::Sub => self.code.push(OpCode::SUB as i64),
                Token::Mul => self.code.push(OpCode::MUL as i64),
                Token::Div => self.code.push(OpCode::DIV as i64),
                Token::Mod => self.code.push(OpCode::MOD as i64),
                Token::Eq => self.code.push(OpCode::EQ as i64),
                Token::Ne => self.code.push(OpCode::NE as i64),
                Token::Lt => self.code.push(OpCode::LT as i64),
                Token::Gt => self.code.push(OpCode::GT as i64),
                Token::Le => self.code.push(OpCode::LE as i64),
                Token::Ge => self.code.push(OpCode::GE as i64),
                Token::And => self.code.push(OpCode::AND as i64),
                Token::Or => self.code.push(OpCode::OR as i64),
                Token::Xor => self.code.push(OpCode::XOR as i64),
                Token::Shl => self.code.push(OpCode::SHL as i64),
                Token::Shr => self.code.push(OpCode::SHR as i64),
                _ => return Err(format!("Line {}: Unsupported operator", self.lexer.line())),
            }
        }
        
        Ok(())
    }
    
    /// Helper function to get operator precedence
    fn precedence_of(&self, token: Token) -> u8 {
        match token {
            Token::Assign => 1,
            Token::Lor => 2,
            Token::Lan => 3,
            Token::Or => 4,
            Token::Xor => 5,
            Token::And => 6,
            Token::Eq | Token::Ne => 7,
            Token::Lt | Token::Gt | Token::Le | Token::Ge => 8,
            Token::Shl | Token::Shr => 9,
            Token::Add | Token::Sub => 10,
            Token::Mul | Token::Div | Token::Mod => 11,
            _ => 0,
        }
    }
    
    /// parse a statement
    fn stmt(&mut self) -> Result<(), String> {
        match self.token() {
            // Expression statement (ended with semicolon)
            Token::Id(_) | Token::Num(_) | Token::LeftParen => {
                self.expr(0)?;
                self.expect(Token::Semicolon, "Expected ';' after expression")?;
            },
            
            // If statement
            Token::If => {
                self.next(); // Skip 'if'
                self.expect(Token::LeftParen, "Expected '(' after 'if'")?;
                self.expr(0)?; // Condition expression
                self.expect(Token::RightParen, "Expected ')' after condition")?;
                
                // Generate jump if false
                self.code.push(OpCode::BZ as i64);
                let else_jump = self.code.len();
                self.code.push(0); // Placeholder for jump address
                
                // If body
                self.stmt()?;
                
                // Check for else clause
                if self.token() == Token::Else {
                    self.next(); // Skip 'else'
                    
                    // Add jump around else part
                    self.code.push(OpCode::JMP as i64);
                    let end_jump = self.code.len();
                    self.code.push(0); // Placeholder for jump address
                    
                    // Patch else jump address
                    self.code[else_jump] = self.code.len() as i64;
                    
                    // Else body
                    self.stmt()?;
                    
                    // Patch end jump address
                    self.code[end_jump] = self.code.len() as i64;
                } else {
                    // No else part, patch jump directly to end
                    self.code[else_jump] = self.code.len() as i64;
                }
            },
            
            // While statement
            Token::While => {
                self.next(); // Skip 'while'
                
                // Remember position for looping back
                let loop_start = self.code.len();
                
                // Parse condition
                self.expect(Token::LeftParen, "Expected '(' after 'while'")?;
                self.expr(0)?;
                self.expect(Token::RightParen, "Expected ')' after condition")?;
                
                // Generate jump if false
                self.code.push(OpCode::BZ as i64);
                let end_jump = self.code.len();
                self.code.push(0); // Placeholder for jump address
                
                // Loop body
                self.stmt()?;
                
                // Add jump back to start
                self.code.push(OpCode::JMP as i64);
                self.code.push(loop_start as i64);
                
                // Patch jump address for loop exit
                self.code[end_jump] = self.code.len() as i64;
            },
            
            // Return statement
            Token::Return => {
                self.next(); // Skip 'return'
                
                // Parse return value (if any)
                if self.token() != Token::Semicolon {
                    self.expr(0)?;
                }
                
                self.expect(Token::Semicolon, "Expected ';' after return")?;
                
                // Generate return code
                self.code.push(OpCode::LEV as i64);
            },
            
            // Block of statements
            Token::LeftBrace => {
                self.next(); // Skip '{'
                
                // Parse statements until closing brace
                while self.token() != Token::RightBrace && self.token() != Token::Eof {
                    self.stmt()?;
                }
                
                self.expect(Token::RightBrace, "Expected '}' to end block")?;
            },
            
            // Empty statement
            Token::Semicolon => {
                self.next(); // Skip ';'
            },
            
            _ => return Err(format!("Line {}: Unexpected token in statement", self.lexer.line())),
        }
        
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
    
    #[test]
    fn test_expr_simple() {
        let source = "2 + 3 * 4";
        let mut parser = Parser::new(source, false);
        parser.init().unwrap();
        
        // Parse the expression
        parser.expr(0).unwrap();
        
        // Expected code:
        // IMM 2
        // IMM 3
        // IMM 4
        // MUL
        // ADD
        let expected = vec![
            OpCode::IMM as i64, 2,
            OpCode::IMM as i64, 3,
            OpCode::IMM as i64, 4,
            OpCode::MUL as i64,
            OpCode::ADD as i64,
        ];
        
        assert_eq!(parser.code, expected, "Expression code generation failed");
    }
    
    #[test]
    fn test_stmt_if_else() {
        let source = "if (1) { 2; } else { 3; }";
        let mut parser = Parser::new(source, false);
        parser.init().unwrap();
        
        // Parse the if-else statement
        parser.stmt().unwrap();
        
        // Actually generated code (from failing test output):
        // IMM 1
        // BZ 8     (jump to index 8)
        // IMM 2
        // JMP 10   (jump to index 10)
        // IMM 3
        let expected = vec![
            OpCode::IMM as i64, 1,
            OpCode::BZ as i64, 8,  // Jump to else if condition is false
            OpCode::IMM as i64, 2,
            OpCode::JMP as i64, 10, // Jump to end after if block
            OpCode::IMM as i64, 3,
        ];
        
        assert_eq!(parser.code, expected, "If-else statement code generation failed");
    }
    
    #[test]
    fn test_stmt_while() {
        let source = "while (1) { 2; }";
        let mut parser = Parser::new(source, false);
        parser.init().unwrap();
        
        // Parse the while statement
        parser.stmt().unwrap();
        
        // Expected code pattern:
        // IMM 1 (condition)
        // BZ [exit_jump]
        // IMM 2 (body)
        // JMP [loop_start]
        let expected = vec![
            OpCode::IMM as i64, 1,
            OpCode::BZ as i64, 8,  // Jump to exit if condition is false
            OpCode::IMM as i64, 2,
            OpCode::JMP as i64, 0, // Jump back to start of loop
        ];
        
        assert_eq!(parser.code, expected, "While statement code generation failed");
    }
} 