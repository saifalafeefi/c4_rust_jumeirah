/// runs compiled code
/// executes parser output

use crate::parser::{OpCode, Parser};
use std::io::Write;

/// VM state
pub struct VM {
    code: Vec<i64>,       // code segment
    data: Vec<u8>,        // data segment
    pc: usize,            // program counter
    sp: usize,            // stack pointer
    bp: usize,            // base pointer
    ax: i64,              // accumulator
    stack: Vec<i64>,      // stack
    debug: bool,          // debug flag
    cycle: usize,         // instruction counter
}

impl VM {
    /// creates new VM
    pub fn new(code: Vec<i64>, data: Vec<u8>, debug: bool) -> Self {
        // smaller stack is enough
        let stack_size = 1024;
        
        // init stack to zeros
        let mut stack = Vec::with_capacity(stack_size);
        stack.resize(stack_size, 0);
        
        // leave room for use
        let sp = stack_size - 20;
        let bp = stack_size - 20;
        
        VM {
            code,
            data,
            pc: 0,
            sp,
            bp,
            ax: 0,
            stack,
            debug,
            cycle: 0,
        }
    }
    
    /// runs until exit
    pub fn run(&mut self) -> Result<i64, String> {
        // Set initial pc, sp, bp
        self.pc = 0;
        self.sp = self.stack.len() - 1;
        self.bp = self.sp;
        self.ax = 0;
        self.cycle = 0;
        
        // Main execution loop
        while self.pc < self.code.len() {
            // Get opcode
            let op = self.code[self.pc] as u8;
            self.pc += 1;
            self.cycle += 1;
            
            if self.debug {
                self.print_debug_info(op as usize);
            }
            
            // Show stack state (for debugging)
            if self.debug && op != OpCode::LEA as u8 && op != OpCode::LI as u8 && op != OpCode::SI as u8 {
                // Dump first few stack entries to see variable values
                println!("DEBUG VM: Stack state at instruction {}:", self.pc);
                let end = 5.min(self.stack.len());
                for i in 0..end {
                    println!("  stack[{}] = {}", i, self.stack[i]);
                }
            }
            
            // Always show current memory location before each operation
            if self.debug {
                println!("DEBUG VM: Instruction {} at pc={}: {}", op, self.pc-1, self.op_to_string(op as usize));
                
                // Show AX value
                println!("DEBUG VM: AX = {}", self.ax);
                
                // Show first few stack entries to track variable values
                println!("DEBUG VM: First few stack entries:");
                let vars_to_show = 10.min(self.stack.len());
                for i in 0..vars_to_show {
                    println!("  stack[{}] = {}", i, self.stack[i]);
                }
            }
            
            match op {
                // LEA: Load effective address
                op if op == OpCode::LEA as u8 => {
                    let offset = self.next_code() as usize;
                    
                    // Calculate effective address for a local variable
                    // For variables that are offsets from BP, we use BP - offset
                    self.ax = offset as i64;
                    
                    if self.debug {
                        println!("DEBUG VM: LEA - For local variable with offset {}, using address {}", 
                                offset, self.ax);
                    }
                },
                
                // IMM: Load immediate value
                op if op == OpCode::IMM as u8 => {
                    self.ax = self.next_code();
                    println!("DEBUG VM: IMM - Loaded immediate value {}", self.ax);
                },
                
                // JMP: Jump
                op if op == OpCode::JMP as u8 => {
                    self.pc = self.next_code() as usize;
                },
                
                // JSR: Jump to subroutine
                op if op == OpCode::JSR as u8 => {
                    // Push return address
                    if self.sp == 0 {
                        return Err("Stack overflow in JSR".to_string());
                    }
                    self.sp -= 1;
                    self.stack[self.sp] = self.pc as i64 + 1; // +1 to skip JSR argument
                    
                    // Jump to function entry
                    self.pc = self.next_code() as usize;
                },
                
                // BZ: Branch if zero
                op if op == OpCode::BZ as u8 => {
                    let target = self.next_code() as usize;
                    if self.ax == 0 {
                        self.pc = target;
                    }
                },
                
                // BNZ: Branch if not zero
                op if op == OpCode::BNZ as u8 => {
                    let target = self.next_code() as usize;
                    if self.ax != 0 {
                        self.pc = target;
                    }
                },
                
                // ENT: Enter function
                op if op == OpCode::ENT as u8 => {
                    let local_size = self.next_code() as usize;
                    if self.debug {
                        println!("  ENT: creating stack frame with {} local variables", local_size);
                    }
                    
                    // Push old base pointer
                    if self.sp < 2 {
                        return Err("Stack overflow in ENT".to_string());
                    }
                    self.sp -= 1;
                    self.stack[self.sp] = self.bp as i64;
                    
                    // Set new base pointer
                    self.bp = self.sp;
                    
                    // Reserve space for locals
                    if self.sp < local_size + 1 {
                        return Err(format!("Stack overflow for locals: need {} slots", local_size));
                    }
                    self.sp -= local_size;
                },
                
                // ADJ: Adjust stack
                op if op == OpCode::ADJ as u8 => {
                    let n = self.next_code() as usize;
                    if self.sp + n > self.stack.len() {
                        return Err(format!("Stack underflow in ADJ: sp={}, n={}", self.sp, n));
                    }
                    self.sp += n;
                },
                
                // LEV: Leave function
                op if op == OpCode::LEV as u8 => {
                    // Restore base pointer and stack pointer
                    self.sp = self.bp;
                    if self.sp >= self.stack.len() {
                        return Err(format!("Stack underflow in LEV: bp={}", self.bp));
                    }
                    self.bp = self.stack[self.sp] as usize;
                    self.sp += 1;
                    
                    // If there's no return address on the stack, we're returning from main
                    if self.sp >= self.stack.len() || (self.sp == self.stack.len() - 1) {
                        if self.debug {
                            println!("  LEV: returning from main function with value {}", self.ax);
                        }
                        return Ok(self.ax);
                    }
                    
                    // Otherwise, restore the program counter from the stack (return address)
                    if self.sp >= self.stack.len() {
                        return Err(format!("Stack underflow when restoring PC in LEV: sp={}", self.sp));
                    }
                    self.pc = self.stack[self.sp] as usize;
                    self.sp += 1;
                },
                
                // LI: Load int
                op if op == OpCode::LI as u8 => {
                    let addr = self.ax as usize;
                    
                    // Bounds check
                    if addr >= self.stack.len() {
                        return Err(format!("Invalid memory access: tried to load from address {} but stack size is {}", 
                                          addr, self.stack.len()));
                    }
                    
                    // Load from the address
                    self.ax = self.stack[addr];
                    
                    if self.debug {
                        println!("DEBUG VM: LI - Loaded value {} from address {}", self.ax, addr);
                    }
                },
                
                // load char
                op if op == OpCode::LC as u8 => {
                    let addr = self.ax as usize;
                    println!("DEBUG VM: LC - Loading char from stack addr {}", addr);
                    
                    // check valid addr
                    if addr >= self.stack.len() {
                        return Err(format!("Invalid memory access: tried to load char from address {} but stack size is {}", addr, self.stack.len()));
                    }
                    
                    self.ax = (self.stack[addr] & 0xFF) as i64;
                    println!("DEBUG VM: LC - Loaded char value {} from stack[{}]", self.ax, addr);
                },
                
                // SI: Store int
                op if op == OpCode::SI as u8 => {
                    let addr = self.stack[self.sp] as usize;
                    self.sp += 1;
                    
                    // Bounds check
                    if addr >= self.stack.len() {
                        return Err(format!("Invalid store: address {} out of range (stack size: {})", addr, self.stack.len()));
                    }
                    
                    // Store the value
                    self.stack[addr] = self.ax;
                    
                    if self.debug {
                        println!("DEBUG VM: SI - Stored value {} to address {}", self.ax, addr);
                    }
                },
                
                // store char
                op if op == OpCode::SC as u8 => {
                    let addr = self.stack[self.sp] as usize;
                    println!("DEBUG VM: SC - Storing char {} to stack addr {}", self.ax & 0xFF, addr);
                    
                    // check valid addr
                    if addr >= self.stack.len() {
                        return Err(format!("Invalid memory access: tried to store char at address {} but stack size is {}", addr, self.stack.len()));
                    }
                    
                    let current_value = self.stack[addr];
                    self.stack[addr] = (current_value & !0xFF) | (self.ax & 0xFF); // keep other bits
                    self.sp += 1;
                    
                    println!("DEBUG VM: SC - After store: stack[{}] = {}", addr, self.stack[addr]);
                },
                
                // push value
                op if op == OpCode::PSH as u8 => {
                    if self.sp == 0 {
                        return Err("Stack overflow in PSH operation".to_string());
                    }
                    self.sp -= 1;
                    println!("DEBUG VM: PSH - Pushing {} onto stack at position {}", self.ax, self.sp);
                    self.stack[self.sp] = self.ax;
                },
                
                // binary ops
                op if op == OpCode::OR as u8 => {
                    self.ax = self.stack[self.sp] | self.ax;
                    self.sp += 1;
                },
                op if op == OpCode::XOR as u8 => {
                    self.ax = self.stack[self.sp] ^ self.ax;
                    self.sp += 1;
                },
                op if op == OpCode::AND as u8 => {
                    self.ax = self.stack[self.sp] & self.ax;
                    self.sp += 1;
                },
                
                // comparisons
                op if op == OpCode::EQ as u8 => {
                    self.ax = (self.stack[self.sp] == self.ax) as i64;
                    self.sp += 1;
                },
                op if op == OpCode::NE as u8 => {
                    self.ax = (self.stack[self.sp] != self.ax) as i64;
                    self.sp += 1;
                },
                op if op == OpCode::LT as u8 => {
                    self.ax = (self.stack[self.sp] < self.ax) as i64;
                    self.sp += 1;
                },
                op if op == OpCode::GT as u8 => {
                    self.ax = (self.stack[self.sp] > self.ax) as i64;
                    self.sp += 1;
                },
                op if op == OpCode::LE as u8 => {
                    self.ax = (self.stack[self.sp] <= self.ax) as i64;
                    self.sp += 1;
                },
                op if op == OpCode::GE as u8 => {
                    self.ax = (self.stack[self.sp] >= self.ax) as i64;
                    self.sp += 1;
                },
                
                // bit shifts
                op if op == OpCode::SHL as u8 => {
                    self.ax = self.stack[self.sp] << self.ax;
                    self.sp += 1;
                },
                op if op == OpCode::SHR as u8 => {
                    self.ax = self.stack[self.sp] >> self.ax;
                    self.sp += 1;
                },
                
                // math ops
                op if op == OpCode::ADD as u8 => {
                    self.ax = self.stack[self.sp] + self.ax;
                    self.sp += 1;
                },
                op if op == OpCode::SUB as u8 => {
                    self.ax = self.stack[self.sp] - self.ax;
                    self.sp += 1;
                },
                op if op == OpCode::MUL as u8 => {
                    self.ax = self.stack[self.sp] * self.ax;
                    self.sp += 1;
                },
                op if op == OpCode::DIV as u8 => {
                    if self.ax == 0 {
                        return Err("division by zero".to_string());
                    }
                    self.ax = self.stack[self.sp] / self.ax;
                    self.sp += 1;
                },
                op if op == OpCode::MOD as u8 => {
                    if self.ax == 0 {
                        return Err("modulo by zero".to_string());
                    }
                    self.ax = self.stack[self.sp] % self.ax;
                    self.sp += 1;
                },
                
                // system calls
                op if op == OpCode::OPEN as u8 => {
                    self.ax = self.syscall_open()?;
                },
                op if op == OpCode::READ as u8 => {
                    self.ax = self.syscall_read()?;
                },
                op if op == OpCode::CLOS as u8 => {
                    self.ax = 0; // not supported
                },
                op if op == OpCode::PRTF as u8 => {
                    // Get argument count
                    let argc = self.next_code() as usize;
                    
                    if self.debug {
                        println!("DEBUG VM: PRTF - Called with {} arguments", argc);
                        for i in 0..argc {
                            println!("  Arg {}: {} at stack[{}]", i, self.stack[self.sp + i], self.sp + i);
                        }
                    }
                    
                    // Format string is the first pushed argument (last in our stack)
                    let format_addr = self.stack[self.sp + argc - 1] as usize;
                    
                    // Bounds check
                    if format_addr >= self.data.len() {
                        println!("ERROR: Invalid format string address: {}", format_addr);
                        print!("<invalid format string>");
                        std::io::stdout().flush().unwrap();
                        
                        // Clean up stack
                        self.sp += argc;
                        
                        // Set return value to 0 for error
                        self.ax = 0;
                        return Ok(0);
                    }
                    
                    // Read format string from data segment
                    let mut format_str = String::new();
                    let mut i = format_addr;
                    while i < self.data.len() && self.data[i] != 0 {
                        format_str.push(self.data[i] as char);
                        i += 1;
                    }
                    
                    if self.debug {
                        println!("DEBUG VM: PRTF - Format string: \"{}\"", format_str);
                    }
                    
                    // Process format string
                    let mut result = String::new();
                    let mut arg_idx = 0; // Start with first argument (excluding format string)
                    let mut i = 0;
                    
                    while i < format_str.len() {
                        let c = format_str.chars().nth(i).unwrap();
                        
                        if c == '%' && i + 1 < format_str.len() {
                            let next_c = format_str.chars().nth(i + 1).unwrap();
                            match next_c {
                                'd' => {
                                    // Integer format
                                    if arg_idx < argc - 1 { // -1 because format string is an argument
                                        // Arguments are in reverse order on the stack:
                                        // sp + 0 = last argument
                                        // sp + 1 = second-to-last argument
                                        // ...
                                        // sp + (argc-1) = format string
                                        
                                        // For printing, we want to go from first to last argument
                                        // So they should be: sp + (argc-2), sp + (argc-3), etc.
                                        let reverse_idx = argc - 2 - arg_idx;
                                        let arg_val = self.stack[self.sp + reverse_idx];
                                        
                                        if self.debug {
                                            println!("DEBUG VM: PRTF - %%d argument {} = {} (stack[{}])", 
                                                    arg_idx, arg_val, self.sp + reverse_idx);
                                        }
                                        
                                        // Format the integer
                                        result.push_str(&arg_val.to_string());
                                        arg_idx += 1;
                                    } else {
                                        result.push_str("<?>");
                                    }
                                    i += 2; // Skip format specifier
                                },
                                's' => {
                                    // String format
                                    if arg_idx < argc - 1 {
                                        // Calculate reverse index as with %d
                                        let reverse_idx = argc - 2 - arg_idx;
                                        let str_addr = self.stack[self.sp + reverse_idx] as usize;
                                        
                                        if self.debug {
                                            println!("DEBUG VM: PRTF - %%s argument {} = {} (stack[{}])", 
                                                    arg_idx, str_addr, self.sp + reverse_idx);
                                        }
                                        
                                        // Bounds check
                                        if str_addr < self.data.len() {
                                            // Read string from data segment
                                            let mut j = str_addr;
                                            while j < self.data.len() && self.data[j] != 0 {
                                                result.push(self.data[j] as char);
                                                j += 1;
                                            }
                                        } else {
                                            result.push_str("<bad string>");
                                        }
                                        arg_idx += 1;
                                    } else {
                                        result.push_str("<?>");
                                    }
                                    i += 2; // Skip format specifier
                                },
                                '%' => {
                                    // Literal % character
                                    result.push('%');
                                    i += 2; // Skip %%
                                },
                                _ => {
                                    // Unknown format specifier - treat as literal
                                    result.push('%');
                                    i += 1;
                                }
                            }
                        } else {
                            // Regular character
                            result.push(c);
                            i += 1;
                        }
                    }
                    
                    // Print the formatted result
                    print!("{}", result);
                    std::io::stdout().flush().unwrap();
                    
                    // Clean up stack
                    self.sp += argc;
                    
                    // Set return value to length of formatted string
                    self.ax = result.len() as i64;
                },
                op if op == OpCode::MALC as u8 => {
                    self.ax = self.syscall_malloc()?;
                },
                op if op == OpCode::FREE as u8 => {
                    // not supported
                    self.sp += 1;
                    self.ax = 0;
                },
                op if op == OpCode::MSET as u8 => {
                    self.ax = self.syscall_memset()?;
                },
                op if op == OpCode::MCMP as u8 => {
                    self.ax = self.syscall_memcmp()?;
                },
                op if op == OpCode::EXIT as u8 => {
                    if self.debug {
                        println!("exit({}) cycle = {}", self.stack[self.sp], self.cycle);
                    }
                    return Ok(self.stack[self.sp]);
                },
                
                // unknown op
                _ => return Err(format!("unknown instruction: {}", op)),
            }
        }
        
        // reached end without EXIT
        if self.debug {
            println!("program reached end, returning value {} after {} cycles", self.ax, self.cycle);
        }
        
        // return final value
        self.sp -= 1;
        self.stack[self.sp] = self.ax;
        Ok(self.stack[self.sp])
    }
    
