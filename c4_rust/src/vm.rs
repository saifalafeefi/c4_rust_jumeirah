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
        // check stack space
        if self.sp < 2 {
            return Err("Insufficient stack space for initial frame".to_string());
        }
        
        // push return address
        self.sp -= 1;
        self.stack[self.sp] = OpCode::EXIT as i64;
        
        // TODO: Set up argc and argv for main
        
        // main loop
        while self.pc < self.code.len() {
            let op = self.code[self.pc] as usize;
            self.pc += 1;
            self.cycle += 1;
            
            // stop if looping forever
            if self.cycle > 1_000_000 {
                return Err("Execution halted: cycle limit exceeded (possible infinite recursion)".to_string());
            }
            
            if self.debug {
                self.print_debug_info(op);
            }
            
            // check op validity
            if op >= 39 { // 39 is the count of valid opcodes
                return Err(format!("Invalid opcode: {}", op));
            }
            
            match op as u8 {
                // load address
                op if op == OpCode::LEA as u8 => {
                    // check next exists
                    if self.pc >= self.code.len() {
                        return Err(format!("Unexpected end of code after LEA at pc={}", self.pc - 1));
                    }
                    
                    let offset = self.next_code();
                    if self.debug {
                        println!("  LEA: bp = {}, offset = {}, addr = {}", self.bp, offset, self.bp - offset as usize);
                    }
                    // locals at negative offset
                    self.ax = (self.bp - offset as usize) as i64;
                },
                
                // load immediate
                op if op == OpCode::IMM as u8 => {
                    // check next exists
                    if self.pc >= self.code.len() {
                        return Err(format!("Unexpected end of code after IMM at pc={}", self.pc - 1));
                    }
                    
                    self.ax = self.next_code();
                },
                
                // jump
                op if op == OpCode::JMP as u8 => {
                    // check next exists
                    if self.pc >= self.code.len() {
                        return Err(format!("Unexpected end of code after JMP at pc={}", self.pc - 1));
                    }
                    
                    let addr = self.next_code() as usize;
                    
                    // check valid address
                    if addr >= self.code.len() {
                        return Err(format!("Invalid jump address: {} (code length: {})", addr, self.code.len()));
                    }
                    
                    self.pc = addr;
                },
                
                // call subroutine
                op if op == OpCode::JSR as u8 => {
                    let addr = self.next_code() as usize;
                    
                    // check stack room
                    if self.sp < 1 {
                        return Err("Stack overflow in JSR".to_string());
                    }
                    
                    // save return address
                    self.sp -= 1;
                    self.stack[self.sp] = self.pc as i64;
                    
                    // jump to function
                    self.pc = addr;
                },
                
                // branch if zero
                op if op == OpCode::BZ as u8 => {
                    let addr = self.next_code() as usize;
                    if self.ax == 0 {
                        self.pc = addr;
                    }
                },
                
                // branch if nonzero
                op if op == OpCode::BNZ as u8 => {
                    let addr = self.next_code() as usize;
                    if self.ax != 0 {
                        self.pc = addr;
                    }
                },
                
                // enter function
                op if op == OpCode::ENT as u8 => {
                    let frame_size = self.next_code() as usize;
                    if self.debug {
                        println!("  ENT: creating stack frame with {} local variables", frame_size);
                    }
                    
                    // check stack space
                    if self.sp < frame_size + 1 {
                        return Err(format!("Stack overflow in ENT: need {} slots but only {} available",
                                         frame_size + 1, self.sp));
                    }
                    
                    // save old bp
                    self.sp -= 1;
                    self.stack[self.sp] = self.bp as i64;
                    
                    // set new bp
                    self.bp = self.sp;
                    
                    // make space for locals
                    if frame_size > 0 {
                        self.sp -= frame_size;
                        
                        // zero local vars
                        for i in 0..frame_size {
                            if self.sp + i < self.stack.len() {
                                self.stack[self.sp + i] = 0;
                            }
                        }
                    }
                },
                
                // adjust stack
                op if op == OpCode::ADJ as u8 => {
                    self.sp += self.next_code() as usize;
                },
                
                // leave function
                op if op == OpCode::LEV as u8 => {
                    // check pointers first
                    if self.bp >= self.stack.len() {
                        return Err(format!("Invalid base pointer in LEV: bp={}, stack len={}", 
                                          self.bp, self.stack.len()));
                    }
                    
                    // restore sp to bp
                    self.sp = self.bp;
                    
                    // get old bp
                    let prev_bp = self.stack[self.sp] as usize;
                    
                    // update bp and sp
                    self.bp = prev_bp;
                    self.sp += 1;
                    
                    // check for return addr
                    if self.sp >= self.stack.len() {
                        // at end of stack
                        return Ok(self.ax);
                    }
                    
                    // get return addr
                    let ret_addr = self.stack[self.sp] as usize;
                    self.sp += 1;
                    
                    // jump back
                    self.pc = ret_addr;
                    
                    // handle exit code
                    if self.pc >= self.code.len() || 
                       (self.pc < self.code.len() && self.code[self.pc] as u8 == OpCode::EXIT as u8) {
                        if self.sp == 0 {
                            return Err("Stack overflow in LEV before EXIT".to_string());
                        }
                        self.sp -= 1;
                        self.stack[self.sp] = self.ax;
                    }
                },
                
                // load int
                op if op == OpCode::LI as u8 => {
                    let addr = self.ax as usize;
                    if self.debug {
                        println!("  LI: loading from stack addr {}", addr);
                    }
                    
                    // check valid addr
                    if addr >= self.stack.len() {
                        return Err(format!("Invalid memory access: tried to load from address {} but stack size is {}", addr, self.stack.len()));
                    }
                    
                    self.ax = self.stack[addr];
                    if self.debug {
                        println!("  LI: loaded value {}", self.ax);
                    }
                },
                
                // load char
                op if op == OpCode::LC as u8 => {
                    let addr = self.ax as usize;
                    if self.debug {
                        println!("  LC: loading char from stack addr {}", addr);
                    }
                    
                    // check valid addr
                    if addr >= self.stack.len() {
                        return Err(format!("Invalid memory access: tried to load char from address {} but stack size is {}", addr, self.stack.len()));
                    }
                    
                    self.ax = (self.stack[addr] & 0xFF) as i64;
                },
                
                // store int
                op if op == OpCode::SI as u8 => {
                    let addr = self.stack[self.sp] as usize;
                    if self.debug {
                        println!("  SI: storing {} to stack addr {}", self.ax, addr);
                    }
                    
                    // check valid addr
                    if addr >= self.stack.len() {
                        return Err(format!("Invalid memory access: tried to store at address {} but stack size is {}", addr, self.stack.len()));
                    }
                    
                    self.stack[addr] = self.ax;
                    self.sp += 1;
                },
                
                // store char
                op if op == OpCode::SC as u8 => {
                    let addr = self.stack[self.sp] as usize;
                    if self.debug {
                        println!("  SC: storing char {} to stack addr {}", self.ax & 0xFF, addr);
                    }
                    
                    // check valid addr
                    if addr >= self.stack.len() {
                        return Err(format!("Invalid memory access: tried to store char at address {} but stack size is {}", addr, self.stack.len()));
                    }
                    
                    let current_value = self.stack[addr];
                    self.stack[addr] = (current_value & !0xFF) | (self.ax & 0xFF); // keep other bits
                    self.sp += 1;
                },
                
                // push value
                op if op == OpCode::PSH as u8 => {
                    if self.sp == 0 {
                        return Err("Stack overflow in PSH operation".to_string());
                    }
                    self.sp -= 1;
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
                
                // syscalls
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
                    self.ax = self.syscall_printf()?;
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
    
    /// handles printf syscall
    fn syscall_printf(&mut self) -> Result<i64, String> {
        // get arg count
        let argc = self.next_code() as usize;
        
        if self.debug {
            println!("PRINTF: called with {} arguments", argc);
        }
        
        // check stack space
        if self.sp + argc > self.stack.len() {
            return Err(format!("Stack overflow in printf: sp={}, argc={}, stack_len={}",
                              self.sp, argc, self.stack.len()));
        }
        
        // args on stack
        let format_addr = self.stack[self.sp] as usize;
        
        if self.debug {
            println!("PRINTF: format string address: {}", format_addr);
        }
        
        // check format addr
        if format_addr >= self.data.len() {
            // allow auto expand
            let needed_size = format_addr + 1;
            self.data.resize(needed_size, 0);
            
            // clean stack
            self.sp += argc;
            
            // return 0
            return Ok(0);
        }
        
        // read format string
        let mut format_str = String::new();
        let mut i = format_addr;
        while i < self.data.len() && self.data[i] != 0 {
            format_str.push(self.data[i] as char);
            i += 1;
        }
        
        if self.debug {
            println!("PRINTF: format string: \"{}\"", format_str);
            println!("PRINTF: args on stack:");
            for i in 0..argc {
                if self.sp + i < self.stack.len() {
                    println!("  Arg {}: {}", i, self.stack[self.sp + i]);
                }
            }
        }
        
        // process format
        let mut result = String::new();
        let mut chars = format_str.chars().peekable();
        let mut arg_index = 1;  // first arg after format
        
        while let Some(ch) = chars.next() {
            if ch == '%' {
                // handle format
                match chars.next() {
                    Some('d') => {
                        // integer
                        if arg_index < argc {
                            let value = self.stack[self.sp + arg_index];
                            result.push_str(&value.to_string());
                            arg_index += 1;
                        } else {
                            result.push_str("?ARG?");
                        }
                    },
                    Some('s') => {
                        // string
                        if arg_index < argc {
                            let str_addr = self.stack[self.sp + arg_index] as usize;
                            if str_addr < self.data.len() {
                                let mut j = str_addr;
                                while j < self.data.len() && self.data[j] != 0 {
                                    result.push(self.data[j] as char);
                                    j += 1;
                                }
                            } else {
                                result.push_str("(invalid str)");
                            }
                            arg_index += 1;
                        } else {
                            result.push_str("?ARG?");
                        }
                    },
                    Some('%') => {
                        // escaped %
                        result.push('%');
                    },
                    Some(c) => {
                        // unknown format
                        result.push('%');
                        result.push(c);
                    },
                    None => {
                        // end after %
                        result.push('%');
                    }
                }
            } else {
                // normal char
                result.push(ch);
            }
        }
        
        // output
        print!("{}", result);
        let _ = std::io::stdout().flush();
        
        // clean stack
        self.sp += argc;
        
        // return length
        Ok(result.len() as i64)
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