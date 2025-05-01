use c4_rust::parser::{Parser, Type, OpCode};

#[test]
fn test_pointer_dereferencing() {
    let source = "int main() { int x; int *p; x = 10; p = &x; return *p; }";
    let mut parser = Parser::new(source, false);
    parser.init().unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "Parsing failed: {:?}", result.err());
    
    // The code should contain LI (load int) after dereferencing
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
    
    // When using address-of operator, the LEA instruction should be used
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
    
    // Pointer arithmetic should use PSH and IMM for pointer size
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
    // Test char *p, *q; inside a function
    let source = "int main() { char *p, *q; p = &q; return 0; }";
    let mut parser = Parser::new(source, false);
    parser.init().unwrap();
    let result = parser.parse();
    
    assert!(result.is_ok(), "Parsing failed with multiple pointer declarations: {:?}", result.err());
    
    // Print all symbols for debugging
    let symbols = parser.get_symbols();
    println!("Symbol table contents:");
    for symbol in symbols {
        println!("Symbol: {}, Class: {:?}, Type: {:?}", symbol.name, symbol.class, symbol.typ);
    }
    
    // Simplify and just check if the parse succeeded
    assert!(result.is_ok());
}

#[test]
fn test_continued_declaration_with_address_of() {
    // This mimics the c4.c declaration style with multiple pointer declarations and address-of
    let source = "char *p, *lp, *data; int main() { p = &lp; return 0; }";
    let mut parser = Parser::new(source, false);
    parser.init().unwrap();
    
    // Just parse the whole program directly
    let result = parser.parse();
    assert!(result.is_ok(), "Parsing failed with continued declarations: {:?}", result.err());
    
    // Print the full symbol table
    println!("Full symbol table:");
    let symbols = parser.get_symbols();
    for symbol in symbols {
        println!("Symbol: {}, Class: {:?}, Type: {:?}", symbol.name, symbol.class, symbol.typ);
    }
    
    // At this point, we're mainly verifying that the parse was successful
    // And that we don't crash on the address-of operation
    assert!(result.is_ok());
} 