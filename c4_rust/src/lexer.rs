/// lexer module for tokenizing C source code
/// converts raw source into tokens for the parser

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
    
    // operators (in precedence order)
    Assign,
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
    Brak,
    
    // special characters
    Semicolon,
    LeftBrace,
    RightBrace,
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    Comma,
    Colon,
    Tilde,
    
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
    lp: usize, // line position for source printing
}

impl<'a> Lexer<'a> {
    /// create a new lexer for the given source code
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
        }
    }
    
    /// retrieve the current token
    pub fn token(&self) -> Token {
        self.current_token
    }
    
    /// retrieve the current numeric value (for Num tokens)
    pub fn value(&self) -> i64 {
        self.current_value
    }
    
    /// retrieve the current line number
    pub fn line(&self) -> usize {
        self.line
    }
    
    /// retrieve the string buffer
    pub fn string_buffer(&self) -> &[u8] {
        &self.string_buffer
    }
    
    /// advance to the next token
    pub fn next(&mut self) -> Token {
        // skip whitespace and comments
        self.skip_whitespace();
        
        // check for EOF
        if let None = self.chars.peek() {
            self.current_token = Token::Eof;
            return self.current_token;
        }
        
        // process the next character
        match self.chars.next() {
            Some(c) => {
                self.pos += 1;
                match c {
                    // Identifiers and keywords
                    'a'..='z' | 'A'..='Z' | '_' => {
                        let mut hash = c as u64;
                        let start_pos = self.pos - 1;
                        
                        // read the entire identifier
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
                        
                        // Check if it's a keyword
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
                            // If not a keyword, it's an identifier
                            _ => Token::Id(hash as usize),
                        };
                    },
                    
                    // Numbers
                    '0'..='9' => {
                        let mut value = (c as i64) - ('0' as i64);
                        
                        // Hex number
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
                        // Octal number
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
                        // Decimal number
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
                    
                    // String literals
                    '\'' | '"' => {
                        let string_delim = c;
                        let start_pos = self.string_buffer.len();
                        
                        while let Some(&next_c) = self.chars.peek() {
                            if next_c == string_delim {
                                self.chars.next(); // consume closing quote
                                self.pos += 1;
                                break;
                            }
                            
                            // Handle escape sequences
                            if next_c == '\\' {
                                self.chars.next(); // consume backslash
                                self.pos += 1;
                                
                                match self.chars.next() {
                                    Some('n') => {
                                        self.string_buffer.push(b'\n');
                                        self.pos += 1;
                                    },
                                    Some(esc_char) => {
                                        self.string_buffer.push(esc_char as u8);
                                        self.pos += 1;
                                    },
                                    None => break,
                                }
                            } else {
                                self.string_buffer.push(next_c as u8);
                                self.chars.next();
                                self.pos += 1;
                            }
                        }
                        
                        // Ensure null termination
                        self.string_buffer.push(0);
                        
                        if string_delim == '"' {
                            // For string literals, return the index to the start of the string
                            self.current_value = start_pos as i64;
                            self.current_token = Token::Str(start_pos);
                        } else {
                            // For character literals, return the character value
                            if start_pos < self.string_buffer.len() {
                                self.current_value = self.string_buffer[start_pos] as i64;
                                self.current_token = Token::Num(self.current_value);
                            } else {
                                self.current_value = 0;
                                self.current_token = Token::Num(0);
                            }
                        }
                    },
                    
                    // Operators and punctuation
                    '=' => {
                        if let Some(&'=') = self.chars.peek() {
                            self.chars.next();
                            self.pos += 1;
                            self.current_token = Token::Eq;
                        } else {
                            self.current_token = Token::Assign;
                        }
                    },
                    '+' => {
                        if let Some(&'+') = self.chars.peek() {
                            self.chars.next();
                            self.pos += 1;
                            self.current_token = Token::Inc;
                        } else {
                            self.current_token = Token::Add;
                        }
                    },
                    '-' => {
                        if let Some(&'-') = self.chars.peek() {
                            self.chars.next();
                            self.pos += 1;
                            self.current_token = Token::Dec;
                        } else {
                            self.current_token = Token::Sub;
                        }
                    },
                    '!' => {
                        if let Some(&'=') = self.chars.peek() {
                            self.chars.next();
                            self.pos += 1;
                            self.current_token = Token::Ne;
                        } else {
                            self.current_token = Token::Tilde; // Using Tilde for '!' in this implementation
                        }
                    },
                    '<' => {
                        if let Some(&'=') = self.chars.peek() {
                            self.chars.next();
                            self.pos += 1;
                            self.current_token = Token::Le;
                        } else if let Some(&'<') = self.chars.peek() {
                            self.chars.next();
                            self.pos += 1;
                            self.current_token = Token::Shl;
                        } else {
                            self.current_token = Token::Lt;
                        }
                    },
                    '>' => {
                        if let Some(&'=') = self.chars.peek() {
                            self.chars.next();
                            self.pos += 1;
                            self.current_token = Token::Ge;
                        } else if let Some(&'>') = self.chars.peek() {
                            self.chars.next();
                            self.pos += 1;
                            self.current_token = Token::Shr;
                        } else {
                            self.current_token = Token::Gt;
                        }
                    },
                    '|' => {
                        if let Some(&'|') = self.chars.peek() {
                            self.chars.next();
                            self.pos += 1;
                            self.current_token = Token::Lor;
                        } else {
                            self.current_token = Token::Or;
                        }
                    },
                    '&' => {
                        if let Some(&'&') = self.chars.peek() {
                            self.chars.next();
                            self.pos += 1;
                            self.current_token = Token::Lan;
                        } else {
                            self.current_token = Token::And;
                        }
                    },
                    '^' => self.current_token = Token::Xor,
                    '%' => self.current_token = Token::Mod,
                    '*' => self.current_token = Token::Mul,
                    '[' => self.current_token = Token::Brak,
                    '?' => self.current_token = Token::Cond,
                    '~' => self.current_token = Token::Tilde,
                    ';' => self.current_token = Token::Semicolon,
                    '{' => self.current_token = Token::LeftBrace,
                    '}' => self.current_token = Token::RightBrace,
                    '(' => self.current_token = Token::LeftParen,
                    ')' => self.current_token = Token::RightParen,
                    ']' => self.current_token = Token::RightBracket,
                    ',' => self.current_token = Token::Comma,
                    ':' => self.current_token = Token::Colon,
                    
                    // Division or comments
                    '/' => {
                        if let Some(&'/') = self.chars.peek() {
                            // This is a comment, skip to end of line
                            self.chars.next(); // consume second '/'
                            self.pos += 1;
                            
                            while let Some(&next_c) = self.chars.peek() {
                                if next_c == '\n' {
                                    break;
                                }
                                self.chars.next();
                                self.pos += 1;
                            }
                            
                            // After skipping comment, recursively get next token
                            return self.next();
                        } else {
                            self.current_token = Token::Div;
                        }
                    },
                    
                    // Unknown character
                    _ => {
                        // Just skip unknown characters
                        self.current_token = self.next();
                    }
                }
                
                self.current_token
            },
            None => Token::Eof
        }
    }
    
    /// skip whitespace and comments
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
                    // skip preprocessor directives
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
                _ => return, // not whitespace
            }
        }
    }
    
    /// get source code line position for error reporting
    pub fn get_line_pos(&self) -> usize {
        self.lp
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
        assert_eq!(lexer.next(), Token::Tilde);
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
        
        // Just check if they're identifiers, exact hash values will vary
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
        
        // String
        assert_eq!(lexer.next(), Token::Str(0));
        let str_content = lexer.string_buffer();
        assert_eq!(str_content, b"hello");
        
        // Character
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