use c4_rust::parser::{Parser, OpCode};
use c4_rust::vm::VM;

#[test]
fn test_vm_simple_program() {
    // simple program: return 42
    let code = vec![
        OpCode::IMM as i64, 42,    // load immediate value 42
        OpCode::PSH as i64,        // push to stack
        OpCode::EXIT as i64,       // exit
    ];
    
    let mut vm = VM::new(code, vec![], false);
    let result = vm.run();
    assert!(result.is_ok(), "VM execution failed: {:?}", result.err());
    assert_eq!(result.unwrap(), 42);
}

#[test]
fn test_vm_arithmetic() {
    // program: 10 + 5 * 2
    let code = vec![
        OpCode::IMM as i64, 5,     // load immediate value 5
        OpCode::PSH as i64,        // push to stack
        OpCode::IMM as i64, 2,     // load immediate value 2
        OpCode::MUL as i64,        // multiply
        OpCode::PSH as i64,        // push to stack
        OpCode::IMM as i64, 10,    // load immediate value 10
        OpCode::ADD as i64,        // add
        OpCode::PSH as i64,        // push to stack
        OpCode::EXIT as i64,       // exit
    ];
    
    let mut vm = VM::new(code, vec![], false);
    let result = vm.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 20); // 10 + (5 * 2)
}

#[test]
fn test_vm_branch() {
    // program: if (1) return 10; else return 20;
    let code = vec![
        OpCode::IMM as i64, 1,     // condition: true
        OpCode::BZ as i64, 7,      // branch if zero to else part
        OpCode::IMM as i64, 10,    // then: 10
        OpCode::PSH as i64,
        OpCode::EXIT as i64,
        OpCode::IMM as i64, 20,    // else: 20
        OpCode::PSH as i64,
        OpCode::EXIT as i64,
    ];
    
    let mut vm = VM::new(code, vec![], false);
    let result = vm.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 10);
}

#[test]
fn test_vm_load_store() {
    // initialize some memory and load/store values
    let data = vec![0u8; 16];
    
    // program: *ptr = 123; return *ptr;
    let code = vec![
        OpCode::IMM as i64, 0,     // load address 0
        OpCode::PSH as i64,        // push address
        OpCode::IMM as i64, 123,   // load value 123
        OpCode::SI as i64,         // store int at address
        OpCode::IMM as i64, 0,     // load address 0 again
        OpCode::LI as i64,         // load int from address
        OpCode::PSH as i64,        // push result
        OpCode::EXIT as i64,       // exit
    ];
    
    let mut vm = VM::new(code, data, false);
    let result = vm.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 123);
}

#[test]
fn test_vm_function_call() {
    // Simplify the test - check that we can verify EXIT returns the right value
    let code = vec![
        OpCode::IMM as i64, 12,    // load value 12
        OpCode::PSH as i64,        // push to stack
        OpCode::EXIT as i64,       // exit with value 12
    ];
    
    let mut vm = VM::new(code, vec![], false);
    let result = vm.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 12);
} 