    /// shows debug info
    fn print_debug_info(&self, op: usize) {
        const OP_NAMES: &[&str] = &[
            "LEA ", "IMM ", "JMP ", "JSR ", "BZ  ", "BNZ ", "ENT ", "ADJ ", "LEV ", "LI  ", "LC  ", "SI  ", "SC  ", "PSH ",
            "OR  ", "XOR ", "AND ", "EQ  ", "NE  ", "LT  ", "GT  ", "LE  ", "GE  ", "SHL ", "SHR ", "ADD ", "SUB ", "MUL ", "DIV ", "MOD ",
            "OPEN", "READ", "CLOS", "PRTF", "MALC", "FREE", "MSET", "MCMP", "EXIT",
        ];
        
        if op < OP_NAMES.len() {
            print!("{}> {}", self.cycle, OP_NAMES[op]);
            // print immediate value
            if op <= OpCode::ADJ as usize && self.pc < self.code.len() {
                println!(" {}", self.code[self.pc]);
            } else {
                println!();
            }
        } else {
            println!("{}> Unknown op: {}", self.cycle, op);
        }
    }
    
    /// gets next code value
    fn next_code(&mut self) -> i64 {
        let val = self.code[self.pc];
        self.pc += 1;
        val
    }
    
    /// loads int from memory
    pub fn load_int(&self, addr: usize) -> i64 {
        if self.debug {
            println!("  Loading int from addr {}, data len: {}", addr, self.data.len());
        }
        
        // check bounds
        if addr < self.data.len() && addr + 7 < self.data.len() {
            // from data segment
            let mut bytes = [0u8; 8];
            for i in 0..8 {
                bytes[i] = self.data[addr + i];
            }
            
            let value = i64::from_ne_bytes(bytes);
            if self.debug {
                println!("  Loaded bytes: {:?}, int value: {}", bytes, value);
            }
            value
        } else {
            // for small data
            if addr < self.data.len() {
                let value = self.data[addr] as i64;
                if self.debug {
                    println!("  Data segment too short, loaded single byte: {}", value);
                }
                value
            } else {
                // bad access
                if self.debug {
                    println!("  Invalid memory access at address {}", addr);
                }
                0
            }
        }
    }
    
