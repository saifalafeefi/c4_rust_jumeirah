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
    If,
    Int,
    Return,
    Sizeof,
    While,
    
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
    string_buffer: Vec<char>,
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
    
    /// advance to the next token
    pub fn next(&mut self) -> Token {
        // skip whitespace
        self.skip_whitespace();
        
        // check for EOF
        if let None = self.chars.peek() {
            self.current_token = Token::Eof;
            return Token::Eof;
        }
        
        // process the next character
        match self.chars.next() {
            Some(c) => {
                self.pos += 1;
                match c {
                    // TODO: implement full tokenization logic here
                    // for now, this is a placeholder
                    _ => Token::Eof
                }
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
                '/' => {
                    // check for comments
                    self.chars.next();
                    self.pos += 1;
                    
                    if let Some(&'/') = self.chars.peek() {
                        // single line comment
                        self.chars.next();
                        self.pos += 1;
                        
                        while let Some(&c) = self.chars.peek() {
                            if c == '\n' {
                                break;
                            }
                            self.chars.next();
                            self.pos += 1;
                        }
                    } else {
                        // not a comment, put the '/' back
                        // in a real implementation, we'd handle this better
                        // but for simplicity, we'll assume it's a division operator
                        self.current_token = Token::Div;
                        return;
                    }
                },
                _ => return, // not whitespace, stop skipping
            }
        }
    }
    
    // TODO: implement more lexer methods as needed
} 