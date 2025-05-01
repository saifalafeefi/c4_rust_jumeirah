use c4_rust::vm::VM;

#[test]
fn test_vm_load_store_functions() {
    // Create a VM with enough data segment space
    let code = vec![]; // Empty code segment
    let mut data = vec![0u8; 16];
    
    // Set up some test data in the data segment
    // Store the integer value 0x12345678 at offset 0
    data[0] = 0x78;
    data[1] = 0x56;
    data[2] = 0x34;
    data[3] = 0x12;
    data[4] = 0x00;
    data[5] = 0x00;
    data[6] = 0x00;
    data[7] = 0x00;
    
    // Store the character value 'A' at offset 8
    data[8] = 65; // ASCII 'A'
    
    let mut vm = VM::new(code, data, false);
    
    // Test load_int
    let value = vm.load_int(0);
    assert_eq!(value, 0x12345678, "load_int didn't return the expected value");
    
    // Test load_char
    let char_val = vm.load_char(8);
    assert_eq!(char_val, 65, "load_char didn't return the expected value");
    
    // Test store_int
    vm.store_int(4, 0x87654321);
    assert_eq!(vm.load_int(4), 0x87654321, "store_int didn't set the expected value");
    
    // Test store_char
    vm.store_char(9, 66); // ASCII 'B'
    assert_eq!(vm.load_char(9), 66, "store_char didn't set the expected value");
} 