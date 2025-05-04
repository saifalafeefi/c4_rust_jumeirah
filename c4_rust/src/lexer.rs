/// tokenizes C code
/// makes tokens for parser

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Token {
    // constants and identifiers
    Num(i64),
    Str(usize), // index to string buffer
    Id(usize),  // index to symbol table
    
    // keywords
    Char,
    Else,
    Enum,
    For,
    If,
    Int,
    Return,
    Sizeof,
    While,
    Void,
    
    // operators by precedence
    Assign,
    AddAssign,   // +=
    SubAssign,   // -=
    MulAssign,   // *=
    DivAssign,   // /=
    ModAssign,   // %=
    ShlAssign,   // <<=
    ShrAssign,   // >>=
    AndAssign,   // &=
    XorAssign,   // ^=
    OrAssign,    // |=
    Cond,
    Lor,
    Lan,
    Or,
    Xor,
    And,
    Eq,
    Ne,
    Lt,
    Gt,
    Le,
    Ge,
    Shl,
    Shr,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Inc,
    Dec,
    Brak,        // [ for array indexing
    
    // special chars
    Semicolon,
    LeftBrace,
    RightBrace,
    LeftParen,
    RightParen,
    LeftBracket,  // [ for array declaration
    RightBracket, // ]
    Comma,
    Colon,
    Tilde,
    Not,
    
    // internal tokens
    Eof,
}

#[derive(Debug)]
pub struct Lexer<'a> {
    source: &'a str,
    chars: std::iter::Peekable<std::str::Chars<'a>>,
    pos: usize,
    line: usize,
    current_token: Token,
    current_value: i64,
    string_buffer: Vec<u8>,
    lp: usize, // for source printing
    debug: bool, // debug flag
}

impl<'a> Lexer<'a> {
    /// creates new lexer
    pub fn new(source: &'a str) -> Self {
        Lexer {
            source,
            chars: source.chars().peekable(),
            pos: 0,
            line: 1,
            current_token: Token::Eof,
            current_value: 0,
            string_buffer: Vec::new(),
            lp: 0,
            debug: false, // default to no debug output
        }
    }
    
    /// sets debug flag
    pub fn set_debug(&mut self, debug: bool) {
        self.debug = debug;
    }
    
    /// gets current token
    pub fn token(&self) -> Token {
        self.current_token
    }
    
    /// gets numeric value
    pub fn value(&self) -> i64 {
        self.current_value
    }
    
    /// gets line number
    pub fn line(&self) -> usize {
        self.line
    }
    
    /// gets string buffer
    pub fn string_buffer(&self) -> &[u8] {
        &self.string_buffer
    }
    
