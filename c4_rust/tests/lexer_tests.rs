use c4_rust::lexer::{Lexer, Token};

#[test]
fn test_lexer_basic_tokens() {
    let source = "int main() { return 0; }";
    let mut lexer = Lexer::new(source);
    
    assert_eq!(lexer.next(), Token::Int);
    assert!(matches!(lexer.next(), Token::Id(_)));
    assert_eq!(lexer.next(), Token::LeftParen);
    assert_eq!(lexer.next(), Token::RightParen);
    assert_eq!(lexer.next(), Token::LeftBrace);
    assert_eq!(lexer.next(), Token::Return);
    assert_eq!(lexer.next(), Token::Num(0));
    assert_eq!(lexer.next(), Token::Semicolon);
    assert_eq!(lexer.next(), Token::RightBrace);
    assert_eq!(lexer.next(), Token::Eof);
}

#[test]
fn test_lexer_identifiers() {
    let source = "x y _var test123";
    let mut lexer = Lexer::new(source);
    
    // Each identifier should be tokenized correctly
    assert!(matches!(lexer.next(), Token::Id(_)));
    assert!(matches!(lexer.next(), Token::Id(_)));
    assert!(matches!(lexer.next(), Token::Id(_)));
    assert!(matches!(lexer.next(), Token::Id(_)));
    assert_eq!(lexer.next(), Token::Eof);
}

#[test]
fn test_lexer_numbers() {
    let source = "123 0 0x1A 052";
    let mut lexer = Lexer::new(source);
    
    assert_eq!(lexer.next(), Token::Num(123));
    assert_eq!(lexer.next(), Token::Num(0));
    assert_eq!(lexer.next(), Token::Num(26)); // 0x1A = 26
    assert_eq!(lexer.next(), Token::Num(42)); // 052 (octal) = 42
    assert_eq!(lexer.next(), Token::Eof);
}

#[test]
fn test_lexer_strings() {
    let source = "\"test\" \"Hello, World!\"";
    let mut lexer = Lexer::new(source);
    
    assert!(matches!(lexer.next(), Token::Str(_)));
    assert!(matches!(lexer.next(), Token::Str(_)));
    assert_eq!(lexer.next(), Token::Eof);
}

#[test]
fn test_lexer_operators() {
    let source = "+ - * / = == != < > <= >= & | ^ % << >>";
    let mut lexer = Lexer::new(source);
    
    assert_eq!(lexer.next(), Token::Add);
    assert_eq!(lexer.next(), Token::Sub);
    assert_eq!(lexer.next(), Token::Mul);
    assert_eq!(lexer.next(), Token::Div);
    assert_eq!(lexer.next(), Token::Assign);
    assert_eq!(lexer.next(), Token::Eq);
    assert_eq!(lexer.next(), Token::Ne);
    assert_eq!(lexer.next(), Token::Lt);
    assert_eq!(lexer.next(), Token::Gt);
    assert_eq!(lexer.next(), Token::Le);
    assert_eq!(lexer.next(), Token::Ge);
    assert_eq!(lexer.next(), Token::And);
    assert_eq!(lexer.next(), Token::Or);
    assert_eq!(lexer.next(), Token::Xor);
    assert_eq!(lexer.next(), Token::Mod);
    assert_eq!(lexer.next(), Token::Shl);
    assert_eq!(lexer.next(), Token::Shr);
    assert_eq!(lexer.next(), Token::Eof);
}

#[test]
fn test_lexer_comments() {
    let source = "int x; // Comment\nint y;";
    let mut lexer = Lexer::new(source);
    
    assert_eq!(lexer.next(), Token::Int);
    assert!(matches!(lexer.next(), Token::Id(_)));
    assert_eq!(lexer.next(), Token::Semicolon);
    
    // Comments should be skipped, next token should be Int
    assert_eq!(lexer.next(), Token::Int);
    assert!(matches!(lexer.next(), Token::Id(_)));
    assert_eq!(lexer.next(), Token::Semicolon);
    assert_eq!(lexer.next(), Token::Eof);
}

#[test]
fn test_lexer_whitespace() {
    let source = "if  \t  (x)\n{\n  return;\n}";
    let mut lexer = Lexer::new(source);
    
    assert_eq!(lexer.next(), Token::If);
    assert_eq!(lexer.next(), Token::LeftParen);
    assert!(matches!(lexer.next(), Token::Id(_)));
    assert_eq!(lexer.next(), Token::RightParen);
    assert_eq!(lexer.next(), Token::LeftBrace);
    assert_eq!(lexer.next(), Token::Return);
    assert_eq!(lexer.next(), Token::Semicolon);
    assert_eq!(lexer.next(), Token::RightBrace);
    assert_eq!(lexer.next(), Token::Eof);
}

#[test]
fn test_lexer_line_counting() {
    let source = "line1\nline2\nline3\n";
    let mut lexer = Lexer::new(source);
    
    assert!(matches!(lexer.next(), Token::Id(_))); // line1
    assert_eq!(lexer.line(), 1);
    
    assert!(matches!(lexer.next(), Token::Id(_))); // line2
    assert_eq!(lexer.line(), 2);
    
    assert!(matches!(lexer.next(), Token::Id(_))); // line3
    assert_eq!(lexer.line(), 3);
    
    assert_eq!(lexer.next(), Token::Eof);
    assert_eq!(lexer.line(), 4);
} 