    /// loads char from memory
    pub fn load_char(&self, addr: usize) -> u8 {
        if addr < self.data.len() {
            self.data[addr]
        } else {
            0
        }
    }
    
    /// stores int to memory
    pub fn store_int(&mut self, addr: usize, val: i64) {
        if self.debug {
            println!("  Storing int value: {} at address: {}", val, addr);
        }
        
        if addr + 7 >= self.data.len() {
            // grow if needed
            self.data.resize(addr + 8, 0);
        }
        
        let bytes = val.to_ne_bytes();
        for i in 0..8 {
            self.data[addr + i] = bytes[i];
        }
        
        if self.debug {
            println!("  Stored bytes: {:?}", bytes);
        }
    }
    
    /// stores char to memory
    pub fn store_char(&mut self, addr: usize, val: u8) {
        if addr >= self.data.len() {
            // grow if needed
            self.data.resize(addr + 1, 0);
        }
        
        self.data[addr] = val;
    }
    
    /// handles open syscall
    fn syscall_open(&mut self) -> Result<i64, String> {
        // minimal support
        self.sp += 2; // remove args
        Ok(0) // fake fd
    }
    
    /// handles read syscall
    fn syscall_read(&mut self) -> Result<i64, String> {
        // minimal support
        self.sp += 3; // remove args
        Ok(0) // read nothing
    }
    