    /// moves to next token
    pub fn next(&mut self) -> Token {
        // skip spaces and comments
        self.skip_whitespace();
        
        // check for EOF
        if let None = self.chars.peek() {
            self.current_token = Token::Eof;
            return self.current_token;
        }
        
        // process next char
        match self.chars.next() {
            Some(c) => {
                self.pos += 1;
                match c {
                    // identifiers and keywords
                    'a'..='z' | 'A'..='Z' | '_' => {
                        let mut hash = c as u64;
                        let start_pos = self.pos - 1;
                        
                        // read whole identifier
                        while let Some(&next_c) = self.chars.peek() {
                            if next_c.is_alphanumeric() || next_c == '_' {
                                hash = hash.wrapping_mul(147).wrapping_add(next_c as u64);
                                self.chars.next();
                                self.pos += 1;
                            } else {
                                break;
                            }
                        }
                        
                        let id_length = self.pos - start_pos;
                        hash = (hash << 6) + id_length as u64;
                        
                        // check if keyword
                        let id_str = &self.source[start_pos..self.pos];
                        self.current_token = match id_str {
                            "char" => Token::Char,
                            "else" => Token::Else,
                            "enum" => Token::Enum,
                            "for" => Token::For,
                            "if" => Token::If,
                            "int" => Token::Int,
                            "return" => Token::Return,
                            "sizeof" => Token::Sizeof,
                            "while" => Token::While,
                            "void" => Token::Void,
                            // otherwise identifier
                            _ => Token::Id(hash as usize),
                        };
                    },
                    
                    // numbers
                    '0'..='9' => {
                        let mut value = (c as i64) - ('0' as i64);
                        
                        // hex number
                        if value == 0 && self.chars.peek() == Some(&'x') || self.chars.peek() == Some(&'X') {
                            self.chars.next(); // consume 'x'
                            self.pos += 1;
                            
                            while let Some(&next_c) = self.chars.peek() {
                                if next_c.is_digit(16) {
                                    let digit_val = if next_c.is_digit(10) {
                                        next_c as i64 - '0' as i64
                                    } else {
                                        (next_c.to_ascii_uppercase() as i64 - 'A' as i64) + 10
                                    };
                                    
                                    value = value * 16 + digit_val;
                                    self.chars.next();
                                    self.pos += 1;
                                } else {
                                    break;
                                }
                            }
                        }
                        // octal number
                        else if value == 0 {
                            while let Some(&next_c) = self.chars.peek() {
                                if next_c >= '0' && next_c <= '7' {
                                    value = value * 8 + (next_c as i64 - '0' as i64);
                                    self.chars.next();
                                    self.pos += 1;
                                } else {
                                    break;
                                }
                            }
                        }
                        // decimal number
                        else {
                            while let Some(&next_c) = self.chars.peek() {
                                if next_c.is_digit(10) {
                                    value = value * 10 + (next_c as i64 - '0' as i64);
                                    self.chars.next();
                                    self.pos += 1;
                                } else {
                                    break;
                                }
                            }
                        }
                        
                        self.current_value = value;
                        self.current_token = Token::Num(value);
                    },
                    
                    // strings and chars
                    '\'' | '"' => {
                        let string_delim = c;
                        let start_pos = self.string_buffer.len();
                        
                        while let Some(&next_c) = self.chars.peek() {
                            self.chars.next(); // consume char
                            self.pos += 1;
                            
                            if next_c == string_delim {
                                break; // end of literal
                            }
                            
                            // handle escapes
                            if next_c == '\\' {
                                if let Some(escaped_char) = self.chars.next() {
                                    self.pos += 1;
                                    match escaped_char {
                                        'n' => self.string_buffer.push(b'\n'),
                                        't' => self.string_buffer.push(b'\t'),
                                        'r' => self.string_buffer.push(b'\r'),
                                        '\\' => self.string_buffer.push(b'\\'),
                                        '"' => self.string_buffer.push(b'\"'),
                                        '\'' => self.string_buffer.push(b'\''),
                                        '0' => self.string_buffer.push(b'\0'),
                                        // Handle hex escapes (e.g., \\x41 for 'A') - Simplified for now
                                        // Handle octal escapes (e.g., \\101 for 'A') - Simplified for now
                                        _ => self.string_buffer.push(escaped_char as u8), // Unknown escape, just use the char
                                    }
                                } else {
                                    // EOF during escape sequence
                                    break;
                                }
                            } else {
                                self.string_buffer.push(next_c as u8);
                            }
                        }
                        
                        if string_delim == '"' {
                            // Null-terminate the string in the buffer
                            self.string_buffer.push(0); 
                            
                            self.current_value = start_pos as i64; // Value is start index
                            self.current_token = Token::Str(start_pos);
                            if self.debug {
                                println!("DEBUG LEXER: Found string literal at index {}", start_pos);
                            }
                        } else {
                            // char literal - value is the ASCII code
                            if start_pos < self.string_buffer.len() {
                                // Handle actual escaped char value from buffer
                                let char_byte = self.string_buffer[start_pos];
                                let char_val = match char_byte {
                                    // Use actual byte values
                                    b'\n' => b'\n' as i64,
                                    b'\t' => b'\t' as i64,
                                    b'\r' => b'\r' as i64,
                                    b'\\' => b'\\' as i64,
                                    b'"' => b'"' as i64,
                                    b'\'' => b'\'' as i64,
                                    b'\0' => 0,
                                    _ => char_byte as i64,
                                };
                                self.current_value = char_val;
                                // Remove the char data from buffer, not needed after value extraction
                                self.string_buffer.truncate(start_pos); 
                            } else {
                                // Empty char literal ''? -> value 0
                                self.current_value = 0;
                            }
                            self.current_token = Token::Num(self.current_value);
                            if self.debug {
                                println!("DEBUG LEXER: Found char literal with value {}", self.current_value);
                            }
                        }
                    },
                    
                    // assignment and compound assignments
                    '=' => {
                        // Check for equality (==) or just assignment (=)
                        if let Some(&next_c) = self.chars.peek() {
                            if next_c == '=' {
                                self.chars.next();
                                self.pos += 1;
                                self.current_token = Token::Eq;
                            } else {
                                self.current_token = Token::Assign;
                            }
                        } else {
                            self.current_token = Token::Assign;
                        }
                    },
                    
                    // add and +=
                    '+' => {
                        if let Some(&next_c) = self.chars.peek() {
                            if next_c == '=' {
                                self.chars.next();
                                self.pos += 1;
                                self.current_token = Token::AddAssign;
                            } else if next_c == '+' {
                                self.chars.next();
                                self.pos += 1;
                                self.current_token = Token::Inc;
                            } else {
                                self.current_token = Token::Add;
                            }
                        } else {
                            self.current_token = Token::Add;
                        }
                    },
                    
                    // subtract and -=
                    '-' => {
                        if let Some(&next_c) = self.chars.peek() {
                            if next_c == '=' {
                                self.chars.next();
                                self.pos += 1;
                                self.current_token = Token::SubAssign;
                            } else if next_c == '-' {
                                self.chars.next();
                                self.pos += 1;
                                self.current_token = Token::Dec;
                            } else {
                                self.current_token = Token::Sub;
                            }
                        } else {
                            self.current_token = Token::Sub;
                        }
                    },
                    
                    // multiply and *=
                    '*' => {
                        if let Some(&next_c) = self.chars.peek() {
                            if next_c == '=' {
                                self.chars.next();
                                self.pos += 1;
                                self.current_token = Token::MulAssign;
                            } else {
                                self.current_token = Token::Mul;
                            }
                        } else {
                            self.current_token = Token::Mul;
                        }
                    },
                    
                    // divide and /=
                    '/' => {
                        if let Some(&next_c) = self.chars.peek() {
                            if next_c == '/' {
                                // Line comment
                                self.chars.next();
                                self.pos += 1;
                                
                                // Consume characters until end of line
                                while let Some(&next_c) = self.chars.peek() {
                                    if next_c == '\n' {
                                        break;
                                    }
                                    self.chars.next();
                                    self.pos += 1;
                                }
                                
                                // Skip to next token
                                return self.next();
                            } else if next_c == '*' {
                                // Block comment
                                self.chars.next();
                                self.pos += 1;
                                
                                // Variables to track comment nesting
                                let mut depth = 1;
                                
                                // Handle nested comments
                                while depth > 0 {
                                    if let Some(c) = self.chars.next() {
                                        self.pos += 1;
                                        
                                        if c == '\n' {
                                            self.line += 1;
                                            self.lp = self.pos;
                                        } else if c == '/' && self.chars.peek() == Some(&'*') {
                                            // Found nested comment start
                                            self.chars.next();
                                            self.pos += 1;
                                            depth += 1;
                                        } else if c == '*' && self.chars.peek() == Some(&'/') {
                                            // Found comment end
                                            self.chars.next();
                                            self.pos += 1;
                                            depth -= 1;
                                        }
                                    } else {
                                        // Reached EOF inside comment
                                        self.current_token = Token::Eof;
                                        return self.current_token;
                                    }
                                }
                                
                                // Skip to next token
                                return self.next();
                            } else if next_c == '=' {
                                self.chars.next();
                                self.pos += 1;
                                self.current_token = Token::DivAssign;
                            } else {
                                self.current_token = Token::Div;
                            }
                        } else {
                            self.current_token = Token::Div;
                        }
                    },
                    
                    // modulo and %=
                    '%' => {
                        if let Some(&next_c) = self.chars.peek() {
                            if next_c == '=' {
                                self.chars.next();
                                self.pos += 1;
                                self.current_token = Token::ModAssign;
                            } else {
                                self.current_token = Token::Mod;
                            }
                        } else {
                            self.current_token = Token::Mod;
                        }
                    },
                    
                    // operators and punctuation
                    '!' => {
                        if let Some(&'=') = self.chars.peek() {
                            self.chars.next();
                            self.pos += 1;
                            self.current_token = Token::Ne;
                        } else {
                            self.current_token = Token::Not;
                        }
                    },
                    '<' => {
                        if let Some(&next_c) = self.chars.peek() {
                            if next_c == '=' {
                                self.chars.next();
                                self.pos += 1;
                                self.current_token = Token::Le;
                            } else if next_c == '<' {
                                self.chars.next();
                                self.pos += 1;
                                
                                // Check for <<=
                                if let Some(&next_next_c) = self.chars.peek() {
                                    if next_next_c == '=' {
                                        self.chars.next();
                                        self.pos += 1;
                                        self.current_token = Token::ShlAssign;
                                    } else {
                                        self.current_token = Token::Shl;
                                    }
                                } else {
                                    self.current_token = Token::Shl;
                                }
                            } else {
                                self.current_token = Token::Lt;
                            }
                        } else {
                            self.current_token = Token::Lt;
                        }
                    },
                    '>' => {
                        if let Some(&next_c) = self.chars.peek() {
                            if next_c == '=' {
                                self.chars.next();
                                self.pos += 1;
                                self.current_token = Token::Ge;
                            } else if next_c == '>' {
                                self.chars.next();
                                self.pos += 1;
                                
                                // Check for >>=
                                if let Some(&next_next_c) = self.chars.peek() {
                                    if next_next_c == '=' {
                                        self.chars.next();
                                        self.pos += 1;
                                        self.current_token = Token::ShrAssign;
                                    } else {
                                        self.current_token = Token::Shr;
                                    }
                                } else {
                                    self.current_token = Token::Shr;
                                }
                            } else {
                                self.current_token = Token::Gt;
                            }
                        } else {
                            self.current_token = Token::Gt;
                        }
                    },
                    '&' => {
                        if let Some(&next_c) = self.chars.peek() {
                            if next_c == '&' {
                                self.chars.next();
                                self.pos += 1;
                                self.current_token = Token::Lan;
                            } else if next_c == '=' {
                                self.chars.next();
                                self.pos += 1;
                                self.current_token = Token::AndAssign;
                            } else {
                                self.current_token = Token::And;
                            }
                        } else {
                            self.current_token = Token::And;
                        }
                    },
                    '|' => {
                        if let Some(&next_c) = self.chars.peek() {
                            if next_c == '|' {
                                self.chars.next();
                                self.pos += 1;
                                self.current_token = Token::Lor;
                            } else if next_c == '=' {
                                self.chars.next();
                                self.pos += 1;
                                self.current_token = Token::OrAssign;
                            } else {
                                self.current_token = Token::Or;
                            }
                        } else {
                            self.current_token = Token::Or;
                        }
                    },
                    '^' => {
                        if let Some(&next_c) = self.chars.peek() {
                            if next_c == '=' {
                                self.chars.next();
                                self.pos += 1;
                                self.current_token = Token::XorAssign;
                            } else {
                                self.current_token = Token::Xor;
                            }
                        } else {
                            self.current_token = Token::Xor;
                        }
                    },
                    '[' => {
                        if self.debug {
                            println!("DEBUG LEXER: Found left bracket token at line {}", self.line);
                        }
                        self.current_token = Token::LeftBracket;
                    },
                    '?' => self.current_token = Token::Cond,
                    '~' => self.current_token = Token::Tilde,
                    ';' => self.current_token = Token::Semicolon,
                    '{' => self.current_token = Token::LeftBrace,
                    '}' => self.current_token = Token::RightBrace,
                    '(' => self.current_token = Token::LeftParen,
                    ')' => self.current_token = Token::RightParen,
                    ']' => {
                        if self.debug {
                            println!("DEBUG LEXER: Found right bracket token at line {}", self.line);
                        }
                        self.current_token = Token::RightBracket;
                    },
                    ',' => self.current_token = Token::Comma,
                    ':' => self.current_token = Token::Colon,
                    
                    // unknown char
                    _ => {
                        // skip it
                        self.current_token = self.next();
                    }
                }
                
                self.current_token
            },
            None => Token::Eof
        }
    }
    
