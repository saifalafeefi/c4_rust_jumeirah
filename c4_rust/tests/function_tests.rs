use c4_rust::parser::{Parser, Type, SymbolClass, OpCode};

#[test]
fn test_function_parameter_parsing() {
    let source = "int add(int a, int b) { return a + b; }\n\nint main() { return add(1, 2); }";
    let mut parser = Parser::new(source, false);
    parser.init().unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "Parsing failed: {:?}", result.err());
    
    // find function
    let symbols = parser.get_symbols();
    let add_fn = symbols.iter().find(|s| s.name == "add");
    assert!(add_fn.is_some(), "Function 'add' not found in symbol table");
    
    // check function type
    let add_fn = add_fn.unwrap();
    assert_eq!(add_fn.class, SymbolClass::Fun);
    assert!(matches!(add_fn.typ, Type::Int));
    
    // check code
    let (code, _) = result.unwrap();
    
    // look for ENT
    let ent_pos = code.iter().position(|&x| x == OpCode::ENT as i64);
    assert!(ent_pos.is_some(), "ENT instruction not found in code");
    
    // look for LEV
    let lev_pos = code.iter().position(|&x| x == OpCode::LEV as i64);
    assert!(lev_pos.is_some(), "LEV instruction not found in code");
}

#[test]
fn test_function_local_variables() {
    let source = "int calc(int x) { int y, z; y = x * 2; z = y + 1; return z; }\n\nint main() { return calc(5); }";
    let mut parser = Parser::new(source, false);
    parser.init().unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "Parsing failed: {:?}", result.err());
    
    // find function
    let symbols = parser.get_symbols();
    let calc_fn = symbols.iter().find(|s| s.name == "calc");
    assert!(calc_fn.is_some(), "Function 'calc' not found in symbol table");
    
    // check code
    let (code, _) = result.unwrap();
    
    // look for ENT
    let ent_pos = code.iter().position(|&x| x == OpCode::ENT as i64);
    assert!(ent_pos.is_some(), "ENT instruction not found in code");
    
    // look for LEV
    let lev_pos = code.iter().position(|&x| x == OpCode::LEV as i64);
    assert!(lev_pos.is_some(), "LEV instruction not found in code");
}

#[test]
fn test_multiple_parameter_types() {
    let source = "int process(char c, int *ptr, int val) { return val; }\n\nint main() { return 0; }";
    let mut parser = Parser::new(source, false);
    parser.init().unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "Parsing failed: {:?}", result.err());
    
    // success means it works
    assert!(result.is_ok());
}

#[test]
fn test_nested_function_blocks() {
    let source = "int complex(int x) { 
                   if (x != 0) { 
                     int y; 
                     y = 10;
                     while (y != 0) { 
                       int z; 
                       z = y * 2;
                       y = y - 1; 
                     } 
                     return x + y; 
                   } else { 
                     return 0; 
                   } 
                 }\n\nint main() { return complex(5); }";
    let mut parser = Parser::new(source, false);
    parser.init().unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "Parsing failed with nested blocks: {:?}", result.err());
} 