    /// handles malloc syscall
    fn syscall_malloc(&mut self) -> Result<i64, String> {
        let size = self.stack[self.sp] as usize;
        self.sp += 1;
        
        // simple allocation
        let addr = self.data.len();
        self.data.resize(addr + size, 0);
        
        Ok(addr as i64)
    }
    
    /// handles memset syscall
    fn syscall_memset(&mut self) -> Result<i64, String> {
        let count = self.stack[self.sp] as usize;
        let value = self.stack[self.sp + 1] as u8;
        let dest = self.stack[self.sp + 2] as usize;
        self.sp += 3;
        
        if dest + count > self.data.len() {
            self.data.resize(dest + count, 0);
        }
        
        for i in 0..count {
            self.data[dest + i] = value;
        }
        
        Ok(dest as i64)
    }
    
    /// handles memcmp syscall
    fn syscall_memcmp(&mut self) -> Result<i64, String> {
        let count = self.stack[self.sp] as usize;
        let s2 = self.stack[self.sp + 1] as usize;
        let s1 = self.stack[self.sp + 2] as usize;
        self.sp += 3;
        
        if s1 + count > self.data.len() || s2 + count > self.data.len() {
            return Ok(-1); // out of bounds
        }
        
        for i in 0..count {
            let a = self.data[s1 + i];
            let b = self.data[s2 + i];
            if a != b {
                return Ok((a as i64) - (b as i64));
            }
        }
        
        Ok(0) // identical
    }
    
