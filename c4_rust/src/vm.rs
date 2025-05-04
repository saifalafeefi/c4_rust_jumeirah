/// runs compiled code
/// executes parser output

use crate::parser::{OpCode, Parser};
use std::io::Write;

// Define threshold to differentiate data/stack addresses
const DATA_STACK_THRESHOLD: usize = 1024 * 1024; // 1MB threshold

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
        // Define stack size and base address
        let stack_size = 8192; // Keep the stack size moderate
        let stack_base_addr = DATA_STACK_THRESHOLD; // Start stack addresses here

        // Ensure stack vector has enough capacity
        let total_stack_capacity = stack_base_addr + stack_size;
        let mut stack = vec![0i64; total_stack_capacity]; // Initialize stack

        // Set initial SP and BP relative to the base address
        let sp = stack_base_addr + stack_size - 20; // Leave room at the top
        let bp = sp;

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
        // Initialize PC, SP, BP
        self.pc = 0;
        
        // Set cycle counter
        self.cycle = 0;
        let max_cycles = 50000; // Instruction limit to prevent infinite loops
        
        // Main execution loop - run until EXIT or end of code
        while self.pc < self.code.len() {
            // Check cycle limit to avoid infinite loops
            if self.cycle >= max_cycles {
                return Err(format!("Execution aborted after {} instructions - possible infinite loop", max_cycles));
            }
            
            // Get current opcode
            let op_addr = self.pc;
            let op = self.code[self.pc] as u8;
            self.pc += 1; // Move past opcode
            
            // Optional debug output
            if self.debug {
                println!("VM LOOP: Processing Opcode {} ({}) at Addr {}", self.op_to_string(op as usize), op, op_addr);
            }
            
            // Increment cycle counter
            self.cycle += 1;
            
            // Execute the instruction
            match op {
                // LEA: Load effective address
                op if op == OpCode::LEA as u8 => {
                    let offset = self.code[op_addr + 1] as usize;
                    self.pc += 1; // Consume argument
                    
                    // Calculate effective address for a local variable
                    let addr = self.bp - offset;
                    
                    if self.debug {
                        println!("VM DEBUG: LEA - Local var offset {} => address {} (bp={})", offset, addr, self.bp);
                    }
                    
                    self.ax = addr as i64;
                },
                
                // IMM: Load immediate value
                op if op == OpCode::IMM as u8 => {
                    self.ax = self.code[op_addr + 1];
                    self.pc += 1; // Consume argument
                    if self.debug {
                        println!("DEBUG VM: IMM - Loaded immediate value {}", self.ax);
                    }
                },
                
                // JMP: Jump
                op if op == OpCode::JMP as u8 => {
                    self.pc = self.code[op_addr + 1] as usize; // Jump target is arg
                },
                
                // JSR: Jump to subroutine
                op if op == OpCode::JSR as u8 => {
                    // Push return address
                    if self.sp == 0 {
                        return Err("Stack overflow in JSR".to_string());
                    }
                    self.sp -= 1;
                    self.stack[self.sp] = self.pc as i64; // PC is already advanced past arg
                    
                    // Jump to function entry
                    self.pc = self.code[op_addr + 1] as usize; // Jump target is arg
                },
                
                // BZ: Branch if zero
                op if op == OpCode::BZ as u8 => {
                    let target = self.code[op_addr + 1] as usize;
                    self.pc += 1; // Consume argument
                    if self.ax == 0 {
                        self.pc = target;
                    }
                },
                
                // BNZ: Branch if not zero
                op if op == OpCode::BNZ as u8 => {
                    let target = self.code[op_addr + 1] as usize;
                    self.pc += 1; // Consume argument
                    if self.ax != 0 {
                        self.pc = target;
                    }
                },
                
                // ENT: Enter function
                op if op == OpCode::ENT as u8 => {
                    let local_size = self.code[op_addr + 1] as usize;
                    self.pc += 1; // Consume argument
                    
                    if self.debug {
                        println!("DEBUG VM: ENT - Creating stack frame with {} local variables", local_size);
                        println!("DEBUG VM: ENT - Old BP: {}, Old SP: {}", self.bp, self.sp);
                        
                        // Debug: dump stack before creating stack frame
                        println!("Stack before function entry:");
                        let dump_start = self.sp.saturating_sub(5);
                        self.dump_stack(dump_start, 10);
                    }
                    
                    // Push old base pointer
                    if self.sp < 2 {
                        // Grow the stack if needed
                        if self.debug {
                            println!("DEBUG VM: ENT - Growing stack to accommodate base pointer");
                        }
                        let new_size = self.stack.len() + 64;
                        self.stack.resize(new_size, 0);
                    }
                    
                    self.sp = self.sp.saturating_sub(1);
                    self.stack[self.sp] = self.bp as i64;
                    
                    // Set new base pointer
                    self.bp = self.sp;
                    
                    // Reserve space for locals - add buffer space based on local_size
                    // For 0-1 locals: 4 buffer slots
                    // For 2+ locals: local_size * 2 buffer slots
                    let buffer_size = if local_size <= 1 { 4 } else { local_size * 2 };
                    let total_space = local_size + buffer_size;
                    
                    if self.debug {
                        println!("DEBUG VM: ENT - Allocating {} locals with {} buffer slots (total: {})", 
                                 local_size, buffer_size, total_space);
                    }
                    
                    // Make sure we have enough stack space
                    if self.sp < total_space + 1 {
                        println!("DEBUG VM: ENT - Growing stack for local variables");
                        let needed_space = total_space + 64;  // Add extra buffer
                        let current_sp = self.sp;
                        let current_bp = self.bp;
                        
                        // Create new stack with more space
                        let new_size = self.stack.len() + needed_space;
                        let mut new_stack = vec![0; new_size];
                        
                        // Copy existing stack to expanded space
                        for i in 0..self.stack.len() {
                            new_stack[i + needed_space] = self.stack[i];
                        }
                        
                        // Update stack pointers
                        self.sp = current_sp + needed_space;
                        self.bp = current_bp + needed_space;
                        self.stack = new_stack;
                    }
                    
                    // Now safely allocate space for locals
                    self.sp = self.sp.saturating_sub(total_space);
                    
                    // Initialize all local variables and buffer space to zero
                    for i in self.sp..self.bp {
                        self.stack[i] = 0;
                    }
                    
                    if self.debug {
                        println!("DEBUG VM: ENT - New BP: {}, New SP: {} (added {} buffer slots)", 
                                self.bp, self.sp, buffer_size);
                        println!("DEBUG VM: ENT - Reserved space from {} to {}", self.sp, self.bp - 1);
                        
                        // Debug: dump stack after creating stack frame
                        println!("Stack after function entry:");
                        let dump_start = self.sp.saturating_sub(2);
                        let dump_count = (self.bp - self.sp + 5).min(20);
                        self.dump_stack(dump_start, dump_count);
                    }
                },
                
                // ADJ: Adjust stack
                op if op == OpCode::ADJ as u8 => {
                    let n = self.code[op_addr + 1] as usize;
                    self.pc += 1; // Consume argument
                    
                    // Check if we need to grow the stack
                    if self.sp + n >= self.stack.len() {
                        let new_size = self.sp + n + 64;  // Add some buffer
                        println!("DEBUG VM: ADJ - Growing stack from {} to {} for adjustment by {}", 
                                 self.stack.len(), new_size, n);
                        self.stack.resize(new_size, 0);
                    }
                    
                    self.sp += n;
                    
                    if self.debug {
                        println!("DEBUG VM: ADJ - Adjusted stack pointer by {} to {}", n, self.sp);
                    }
                },
                
                // LEV: Leave function
                op if op == OpCode::LEV as u8 => {
                    // Safety checks
                    if self.bp >= self.stack.len() {
                        if self.debug {
                            println!("ERROR: LEV - Invalid BP value: {}", self.bp);
                        }
                        return Err("Stack corruption - invalid base pointer".to_string());
                    }

                    // Clean up stack frame
                    let sp = self.bp;
                    
                    // Bounds check for stack access
                    if sp + 1 >= self.stack.len() {
                        if self.debug {
                            println!("ERROR: LEV - Stack frame too small, can't read return address");
                        }
                        return Err("Stack corruption - can't read return address".to_string());
                    }
                    
                    let bp = self.stack[sp];
                    let pc = self.stack[sp + 1];
                    
                    if self.debug {
                        println!("DEBUG VM: LEV - Leaving function with SP={}, BP={}", self.sp, self.bp);
                        println!("              - Return address: PC={}, new BP={}", pc, bp);
                    }
                    
                    self.sp = sp + 2; // Remove frame
                    self.bp = bp as usize;
                    
                    // Check if we're returning from main
                    if pc == 0 || bp == 0 {
                        if self.debug {
                            println!("  LEV: returning from main function with value {}", self.ax);
                        }
                        // Return from main function - exit program
                        return Ok(self.ax);
                    }
                    
                    // Continue execution at return address
                    self.pc = pc as usize;
                },
                
                // load int
                op if op == OpCode::LI as u8 => {
                    let addr = self.ax as usize;
                    
                    if addr < DATA_STACK_THRESHOLD {
                        // Load from data segment (assuming it's aligned)
                        if addr + std::mem::size_of::<i64>() > self.data.len() {
                             return Err(format!("Data segment read out of bounds: addr={}, size={}", addr, self.data.len()));
                        }
                        let bytes = self.data[addr..addr + std::mem::size_of::<i64>()].try_into().unwrap();
                        self.ax = i64::from_ne_bytes(bytes);
                        if self.debug {
                            println!("VM DEBUG: LI - Loaded int {} from data address {}", self.ax, addr);
                        }
                    } else {
                        // Load from stack
                        if addr >= self.stack.len() {
                            return Err(format!("Stack read out of bounds: addr={}, size={}", addr, self.stack.len()));
                        }
                        self.ax = self.stack[addr];
                        if self.debug {
                            println!("VM DEBUG: LI - Loaded int {} from stack address {}", self.ax, addr);
                            // Print stack around the loaded address to help debug array issues
                            self.dump_stack(addr.saturating_sub(3), 6);
                        }
                    }
                    
                    self.cycle += 1;
                },
                
                // load char
                op if op == OpCode::LC as u8 => {
                    let addr = self.ax as usize;
                    if addr < DATA_STACK_THRESHOLD {
                        // Load from data segment
                        if addr >= self.data.len() {
                            return Err(format!("Data segment read out of bounds: addr={}, size={}", addr, self.data.len()));
                        }
                        self.ax = self.data[addr] as i64;
                        println!("DEBUG VM: LC - Loaded char '{}' ({}) from data address {}", self.ax as u8 as char, self.ax, addr);
                    } else {
                        // Load from stack (lowest byte)
                        if addr >= self.stack.len() {
                            return Err(format!("Stack read out of bounds: addr={}, size={}", addr, self.stack.len()));
                        }
                        self.ax = self.stack[addr] & 0xFF;
                        println!("DEBUG VM: LC - Loaded char '{}' ({}) from stack address {}", self.ax as u8 as char, self.ax, addr);
                    }
                },
                
                // SI: Store int
                op if op == OpCode::SI as u8 => {
                    // Check if we need to read the stack
                    if self.sp >= self.stack.len() {
                        return Err(format!("Stack empty in SI: sp={}", self.sp));
                    }
                    
                    // Get address from top of stack
                    let raw_addr_from_stack = self.stack[self.sp];
                    self.sp += 1; // Pop address
                    
                    // Print debug info
                    if self.debug {
                        println!("VM SI HANDLER: Reading address {} from stack[{}]", raw_addr_from_stack, self.sp - 1);
                        println!("VM SI HANDLER: Checking addr {} < DATA_STACK_THRESHOLD {}", 
                                raw_addr_from_stack as usize, DATA_STACK_THRESHOLD);
                    }
                    
                    // Convert the address to usize
                    let addr = raw_addr_from_stack as usize;
                    
                    // Get value to store from accumulator
                    let value_to_store = self.ax;
                    
                    // Store in appropriate segment based on address range
                    if addr < DATA_STACK_THRESHOLD {
                        // Store in data segment (for static data)
                        if addr + std::mem::size_of::<i64>() > self.data.len() {
                            // Resize the data segment to accommodate the new value
                            let new_size = addr + std::mem::size_of::<i64>() + 64;
                            if self.debug {
                                println!("DEBUG VM: SI - Resized data segment to {} for address {}", new_size, addr);
                            }
                            self.data.resize(new_size, 0);
                        }
                        
                        // Store value as bytes in data segment
                        let bytes = value_to_store.to_ne_bytes();
                        for i in 0..std::mem::size_of::<i64>() {
                            self.data[addr + i] = bytes[i];
                        }
                        if self.debug {
                            println!("DEBUG VM: SI - Stored int {} to data address {}", value_to_store, addr);
                        }
                    } else {
                        // Store in stack
                        if addr >= self.stack.len() {
                            // Grow the stack to accommodate the address
                            let new_size = addr + 64;
                            if self.debug {
                                println!("DEBUG VM: SI - Growing stack from {} to {} for address {}", self.stack.len(), new_size, addr);
                            }
                            self.stack.resize(new_size, 0);
                        }
                        
                        // Store directly in stack as i64
                        self.stack[addr] = value_to_store;
                        if self.debug {
                            println!("DEBUG VM: SI - Stored int {} to stack address {}", value_to_store, addr);
                        }
                    }
                    
                    self.cycle += 1;
                },
                
                // store char
                op if op == OpCode::SC as u8 => {
                    // Pop the address from the stack
                    if self.sp >= self.stack.len() {
                        return Err(format!("Stack empty in SC: sp={}", self.sp));
                    }
                    let addr = self.stack[self.sp] as usize;
                    self.sp += 1;
                    let char_val = (self.ax & 0xFF) as u8;
                    
                    if addr < DATA_STACK_THRESHOLD {
                        // Store to data segment
                        if addr >= self.data.len() {
                           self.data.resize(addr + 1, 0);
                           println!("DEBUG VM: SC - Resized data segment to {} for address {}", self.data.len(), addr);
                        }
                        self.data[addr] = char_val;
                         println!("DEBUG VM: SC - Stored char '{}' ({}) to data address {}", char_val as char, char_val, addr);
                    } else {
                         // Store to stack (lowest byte)
                         if addr >= self.stack.len() {
                             let new_size = addr + 64; // Add buffer
                             println!("DEBUG VM: SC - Growing stack from {} to {} for address {}", self.stack.len(), new_size, addr);
                             self.stack.resize(new_size, 0);
                         }
                         // Modify only the lowest byte, preserving higher bytes
                         self.stack[addr] = (self.stack[addr] & !0xFF) | (char_val as i64);
                         println!("DEBUG VM: SC - Stored char '{}' ({}) to stack address {}, stack[{}] now {}", char_val as char, char_val, addr, addr, self.stack[addr]);
                     }
                },
                
                // push value
                op if op == OpCode::PSH as u8 => {
                    // Check if we need to grow/protect the stack
                    if self.sp == 0 {
                        // Grow the stack if needed
                        if self.debug {
                            println!("DEBUG VM: PSH - Growing stack to accommodate more pushes");
                        }
                        let new_size = self.stack.len() + 64;
                        let mut new_stack = vec![0; new_size];
                        
                        // Copy existing stack to new space
                        for i in 0..self.stack.len() {
                            new_stack[i + 64] = self.stack[i];
                        }
                        
                        // Update stack pointers
                        self.sp += 64;
                        self.bp += 64;
                        self.stack = new_stack;
                    }
                    
                    // Now push the value safely
                    self.sp = self.sp.saturating_sub(1);
                    if self.debug {
                        println!("DEBUG VM: PSH - Pushing {} onto stack at position {}", self.ax, self.sp);
                    }
                    self.stack[self.sp] = self.ax;
                },
                
                // swap top of stack with ax
                op if op == OpCode::SWP as u8 => {
                    if self.sp >= self.stack.len() {
                        return Err("Stack underflow in SWP operation".to_string());
                    }
                    let temp = self.stack[self.sp];
                    self.stack[self.sp] = self.ax;
                    self.ax = temp;
                    println!("DEBUG VM: SWP - Swapped with top of stack, AX now = {}", self.ax);
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
                    let argc = self.code[op_addr + 1] as usize;
                    self.pc += 1; // Consume argument

                    // Debug info for PRTF call
                    if self.debug {
                        println!("DEBUG VM: PRTF - Called with {} arguments", argc);
                    }
                    
                    // Create a temporary slice reference to the arguments for easier access
                    let t: &[i64] = &self.stack[self.sp..self.sp + argc];
                    
                    // First argument is the format string address
                    let format_addr = t[argc - 1] as usize; // t[-1] in original code
                    
                    // Bounds check
                    if format_addr >= self.data.len() {
                        if self.debug {
                            println!("ERROR: Invalid format string address: {}", format_addr);
                        }
                        print!("<invalid format string>");
                        std::io::stdout().flush().unwrap();
                        
                        // Clean up stack
                        self.sp += argc;
                        
                        // Set return value to 0 for error
                        self.ax = 0;
                        continue; // Skip the rest of the loop body
                    }
                    
                    // Read format string from data segment
                    let mut format_str = String::new();
                    let mut i = format_addr;
                    while i < self.data.len() && self.data[i] != 0 {
                        format_str.push(self.data[i] as char);
                        i += 1;
                    }
                    
                    // Show the format string contents clearly for debugging
                    if self.debug {
                        println!("DEBUG VM: PRTF - Format string: \"{}\"", format_str);
                    }
                    
                    // Process format string
                    let mut result = String::new();
                    let mut arg_idx = 0; // Track which format specifier we're processing
                    let format_chars: Vec<char> = format_str.chars().collect();
                    let mut i = 0;

                    while i < format_chars.len() {
                        let c = format_chars[i];

                        if c == '%' && i + 1 < format_chars.len() {
                            let next_c = format_chars[i + 1];
                            match next_c {
                                'd' => {
                                    // Integer format
                                    if arg_idx < argc - 1 {
                                        let arg_val = t[argc - 2 - arg_idx];
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
                                        // Get string address from arg stack
                                        let str_addr = t[argc - 2 - arg_idx] as usize;
                                        
                                        // Read from data segment
                                        if str_addr < DATA_STACK_THRESHOLD {
                                            let mut j = str_addr;
                                            while j < self.data.len() && self.data[j] != 0 {
                                                result.push(self.data[j] as char);
                                                j += 1;
                                            }
                                        } else {
                                            // Read from stack segment
                                            let mut stack_idx = str_addr;
                                            while stack_idx < self.stack.len() {
                                                let char_byte = (self.stack[stack_idx] & 0xFF) as u8;
                                                if char_byte == 0 {
                                                    break;
                                                }
                                                result.push(char_byte as char);
                                                stack_idx += 1;
                                            }
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
                    // Check for valid stack access
                    if self.sp >= self.stack.len() {
                        if self.debug {
                            println!("ERROR: EXIT - Invalid stack pointer: {}", self.sp);
                        }
                        return Err("Stack corruption on EXIT - invalid stack pointer".to_string());
                    }
                    
                    let exit_code = self.stack[self.sp];
                    
                    if self.debug {
                        println!("exit({}) cycle = {}", exit_code, self.cycle);
                    }
                    return Ok(exit_code);
                },
                
                // Unknown opcode - error out
                _ => return Err(format!("unknown instruction: {}", op)),
            }
        }
        
        // If code reached end without EXIT, return AX value
        if self.debug {
            println!("Program reached end without EXIT instruction. AX = {}", self.ax);
        }
        Ok(self.ax)
    }
    
    /// print debug info
    fn print_debug_info(&self, op: usize, addr: usize, _arg: Option<i64>) { // Arg no longer passed
        // Disable most debug output but keep important diagnostics
        if self.debug { // Use the VM's debug flag
            // Print cycle count and PC
            println!("DEBUG VM: cycle = {}, PC = {}", self.cycle, self.pc);
            
            // Print opcode and AX
            let opcode_name = self.op_to_string(op);
            print!("DEBUG VM: Opcode={}, AX={}", opcode_name, self.ax);
            println!(); // Newline
            
            // Print stack pointer, base pointer
            println!("DEBUG VM: SP = {}, BP = {}", self.sp, self.bp);
        }
    }
    
    /// gets next code value
    // fn next_code(&mut self) -> i64 { // No longer needed as args are pre-fetched
    //     let val = self.code[self.pc];
    //     self.pc += 1;
    //     val
    // }
    
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
            x if x == OpCode::SWP as usize => "SWP".to_string(),
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
    
    /// debug helper to print stack
    fn dump_stack(&self, start: usize, count: usize) {
        if !self.debug {
            return; // Don't print stack dump if not in debug mode
        }
        
        println!("==== STACK DUMP ====");
        println!("SP: {}, BP: {}, PC: {}", self.sp, self.bp, self.pc);
        
        // Ensure start is not underflowing
        let safe_start = if start > self.stack.len() {
            0 // If start is too large (unsigned underflow happened), start from 0
        } else {
            start
        };
        
        // Calculate end index carefully to avoid overflow
        let end = std::cmp::min(safe_start.saturating_add(count), self.stack.len());
        
        // Print stack entries
        for i in safe_start..end {
            println!("stack[{}] = {}", i, self.stack[i]);
        }
        println!("====================");
    }
}

fn opcode_has_argument(op: u8) -> bool {
    matches!(op,
        x if x == OpCode::LEA as u8 ||
             x == OpCode::IMM as u8 ||
             x == OpCode::JMP as u8 ||
             x == OpCode::JSR as u8 ||
             x == OpCode::BZ as u8 ||
             x == OpCode::BNZ as u8 ||
             x == OpCode::ENT as u8 ||
             x == OpCode::ADJ as u8 ||
             x == OpCode::PRTF as u8
    )
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