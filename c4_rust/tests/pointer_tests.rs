use c4_rust::parser::{Parser, Type, OpCode};

#[test]
fn test_pointer_dereferencing() {
    let source = "int main() { int x; int *p; x = 10; p = &x; return *p; }";
    let mut parser = Parser::new(source, false);
    parser.init().unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "Parsing failed: {:?}", result.err());
    
    // code should have LI
    let (code, _) = result.unwrap();
    let li_ops_count = code.iter().filter(|&&x| x == OpCode::LI as i64).count();
    assert!(li_ops_count > 0, "No LI operation found after dereferencing");
}

#[test]
fn test_address_of_operator() {
    let source = "int main() { int x; int *p; x = 10; p = &x; return 0; }";
    let mut parser = Parser::new(source, false);
    parser.init().unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "Parsing failed: {:?}", result.err());
    
    // check for LEA
    let (code, _) = result.unwrap();
    let lea_ops_count = code.iter().filter(|&&x| x == OpCode::LEA as i64).count();
    assert!(lea_ops_count > 0, "No LEA operation found for address-of operator");
}

#[test]
fn test_pointer_arithmetic() {
    let source = "int main() { int x; int *p; p = &x; p = p + 1; return 0; }";
    let mut parser = Parser::new(source, false);
    parser.init().unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "Parsing failed: {:?}", result.err());
    
    // need PSH and IMM
    let (code, _) = result.unwrap();
    let psh_count = code.iter().filter(|&&x| x == OpCode::PSH as i64).count();
    let imm_count = code.iter().filter(|&&x| x == OpCode::IMM as i64).count();
    assert!(psh_count > 0 && imm_count > 0, "Expected PSH and IMM instructions for pointer arithmetic");
}

#[test]
fn test_double_pointer() {
    let source = "int main() { int x; int *p; int **pp; x = 10; p = &x; pp = &p; return **pp; }";
    let mut parser = Parser::new(source, false);
    parser.init().unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "Parsing failed with double pointer: {:?}", result.err());
}

#[test]
fn test_multiple_pointer_declarations() {
    // test char *p, *q
    let source = "int main() { char *p, *q; p = &q; return 0; }";
    let mut parser = Parser::new(source, false);
    parser.init().unwrap();
    let result = parser.parse();
    
    assert!(result.is_ok(), "Parsing failed with multiple pointer declarations: {:?}", result.err());
    
    // print for debugging
    let symbols = parser.get_symbols();
    println!("Symbol table contents:");
    for symbol in symbols {
        println!("Symbol: {}, Class: {:?}, Type: {:?}", symbol.name, symbol.class, symbol.typ);
    }
    
    // just check it works
    assert!(result.is_ok());
}

#[test]
fn test_continued_declaration_with_address_of() {
    // mimics c4.c style
    let source = "char *p, *lp, *data; int main() { p = &lp; return 0; }";
    let mut parser = Parser::new(source, false);
    parser.init().unwrap();
    
    // parse whole program
    let result = parser.parse();
    assert!(result.is_ok(), "Parsing failed with continued declarations: {:?}", result.err());
    
    // show symbols
    println!("Full symbol table:");
    let symbols = parser.get_symbols();
    for symbol in symbols {
        println!("Symbol: {}, Class: {:?}, Type: {:?}", symbol.name, symbol.class, symbol.typ);
    }
    
    // verify it worked
    assert!(result.is_ok());
} 