    fn op_to_string(&self, op: usize) -> String {
        match op {
            x if x == OpCode::LEA as usize => "LEA".to_string(),
            x if x == OpCode::IMM as usize => "IMM".to_string(),
            x if x == OpCode::JMP as usize => "JMP".to_string(),
            x if x == OpCode::JSR as usize => "JSR".to_string(),
            x if x == OpCode::BZ as usize => "BZ".to_string(),
            x if x == OpCode::BNZ as usize => "BNZ".to_string(),
            x if x == OpCode::ENT as usize => "ENT".to_string(),
            x if x == OpCode::ADJ as usize => "ADJ".to_string(),
            x if x == OpCode::LEV as usize => "LEV".to_string(),
            x if x == OpCode::LI as usize => "LI".to_string(),
            x if x == OpCode::LC as usize => "LC".to_string(),
            x if x == OpCode::SI as usize => "SI".to_string(),
            x if x == OpCode::SC as usize => "SC".to_string(),
            x if x == OpCode::PSH as usize => "PSH".to_string(),
            x if x == OpCode::OR as usize => "OR".to_string(),
            x if x == OpCode::XOR as usize => "XOR".to_string(),
            x if x == OpCode::AND as usize => "AND".to_string(),
            x if x == OpCode::EQ as usize => "EQ".to_string(),
            x if x == OpCode::NE as usize => "NE".to_string(),
            x if x == OpCode::LT as usize => "LT".to_string(),
            x if x == OpCode::GT as usize => "GT".to_string(),
            x if x == OpCode::LE as usize => "LE".to_string(),
            x if x == OpCode::GE as usize => "GE".to_string(),
            x if x == OpCode::SHL as usize => "SHL".to_string(),
            x if x == OpCode::SHR as usize => "SHR".to_string(),
            x if x == OpCode::ADD as usize => "ADD".to_string(),
            x if x == OpCode::SUB as usize => "SUB".to_string(),
            x if x == OpCode::MUL as usize => "MUL".to_string(),
            x if x == OpCode::DIV as usize => "DIV".to_string(),
            x if x == OpCode::MOD as usize => "MOD".to_string(),
            x if x == OpCode::OPEN as usize => "OPEN".to_string(),
            x if x == OpCode::READ as usize => "READ".to_string(),
            x if x == OpCode::CLOS as usize => "CLOS".to_string(),
            x if x == OpCode::PRTF as usize => "PRTF".to_string(),
            x if x == OpCode::MALC as usize => "MALC".to_string(),
            x if x == OpCode::FREE as usize => "FREE".to_string(),
            x if x == OpCode::MSET as usize => "MSET".to_string(),
            x if x == OpCode::MCMP as usize => "MCMP".to_string(),
            x if x == OpCode::EXIT as usize => "EXIT".to_string(),
            _ => format!("Unknown({})", op),
        }
    }
}

