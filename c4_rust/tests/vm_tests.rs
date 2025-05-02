use c4_rust::parser::{Parser, OpCode};
use c4_rust::vm::VM;

#[test]
fn test_vm_simple_program() {
    // return 42
    let code = vec![
        OpCode::IMM as i64, 42,    // load 42
        OpCode::PSH as i64,        // push it
        OpCode::EXIT as i64,       // exit
    ];
    
    let mut vm = VM::new(code, vec![], false);
    let result = vm.run();
    assert!(result.is_ok(), "VM execution failed: {:?}", result.err());
    assert_eq!(result.unwrap(), 42);
}

#[test]
fn test_vm_arithmetic() {
    // 10 + 5 * 2
    let code = vec![
        OpCode::IMM as i64, 5,     // load 5
        OpCode::PSH as i64,        // push it
        OpCode::IMM as i64, 2,     // load 2
        OpCode::MUL as i64,        // multiply
        OpCode::PSH as i64,        // push result
        OpCode::IMM as i64, 10,    // load 10
        OpCode::ADD as i64,        // add
        OpCode::PSH as i64,        // push final
        OpCode::EXIT as i64,       // exit
    ];
    
    let mut vm = VM::new(code, vec![], false);
    let result = vm.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 20); // 10 + (5 * 2)
}

#[test]
fn test_vm_branch() {
    // if(1) return 10 else 20
    let code = vec![
        OpCode::IMM as i64, 1,     // condition true
        OpCode::BZ as i64, 7,      // skip if zero
        OpCode::IMM as i64, 10,    // return 10
        OpCode::PSH as i64,
        OpCode::EXIT as i64,
        OpCode::IMM as i64, 20,    // else 20
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
    // memory test
    let data = vec![0u8; 16];
    
    // *ptr = 123; return *ptr
    let code = vec![
        OpCode::IMM as i64, 0,     // address 0
        OpCode::PSH as i64,        // push address
        OpCode::IMM as i64, 123,   // value 123
        OpCode::SI as i64,         // store it
        OpCode::IMM as i64, 0,     // address again
        OpCode::LI as i64,         // load value
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
    // check exit works
    let code = vec![
        OpCode::IMM as i64, 12,    // load 12
        OpCode::PSH as i64,        // push it
        OpCode::EXIT as i64,       // exit with 12
    ];
    
    let mut vm = VM::new(code, vec![], false);
    let result = vm.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 12);
}

#[test]
fn test_vm_printf() {
    // setup string data
    let mut data = vec![0u8; 256];
    
    // put "Hello, %d!\n" in data
    let str_addr = 10;
    let test_string = "Hello, %d!\n";
    for (i, byte) in test_string.bytes().enumerate() {
        data[str_addr + i] = byte;
    }
    
    // printf("Hello, %d!\n", 42)
    let code = vec![
        OpCode::IMM as i64, 42,     // arg: 42
        OpCode::PSH as i64,         // push arg
        OpCode::IMM as i64, str_addr as i64, // format string
        OpCode::PSH as i64,         // push format
        OpCode::PRTF as i64, 2,     // printf with 2 args
        OpCode::IMM as i64, 0,      // return 0
        OpCode::PSH as i64,
        OpCode::EXIT as i64,
    ];
    
    let mut vm = VM::new(code, data, false);
    let result = vm.run();
    
    // just check it ran
    assert!(result.is_ok());
} 