    /// skips spaces and comments
    fn skip_whitespace(&mut self) {
        while let Some(&c) = self.chars.peek() {
            match c {
                ' ' | '\t' | '\r' => {
                    self.chars.next();
                    self.pos += 1;
                },
                '\n' => {
                    self.chars.next();
                    self.pos += 1;
                    self.line += 1;
                    self.lp = self.pos;
                },
                '#' => {
                    // skip preprocessor stuff
                    self.chars.next();
                    self.pos += 1;
                    while let Some(&c) = self.chars.peek() {
                        if c == '\n' {
                            break;
                        }
                        self.chars.next();
                        self.pos += 1;
                    }
                },
                _ => return, // not space
            }
        }
    }
    
    /// gets line position
    pub fn get_line_pos(&self) -> usize {
        self.lp
    }
    
    /// checks for text
    pub fn source_contains(&self, text: &str) -> bool {
        self.source.contains(text)
    }
    
    /// peeks next char
    pub fn peek_next(&self) -> Option<char> {
        // access source directly
        let current_pos = self.pos;
        if current_pos < self.source.len() {
            self.source[current_pos..].chars().next()
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_simple_tokens() {
        let mut lexer = Lexer::new("+ - * / % = == != < > <= >= && || !");
        
        assert_eq!(lexer.next(), Token::Add);
        assert_eq!(lexer.next(), Token::Sub);
        assert_eq!(lexer.next(), Token::Mul);
        assert_eq!(lexer.next(), Token::Div);
        assert_eq!(lexer.next(), Token::Mod);
        assert_eq!(lexer.next(), Token::Assign);
        assert_eq!(lexer.next(), Token::Eq);
        assert_eq!(lexer.next(), Token::Ne);
        assert_eq!(lexer.next(), Token::Lt);
        assert_eq!(lexer.next(), Token::Gt);
        assert_eq!(lexer.next(), Token::Le);
        assert_eq!(lexer.next(), Token::Ge);
        assert_eq!(lexer.next(), Token::Lan);
        assert_eq!(lexer.next(), Token::Lor);
        assert_eq!(lexer.next(), Token::Not);
        assert_eq!(lexer.next(), Token::Eof);
    }
    
    #[test]
    fn test_keywords() {
        let mut lexer = Lexer::new("int char if else while return sizeof enum void");
        
        assert_eq!(lexer.next(), Token::Int);
        assert_eq!(lexer.next(), Token::Char);
        assert_eq!(lexer.next(), Token::If);
        assert_eq!(lexer.next(), Token::Else);
        assert_eq!(lexer.next(), Token::While);
        assert_eq!(lexer.next(), Token::Return);
        assert_eq!(lexer.next(), Token::Sizeof);
        assert_eq!(lexer.next(), Token::Enum);
        assert_eq!(lexer.next(), Token::Void);
        assert_eq!(lexer.next(), Token::Eof);
    }
    
    #[test]
    fn test_numbers() {
        let mut lexer = Lexer::new("123 0x1A 042");
        
        assert_eq!(lexer.next(), Token::Num(123));
        assert_eq!(lexer.value(), 123);
        
        assert_eq!(lexer.next(), Token::Num(26)); // 0x1A = 26
        assert_eq!(lexer.value(), 26);
        
        assert_eq!(lexer.next(), Token::Num(34)); // octal 042 = 34
        assert_eq!(lexer.value(), 34);
        
        assert_eq!(lexer.next(), Token::Eof);
    }
    
    #[test]
    fn test_identifiers() {
        let mut lexer = Lexer::new("a abc x123 _var");
        
        // check if identifiers
        match lexer.next() {
            Token::Id(_) => assert!(true),
            _ => assert!(false, "Expected identifier"),
        }
        
        match lexer.next() {
            Token::Id(_) => assert!(true),
            _ => assert!(false, "Expected identifier"),
        }
        
        match lexer.next() {
            Token::Id(_) => assert!(true),
            _ => assert!(false, "Expected identifier"),
        }
        
        match lexer.next() {
            Token::Id(_) => assert!(true),
            _ => assert!(false, "Expected identifier"),
        }
        
        assert_eq!(lexer.next(), Token::Eof);
    }
    
    #[test]
    fn test_strings() {
        let mut lexer = Lexer::new("\"hello\" 'c'");
        
        // Test string literal
        assert_eq!(lexer.next(), Token::Str(0));
        let str_content = lexer.string_buffer();
        assert_eq!(str_content, b"hello\0");
        
        // Test char literal
        assert_eq!(lexer.next(), Token::Num('c' as i64));
        assert_eq!(lexer.value(), 'c' as i64);
        
        assert_eq!(lexer.next(), Token::Eof);
    }
    
    #[test]
    fn test_comments() {
        let mut lexer = Lexer::new("a // this is a comment\nb");
        
        match lexer.next() {
            Token::Id(_) => assert!(true),
            _ => assert!(false, "Expected identifier 'a'"),
        }
        
        match lexer.next() {
            Token::Id(_) => assert!(true),
            _ => assert!(false, "Expected identifier 'b'"),
        }
        
        assert_eq!(lexer.next(), Token::Eof);
    }
} 