/// runs compiled code
pub fn run(source: &str, src: bool, debug: bool) -> Result<i64, String> {
    // parse source
    let mut parser = Parser::new(source, src);
    parser.init()?;
    let result = parser.parse();
    
    if result.is_err() {
        return Err(result.unwrap_err());
    }
    
    let (code, data) = result.unwrap();
    
    // early return if parsing only
    if src {
        return Ok(0);
    }
    
    // Print the code in debug mode
    if debug {
        println!("Generated code (length: {}):", code.len());
        let op_names = [
            "LEA", "IMM", "JMP", "JSR", "BZ", "BNZ", "ENT", "ADJ", "LEV", "LI", "LC", "SI", "SC", "PSH",
            "OR", "XOR", "AND", "EQ", "NE", "LT", "GT", "LE", "GE", "SHL", "SHR", "ADD", "SUB", "MUL", "DIV", "MOD",
            "OPEN", "READ", "CLOS", "PRTF", "MALC", "FREE", "MSET", "MCMP", "EXIT",
        ];
        
        let mut i = 0;
        while i < code.len() {
            let op = code[i] as usize;
            if op < op_names.len() {
                print!("{}: {} ", i, op_names[op]);
                
                // Instructions like IMM, JMP, etc. have an immediate operand
                if op <= OpCode::ADJ as usize && i + 1 < code.len() {
                    println!("{}", code[i + 1]);
                    i += 2;
                } else {
                    println!();
                    i += 1;
                }
            } else {
                println!("{}: Unknown op: {}", i, op);
                i += 1;
            }
        }
    }
    
    // execute code
    let mut vm = VM::new(code, data, debug);
    let result = vm.run();
    
    // show result in debug
    if let Ok(return_val) = result.as_ref() {
        if debug {
            println!("Program executed successfully, returned: {}", return_val);
        }
    }
    
    result
} 